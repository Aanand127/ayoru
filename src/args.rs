use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ani", version, about = "Fast anime CLI")]
pub struct Args {
    #[arg(required = true)]
    pub query: Vec<String>,
}

pub fn parse_from<I, T>(itr: I) -> Result<Args, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    Args::try_parse_from(itr)
}
