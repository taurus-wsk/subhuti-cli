use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

// ========== 你的请求结构 完全不动 ==========
#[derive(Debug, Serialize)]
struct InputRequest {
model: String,
input: Vec<Message>,
temperature: f32,
}

#[derive(Debug, Serialize)]
struct Message {
role: String,
content: String,
}

// ========== 先不解析！直接读原始JSON ==========

pub struct DouBaoRaw {
client: Client,
api_key: String,
}

impl DouBaoRaw {
pub fn new() -> Result<Self> {
dotenv().ok();
let api_key = env::var("DOUBAO_API_KEY")?;
let client = Client::new();
Ok(Self { client, api_key })
}

    pub async fn ask(&self, prompt: impl Into<String>) -> Result<String> {
        let prompt = prompt.into();
        
        // ✅ 你的 URL 完全不动
        let url = "https://ark.cn-beijing.volces.com/api/v3/responses";

        let request = InputRequest {
            model: "doubao-seed-2-0-lite-260215".to_string(),
            input: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.7,
        };

        let resp = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        // ==============================
        // 🔥 直接打印真实返回值！
        // ==============================
        println!("==== 真实API返回 ====");
        println!("{}", text);
        println!("=====================");

        if !status.is_success() {
            anyhow::bail!("API请求失败：{}", text);
        }

        // 先随便返回，让程序不崩溃
        Ok("正在查看真实返回...".to_string())
    }
}