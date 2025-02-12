use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "identity-kmers",
    version = "1.0",
    about = "sequence similarity based on shared kmers"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// profile the similarity based on the observed kmer
    Sequence {
        /// provide the path to sequence file
        sequencepath: String,
        /// provide the kmer to be profiled for the sequence similarity
        sequencekmer: String,
    },
}
