use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Import the DB instance from lib.rs
use crate::DB; 
use surrealdb::engine::remote::ws::{Ws, Wss};
use surrealdb::opt::Config;
use std::time::Duration;
use surrealdb::Surreal;
use tokio::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new SurrealDB client
    println("opens this shi");
    let db = Surreal::new::<surrealdb::opt::auth::Root>("http://localhost:8000").await?;

    // Authenticate with root username and password
    db.signin(surrealdb::opt::auth::Root {
        username: "root",  // Username
        password: "root",  // Password
    })
    .await?;
    println("Connected to DB");
    // Select the namespace and database
    db.use_ns("SurveyTest").use_db("SurveyTestDb").await?;

    // // Example: Run a query to retrieve data
    // let persons: Vec<Value> = db.select("person").await?;
    // println!("Persons: {:?}", persons);

    Ok(())
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/survey-test-template.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    use leptos::*;
    use leptos_meta::*;
    use leptos_router::*;
    
    #[component]
    pub fn App() -> impl IntoView {
        // Provides context that manages stylesheets, titles, meta tags, etc.
        provide_meta_context();
    
        view! {
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            <Stylesheet id="leptos" href="/pkg/survey-test-template.css"/>
    
            // sets the document title
            <Title text="Welcome to Leptos"/>
    
            // content for this welcome page
            <Router>
                <main>
                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </main>
            </Router>
        }
    }
    
    /// Renders the home page of your application.
    #[component]
    fn HomePage() -> impl IntoView {
        // // Creates a reactive value to update the button
        // let (count, set_count) = create_signal(0);
        // let on_click = move |_| set_count.update(|count| *count += 1);
    
        // view! {
        //     <h1>"Welcome to Leptos!"</h1>
        //     <button on:click=on_click>"Click Me: " {count}</button>
        // }
    }
    
    /// 404 - Not Found
    #[component]
    fn NotFound() -> impl IntoView {
        // set an HTTP status code 404
        // this is feature gated because it can only be done during
        // initial server-side rendering
        // if you navigate to the 404 page subsequently, the status
        // code will not be set because there is not a new HTTP request
        // to the server
        #[cfg(feature = "ssr")]
        {
            // this can be done inline because it's synchronous
            // if it were async, we'd use a server function
            let resp = expect_context::<leptos_actix::ResponseOptions>();
            resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
        }
    
        view! {
            <h1>"Not Found"</h1>
        }
    }
    
    // let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Survey Creator Prototype"</h1>
        // <button on:click=on_click>"Get questions"</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
