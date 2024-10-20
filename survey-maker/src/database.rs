cfg_if::cfg_if! {

    if #[cfg(feature = "ssr")] {

        use crate::app::models::Person;
        use crate::app::errors::{ ErrorMessage, PersonError };
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() {

            let _ = DB.connect::<Ws>("127.0.0.1:8000").await;
            let _ = DB.signin(Root {
                username: "root",
                password: "root",
            })
            .await;
            let _ = DB.use_ns("SurveyTest").use_db("SurveyTestDb").await;
        }
    

pub async fn get_all_Questions() -> Option<Vec<Person>> {

    open_db_connection().await;
    let get_all_Questions = DB.query("SELECT * FROM SurveyQs ;").await;
    let _ = DB.invalidate().await;

    match get_all_Questions {

        Ok(mut res) => {
            let found = res.take(0);
            match found {
                Ok(found_question) => Some(found_question),
                Err(_) => None,
            }
        },
        Err(_) => None,
    }
}

}

}