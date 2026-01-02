🛡️ Safe-RM (Hehe)
Đã bao giờ bạn gõ rm -rf * và nhận ra mình chưa push code lên Git? > Tôi đã trải qua cảm giác đó ngay trước ngày báo cáo dự án, và đó là lý do safe-rm ra đời.

✨ Tính năng
Không xóa thật: Chuyển file vào Thùng rác hệ thống (Trash) để có thể khôi phục.

Tương thích: Hỗ trợ các flag -r, -f, -rf như lệnh rm gốc.

Đa nền tảng: Chạy mượt trên macOS và Linux.

🚀 Cài đặt nhanh
Bash

cargo install safe-rm-hehe
Sau đó thêm alias vào .zshrc hoặc .bashrc:

Bash

alias rm='safe-rm-hehe'
