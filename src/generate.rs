use crate::types;

use std::time::Instant;
use needletail::{parse_fastx_file};

use itertools::intersperse;
use vcf::{VCFReader, VCFError};
use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::{self, Write};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::BufReader;
use std::collections::HashSet;
use num;

pub fn gen_index(
    num_bases: u32,
    config: &types::AppConfig
) -> Result<types::Index, VCFError>
{
    let verbosity = config.verbosity;

    // ------------
    // Progress bar
    // ------------
    let bar = ProgressBar::new(num_bases as u64);
    let template = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}]  {pos:>7}/{len:7}  ({eta_precise})";
    bar.set_style(ProgressStyle::default_bar()
                  .template(template)
                  .progress_chars("=> "));

    let mut index = types::Index::new();


    // ------------
    // Parse VCF
    // ------------
    if verbosity > 1 {
        eprintln!("Parsing VCF");
    }

    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(&config.vcf)?)))?;

    if verbosity > 2 {
        eprintln!("Done parsing VCF.");
    }

    let mut vcf_record = reader.empty_record();
    let mut cursor = 0;

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

        let position = vcf_record.position as u32;

        if position > num_bases {
            eprintln!("{} {:?} {:?}", vcf_record.position, vcf_record.reference, vcf_record.alternative );
            break;
        }

        let mut foo: HashSet<Vec<u8>> = HashSet::from([vcf_record.reference.clone()]);
        for alt in &vcf_record.alternative {
            foo.insert(alt.to_vec());
        }

        index.positions.push(position);
        match index.data.get_mut(&position) {
            Some(s) => {
                for f in foo {
                    s.insert(f);
                }
            },
            _ => {
                index.data.insert(position, foo);
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
    let mut reader = parse_fastx_file(&config.fasta)
        .expect(&format!("[generate::generate] invalid fasta path/file {}", config.fasta));
    let seq_record = reader
        .next()
        .expect("[generate::generate] end of iter")
        .expect("[generate::generate] invalid record");

    let seq = seq_record.seq();
    let num_bases = seq.len() as u32;

    if verbosity > 2 {
        eprintln!("Done processing fasta. \n\
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
    // Generate EDS
    // ------------
    if verbosity > 1 {
        eprintln!("Writing EDS");
    }
    let now = Instant::now();

    let stdout = io::stdout();

    // let mut buffer = BufWriter::new(File::create("foo.eds")?);
    let mut handle = stdout.lock();

    let mut begining: usize = 0;
    let mut end: usize = 0;
    let comma: Vec<u8> = Vec::from([b',']); // comma
    //let newline: Vec<u8> = Vec::from([b'\n']); // newline

    for pos in &index.positions {
        end = *pos as usize - 1;


        // buffer.write(&seq[begining..end]).unwrap();
        let mut faux_beginning = begining as usize;
        while faux_beginning + 50 < end {
            handle.write_all(&seq[faux_beginning..faux_beginning+50]).unwrap();
            handle.write_all(b"\n").expect("Failed to add newline");
            faux_beginning += 50;
        }
        handle.write_all(&seq[faux_beginning..end]).unwrap();
        handle.write_all(b"\n").expect("Failed to add newline");


        let variants: &HashSet<Vec<u8>> = index
            .data
            .get(&pos)
            .expect(&format!("[generate::generate] index error pos {}", pos));

        let variants = intersperse(variants, &comma);

        handle.write_all(b"{").unwrap();
        for i in variants {
            handle.write_all(&i).expect(&format!("[generate::generate] error writing {}", pos));
        }
        handle.write_all(b"}").unwrap();

        let delta = num::abs_sub(end as i64, begining as i64) as u64;
        bar.inc(delta);

        begining = end;
    }

    let last: u32 = *index
        .positions
        .last()
        .expect("Could not get last position") - 1;

    // write the last bit
    handle.write_all(&seq[last as usize..num_bases as usize]).unwrap();

    if verbosity > 2 {
        eprintln!("Done writing EDS. \n\
                   Time taken {} seconds.",
                  now.elapsed().as_millis() as f64 / 1000.0
        );
    }

    Ok(())
}
