use clap::{Args, Parser, Subcommand};
use core::fmt;
use std::{path::Path, str::FromStr};
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

    #[command(name = "genpass", about = "生成密码")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Args)]
pub struct CvsOpts {
    #[arg(short, long, value_parser=validate_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser=parse_format, default_value="json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Args)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = false)]
    pub uppercase: bool,

    #[arg(long, default_value_t = false)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = false)]
    pub symbol: bool,
}

fn validate_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into()) // &str -> String
    } else {
        Err("File does not exist")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v => anyhow::bail!("Unsupported format: {}", v),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
