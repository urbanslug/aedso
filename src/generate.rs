use crate::types;

use needletail::parse_fastx_file;
use std::time::Instant;

use flate2::read::MultiGzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::intersperse;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};
use vcf::{VCFError, VCFReader};

pub fn gen_index(num_bases: usize, config: &types::AppConfig) -> Result<types::Index, VCFError> {
    let verbosity = config.verbosity;

    // ------------
    // Progress bar
    // ------------
    let bar = ProgressBar::new(num_bases as u64);
    let template = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}]  {pos:>7}/{len:7}  ({eta_precise})";
    bar.set_style(
        ProgressStyle::default_bar()
            .template(template)
            .progress_chars("=> "),
    );

    let mut index = types::Index::new();

    // ------------
    // Parse VCF
    // ------------
    if verbosity > 1 {
        eprintln!("Parsing VCF");
    }

    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(
        &config.vcf,
    )?)))?;

    if verbosity > 2 {
        eprintln!("Done parsing VCF.");
    }

    let mut vcf_record = reader.empty_record();
    let mut cursor = 0;

    let mut max_variant_capacity: usize = 0;

    loop {
        // TODO: handle errors
        match reader.next_record(&mut vcf_record) {
            Ok(false) => break,
            Ok(true) => (),
            Err(e) => {
                eprintln!("[generate::generate] skipping invalid record {e}");
                continue;
            }
        }

        let position = vcf_record.position as usize;

        if position > num_bases {
            eprintln!(
                "{} {:?} {:?}",
                vcf_record.position, vcf_record.reference, vcf_record.alternative
            );
            break;
        }

        let mut b: Vec<u8> = vcf_record.reference.clone();
        b.shrink_to_fit();
        let mut variants: Vec<Vec<u8>> = Vec::from([b]);
        for alt in &vcf_record.alternative {
            let mut x = alt.to_vec();
            x.shrink_to_fit();
            variants.push(x);
        }

        variants.shrink_to_fit();

        if variants.capacity() > max_variant_capacity {
            max_variant_capacity = variants.capacity();
        }

        index.positions.push(position);
        match index.data.get_mut(&position) {
            Some(v) => {
                for variant in variants {
                    v.push(variant);
                    v.shrink_to_fit();
                }

                v.sort();
                v.dedup();
            }
            _ => {
                index.data.insert(position, variants);
            }
        };

        let delta = position - cursor;
        bar.inc(delta as u64);

        cursor = position;
    }

    Ok(index)
}

// TODO: should we write to stdout?
pub fn generate(config: &types::AppConfig) -> Result<(), VCFError> {
    let verbosity = config.verbosity;

    // ------------
    // Fasta
    // ------------
    if verbosity > 1 {
        eprintln!("Processing Fasta.");
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

    if verbosity > 1 {
        eprintln!("Indexing VCF");
    }
    let now = Instant::now();

    let index = gen_index(num_bases, config).expect("Incorrect index");

    if verbosity > 2 {
        eprintln!(
            "Done indexing VCF. Time taken {} seconds.",
            now.elapsed().as_millis() as f64 / 1000.0
        );
    }

    // ------------
    // Progress bar
    // ------------
    let bar = ProgressBar::new(num_bases as u64);
    let template = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}]  {pos:>7}/{len:7}  ({eta_precise})";
    bar.set_style(
        ProgressStyle::default_bar()
            .template(template)
            .progress_chars("=> "),
    );

    // ------------
    // Generate EDS
    // ------------
    if verbosity > 1 {
        eprintln!("Writing EDS");
    }
    let now = Instant::now();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut begining: usize = 0;
    let mut end: usize;
    let comma: Vec<u8> = Vec::from([b',']); // comma

    for pos in &index.positions {
        end = *pos as usize - 1;

        let mut faux_beginning = begining as usize;
        while faux_beginning + config.output_line_length < end {
            handle
                .write_all(&seq[faux_beginning..faux_beginning + config.output_line_length])
                .unwrap();
            handle.write_all(b"\n").expect("Failed to add newline");
            faux_beginning += config.output_line_length;
        }
        handle.write_all(&seq[faux_beginning..end]).unwrap();
        handle.write_all(b"\n").expect("Failed to add newline");

        let variants: &Vec<Vec<u8>> = index
            .data
            .get(pos)
            .unwrap_or_else(|| panic!("[generate::generate] index error pos {}", pos));

        let variants = intersperse(variants, &comma);

        handle.write_all(b"{").unwrap();
        for i in variants {
            handle
                .write_all(i)
                .unwrap_or_else(|_| panic!("[generate::generate] error writing {}", pos));
        }
        handle.write_all(b"}").unwrap();

        let delta = num::abs_sub(end as i64, begining as i64) as u64;
        bar.inc(delta);

        begining = end;
    }

    let last: usize = *index.positions.last().expect("Could not get last position") - 1;

    // write the last bit
    handle.write_all(&seq[last..num_bases]).unwrap();

    if verbosity > 2 {
        eprintln!(
            "Done writing EDS. Time taken {} seconds.",
            now.elapsed().as_millis() as f64 / 1000.0
        );
    }

    Ok(())
}
