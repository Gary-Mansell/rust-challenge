#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rocket::{Request, State};
use rocket_contrib::JSON;
use std::cell::RefCell;

#[derive(Deserialize)]
struct Command {
    command: String,
    arguments: Vec<String>,
    cwd: String,
    state: String,
}

#[get("/")]
fn index() -> &'static str {
    "This is a basic Rust web service."
}

#[get("/")]
fn list(state: State<Vec<Command>>) -> &'static str {
    // println!("Recieved: command = {}", state[0].command state.inner().0.command );
    for task in state.iter() {
        println!("command = {}", task.command);
    }

    "Ok"
}

#[post("/", format = "application/json", data = "<command_json>")]
fn command(command_json: JSON<Command>, state: State<RefCell<Vec<Command>>>) -> &'static str {
    println!("Recieved: command = {}, arguments = {:?}, cwd = {}, state = {}",
             command_json.command,
             command_json.arguments,
             command_json.cwd,
             command_json.state);

    state.get_mut().push(command_json.into_inner());
    "success"
}

#[error(404)]
fn not_found(request: &Request) -> &'static str {
    // println!("Request = {}," request.uri().as_str()); // error: no rules expected the token `request` ?
    "Not found!"
}

// #[get("/stream")]
// fn stream() -> io::Result<Stream<UnixStream>> {
//     UnixStream::connect("/path/to/my/socket").map(|s| Stream::from(s))
// }

fn main() {
    let mut commands: Vec<Command> = Vec::new();

    // Test commands
    let command1 = Command {
        command: "path".to_string(),
        arguments: vec!["argument1".to_string(), "argument2".to_string()],
        cwd: "workdir".to_string(),
        state: "running".to_string(),
    };
    let command2 = Command {
        command: "path2".to_string(),
        arguments: vec!["argument12".to_string(), "argument22".to_string()],
        cwd: "workdir2".to_string(),
        state: "running2".to_string(),
    };
    commands.push(command1);
    commands.push(command2);

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/command", routes![command])
        .mount("/list", routes![list])
        .catch(errors![not_found])
        .manage(commands)
        .launch();
}
