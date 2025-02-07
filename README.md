# sentence-kmer
 - sentence based sequence selector for the transformers and also for the graph algorithms.
 - implementation: profling the uniquekmer and then checking the presence of the shared kmer across the sequences and then estimating the kmer ratio.
 - this allows for the sequences that already having a high similarity ratio are likely to have some suffix tree and can be excluded from building the graphs and will create the graph in less time.
 - please see the last commit message and if it says compiled binary then it is completed or else still in development version.

 ```
  cargo build
 ```

 ```
 ➜  sentence-kmer git:(main) ✗ ./target/debug/sequence-transformers -h                                                                                   
 sentence-kmer       
 Usage: sentence-kmer <COMMAND>                                                                                                                         
                                                                                                                                                               
 Commands:                                                                                                                                                      
  sequence  profile the similarity based on the observed kmer                                                                                                 
  help      Print this message or the help of the given subcommand(s)                                                                                          
                                                                                                                                                               
 Options:                                                                                                                                                       
  -h, --help     Print help                                                                                                                                    
  -V, --version  Print version 
```
 Gaurav Sablok
