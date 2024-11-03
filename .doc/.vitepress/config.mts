import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Ziafp",
  description: "Regedit for self-triggering",
  head: [['link', { rel: 'icon', href: '/favicon.ico' }]],
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/getting-started' }
    ],

    sidebar: [
      {
        text: 'Getting Started',
        items: [
          { text: '安装', link: '/getting-started' },
          { text: '启动', link: '/run' },
          { text: '文件复制', link: '/cfth' },
          { text: '文档生成', link: '/doc' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/initialencounter/Ziafp' }
    ]
  }
})
