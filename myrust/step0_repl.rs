use std::io;
use std::io::Write;

fn read(inp: String) -> String {
    return inp;
}

fn eval(inp: String) -> String {
    return inp;
}

fn print(inp: String) -> String {
    return inp;
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
