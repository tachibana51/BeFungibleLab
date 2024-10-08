// src/usecases/commands/subtract_command.rs

use super::command::Command;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};

/// SubtractCommand は '-' コマンドを実行し、スタックから2つの値を引きます。
pub struct SubtractCommand;

impl Command for SubtractCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let b = interpreter.pop(ip.clone())? as isize;
        let a = interpreter.pop(ip.clone())? as isize;
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::LockError("Failed to lock IPState".to_string()))?;
        ip_locked.stk.push((a - b) as usize);
        Ok(())
    }
}
