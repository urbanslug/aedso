//! Generate EDS.
//!
//! VCF is 1 indexed
//!

mod cli;
mod index;
mod io;
mod types;

use needletail::parse_fastx_file;
use std::time::Instant;
use vcf::VCFError;

fn main() -> Result<(), VCFError> {
    let total_time = Instant::now();

    let config: types::AppConfig = cli::start();
    let verbosity = config.verbosity;

    if verbosity > 0 {
        eprintln!("{:#?}", config)
    }

    let verbosity = config.verbosity;

    // ------------
    // Fasta
    // ------------
    if verbosity > 1 {
        eprintln!("[generate::generate] Processing Fasta.");
    }

    let now = Instant::now();
    let mut reader = parse_fastx_file(&config.fasta).unwrap_or_else(|_| {
        panic!(
            "[generate::generate] invalid fasta path/file {}",
            config.fasta
        )
    });
    let seq_record = reader
        .next()
        .expect("[generate::generate] end of iter")
        .expect("[generate::generate] invalid record");

    let seq = seq_record.seq();
    let num_bases = seq.len();

    if verbosity > 2 {
        eprintln!(
            "Done processing fasta. \n\
                   Number of bases: {}. \n\
                   Time taken {} seconds.",
            num_bases,
            now.elapsed().as_millis() as f64 / 1000.0
        );
    }

    // --------------
    // Generate index
    // --------------
    if verbosity > 1 {
        eprintln!("Indexing VCF");
    }

    let now = Instant::now();

    let index = index::index(num_bases, &config).expect("Incorrect index");

    if verbosity > 2 {
        eprintln!(
            "Done indexing VCF. Time taken {} seconds.",
            now.elapsed().as_millis() as f64 / 1000.0
        );
    }

    // ------------
    // Generate EDS
    // ------------
    let now = Instant::now();

    io::write_eds(&config, num_bases, &seq, &index);

    if verbosity > 2 {
        eprintln!(
            "Done writing EDS. Time taken {} seconds.",
            now.elapsed().as_millis() as f64 / 1000.0
        );
    }

    if verbosity > 1 {
        eprintln!(
            "[aedso::main] all done. Total time taken {} minutes.",
            total_time.elapsed().as_millis() as f64 / 1000.0 / 60.0
        )
    }

    Ok(())
}
