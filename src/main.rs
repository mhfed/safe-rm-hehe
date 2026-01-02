use clap::{Parser, ArgAction};
use std::path::PathBuf;
use std::io::{self, Write};
use trash;
use colored::*; // Thêm crate 'colored' để thông báo lỗi nổi bật hơn

#[derive(Parser)]
#[command(
    name = "safe-rm", 
    about = "🛡️ Vị cứu tinh cho Dev - Chuyển rm thành Trash",
    version = "1.0"
)]
struct Cli {
    /// Danh sách file hoặc thư mục
    files: Vec<PathBuf>,

    /// Chấp nhận -r, -R (đệ quy) để tương thích với lệnh rm gốc
    #[arg(short, short_alias = 'R', long, action = ArgAction::SetTrue)]
    recursive: bool,

    /// Chấp nhận -f (force) để không báo lỗi nếu file không tồn tại
    #[arg(short, long, action = ArgAction::SetTrue)]
    force: bool,

    /// Flag tùy chỉnh: Hiển thị danh sách vừa xóa
    #[arg(short, long, action = ArgAction::SetTrue)]
    list: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.files.is_empty() && !cli.list {
        println!("{}", "💡 Cách dùng: safe-rm <file_name> hoặc safe-rm -rf <dir_name>".yellow());
        return;
    }

    for file in &cli.files {
        if file.exists() {
            match trash::delete(file) {
                Ok(_) => println!("{} {}", "✓ Đã đưa vào thùng rác:".green(), file.display()),
                Err(e) => eprintln!("{} {:?}: {}", "✘ Lỗi khi xóa".red(), file, e),
            }
        } else if !cli.force {
            // Chỉ hiện lỗi nếu không dùng flag -f (đúng chuẩn lệnh rm)
            eprintln!("{} {:?}: Không tìm thấy file", "⚠️ Warning:".yellow(), file);
        }
    }
}
