use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateLink {
    pub href: String,
    pub templated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: TemplateLink,
    pub next: Option<TemplateLink>,
    pub prev: Option<TemplateLink>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Embedded<T> {
    pub records: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Record<T> {
    pub _links: RecordLinks,
    pub _embedded: Embedded<T>,
}
