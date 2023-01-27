//! Input-Output functions

use crate::types;
use fbox::ux;
use flate2::read::MultiGzDecoder;
use itertools::intersperse;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};
use vcf::{VCFError, VCFReader};

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

    // let mut prev_pos: Option<usize> = None;
    // eprintln!("found -> {}", index.data.len());

    for pos in &index.positions {
        end = *pos as usize - 1; // because VCF is 1 indexed

        if *pos == 0 {
            panic!("[io::io] Encountered a position of 0. VCF should be 1 indexed.")
        }
        let tru_ref: Vec<u8> = Vec::from([seq[pos - 1]]);

        // eprintln!("{} {} {}", begining, end, pos);
        //eprintln!("{}", prev_pos.unwrap());

        let mut faux_beginning = begining as usize;
        while faux_beginning + config.output_line_length < end {
            handle
                .write_all(&seq[faux_beginning..faux_beginning + config.output_line_length])
                .unwrap();
            /*
            handle
                .write_all(b"\n")
                .expect("[io::io] Failed to add newline");
            */
            faux_beginning += config.output_line_length;
        }

        handle.write_all(&seq[faux_beginning..end]).unwrap();
        /*
        handle
            .write_all(b"\n")
            .expect("[io::io] Failed to add newline");
        */

        let variants: &Vec<Vec<u8>> = index
            .data
            .get(pos)
            .unwrap_or_else(|| panic!("[io::io] index error pos {}", pos));

        let l = variants.iter().any(|v| v.clone() == tru_ref);

        let variants = intersperse(variants, &comma);

        handle.write_all(b"{").unwrap();
        for i in variants {
            handle
                .write_all(i)
                .unwrap_or_else(|_| panic!("[io::io] error writing {}", pos));
        }

        if !l {
            handle.write_all(b",").unwrap();
            handle.write_all(&tru_ref).unwrap();
        }

        handle.write_all(b"}").unwrap();

        let delta = num::abs_sub(end as i64, begining as i64) as u64;
        bar.inc(delta);

        begining = *pos;
    }

    if begining < num_bases {
        // write the last part
        handle.write_all(&seq[begining..num_bases]).unwrap();
    }
}
