// use crate::core::context::ContextHub;
// 
// 
// pub struct BaseExecutor;
// impl BaseExecutor {
//     fn execute(&self, cmd: &str, _ctx: &ContextHub) -> anyhow::Result<String> {
//         let output = std::process::Command::new("sh")
//             .arg("-c")
//             .arg(cmd)
//             .output()?;
// 
//         let out = String::from_utf8_lossy(&output.stdout);
//         Ok(out.to_string())
//     }
// }