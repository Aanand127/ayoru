use clap::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Tui,
    Play { query: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    pub command: Command,
}

#[derive(Parser, Debug)]
#[command(name = "ayoru", version, about = "A quieter way to watch anime.")]
struct RawArgs {
    #[arg()]
    rest: Vec<String>,
}

pub fn parse_from<I, T>(itr: I) -> Result<Args, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let raw = RawArgs::try_parse_from(itr)?;

    match raw.rest.as_slice() {
        [] => Err(clap::Error::raw(
            clap::error::ErrorKind::MissingRequiredArgument,
            "query is required",
        )),
        [command] if command == "tui" => Ok(Args {
            command: Command::Tui,
        }),
        [command, ..] if command == "tui" => Err(clap::Error::raw(
            clap::error::ErrorKind::TooManyValues,
            "ayoru tui does not accept additional arguments",
        )),
        query => Ok(Args {
            command: Command::Play {
                query: query.to_vec(),
            },
        }),
    }
}
