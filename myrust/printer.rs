use super::types::MalType;

pub fn pr_str(inp: &MalType) -> String {
    match inp {
        MalType::MalNumber(n) => { format!("{}", n) },
        MalType::MalSymbol(s) => { format!("{}", s) },
        MalType::MalList(l) => {
            let mut list_content = Vec::new();
            for item in l.iter() {
                list_content.push(pr_str(item));
            }
            format!("({})", list_content.join(" "))
        },
        _ => { "other type".to_string() }
    }
}
