use std::thread::sleep;
use std::time::Duration;
use subhuti_cli::core::schedule::PipelineBuilder;

fn run_app() -> anyhow::Result<()> {
    // let mut pipeline = PipelineBuilder::from_config("pipeline.json")?;

    let mut pipeline = PipelineBuilder::from_task("打开edge浏览器，到b站搜索，攻壳机动队")?;
    pipeline.default();
    pipeline.run()?;
    println!("🎉 单元测试通过：流水线闭环完成");
    sleep(Duration::from_secs(5));
    Ok(())
}

fn main() {
    // 🔥 所有错误全部在这里统一处理
    if let Err(e) = run_app() {
        println!("程序异常退出：{}", e);
        // 你可以在这里加日志、上报、退出码...
        std::process::exit(1);
    }

    // let ai = DouBaoRaw::new()?;
    // // let reply = ai.ask("你好").await?;
    // println!("🤖 豆包：{}", reply);

    // 1. 初始化 CLI
    // let cli = SubhutiCli::parse();
    // let renderer = TerminalRenderer::new();

    // 2. 打印欢迎横幅
    // renderer.print_banner();

    // 3. 初始化插件管理器
    // let plugin_mgr = PluginManager::new();
    // plugin_mgr.load_plugins("../cells/")?;

    // 4. 处理命令
    // 【关键】先访问 cli.command，然后匹配 Commands 枚举
    // match &cli.command {
    //     // ==========================================
    //     // 模式1：流水线模式
    //     // ==========================================
    //     Commands::Pipeline { template, var } => { // 👈 改成 Commands::Pipeline
    //         println!("{}", "🚀 启动流水线模式...".bold());
    //         println!("  模板: {:?}", template);
    //         println!("  变量: {:?}", var);
    //
    //         let main_pb = renderer.create_main_progress(5, "执行流水线...");
    //         let steps = vec![
    //             ("初始化环境", StepStatus::Completed),
    //             ("拉取代码", StepStatus::Completed),
    //             ("构建项目", StepStatus::Running),
    //             ("运行测试", StepStatus::Pending),
    //             ("部署", StepStatus::Pending),
    //         ];
    //
    //         for (i, (name, status)) in steps.iter().enumerate() {
    //             main_pb.inc(1);
    //             renderer.print_step(i + 1, steps.len(), name, *status);
    //             std::thread::sleep(std::time::Duration::from_millis(500));
    //         }
    //
    //         main_pb.finish_with_message("流水线执行完成");
    //         renderer.clear();
    //         renderer.print_success("流水线执行成功！");
    //     }
    //
    //     // ==========================================
    //     // 模式2：任务模式
    //     // ==========================================
    //     Commands::Task { description, interactive } => { // 👈 改成 Commands::Task
    //         println!("{}", "💡 启动任务模式...".bold());
    //         println!("  任务: {}", description);
    //         if *interactive {
    //             println!("  模式: 交互式");
    //         }
    //
    //
    //
    //         let step_pb = renderer.create_step_progress("AI 正在分解任务...");
    //         std::thread::sleep(std::time::Duration::from_secs(2));
    //         step_pb.finish_with_message("任务分解完成");
    //         renderer.print_success("任务执行成功！");
    //     }
    //
    //     // ==========================================
    //     // 插件管理
    //     // ==========================================
    //     Commands::Plugin { action } => { // 👈 改成 Commands::Plugin
    //         match action {
    //             PluginCommands::List => { // 👈 这里用 PluginCommands::List
    //                 println!("{}", "📦 已加载插件:".bold());
    //             }
    //             PluginCommands::Load { path } => {
    //                 println!("加载插件: {:?}", path);
    //             }
    //             PluginCommands::Unload { name } => {
    //                 println!("卸载插件: {}", name);
    //             }
    //         }
    //     }
    //
    //     // ==========================================
    //     // 列出技能
    //     // ==========================================
    //     // Commands::ListSkills => { // 👈 改成 Commands::ListSkills
    //     //     let hub = CONTEXT_HUB.lock().unwrap();
    //     //     let skills: Vec<_> = hub.list_skills()
    //     //         .iter()
    //     //         .map(|s| (s.name.clone(), s.description.clone(), s.plugin_name.clone()))
    //     //         .collect();
    //     //     renderer.print_skills(skills);
    //     // }
    //
    //     // ==========================================
    //     // 查看历史
    //     // ==========================================
    //     Commands::History { limit } => { // 👈 改成 Commands::History
    //         println!("查看最近 {} 条执行历史...", limit);
    //     }
    // }
}
