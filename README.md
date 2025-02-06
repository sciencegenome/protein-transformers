# sentence-transformers

 - sentence based sequence selector for the transformers and also for the graph algorithms.
 - implementation: profling the uniquekmer and then checking the presence of the shared kmer across the sequences and then estimating the kmer ratio.
 - this allows for the sequences that already having a high similarity ratio are likely to have some suffix tree and can be excluded from building the graphs and will create the graph in less time.

 ```
  cargo build
 ```

 ```
 ➜  protein-transformers git:(main) ✗ ./target/debug/sequence-transformers -h                                                                                   
 sequencetranformersselection       
 Usage: sequence-transformers <COMMAND>                                                                                                                         
                                                                                                                                                               
 Commands:                                                                                                                                                      
  sequence  profilei the similarity based on the observed kmer                                                                                                 
  help      Print this message or the help of the given subcommand(s)                                                                                          
                                                                                                                                                               
 Options:                                                                                                                                                       
  -h, --help     Print help                                                                                                                                    
  -V, --version  Print version 
```
 Gaurav Sablok
