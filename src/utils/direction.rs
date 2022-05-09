#[derive(Debug)]
pub enum Direction {
    Asc,
    Desc,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Asc => "asc",
            Direction::Desc => "desc",
        }
    }
}
