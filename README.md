# sentence-kmer
 - sentence based sequence selector for the transformers and also for the graph algorithms. implementation: profiling the uniquekmer and then checking the presence of the shared kmer across the sequences and then estimating the kmer ratio. This allows for the sequences that already having a high similarity ratio are likely to have some suffix tree and can be excluded from building the graphs and will create the graph in less time.
 - please see the last commit message and if it says compiled binary then it is completed or else still in development version.

 ```
  cargo build
 ```
 ```
 14:10:05 gauravsablok@genome sentence-kmer main ? ./identity-kmers -h
 sequence similarity based on shared kmers

 Usage: identity-kmers <COMMAND>

 Commands:
  sequence  profile the similarity based on the observed kmer
  help      Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version
 14:10:10 gauravsablok@genome sentence-kmer main ? ./identity-kmers sequence -h
 profile the similarity based on the observed kmer

 Usage: identity-kmers sequence <SEQUENCEPATH> <SEQUENCEKMER>

 Arguments:
  <SEQUENCEPATH>  provide the path to sequence file
  <SEQUENCEKMER>  provide the kmer to be profiled for the sequence similarity

 Options:
  -h, --help  Print help

 ```
 - to run the compiled binary 
 ```
 14:06:49 gauravsablok@genome sentence-kmer main ? 
    ./target/debug/identity-kmers sequence ./samplefile/sample.fasta 4
 The sequence similarity and the clustering of the sequences based on 
     the kmer means have been written: Ok("The sequence similarity scores and the cluster of the sequences have been written")
 ```
 Gaurav Sablok
