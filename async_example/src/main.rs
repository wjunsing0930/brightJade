use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
    time::{Duration, Instant},
    sync::{Arc, Mutex},
    thread,
};

/// -------- 1. 自定义 Task 结构，持有 Future + Waker --------
struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    // 这里只做单线程示例，所以 Waker 里只需要能把自己丢回队列
    executor: ExecutorHandle,
}

type TaskPtr = Arc<Task>;

/// -------- 2. 执行器 --------
#[derive(Clone)]
struct ExecutorHandle {
    queue: Arc<Mutex<Vec<TaskPtr>>>,
}

impl ExecutorHandle {
    fn spawn(&self, fut: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(fut)),
            executor: self.clone(),
        });
        self.queue.lock().unwrap().push(task.clone());
    }

    fn run(&self) {
        while let Some(task) = self.queue.lock().unwrap().pop() {
            // 创建一个 Waker，内部保存 task + queue
            let waker = futures::task::waker_ref(&task);
            let mut cx = Context::from_waker(&*waker);
            let mut fut = task.future.lock().unwrap();
            if let Poll::Pending = fut.as_mut().poll(&mut cx) {
                // 未完成，由 wake* 决定是否重新入队
            }
        }
    }
}

/// Task 变 Waker 需要实现 ArcWake
impl futures::task::ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let mut q = arc_self.executor.queue.lock().unwrap();
        q.push(arc_self.clone());   // 重新放入队列
    }
}

/// -------- 3. 一个定时器 Future --------
struct Delay {
    when: Instant,
}

impl Delay {
    fn new(dur: Duration) -> Self {
        Delay { when: Instant::now() + dur }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if Instant::now() >= self.when {
            println!("Delay done at {:?}", Instant::now());
            Poll::Ready(())
        } else {
            // 还没到时间 —— 需要在另外的线程里等到时间到了再唤醒
            let waker = cx.waker().clone();
            let when = self.when;
            thread::spawn(move || {
                let now = Instant::now();
                if when > now {
                    thread::sleep(when - now);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}

/// -------- 4. main --------
fn main() {
    let exec = ExecutorHandle { queue: Arc::new(Mutex::new(Vec::new())) };

    exec.spawn(async {
        println!("Hello, world @ {:?}", Instant::now());
        Delay::new(Duration::from_secs(2)).await;
        println!("Goodbye @ {:?}", Instant::now());
    });

    exec.run();
}