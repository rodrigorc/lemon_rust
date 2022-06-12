extern crate regex;
mod parser;
mod lexer;

use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

fn parse<R: BufRead>(file: &mut R) -> io::Result<parser::Expression> {

    use lexer::LexerAction::*;
    let lexer = lexer::Lexer::<parser::Token>::new(vec![
            (r"\s+", Ignore),
            (r"\d+", Action(Box::new(
                        |s| i32::from_str(s).ok().map(|x| parser::Token::VALUE(x))
            ))),
            (r"\+", Token(Box::new(|| parser::Token::PLUS))),
            (r"-", Token(Box::new(|| parser::Token::MINUS))),
            (r"\*", Token(Box::new(|| parser::Token::TIMES))),
            (r"/", Token(Box::new(|| parser::Token::DIV))),
            (r"\(", Token(Box::new(|| parser::Token::LPAREN))),
            (r"\)", Token(Box::new(|| parser::Token::RPAREN))),
    ]);
    let mut parser = parser::Parser::new(None);

    let mut line = String::new();
    file.read_line(&mut line)?;

    let mut subline : &str = &line[..];

    while !subline.is_empty() {
        use lexer::LexerAction::*;
        match lexer.next(subline) {
            (col, Some(action)) => {
                let stoken = &subline[..col];
                subline = &subline[col..];

                match action {
                    &Ignore => {},
                    &Action(ref func) => {
                        match func(stoken) {
                            Some(token) => {
                                parser.parse(token);
                            },
                            None => {
                                return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input"));
                            }
                        }
                    },
                    &Token(ref ftoken) => {
                        let token = ftoken();
                        parser.parse(token);

                    },
                };
            }
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input"));
            }
        }
    }
    parser.parse(parser::Token::EOI);
    match parser.into_extra() {
        None => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input")),
        Some(x) => Ok(x),
    }
}

fn main() {

    let file = std::io::stdin();
    //let mut file = try!(std::fs::File::open(file_name));
    let mut file = BufReader::new(file);
    match parse(&mut file) {
        Ok(expr) => { println!("{:?}", expr); },
        Err(e) => { println!("Error {0}", e); }
    }
}
