import { defaultTheme } from '@vuepress/theme-default'
import { defineUserConfig } from 'vuepress'
import { webpackBundler } from '@vuepress/bundler-webpack'

export default defineUserConfig({
  lang: 'zh-CN',

  title: 'faml',
  description: '一款简单清晰动态配置语言',

  theme: defaultTheme({
    //logo: 'https://vuejs.press/images/hero.png',

    navbar: ['/', '/guide/01_hello_world', {
      text: 'GitHub',
      link: 'https://github.com/fawdlstty/faml'
    }],
    sidebar: {
      '/guide/': [
        "00_introduction", "01_hello_world", "02_structs_and_types",
        "03_expressions", "04_methods"
      ]
    }
  }),

  bundler: webpackBundler(),
})
