use regex::Regex;

use super::types::MalType;

struct Reader {
    // TODO: rewrite with references ?
    tokens: Vec<String>,
    position: usize
}

impl Reader {
    pub fn new(v: Vec<String>) -> Reader {
        return Reader { tokens: v, position: 0 };
    }

    pub fn peek(&self) -> Option<&str> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        }
        else {
            None
        }
    }

    pub fn next(&mut self) -> Option<&str> {
        if self.position < self.tokens.len() {
            let p : Option<&str> = Some(&self.tokens[self.position]);
            self.position += 1;
            return p;
        }
        else {
            None
        }
    }
}

fn tokenize(inp: &str) -> Vec<String> {
    let mut ret_vec = Vec::new();
    let r = Regex::new(r##"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"##).unwrap();
    for cap in r.captures_iter(inp) {
        ret_vec.push(cap[1].to_string());
    }
    return ret_vec;
}

pub fn read_str(inp: &str) -> Result<MalType, &'static str> {
    let mut reader = Reader::new(tokenize(inp));
    return read_form(&mut reader);
}

fn read_atom(reader: &mut Reader) -> Result<MalType, &'static str> {
    match reader.next()
    {
        Some(n) => {
            let r = Regex::new(r##"^[0-9]+"##).unwrap();
            if r.is_match(n) {
                Ok(MalType::MalNumber(n.parse().unwrap()))
            }
            else {
                Ok(MalType::MalSymbol(n.to_string()))
            }
        }
        None => Err("Syntax error")
    }
}

fn read_list(reader: &mut Reader) -> Result<MalType, &'static str> {
    // skip the opening "("
    reader.next();
    let mut accum = Vec::new();
    loop {
        let p = read_form(reader);
        match p {
            Ok(MalType::MalSymbol(s)) => {
                if s == ")" {
                    return Ok(MalType::MalList(accum));
                }
                else {
                    accum.push(MalType::MalSymbol(s));
                }
            },
            Err(e) => { return Err("Unfinished list"); },
            Ok(o) => { accum.push(o); }
        }
    }
}


fn read_form(reader: &mut Reader) -> Result<MalType, &'static str> {
    match reader.peek() {
        Some("(") => read_list(reader),
        Some(_) => read_atom(reader),
        None => Err("EOF reached")
    }
}
