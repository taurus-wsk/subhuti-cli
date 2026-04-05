subhuti-cli/
├── subhuti/ # 【核心框架】100%通用抽象，无业务逻辑
│ ├── __init__.py
│ │
│ ├── core/ # 【新增】核心调度与执行闭环层（原核心模块重新组织）
│ │ ├── __init__.py
│ │ ├── pipeline/ # 流水线引擎（原pipeline/目录）
│ │ │ ├── __init__.py
│ │ │ ├── pipeline.py # Pipeline 定义与执行
│ │ │ ├── stage.py # Stage 抽象
│ │ │ ├── step.py # Step 抽象
│ │ │ └── context.py # Pipeline Context
│ │ │
│ │ ├── task/ # 任务分解系统（原task/目录）
│ │ │ ├── __init__.py
│ │ │ ├── decomposer.py # Task Decomposer (任务分解器)
│ │ │ ├── task.py # Task / SubTask 数据模型
│ │ │ ├── tracker.py # Progress Tracker (进度跟踪器)
│ │ │ └── executor.py # Task Executor (任务执行器)
│ │ │
│ │ ├── agent/ # AI 代理层（原agent/目录）
│ │ │ ├── __init__.py
│ │ │ ├── state.py # Agent State
│ │ │ ├── graph.py # LangGraph 定义
│ │ │ ├── nodes/ # 节点实现
│ │ │ │ ├── __init__.py
│ │ │ │ ├── plan.py
│ │ │ │ ├── execute.py
│ │ │ │ ├── observation.py
│ │ │ │ └── answer.py
│ │ │ └── llm/ # LLM 适配层
│ │ │ ├── __init__.py
│ │ │ ├── base.py # LLM 基类
│ │ │ ├── doubao.py # 豆包 API
│ │ │ └── ollama.py # 本地 Ollama
│ │ │
│ │ └── diagnosis/ # 错误诊断与修复引擎（原diagnosis/目录）
│ │ ├── __init__.py
│ │ ├── base.py # 抽象基类
│ │ ├── classifier.py # 错误分类器
│ │ ├── analyzer.py # 根因分析器
│ │ ├── generator.py # 修复方案生成器
│ │ ├── executor.py # 修复执行器
│ │ ├── validator.py # 修复验证器
│ │ ├── fallback.py # 保底降级策略
│ │ └── audit.py # 错误审计日志
│ │
│ ├── extensions/ # 【新增】通用能力扩展层（核心框架提供抽象）
│ │ ├── __init__.py
│ │ ├── template/ # 1. 参数化模板引擎
│ │ │ ├── __init__.py
│ │ │ ├── parser.py # 模板解析器
│ │ │ ├── renderer.py # 参数渲染器
│ │ │ └── loader.py # 模板加载器
│ │ │
│ │ ├── snapshot/ # 2. 断点续跑 + 5. 全链路回溯
│ │ │ ├── __init__.py
│ │ │ ├── manager.py # 快照管理器
│ │ │ ├── storage.py # 快照存储
│ │ │ └── resume.py # 断点续跑
│ │ │
│ │ ├── memory/ # 3. 个人记忆复用引擎
│ │ │ ├── __init__.py
│ │ │ ├── orchestrator.py # 记忆智能调度器
│ │ │ ├── preference.py # 个人偏好库
│ │ │ ├── experience.py # 修复经验库
│ │ │ └── optimization.py # 流程优化经验库
│ │ │
│ │ ├── recorder/ # 4. 固定流程一键录制器
│ │ │ ├── __init__.py
│ │ │ ├── capture.py # 命令录制
│ │ │ ├── extractor.py # 参数提取
│ │ │ └── generator.py # 模板生成
│ │ │
│ │ └── trigger/ # 6. 定时/事件触发器
│ │ ├── __init__.py
│ │ ├── cron.py # Cron 定时触发
│ │ ├── fs.py # 文件系统事件触发
│ │ └── webhook.py # Webhook 事件触发
│ │
│ ├── skills/ # 【核心】增强技能系统（原skills/目录，核心框架层）
│ │ ├── __init__.py
│ │ ├── base.py # 技能基类
│ │ ├── registry.py # Skill Registry (技能注册中心)
│ │ ├── loader.py # Skill Loader (技能加载器)
│ │ ├── executor.py # Skill Executor (技能执行器)
│ │ ├── validator.py # Skill Validator (技能验证器)
│ │ ├── dependency.py # Skill Dependency Manager (依赖管理)
│ │ ├── version.py # Skill Version Manager (版本控制)
│ │ └── precheck.py # Skill PreCheck Manager (预检查)
│ │
│ ├── context/ # 项目上下文与智能管理（原context/目录）
│ │ ├── __init__.py
│ │ ├── base.py # 上下文基类
│ │ ├── project.py # 项目上下文
│ │ ├── scanner.py # 上下文扫描器
│ │ ├── orchestrator.py # 智能上下文编排器
│ │ └── budget.py # Token 预算管理器
│ │
│ ├── memory/ # 记忆与历史（原memory/目录，本地持久化层）
│ │ ├── __init__.py
│ │ ├── base.py # 存储基类
│ │ ├── store.py # Memory Store (记忆存储)
│ │ ├── models.py # Data Models (数据模型)
│ │ └── migrator.py # DB Migrator (数据库迁移)
│ │
│ ├── infra/ # 【新增】基础设施层（原tool/目录重新组织）
│ │ ├── __init__.py
│ │ ├── plugin/ # 插件管理器
│ │ │ ├── __init__.py
│ │ │ ├── base.py # 插件基类
│ │ │ ├── manager.py # 插件管理器
│ │ │ └── loader.py # 插件加载器
│ │ │
│ │ ├── tool/ # 系统工具集（原tool/目录）
│ │ │ ├── __init__.py
│ │ │ ├── cmd_exec.py # 命令执行
│ │ │ └── fs.py # 文件系统操作
│ │ │
│ │ ├── storage/ # 本地存储引擎
│ │ │ ├── __init__.py
│ │ │ ├── sqlite.py # SQLite 存储
│ │ │ └── file.py # 文件存储
│ │ │
│ │ └── env/ # 环境适配层
│ │ ├── __init__.py
│ │ ├── os.py # 跨 OS 兼容
│ │ └── vars.py # 环境变量
│ │
│ ├── terminal/ # 【新增】终端交互层（轻量可视化）
│ │ ├── __init__.py
│ │ ├── ui/ # 终端 UI 组件
│ │ │ ├── __init__.py
│ │ │ ├── progress.py # 进度条
│ │ │ ├── tree.py # 步骤树
│ │ │ └── panel.py # 面板
│ │ ├── hooks/ # 人工干预钩子
│ │ │ ├── __init__.py
│ │ │ ├── confirm.py # 确认钩子
│ │ │ └── modify.py # 参数修改钩子
│ │ └── debug/ # 回溯调试面板
│ │ ├── __init__.py
│ │ ├── traceback.py # 链路追踪
│ │ └── re_run.py # 单步重跑
│ │
│ └── cli/ # 命令行入口（原cli/目录）
│ ├── __init__.py
│ ├── main.py # 主入口
│ ├── commands/ # 子命令
│ │ ├── __init__.py
│ │ ├── run.py # 运行流程
│ │ ├── record.py # 录制流程
│ │ ├── debug.py # 调试流程
│ │ └── template.py # 模板管理
│ └── args.py # 参数解析
│
├── plugins/ # 【插件层】100%业务逻辑，零侵入核心框架
│ ├── __init__.py
│ ├── file_ops/ # 【试验性插件】文件操作插件
│ │ ├── __init__.py
│ │ ├── plugin.yaml # 插件定义
│ │ ├── skills/ # 技能实现
│ │ │ ├── __init__.py
│ │ │ ├── create_file.py
│ │ │ ├── delete_file.py
│ │ │ └── mkdir.py
│ │ ├── operators/ # 操作原语
│ │ │ ├── __init__.py
│ │ │ └── file_ops.py
│ │ ├── context/ # 插件专属上下文
│ │ │ ├── __init__.py
│ │ │ └── fs_context.py
│ │ ├── diagnosis/ # 插件专属诊断/修复/兜底
│ │ │ ├── __init__.py
│ │ │ ├── strategies.py # 修复策略
│ │ │ └── fallback.py # 兜底策略
│ │ ├── templates/ # 插件专属流程模板
│ │ │ └── file_operations.yaml
│ │ └── terminal/ # 插件专属终端交互
│ │ └── __init__.py
│ │
│ ├── dev_tools/ # 开发自动化插件
│ │ ├── __init__.py
│ │ ├── plugin.yaml
│ │ ├── skills/
│ │ ├── operators/
│ │ └── templates/
│ │
│ └── blender_3d/ # Blender 3D 自动化插件（后期实现）
│ ├── __init__.py
│ ├── plugin.yaml
│ ├── skills/
│ ├── operators/
│ ├── context/
│ ├── diagnosis/
│ └── templates/
│
├── .subhuti/ # 运行时数据目录
│ ├── memory/ # 记忆数据库
│ │ └── subhuti.db
│ ├── snapshots/ # 执行快照
│ ├── cache/ # 缓存
│ ├── skill_versions/ # 技能版本存储
│ └── config.yaml # 全局配置
│
├── templates/ # 【新增】全局流程模板库
│ ├── go_project.yaml
│ ├── python_project.yaml
│ └── blender_render.yaml
│
├── tests/ # 测试
│ ├── core/ # 核心框架测试
│ │ ├── test_pipeline.py
│ │ ├── test_skills.py
│ │ ├── test_task.py
│ │ └── test_diagnosis.py
│ ├── extensions/ # 扩展能力测试
│ │ ├── test_template.py
│ │ ├── test_snapshot.py
│ │ └── test_memory.py
│ └── plugins/ # 插件测试
│ └── test_file_ops.py
│
├── pyproject.toml
├── requirements.txt
└── README.md

一、目录结构设计原则（贴合单人开发）
核心框架与业务插件完全分离：subhuti/ 目录是 100% 通用核心框架，无任何业务逻辑；plugins/ 目录是 100% 业务实现，零侵入核心框架
保留原有核心骨架：你之前的 pipeline/、task/、agent/、diagnosis/、skills/、context/、memory/、tool/、cli/ 目录全部保留，只做重新组织和增量调整
新增目录清晰明确：新增的 core/、extensions/、infra/、terminal/、plugins/、templates/ 目录，职责单一，一看就懂
插件目录结构标准化：所有插件都遵循统一的目录结构，方便开发、维护、扩展
测试目录分层：核心框架、扩展能力、插件测试分离，单人开发时可以只跑相关测试

二、主要变化说明

1. 核心框架重新组织（subhuti/core/）
   把你之前的核心模块（pipeline/、task/、agent/、diagnosis/）统一放到 core/ 目录下，作为「核心调度与执行闭环层」，职责更清晰。
2. 新增通用能力扩展层（subhuti/extensions/）
   把你要的 6 个核心能力（参数化模板、断点续跑、记忆复用、一键录制、全链路回溯、定时触发）统一放到 extensions/
   目录下，作为核心框架提供的通用扩展能力，插件可以直接使用或扩展。
3. 新增基础设施层（subhuti/infra/）
   把你之前的 tool/ 目录重新组织，加上插件管理器、存储引擎、环境适配，作为底层基础设施，支撑上层所有模块。
4. 新增终端交互层（subhuti/terminal/）
   新增终端可视化、人工干预钩子、回溯调试面板，完全基于终端实现，无 Web 依赖，适合单人开发使用。
5. 插件层独立（plugins/）
   把你之前的 skills/ 目录（业务技能）移到 plugins/ 目录下，作为独立的插件层，100% 业务逻辑都在这里，零侵入核心框架。
6. 新增全局模板库（templates/）
   新增全局流程模板库，存放你常用的固定流程模板，插件也可以提供自己的专属模板。
   三、插件目录结构标准（所有插件统一遵循）
   plugins/{plugin_name}/
   ├── __init__.py # 插件主类，继承 BasePlugin
   ├── plugin.yaml # 插件定义文件（元数据、技能注册、扩展能力声明）
   ├── skills/ # 技能实现（面向用户任务）
   │ ├── __init__.py
   │ ├── {skill_1}.py
   │ └── {skill_2}.py
   ├── operators/ # 操作原语（原子操作，被技能调用）
   │ ├── __init__.py
   │ └── {operator_name}.py
   ├── context/ # 插件专属上下文（可选）
   │ ├── __init__.py
   │ └── {context_name}.py
   ├── diagnosis/ # 插件专属诊断/修复/兜底（可选）
   │ ├── __init__.py
   │ ├── strategies.py # 专属修复策略
   │ └── fallback.py # 专属兜底策略
   ├── templates/ # 插件专属流程模板（可选）
   │ └── {template_name}.yaml
   ├── terminal/ # 插件专属终端交互（可选）
   │ └── __init__.py
   └── tests/ # 插件测试（可选）
   └── test_{plugin_name}.py

四、分阶段落地建议（按目录优先级）
第一阶段（先跑通最小闭环）
先实现 subhuti/infra/：基础设施层，插件管理器、系统工具集、本地存储
再实现 subhuti/core/skills/：增强技能系统，技能加载、注册、执行
实现 plugins/file_ops/：试验性文件操作插件，跑通「插件加载→技能注册→执行」的最小闭环
实现 subhuti/cli/：基础命令行入口，能调用插件技能
第二阶段（核心执行闭环）
实现 subhuti/core/pipeline/：Pipeline 引擎，Stage/Step 调度
实现 subhuti/core/agent/：LangGraph AI 执行层
实现 subhuti/core/diagnosis/：错误诊断与修复引擎
实现 subhuti/extensions/memory/：基础记忆库，修复经验、个人偏好
实现 subhuti/terminal/ui/：基础终端可视化
第三阶段（完整能力闭环）
实现 subhuti/extensions/template/：参数化模板引擎
实现 subhuti/extensions/snapshot/：断点续跑、全链路回溯
实现 subhuti/extensions/recorder/：固定流程一键录制
实现 subhuti/extensions/trigger/：定时 / 事件触发器
实现 subhuti/terminal/hooks/ 和 subhuti/terminal/debug/：人工干预、回溯调试
第四阶段（业务扩展）
开发 plugins/dev_tools/：开发自动化插件
开发 plugins/blender_3d/：Blender 3D 自动化插件
完善全局模板库 templates/
完善测试 tests/

```md
Pipeline (流水线)
└── Stage (阶段)
└── Step (步骤)
└── Action (动作：执行脚本/命令)
```

```md
Automation Pipeline (自动化流水线)
└── Stage (阶段：Plan / Execute / Verify / Repair)
└── Step (步骤：具体的脚本/命令)
└── Skill (技能：Python脚本 / Bash命令)
```

┌─────────────────────────────────────────────────────────────────┐
│ Subhuti 自动化流水线 │
├─────────────────────────────────────────────────────────────────┤
│ │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ 【上层】原生 Pipeline 引擎（你自己写） │ │
│ │ ┌─────────────────────────────────────────────────────┐ │ │
│ │ │ Pipeline (流水线)                                    │ │ │
│ │ │ └── Stage 1: Init │ │ │
│ │ │ └── Step 1.1: Check Environment │ │ │
│ │ │ └── Step 1.2: Load Skills │ │ │
│ │ │ └── Stage 2: AI Planning (调用 LangGraph)          │ │ │
│ │ │ └── Stage 3: Execute │ │ │
│ │ │ └── Step 3.1: Task 1 │ │ │
│ │ │ └── Step 3.2: Task 2 │ │ │
│ │ │ └── Stage 4: Verify │ │ │
│ │ └─────────────────────────────────────────────────────┘ │ │
│ └───────────────────────────────────────────────────────────┘ │
│ ↓ (调用)                                │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ 【下层】LangGraph AI 层（AI 决策） │ │
│ │ ┌─────────────────────────────────────────────────────┐ │ │
│ │ │ Plan → Execute → Observation → Judge → Answer │ │ │
│ │ │ ↓ (retry) ↑ │ │ │
│ │ └─────────────────────────────────────────────────────┘ │ │
│ └───────────────────────────────────────────────────────────┘ │
│ │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ 【底层】技能系统 + 项目上下文 + 错误诊断 │ │
│ └───────────────────────────────────────────────────────────┘ │
│ │
└─────────────────────────────────────────────────────────────────┘