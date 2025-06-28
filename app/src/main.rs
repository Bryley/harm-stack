use axum::{Router, extract::Query, response::IntoResponse, routing::get};
use maud::html;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

const PORT: u64 = 3000;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(LevelFilter::INFO)
        .init();

    let router = Router::new()
        .route("/", get(index))
        .route("/htmx-example", get(htmx_example))
        .nest_service("/assets", ServeDir::new("./assets"));

    let listener = TcpListener::bind(format!("0.0.0.0:{PORT}")).await?;
    info!("Started server on http://127.0.0.1:{PORT}");
    axum::serve(listener, router).await?;

    Ok(())
}

async fn index() -> impl IntoResponse {
    html! {
        html {
            head {
                title {
                    "Example HARM Application"
                }
                // Tailwind
                link rel="stylesheet" href="/assets/tailwind.css";
                // Symbols
                link rel="stylesheet" href="https://fonts.googleapis.com/icon?family=Material+Symbols+Outlined";
                // HTMX
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js" integrity="sha384-Akqfrbj/HpNVo8k11SXBb6TlBWmXXlYQrCSqEWmyKJe+hDm3Z/B2WVG4smwBkRVm" crossorigin="anonymous" {}
                // Alpinejs
                script src="https://cdn.jsdelivr.net/npm/@alpinejs/persist@3.x.x/dist/cdn.min.js" defer {}
                script src="//unpkg.com/alpinejs" defer {}
            }
            body x-data="{
                dark: $persist(null).as('dark-mode')
            }" x-bind:class="dark === true ? 'dark' : (dark === false ? 'light' : '')" x-cloak {
                div." bg-bg-back text-contrast h-screen px-30 py-10" {
                    h1." text-3xl" {
                        "Hello World!"
                    }
                    p {
                        "This is a paragraph, Material Icons are included too:"
                    }
                    span."material-symbols-outlined" {
                        "settings"
                    }
                    form hx-get="/htmx-example" {
                        input type="text" name="name" placeholder="Name";
                        input type="number" name="age" placeholder="Age";
                        button type="submit" {
                            "Submit"
                        };
                    }
                    button x-on:click="if (dark) {
                        dark = false
                    } else if (dark === false) {
                        dark = null
                    } else {
                        dark = true
                    }" {
                        "Switch to light/dark/system"
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct FormData {
    name: String,
    age: i64,
}

async fn htmx_example(Query(query): Query<FormData>) -> impl IntoResponse {
    let new_age = server::add(query.age, 100i64);

    html! {
        "Hello " (query.name) ", looks like your age is " (query.age) ", your age in 100 years will be " (new_age)
    }
}
