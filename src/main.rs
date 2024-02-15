use askama::Template; // bring trait in scope
use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

#[derive(Deserialize)]
struct InputQuery {
    input: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    let listener = TcpListener::bind("127.0.0.1:3030").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    println!("{}", hello.render().unwrap()); // then render it.
}

async fn handler(Query(myquery): Query<InputQuery>) -> Html<String> {
    return Html(HelloTemplate { name: &myquery.input }.render().unwrap()); 
}