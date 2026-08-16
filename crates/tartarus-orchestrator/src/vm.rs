use std::{
    io,
    process::{Child, Command},
};

pub struct Vm {
    child: Child,
}

impl Vm {
    pub fn spawn() -> io::Result<Self> {
        todo!("Implement VM spawning logic here")
    }
    pub fn shutdown(&mut self) -> io::Result<()> {
        todo!("Implement VM shutdown logic here")
    }
}
