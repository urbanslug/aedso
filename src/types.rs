//! Config, convenience types & Index

use std::collections::HashMap;
use std::fmt;

// ---------
// Constants
// ---------
// Expected row count
const NUM_ROWS: usize = 1_000_000;

// ----------
// App config
// ----------
#[derive(Debug)]
pub struct AppConfig {
    pub fasta: String,
    pub vcf: String,
    pub verbosity: u8,
    pub output_line_length: usize,
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Config {{\n\
                   \tfasta: {},\n\
                   \tvcf: {}\n\
                   \toutput line length: {}\n\
                   \tverbosity: {}\n\
                   }}",
            self.fasta, self.vcf, self.output_line_length, self.verbosity
        )
    }
}

// -----------
// Convenience
// -----------
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
