#[derive(
    Debug,
    strum::Display,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumString,
    PartialEq,
    Eq,
    Hash,
)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum SortDirection {
    Asc,
    Desc,
}
