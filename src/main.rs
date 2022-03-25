//! Generate EDS.
//!
//! VCF is 1 indexed i.e. genome loci are started counting from zero.
//! Expects a single sequence in a fasta file and a VCF from one sequence.
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
        eprintln!("{}", config)
    }

    let verbosity = config.verbosity;

    // ------------
    // Fasta
    // ------------
    if verbosity > 1 {
        eprintln!("[aedso::main] Processing Fasta.");
    }

    let now = Instant::now();
    let mut reader = parse_fastx_file(&config.fasta).unwrap_or_else(|_| {
        panic!(
            "[aedso::main] invalid fasta path/file {}",
            config.fasta
        )
    });
    let seq_record = reader
        .next()
        .expect("[aedso::main] end of iter")
        .expect("[aedso::main] invalid record");

    let seq = seq_record.seq();
    let num_bases = seq.len();

    if verbosity > 2 {
        eprintln!(
            "{0:two_spaces$}Done processing fasta. \n\
             {0:four_spaces$}Number of bases: {bases}. \n\
             {0:four_spaces$}Time taken {time} seconds.",
            "",
            bases = num_bases,
            time = now.elapsed().as_millis() as f64 / 1000.0,
            two_spaces=2, four_spaces=4
        );
    }

    // --------------
    // Generate index
    // --------------

    let now = Instant::now();

    let index = index::index(num_bases, &config).expect("Incorrect index");

    if verbosity > 2 {
        eprintln!(
            "{0:two_spaces$}Done indexing VCF. Time taken {time} seconds.",
            "",
            time=now.elapsed().as_millis() as f64 / 1000.0,
            two_spaces=2
        );
    }

    // ------------
    // Generate EDS
    // ------------
    let now = Instant::now();

    io::write_eds(&config, num_bases, &seq, &index);

    if verbosity > 2 {
        eprintln!(
            "{0:two_spaces$}Done writing EDS. Time taken {time} seconds.",
            "",
            time=now.elapsed().as_millis() as f64 / 1000.0,
            two_spaces=2
        );
    }

    if verbosity > 1 {
        // time in seconds
        let time = total_time.elapsed().as_millis() as f64 / 1000.0;

        let time_str = if time > 60.0 {
            format!("{} minutes", time/60.0)
        } else {
            format!("{} seconds", time)
        };

        eprintln!("[aedso::main] all done. Total time taken {}.", time_str);
    }

    Ok(())
}
