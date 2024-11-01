import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Ziafp",
  description: "Regedit for self-triggering",
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
          { text: '启动', link: '/run' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/initialencounter/Ziafp' }
    ]
  }
})
