#![feature(plugin)]
#![plugin(rocket_codegen)]

// extern crate process_supervisor;

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate crossbeam;
extern crate futures;
extern crate tokio_core;
extern crate tokio_process;

use futures::sync::mpsc;
use futures::{Future, Sink, Stream};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio_process::CommandExt;

mod web {
    use futures::sync::mpsc;
    use futures::Sink;
    use rocket::{Request, State};
    use rocket_contrib::JSON;
    use std::collections::HashSet;
    use std::process::Command;
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Deserialize, Debug)]
    pub struct Event {
        pub command: String,
        pub arguments: Vec<String>,
        pub cwd: String,
        pub state: String,
    }

    impl Event {
        pub fn to_process(self) -> Command {
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
    fn command(
        command_json: JSON<Event>,
        worker_state: State<Arc<Mutex<mpsc::Sender<Event>>>>,
        event_map_state: State<Arc<Mutex<HashSet<String>>>>,
    ) -> &'static str {
        println!(
            "Recieved: command = {}, arguments = {:?}, cwd = {}, state = {}",
            command_json.command, command_json.arguments, command_json.cwd, command_json.state
        );

        let event_map_arc = event_map_state.inner().clone();
        match command_json.state.clone().trim().to_lowercase().as_ref() {
            "running" => {
                println!("Executing command...");
                event_map_arc
                    .lock()
                    .unwrap()
                    .insert(command_json.command.clone());

                let worker_arc = worker_state.inner().clone();
                worker_arc
                    .lock()
                    .unwrap()
                    .start_send(command_json.into_inner())
                    .expect("Could not add to event to channel!");
                "Executing command..."
            }
            "stopped" => {
                println!("Ignoring command...(Stopping if already executing)");
                event_map_arc.lock().unwrap().remove(&command_json.command);
                "Ignoring command...(Stopping if already executing)"
            }
            _ => {
                println!("Unhandled state! Command not being executed");
                "Unhandled state! Command not being executed"
            }
        }
    }

    #[error(404)]
    fn not_found(request: &Request) -> &'static str {
        // error: no rules expected the token `request` ?
        // println!("Request = {}," request.uri().as_str());
        "Not found!"
    }
}

fn main() {
    let (worker, stealer) = mpsc::channel(100);
    let worker_arc = Arc::new(Mutex::new(worker.clone()));
    let worker_arc_2 = worker_arc.clone();
    let event_map_arc: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
    let event_map_arc_2 = event_map_arc.clone();

    let t1 = std::thread::spawn(move || {
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();

        let process_manager = stealer.for_each(move |event: web::Event| {
            // There has to be a better approach than cloning these values
            let worker_arc_3 = worker_arc_2.clone();
            let worker_arc_4 = worker_arc_2.clone();
            let event_map_arc_3 = event_map_arc_2.clone();
            let event_2 = event.clone();
            event
                .clone()
                .to_process()
                .spawn_async(&handle)
                .and_then(|_child| {
                    handle.spawn(
                        _child
                            .and_then(move |_status| {
                                println!("Command executed successfully! Running again...");
                                if event_map_arc_3.lock().unwrap().contains(&event.command) {
                                    worker_arc_3.lock().unwrap().start_send(event);
                                }
                                Ok(())
                            })
                            .or_else(move |_status| {
                                println!("Command execution FAILED! Retrying...");
                                worker_arc_4.lock().unwrap().start_send(event_2);
                                Err(())
                            }),
                    );
                    Ok(())
                })
                .expect("");

            Ok(())
        });

        core.run(process_manager)
            .expect("Processor thread crashed!");
    });

    std::thread::spawn(move || {
        println!("Launching server...");
        rocket::ignite()
            .mount("/", routes![web::index])
            .mount("/command", routes![web::command])
            .catch(errors![web::not_found])
            .manage(worker_arc.clone())
            .manage(event_map_arc.clone())
            .launch();
    });

    t1.join().expect("Server thread crashed!");
}
