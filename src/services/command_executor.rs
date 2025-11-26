// src/services/command_executor.rs
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::process::Command;

#[derive(Debug, Clone)]
pub enum CommandStatus {
    Running(String),
    Success(String),
    Error(String),
}

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn execute_powershell_async(script: String, description: String) -> Receiver<CommandStatus> {
        let (tx, rx) = channel();
        
        println!("CommandExecutor: Spawning thread for: {}", description);
        println!("CommandExecutor: Script: {}", script);
        
        thread::spawn(move || {
            println!("Thread started!");
            let _ = tx.send(CommandStatus::Running(description.clone()));
            
            println!("Executing PowerShell command...");
            let result = Command::new("powershell")
                .args(&["-Command", &script])
                .output();
            
            match result {
                Ok(output) => {
                    println!("Command executed. Status: {}", output.status);
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                        println!("Success output: {}", stdout);
                        let _ = tx.send(CommandStatus::Success(stdout));
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                        println!("Error output: {}", stderr);
                        let _ = tx.send(CommandStatus::Error(stderr));
                    }
                },
                Err(e) => {
                    println!("Failed to execute: {}", e);
                    let _ = tx.send(CommandStatus::Error(format!("Failed: {}", e)));
                }
            }
            println!("Thread finished!");
        });
        
        println!("CommandExecutor: Thread spawned, returning receiver");
        rx
    }
}