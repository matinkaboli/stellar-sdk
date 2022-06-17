#[derive(Debug)]
pub enum Direction {
    Asc,
    Desc,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Asc => String::from("asc"),
            Direction::Desc => String::from("desc"),
        }
    }
}
