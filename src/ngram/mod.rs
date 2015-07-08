
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

    pub fn create_from_string(size: usize, input: String) -> NGramSet {
        let mut result = NGramSet::create_empty(size);

        let tokens = tokenizer::tokenize(input);

        // TODO: REPLACE THIS WITH A QUEUE ???
        let mut temp = NGram::new();
        for i in tokens.iter() {
            if temp.len() < size {
                temp.insert(0, i.clone());
            }

            if temp.len() == size {
                temp.reverse();
                result.push(temp.clone());
                temp.reverse();

                temp.pop();
            }
        }

        result
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
