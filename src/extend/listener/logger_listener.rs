use crate::core::context::ContextHub;
use crate::core::schedule::Step;
use crate::extend::PipelineListener;

// 默认日志监听器
pub struct DefaultLogger;

impl PipelineListener for DefaultLogger {
    fn on_start(&self, _ctx: &ContextHub) {
        println!("[日志] 流水线开始");
    }

    fn on_step_begin(&self, step: &Step, _ctx: &ContextHub) {
        println!("[日志] 执行步骤: {}", step.desc);
    }

    fn on_step_end(&self, step: &Step, _ctx: &ContextHub) {
        // let output = ctx.outputs.get(&step.id).unwrap();
        println!("{}", step.desc)
        // println!("[日志] 步骤完成: {} → 输出: {}", step.name, output);
    }

    fn on_success(&self, _ctx: &ContextHub) {
        println!("[日志] 流水线全部成功");
    }

    fn on_fail(&self, _ctx: &ContextHub, err: &anyhow::Error) {
        println!("[日志] 流水线失败: {}", err);
    }
}