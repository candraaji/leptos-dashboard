use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Serialize, Deserialize, Validate, PartialEq, Eq, Clone)]
pub struct Person {

    pub uuid: String,
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    #[validate(length(min = 1, max = 100, message = "Title must be between 1 and 100 characters"))]
    pub title: String,
    #[validate(length(min = 1, max = 100, message = "Level must be between 1 and 100 characters"))]
    pub level: String,
    #[validate(range(min = 2000, max = 99999))]
    pub compensation: i32,
    pub joined_date: String,
}

impl Person {
    pub fn new(uuid: String, name: String, title: String, level: String, compensation: i32, joined_date: String) -> Self {
        Self {
            uuid,
            name,
            title,
            level,
            compensation,
            joined_date,
        }
    }
}