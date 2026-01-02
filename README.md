🛡️ Safe-RM (safe-rm-hehe)
"Đã bao giờ bạn gõ rm -rf * và nhận ra mình chưa push code? Tôi đã từng, và đó là lý do Safe-RM ra đời."
Safe-RM là một công cụ dòng lệnh (CLI) thay thế lệnh rm mặc định. Thay vì hủy diệt dữ liệu vĩnh viễn, nó đưa tệp tin vào Thùng rác (Trash), giúp bạn có cơ hội sửa sai trước khi quá muộn.
🚀 Cài đặt nhanh (Quick Start)
1. Cài đặt công cụ
Chọn một trong hai cách phổ biến nhất sau đây:
Cách 1: Qua Homebrew (Khuyên dùng cho macOS/Linux)
brew tap mhfed/tap
brew install safe-rm-hehe


Cách 2: Qua Cargo (Dành cho lập trình viên Rust)
cargo install safe-rm-hehe


2. Thiết lập Alias (Bắt buộc)
Để Safe-RM thực sự bảo vệ bạn, bạn cần ánh xạ lệnh rm gốc sang safe-rm-hehe trong file cấu hình Shell.
Đối với Zsh (mặc định trên macOS):
echo "alias rm='safe-rm-hehe'" >> ~/.zshrc
source ~/.zshrc


Đối với Bash:
echo "alias rm='safe-rm-hehe'" >> ~/.bashrc
source ~/.bashrc


📖 Cách sử dụng
Sử dụng Safe-RM hoàn toàn giống với lệnh rm truyền thống, không cần thay đổi thói quen của bạn:
Xóa tệp đơn lẻ: rm document.pdf
Xóa thư mục đệ quy: rm -rf node_modules/ (Vẫn đưa vào thùng rác an toàn)
Xóa nhiều mục cùng lúc: rm file1.txt photo.png backup_folder/
✨ Điểm nổi bật
🛡️ Lớp bảo vệ an toàn: Mọi dữ liệu bị xóa đều có thể khôi phục dễ dàng từ Thùng rác hệ thống.
🧩 Tương thích hoàn toàn: Hỗ trợ đầy đủ các flag quen thuộc như -r, -f, -rf, -R.
🎨 Thông báo trực quan: Sử dụng màu sắc để phân biệt thao tác thành công (xanh) và lỗi (đỏ/vàng).
⚡ Hiệu suất cao: Được xây dựng bằng Rust, đảm bảo tốc độ thực thi cực nhanh và an toàn bộ nhớ.
⚠️ Lưu ý quan trọng
Dung lượng ổ đĩa: Dữ liệu trong Thùng rác vẫn chiếm dung lượng ổ cứng. Bạn cần "Empty Trash" để giải phóng hoàn toàn dung lượng khi cần thiết.
Xóa vĩnh viễn: Trong trường hợp bạn thực sự muốn xóa mà không qua thùng rác (không thể khôi phục), hãy sử dụng đường dẫn tuyệt đối của lệnh hệ thống:
/bin/rm -rf path/to/file


📜 Giấy phép & Tác giả
Giấy phép: MIT.
Phát triển bởi: Hieu Minh với ❤️ và bài học xương máu về việc mất code.

