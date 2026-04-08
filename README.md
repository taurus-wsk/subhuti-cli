# 打包命令
cargo build --release

调试命令
cargo run -- list-skills
# 或者用缩写
cargo run -- ls

# 随便指定一个模板文件（虽然是空实现，但能跑通流程）
cargo run -- pipeline --template dummy.yaml --var name=my-demo --var version=1.0
# 或者用缩写
cargo run -- p --template dummy.yaml

# 自然语言描述任务
cargo run -- task "帮我创建一个 Blender 场景"
# 或者用缩写
cargo run -- t "帮我创建一个 Blender 场景"

# 加上交互式模式
cargo run -- task "帮我创建一个 Blender 场景" --interactive

# 列出已加载插件
cargo run -- plugin list
# 或者用缩写
cargo run -- pl list

# 加载插件
cargo run -- plugin load --path plugins/file_ops

# 卸载插件
cargo run -- plugin unload --name file_ops

cargo run -- history --limit 20
# 或者用缩写
cargo run -- h

# 编译 release 版本（优化过，速度快）
cargo build --release

# 编译好的文件在 target/release/subhuti
# 直接运行
./target/release/subhuti --help
./target/release/subhuti list-skills




