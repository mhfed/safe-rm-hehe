use clap::{Parser, ArgAction};
use std::path::PathBuf;
use colored::*;
use trash;

#[derive(Parser)]
#[command(
    name = "safe-rm", 
    about = "🛡️ Vị cứu tinh cho Dev - Chuyển rm thành Trash",
    version = "0.1.1"
)]
struct Cli {
    /// Danh sách file hoặc thư mục
    files: Vec<PathBuf>,

    /// Tương thích với -r, -R
    #[arg(short, short_alias = 'R', long, action = ArgAction::SetTrue)]
    recursive: bool,

    /// Tương thích với -f (không báo lỗi nếu file không tồn tại)
    #[arg(short, long, action = ArgAction::SetTrue)]
    force: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.files.is_empty() {
        println!("{}", "💡 Cách dùng: rm <file_name> hoặc rm -rf <dir_name>".yellow());
        return;
    }

    for file in &cli.files {
        if file.exists() {
            match trash::delete(file) {
                Ok(_) => {
                    println!("{} {}", "✓ Đã đưa vào Trash:".green().bold(), file.display());
                }
                Err(e) => {
                    eprintln!("{} {:?}: {}", "✘ Lỗi:".red().bold(), file, e);
                }
            }
        } else if !cli.force {
            eprintln!("{} {:?}: Không tìm thấy file", "⚠️ Warning:".yellow().bold(), file);
        }
    }
}
