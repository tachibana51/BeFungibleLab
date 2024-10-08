// src/usecases/commands/print_number_command.rs

use super::command::Command;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};

/// PrintNumberCommand は '.' コマンドを実行し、スタックのトップの数値を出力します。
pub struct PrintNumberCommand;

impl Command for PrintNumberCommand {
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
        io_handler.write_output(&format!("{} ", value))
    }
}
