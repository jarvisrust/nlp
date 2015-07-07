
use tokenizer;

pub type NGram = Vec<String>;

pub struct NGramSet {
    n: usize,
    set: Vec<NGram>,
}

impl NGramSet {

    pub fn create_empty(size: usize) -> NGramSet {
        NGramSet{n: size, set: Vec::<NGram>::new()}
    }

    pub fn pop(&mut self) -> NGram {
        self.set.pop().unwrap()
    }
    
    pub fn push(&mut self, item: NGram) -> usize {
        if item.len() != self.n {
            return self.n-item.len();
        }

        self.set.push(item);

        0
    }

    pub fn get_vec(&self) -> &Vec<NGram> {
        return &self.set;
    }
}
