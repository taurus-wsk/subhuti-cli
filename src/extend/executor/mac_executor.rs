use crate::core::agent::{AgentTaskStep, Executor};
use crate::core::context::ContextHub;
use anyhow::Result;
use std::process::Command;

pub struct MacExecutor;
impl Executor for MacExecutor {
    fn execute(&self, step: &AgentTaskStep, _ctx: &ContextHub) -> Result<String> {
        // let params = json!({ "value": step.parameters });
        run_skill(&step.action, &step.parameters.to_string())
    }
}
pub fn run_skill(skill: &str, params: &str) -> anyhow::Result<String> {
    // let params_str = serde_json::to_string(&params)?;

    let output = Command::new("python3")
        .arg("/.subhuti/mac-skills/run_skill.py") // 改成你自己的路径
        .arg(skill)
        .arg(params)
        .output()?;

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(result)
}