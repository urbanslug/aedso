//! Generate EDS.
//!
//! VCF is 1 indexed
//!

mod cli;
mod types;
mod generate;

use std::time::Instant;

fn main() {
    let total_time = Instant::now();

    let config: types::AppConfig = cli::start();
    let verbosity = config.verbosity;

    if verbosity > 0 {
        eprintln!("{:#?}", config)
    }

    generate::generate(&config).expect("[main::main] Problem generating eds");

    if verbosity > 1 {
        eprintln!(
            "[aedso::main] all done. Total time taken {} minutes.",
            total_time.elapsed().as_millis() as f64 / 1000.0 / 60.0
        )
    }
}
