# faml

[![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Ffawdlstty%2Ffaml%2Fmain%2Ffaml%2FCargo.toml&query=package.version&label=version)](https://crates.io/crates/faml)
![status](https://img.shields.io/github/actions/workflow/status/fawdlstty/faml/rust.yml)
[![docs](https://img.shields.io/badge/docs-latest-blue)](https://faml.fawdlstty.com/)

[English](README.md) | 简体中文

**faml** 是一款极简而强大的动态配置语言，它扩展了 TOML 语法，增加了脚本能力，支持在运行时动态更新配置。

## 帮助

遇到问题？加QQ群求助：`1018390466`

## 核心特性

- **TOML 兼容语法**：对熟悉 TOML 的用户友好，同时增强动态能力
- **动态表达式**：使用 `$"..."` 语法嵌入表达式，实现运行时值计算
- **条件配置**：通过 `@if` 指令有条件地启用配置块
- **丰富的数据类型**：内置支持持续时间、距离和量化数字等特殊类型
- **多语言支持**：提供 Rust 原生 API 以及 C/C++ 和 C# 绑定
- **运行时可变性**：支持在运行时修改配置值并自动更新依赖项

## 文档资源

- [前言](https://faml.fawdlstty.com/guide/00_introduction.html)
- [快速开始](https://faml.fawdlstty.com/guide/01_hello_world.html)
- [语法指南](https://faml.fawdlstty.com/guide/02_structs_and_types.html)
- [表达式](https://faml.fawdlstty.com/guide/03_expressions.html)
- [方法](https://faml.fawdlstty.com/guide/04_methods.html)

## 许可证

本项目采用 MIT 许可证，详情请见 [LICENSE](LICENSE) 文件。
