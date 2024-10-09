// src/usecases/interpreter.rs

use crate::engine::commands::command::CommandGrid;
use crate::entities::{CodeGrid, Direction, IPState};
use crate::errors::InterpreterError;
use crate::interfaces::{CommandResolve, IOHandle};
use std::any::Any;
use std::io::Write;
use std::sync::{Arc, Condvar, Mutex};
use std::{char, io, thread};

pub struct Interpreter {
    program: CodeGrid,
    debug_mode: bool,
    ips: Arc<Mutex<Vec<Arc<Mutex<IPState>>>>>,
    command_registry: Arc<dyn CommandResolve + Send + Sync>,
    threads: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
    step_mode: Arc<(Mutex<bool>, Condvar)>,
}

impl Clone for Interpreter {
    fn clone(&self) -> Self {
        Self {
            program: self.program.clone(),
            debug_mode: self.debug_mode,
            ips: Arc::clone(&self.ips),
            command_registry: Arc::clone(&self.command_registry),
            threads: Arc::clone(&self.threads),
            step_mode: Arc::clone(&self.step_mode),
        }
    }
}

impl Interpreter {
    const HEIGHT: usize = 1024;
    const WIDTH: usize = 1024;

    pub fn new(
        program: CodeGrid,
        debug_mode: bool,
        command_registry: Arc<dyn CommandResolve + Send + Sync>,
    ) -> Self {
        Self {
            program,
            debug_mode,
            ips: Arc::new(Mutex::new(Vec::new())),
            command_registry,
            threads: Arc::new(Mutex::new(Vec::new())),
            step_mode: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    /// ステップ実行モードがアクティブかどうかを確認するメソッド
    fn is_step_mode_active(&self) -> bool {
        let (lock, _) = &*self.step_mode;
        let step = lock.lock().unwrap();
        *step
    }

    /// ステップ実行モードで一時停止し、次のステップの入力を待つメソッド
    fn wait_for_step(&self) -> Result<(), InterpreterError> {
        let (lock, cvar) = &*self.step_mode;
        let mut step = lock.lock().unwrap();
        while *step {
            step = cvar.wait(step).unwrap();
        }
        Ok(())
    }

    /// ステップを進めるためのメソッド（外部から呼ばれる）
    pub fn step(&self) -> Result<(), InterpreterError> {
        let (lock, cvar) = &*self.step_mode;
        let mut step = lock.lock().unwrap();
        *step = false;
        cvar.notify_one(); // 次のステップを実行させる
        Ok(())
    }

    pub fn enable_step_mode(&self) {
        let (lock, _) = &*self.step_mode;
        let mut step = lock.lock().unwrap();
        *step = true;
    }

    pub fn disable_step_mode(&self) {
        let (lock, _) = &*self.step_mode;
        let mut step = lock.lock().unwrap();
        *step = false;
    }

    pub fn run(
        self: Arc<Self>,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        // get initial IPs
        let initial_ips = self.get_initial_ips()?;

        //  add init IPs into ips
        {
            let mut ips_locked = self
                .ips
                .lock()
                .map_err(|_| InterpreterError::ThreadError("Failed to lock ips".to_string()))?;
            ips_locked.extend(initial_ips.clone());
        }

        //  run ips
        for ip in initial_ips {
            self.add_ip(ip.clone(), io_handler.clone())?;
        }

        // wait for threads
        let threads = {
            let mut threads_locked = self
                .threads
                .lock()
                .map_err(|_| InterpreterError::ThreadError("Failed to lock threads".to_string()))?;
            threads_locked.drain(..).collect::<Vec<_>>()
        };

        for t in threads {
            t.join()
                .map_err(|_| InterpreterError::ThreadError("Failed to join thread".to_string()))?;
        }

        Ok(())
    }

    pub fn get_initial_ips(&self) -> Result<Vec<Arc<Mutex<IPState>>>, InterpreterError> {
        let mut initial_ips = Vec::new();
        {
            let mut grid = self.program.grid.lock()?;
            for y in 0..self.program.code_height {
                for x in 0..self.program.code_width {
                    let c = grid[y][x];
                    if c == '→' || c == '↓' || c == '↑' || c == '←' {
                        let direction = match c {
                            '→' => Direction::Right,
                            '←' => Direction::Left,
                            '↑' => Direction::Up,
                            '↓' => Direction::Down,
                            _ => unreachable!(),
                        };
                        let ip = IPState::new(x, y, direction);
                        initial_ips.push(Arc::new(Mutex::new(ip)));
                        grid[y][x] = ' ';
                    }
                }
            }
        }

        if initial_ips.is_empty() {
            let ip = IPState::new(0, 0, Direction::Right);
            initial_ips.push(Arc::new(Mutex::new(ip)));
        }

        Ok(initial_ips)
    }

    pub fn dump_stack(
        &self,
        ip: Arc<Mutex<IPState>>,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let stack = {
            let ip_locked = ip
                .lock()
                .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
            ip_locked.stk.clone()
        };
        io_handler.display_stack(&stack)
    }

    pub fn dump_grid(
        &self,
        ip: Arc<Mutex<IPState>>,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        let grid = self
            .program
            .grid
            .lock()
            .map_err(|_| InterpreterError::ThreadError("Failed to lock grid".to_string()))?;
        let (ip_x, ip_y) = {
            let ip_locked = ip
                .lock()
                .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
            (ip_locked.ip_x, ip_locked.ip_y)
        };
        io_handler.display_grid(&grid, ip_x, ip_y)
    }
}

impl CommandGrid for Interpreter {
    fn pop(&self, ip: Arc<Mutex<IPState>>) -> Result<usize, InterpreterError> {
        let mut ip_locked = ip.lock()?;
        Ok(ip_locked.stk.pop().unwrap_or(0))
    }

    /// add new ip & run
    fn add_ip(
        &self,
        new_ip: Arc<Mutex<IPState>>,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        // add to ips
        {
            let mut ips_locked = self
                .ips
                .lock()
                .map_err(|_| InterpreterError::ThreadError("Failed to lock ips".to_string()))?;
            ips_locked.push(new_ip.clone());
        }

        // prepare Interpreter, IOHandle
        let interpreter_clone = Arc::clone(&Arc::new(self.clone()));
        let io_handler_clone = io_handler.clone();
        // prepare thread handle
        let threads_clone = Arc::clone(&self.threads);

        // spawn new thread
        let handle = thread::spawn(move || {
            if let Err(e) = interpreter_clone.run_ip(new_ip, io_handler_clone) {
                eprintln!("Thread Error: {}", e);
            }
        });

        // handle threads
        {
            let mut threads_locked = threads_clone
                .lock()
                .map_err(|_| InterpreterError::ThreadError("Failed to lock threads".to_string()))?;
            threads_locked.push(handle);
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    /// thread
    fn run_ip(
        self: Arc<Self>,
        ip: Arc<Mutex<IPState>>,
        io_handler: Arc<dyn IOHandle + Send + Sync>,
    ) -> Result<(), InterpreterError> {
        loop {
            // is terminated
            if ip
                .lock()
                .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?
                .terminated
            {
                break;
            }

            // current (x, y)
            let (x, y) = {
                let ip_locked = ip.lock().map_err(|_| {
                    InterpreterError::ThreadError("Failed to lock IPState".to_string())
                })?;
                (ip_locked.ip_x, ip_locked.ip_y)
            };

            // get cmd (x, y)
            let cmd = {
                let grid_locked = self.program.grid.lock().map_err(|_| {
                    InterpreterError::ThreadError("Failed to lock grid".to_string())
                })?;
                if y >= grid_locked.len() || x >= grid_locked[0].len() {
                    io_handler.write_error(&format!("IP out of bounds: x={}, y={}", x, y))?;
                    // IP を終了
                    {
                        let mut ip_locked = ip.lock().map_err(|_| {
                            InterpreterError::ThreadError("Failed to lock IPState".to_string())
                        })?;
                        ip_locked.terminated = true;
                    }
                    break;
                }
                grid_locked[y][x]
            };

            // string mode flag
            let is_string_mode = {
                let ip_locked = ip.lock().map_err(|_| {
                    InterpreterError::ThreadError("Failed to lock IPState".to_string())
                })?;
                ip_locked.string_mode_active
            };

            if is_string_mode {
                if cmd == '"' {
                    //  toggle mode
                    let command = self.command_registry.get_command(cmd).unwrap();
                    command.execute(ip.clone(), self.as_ref(), io_handler.clone())?;
                } else {
                    // push to stack
                    let ascii = cmd as usize;
                    {
                        let mut ip_locked = ip.lock().map_err(|_| {
                            InterpreterError::ThreadError("Failed to lock IPState".to_string())
                        })?;
                        ip_locked.stk.push(ascii);
                    }
                }
            } else {
                // execute command
                let command = match self.command_registry.get_command(cmd) {
                    Some(cmd) => cmd,
                    None => {
                        // ignore unknown command
                        self.move_ip(ip.clone())?;
                        continue;
                    }
                };
                // execute
                command.execute(ip.clone(), self.as_ref(), io_handler.clone())?;
            }

            // IP を移動
            self.move_ip(ip.clone())?;
            if self.debug_mode {
                self.dump_grid(ip.clone(), io_handler.clone())?;
                self.dump_stack(ip.clone(), io_handler.clone())?;
                io::stdout().flush().unwrap();
            }

            if self.is_step_mode_active() {
                {
                    println!("Press Enter to execute the next IP, or type 'q' to quit:");
                    print!("$ > ");
                    io::stdout().flush().unwrap();
                    self.wait_for_step()?;
                    self.enable_step_mode();
                }
            }
        }

        Ok(())
    }

    fn move_ip(&self, ip: Arc<Mutex<IPState>>) -> Result<(), InterpreterError> {
        if self.debug_mode {
            // ロックを取得して現在の位置を取得し、ロックを解除
            let (_current_x, _current_y) = {
                let ip_locked = ip.lock().map_err(|_| {
                    InterpreterError::ThreadError("Failed to lock IPState".to_string())
                })?;
                (ip_locked.ip_x, ip_locked.ip_y)
            };

            // 再度ロックを取得して IP を移動
            let mut ip_locked = ip
                .lock()
                .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
            println!("Executed At ({}, {})", ip_locked.ip_x, ip_locked.ip_y);

            let new_x = (ip_locked.ip_x as isize + ip_locked.direction.dx())
                .rem_euclid(Self::WIDTH as isize);
            let new_y = (ip_locked.ip_y as isize + ip_locked.direction.dy())
                .rem_euclid(Self::HEIGHT as isize);
            ip_locked.ip_x = new_x as usize;
            ip_locked.ip_y = new_y as usize;
        } else {
            let mut ip_locked = ip
                .lock()
                .map_err(|_| InterpreterError::ThreadError("Failed to lock IPState".to_string()))?;
            let new_x = (ip_locked.ip_x as isize + ip_locked.direction.dx())
                .rem_euclid(Self::WIDTH as isize);
            let new_y = (ip_locked.ip_y as isize + ip_locked.direction.dy())
                .rem_euclid(Self::HEIGHT as isize);
            ip_locked.ip_x = new_x as usize;
            ip_locked.ip_y = new_y as usize;
        }
        Ok(())
    }

    fn set_value(&self, x: usize, y: usize, value: char) -> Result<(), InterpreterError> {
        let mut grid = self
            .program
            .grid
            .lock()
            .map_err(|_| InterpreterError::ThreadError("Failed to lock grid".to_string()))?;
        let sized_x: usize = (x as usize).try_into().unwrap();
        let sized_y: usize = (y as usize).try_into().unwrap();
        if sized_y < grid.len() && sized_x < grid[sized_y].len() {
            grid[sized_y][sized_x] = value;
            Ok(())
        } else {
            Err(InterpreterError::ThreadError(format!(
                "Attempt to set value out of bounds at ({}, {})",
                x, y
            )))
        }
    }

    fn get_value(&self, x: usize, y: usize) -> Result<char, InterpreterError> {
        let grid = self
            .program
            .grid
            .lock()
            .map_err(|_| InterpreterError::ThreadError("Failed to lock grid".to_string()))?;

        if y < grid.len() && x < grid[y].len() {
            Ok(grid[y][x])
        } else {
            Err(InterpreterError::ThreadError(format!(
                "Attempt to get value out of bounds at ({}, {})",
                x, y
            )))
        }
    }
}
