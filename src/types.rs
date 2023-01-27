//! Config, convenience types & Index

use std::collections::HashMap;
use std::fmt;

// ---------
// Constants
// ---------
/// Expected row count `1_000_000`
const NUM_ROWS: usize = 1_000_000;

// ----------
// App config
// ----------
#[derive(Debug)]
pub struct AppConfig {
    pub fasta: String,
    pub vcf: String,
    pub verbosity: u8,
    pub region_start: usize,
    pub region_end: Option<usize>,
    pub output_line_length: usize,
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Config {{\n\
                   {0:two_spaces$}fasta: {fasta},\n\
                   {0:two_spaces$}vcf: {vcf}\n\
                   {0:two_spaces$}output line length: {l}\n\
                   {0:two_spaces$}verbosity: {v}\n\
                   }}",
            "",
            fasta = self.fasta,
            vcf = self.vcf,
            l = self.output_line_length,
            v = self.verbosity,
            two_spaces = 2
        )
    }
}

// -----------
// Convenience
// -----------
/// `Vec<u8>`
pub type U8Vec = Vec<u8>;

// -------------------
// Index related types
// -------------------
pub struct Index {
    pub data: HashMap<usize, Vec<U8Vec>>,
    pub positions: Vec<usize>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            data: HashMap::with_capacity(NUM_ROWS),
            positions: Vec::with_capacity(NUM_ROWS),
        }
    }
}
