# Doc 使用指南

本章节主要介绍 Doc 的安装与配置

## 介绍

`doc` 是一个 doc 文档生成工具，支持根据项目编号和名称自动生成 doc 文档。

## 安装

### 前置条件

- 需要安装 [server](getting-started.md)
- 需要安装 [doc-modifier](https://github.com/initialencounter/doc-modifier)
- 需要安装 python
- 需要安装 wps 或 office word

### 下载

1. 前往 [GitHub Releases 页面](https://github.com/initialencounter/Ziafp/releases/latest) 下载 `doc.exe`。
2. 如果下载过程中被杀毒软件阻止，请按照以下步骤操作：
   - 退出所有杀毒软件。
   - 保留程序。
     ![保留程序步骤1](image-1.png)
     ![保留程序步骤2](image-2.png)

### 安装 doc-modifier

1. 前往 [doc-modifier](https://github.com/initialencounter/Ziafp/blob/master/doc-modifier/main.py) 下载 `main.py`。
2. 安装 python，并安装python依赖。
```shell
pip install pywin32 fastapi pydantic uvicorn -i https://pypi.tuna.tsinghua.edu.cn/simple
```
3. 运行 `main.py`，启动服务。
```shell
python main.py
```

### 修改注册表

1. 双击 `doc.exe`，在弹出的用户账户控制窗口中，点击“是”。
2. 如果在文件管理器右键菜单中看到 `Edit doc`，那么恭喜你，Doc 的安装已完成。

### 准备 doc 模板

1. 打开 `doc.exe` 所在文件夹，将图片的模板放到这个文件夹，删除模板中的 `项目编号` 和 `项目名称` 这两行。

### 开始使用

1. 打开要编辑的文档文件夹，右键点击 `Edit doc` 即可在当前文件夹中生成 doc 文档。
   ![Edit doc](image-13.png)
