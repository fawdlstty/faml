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
  - title: 简洁易用的语法
    details: 兼容TOML语法风格，使用方括号定义配置块，等号赋值，保持与TOML一致的可读性和易写性，学习门槛低
  - title: 强大的动态表达式
    details: 支持在配置中嵌入表达式，可引用其他配置项、进行算术运算、逻辑判断等操作，实现真正的动态配置
  - title: 条件配置控制
    details: 通过@if指令根据条件控制配置项是否生效，便于不同环境下的配置管理
  - title: 特殊数据类型支持
    details: 内置对持续时间、距离等特殊数据类型的支持，可直接在配置中使用这些单位（如5 minutes、100 MB）
  - title: 丰富的表达式系统
    details: 支持算术运算、比较运算、逻辑运算、三元条件运算符和函数调用等丰富的表达式操作
  - title: 运行时动态修改
    details: 配置可在运行时被程序动态修改，并自动重新计算依赖这些值的其他配置项，实现真正的动态配置更新
  - title: 智能计算跟踪
    details: 提供强大的trace功能，可以追踪复杂表达式的每一步计算过程，快速定位配置计算错误的根源
  - title: 模板化配置管理
    details: 支持模板配置和定制配置的组合应用，便于管理多个相似但略有差异的配置方案
  - title: 无缝集成现有配置
    details: 可以轻松将FAML表达式功能集成到现有的JSON配置中，无需修改原有配置结构即可享受动态计算能力

footer: MIT Licensed | Copyright © 2025 faml
---

<!--
npm run docs:build
npm run docs:dev
-->
