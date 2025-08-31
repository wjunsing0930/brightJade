use crate::CmdExector;
user super::verify_file;

#[derive(Debug,Clone,Copy)]
pub enum OutputFormat{
    Jons, 
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts{
    #[arg(short,long,value_parser=verify_file)] 
    pub input : String,
    #[arg(short,long)]
    pub output: Option<String>,
    #[arg(long , value_parser=parse_format, default_value="json")]
    pub format:OutputFormat,
}

pub fn parse_format(format:&str) -> Result<OutputFormat,anyhow::Error>{
    format.parse()
}


