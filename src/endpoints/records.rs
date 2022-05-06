use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateLink<'a> {
    pub href: &'a str,
    pub templated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordLinks<'a> {
    // #[serde(rename(serialize = "self", deserialize = "self"))]
    // pub itself: TemplateLink<'a>,
    pub next: Option<TemplateLink<'a>>,
    pub prev: Option<TemplateLink<'a>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Embedded<T> {
    records: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Record<T> {
    pub _links: RecordLinks,
    pub _embedded: Embedded<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: TemplateLink,
    pub transactions: Option<TemplateLink>,
    pub operations: Option<TemplateLink>,
    pub payments: Option<TemplateLink>,
    pub effects: Option<TemplateLink>,
    pub offers: Option<TemplateLink>,
    pub trades: Option<TemplateLink>,
}
