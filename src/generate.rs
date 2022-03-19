use crate::types;

use std::time::Instant;
use needletail::{parse_fastx_file};

use itertools::intersperse;
use vcf::{VCFReader, VCFError};
use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::BufReader;
use std::collections::HashSet;
use num;
use std::cmp::Ordering;

pub fn gen_index(num_bases: usize, _config: &types::AppConfig) -> Result<types::Index, VCFError> {

    // ------------
    // Progress bar
    // ------------
    let bar = ProgressBar::new(num_bases as u64);
    let template = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}]  {pos:>7}/{len:7}  ({eta_precise})";
    bar.set_style(ProgressStyle::default_bar()
                  .template(template)
                  .progress_chars("=> "));


    let mut index = types::Index::new(num_bases);

    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(
        "/home/sluggie/data/1000-genomes/homo_sapiens-chr1.vcf.bgz",
    )?)))?;

    let mut vcf_record = reader.empty_record();
    let mut cursor = 0;

    // TODO: remove
    let mut counter = 0;


    loop {
        // TODO: handle errors
        match reader.next_record(&mut vcf_record) {
            Ok(false) => break,
            Ok(true) => (),
            Err(e) => {
                eprintln!("[generate::generate] skipping invalid record {e}");
                continue
            },
        }

        counter += 1;

        if counter > 1_000_000 {
            break
        }

        let position = vcf_record.position as usize;

        if position > num_bases {
            eprintln!("{} {:?} {:?}", vcf_record.position, vcf_record.reference, vcf_record.alternative );
            break;
        }

        let mut foo: Vec<types::U8Vec> = vec![ vcf_record.reference.clone() ];
        foo.extend_from_slice(&vcf_record.alternative);

        index.positions_bin[position] = 1;
        index.positions.push(position);
        match index.data.get_mut(&position) {
            Some(s) => {
                s.insert(foo);
            },
            _ => {
                let a = HashSet::from([
                    foo
                ]);

                index.data.insert(position, a);
            }
        };

        let delta = position - cursor;
        bar.inc(delta as u64);

        cursor = position;
    }

    Ok(index)
}

fn handle_fasta(config: &types::AppConfig) {
    let verbosity = config.verbosity;

    // ------------
    // Fasta
    // ------------
    if verbosity > 1 {
        eprintln!("Processing Fasta.");
    }
    let filename = &config.fasta;
    let mut reader = parse_fastx_file(&filename)
        .expect(&format!("[generate::generate] invalid fasta path/file {}", filename));
    let seq_record = reader
        .next()
        .expect("[generate::generate] end of iter")
        .expect("[generate::generate] invalid record");

    let seq = seq_record.seq();
    let num_bases = seq.len();

    todo!()
}

fn handle_vcf(config: &types::AppConfig) {
    todo!()
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
    let filename = "/home/sluggie/data/1000-genomes/Homo_sapiens.GRCh38.dna.chromosome.1.fa";
    let mut reader = parse_fastx_file(&filename)
        .expect(&format!("[generate::generate] invalid fasta path/file {}", filename));
    let seq_record = reader
        .next()
        .expect("[generate::generate] end of iter")
        .expect("[generate::generate] invalid record");

    let seq = seq_record.seq();
    let num_bases = seq.len();

    if verbosity > 2 {
        eprintln!("Done processing fasta. \n\
                   Number of bases: {}. \n\
                   Time taken {} seconds.",
                  num_bases,
                  now.elapsed().as_millis() as f64 / 1000.0
        );
    }

    // ------------
    // VCF
    // ------------
    if verbosity > 1 {
        eprintln!("Parsing VCF");
    }

    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(
        "/home/sluggie/data/1000-genomes/homo_sapiens-chr1.vcf.bgz",
    )?)))?;

    if verbosity > 2 {
        eprintln!("Done parsing VCF.");
    }


    if verbosity > 1 {
        eprintln!("Indexing VCF");
    }
    let now = Instant::now();

    let index = gen_index(num_bases, config);

    if verbosity > 2 {
        eprintln!("Done indexing VCF. Time taken {} seconds.",
                  now.elapsed().as_millis() as f64 / 1000.0);
    }

    // ------------
    // Progress bar
    // ------------
    let bar = ProgressBar::new(num_bases as u64);
    let template = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}]  {pos:>7}/{len:7}  ({eta_precise})";
    bar.set_style(ProgressStyle::default_bar()
                  .template(template)
                  .progress_chars("=> "));

    // ------------
    // Output
    // ------------
    let mut buffer = BufWriter::new(File::create("foo.eds")?);

    // ------------
    // Generate EDS
    // ------------
    let mut start = 0;
    let mut stop = 0;

    // TODO: remove
    let mut counter = 0;

    let dict = index.expect("Incorrect index").data;

    for pos in 0..num_bases {
        match dict.get(&pos) {
            Some(_) => {},
            _ => {},
        }
    }

    // prepare VCFRecord object
    let mut vcf_record = reader.empty_record();


    loop {
        // TODO: handle errors
        match reader.next_record(&mut vcf_record) {
            Ok(false) => break,
            Ok(true) => (),
            Err(e) => {
                eprintln!("[generate::generate] skipping invalid record {e}");
                continue
            },
        }

        if counter > 20 {
            break
        }

        counter += 1;
        // dbg!(counter);
        // eprintln!("{} {:?} {:?}", vcf_record.position, vcf_record.reference, vcf_record.alternative );

        stop = vcf_record.position - 1 ;
        // eprintln!("{}",std::str::from_utf8(&vcf_record.reference).unwrap());

        // solid string
        match start.cmp(&stop) {
            Ordering::Greater => {
                eprintln!("[generate::generate] found another variant at pos {}, being sloppy", vcf_record.position);
            },
            Ordering::Less => {
                buffer.write(&seq[start as usize..stop as usize]).unwrap();
            },
            _ => {} // ignore equality
        }

        // start degenerate letter
        buffer.write(b"{").unwrap();

        // Handle more than one alt
        let mut x: Vec<Vec<u8>> = vec![ vcf_record.reference.clone() ]; // ref
        let y: Vec<u8> = b",".to_vec(); // comma

        x.extend_from_slice(&vcf_record.alternative);
        let x = intersperse(x, y);
        for i in x {
            // eprint!("{}", std::str::from_utf8(&i).unwrap());
            buffer.write(&i).expect("[generate::generate] error writing {i}"); // alt
        }
        // eprintln!();

        buffer.write(b"}").unwrap();
        buffer.flush().unwrap();

        let ref_allele_len = vcf_record.reference.len() as u64;
        let delta = num::abs_sub(stop as i64, start as i64) as u64 + ref_allele_len;
        bar.inc(delta);

        start = stop + ref_allele_len;

        dbg!(start, stop);
    }

    // write the last bit
    buffer.write(&seq[stop as usize..num_bases]).unwrap();

    Ok(())
}
