use {
    argh::FromArgs,
    std::path::PathBuf,
};

#[derive(Debug, FromArgs)]
/// apply patches to YAML files
pub struct Args {

    /// print the version
    #[argh(switch, short = 'v')]
    pub version: bool,

    /// base file
    #[argh(option, short = 'b')]
    pub base: PathBuf,

    /// patch file, containing the changes
    #[argh(option, short = 'p')]
    pub patch: PathBuf,
}
