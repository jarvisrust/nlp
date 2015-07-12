use std::collections::HashMap;

// Form of key ("tag1+tag2->(tag3, pct)")
pub type TagProbabilityQ = HashMap<String, HashMap<String, f32>>;

// Form of key ("word->(tag, pct)")
pub type TagProbabilityE = HashMap<String, HashMap<String, f32>>;

pub type TagPair = (String, String);
