use std::fmt;

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
                   }}", self.fasta, self.vcf)
    }
}
