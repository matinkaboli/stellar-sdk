use super::Asset;

#[derive(Debug)]
pub enum StrictPathSource<'a> {
    Account(String),
    Assets(Vec<&'a Asset<'a>>),
}
