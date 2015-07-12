
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

    pub fn add_front_padding(&mut self) {
        let mut first = match self.set.first() {
            Some(item) => item.clone(),
            None => return,
        };

        for _ in 0..self.n {
            first.insert(0, "*".to_string());
            first.pop();
            self.set.insert(0, first.clone());
        }
    }

    pub fn add_stop(&mut self) {
        let mut last = match self.set.last() {
            Some(item) => item.clone(),
            None => return,
        };

        for i in 1..last.len() {
            last.swap(i, i-1);
        }

        last.pop();

        last.push("STOP".to_string());

        self.set.push(last);
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
