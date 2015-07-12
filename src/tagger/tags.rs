pub struct TagPair<'a, T> {
    pub word: &'a str,
    pub tag: T,
}

pub trait TagSet<T> {
    fn parse_word<'a>(input: &'a str) -> TagPair<'a, T>;
}
