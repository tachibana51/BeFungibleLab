// src/usecases/commands/print_char_command.rs

use super::command::Command;
use crate::engine::commands::command::CommandGrid;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use std::sync::{Arc, Mutex};

pub struct PrintCharCommand;

impl Command for PrintCharCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        _interpreter: &dyn CommandGrid,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let value = {
            let mut ip_locked = ip
                .lock()
                .map_err(|_| InterpreterError::LockError("Failed to lock IPState".to_string()))?;
            ip_locked.stk.pop().unwrap_or(0)
        };
        let c = std::char::from_u32(value as u32).unwrap_or('\u{FFFD}');
        io_handler.write_output(&c.to_string())
    }
}
