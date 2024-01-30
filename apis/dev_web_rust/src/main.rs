#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::content;
use rocket::State;
use std::collections::HashMap;


struct Local {
  temperatura: f64,
  cidade: String
}

fn obter_temperatura(local: &Local) -> String {
  local.temperatura.to_string()
}

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


#[catch(404)]
fn not_found() -> &'static str {
  "Página não encontrada :("
}

#[get("/temperatura/<cidade>")]
fn temperatura(local_state: State<Local>, cidade: String) -> content::Html<String>{
  let local = local_state.inner();
  if local.cidade == cidade {
    content::Html(format!("A temperatura da cidade {} é de {}°C", cidade, obter_temperatura(local)))
  } else {
    content::Html(format!("Não foi possivel obter a temperatura da cidade {}", cidade))
  }
}

#[get("/temperatura2/<cidade>")]
fn temperatura2(local_state: State<HashMap<String,Local>>, cidade: String) -> content::Html<String>{
  let local = local_state.inner();
  match local.get(&cidade) {
    Some(local) => content::Html(format!("A temperatura da cidade {} é de {}°C", cidade, obter_temperatura(local))),
    None => content::Html(format!("Não foi possivel obter a temperatura da cidade {}", cidade))
  }
}

fn main() {

    let mut local_state = HashMap::<String, Local>::new();
    local_state.insert("São Paulo".to_string(), Local{temperatura: 25.5, cidade: "São Paulo".to_string()});
    local_state.insert("Rio de Janeiro".to_string(), Local{temperatura: 35.5, cidade: "Rio de Janeiro".to_string()});
    local_state.insert("Belo Horizonte".to_string(), Local{temperatura: 30.5, cidade: "Belo Horizonte".to_string()});

    rocket::ignite().mount("/", routes![index, hello, number, search, search_two, get_user, create_user, delete_user, update_user, search_users, temperatura, temperatura2]).register(catchers![not_found]).manage(local_state).launch();
  }
