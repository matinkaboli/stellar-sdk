use super::Asset;

#[derive(Debug)]
pub enum StrictPathSource {
    Account(String),
    Assets(Vec<Asset>),
}
