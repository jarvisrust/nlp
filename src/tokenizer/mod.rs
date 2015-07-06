pub type Token<'a> = &'a str;
pub type TokenList<'a> = Vec<Token<'a>>;


pub fn tokenize<'a>(input: &'a str) -> TokenList {
    let mut out = TokenList::new();

    out.push(input);

    out = split_on_spaces(out);
    out = split_on_hyphens(out);

    out
}

fn split_on_spaces<'a>(input: TokenList) -> TokenList {
    let mut out = TokenList::new();

    for token in input {
        let split = token.split(" ");
        for item in split {
            out.push(item);
        }
    }

    out
}

fn split_on_hyphens<'a>(input: TokenList) -> TokenList {
    let mut out = TokenList::new();

    for token in input {
        let split = token.split("-");
        for item in split {
            out.push(item);
        }
    }

    out
}


// `!@#$%^&*()-_+={[}]\|:;'”<,>.?/
fn split_off_punctuation<'a>(input: TokenList) -> TokenList {
    let mut out = TokenList::new();

    for token in input {

        let mut split = token.chars();

        let mut current = String::new();
        loop {
            match split.next() {
                None => break,
                Some('`') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('!') => { out.push(current.as_str()); out.push(&"!"); current = String::new() },
                Some('@') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('#') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('$') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('%') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('^') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('&') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('*') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('(') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some(')') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('-') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('_') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('+') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('=') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('{') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('[') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('}') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some(']') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('\\') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('|') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some(':') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some(';') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('\'') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('”') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('<') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some(',') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('>') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('.') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('?') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some('/') => { out.push(current.as_str()); out.push(&"`"); current = String::new() },
                Some(x) => {
                    current.push(x);
                },
            }
        }

    }

    out
}

