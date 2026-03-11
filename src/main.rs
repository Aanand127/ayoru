#[tokio::main]
async fn main() {
    let args = match ani::args::parse_from(std::env::args_os()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(2);
        }
    };

    let query = args.query.join(" ");
    let provider = ani::provider::allanime::AllAnimeProvider;
    let runtime = ani::app::SystemPlayerRuntime;

    if let Err(err) = ani::app::run_with(&query, &provider, &runtime).await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
