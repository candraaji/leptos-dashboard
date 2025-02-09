cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() {
            DB.connect::<WS>("127.0.0.1:8000").await;
            DB.signin(Root, {
                username: "root",
                password: "candra123"
            }).await;
            DB.use_ns("surreal").use_db("person").await;
        }
    }
}