# 桌面端框架

## 前置要求

### Microsoft C++ 生成工具

### WebView2 运行时

常青独立安装程序

## Rust

`winget install Rustlang.Rustup`

## deno

`winget install --id=DenoLand.Deno`

## Tauri

进入项目
`cargo install tauri-cli`
`rustup default stable`
`deno install`
`([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)`
`netsh int ip reset; netsh winsock reset`
运行 `cargo tauri dev`
