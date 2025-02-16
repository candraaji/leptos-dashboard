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

#[derive(Debug, Serialize, Deserialize, Validate, PartialEq, Eq, Clone)]
pub struct AddPersonRequest {
    #[validate(length(min = 1, max = 200, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    #[validate(length(min = 1, max = 300, message = "Title must be between 1 and 100 characters"))]
    pub title: String,
    #[validate(length(min = 1, max = 500, message = "Level must be between 1 and 100 characters"))]
    pub level: String,
    #[validate(range(min = 2000, max = 999999999))]
    pub compensation: i32,
}

impl AddPersonRequest {
    pub fn new(name: String, title: String, level: String, compensation: i32) -> Self {
        Self {
            name,
            title,
            level,
            compensation,
        }
    }   
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct EditPersonRequest {
    #[validate(length(min = 1, max = 200, message = "id is required"))]
    pub uuid: String,
    #[validate(length(min = 1, max = 300, message = "Title must be between 1 and 100 characters"))]
    pub title: String,
    #[validate(length(min = 1, max = 500, message = "Level must be between 1 and 100 characters"))]
    pub level: String,
    #[validate(range(min = 2000, max = 999999999))]
    pub compensation: i32,
}

impl EditPersonRequest {
    pub fn new(uuid: String, title: String, level: String, compensation: i32) -> Self {
        Self {
            uuid,
            title,
            level,
            compensation,
        }
    }   
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct DeletePersonRequest {
    #[validate(length(min = 1, message = "id is required"))]
    pub uuid: String,
}

impl DeletePersonRequest {
    pub fn new(uuid: String) -> DeletePersonRequest {
        DeletePersonRequest { uuid }
    }
}