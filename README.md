🛡️ Safe-RM (safe-rm-hehe)

Safe-RM là một công cụ dòng lệnh (CLI) giúp thay thế lệnh rm mặc định trên hệ thống của bạn. Thay vì xóa vĩnh viễn dữ liệu và gây ra những thảm họa không đáng có, safe-rm sẽ di chuyển các tệp tin vào Thùng rác (Trash) của hệ điều hành.

✨ Điểm nổi bật

Lớp bảo vệ an toàn: Mọi dữ liệu bị xóa đều có thể khôi phục dễ dàng từ Thùng rác.

Tương thích hoàn toàn: Bạn vẫn có thể sử dụng các flag quen thuộc như -r, -f, -rf, -R.

Thông báo trực quan: Sử dụng màu sắc để phân biệt giữa thao tác thành công (xanh) và lỗi (đỏ/vàng).

Hiệu suất cao: Được xây dựng bằng Rust, đảm bảo tốc độ thực thi nhanh và an toàn bộ nhớ.

🚀 Cài đặt

1. Cài đặt qua Cargo

Đây là cách nhanh nhất nếu bạn đã cài đặt môi trường Rust:

cargo install safe-rm-hehe


2. Thiết lập Alias (Bí danh)

Để safe-rm thực sự thay thế được lệnh rm gốc, bạn cần thêm bí danh vào file cấu hình Shell của mình.

Đối với Zsh (mặc định trên macOS):

echo "alias rm='safe-rm-hehe'" >> ~/.zshrc
source ~/.zshrc


Đối với Bash:

echo "alias rm='safe-rm-hehe'" >> ~/.bashrc
source ~/.bashrc


📖 Cách sử dụng

Sử dụng safe-rm hoàn toàn giống với lệnh rm truyền thống:

# Xóa một tệp tin đơn lẻ
rm document.pdf

# Xóa thư mục đệ quy (vẫn đưa vào Thùng rác)
rm -rf node_modules/

# Xóa nhiều tệp tin cùng lúc
rm file1.txt photo.png backup_folder/


⚠️ Lưu ý quan trọng

Dung lượng ổ đĩa: Dữ liệu trong Thùng rác vẫn chiếm dung lượng ổ cứng. Bạn cần "Empty Trash" để giải phóng hoàn toàn dung lượng.

Xóa vĩnh viễn: Trong trường hợp bạn thực sự muốn xóa mà không qua thùng rác (không thể khôi phục), hãy sử dụng đường dẫn tuyệt đối của lệnh hệ thống:

/bin/rm -rf path/to/file


📜 Giấy phép

Dự án này được phân phối dưới giấy phép MIT. Xem tệp LICENSE để biết thêm chi tiết.

Phát triển bởi Hieu Minh với ❤️ và Rust.
