pub enum MalType {
    MalNumber(i32),
    MalSymbol(String),
    MalString(String),
    MalList(Vec<MalType>)
}
