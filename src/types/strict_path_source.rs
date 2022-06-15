use super::Asset;

#[derive(Debug)]
pub enum StrictPathSource<'a> {
    Account(&'a str),
    Assets(Vec<&'a Asset<'a>>),
}
