use clap::{Args, Parser, Subcommand};
use std::path::Path;
// rcli cvs -i xxx.csv -o xxx.json
#[derive(Parser, Debug)] // 这是一个过程宏，用于自动为结构体Cli实现clap::Parser trait
#[command(name="rcli",version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CvsOpts),
}

#[derive(Debug, Args)]
pub struct CvsOpts {
    #[arg(short, long, value_parser=validate_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn validate_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into()) // &str -> String
    } else {
        Err("File does not exist")
    }
}
