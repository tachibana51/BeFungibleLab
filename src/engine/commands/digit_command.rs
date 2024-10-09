// src/usecases/commands/digit_command.rs

use super::command::Command;
use crate::engine::commands::command::CommandGrid;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use std::sync::{Arc, Mutex};

pub struct DigitCommand {
    value: usize,
}

impl DigitCommand {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

impl Command for DigitCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        _interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
        ip_locked.stk.push(self.value);
        Ok(())
    }
}
