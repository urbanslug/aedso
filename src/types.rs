use std::fmt;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct AppConfig {
    pub fasta: String,
    pub vcf: String,
    pub verbosity: u8
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config {{\n\
                   \tfasta: {},\n\
                   \tvcf: {}\n\
                   \tverbosity: {}\n\
                   }}",
               self.fasta, self.vcf, self.verbosity)
    }
}

pub type U8Vec = Vec<u8>;

pub struct Index {
    pub data: HashMap<u32,  HashSet<U8Vec>>,
    pub positions: Vec<u32>,
}


impl Index {
    pub fn new() -> Self {
        Self {
            data: HashMap::with_capacity(1_000_000),
            positions: Vec::with_capacity(1_000_000)
        }
    }
}
