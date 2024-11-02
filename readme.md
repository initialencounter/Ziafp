# Ziafp

一个用于文件自动上传的Windows工具。

## 功能特点

- 🚀 自动匹配文件 - 根据文件夹名称和文件类型自动匹配文件
- 📤 一键上传 - 通过右键菜单快速上传匹配的文件
- 💗 心跳机制 - 自动保持登录状态
- 🔄 自启动 - 支持开机自动启动服务
- 🔒 安全可靠 - 支持用户认证和权限验证

## 安装说明

1. 从 [Releases](https://github.com/initialencounter/ziafp/releases/latest) 页面下载最新版本的 `server.exe` 和 `client.exe`

2. 创建配置文件 `local.env`，内容如下：
```ini
BASE_URL=系统域名
USER_NAME=用户名
PASSWORD=密码
PORT=25455
```

3. 运行 `client.exe` 注册右键菜单（需要管理员权限）

4. 运行 `server.exe` 启动服务

## 使用方法

1. 确保 `server.exe` 正在运行
2. 在文件资源管理器中，右键点击目标文件夹
3. 选择 "Upload file here" 选项
4. 确认上传文件列表
5. 等待上传完成

## 系统要求

- Windows 操作系统
- 管理员权限（首次安装时需要）

## 开发相关

本项目使用 Rust 语言开发，主要依赖：

- tokio - 异步运行时
- warp - Web 服务器框架
- reqwest - HTTP 客户端
- winreg - Windows 注册表操作

## 许可证

[AGPL-3.0](LICENSE)

## 贡献指南

欢迎提交 Issue 和 Pull Request！
