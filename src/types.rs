use std::collections::HashMap;
use std::fmt;

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

pub type U8Vec = Vec<u8>;

pub struct Index {
    pub data: HashMap<usize, Vec<U8Vec>>,
    pub positions: Vec<usize>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            data: HashMap::with_capacity(1_000_000),
            positions: Vec::with_capacity(1_000_000),
        }
    }
}
