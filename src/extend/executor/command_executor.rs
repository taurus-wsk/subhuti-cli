// use crate::core::context::{ContextHub};
// use anyhow::Result;
// use crate::extend::Executor;
// 
// pub struct CommandExecutor {
//     command: String,
// }
// 
// impl Executor for CommandExecutor {
//     fn execute(&self, cmd: &str, _ctx: &ContextHub) -> Result<String> {
//         let output = std::process::Command::new("sh")
//             .arg("-c")
//             .arg(cmd)
//             .output()?;
// 
//         let out = String::from_utf8_lossy(&output.stdout);
//         Ok(out.to_string())
//     }
// }