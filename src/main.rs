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
use futures::{Future, Stream, Async, Sink};
use futures::sync::mpsc;
use crossbeam::sync::chase_lev;

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

#[get("/")]
fn list(state: State<Arc<Mutex<mpsc::Sender<Event>>>>) -> &'static str {
    // let arc = state.inner().clone();
    // for task in arc.lock().unwrap().iter() {
    //     println!("command = {}", task.command);
    // }

    "Ok"
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
    arc.lock().unwrap().start_send(command_json.into_inner());
    "Command added"
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
        command: "/tmp/script.sh".to_string(),
        arguments: vec![],
        cwd: "/tmp".to_string(),
        state: "running".to_string(),
    };

    let (worker, stealer) = mpsc::channel(100);
    let arc = Arc::new(Mutex::new(worker.clone()));

    let t1 = thread::spawn(move || {
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();

        let process_manager = stealer.for_each(|event: Event| {
            event
                .clone()
                .to_process()
                .spawn_async(&handle)
                .and_then(|_child| {
                    println!("Spawned {:?}", _child.id());
                    handle.spawn(_child
                                     .and_then(move |_status| {
                                                   println!("Success!");
                                                //    arc3.lock().unwrap().push(event);
                                                   Ok(())
                                               })
                                     .or_else(|_status| {
                                                  println!("Failed!");
                                                  Err(())
                                              }));
                    Ok(())
                });

            Ok(())
        });

        let _ = core.run(process_manager);
        println!("Done and dusted");
    });

    println!("Launching");

    let t2 = thread::spawn(move || {
    let rocket = rocket::ignite()
        .mount("/", routes![index])
        .mount("/command", routes![command])
        .mount("/list", routes![list])
        .catch(errors![not_found])
        .manage(arc.clone())
        .launch();
    });

    t1.join();
}
