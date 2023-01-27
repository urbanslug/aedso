//! Command Line Interface

use crate::types::AppConfig;
use clap::{Arg, Command};
use std::env;

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
                .help("Path to input VCF file (gzip or plaintext)"),
        )
        .arg(
            Arg::new("output_line_length")
                .short('l')
                .long("output-line-length")
                .multiple_values(false)
                .default_value("80")
                .help("Max length of lines in eds"),
        )
        .arg(
            Arg::new("region_start")
                .short('s')
                .long("region-start")
                .multiple_values(false)
                .takes_value(true)
                .default_value("0")
                .help("Region to start creating ED-string from"),
        )
        .arg(
            Arg::new("region_end")
                .short('e')
                .long("region-end")
                .takes_value(true)
                .multiple_values(false)
                .help("Region to finish creating ED-string at"),
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
    let region_start: usize = matches
        .value_of("region_start")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let region_end: Option<usize> = match matches.value_of("region_end") {
        Some(v) => Some(v.parse::<usize>().unwrap()),
        _ => None,
    };
    let verbosity: u8 = matches.occurrences_of("v") as u8;
    let output_line_length: usize = matches
        .value_of("output_line_length")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    AppConfig {
        fasta: String::from(fasta),
        vcf: String::from(vcf),
        region_start,
        region_end,
        verbosity,
        output_line_length,
    }
}
