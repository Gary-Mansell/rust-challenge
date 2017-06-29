#![feature(plugin)]
#![plugin(rocket_codegen)]

// extern crate process_supervisor;

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate tokio_process;

use rocket::{Request, State};
use rocket_contrib::JSON;
use std::{thread, time};
use std::sync::{Mutex, Arc};
// use std::process::Command;
use tokio_process::CommandExt;


// mod web {
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
fn list(state: State<Arc<Mutex<Vec<Command>>>>) -> &'static str {
    let arc = state.inner().clone();
    for task in arc.lock().unwrap().iter() {
        println!("command = {}", task.command);
    }

    "Ok"
}

#[post("/", format = "application/json", data = "<command_json>")]
fn command(command_json: JSON<Command>, state: State<Arc<Mutex<Vec<Command>>>>) -> &'static str {
    println!("Recieved: command = {}, arguments = {:?}, cwd = {}, state = {}",
             command_json.command,
             command_json.arguments,
             command_json.cwd,
             command_json.state);

    let arc = state.inner().clone();
    arc.lock().unwrap().push(command_json.into_inner());
    "success"
}

#[error(404)]
fn not_found(request: &Request) -> &'static str {
    // println!("Request = {}," request.uri().as_str()); // error: no rules expected the token `request` ?
    "Not found!"
}
// }

fn main() {
    let mut commands: Vec<Command> = Vec::new();

    // Test commands
    let command1 = Command {
        command: "echo".to_string(),
        arguments: vec!["hello".to_string(), "world".to_string()],
        cwd: "/tmp".to_string(),
        state: "running".to_string(),
    };
    // let command2 = Command {
    //     command: "path2".to_string(),
    //     arguments: vec!["argument12".to_string(), "argument22".to_string()],
    //     cwd: "workdir2".to_string(),
    //     state: "running2".to_string(),
    // };
    commands.push(command1);

    let arc = Arc::new(Mutex::new(commands));

    let rocket = rocket::ignite()
        .mount("/", routes![index])
        .mount("/command", routes![command])
        .mount("/list", routes![list])
        .catch(errors![not_found])
        .manage(arc.clone());

    thread::spawn(move || {

        let mut core = tokio_core::reactor::Core::new().unwrap();

        // Supervisor event loop
        loop {
            println!("Process supervisor thread running...");
            let arc = arc.clone(); // Shadowing
            for task in arc.lock().unwrap().iter() {
                println!("Command = {}", task.command);

                let child = std::process::Command::new(task.command.clone())
                    .current_dir(task.cwd.clone())
                    .args(&task.arguments)
                    .spawn_async(&core.handle());

                let child = child.expect("Failed to execute command thread!");

                match core.run(child) {
                    Ok(status) => println!("Exit status: {}", status),
                    Err(e) => panic!("Failed to wait for exit: {}", e),
                }
            }

            let sleep_duration = time::Duration::new(5, 0); // s, ms
            let now = time::Instant::now();
            thread::sleep(sleep_duration);
            println!("secs = {}", now.elapsed().as_secs());
        }
    });

    rocket.launch();
}
