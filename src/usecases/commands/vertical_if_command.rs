// src/usecases/commands/vertical_if_command.rs

use super::command::Command;
use crate::entities::{ip_state::IPState, Direction};
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};

/// VertIfCommand は '|' コマンドを実行し、stackのtopで垂直方向に条件分岐します。
pub struct VerticalIfCommand;

impl Command for VerticalIfCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let a = interpreter.pop(ip.clone())?;
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::LockError("Failed to lock IPState".to_string()))?;
        if a == 0 {
            ip_locked.direction = Direction::Down
        } else {
            ip_locked.direction = Direction::Up
        };
        Ok(())
    }
}
