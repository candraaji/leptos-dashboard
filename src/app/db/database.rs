cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {

        use crate::app::models::Person;
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() {
            DB.connect::<Ws>("127.0.0.1:8000").await;
            let _ = DB.signin({ Root {
                username: "root",
                password: "candra123"
            }}).await;
            let _ = DB.use_ns("surreal").use_db("person").await;
        }

        pub async fn get_all_persons() -> Option<Vec<Person>> {
            open_db_connection().await;
            let get_all_persons = DB.query("SELECT * FROM person ORDER BY joined_date DESC").await;
            let _ = DB.invalidate().await;
            
            match get_all_persons {
                Ok(mut res) => {
                    let found =  res.take(0);
                    match found {
                        Ok(found_persons) => Some(found_persons),
                        Err(_) => None
                    }
                }
                Err(_) => None
            }
        }

        pub async fn add_person(new_person: Person) -> Option<Person> {
            open_db_connection().await;
            let add_person = DB.create(("person",new_person.uuid.clone())).content(new_person).await;
            let _ = DB.invalidate().await;
            
            match add_person {
                Ok(person) => person,
                Err(_) => None
            }
        }

        pub async fn update_person(uuid: String, title: String, level: String, compensation: i32) -> Option<Person> {
            open_db_connection().await;

            let find_person = Result<Option<Person>, Error> = DB.select(("person", &uuid)).await;
            
            match find_person {
                Ok(found) => {
                    match found {
                        Some(found_person) => {
                            let updated_user: Result<Option<Person>,Error> = DB.update(("person", &uuid)).merge(Person::new(
                                uuid,
                                found_person.name,
                                title,
                                level,
                                compensation,
                                found_person.joined_date
                            )).await;
                            let _ = DB.invalidate().await;
                            match updated_user {
                                Ok(returned_user) => returned_user,
                                Err(_) => None
                            }
                        },
                        None => None
                    }
                },
                Err(_) => {
                    let _ = DB.invalidate().await;
                    None
                }
            }
        }
    }
}