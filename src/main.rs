#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::JSON;

#[derive(Deserialize)]
struct Command {
    command: String, // TODO Handle paramaters
    cwd: String,
    state: String,
}

#[get("/")]
fn index() -> &'static str {
    "This is a basic Rust web service."
}

#[post("/", format = "application/json", data = "<command>")]
fn command(command: JSON<Command>) -> &'static str {
    println!("Recieved: command = {}, cwd = {}, state = {}",
             command.command,
             command.cwd,
             command.state);
    "success"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/command", routes![command])
        .launch();
}
