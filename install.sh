#!/bin/bash

# 1. Xác định hệ điều hành
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "🚀 Đang cài đặt safe-rm cho $OS ($ARCH)..."

# 2. Tải binary từ GitHub Release (Bạn cần thay URL sau khi publish)
# Ví dụ: URL="https://github.com/user/safe-rm/releases/latest/download/safe-rm-$OS"
# curl -L $URL -o /usr/local/bin/safe-rm

# 3. Cài đặt quyền thực thi
# chmod +x /usr/local/bin/safe-rm

# 4. Tự động thêm Alias vào Shell (Tính năng quan trọng nhất)
SHELL_CONFIG=""
if [[ $SHELL == *"zsh"* ]]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [[ $SHELL == *"bash"* ]]; then
    SHELL_CONFIG="$HOME/.bashrc"
fi

if [ -n "$SHELL_CONFIG" ]; then
    if ! grep -q "alias rm=" "$SHELL_CONFIG"; then
        echo "alias rm='safe-rm'" >> "$SHELL_CONFIG"
        echo "✅ Đã thêm alias vào $SHELL_CONFIG. Hãy khởi động lại Terminal hoặc gõ 'source $SHELL_CONFIG'"
    else
        echo "ℹ️ Alias 'rm' đã tồn tại, hãy kiểm tra lại file cấu hình của bạn."
    fi
fi

echo "🎉 Cài đặt hoàn tất! Từ nay gõ 'rm' sẽ cực kỳ an toàn."
