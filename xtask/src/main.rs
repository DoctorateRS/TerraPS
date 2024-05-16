use std::{
    env::args,
    io::Error as IoError,
    process::{exit, Command},
    sync::mpsc::channel,
    thread::spawn,
};

use anyhow::Result;

fn print_help() {
    println!(
        r#"
xtask must specify a task to run.

Usage: `cargo xtask <task>`

Tasks:
    run
        Start the server.
    watch
        Watch for changes in the project and restart the servers if any file changes.
"#
    );
}

fn start_server(rel: bool) -> Result<()> {
    let (send, receive) = channel();
    let handle = spawn(move || {
        let mut server = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("terra-ps")
            .args(if rel { vec!["--release"] } else { vec![] })
            .spawn()
            .expect("Failed to start server.");

        server.wait()?;
        send.send(()).expect("Failed to send signal.");

        Ok::<(), IoError>(())
    });

    receive.recv().expect("Failed to receive signal.");
    handle.join().expect("Failed to join thread.")?;

    Ok(())
}

fn watch(rel: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("watch")
        .arg("-x")
        .arg(format!("xtask run {}", if rel { "--release" } else { "" }));

    let mut child = cmd.spawn()?;

    child.wait()?;

    Ok(())
}

fn main() -> Result<()> {
    let Some(task) = args().nth(1) else {
        print_help();
        exit(0);
    };
    let release = args().any(|arg| arg == "--release");

    match task.as_str() {
        "run" => start_server(release)?,
        "watch" => watch(release)?,
        _ => {
            println!("invalid task: `{task}`, run `cargo xtask` for a list of tasks");
            exit(1);
        }
    };

    Ok(())
}
