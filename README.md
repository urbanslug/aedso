# aedso

Another Elastic Degenerate String Outputter

Expects a single sequence in the fasta file and the VCF to only apply to the fasta file.
*Making it work for multi-fasta is not a priority*.

## Compile
```
cargo install
```

## Run
```
aedso -h
aedso 0.0.1

Another Elastic Degenerate String Outputter (aedso)

USAGE:
    aedso [OPTIONS] <fasta> <vcf>

ARGS:
    <fasta>    Path to input fasta file
    <vcf>      Path to input VCF file

OPTIONS:
    -h, --help                                       Print help information
    -l, --output-line-length <output_line_length>    Max length of lines in eds [default: 80]
    -v                                               Sets the level of verbosity [default: 0]
    -V, --version                                    Print version information
```

## Example
```
aedso -vvv Homo_sapiens.GRCh38.dna.chromosome.1.fa homo_sapiens-chr1.vcf.bgz > x.eds
```
