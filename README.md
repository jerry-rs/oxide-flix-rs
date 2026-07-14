# Oxide Flix 🦀

一个基于 Rust + Axum 的轻量级本地视频流媒体播放器。自动扫描指定目录下的视频文件，提供 Web 界面进行浏览、播放和管理。

## 截图

```
┌─────────────────────────────────────────────┐
│  🦀 Oxide Flix                                │
├─────────────────────────────────────────────┤
│  📁 根目录 / Movies / Sci-Fi                 │
├─────────────────────────────────────────────┤
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐      │
│  │ 📁   │ │ 📁   │ │ 🎬  │ │ 🎬  │      │
│  │Movies│ │  TV  │ │alien│ │ dune│      │
│  │文件夹 │ │文件夹 │ │1.2GB│ │ 2.1G│      │
│  └──────┘ └──────┘ └──────┘ └──────┘      │
├─────────────────────────────────────────────┤
│  ┌─────────────────────────────────────┐    │
│  │           ▶ 视频播放器               │    │
│  └─────────────────────────────────────┘    │
│  ┌─────────────────────────────────────┐    │
│  │ 🎬 alien.mp4                        │    │
│  │ 文件名称: alien.mp4  大小: 1.2 GB   │    │
│  │ 编码格式: video/mp4  模式: html5    │    │
│  └─────────────────────────────────────┘    │
└─────────────────────────────────────────────┘
```

## 功能

- 📂 **文件浏览器** — 以网格形式展示目录下的所有视频文件和子文件夹，支持面包屑导航
- ▶️ **视频播放** — 基于 HTML5 `<video>` 的流式播放，支持进度条拖拽（HTTP Range）
- 📋 **元数据显示** — 展示文件名、大小、MIME 类型等基本信息
- 🗑️ **删除管理** — 鼠标悬停时出现删除按钮，支持删除文件或整个文件夹
- 🔒 **路径安全** — 所有文件操作都经过路径净化，防止目录穿越攻击
- 🌐 **响应式布局** — 支持不同屏幕尺寸

## 支持的视频格式

| 格式 | 扩展名 |
|------|--------|
| MP4  | `.mp4` |
| MKV  | `.mkv` |
| WebM | `.webm` |
| AVI  | `.avi` |
| MOV  | `.mov` |
| WMV  | `.wmv` |
| FLV  | `.flv` |
| M4V  | `.m4v` |
| TS   | `.ts`  |
| M3U8 | `.m3u8` |

> 实际能否播放取决于浏览器的解码能力。推荐使用 Chrome / Edge 等现代浏览器以获得最佳兼容性。

## 快速开始

### 前置条件

- Rust 工具链（edition 2024）

### 编译 & 运行

```bash
# 编译
cargo build --release

# 运行（使用当前目录作为媒体目录）
./target/release/oxide-flix-rs

# 指定媒体目录
./target/release/oxide-flix-rs --data-dir /path/to/your/videos

# 指定端口
./target/release/oxide-flix-rs --port 8080
```

打开浏览器访问 `http://127.0.0.1:1000`。

### 配置项

| 参数 | 环境变量 | 默认值 | 说明 |
|------|---------|--------|------|
| `--ipv4` | `APP_IPV4` | `0.0.0.0` | 监听地址 |
| `--port` | `APP_PORT` | `1000` | 监听端口 |
| `--data-dir` | `APP_DATA_DIR` | 当前目录 | 媒体文件根目录 |

## API 接口

所有 API 均返回 JSON，并经过路径安全校验。

### `GET /` — 首页

返回 Web 管理界面。

### `GET /api/video/list` — 根目录列表

列出 `APP_DATA_DIR` 根目录下的文件和文件夹。

### `GET /api/video/list/{*path}` — 子目录列表

列出指定子目录下的文件和文件夹。

**响应示例：**
```json
{
  "entries": [
    { "name": "Movies", "is_dir": true, "size_bytes": 0 },
    { "name": "video.mp4", "is_dir": false, "size_bytes": 123456789 }
  ],
  "current_path": "Movies"
}
```

### `GET /api/video/info/{*path}` — 视频元数据

获取视频文件的基本信息。

### `GET /api/video/stream/{*path}` — 视频流

流式传输视频文件，支持 HTTP Range 头实现进度拖拽。

### `DELETE /api/video/delete/{*path}` — 删除

删除指定的文件或文件夹（文件夹会递归删除所有内容）。

## 项目结构

```
oxide-flix-rs/
├── Cargo.toml          # 项目依赖与配置
├── src/
│   └── main.rs         # 服务端入口：路由、Handler、路径安全逻辑
├── templates/
│   └── index.html      # 前端页面：文件浏览器 + 播放器
└── README.md
```

## 技术栈

- **后端**: [Axum](https://github.com/tokio-rs/axum) (Web 框架) + [Tokio](https://tokio.rs) (异步运行时)
- **前端**: 原生 HTML + CSS + JavaScript（无框架依赖）
- **模板**: [Askama](https://github.com/rust-askama/askama) (服务端模板引擎)
- **命令行**: [Clap](https://github.com/clap-rs/clap) (参数解析)
