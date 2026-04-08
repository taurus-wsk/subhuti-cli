use crate::core::context::ContextHub;
use anyhow::Result;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 保存 AI 优化后的流水线到 report/ 目录，带时间戳
pub fn save_ai_pipeline(pipeline: &AiExecutedPipeline) -> Result<PathBuf> {
    // 1. 项目根目录 = 当前执行目录（cargo run 永远是这里）
    let mut report_path = PathBuf::from("report");

    // 2. 自动创建 report 文件夹（不存在就创建）
    if !report_path.exists() {
        fs::create_dir_all(&report_path)?;
    }

    // 3. 生成时间戳  20260407_182235
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();

    // 4. 文件名
    let filename = format!("ai_executed_pipeline_{}.json", timestamp);
    report_path.push(filename);

    // 5. 序列化并写入文件（美化格式）
    let json = serde_json::to_string_pretty(pipeline)?;
    fs::write(&report_path, json)?;

    println!("📄 执行报告已保存：{}", report_path.display());

    // 返回路径，方便后续使用
    Ok(report_path)
}
// 用户简易结构
#[derive(Debug, Deserialize, Serialize)]
pub struct UserPipeline {
    pub name: String,
    pub steps: Vec<String>,
}

 

// ------------------------------
// AI 输出 → 专业执行结构
// ------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct AiExecutedPipeline {
    pub name: String,
    pub context: String,
    pub steps: Vec<AiExecutedStep>,

}

// AI 优化后的步骤
#[derive(Debug, Deserialize, Serialize)]
pub struct AiExecutedStep {
    pub id: String,
    pub desc: String,
    pub executor: String,
    pub retry: u8,
    pub on_fail: Option<String>,
    pub on_success: Option<String>,
}

// 执行报告（断点续跑、日志、修复）
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PipelineReport {
    pub status: String,
    pub current_step: Option<String>,
    pub success_steps: Vec<String>,
    pub failed_steps: Vec<String>,
    pub logs: Vec<String>,
}
// ------------------------------------
// 🔥 核心：AI 转换函数（共用）
// ------------------------------------
pub fn ai_optimize_pipeline(
    ctx: &ContextHub,
    user: &UserPipeline,
) -> Result<AiExecutedPipeline> {
    let user_json = serde_json::to_string_pretty(user)?;

    let prompt = format!(
        r#"
你是自动化流水线专家。
请把用户输入转换成【可执行的专业流水线JSON】。
只输出JSON，不要任何解释。

结构：
{{
  "name": "流程名",
  "context": "用户输入目标",
  "steps": [{{
    "id": "step1",
    "desc": "说明",
    "type": "ai",
    "retry": 3,
  }}],
}}

用户输入：
{user_json}
"#
    );

    let ai_response = ctx.get_vlm().ask_sync(&prompt)?;
    let pipeline: AiExecutedPipeline = serde_json::from_str(&ai_response)?;

    // 自动保存报告
    save_ai_pipeline(&pipeline)?;

    Ok(pipeline)
}

// ------------------------------------
// 📄 保存报告（带时间戳）
// ------------------------------------
// pub fn save_ai_pipeline(pipeline: &AiExecutedPipeline) -> Result<PathBuf> {
//     let mut root = PathBuf::from("report");
//     if !root.exists() {
//         fs::create_dir_all(&root)?;
//     }
//
//     let time = Local::now().format("%Y%m%d_%H%M%S");
//     let filename = format!("ai_pipeline_{}.json", time);
//     root.push(filename);
//
//     let json = serde_json::to_string_pretty(pipeline)?;
//     fs::write(&root, json)?;
//
//     println!("📄 报告已生成：{}", root.display());
//     Ok(root)
// }
