Challenge text:

---

# The Technical challenge.

Using Go or Rust language, create a process supervisor daemon, that receives JSON commands over HTTP protocol, which control the state of each running application.
The following command structures should be handled:

{ command: ["/path/to/executable", "argument1", "argument2"],
  cwd: "/path/to/workdir",
  state: "running"
}
{ command: ["/path/to/executable", "argument1", "argument2"],
   state: "stopped"
}

The supervisor daemon should restart applications when they quit, disregarding the exit code until a "stopped" command is received. There's no expectation that multiple instances of the same command will need to be managed.

It would help us if you could upload your solution to GitHub or Bitbucket, and use git to commit your work in logical pieces as you work through the problem, along with a short message explaining what each change does.

For info the output of the technical challenge will be assessed based on the quality of your solution and use of git to show history of the solution as you evolve through the code to the final solution. We will focus less on the number of features you cover.

Because Rust has quite a steep learning curve, we don't expect you to provide a fully working solution if you choose that language. You should look at the following libraries to get started: https://crates.io/crates/serde_json https://crates.io/crates/hyper https://crates.io/crates/tokio-process and in particular https://tokio.rs/ . Good introductions to the language can be found at https://doc.rust-lang.org/book/second-edition/ (the official book) and https://github.com/nrc/r4cppp (Rust for C++ programmers).

Go will have all the necessary utilities included in the standard library. Try using the last stable version, 1.8.
If you decide that a working solution in Go is easier after attempting one in Rust, please submit both so we can discuss your approach and the challenges.