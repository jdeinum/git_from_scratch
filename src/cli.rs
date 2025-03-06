use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initializes a new git repository in the current directory
    Init {},
    CatFile {
        /// Pretty Print
        #[arg(short = 'p')]
        pretty_print: bool,
        hash: String,
    },
    HashFile {
        /// Write to object store
        #[arg(short = 'w')]
        write_to_store: Option<bool>,
        filename: String,
    },
    LsTree {
        /// Only print the names of the files
        #[arg(long = "name-only")]
        name_only: Option<bool>,
        hash: String,
    },
    WriteTree {},
}
