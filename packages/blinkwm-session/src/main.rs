use std::process::{Command, Child};
use std::thread;
use std::time::Duration;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

struct Component {
    name: String,
    command: String,
    child: Option<Child>,
}

impl Component {
    fn new(name: &str, command: &str) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
            child: None,
        }
    }

    fn spawn(&mut self) {
        info!("Spawning component: {}", self.name);
        // Using full path if needed, but for now we'll assume they are in PATH or relative
        let cmd = if std::path::Path::new(&self.command).exists() {
            self.command.clone()
        } else {
            format!("./target/debug/{}", self.command)
        };

        match Command::new(&cmd).spawn() {
            Ok(child) => self.child = Some(child),
            Err(e) => error!("Failed to spawn {} ({}): {}", self.name, cmd, e),
        }
    }

    fn check_and_restart(&mut self) {
        if let Some(ref mut child) = self.child {
            match child.try_wait() {
                Ok(Some(status)) => {
                    error!("Component {} exited with status: {}. Restarting...", self.name, status);
                    self.spawn();
                }
                Ok(None) => {} // Still running
                Err(e) => {
                    error!("Error checking status of {}: {}. Restarting...", self.name, e);
                    self.spawn();
                }
            }
        } else {
            self.spawn();
        }
    }
}

fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Initializing BlinkWM Session...");

    let mut components = vec![
        Component::new("blinkwm", "blinkwm"),
        Component::new("blinkwm-bar", "blinkwm-bar"),
    ];

    // Initial spawn
    for component in &mut components {
        component.spawn();
        // Small delay to let WM take over before bar tries to connect
        thread::sleep(Duration::from_millis(500));
    }

    info!("Session active. Monitoring components...");

    loop {
        for component in &mut components {
            component.check_and_restart();
        }
        thread::sleep(Duration::from_secs(2));
    }
}
