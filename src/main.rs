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
extern crate futures;
extern crate crossbeam;

use rocket::{Request, State};
use rocket_contrib::JSON;
use std::sync::{Mutex, Arc};
use std::process::Command;
use tokio_process::CommandExt;
use futures::{Future, Stream, Async};
use crossbeam::sync::chase_lev;

// mod web {
#[derive(Deserialize)]
struct Event {
    command: String,
    arguments: Vec<String>,
    cwd: String,
    state: String,
}

impl Event {
    fn to_process(self) -> Command {
        let mut command = Command::new(&self.command);
        command.current_dir(&self.cwd);
        command.args(&self.arguments);
        return command;
    }
}

struct EventQueue(chase_lev::Stealer<Event>);
struct Error;
impl Stream for EventQueue {
    type Item = Event;
    type Error = Error;

    fn poll(&mut self) -> futures::Poll<Option<Self::Item>, Self::Error> {
        match self.0.steal() {
            chase_lev::Steal::Data(event) => Ok(Async::Ready(Some(event))), // .to_process()
            _ => Ok(Async::NotReady),
        }
    }
}


#[get("/")]
fn index() -> &'static str {
    "This is a basic Rust web service."
}

#[get("/")]
fn list(state: State<Arc<Mutex<crossbeam::sync::chase_lev::Worker<Event>>>>) -> &'static str {
    // let arc = state.inner().clone();
    // for task in arc.lock().unwrap().iter() {
    //     println!("command = {}", task.command);
    // }

    "Ok"
}

#[post("/", format = "application/json", data = "<command_json>")]
fn command(command_json: JSON<Event>, state: State<Arc<Mutex<crossbeam::sync::chase_lev::Worker<Event>>>>) -> &'static str {
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
    // Test event
    let event1 = Event {
        command: "echo".to_string(),
        arguments: vec!["hello".to_string(), "world".to_string()],
        cwd: "/tmp".to_string(),
        state: "running".to_string(),
    };

    let (worker, stealer) = chase_lev::deque();
    let arc = Arc::new(Mutex::new(worker));
    let rocket = rocket::ignite()
        .mount("/", routes![index])
        .mount("/command", routes![command])
        .mount("/list", routes![list])
        .catch(errors![not_found])
        .manage(arc.clone());
    
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    let process_manager = EventQueue(stealer).for_each(|event| {
                       return event.to_process()
                        .spawn_async(&handle)
                        .and_then(|_success| Ok(()))
                        .or_else(|_failed| Ok(()));
    });
    core.run(process_manager); //.expect("Failed to run process manager!");
    arc.lock().unwrap().push(event1);

    rocket.launch();
}
