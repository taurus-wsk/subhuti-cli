use std::thread::sleep;
use std::time::Duration;
use crate::core::context::ContextHub;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// AI 智能体核心（所有英雄/大脑都实现这个）
pub trait Agent: Send + Sync {
    /// 思考：输入任务 → 输出【步骤执行列表】
    fn think(&self,ctx :&ContextHub, task: &str) -> Result<Vec<AgentTaskStep>>;

    /// 修复：输入任务+错误 → 输出修复命令/方案
    fn fix(&self,ctx :&ContextHub,  task: &AgentTaskStep, err: &anyhow::Error) -> Result<String>;
}

/// AI 思考出来的步骤单元
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTaskStep {
    pub action: String,    // open_app / open_url / search / click / type
    pub parameters: String,     // 参数
    pub desc: String,      // 描述
}

/// AI 装饰器：包裹任意执行器，自动思考 + 执行 + 修复
pub struct AgentDecorator {
    pub name: String,            // 唯一名称："gai-agent", "fix-agent", "hero"
    pub desc: String,            // 描述：让AI知道它是干嘛的
    pub inner: Box<dyn Executor>,   // 原始执行器（command/ai等）
    pub agent: Box<dyn Agent>,      // AI大脑
    max_retry: u8,              // 最大修复次数 = 5
}
pub trait Executor: Send + Sync {
    fn execute(&self, step:&AgentTaskStep, ctx: &ContextHub) -> anyhow::Result<String>;
}
impl AgentDecorator {
    /// 丝滑套壳！默认最多修复5次
    pub fn new(
        name: &str,
        desc: &str,
        inner: Box<dyn Executor>,
        agent: Box<dyn Agent>,
    ) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
            inner,
            agent,
            max_retry: 3,
        }
    }
}

/// 核心：对外还是 Executor，完全透明！

impl AgentDecorator {
    pub fn execute(&self, task: &str, ctx: &ContextHub) -> Result<String> {
        // ==========================================
        // 1. 触发 AI 思考监听
        // ==========================================
        ctx.listeners.iter().for_each(|l| l.on_agent_think(task));
        let step_list = self.agent.think(ctx,task)?;
        let mut retries = 0;
        // ==========================================
        // 2. 执行子步骤
        // ==========================================
        for (_idx, step) in step_list.iter().enumerate() {
            loop {
               
                // 触发子步骤开始
                ctx.listeners.iter().for_each(|l| l.on_agent_step_begin(step));

                match self.inner.execute(&step, ctx) {
                    Ok(output) => {
                        // 子步骤成功
                        ctx.listeners.iter().for_each(|l| l.on_agent_step_success(step, &output));
                        break;
                    }
                    Err(e) => {
                        retries += 1;
                        // 子步骤失败
                        ctx.listeners.iter().for_each(|l| l.on_agent_step_failed(step, &e, retries));

                        if retries > self.max_retry {
                            return Err(e);
                        }

                        // AI 修复
                        let fix_cmd = self.agent.fix(ctx,&step, &e)?;
                        ctx.listeners.iter().for_each(|l| l.on_agent_fix(step, &fix_cmd));

                        // 执行修复
                        let _ = std::process::Command::new("sh")
                            .arg("-c")
                            .arg(fix_cmd)
                            .output();
                    }
                }
            }
        }
        sleep(Duration::from_secs(1));
        Ok("🎉 任务完成".to_string())
    }
}