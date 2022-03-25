use crate::types;
use fbox::ux;
use itertools::intersperse;
use std::io::{self, Write};
use std::time::Instant;

// TODO: should we only write to stdout?
pub fn write_eds(config: &types::AppConfig, num_bases: usize, seq: &[u8], index: &types::Index) {
    let verbosity = config.verbosity;

    if verbosity > 1 {
        eprintln!("[io::write_eds] Writing EDS");
    }

    // ------------
    // Progress bar
    // ------------
    let bar = ux::progress_bar(num_bases as u64);

    // ------------
    // Generate EDS
    // ------------
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
}
