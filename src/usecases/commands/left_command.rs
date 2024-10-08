// src/usecases/commands/left_command.rs

use super::command::Command;
use crate::entities::ip_state::IPState;
use crate::entities::Direction;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};

/// LeftCommand は '<' コマンドを実行し、IP の方向を左に変更します。
pub struct LeftCommand;

impl Command for LeftCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        _interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
        ip_locked.direction = Direction::Left;
        Ok(())
    }
}
