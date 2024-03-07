use std::process::{Command, Output};
use actix_web::{get, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 4040))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    read_from_db()
}

fn read_from_db() -> String {
    let output: Output = Command::new(String::from("./main")).args(&["-read"]).output().expect("Error while connecting with db");

    String::from_utf8_lossy(&output.stdout).to_string()
}