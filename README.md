<h1 align="center">
  <br>
  <img src="https://raw.githubusercontent.com/zellij-org/zellij/main/assets/logo.png" alt="logo" width="200">
  <br>
  Zellij 中文版 (zellij-cn)
  <br>
  <br>
</h1>

<p align="center">
  <img src="https://raw.githubusercontent.com/zellij-org/zellij/main/assets/demo.gif" alt="demo">
</p>

<p align="center">
  <a href="https://github.com/Yintel12138/zellij-cn/releases/latest"><img alt="Latest Release" src="https://img.shields.io/github/v/release/Yintel12138/zellij-cn?style=flat-square&label=最新版本"></a>
  <a href="https://github.com/Yintel12138/zellij-cn/actions/workflows/rust.yml"><img alt="Build Status" src="https://img.shields.io/github/actions/workflow/status/Yintel12138/zellij-cn/rust.yml?branch=main&style=flat-square&label=构建状态"></a>
  <a href="LICENSE.md"><img alt="License" src="https://img.shields.io/badge/license-MIT-blue?style=flat-square"></a>
</p>

---

**zellij-cn** 是 [Zellij](https://github.com/zellij-org/zellij) 的中文本地化 fork，在不修改原有功能的前提下，新增了 `language` 配置项，可将终端多路复用器的界面切换为中文。

> **与上游保持同步**：所有改动均为加法式（新增字段，默认英文），可直接 `git merge` 上游更新，几乎不产生冲突。

---

## 安装方法

### 方法一：下载预编译二进制（推荐，无需 Rust 环境）

前往 [Releases 页面](https://github.com/Yintel12138/zellij-cn/releases/latest) 下载对应平台的压缩包，或使用以下一键命令：

**Linux x86_64：**
```bash
curl -L https://github.com/Yintel12138/zellij-cn/releases/latest/download/zellij-x86_64-unknown-linux-musl.tar.gz \
  | tar xz
sudo mv zellij /usr/local/bin/
```

**Linux ARM64（树莓派等）：**
```bash
curl -L https://github.com/Yintel12138/zellij-cn/releases/latest/download/zellij-aarch64-unknown-linux-musl.tar.gz \
  | tar xz
sudo mv zellij /usr/local/bin/
```

**macOS Apple Silicon (M1/M2/M3)：**
```bash
curl -L https://github.com/Yintel12138/zellij-cn/releases/latest/download/zellij-aarch64-apple-darwin.tar.gz \
  | tar xz
sudo mv zellij /usr/local/bin/
```

**macOS Intel：**
```bash
curl -L https://github.com/Yintel12138/zellij-cn/releases/latest/download/zellij-x86_64-apple-darwin.tar.gz \
  | tar xz
sudo mv zellij /usr/local/bin/
```

**Windows：** 在 [Releases](https://github.com/Yintel12138/zellij-cn/releases/latest) 下载 `zellij-x86_64-pc-windows-msvc.zip` 或 `.msi` 安装包。

---

### 方法二：`cargo binstall`（自动选择平台二进制）

```bash
cargo binstall --git https://github.com/Yintel12138/zellij-cn zellij
```

> 需要先安装 [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)：`cargo install cargo-binstall`

---

### 方法三：从源码编译

```bash
# 克隆本仓库
git clone https://github.com/Yintel12138/zellij-cn
cd zellij-cn

# 安装依赖并编译（需要 Rust 1.92+ 及 wasm32-wasip1 target）
rustup target add wasm32-wasip1
cargo install --path . --locked
```

---

## 启用中文界面

安装完成后，编辑 Zellij 配置文件，添加 `language "zh"` 即可：

**配置文件路径：** `~/.config/zellij/config.kdl`

```kdl
// 将界面语言切换为中文
language "zh"

// 其余配置保持不变...
```

然后启动 Zellij：

```bash
zellij
```

### 效果预览

| 区域 | 英文（默认） | 中文（`language "zh"`） |
|------|------------|------------------------|
| 普通模式 | `NORMAL` | `普通` |
| 面板模式 | `PANE` | `面板` |
| 标签模式 | `TAB` | `标签` |
| 调整模式 | `RESIZE` | `调整` |
| 会话模式 | `SESSION` | `会话` |
| 搜索模式 | `SEARCH` | `搜索` |
| 锁定模式 | `LOCK` | `锁定` |
| 界面锁定横幅 | `-- INTERFACE LOCKED --` | `-- 界面已锁定 --` |
| 标签重命名输入框 | `Enter name...` | `输入名称...` |

---

## 开发环境

```bash
# 克隆并运行（调试构建）
git clone https://github.com/Yintel12138/zellij-cn
cd zellij-cn
cargo xtask run

# 运行全部测试
cargo xtask test
```

更多构建命令请参阅 [CONTRIBUTING.md](CONTRIBUTING.md)。

---

## GitHub Actions

本仓库已配置自动化工作流：

| 工作流 | 触发条件 | 说明 |
|--------|---------|------|
| **Rust**（[rust.yml](.github/workflows/rust.yml)） | push/PR 到 `main` | 编译 + 测试（Linux、macOS、Windows） |
| **Release**（[release.yml](.github/workflows/release.yml)） | 推送 `v*.*.*` tag 或手动触发 | 编译多平台二进制并发布到 GitHub Releases |

**手动触发 Release：** 在仓库的 `Actions → Release → Run workflow` 中输入 tag 名称（如 `v0.44.0-cn1`）即可触发构建。

---

## 与上游的关系

本项目基于 [zellij-org/zellij](https://github.com/zellij-org/zellij) v0.44.0，仅新增以下内容：
- `Options` 结构体中的 `language` 字段（含 KDL 解析、protobuf 传递）
- 各内置插件的 `translations.rs` 翻译模块
- 插件 `load()` 入口处读取语言配置的少量修改

上游升级时，冲突极少，可直接 merge。

---

## 许可证

MIT

