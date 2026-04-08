use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

// ======================
// 请求结构体（完全不变）
// ======================
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

// ======================
// ✅ 只解析你要的部分
// ======================
#[derive(Debug, Deserialize)]
pub struct DoubaoResponse {
    pub output: Vec<Output>,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub content: Option<Vec<Content>>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub text: String,
}

// ======================
// 客户端
// ======================
pub struct DouBaoRaw {
    client: Client,
    api_key: String,
}

impl DouBaoRaw {
    pub fn new() -> Self {
        load_subhuti_env();
        let api_key = env::var("DOUBAO_API_KEY").expect("拿不到api-key");
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn ask(&self, prompt: impl Into<String>) -> Result<String> {
        let prompt = prompt.into();
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

        let result: DoubaoResponse = resp.json().await?;

        // ✅ 只提取你要的 text
        for out in result.output {
            if let Some(content) = out.content {
                if !content.is_empty() {
                    return Ok(content[0].text.clone());
                }
            }
        }

        anyhow::bail!("没有获取到回答")
    }
    // 🔥 新增：同步版本！给 Executor 用
    pub fn ask_sync(&self, prompt: impl Into<String>) -> Result<String> {
        // 创建一个异步运行时，阻塞执行
        tokio::runtime::Runtime::new()?.block_on(async {
            self.ask(prompt).await
        })
    }
}

fn load_subhuti_env() {
    // 定位到 .subhuti/.env
    let mut path = PathBuf::from(".subhuti");
    path.push(".env");

    // 尝试加载
    match dotenvy::from_path(&path) {
        Ok(_) => println!("✅ 成功加载环境变量：{}", path.display()),
        Err(e) => eprintln!("⚠️ 环境变量加载失败：{}", e),
    }
}