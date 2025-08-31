mod cli;
mod process;
mod utils;

pub use cli::*;  //通用的导出
pub use process::*;
pub use utils::*;

use enum_dispatch::enum_dispatch;

#[allow(async_fn_in_trait)]  //允许在trait中定义异步方法
#[enum_dispatch]  //enum_dispatch crate 的过程宏  自动为枚举实现trait，实现基于枚举变体的动态分发
pub trait CmdExector{
    async fn execute(self) -> anyhow::Result<()>;
}

//为什么这样设计？  没理解如何避免动态分发的开销


























