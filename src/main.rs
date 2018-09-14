extern crate clap;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate prettytable;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use clap::{App, Arg};
use hyper::rt::{self, Future};
use prettytable::{cell::Cell, row::Row, Table};

mod iex;

fn color_code<'a>(num: f32) -> &'a str {
    if num > 0.0 {
        "Fg"
    } else if num < 0.0 {
        "Fr"
    } else {
        ""
    }
}

fn main() {
    let mut default_config = env::home_dir().unwrap();
    default_config.push(".stockcli");
    default_config.push("config");
    default_config.set_extension("csv");

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

    let symbols: Vec<String>;
    if let Some(s) = matches.value_of("symbols") {
        symbols = s.split(',').map(String::from).collect();
    } else {
        let config = matches
            .value_of("config")
            .unwrap_or_else(|| default_config.to_str().unwrap());
        let mut file = File::open(config).expect("config file not found");
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        symbols = s
            .trim()
            .split(',')
            .map(|s| String::from(s.trim()))
            .collect();
    }

    let client = iex::IexClient::new();
    let futures = symbols.into_iter().map(move |sym| {
        let client = client.clone();
        client.fetch_stock(&sym)
    });
    let f = futures::future::join_all(futures)
        .map(|results| {
            let mut table = Table::new();
            table.add_row(row!["SYMBOL", "PRICE", "DAILY", "YTD"]);
            for res in results {
                let daily_change = format!("{:.2}%", res.daily_percentage());
                let style = color_code(res.daily_percentage());
                let daily_change = Cell::new(daily_change.as_str()).style_spec(style);
                let ytd_change = format!("{:.2}%", res.ytd_percentage());
                let style = color_code(res.ytd_percentage());
                let ytd_change = Cell::new(ytd_change.as_str()).style_spec(style);
                table.add_row(Row::new(vec![
                    Cell::new(res.get_symbol()),
                    Cell::new(format!("{:.2}", res.delayed_price).as_str()),
                    daily_change,
                    ytd_change,
                ]));
            }
            table.printstd();
        })
        .map_err(|err| println!("{:?}", err));

    rt::run(f);
}
