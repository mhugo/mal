use std::io;
use std::io::Write;

mod types;
mod reader;
mod printer;

use types::MalType;
use reader::read_str;
use printer::pr_str;

fn read(inp: String)  -> Result<MalType, &'static str> {
    return read_str(&inp);
}

fn eval(inp: Result<MalType, &'static str>) ->  Result<MalType, &'static str> {
    return inp;
}

fn print(inp: Result<MalType, &'static str>) -> String {
    match inp {
        Ok(m) => pr_str(&m),
        Err(e) => e.to_string()
    }
}

fn rep(inp: String) -> String {
    return print(eval(read(inp)));
}

fn main()
{
    loop {
        print!("user> ");
        io::stdout().flush();
        let mut line = String::new();
        io::stdin().read_line(&mut line);
        if line == "" {
            break;
        }
        println!("{}", rep(line));
    }
}
