use structopt::StructOpt;
use std::vec;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "whereis Options", about = "Different Options for whereis CLI command.")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "b")]
    debug: bool,
    /// Search for binaries
    #[structopt(short = "b", parse(from_os_str))]
    binaries: PathBuf,
    /// Search for manuals
    #[structopt(short = "m", parse(from_os_str))]
    manuals: PathBuf,
    /// Search for sources
    #[structopt(short = "m", parse(from_os_str))]
    sources: PathBuf,
    // Search for Binaries limitly in a list of directories seperated with whitespaces.
    #[structopt(short = "B", parse(from_os_str))]
    binariesList: Vec<PathBuf>,
    // Search for Manuals limitly in a list of directories seperated with whitespaces.
    #[structopt(short = "M", parse(from_os_str))]
    manualsList: Vec<PathBuf>,
    // Search for Sources limitly in a list of directories seperated with whitespaces.
    #[structopt(short = "S", parse(from_os_str))]
    sourcesList: Vec<PathBuf>,
}

fn main() {
    let args = Opt::from_args();
}
