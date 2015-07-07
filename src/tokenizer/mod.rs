/**
   Copyright 2015 W. Max Lees

   This file is part of Jarvis OS.

   Jarvis OS is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   Jarvis OS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with Jarvis OS.  If not, see <http://www.gnu.org/licenses/>.
*/

pub type Token = String;
pub type TokenList = Vec<Token>;


pub fn tokenize<'a>(input: String) -> TokenList {
    let mut out = TokenList::new();

    out.push(input);

    out = split_on_spaces(out);
    out = split_on_hyphens(out);
    out = split_on_punctuation(out);

    out
}

pub fn split_on_spaces<'a>(input: TokenList) -> TokenList {
    let mut out = TokenList::new();

    for token in input {
        let split = token.split(" ");
        for item in split {
            out.push(item.to_string());
        }
    }

    out
}

pub fn split_on_hyphens<'a>(input: TokenList) -> TokenList {
    let mut out = TokenList::new();

    for token in input {
        let split = token.split("-");
        for item in split {
            out.push(item.to_string());
        }
    }

    out
}

pub fn split_on_punctuation<'a>(input: TokenList) -> TokenList {
    let mut out = TokenList::new();
    let mut current = String::new();

    for token in input {

        let mut split = token.chars();

        loop {
            match split.next() {
                None => {
                    out.push(current.clone());
                    current.truncate(0);

                    break;
                },
                Some(x) => {
                    let c: u8 = x as u8;
                    if  (c > 32  && c < 48) ||
                        (c > 57  && c < 65) ||
                        (c > 90  && c < 97) ||
                        (c > 122 && c < 127) {

                        out.extend(&mut take_symbol(x.to_string(), &mut current).into_iter());
                    } else {
                        current.push(x);
                    }
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

