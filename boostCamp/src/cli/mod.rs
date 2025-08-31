mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};
#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts{
    #[command(subcommand)]
    pub cmd : SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand{
    #[command(name="csv",about="parser csv file to json or yaml")]
    Csv(CsvOpts),
}


fn verify_file(filename:&str) ->Result<String,&'static str>{
    if filename == "-" || Path::new(filename).exists(){
        Ok(filename.into())
    } else {
        Err("File not exists")
    }
}









