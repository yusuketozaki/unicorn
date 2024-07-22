use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version, author, about, arg_required_else_help = true)]

pub struct CliOpts {
    #[clap(short = 'm', long = "mode", default_value_t = RunMode::Auto, value_name = "MODE", required = false, ignore_case = true, value_enum, help = "Mode of operation.")]
    pub mode: RunMode,
    #[clap(
        short = 'o',
        short_alias = 'd',
        long = "output",
        alias = "dest",
        value_name = "DEST",
        required = false,
        help = "Output file in archive mode, or output directory in extraction mode"
    )]
    pub output: Option<PathBuf>,
    #[clap(
        long = "to-archive-name-dir",
        help = "extract files to DEST/ARCHIVE_NAME directory (extract mode).",
        default_value_t = false
    )]
    pub to_archive_name_dir: bool,
    #[clap(
        short = 'n',
        long = "no-recursive",
        help = "No recursive directory (archive mode).",
        default_value_t = false
    )]
    pub no_recursive: bool,
    #[clap(
        short = 'v',
        long = "verbose",
        help = "Display verbose output.",
        default_value_t = false
    )]
    pub verbose: bool,
    #[clap(long, help = "Overwrite existing files.")]
    pub overwrite: bool,
    #[clap(
        value_name = "ARGUMENTS",
        help = "List of files or directories to be processed."
    )]
    pub args: Vec<PathBuf>,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum RunMode {
    Auto,
    Archive,
    Extract,
}
