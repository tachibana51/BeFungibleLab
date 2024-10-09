// src/usecases/commands/multiply_command.rs

use super::command::Command;
use crate::engine::commands::command::CommandGrid;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use std::sync::{Arc, Mutex};

pub struct MultiplyCommand;

impl Command for MultiplyCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let a = interpreter.pop(ip.clone())? as isize;
        let b = interpreter.pop(ip.clone())? as isize;
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::LockError("Failed to lock IPState".to_string()))?;
        ip_locked.stk.push((b * a) as usize);
        Ok(())
    }
}
