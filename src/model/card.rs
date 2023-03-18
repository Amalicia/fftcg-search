use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Cost")]
    pub cost: String,
    #[serde(rename = "Power")]
    pub power: String,
    #[serde(rename = "Category_1")]
    pub cat_1: String,
    #[serde(rename = "Category_2")]
    pub cat_2: String,
    #[serde(rename = "Name_EN")]
    pub name: String,
    #[serde(rename = "Type_EN")]
    pub typ: String,
    #[serde(rename = "Job_EN")]
    pub job: String,
    #[serde(rename = "Text_EN")]
    pub text: String,
    #[serde(rename = "Set")]
    pub set: String,
}