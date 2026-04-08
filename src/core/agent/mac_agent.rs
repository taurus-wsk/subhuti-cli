use crate::core::agent::agent::{Agent, AgentTaskStep};
use anyhow::Result;
use crate::core::context::ContextHub;

pub struct MacAgent;

impl Agent for MacAgent {
    /// AI 思考：任务 → 生成步骤
    fn think(&self, ctx :&ContextHub, task: &str) -> Result<Vec<AgentTaskStep>> {
        // ==============================
        // 🔥 终极提示词（你要的：AI生成执行步骤）
        // ==============================
        let prompt = format!(
            r#"
你是一个电脑控制专家。
根据用户的任务，生成**可直接执行**的电脑操作步骤。
必须严格返回 JSON 数组，不要其他任何文字。

支持的动作：
- open_app: 打开软件（value填软件名）
- open_url: 打开网址（value填完整URL）
- search: 搜索关键词（B站搜索）

用户任务：{}

输出格式示例：
[
  {{"action":"open_app","value":"Microsoft Edge","desc":"打开浏览器"}},
  {{"action":"open_url","value":"https://www.bilibili.com/","desc":"打开B站"}},
  {{"action":"search","value":"间谍过家家","desc":"搜索关键词"}}
]
"#,
            task
        );

        // ==============================
        // 1. 调用你的AI接口
        // ==============================
        let ai_result = ctx.get_vlm().ask_sync(&prompt)?;

        // ==============================
        // 2. 解析AI返回的步骤
        // ==============================
        let steps: Vec<AgentTaskStep> = serde_json::from_str(&ai_result)?;

        Ok(steps)
    }

    /// AI 修复
    fn fix(&self, ctx:&ContextHub,task: &AgentTaskStep, err: &anyhow::Error) -> Result<String> {
        Ok(format!("错误{}",err))
    }
}