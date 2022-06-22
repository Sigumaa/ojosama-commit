use std::process::Command;

fn main() {
    let commit_message = "test message";
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .spawn()
        .expect("failed to start `git commit`");
}
