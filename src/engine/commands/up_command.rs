// src/usecases/commands/up_command.rs

use super::command::Command;
use crate::engine::commands::command::CommandGrid;
use crate::entities::ip_state::IPState;
use crate::entities::Direction;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use std::sync::{Arc, Mutex};

pub struct UpCommand;

impl Command for UpCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        _interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
        ip_locked.direction = Direction::Up;
        Ok(())
    }
}
