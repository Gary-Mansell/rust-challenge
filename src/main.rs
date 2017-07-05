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
use std::thread;
use tokio_process::CommandExt;
use futures::{Future, Stream, Sink};
use futures::sync::mpsc;

// mod web {
#[derive(Clone, Deserialize)]
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
        command
    }
}

#[get("/")]
fn index() -> &'static str {
    "This is a basic Rust web service."
}

#[post("/", format = "application/json", data = "<command_json>")]
fn command(command_json: JSON<Event>,
           state: State<Arc<Mutex<mpsc::Sender<Event>>>>)
           -> &'static str {
    println!("Recieved: command = {}, arguments = {:?}, cwd = {}, state = {}",
             command_json.command,
             command_json.arguments,
             command_json.cwd,
             command_json.state);

    let arc = state.inner().clone();
    arc.lock()
        .unwrap()
        .start_send(command_json.into_inner())
        .expect("Could not add to event to channel!");
    "Command added"
}

#[error(404)]
fn not_found(request: &Request) -> &'static str {
    // error: no rules expected the token `request` ?
    // println!("Request = {}," request.uri().as_str());
    "Not found!"
}
// }


fn main() {
    let (worker, stealer) = mpsc::channel(100);
    let arc = Arc::new(Mutex::new(worker.clone()));
    let arc2 = arc.clone();

    let t1 = thread::spawn(move || {
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();

        let process_manager = stealer.for_each(move |event: Event| {
            let arc3 = arc2.clone();
            event
                .clone()
                .to_process()
                .spawn_async(&handle)
                .and_then(|_child| {
                    println!("Spawned {:?}", _child.id());
                    handle.spawn(_child
                                     .and_then(move |_status| {
                                                   println!("Success!");
                                                   arc3.lock().unwrap().start_send(event)
                                                   .expect("Could not add to event to channel!");
                                                   Ok(())
                                               })
                                     .or_else(|_status| {
                                                  println!("Failed!");
                                                  Err(())
                                              }));
                    Ok(())
                })
                .expect("");

            Ok(())
        });

        core.run(process_manager)
            .expect("Processor thread crashed!");
    });

    thread::spawn(move || {
        println!("Launching server...");
        rocket::ignite()
            .mount("/", routes![index])
            .mount("/command", routes![command])
            .catch(errors![not_found])
            .manage(arc.clone())
            .launch();
    });

    t1.join().expect("Server thread crashed!");
}
