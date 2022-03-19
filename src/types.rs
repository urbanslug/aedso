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
    pub data: HashMap<usize,  HashSet<Vec<U8Vec>>>,
    pub positions_bin: Vec<u8>, // TODO: use a bitvector
    pub positions: Vec<usize>,
}


impl Index {
    pub fn new(max: usize) -> Self {
        Self {
            data: HashMap::with_capacity(100_000),
            positions_bin: vec![0; max],
            positions: Vec::with_capacity(1000_000)
        }
    }
}
