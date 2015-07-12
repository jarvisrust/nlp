use std::collections::HashMap;

// Form of key ("tag1`tag2`tag3")
pub type TagProbabilityQ = HashMap<String, f32>;

// Form of key ("word->tag")
pub type TagProbabilityE = HashMap<String, HashMap<String, f32>>;

pub type TagPair = (String, String);

pub fn generate_key_from_pair(input: TagPair) -> String {
    let mut result = String::new();
    match input {
        (word, tag) => {
            result.push_str(&word[..]);
            result.push_str("->");
            result.push_str(&tag[..]);
        },
    }

    result
}
