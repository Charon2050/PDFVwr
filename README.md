# PDFVwr

A lightweight PDF viewer on Windows. Based on WebView2.  
Windows 平台的轻量级 PDF 查看器。基于 WebView2。

## Usage 使用

Download the executable file from the Release page of this project, place it in a fixed location, and run it directly or drag the PDF document into it.  
从本项目的 Release 页面下载可执行文件后，放置在固定位置，直接运行或将 PDF 文档拖入即可。

## Build 构建

<details>

<summary>
Rust + msvc runtime environment is required. Please install it first if not available.<br>
需要 Rust + msvc 运行环境。如果没有，请先安装。
</summary>

1. Download and run [Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/);  
   下载并运行[适用于 C++ 的 Visual Studio 生成工具](https://visualstudio.microsoft.com/visual-cpp-build-tools/)；

2. Install `MSVC Build Tools for x64/x86 (Latest)` and `Windows 11 SDK` in "Individual components" tab;  
   在「单个字段」选项卡下安装 `MSVC x64/x86 生成工具（最新版）` 和 `Windows 11 SDK`；

3. Download and install [Rust](https://rust-lang.org/tools/install/);  
   下载并安装 [Rust](https://rust-lang.org/tools/install/)；

4. [optional] Download and install [Node.JS](https://nodejs.org/en/download);  
   [可选] 下载并安装 [Node.JS](https://nodejs.org/zh-cn/download)；

</details>

```cmd
git clone https://github.com/Charon2050/PDFVwr
cd PDFVwr
npm install
npm run tauri build
```

## Prompts 提示词

https://claude.ai/share/fd95fcd9-106b-42f7-a6a6-d1352302742d
