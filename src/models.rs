use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Product {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub price: u32,
    pub quantity: u32,
    pub status: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShortProduct {
    pub _id: ObjectId,
    pub name: String,
    pub price: u32,
}
