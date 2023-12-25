pub struct CmdHandler {
    cmd: Cmd,
}

impl CmdHandler {
    pub fn new(cmd: Cmd) -> Self {
        Self { cmd: cmd }
    }
}

#[derive(Debug, Clone)]
pub enum Cmd {
    Get(String),
    Set(String),
}
