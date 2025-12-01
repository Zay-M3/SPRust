
use std::sync::mpsc::Receiver;
use crate::hooks::command_executor::CommandStatus;
use crate::utils::vectores_strings::get_iis_security_commands_map;
use crate::models::config::AppSettings;



pub fn commands_queue_receiver(active_command: &mut Vec<(String, Receiver<CommandStatus>)>, server_settings: &AppSettings) {
    // Implementation of commands queue receiver
    for feacture in get_iis_security_commands_map() {
            let (feature_id, (_label, command_fn)) = feacture;
            if let Some(enabled) = server_settings.set_security_checkboxes.get(feature_id) {
                if *enabled {
                    let receiver = command_fn();
                    active_command.push((feature_id.to_string(), receiver));
                }
            }
    }
}

pub fn excute_commands_queue(active_command: &mut Vec<(String, Receiver<CommandStatus>)>, last_status: &mut Option<CommandStatus>, logs: &mut Vec<String>, completed: &mut Vec<usize>, ui: &egui::Ui) {
    
    for (i, (feature_id, receiver)) in active_command.iter().enumerate() {
        if let Ok(status) = receiver.try_recv() {
            *last_status = Some(status.clone());
            match status {
                CommandStatus::Running(message) => {
                    logs.push(format!("• [{}] {}", feature_id, message));
                },
                CommandStatus::Success(message) => {
                    logs.push(format!("→ [{}] {}", feature_id, message));
                    completed.push(i);
                },
                CommandStatus::Progress(message) => {
                    logs.push(format!("• [{}] {}", feature_id, message));
                },              
                CommandStatus::Error(message) => {
                    logs.push(format!("✗ [{}] {}", feature_id, message));
                    completed.push(i);
                },
            }
            ui.ctx().request_repaint(); 
        }
    }
}



pub fn clean_commands_queue(active_command: &mut Vec<(String, Receiver<CommandStatus>)>, completed: &Vec<usize>) {
    // Implementation of cleaning commands queue
    for &index in completed.iter().rev() {
        active_command.remove(index);
    }
}