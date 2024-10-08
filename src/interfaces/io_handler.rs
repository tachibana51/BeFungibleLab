// src/interfaces/io_handler.rs

use crate::errors::InterpreterError;

pub trait IOHandle {
    fn write_output(&self, output: &str) -> Result<(), InterpreterError>;
    fn write_error(&self, error: &str) -> Result<(), InterpreterError>;
    fn read_number(&self) -> Result<usize, InterpreterError>;
    fn read_char(&self) -> Result<char, InterpreterError>;
    fn display_stack(&self, stack: &[usize]) -> Result<(), InterpreterError>;
    fn display_grid(
        &self,
        grid: &[Vec<char>],
        ip_x: usize,
        ip_y: usize,
    ) -> Result<(), InterpreterError>;
}
