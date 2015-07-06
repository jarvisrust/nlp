pub type Token = String;
pub type TokenList = Vec<Token>;


pub fn tokenize<'a>(input: String) -> TokenList {
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
            out.push(item.to_string());
        }
    }

    out
}

fn split_on_hyphens<'a>(input: TokenList) -> TokenList {
    let mut out = TokenList::new();

    for token in input {
        let split = token.split("-");
        for item in split {
            out.push(item.to_string());
        }
    }

    out
}


// `!@#$%^&*()-_+={[}]\|:;'”<,>.?/
fn split_off_punctuation<'a>(input: TokenList) -> TokenList {
    let mut out = TokenList::new();
    let mut current = String::new();

    for token in input {

        let mut split = token.chars();

        loop {
            match split.next() {
                None => break,
                Some('`') => out.append(&mut take_symbol("`".to_string(), &mut current)),
                Some(x) => {
                    current.push(x);
                },
            }
        }

    }

    out
}

fn take_symbol<'a>(ch: String, current: &mut String) -> TokenList {
    let mut out = TokenList::new();

    out.push(current.clone());
    out.push(ch);

    current.truncate(0); // = String::new();

    out
}

