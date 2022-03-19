use clap::{Command, Arg};
use std::env;
use crate::types::AppConfig;

// Env vars
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub fn start() -> AppConfig {
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(
            Arg::new("fasta")
                .required(true)
                .takes_value(true)
                .help("Path to input fasta file"),
        )
        .arg(
            Arg::new("vcf")
                .required(true)
                .takes_value(true)
                .help("Path to input VCF file"),
        )
        .arg(
            Arg::with_name("output_line_length")
                .short('l')
                .long("output-line-length")
                .multiple(false)
                .default_value("80")
                .help("Max length of lines in eds"),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .help("Sets the level of verbosity [default: 0]"),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let fasta: &str = matches.value_of("fasta").unwrap();
    let vcf: &str = matches.value_of("vcf").unwrap();
    let verbosity: u8 = matches.occurrences_of("v") as u8;
    let output_line_length: usize = matches
        .value_of("output_line_length")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    AppConfig {
        fasta: String::from(fasta),
        vcf: String::from(vcf),
        verbosity,
        output_line_length
    }
}
