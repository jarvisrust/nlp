
use tokenizer;


pub type Bigram = Vec<[String; 2]>;
pub type Trigram = Vec<[String; 3]>;

pub type Ngram<N> = Vec<[String; N]>;

fn test () {
    let a = ["Test".to_string(), "Test".to_string(), "Test".to_string(), "Test".to_string(), "Test".to_string()];
    let b = ["Test".to_string(), "Test".to_string(), "Test".to_string(), "Test".to_string(), "Test".to_string()];
    let c = ["Test".to_string(), "Test".to_string(), "Test".to_string(), "Test".to_string(), "Test".to_string()];
    let d = ["Test".to_string(), "Test".to_string(), "Test".to_string(), "Test".to_string(), "Test".to_string()];

    let vec = Ngram::<5>::new();
    vec.push(a);
    vec.push(b);
    vec.push(c);
    vec.push(d);
}
