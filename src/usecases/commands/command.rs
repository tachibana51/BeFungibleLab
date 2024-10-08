// src/usecases/commands/command.rs

use crate::entities::ip_state::IPState;
use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use std::sync::{Arc, Mutex};

pub trait Command {
    fn execute(
        &self,
        ip: Arc<Mutex<IPState>>,
        interpreter: &dyn CommandGrid,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError>;
}

pub trait CommandGrid {
    fn pop(&self, ip: Arc<Mutex<IPState>>) -> Result<usize, InterpreterError>;
    fn move_ip(&self, ip: Arc<Mutex<IPState>>) -> Result<(), InterpreterError>;

    fn add_ip(
        &self,
        new_ip: Arc<Mutex<IPState>>,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError>;

    fn as_any(&self) -> &dyn std::any::Any;
    fn run_ip(
        self: Arc<Self>,
        ip: Arc<Mutex<IPState>>,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError>;
    fn set_value(&self, x: usize, y: usize, value: char) -> Result<(), InterpreterError>;
    fn get_value(&self, x: usize, y: usize) -> Result<char, InterpreterError>;
}
