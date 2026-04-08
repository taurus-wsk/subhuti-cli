// use crate::core::context::ContextHub;
// use anyhow::Result;
// // AI 执行的 4 个标准状态
// pub enum AIState {
//     Think,  // 思考
//     Act,    // 执行
//     Observe, // 观察
//     Judge,  // 判断
// }
// pub struct AIMemory {
//     pub prompt: String,
//     pub thought: Option<String>,
//     pub action_result: Option<String>,
//     pub observation: Option<String>,
//     pub judgment: Option<String>,
//     pub retry_count: u8,
//     pub max_retry: u8,
// }
// 
// impl AIMemory {
//     pub fn new(prompt: String, max_retry: u8) -> Self {
//         Self {
//             prompt,
//             thought: None,
//             action_result: None,
//             observation: None,
//             judgment: None,
//             retry_count: 0,
//             max_retry,
//         }
//     }
// }
// pub struct AIExecutor {
//     prompt: String,
//     max_retry: u8,
// }
// 
// impl AIExecutor {
//     pub fn new(prompt: String, max_retry: u8) -> Self {
//         Self { prompt, max_retry }
//     }
// 
//     // 🔥 主逻辑：ReAct 工作流（真正的AI智能体）
//     fn react(&self, ctx: &ContextHub) -> Result<String> {
//         let mut memory = AIMemory::new(self.prompt.clone(), self.max_retry);
//         let mut state = AIState::Think;
// 
//         loop {
//             state = match state {
//                 AIState::Think => self.think(ctx, &mut memory)?,
//                 AIState::Act => self.act(ctx, &mut memory)?,
//                 AIState::Observe => self.observe(ctx, &mut memory)?,
//                 AIState::Judge => self.judge(ctx, &mut memory)?,
//             };
// 
//             // 如果判断完成，退出
//             if matches!(state, AIState::Judge) {
//                 break;
//             }
//         }
// 
//         Ok(memory.judgment.unwrap_or_default())
//     }
// 
//     // 1. 思考
//     fn think(&self, ctx: &ContextHub, memory: &mut AIMemory) -> Result<AIState> {
//         let prompt = format!(
//             "任务：{}\n请思考：你要做什么？目的是什么？",
//             memory.prompt
//         );
//         let thought = ctx.get_vlm().ask_sync(&prompt)?;
//         memory.thought = Some(thought);
//         Ok(AIState::Act)
//     }
// 
//     // 2. 执行（真正干活）
//     fn act(&self, ctx: &ContextHub, memory: &mut AIMemory) -> Result<AIState> {
//         let thought = memory.thought.as_ref().unwrap();
//         let prompt = format!(
//             "根据思考结果执行任务：\n{}\n只输出执行结果，不要多余内容",
//             thought
//         );
//         let result = ctx.get_vlm().ask_sync(&prompt)?;
//         memory.action_result = Some(result);
//         Ok(AIState::Observe)
//     }
// 
//     // 3. 观察结果
//     fn observe(&self, ctx: &ContextHub, memory: &mut AIMemory) -> Result<AIState> {
//         let result = memory.action_result.as_ref().unwrap();
//         let prompt = format!(
//             "观察执行结果：\n{}\n请总结观察到的内容",
//             result
//         );
//         let observation = ctx.get_vlm().ask_sync(&prompt)?;
//         memory.observation = Some(observation);
//         Ok(AIState::Judge)
//     }
// 
//     // 4. 判断：成功？失败？重试？
//     fn judge(&self, ctx: &ContextHub, memory: &mut AIMemory) -> Result<AIState> {
//         let observation = memory.observation.as_ref().unwrap();
//         let prompt = format!(
//             "观察结果：{}\n请判断任务是否成功。成功输出 SUCCESS，失败输出 FAILED",
//             observation
//         );
//         let judgment = ctx.get_vlm().ask_sync(&prompt)?;
// 
//         memory.judgment = Some(judgment.clone());
// 
//         if judgment.contains("FAILED") && memory.retry_count < memory.max_retry {
//             memory.retry_count += 1;
//             return Ok(AIState::Think);
//         }
// 
//         Ok(AIState::Judge)
//     }
// }