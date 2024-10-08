use super::command::{Command, CommandGrid};
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use std::sync::{Arc, Mutex};


pub struct ReadNumberCommand;

impl Command for ReadNumberCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        _interpreter: &dyn CommandGrid,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let mut ip_locked = ip
            .lock()
            .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
        ip_locked.stk.push(io_handler.read_number().unwrap_or(0));
        Ok(())
    }
}
