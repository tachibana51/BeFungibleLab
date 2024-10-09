// src/main.rs
mod engine;
mod entities;
mod errors;
mod interfaces;
use clap::{ArgAction, Parser};
use engine::interpreter::Interpreter;
use entities::code_grid::CodeGrid;
use errors::InterpreterError;
use interfaces::command_registry::CommandRegistry;
use interfaces::ConsoleIOHandler;
use std::sync::Arc;
use std::{io, thread};
/// Befunge Interpreter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the Befunge program file
    #[arg(value_name = "FILE")]
    file: String,

    /// Enable step-by-step execution
    #[arg(short, long, action = ArgAction::SetTrue)]
    step: bool,

    /// Enable debug (trace) mode
    #[arg(short, long, action = ArgAction::SetTrue)]
    debug: bool,
}
fn main() -> Result<(), InterpreterError> {
    // parse args
    let cli = Cli::parse();

    // init grid
    let program = CodeGrid::load(&cli.file)?;

    // init command registry
    let command_registry = Arc::new(CommandRegistry::new());

    // Initialize ThreadPool with 3 threads.
    // init Interpreter
    let interpreter = Arc::new(Interpreter::new(
        program,
        if cli.step == true { true } else { cli.debug },
        command_registry,
    ));

    // init IOHandler
    let io_handler = Arc::new(ConsoleIOHandler::new(if cli.step || cli.debug {
        true
    } else {
        false
    }));

    if cli.step {
        interpreter.enable_step_mode();
    }

    //  interpreter thread
    let interpreter_clone = Arc::clone(&interpreter);
    let io_handler_clone = Arc::clone(&io_handler);
    let run_handle = thread::spawn(move || {
        if let Err(e) = interpreter_clone.run(io_handler_clone) {
            eprintln!("Interpreter Error: {}", e);
        }
        if cli.step || cli.debug {
            eprintln!("[*] Program has Terminated.");
            if cli.step {
                eprintln!("[*] Type 'q' to quit. ");
            }
        }
    });

    // step mode loop
    if cli.step {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input.eq_ignore_ascii_case("q") {
                interpreter.clone().disable_step_mode();
                break;
            }
            interpreter.step()?;
        }
    }

    // wait for interpreter thread
    run_handle.join().expect("Interpreter thread panicked");

    Ok(())
}
