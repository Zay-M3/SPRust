// src/services/command_executor.rs
use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::process::Command; 

#[derive(Debug, Clone)]
pub enum CommandStatus {
    Running(String),
    Progress(String),
    Success(String),
    Error(String),
}

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn execute_powershell_async(script: String, description: String) -> Receiver<CommandStatus> {
        let (tx, rx) = channel();
                
        thread::spawn(move || {
            let _ = tx.send(CommandStatus::Running(description.clone()));
            let _ = tx.send(CommandStatus::Progress(format!("Executing script...")));

            //Here I can used the spawned thread to run the PowerShell script
            let result = Command::new("powershell")
                .args(&["-Command", &script])
                .output();
            
            match result {
                Ok(output) => {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                        for line in stdout.lines() {
                            let _ = tx.send(CommandStatus::Progress(line.to_string()));
                        }
                        let _ = tx.send(CommandStatus::Success(format!("{} completed successfully.", description)));
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                        for line in stderr.lines() {
                            if !line.trim().is_empty() {
                                let _ = tx.send(CommandStatus::Progress(line.to_string()));
                            }
                        }
                        let _ = tx.send(CommandStatus::Error(format!("{} failed.", description)));
                    }
                },
                Err(e) => {
                    let _ = tx.send(CommandStatus::Error(format!("Failed: {}", e)));
                }
            }
        });
        
        rx
    }
}