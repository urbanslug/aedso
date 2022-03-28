//! Input-Output functions

use crate::types;
use fbox::ux;
use itertools::intersperse;
use std::io::{self, Write};
use vcf::{VCFError, VCFReader};
use std::io::BufReader;
use std::fs::File;
use flate2::read::MultiGzDecoder;

// ---
// VCF
// ---
pub fn get_reader_pt(fp: &str) -> Result<VCFReader<BufReader<File>>, VCFError> {
    let f = File::open(fp).unwrap();
    let b = BufReader::new(f);

    VCFReader::new(b)
}

pub fn get_reader_gz(fp: &str) -> Result<VCFReader<BufReader<MultiGzDecoder<File>>>, VCFError> {
    let f = File::open(fp).unwrap();
    let x = MultiGzDecoder::new(f);
    let b = BufReader::new(x);

    VCFReader::new(b)
}


// TODO: should we only write to stdout?
pub fn write_eds(config: &types::AppConfig, num_bases: usize, seq: &[u8], index: &types::Index) {
    let verbosity = config.verbosity;

    if verbosity > 1 {
        eprintln!("[io::write_eds] Writing EDS");
    }

    let bar = ux::progress_bar(num_bases as u64);

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
            handle.write_all(b"\n").expect("[io::io] Failed to add newline");
            faux_beginning += config.output_line_length;
        }
        handle.write_all(&seq[faux_beginning..end]).unwrap();
        handle.write_all(b"\n").expect("[io::io] Failed to add newline");

        let variants: &Vec<Vec<u8>> = index
            .data
            .get(pos)
            .unwrap_or_else(|| panic!("[io::io] index error pos {}", pos));

        let variants = intersperse(variants, &comma);

        handle.write_all(b"{").unwrap();
        for i in variants {
            handle
                .write_all(i)
                .unwrap_or_else(|_| panic!("[io::io] error writing {}", pos));
        }
        handle.write_all(b"}").unwrap();

        let delta = num::abs_sub(end as i64, begining as i64) as u64;
        bar.inc(delta);

        begining = end;
    }

    let last: usize = *index.positions.last().expect("[io::io] could not get last position") - 1;

    // write the last bit
    handle.write_all(&seq[last..num_bases]).unwrap();
}
