import{_ as s,a as i}from"./chunks/image-2.GH964e0H.js";import{_ as e}from"./chunks/image-8.mfPpsQ9A.js";import{_ as t,c as l,a0 as r,o as n}from"./chunks/framework.CGHvQLJz.js";const o="/assets/image.Bilu5kNq.png",p="/assets/image-3.CkzCOCAp.png",h="/assets/image-4.BJDSXjZ0.png",c="/assets/image-5.CB68kANB.png",d="/assets/image-6.DVfsUPQi.png",P=JSON.parse('{"title":"快速开始","description":"","frontmatter":{},"headers":[],"relativePath":"getting-started.md","filePath":"getting-started.md"}'),g={name:"getting-started.md"};function k(m,a,u,_,E,b){return n(),l("div",null,a[0]||(a[0]=[r('<h1 id="快速开始" tabindex="-1">快速开始 <a class="header-anchor" href="#快速开始" aria-label="Permalink to &quot;快速开始&quot;">​</a></h1><h2 id="功能特点" tabindex="-1">功能特点 <a class="header-anchor" href="#功能特点" aria-label="Permalink to &quot;功能特点&quot;">​</a></h2><ul><li>🚀 <strong>自动匹配文件</strong> - 根据文件夹名称和文件类型自动匹配文件。</li><li>📤 <strong>一键上传</strong> - 通过右键菜单快速上传匹配的文件。</li><li>💗 <strong>心跳机制</strong> - 自动保持登录状态，防止会话过期。</li><li>🔄 <strong>自启动</strong> - 支持开机自动启动服务。</li><li>🔒 <strong>安全可靠</strong> - 支持用户认证和权限验证。</li><li>📄 <strong>文档生成</strong> - 根据项目编号和名称自动生成 doc 文档。</li><li>📁 <strong>文件复制</strong> - 根据项目编号复制公共盘的资料到当前文件夹。</li></ul><h2 id="安装" tabindex="-1">安装 <a class="header-anchor" href="#安装" aria-label="Permalink to &quot;安装&quot;">​</a></h2><h3 id="下载" tabindex="-1">下载 <a class="header-anchor" href="#下载" aria-label="Permalink to &quot;下载&quot;">​</a></h3><h3 id="新建一个文件夹-用于存放程序和配置文件" tabindex="-1">新建一个文件夹，用于存放程序和配置文件 <a class="header-anchor" href="#新建一个文件夹-用于存放程序和配置文件" aria-label="Permalink to &quot;新建一个文件夹，用于存放程序和配置文件&quot;">​</a></h3><p>文件夹名称可以随便取，位置尽量放在方便找到的地方。</p><h3 id="前往-release-页面下载-server-exe-和-client-exe" tabindex="-1">前往 release 页面下载 server.exe 和 client.exe <a class="header-anchor" href="#前往-release-页面下载-server-exe-和-client-exe" aria-label="Permalink to &quot;前往 release 页面下载 server.exe 和 client.exe&quot;">​</a></h3><p><a href="https://github.com/initialencounter/Ziafp/releases/latest" target="_blank" rel="noreferrer">点我前往</a></p><p><img src="'+o+'" alt="下载页面"></p><p>如果被杀毒软件阻止，可以按照以下步骤进行操作:</p><ol><li>退出所有杀毒软件</li><li>保留程序 <img src="'+s+'" alt="保留程序步骤1"><img src="'+i+'" alt="保留程序步骤2"></li></ol><h3 id="创建配置文件" tabindex="-1">创建配置文件 <a class="header-anchor" href="#创建配置文件" aria-label="Permalink to &quot;创建配置文件&quot;">​</a></h3><ol><li><p>打开上面创建的文件夹</p></li><li><p>新建一个文本文件，命名为 local.env <img src="'+p+'" alt="创建配置文件步骤1"><img src="'+h+'" alt="创建配置文件步骤2"></p></li><li><p>选择打开方式为记事本 <img src="'+c+'" alt="打开方式步骤1"><img src="'+d+`" alt="打开方式步骤2"></p></li><li><p>写入下面的内容，并保存:</p></li></ol><div class="language-ini vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">ini</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;">BASE_URL</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">=系统的域名</span></span>
<span class="line"><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;">USER_NAME</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">=主检员的账号</span></span>
<span class="line"><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;">PASSWORD</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">=主检员的密码</span></span>
<span class="line"><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;">PORT</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">=25455</span></span>
<span class="line"><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;">LOG_ENABLED</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">=true</span></span></code></pre></div><p>系统的域名，主检员的账号，主检员的密码需要按实际情况来填写</p><h3 id="修改注册表" tabindex="-1">修改注册表 <a class="header-anchor" href="#修改注册表" aria-label="Permalink to &quot;修改注册表&quot;">​</a></h3><p>双击 client.exe , 在弹出的用户账户控制窗口中，点击“是”</p><p>如果你在文件管理器右键，看到了这个菜单，那么恭喜你完成了 ziafp 的安装 <img src="`+e+'" alt="安装完成"></p>',19)]))}const v=t(g,[["render",k]]);export{P as __pageData,v as default};
