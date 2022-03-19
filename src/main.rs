///! VCF is 1 indexed
use needletail::{parse_fastx_file, Sequence, FastxReader};

use itertools::intersperse;
use vcf::{VCFReader, U8Vec, VCFHeaderFilterAlt, VCFError, VCFRecord};
use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::BufReader;

fn loop_vcf() -> Result<(), VCFError> {
    let filename = "/home/sluggie/data/1000-genomes/Homo_sapiens.GRCh38.dna.chromosome.1.fa";
    let mut reader = parse_fastx_file(&filename).expect("valid path/file");
    let seq_record = reader.next().expect("end of iter").expect("invalid record");
    let num_bases = seq_record.num_bases();
    let seq = seq_record.seq();

    let bar = ProgressBar::new(num_bases as u64);
    let template = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}]  {pos:>7}/{len:7} ({eta_precise})";
    bar.set_style(ProgressStyle::default_bar()
                  .template(template)
                  .progress_chars("=> "));


    let mut buffer = BufWriter::new(File::create("foo.eds")?);

    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(
        "/home/sluggie/data/1000-genomes/homo_sapiens-chr1.vcf.bgz",
    )?)))?;


    let mut start = 0;
    let mut stop = 0;
    // prepare VCFRecord object
    let mut vcf_record = reader.empty_record();
    let mut counter = 0;

    loop {
        // TODO: handle errors
        match reader.next_record(&mut vcf_record) {
            Ok(false) => break,
            Ok(true) => (),
            Err(e) => {
                dbg!(e);
                continue
            },
        }

        counter += 1;
        // dbg!(counter);
        // eprintln!("{} {:?} {:?}", vcf_record.position, vcf_record.reference, vcf_record.alternative );
        stop = vcf_record.position - 1 ;
        //eprintln!("{}",std::str::from_utf8(&vcf_record.reference).unwrap());

        buffer.write(&seq[start as usize..stop as usize]).unwrap();
        buffer.write(b"{").unwrap();

        // TODO: handle more than one base on the ref and alt

        let mut x: Vec<Vec<u8>> = vec![ vcf_record.reference.clone() ]; // ref
        let mut y: Vec<u8> = b",".to_vec(); // comma

        x.extend_from_slice(&vcf_record.alternative);
        let x = intersperse(x, y);
        for i in x {
            // eprint!("{}", std::str::from_utf8(&i).unwrap() );
            buffer.write(&i); // alt
        }
        // eprintln!();

        buffer.write(b"}").unwrap();
        buffer.flush().unwrap();

        // copy the file over
        // seq_record.write(&mut buffer, None).unwrap();
        let _ref_allele_len = vcf_record.reference.len() as u64;
        let delta = stop-start;

        start = stop;

        // dbg!(delta);
        bar.inc(delta);
        //start = stop + ref_allele_len;

    }

    Ok(())
}



fn loop_fasta() {
    let filename = "/home/sluggie/data/1000-genomes/Homo_sapiens.GRCh38.dna.chromosome.1.fa";

    let mut reader = parse_fastx_file(&filename).expect("valid path/file");
    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        for base in seqrec.seq().iter().take(10) {
            eprint!("{}", base);
        }
    }
}

fn main() {
    //loop_fasta();
    loop_vcf().unwrap();
    ()
}
