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
```

```
aedso -vvv Homo_sapiens.GRCh38.dna.chromosome.1.fa homo_sapiens-chr1.vcf.bgz > x.eds
```
