use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)] // Ignore unused field warning
struct SurveyResponse {
    name: String,
    email: String,
    satisfaction: String,
    features: Vec<String>,
    comments: Option<String>,
}



#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/survey-maker.css"/>

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
    view! {
        <div class="container">
            <h1>"Survey Form"</h1>
            <form action="/submit-survey" method="POST">
                <label for="name">"Name:"</label>
                <input type="text" id="name" name="name" required/>

                <label for="email">"Email:"</label>
                <input type="email" id="email" name="email" required/>

                <label>"How satisfied are you with our product?"</label>
                <select name="satisfaction" required>
                    <option value="">"Select..."</option>
                    <option value="very_satisfied">"Very Satisfied"</option>
                    <option value="satisfied">"Satisfied"</option>
                    <option value="neutral">"Neutral"</option>
                    <option value="dissatisfied">"Dissatisfied"</option>
                    <option value="very_dissatisfied">"Very Dissatisfied"</option>
                </select>

                <label>"What features do you use the most? (Select all that apply)"</label>
                <div class="checkbox-group">
                <input type="checkbox" id="feature1" name="features" value="feature1" />
                <label for="feature1">Feature 1</label><br/>
                <input type="checkbox" id="feature2" name="features" value="feature2" />
                <label for="feature2">Feature 2</label><br/>
                <input type="checkbox" id="feature3" name="features" value="feature3" />
                <label for="feature3">Feature 3</label><br/>
                </div>

                <label for="comments">"Additional Comments:"</label>
                <textarea id="comments" name="comments" rows="4"></textarea>

                <input type="submit" value="Submit Survey"/>
            </form>
        </div>
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
