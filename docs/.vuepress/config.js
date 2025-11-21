import { defaultTheme } from '@vuepress/theme-default'
import { defineUserConfig } from 'vuepress'
import { webpackBundler } from '@vuepress/bundler-webpack'

export default defineUserConfig({
  locales: {
    '/': {
      lang: 'zh-CN',
      title: 'faml',
      description: '一款简单清晰动态配置语言'
    },
    '/en/': {
      lang: 'en-US',
      title: 'faml',
      description: 'A simple and clear dynamic configuration language'
    }
  },

  theme: defaultTheme({
    locales: {
      '/': {
        selectLanguageName: '简体中文',
        selectLanguageText: 'Languages',
        selectLanguageAriaLabel: 'Select language',
        //logo: 'https://vuejs.press/images/hero.png',
        navbar: [
          '/',
          '/guide/01_hello_world',
          {
            text: 'GitHub',
            link: 'https://github.com/fawdlstty/faml'
          }
        ],
        sidebar: {
          '/guide/': [
            "00_introduction", "01_hello_world", "02_structs_and_types",
            "03_expressions", "04_methods", "05_advanced_usage"
          ]
        }
      },
      '/en/': {
        selectLanguageName: 'English',
        selectLanguageText: 'Languages',
        selectLanguageAriaLabel: 'Select language',
        //logo: 'https://vuejs.press/images/hero.png',
        navbar: [
          '/en/',
          '/en/guide/01_hello_world.html',
          {
            text: 'GitHub',
            link: 'https://github.com/fawdlstty/faml'
          }
        ],
        sidebar: {
          '/en/guide/': [
            "00_introduction", "01_hello_world", "02_structs_and_types",
            "03_expressions", "04_methods", "05_advanced_usage"
          ]
        }
      }
    }
  }),

  bundler: webpackBundler(),
})