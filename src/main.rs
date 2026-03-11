#[tokio::main]
async fn main() {
    let args = match ayoru::args::parse_from(std::env::args_os()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(2);
        }
    };

    match args.command {
        ayoru::args::Command::Tui => {
            if let Err(err) = ayoru::tui::run().await {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
        ayoru::args::Command::Play { query } => {
            let query = query.join(" ");
            let provider = ayoru::provider::allanime::AllAnimeProvider::new();
            let runtime = ayoru::app::SystemPlayerRuntime;
            let picker = ayoru::cli::interactive::InteractivePickerRuntime;

            if let Err(err) = ayoru::app::run_with(&query, &provider, &runtime, &picker).await {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
    }
}
