
use leptos::{*};
use crate::app::models::person::{AddPersonRequest, Person};
use serde::{*};

#[server(GetPersons, "/api")]
pub async fn get_persons() -> Result<Vec<Person>, ServerFnError> {
    let persons = retrive_all_persons().await;
    match persons {
       Ok(persons) => Ok(Json(persons)),
        None => Err(ServerFnError::Args(String::from("Failed to get persons")))
    }
}

#[server(AddPerson, "/api")]
pub async fn add_person(add_person_request: AddPersonRequest) -> Result<Person, ServerFnError> {
    let person = add_new_person(add_person_request.name, add_person_request.title, add_person_request.level, add_person_request.compensation).await;
    match person {
        Some(person) => Ok(person),
        None => Err(ServerFnError::Args(String::from("Failed to add new person")))
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::app::db::database;
        use chrono::{DateTime, Local};
        use uuid::Uuid;

        async fn retrive_all_persons() -> Vec<Person> {
            let persons = database::get_all_persons().await;
            match persons {
                Some(persons) => Some(persons),
                None => Vec::new()
            }
        }

        async fn add_new_person<T>(name:T, title:T, level:T, compensation:i32) -> Option<Person> where T: Into<String> {
           let mut buffer = Uuid::encode_buffer();
           let uuid = Uuid::new_v4().to_hyphenated().encode_lower(&mut buffer).to_string();

           //getting the current timestamp
           let current_now = Local::now();
           let current_formatted = current_now.to_string();

           let new_person = Person::new(String::from(uuid), name.into(), title.into(), level.into(), compensation, current_formatted);

           database::add_person(new_person).await;
        }
    }
}

