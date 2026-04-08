// // 测试用的外部文件
// use anyhow::Result;
// use crate::extend::{PipelineBuilder, DefaultLogger}; // 你的项目名
// 
// #[test]
// fn test_full_pipeline_run() -> Result<()> {
//     // =======================
//     // 这就是你原来的 main 函数
//     // =======================
//     let json = std::fs::read_to_string("pipeline.json")?;
//     let config = serde_json::from_str(&json)?;
// 
//     let mut pipeline = PipelineBuilder::from_config(config)?;
//     pipeline.add_listener(Box::new(DefaultLogger));
//     pipeline.run()?;
// 
//     println!("🎉 单元测试通过：流水线闭环完成");
//     Ok(())
// }