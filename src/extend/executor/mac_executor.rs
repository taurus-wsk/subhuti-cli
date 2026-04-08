use crate::core::agent::{AgentTaskStep, Executor};
use crate::core::context::ContextHub;
use anyhow::Result;

pub struct MacExecutor;
impl Executor for MacExecutor {
    fn execute(&self, step: &AgentTaskStep, _ctx: &ContextHub) -> Result<String> {
        // std::fs::File::create(path)?;
        match step.action.as_str() {
            // 打开软件
            "open_app" => {
                println!("🚀 打开应用：{}", step.value);
                std::process::Command::new("open")
                    .arg("-a")
                    .arg(&step.value)
                    .spawn()?;
            }

            // 打开网址
            "open_url" => {
                println!("🌍 打开网址：{}", step.value);
                std::process::Command::new("open")
                    .arg(&step.value)
                    .spawn()?;
            }

            // B站搜索
            "search" => {
                let url = format!(
                    "https://search.bilibili.com/all?keyword={}",
                    step.value
                );
                println!("🔍 B站搜索：{}", step.value);
                std::process::Command::new("open")
                    .arg(url)
                    .spawn()?;
            }

            _ => anyhow::bail!("不支持的动作"),
        }

        Ok(format!("执行：{}", step.desc))
    }
}