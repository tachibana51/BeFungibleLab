// src/usecases/commands/put_command.rs

use super::command::Command;
use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use crate::usecases::commands::command::CommandGrid;
use std::sync::{Arc, Mutex};


pub struct PutCommand;

impl Command for PutCommand {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        interpreter: &dyn CommandGrid,
        _io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let y = interpreter.pop(ip.clone())?;
        let x = interpreter.pop(ip.clone())?;
        let v = interpreter.pop(ip.clone())? as usize;
        let value = std::char::from_u32(v as u32).unwrap_or('\u{FFFD}');
        interpreter.set_value(x, y, value)?;
        Ok(())
    }
}
