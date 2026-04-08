use crate::core::agent::agent::AgentTaskStep;
use crate::core::context::ContextHub;
use crate::extend::Step;

pub trait PipelineListener: Send + Sync {
    fn on_start(&self, ctx: &ContextHub);
    fn on_step_begin(&self, step: &Step, ctx: &ContextHub);
    fn on_step_end(&self, step: &Step, ctx: &ContextHub);
    fn on_success(&self, ctx: &ContextHub);
    fn on_fail(&self, ctx: &ContextHub, err: &anyhow::Error);

    /// AI 开始思考任务
    fn on_agent_think(&self, task: &str) {
        println!("{}", task)
    }
    /// AI 子步骤开始
    fn on_agent_step_begin(&self, step: &AgentTaskStep) {
        println!("{}", step.desc)
    }
    /// AI 子步骤成功
    fn on_agent_step_success(&self, step: &AgentTaskStep, output: &str) {
        println!("{},out:{}", step.desc,output)
    }
    /// AI 子步骤失败
    fn on_agent_step_failed(&self, step: &AgentTaskStep, err: &anyhow::Error, retry: u8) {
        println!("{},err:{},retry:{}", step.desc,err,retry)
    }
    /// AI 开始修复
    fn on_agent_fix(&self, step: &AgentTaskStep, fix_cmd: &str) {
        println!("step:{},fix:{}",step.desc, fix_cmd)
    }
}
