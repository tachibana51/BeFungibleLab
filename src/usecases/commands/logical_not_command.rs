// src/usecases/commands/logical_not_command.rs

use super::command::Command;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};

/// LogicalNotCommand は '!' コマンドを実行し、スタックのトップの値を否定します。
pub struct LogicalNotCommand;

impl Command for LogicalNotCommand {
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
        ip_locked.stk.push(if a == 0 { 1 } else { 0 });
        Ok(())
    }
}
