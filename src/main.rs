use clap::{Arg, Command};
mod bar;
mod get;

fn main() {
    let matches = Command::new("Myget")
        .author("Philip Leake")
        .about("wget clone written in Rust")
        .version("0.1.0")
        .arg(
            Arg::new("URL")
                .required(true)
                .help("URL to download"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .required(false)
                .help("Run in quiet mode, suppressing output"),
        )
        .get_matches();

    let url = matches.get_one::<String>("URL").expect("required");
    let quiet_mode = matches.contains_id("quiet");

    println!("Starting download for: {}", url);

    match get::download(url, quiet_mode) {
        Ok(_) => println!("Download completed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
