extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("stockcli")
        .version("0.1.0")
        .author("Stephen Cirner <scirner22@gmail.com>")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help(
                    "Path to file containing comma separated list \
                     of ticker symbols (defaults to ~/.stockcli/config.csv)",
                ),
        )
        .arg(
            Arg::with_name("symbols")
                .short("s")
                .long("symbols")
                .takes_value(true)
                .help("Comma separated list of ticker symbols (overides --config)"),
        )
        .get_matches();

    if let Some(symbols) = matches.value_of("symbols") {
        let symbols: Vec<&str> = symbols.split(',').collect();
        print!("symbols {:?}", symbols);
    } else {
        let config = matches
            .value_of("config")
            .unwrap_or("~/.stockcli/config.csv");
        print!("config {}", config);
    }
}
