use crate::core::agent::agent::{Agent, AgentTaskStep};
use crate::core::context::ContextHub;
use anyhow::Result;

pub struct MacAgent;
pub fn get_skill_prompt() -> String {
    r#"
可用技能：
- open_app: 打开 Mac 应用，如 Microsoft Edge、微信、访达
- open_url: 打开网页 URL
- close_app: 关闭指定应用
- hide_app: 隐藏当前活动窗口
- sleep: 等待指定秒数
- type_text: 输入文字
- click: 鼠标点击指定坐标，格式：x,y
- scroll_up: 向上滚动
- scroll_down: 向下滚动
- screenshot: 截图并保存，默认保存到桌面
- open_desktop: 打开桌面文件夹
- open_downloads: 打开下载文件夹
- open_folder: 打开指定文件夹路径
- volume_up: 音量增加 10%
- volume_down: 音量减少 10%
- mute: 静音
- wifi_on: 打开 Wi-Fi
- wifi_off: 关闭 Wi-Fi
- lock: 锁定屏幕
- restart: 重启电脑
"#.to_string()
}
impl Agent for MacAgent {
    /// AI 思考：任务 → 生成步骤
    fn think(&self, ctx: &ContextHub, task: &str) -> Result<Vec<AgentTaskStep>> {
        // ==============================
        // 🔥 终极提示词（你要的：AI生成执行步骤）
        // ==============================
        let skill_prompt = get_skill_prompt();

        let prompt = format!(
            r#"
你是电脑控制专家。
根据用户任务，选择合适的技能并生成执行步骤。
只返回 JSON 数组，不要其他任何文字。

{}

输出格式：
[
  {{"action":"skill_name","parameters":"value","desc":"说明"}}
]

用户任务：{}
"#,
            skill_prompt,
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
    fn fix(&self, ctx: &ContextHub, task: &AgentTaskStep, err: &anyhow::Error) -> Result<String> {
        println!("{}", ctx.list_all_agents_prompt());
        Ok(format!("错误{},desc{}", err, task.desc))
    }
}