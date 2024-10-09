// src/usecases/commands/duplicate_top_command.rs

use super::command::Command;
use crate::engine::commands::command::CommandGrid;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use std::sync::{Arc, Mutex};

pub struct DuplicateTopCommand;

impl Command for DuplicateTopCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        _interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::LockError("Failed to lock IPState".to_string()))?;
        if let Some(&value) = ip_locked.stk.last() {
            ip_locked.stk.push(value);
        } else {
            ip_locked.stk.push(0);
            ip_locked.stk.push(0);
        }
        Ok(())
    }
}
