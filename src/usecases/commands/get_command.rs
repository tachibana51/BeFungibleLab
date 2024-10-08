// src/usecases/commands/logical_not_command.rs

use super::command::Command;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};


pub struct GetCommand;

impl Command for GetCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let y = interpreter.pop(ip.clone())?;
        let x = interpreter.pop(ip.clone())?;

        // 指定位置の値を取得してスタックにプッシュ
        let value = interpreter.get_value(x, y)?;
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
        ip_locked.stk.push(value as usize);
        Ok(())
    }
}
