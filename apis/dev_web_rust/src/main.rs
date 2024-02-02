#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::Json;
use rocket::http::{ContentType, Status};
use rocket::response::{Content, content};
use rocket::State;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct FormInput {
  name: String,
  email: String,
}

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


#[get("/form")]
fn form() -> Content<String> {
  
  let html = r#"
      <!DOCTYPE html>
      <html>
          <head>
              <title> Formulario </title>
          </head>
          <body>
            <h1> Exemplo de Formulario</h1>
            <form method="post" action="/form">
              <label for="name">Nome:</label>
              <input type="text" name="name">
              <br>
              <label for="email">E-mail:</label>
              <input type="email" name="email">
              <br>
              <button type="submit">Enviar</button>
            </form>

          </body>
      </html>
  "#;
  Content(ContentType::HTML, html.to_string())

}

#[post("/form", data="<form_input>")]
fn subimit_form(form_input: Json<FormInput>) -> Result<Content<String>, Status> {
  match validate_input(&form_input)  {
   Ok(_) => {
      let msg = format!("Name: {}\n Email: {}", form_input.name, form_input.email);
      let html = format!(r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title> Formulario </title>
            </head>
            <body>
              <h1> Sucesso! </h1>
              <p>{}</p>

            </body>
        </html>
      "#, msg);
      Ok(Content(ContentType::HTML, html.to_string()))
   },
   Err(error) => {
    let html = format!(r#"
    <!DOCTYPE html>
    <html>
        <head>
            <title> Formulario </title>
        </head>
        <body>
          <h1> Erro! </h1>
          <p>{}</p>
          <p><a href="/form">Voltar</a></p>
        </body>
    </html>
  "#, error);
    Err(Status::UnprocessableEntity)
   }    
  }
}

fn validate_input(form_input: &FormInput) -> Result<(), String> {
  if form_input.name.is_empty() {
    return Err("Name is required".to_string());
  }

  if form_input.email.is_empty() {
    return  Err("Email is required".to_string());
  }

  Ok(())
}

fn main() {

    let mut local_state = HashMap::<String, Local>::new();
    local_state.insert("São Paulo".to_string(), Local{temperatura: 25.5, cidade: "São Paulo".to_string()});
    local_state.insert("Rio de Janeiro".to_string(), Local{temperatura: 35.5, cidade: "Rio de Janeiro".to_string()});
    local_state.insert("Belo Horizonte".to_string(), Local{temperatura: 30.5, cidade: "Belo Horizonte".to_string()});

    rocket::ignite().mount("/", routes![index, hello, number, search, search_two, get_user, create_user, delete_user, update_user, search_users, temperatura, temperatura2, form, subimit_form]).register(catchers![not_found]).manage(local_state).launch();
  }

