use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub _id: ObjectId,
    pub task: String,
}