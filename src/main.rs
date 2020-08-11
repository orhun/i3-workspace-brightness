extern crate i3ipc;
use hashbrown::HashMap;
use i3ipc::event::Event;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use std::env;
use std::error::Error;
use std::process::Command;

fn execute(cmd: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    Ok(
        String::from_utf8_lossy(&Command::new(cmd).args(args).output()?.stdout)
            .trim()
            .to_string(),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut listener = I3EventListener::connect().expect("Could not connect to i3wm");
    listener.subscribe(&[Subscription::Workspace])?;
    let mut workspaces = HashMap::<i64, f32>::new();
    let (get_brightness, set_brightness) = (
        env::var("GET_BRIGHTNESS").unwrap_or_else(|_| String::from("xbacklight -get")),
        env::var("SET_BRIGHTNESS").unwrap_or_else(|_| String::from("xbacklight -set {}")),
    );
    for event in listener.listen() {
        if let Event::WorkspaceEvent(e) = event? {
            let brightness = execute("sh", &["-c", &get_brightness])?.parse()?;
            if let Some(old) = e.old {
                workspaces.insert(old.id, brightness);
            }
            if let Some(current) = e.current {
                if let Some(level) = workspaces.get(&current.id) {
                    execute(
                        "sh",
                        &["-c", &set_brightness.replace("{}", &level.to_string())],
                    )?;
                }
            }
        }
    }
    Ok(())
}
