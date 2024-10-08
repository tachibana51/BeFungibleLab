use super::command::{Command, CommandGrid};
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use std::sync::{Arc, Mutex};

/// GraterThanCommand は '`' コマンドを実行し、IPを移動させます。
pub struct GraterThanCommand;

impl Command for GraterThanCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        _interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
        let a = ip_locked.stk.pop();
        let b = ip_locked.stk.pop();
        ip_locked.stk.push(if b > a { 1 } else { 0 });
        Ok(())
    }
}
