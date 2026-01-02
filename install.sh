#!/bin/bash

echo "🚀 Đang chuẩn bị cài đặt safe-rm-hehe..."

# 1. Kiểm tra và cài đặt binary qua Cargo
if ! command -v safe-rm-hehe &> /dev/null; then
    echo "📦 Đang cài đặt từ crates.io..."
    cargo install safe-rm-hehe
else
    echo "✅ safe-rm-hehe đã được cài đặt."
fi

# 2. Xác định file cấu hình Shell
SHELL_CONFIG=""
case $SHELL in
    */zsh)  SHELL_CONFIG="$HOME/.zshrc" ;;
    */bash) SHELL_CONFIG="$HOME/.bashrc" ;;
    *)      echo "⚠️ Không hỗ trợ shell này, hãy tự thêm alias thủ công." ;;
esac

# 3. Thêm Alias (Sử dụng tên binary chính xác trên crates.io)
if [ -n "$SHELL_CONFIG" ]; then
    if ! grep -q "alias rm=" "$SHELL_CONFIG"; then
        echo -e "\n# Safe RM Alias\nalias rm='safe-rm-hehe'" >> "$SHELL_CONFIG"
        echo "✅ Đã thêm alias vào $SHELL_CONFIG"
        echo "👉 Hãy chạy: source $SHELL_CONFIG để bắt đầu sử dụng."
    else
        echo "ℹ️ Alias 'rm' đã tồn tại trong $SHELL_CONFIG."
    fi
fi

echo "🎉 Xong! Bây giờ bạn có thể gõ 'rm -rf' mà không sợ mất dữ liệu."
