//! Index the VCF

use crate::types;
use crate::io;
use fbox::ux;
use std::io::BufReader;
use vcf::{VCFError, VCFReader, VCFRecord};



// -----
// Index
// -----
fn loop_records<T: std::io::Read>(
    seq_name: &[u8],
    mut reader: VCFReader<BufReader<T>>,
    num_bases: usize,
    config: &types::AppConfig,
) -> Result<types::Index, VCFError>
{
    let verbosity = config.verbosity;

    // --------------
    // Generate index
    // --------------
    if verbosity > 1 {
        eprintln!("[index::index] Indexing VCF");
    }

    let mut vcf_record = reader.empty_record();
    let mut index = types::Index::new();

    // ------------
    // Progress bar
    // ------------
    let bar = ux::progress_bar(num_bases as u64);
    let mut cursor: u64 = 0;

    loop {
        // TODO: handle errors
        match reader.next_record(&mut vcf_record) {
            Ok(false) => break,
            Ok(true) => {
                if vcf_record.chromosome != seq_name {
                    continue;
                }
            },
            Err(e) => {
                if verbosity > 2 {
                    eprintln!("[index::index] skipping invalid record {e}");
                }
                continue;
            }
        }

        let position = vcf_record.position;

        if index_record(&vcf_record, num_bases,&mut index).is_err() {
            break;
        }

        // Progress bar
        let delta = position - cursor;
        bar.inc(delta);

        cursor = position;
    }

    Ok(index)
}


fn index_record(
    vcf_record: &VCFRecord,
    num_bases: usize,
    index: &mut types::Index
) -> Result<(), String>
{
    let position = vcf_record.position as usize;

    if position > num_bases {
        eprintln!(
            "{} {:?} {:?}",
            vcf_record.position, vcf_record.reference, vcf_record.alternative
        );
        return Err(String::from("Position is beyond number of bases"));
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

    Ok(())
}


// -----------
// Handle file
// -----------
pub fn index(
    seq_name: &[u8],
    num_bases: usize,
    config: &types::AppConfig
) -> Result<types::Index, VCFError> {
    let f = &config.vcf;
    let maybe_index: Result<types::Index, VCFError> = match io::get_reader_gz(f) {
        Ok(r) => {
            loop_records(seq_name, r, num_bases, config)
        },
        Err(_) => {
            match io::get_reader_pt(f) {
                Ok(r) => loop_records(seq_name, r, num_bases, config),
                Err(e) => {
                    panic!("[aesdo::index] Unable to parse VCF {} {}", f, e);
                }
            }
        }
    };

    maybe_index
}
