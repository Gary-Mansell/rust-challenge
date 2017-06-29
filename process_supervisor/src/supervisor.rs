#[derive(Deserialize)]
struct Command {
    command: String,
    arguments: Vec<String>,
    cwd: String,
    state: String,
}