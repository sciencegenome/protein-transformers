#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]

pub struct Genomeiter {
    pub header: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct ProfileKmer {
    pub name: String,
    pub nextname: String,
    pub sequence: Vec<String>,
    pub nextsequence: Vec<String>,
    pub count: usize,
    pub total: usize,
    pub ratio: f32,
}
