use crate::core::context::{ContextHub, UserPipeline};

use crate::core::agent::agent::AgentDecorator;
use crate::core::agent::MacAgent;
use crate::extend::{DefaultLogger, MacExecutor, PipelineListener};
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
// 2. 拦截器 Trait

// ----------------------------------------------------
// 2. 策略模式 Executor
// ----------------------------------------------------

// // 脚本执行器
// pub struct ScriptExecutor {
//     script: String,
// }
//
// impl Executor for ScriptExecutor {
//     fn execute(&self, _ctx: &ContextHub) -> Result<String> {
//         let output = Command::new("sh")
//             .arg("-c")
//             .arg(&self.script)
//             .output()?;
//         Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
//     }
// }

// ----------------------------------------------------
// 3. 命令模式 Step
// ----------------------------------------------------

pub struct Step {
    pub desc: String,
}

impl Step {
    pub fn run(&self, ctx: &mut ContextHub) -> Result<String> {
        // ==============================
        // 1. 组合提示词（你要的组合好的提示）
        // ==============================
        let agents_prompt = ctx.list_all_agents_prompt();
        let task = &self.desc;

        let prompt = format!(
            r#"
你是任务分配专家。
请根据用户任务，从下面列表中**选择最合适的一个智能体（Agent）**。
只输出智能体名称，不要输出任何多余内容。

{agents_prompt}

用户任务：{task}
请输出最合适的Agent名称：
"#
        );

        // ==============================
        // 2. AI 选择 Agent 名称
        // ==============================
        let agent_name = ctx.get_vlm().ask_sync(&prompt)?;
        let agent_name = agent_name.trim(); // 清理空白字符

        println!("🤖 AI 选择的智能体：{}", agent_name);

        // ==============================
        // 3. 获取选中的 Agent
        // ==============================
        let agent = ctx
            .agents
            .get(agent_name)
            .ok_or_else(|| anyhow::anyhow!("未找到Agent：{}", agent_name))?;

        // ==============================
        // 4. 执行（用 Agent 的执行器）
        // ==============================
        agent.execute(task, ctx)
    }
}

// ----------------------------------------------------
// 5. 责任链模式 Pipeline
// ----------------------------------------------------

pub struct Pipeline {
    steps: Vec<Step>,
    context: ContextHub,
    listeners: Vec<Box<dyn PipelineListener>>,
}

impl Pipeline {
    pub fn new(steps: Vec<Step>, context: ContextHub) -> Self {
        Self {
            steps,
            context,
            listeners: Vec::new(),
        }
    }
    pub fn add_agent(&mut self, agent: AgentDecorator) {
        self.context.agents.insert(agent.name.clone(), agent);
    }
    pub fn add_listener(&mut self, listener: Box<dyn PipelineListener>) {
        self.listeners.push(listener);
    }
    // 添加拦截器（插拔式）

    pub fn default(&mut self) {
        self.add_listener(Box::new(DefaultLogger));
        let mac_agent = AgentDecorator::new(
            "mac",
            "mac电脑助手，可以操作文件，使用bash命令，日常打开浏览器，使用 点击滑动输入等操作帮助主人",
            Box::new(MacExecutor {}), // 👈 必须包 Box
            Box::new(MacAgent {}),
        );
        self.add_agent(mac_agent)
        // self.context.executor_registry.register(
        //     "gai",
        //     Box::new(GAIExecutor {
        //         prompt: String::new(),
        //     }),
        // );
    }
    // 闭环执行
    pub fn run(&mut self) -> Result<()> {
        for l in &self.listeners {
            l.on_start(&self.context);
        }

        let step_count = self.steps.len();
        for idx in 0..step_count {
            // let step = self.steps[idx];
            for l in &self.listeners {
                l.on_step_begin(&self.steps[idx], &self.context);
            }

            // 🔥 执行 + 拦截器自愈（核心优雅点）
            let run_result = self.steps[idx].run(&mut self.context);

            match run_result {
                Ok(output) => {
                    for l in &self.listeners {
                        l.on_step_end(&self.steps[idx], &self.context);
                    }
                    println!("Step {} output", output);
                }
                Err(e) => {
                    for l in &self.listeners {
                        l.on_fail(&self.context, &e);
                    }
                    return Err(e);
                }
            }
        }

        // self.context.success = true;
        for l in &self.listeners {
            l.on_success(&self.context);
        }

        Ok(())
    }
    // 接收索引 idx
}

// ----------------------------------------------------
// 6. 建造者模式 Builder（从JSON构建）
// ----------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct PipelineConfig {
    pub name: String,
    pub steps: Vec<StepConfig>,
}

#[derive(Debug, Deserialize)]
pub struct StepConfig {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub command: Option<String>,
    pub script: Option<String>,
}

pub struct PipelineBuilder;

impl PipelineBuilder {
    /// 从文件加载（旧接口，兼容）
    pub fn from_config(json_path: &str) -> Result<Pipeline> {
        let mut path = PathBuf::from(".subhuti");
        path.push(json_path);
        let json = fs::read_to_string(&path)?;
        let user_config: UserPipeline = serde_json::from_str(&json)?;

        // AI 转换成专业版
        // let ai_pipeline = ai_optimize_pipeline(&ctx, &user_config)?;
        Self::from_ai_pipeline(user_config)
    }

    /// 🔥 新接口：一句话任务直接执行
    /// 例：PipelineBuilder::from_task(ctx, "帮我构建项目并完成")
    pub fn from_task(task: &str) -> Result<Pipeline> {
        // let ctx = ContextHub::new();
        let mut up = UserPipeline {
            name: "临时任务".to_string(),
            steps: Vec::new(),
        };
        up.steps.push(task.to_string());

        // let ai_pipeline = ai_optimize_pipeline(&ctx, &user_config)?;
        Self::from_ai_pipeline(up)
    }

    /// 内部共用：AI专业结构 → 可执行Pipeline
    fn from_ai_pipeline(up: UserPipeline) -> Result<Pipeline> {
        let mut steps = vec![];
        let ctx = ContextHub::new();
        for s in up.steps {
            steps.push(Step { desc: s });
        }
        Ok(Pipeline::new(steps, ctx))
    }
}
