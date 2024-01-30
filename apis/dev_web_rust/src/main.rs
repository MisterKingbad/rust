#![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::hyper::header::q;

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
    None => format!("Searching for '{}' (no type specified)", query)
  }
  
}

#[get("/users/<id>")]
fn get_user(id: u32) -> String {
  format!("Retornando o usuário com ID: {}", id)
}

#[post("/users/<name>")]
fn create_user(name: String) {
  println!("Criando usuário com nome: {}", name)
}

#[delete("/users/<id>")]
fn delete_user(id: u32) {
  println!("Deletando usuário de id: {}", id)
}

#[put("/users/<id>/<name>")]
fn update_user(id: u32, name: String) {
  println!("Atualizando usuário de id {} para o nome {}", id, name)
}

#[get("/users?<query>&<page>")]
fn search_users(query: String, page: Option<u32>) -> String {
   match page {
       Some(p) => format!("Buscando usuários com a consulta '{}' na página {}", query, p),
       None => format!("Buscando usuários com a consulta '{}' (sem especificar a página)", query)
   }
}


fn main() {
    rocket::ignite().mount("/", routes![index, hello, number, search, search_two, get_user, create_user, delete_user, update_user, search_users]).launch();
  }
