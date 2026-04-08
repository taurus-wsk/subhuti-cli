use crate::core::agent::{AgentDecorator, DouBaoRaw, Executor};
use crate::core::context::LogLevel::Info;
use crate::extend::{ PipelineListener};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
// src/core/context/hub.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// =============================================================================
// 1. 全局单例 ContextHub
// =============================================================================
// pub static CONTEXT_HUB: Lazy<Arc<Mutex<ContextHub>>> = Lazy::new(|| {
//     // 🔥 这里必须 unwrap 或 expect，不能用 ?
//     let context_hub = ContextHub::new()
//         .expect("全局 ContextHub 初始化失败！请检查 DOUBAO_API_KEY");
//
//     Arc::new(Mutex::new(context_hub))
// });
// =============================================================================
// 2. 核心数据结构定义
// =============================================================================

// 执行器注册中心（全局技能池）
pub struct ExecutorRegistry {
    executors: HashMap<String, Box<dyn Executor>>,
}

impl ExecutorRegistry {
    pub fn new() -> Self {
        Self {
            executors: HashMap::new(),
        }
    }

    // 注册英雄/技能
    pub fn register(&mut self, name: &str, executor: Box<dyn Executor>) {
        self.executors.insert(name.to_string(), executor);
    }

    // 获取执行器
    pub fn get(&self, name: &str) -> Result<&Box<dyn Executor>> {
        self.executors
            .get(name)
            .ok_or_else(|| anyhow!("执行器不存在：{}", name))
    }
}
// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Initializing,
    Thinking,
    Executing,
    Observing,
    Judging,
    Repairing,
    Fallback,
    Success,
    Failed,
}

// 当前任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentTask {
    pub task_id: String,
    pub user_input: String,
    pub status: TaskStatus,
    pub current_step: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub agent_plugin_id: Option<String>, // 当前使用的插件
}

// Skill 执行日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExecutionLog {
    pub log_id: String,
    pub task_id: String,
    pub step_name: String,
    pub skill_name: String,
    pub command: String,
    pub args: Vec<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub return_code: i32,
}

// 系统日志级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

// 系统日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLog {
    pub log_id: String,
    pub level: LogLevel,
    pub message: String,
    pub module: String,
    pub timestamp: DateTime<Utc>,
}
// =============================================================================
// 🔥 你要的：泛型接口（只定义 set + get）
// =============================================================================
pub trait AgentState: Send + Sync {
    fn set(&mut self, key: &str, value: Value);
    fn get(&self, key: &str) -> Option<Value>;
}
// ------------------------------
// 默认状态实现
// ------------------------------
#[derive(Default)]
pub struct DefaultState {
    data: HashMap<String, Value>,
}

impl AgentState for DefaultState {
    fn set(&mut self, key: &str, value: Value) {
        self.data.insert(key.to_string(), value);
    }
    fn get(&self, key: &str) -> Option<Value> {
        self.data.get(key).cloned()
    }
}
// =============================================================================
// 3. 升级后的 ContextHub
// =============================================================================
pub struct ContextHub {
    // 系统日志
    system_logs: Vec<SystemLog>,

    // 全局变量存储
    variables: HashMap<String, String>,

    // 任务历史（最近100个）
    task_history: Vec<CurrentTask>,
    vlm: DouBaoRaw,
    // pub executor_registry: ExecutorRegistry,
    pub listeners: Vec<Box<dyn PipelineListener>>,
    pub agents: HashMap<String, AgentDecorator>,
}

impl ContextHub {
    pub fn new() -> Self {
        let doubao = DouBaoRaw::new();
        Self {
            system_logs: Vec::new(),
            variables: HashMap::new(),
            task_history: Vec::new(),
            vlm: doubao,
            listeners: Vec::new(),
            agents: HashMap::new(),

        }
    }

    /// 🔥 你要的函数：获取所有Agent的名称+描述，给AI看
    pub fn list_all_agents_prompt(&self) -> String {
        let mut lines = vec!["可用智能体列表：".to_string()];

        for (name, agent) in &self.agents {
            lines.push(format!("- 名称：{}，描述：{}", name, agent.desc));
        }

        lines.join("\n")
    }
    // =============================================================================
    // 当前任务管理
    // =============================================================================

    pub fn add_listener(&mut self, listener: Box<dyn PipelineListener>) {
        self.listeners.push(listener);
    }
    pub fn get_vlm(&self) -> &DouBaoRaw {
        &self.vlm
    }

    pub fn get_task_history(&self) -> Vec<CurrentTask> {
        self.task_history.clone()
    }

    // =============================================================================
    // Skill 执行日志管理
    // =============================================================================


    // =============================================================================
    // 系统日志工具
    // =============================================================================
    pub fn log(&mut self, level: LogLevel, module: &str, message: &str) {
        let log = SystemLog {
            log_id: uuid::Uuid::new_v4().to_string(),
            level,
            message: message.to_string(),
            module: module.to_string(),
            timestamp: Utc::now(),
        };

        // 打印到控制台
        println!("[{}][{:?}][{}] {}", log.timestamp.format("%H:%M:%S"), log.level, log.module, log.message);

        // 保存到内存（保留最近1000条）
        self.system_logs.push(log);
        if self.system_logs.len() > 1000 {
            self.system_logs.remove(0);
        }
    }
    pub fn info(&mut self, message: &str) {
        self.log(Info, "", message)
    }
    pub fn get_system_logs(&self, level: Option<LogLevel>, limit: usize) -> Vec<SystemLog> {
        let mut logs: Vec<SystemLog> = self.system_logs
            .iter()
            .filter(|log| level.as_ref().map_or(true, |l| log.level == *l))
            .cloned()
            .collect();

        logs.reverse();
        logs.truncate(limit);
        logs
    }

    // =============================================================================
    // 全局变量数据操作
    // =============================================================================
    pub fn set_var(&mut self, key: &str, value: &str) {
        self.log(LogLevel::Debug, "core", &format!("设置变量: {} = {}", key, value));
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn get_var_or(&self, key: &str, default: &str) -> String {
        self.variables.get(key).cloned().unwrap_or_else(|| default.to_string())
    }

    pub fn delete_var(&mut self, key: &str) {
        self.log(LogLevel::Debug, "core", &format!("删除变量: {}", key));
        self.variables.remove(key);
    }

    pub fn list_vars(&self) -> HashMap<String, String> {
        self.variables.clone()
    }

    pub fn clear_vars(&mut self) {
        self.log(LogLevel::Info, "core", "清空所有变量");
        self.variables.clear();
    }
}

// =============================================================================
// 4. 便捷宏（可选，方便使用）
// =============================================================================
#[macro_export]
macro_rules! hub_log {
    ($level:expr, $module:expr, $($arg:tt)*) => {{
        let mut hub = $crate::core::context::hub::CONTEXT_HUB.lock().unwrap();
        hub.log($level, $module, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! hub_set_var {
    ($key:expr, $value:expr) => {{
        let mut hub = $crate::core::context::hub::CONTEXT_HUB.lock().unwrap();
        hub.set_var($key, $value);
    }};
}

#[macro_export]
macro_rules! hub_get_var {
    ($key:expr) => {{
        let hub = $crate::core::context::hub::CONTEXT_HUB.lock().unwrap();
        hub.get_var($key).cloned()
    }};
}