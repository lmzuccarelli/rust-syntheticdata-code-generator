// module schema
use clap::{Parser, ValueEnum};

/// cli struct
#[derive(Parser, Debug)]
#[command(name = "rust-synthetic-data-generator")]
#[command(author = "Luigi Mario Zuccarelli <luzuccar@redhat.com>")]
#[command(version = "0.3.0")]
#[command(about = "Used to generate synthetic data for neural network tarining and inference", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// config file to use
    #[arg(short, long, value_name = "config")]
    pub config: String,

    /// set the loglevel. Valid arguments are info, debug, trace
    #[arg(value_enum, long, value_name = "loglevel", default_value = "info")]
    pub loglevel: Option<String>,

    /// set the mode flag.
    #[arg(value_enum)]
    pub mode: Mode,

    /// set the dry-run flag.
    #[arg(long, value_name = "dry-run", default_value = "false")]
    pub dry_run: Option<bool>,

    /// set the name (used in generate).
    #[arg(long, value_name = "name")]
    pub name: Option<String>,

    /// set the count (number of records to generate).
    #[arg(long, value_name = "count")]
    pub count: Option<usize>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    /// Execute generate synthetic code (used to generate data)
    Generate,
    /// Execute the genertaed code.
    Execute,
}
