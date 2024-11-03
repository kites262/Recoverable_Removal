#!/bin/bash

# 定义下载 URL 和目标路径
URL="https://github.com/kites262/Recoverable_Removal/releases/download/v0.1.2/rr-amd64"
DEST="/usr/local/bin/rr"

# 下载文件
echo "Downloading rr-amd64 from $URL..."
curl -L -o rr-amd64 $URL

# 确保下载成功
if [ ! -f "rr-amd64" ]; then
    echo "Download failed. Please check the URL and try again."
    exit 1
fi

# 赋予执行权限
chmod +x rr-amd64

# 移动文件到目标路径
echo "Moving rr-amd64 to /usr/local/bin/ as rr..."
sudo mv rr-amd64 $DEST

# 检查是否成功
if [ -f "$DEST" ]; then
    echo "rr has been successfully installed to /usr/local/bin."
else
    echo "Failed to move rr-amd64 to /usr/local/bin."
    exit 1
fi
