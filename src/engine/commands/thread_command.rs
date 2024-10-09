// src/usecases/commands/thread_command.rs

use super::command::Command;
use crate::engine::commands::command::CommandGrid;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use std::sync::{Arc, Mutex};

pub struct ThreadCommand;

impl Command for ThreadCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        interpreter: &dyn CommandGrid,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let dy = interpreter.pop(ip.clone())?;
        let dx = interpreter.pop(ip.clone())?;
        let new_ip_state = {
            let ip_locked = ip
                .lock()
                .map_err(|_| InterpreterError::LockError("Failed to lock IPState".to_string()))?;
            IPState {
                ip_x: ip_locked.ip_x,
                ip_y: ip_locked.ip_y,
                direction: crate::entities::Direction::from_dx_dy(dx as isize, dy as isize)
                    .unwrap_or(ip_locked.direction),
                terminated: false,
                stk: ip_locked.stk.clone(),
                string_mode_active: ip_locked.string_mode_active,
            }
        };
        let new_ip = Arc::new(Mutex::new(new_ip_state));
        // Assuming interpreter has a method to add new IPs
        // For simplicity, here we just spawn a new thread
        interpreter.add_ip(new_ip.clone(), io_handler.clone())?;
        interpreter.move_ip(new_ip.clone())?;
        Ok(())
    }
}
