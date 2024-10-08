// src/usecases/commands/modulo_command.rs

use super::command::Command;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};


pub struct ModuloCommand;

impl Command for ModuloCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        interpreter: &dyn CommandGrid,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let a = interpreter.pop(ip.clone())? as isize;
        let b = interpreter.pop(ip.clone())? as isize;
        if a == 0 {
            io_handler.write_error("Modulo by zero.")?;
            let mut ip_locked = ip
                .lock()
                .map_err(|_| InterpreterError::LockError("Failed to lock IPState".to_string()))?;
            ip_locked.stk.push(0);
        } else {
            let mut ip_locked = ip
                .lock()
                .map_err(|_| InterpreterError::LockError("Failed to lock IPState".to_string()))?;
            ip_locked.stk.push((b % a) as usize);
        }
        Ok(())
    }
}
