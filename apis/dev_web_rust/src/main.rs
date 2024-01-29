#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
  format!("Hello, {}", name)
}

#[get("/number/<number>")]
fn number(number: i32) -> String {
  format!("o numero é: {}", number)
}

#[get("/search?<query>&<max_results>&<page>")]
fn search(query: String, max_results: i32, page:i32) -> String {
  format!("Searching for '{}' (max: {}, page: {})", query, max_results, page)
}

#[get("/search_two?<query>&<typ>")]
fn search_two(query: String, typ: Option<String>) -> String {
  match typ {
    Some(t) => format!("Searching for '{}' (type: {})", query, t),
    None => format!("Searching for '{}' (no type specified)", query),
  }
  
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello, number, search, search_two]).launch();
  }
