use waker_fn::waker_fn;      // Cargo.toml: waker-fn = "1.1"
use futures_lite::pin;       // Cargo.toml: futures-lite = "1.11"
use crossbeam::sync::Parker; // Cargo.toml: crossbeam = "0.8"
use std::future::Future;
use std::task::{Context, Poll};

//作用是 同步地等待一个异步 Future 完成
//F: Future 是一个泛型约束，表示 block_on 可以接受任何实现了 Future trait 的类型
pub fn block_on<F: Future>(future: F) -> F::Output {
//Parker crossbeam::sync，是一个用于线程间同步的工具。用于阻塞当前线程直到异步任务的进度改变
    let parker = Parker::new();
//Unparker 是 Parker 的配对对象，用来从其他线程唤醒当前线程。通过 unparker.unpark()，可以通知 block_on 中的循环继续执
    let unparker = parker.unparker().clone();
//在异步编程中，Waker 是用来通知线程某个任务的状态发生了变化
//它通常与 poll 方法一起工作，确保异步任务能够继续执行。
//当任务的状态从 "挂起" (Poll::Pending) 变为 "完成" (Poll::Ready)，需要通过 Waker 来唤醒线程，让它继续执行
    let waker = waker_fn(move || unparker.unpark());
    let mut context = Context::from_waker(&waker);
//pin! 来自 futures-lite，它是一个宏，确保 future 被固定在内存中不可移动
    pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => parker.park(),
        }
    }
}

#[test]
fn test() {
    assert_eq!(block_on(std::future::ready(42)), 42);

    use async_std::task::{spawn, sleep};
    use futures_lite::FutureExt;
    use std::time::Duration;

    assert_eq!(
        block_on({
            let one_sec = async {
                sleep(Duration::from_secs(1)).await;
                43
            };
            let half_sec = async {
                sleep(Duration::from_millis(500)).await;
                44
            };
            spawn(one_sec.race(half_sec))
        }),
        44);
}
