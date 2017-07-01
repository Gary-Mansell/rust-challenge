

    //    let timer = Timer::default();
    //     let sleep = timer.sleep(Duration::from_millis(500));
    //     sleep.wait();

    // thread::spawn(move || {
    //     // let mut core = tokio_core::reactor::Core::new().unwrap();
    //     // core.run();
    //     // Supervisor event loop
    //     loop {
    //         println!("Process supervisor thread running...");
    //         let arc = arc.clone(); // Shadowing
    //         for task in arc.lock().unwrap().iter() {
    //             task.poll();
    //             // if task.state.trim().to_lowercase() != "running" {
    //             //     println!("Ignoring! {}", task.command);
    //             //     continue;
    //             // }
    //             // println!("Executing... {}", task.command);
    //             // let output = std::process::Command::new(&task.command)
    //             //     .current_dir(&task.cwd)
    //             //     .args(&task.arguments)
    //             //     .output_async(&core.handle());
    //             // // let output = core.run(output).expect("Failed to execute command thread!");
    //             // let output = output.expect("Failed to execute command thread!");
    //             // match core.run(output) {
    //             //     Ok(status) => println!("Exit status: {}", status),
    //             //     Err(e) => panic!("Failed to wait for exit: {}", e),
    //             // }
    //         }
    //         let sleep_duration = time::Duration::new(5, 0); // s, ms
    //         let now = time::Instant::now();
    //         thread::sleep(sleep_duration);
    //         println!("secs = {}", now.elapsed().as_secs());
    //     }
    // });

    // let command1 = Command {
    //     command: "echo".to_string(),
    //     arguments: vec!["hello".to_string(), "world".to_string()],
    //     cwd: "/tmp".to_string(),
    //     state: "running".to_string(),
    // };

    // let mut core = tokio_core::reactor::Core::new().unwrap();
    // let mut command = std::process::Command::new(&command1.command);
    // command.current_dir(&command1.cwd);
    // command.args(&command1.arguments);
    // let mut child = command.spawn_async(&core.handle()).unwrap();
    // let status = core.run(&mut child).unwrap();
    // drop(child.kill());