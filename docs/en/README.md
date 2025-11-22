---
home: true
title: Home
#heroImage: https://vuejs.press/images/hero.png
actions:
  - text: Getting Started
    link: /en/guide/01_hello_world.html
    type: primary
  - text: GitHub
    link: https://github.com/fawdlstty/faml
    type: secondary

features:
  - title: TOML-Style Syntax
    details: Compatible with TOML syntax style, using square brackets to define configuration blocks, assignment with equals sign, maintaining high readability and ease of use, with a low learning threshold
  - title: Dynamic Expression Evaluation
    details: Supports embedding expressions in configurations, allowing references to other configuration items, arithmetic operations, logical judgments, and other operations to achieve truly dynamic configuration
  - title: Conditional Configuration Blocks
    details: Control whether configuration items take effect based on conditions through the @if directive, facilitating configuration management in different environments and conditional configurations
  - title: Rich Data Types
    details: Built-in support for special data types such as duration, distance, and storage capacity, which can be used directly in configurations (such as 5 minutes, 100 MB)
  - title: Runtime Dynamic Modification
    details: Configurations can be dynamically modified by programs at runtime and automatically recalculate dependent items, achieving true dynamic configuration updates
  - title: Intelligent Computation Tracing
    details: Provides powerful trace functionality to track each step of complex expression computations, quickly locating the root cause of configuration calculation errors

footer: MIT Licensed | Copyright © 2025 faml
---