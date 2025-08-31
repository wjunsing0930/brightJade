use clap::Parser;
use rli::{CmdExector, Opts};

#[tokio::main]
async fn main(){
    tracing_subscribe::fmt::init();
    let opts = Opts::parse();
    opts.cmd.execute().await?;

    Ok(())
}

