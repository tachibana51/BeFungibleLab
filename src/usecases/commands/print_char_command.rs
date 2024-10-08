// src/usecases/commands/print_char_command.rs

use super::command::Command;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};

/// PrintCharCommand は ',' コマンドを実行し、スタックのトップの値を文字として出力します。
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
