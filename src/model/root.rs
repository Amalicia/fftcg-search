use serde::{Deserialize, Serialize};
use crate::model::card::Card;

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub count: u32,
    pub cards: Vec<Card>
}