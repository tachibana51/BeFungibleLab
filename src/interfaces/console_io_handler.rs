// src/interfaces/console_io_handler.rs

use crate::errors::InterpreterError;
use crate::interfaces::IOHandle;
use std::io::{self, Read, Write};

pub struct ConsoleIOHandler {
    is_verbose_mode: bool,
}

impl ConsoleIOHandler {
    pub fn new(is_verbose_mode: bool) -> Self {
        ConsoleIOHandler { is_verbose_mode }
    }
}

impl IOHandle for ConsoleIOHandler {
    fn write_output(&self, output: &str) -> Result<(), InterpreterError> {
        if self.is_verbose_mode {
            println!();
            println!("[STDOUT] {}", output);
            io::stdout()
                .flush()
                .map_err(|e| InterpreterError::IoError(e))
        } else {
            print!("{}", output);
            io::stdout()
                .flush()
                .map_err(|e| InterpreterError::IoError(e))
        }
    }

    fn write_error(&self, error: &str) -> Result<(), InterpreterError> {
        if self.is_verbose_mode {
            eprintln!();
            eprintln!("[STDERR] {}", error);
            io::stderr()
                .flush()
                .map_err(|e| InterpreterError::IoError(e))
        } else {
            eprintln!("{}", error);
            io::stderr()
                .flush()
                .map_err(|e| InterpreterError::IoError(e))
        }
    }

    fn read_number(&self) -> Result<usize, InterpreterError> {
        if self.is_verbose_mode {
            println!("[Reading Num] >");
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| InterpreterError::IoError(e))?;
        input
            .trim()
            .parse::<usize>()
            .map_err(|_| InterpreterError::ParseError("Invalid number".to_string()))
    }

    fn read_char(&self) -> Result<char, InterpreterError> {
        let mut buffer = [0; 1];
        if self.is_verbose_mode {
            println!("[Reading Char] >");
        }
        io::stdin()
            .read_exact(&mut buffer)
            .map_err(|e| InterpreterError::IoError(e))?;
        Ok(buffer[0] as char)
    }

    fn display_stack(&self, stack: &[usize]) -> Result<(), InterpreterError> {
        print!("Ord Stack: [");
        for (i, value) in stack.iter().enumerate() {
            print!("{}", value);
            if i < stack.len() - 1 {
                print!(", ");
            }
        }
        println!("]");
        print!("Chr Stack: [");
        for (i, value) in stack.iter().enumerate() {
            print!(
                "{:#?}",
                match char::from_u32(*value as u32) {
                    Some(ov) => ov,
                    None => char::from_digit(*value as u32, 10).unwrap_or_default(),
                }
            );
            if i < stack.len() - 1 {
                print!(", ");
            }
        }
        println!("]");
        Ok(())
    }
    fn display_grid(
        &self,
        grid: &[Vec<char>],
        ip_x: usize,
        ip_y: usize,
    ) -> Result<(), InterpreterError> {
        println!("[IP on Grid] : ");
        for (y, row) in grid.iter().enumerate() {
            print!("{:5}", y);
            for (x, &cell) in row.iter().enumerate() {
                if x == ip_x && y == ip_y {
                    print!("[{}]", cell);
                } else {
                    print!(" {} ", cell);
                }
            }
            println!(); // 改行
        }

        Ok(())
    }
}
