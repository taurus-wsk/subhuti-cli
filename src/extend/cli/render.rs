// src/core/cli/render.rs
use colored::*;
use indicatif::{ProgressBar};
use std::time::Duration;

pub struct TerminalRenderer;

impl TerminalRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn print_banner(&self) {
        println!("{}", "Subhuti 个人自动化流水线引擎".blue().bold());
        println!();
    }

    pub fn create_main_progress(&self, _total: u64, _message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    pub fn create_step_progress(&self, _message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(80));
        pb
    }

    pub fn print_step(&self, step_num: usize, total: usize, name: &str, status: StepStatus) {
        let status_str = match status {
            StepStatus::Pending => "⏳",
            StepStatus::Running => "🔄",
            StepStatus::Completed => " ✅",
            StepStatus::Failed => "❌",
            StepStatus::Skipped => "⏭️",
        };
        println!("  {} [{}/{}] {}", status_str, step_num, total, name);
    }

    pub fn print_error(&self, message: &str) {
        eprintln!("\n{} {}", "❌ 错误:".red().bold(), message);
    }

    pub fn print_success(&self, message: &str) {
        println!("\n{} {}", " ✅ 成功:".green().bold(), message);
    }

    pub fn print_warning(&self, message: &str) {
        println!("\n{} {}", "⚠️  警告:".yellow().bold(), message);
    }
    

    pub fn clear(&self) {}
}

#[derive(Debug, Clone, Copy)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}