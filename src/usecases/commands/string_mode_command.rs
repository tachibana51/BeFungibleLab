// src/usecases/commands/string_mode_command.rs

use super::command::Command;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};

/// StringModeCommand は '"' コマンドを実行し、文字列モードをトグルします。
pub struct StringModeCommand;

impl Command for StringModeCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        _interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
        ip_locked.string_mode_active = !ip_locked.string_mode_active;
        Ok(())
    }
}
