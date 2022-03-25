//! Index the VCF

use crate::types;
use fbox::ux;
use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::BufReader;
use vcf::{VCFError, VCFReader};

pub fn index(num_bases: usize, config: &types::AppConfig) -> Result<types::Index, VCFError> {
    let verbosity = config.verbosity;

    // ------------
    // Progress bar
    // ------------
    let bar = ux::progress_bar(num_bases as u64);

    let mut index = types::Index::new();

    // ---------
    // Parse VCF
    // ---------
    if verbosity > 1 {
        eprintln!("[index::index] Parsing VCF");
    }

    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(
        &config.vcf,
    )?)))?;

    if verbosity > 2 {
        eprintln!("{0:two_spaces$}Done parsing VCF.", "", two_spaces=2);
    }

    let mut vcf_record = reader.empty_record();
    let mut cursor = 0;

    let mut max_variant_capacity: usize = 0;

    // --------------
    // Generate index
    // --------------
    if verbosity > 1 {
        eprintln!("[index::index] Indexing VCF");
    }

    loop {
        // TODO: handle errors
        match reader.next_record(&mut vcf_record) {
            Ok(false) => break,
            Ok(true) => (),
            Err(e) => {
                eprintln!("[index::index] skipping invalid record {e}");
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
