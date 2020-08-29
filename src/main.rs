extern crate i3ipc;
use hashbrown::HashMap;
use i3ipc::event::inner::WorkspaceChange;
use i3ipc::event::Event;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use std::env;
use std::error::Error;
use std::process::Command;

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
            if e.change == WorkspaceChange::Focus {
                let brightness = String::from_utf8_lossy(
                    &Command::new("sh")
                        .args(&["-c", &get_brightness])
                        .output()?
                        .stdout,
                )
                .trim()
                .parse()?;
                if let Some(old) = e.old {
                    workspaces.insert(old.id, brightness);
                }
                if let Some(current) = e.current {
                    if let Some(level) = workspaces.get(&current.id) {
                        Command::new("sh")
                            .args(&["-c", &set_brightness.replace("{}", &level.to_string())])
                            .spawn()?
                            .wait()?;
                    }
                }
            }
        }
    }
    Ok(())
}
