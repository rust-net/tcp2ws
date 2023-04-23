#![windows_subsystem = "console"]

#[path ="tcp2ws/mod.rs"]
mod proc;

fn main() {
    proc::main()
}