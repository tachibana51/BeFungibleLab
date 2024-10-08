// src/usecases/commands/swap_command.rs

use super::command::Command;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};

/// SwapCommand は '\' コマンドを実行し、スタックのトップ2つの値を交換します。
pub struct SwapCommand;

impl Command for SwapCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        _interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::LockError("Failed to lock IPState".to_string()))?;
        if ip_locked.stk.len() >= 2 {
            let a = ip_locked.stk.pop().unwrap();
            let b = ip_locked.stk.pop().unwrap();
            ip_locked.stk.push(a);
            ip_locked.stk.push(b);
        } else if ip_locked.stk.len() == 1 {
            let a = ip_locked.stk.pop().unwrap();
            ip_locked.stk.push(a);
            ip_locked.stk.push(0);
        } else {
            ip_locked.stk.push(0);
            ip_locked.stk.push(0);
        }
        Ok(())
    }
}
