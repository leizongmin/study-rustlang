#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
use rocket::*;
use rocket_contrib::*;

#[derive(Serialize, Deserialize)]
struct Message {
    id: i32,
    contents: String,
}

#[get("/")]
fn index() -> Json {
    Json(json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }))
}

#[post("/post", format = "application/json", data = "<message>")]
fn post(message: Json<Message>) -> Json {
    Json(json!({ "id": message.id, "contents": message.contents}))
}

#[get("/serve_file/<name>")]
fn serve_file(name: String) -> Option<response::NamedFile> {
    response::NamedFile::open(name).ok()
}

// #[get("/hello/<name>/<age>/<cool>")]
// fn hello(name: String, age: u8, cool: bool) -> String {
//     if cool {
//         format!("You're a cool {} year old, {}!", age, name)
//     } else {
//         format!("{}, we need to talk about your coolness.", name)
//     }
// }

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![serve_file])
        .mount("/", routes![post])
        .launch();
}
