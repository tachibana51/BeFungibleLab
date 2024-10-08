// src/usecases/commands/add_command.rs

use super::command::{Command, CommandGrid};
use crate::interfaces::IOHandle;
use crate::{entities::IPState, errors::InterpreterError};
use std::sync::{Arc, Mutex};

pub struct AddCommand;

impl Command for AddCommand {
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
        ip_locked.stk.push((b + a) as usize);
        Ok(())
    }
}
