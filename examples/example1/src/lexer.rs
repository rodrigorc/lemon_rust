
use regex::Regex;

pub enum LexerAction<TOKEN> {
    Ignore,
    Action(Box<dyn Fn(&str) -> Option<TOKEN>>),
    Token(Box<dyn Fn() -> TOKEN>),
}

pub struct Lexer<TOKEN> {
    re : Vec<(Regex, LexerAction<TOKEN>)>,
}

impl<TOKEN> Lexer<TOKEN> {
    pub fn new<I>(rules: I) -> Lexer<TOKEN>
        where I : IntoIterator<Item=(&'static str, LexerAction<TOKEN>)> {

        let re = rules.into_iter().map(|(s, ac)| {
            let expr = format!("^({})", s);
            (Regex::new(&expr).unwrap(), ac)
        }).collect::<Vec<_>>();

        Lexer{re : re}
    }

    pub fn next(&self, s : &str) -> (usize, Option<&LexerAction<TOKEN>>) {
        for &(ref re, ref action) in self.re.iter() {
            if let Some(to) = re.find(s) {
                return (to.range().len(), Some(action));
            }
        }
        (0, None)
    }
}

