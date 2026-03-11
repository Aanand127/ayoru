fn main() {
    match ani::args::parse_from(std::env::args_os()) {
        Ok(args) => {
            let query = args.query.join(" ");
            println!("{query}");
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(2);
        }
    }
}
