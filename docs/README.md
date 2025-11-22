---
home: true
title: Home
#heroImage: https://vuejs.press/images/hero.png
actions:
  - text: 入门示例
    link: /guide/01_hello_world.md
    type: primary
  - text: GitHub
    link: https://github.com/fawdlstty/faml
    type: secondary

features:
  - title: TOML风格语法
    details: 兼容TOML语法风格，使用方括号定义配置块，等号赋值，保持高可读性和易写性，学习门槛低
  - title: 动态表达式计算
    details: 支持在配置中嵌入表达式，可引用其他配置项、进行算术运算、逻辑判断等操作，实现真正的动态配置
  - title: 条件配置块
    details: 通过@if指令根据条件控制配置项是否生效，便于多环境配置管理和条件化配置
  - title: 丰富数据类型
    details: 内置对持续时间、距离、存储容量等特殊数据类型的支持，可直接在配置中使用单位（如5 minutes、100 MB）
  - title: 运行时动态修改
    details: 配置可在运行时被程序动态修改，并自动重新计算依赖项，实现真正的动态配置更新
  - title: 智能计算跟踪
    details: 提供强大的trace功能，可以追踪复杂表达式的每一步计算过程，快速定位配置计算错误的根源

footer: MIT Licensed | Copyright © 2025 faml
---

<!--
npm run docs:build
npm run docs:dev
-->