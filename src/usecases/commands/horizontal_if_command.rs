// src/usecases/commands/horizontal_if_command.rs

use super::command::Command;
use crate::entities::{ip_state::IPState, Direction};
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};

/// HorizontalIfCommand は '_' コマンドを実行し、stackのtopで水平方向に条件分岐します。
pub struct HorizontalIfCommand;

impl Command for HorizontalIfCommand {
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
            ip_locked.direction = Direction::Right
        } else {
            ip_locked.direction = Direction::Left
        };
        Ok(())
    }
}
