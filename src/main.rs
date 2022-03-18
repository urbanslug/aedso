///! VCF is 1 indexed
use needletail::{parse_fastx_file, Sequence, FastxReader};

use itertools::intersperse;
use vcf::{VCFReader, U8Vec, VCFHeaderFilterAlt, VCFError, VCFRecord};
use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;

fn loop_vcf() -> Result<(), VCFError> {
    let filename = "/home/sluggie/data/1000-genomes/Homo_sapiens.GRCh38.dna.chromosome.1.fa";
    let mut reader = parse_fastx_file(&filename).expect("valid path/file");
    let seq_record = reader.next().expect("mom").expect("dad");
    let seq = seq_record.seq();


    let mut buffer = BufWriter::new(File::create("foo.eds")?);

    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(
        "/home/sluggie/data/1000-genomes/homo_sapiens-chr1.vcf.bgz",
    )?)))?;


    let mut start = 0;
    let mut stop = 0;
    // prepare VCFRecord object
    let mut vcf_record = reader.empty_record();
    let mut counter = 0;
    while counter < 20 && reader.next_record(&mut vcf_record).expect("What?") {
        counter += 1;
        // dbg!(counter);
        // eprintln!("{} {:?} {:?}", vcf_record.position, vcf_record.reference, vcf_record.alternative );
        stop = vcf_record.position - 1 ;


        buffer.write(&seq[start as usize..stop as usize]).unwrap();
        buffer.write(b"{").unwrap();

        // TODO: handle more than one base on the ref and alt

        let mut x: Vec<Vec<u8>> = vec![vec![seq[stop as usize]]]; // ref
        let mut y: Vec<u8> = b",".to_vec(); // comma

        x.extend_from_slice(&vcf_record.alternative);
        let x = intersperse(x, y);
        for i in x {
            eprint!("{}", std::str::from_utf8(&i).unwrap() );
            buffer.write(&i); // alt

        }
        eprintln!();

        buffer.write(b"}").unwrap();
        buffer.flush().unwrap();

        // copy the file over
        // seq_record.write(&mut buffer, None).unwrap();
        start = stop;
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
