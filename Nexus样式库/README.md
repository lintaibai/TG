---
outline: Nexus样式库
title: Nexus样式库
author: 林太白
---

# Nexus样式库
## 1、介绍
```JS
项目仓库
  【GIthub地址】 https://github.com/lintaibai/TG
  【Gitee地址】 https://gitee.com/lintaibai/TG

介绍
  轻量手写Tailwindcss样式库，目的是支持旧项目的Vue2、Vue3、React、小程序等
```



### 👉样式库介绍

| 样式名称 |  内容 |  备注|
|----------|----------------|-------------|
| nexus.css | nexus仿tailwindcss|不包含任何尺寸信息|
| nexuspx.css| px尺寸 | 结合使用 |
| nexusrem.css |rem(rem尺寸)|font-size固定100px |
| nexusrpx.css | 10px =  20rpx | uniapp使用 |
| nexusmodule.css | 模块化css  | 群友贡献精美模块 |

### 👉样式

| 模块             | 样式      | 👉含义  |
|-----------|-------|-------|
|    font字体 |     |  |  |
|  | font-family-pingfang | font-family: PingFangSC, "PingFang SC"; |
|    fontw字体宽度 |     |  |  |
|    |    fontw100      | font-weight: 100; |  |
|          |    fontw200      | font-weight: 200; |  |
|  | fontw400  | font-weight: 400;  |
|          |    fontw430      | font-weight: 430; |  |
|          |    fontw600      | font-weight: 600; |  |
|          |    fontw700      | font-weight: 700; |  |
|          |    fontw900      | font-weight: 900; |  |
| 文字省略 | txtellipsis | white-space: nowrap; overflow: hidden; text-overflow: ellipsis; |
|  | txtellipsis1 | white-space: nowrap; overflow: hidden; text-overflow: ellipsis; |
|  | txtellipsis2 | text-overflow: -o-ellipsis-lastline; overflow: hidden; text-overflow: ellipsis; display: -webkit-box; -webkit-line-clamp: 2; line-clamp: 2; -webkit-box-orient: vertical; |
|  | txtellipsis3 | text-overflow: -o-ellipsis-lastline; overflow: hidden; text-overflow: ellipsis; display: -webkit-box; -webkit-line-clamp: 3; line-clamp: 3; -webkit-box-orient: vertical; |
| 字体换行 | whitespace-pre-wrap | white-space: pre-wrap; |
| 文字排列 | text-center | text-align: center; |
|  | text-left | text-align: left; |
|  | text-right | text-align: right; |
| 宽高 | w-onehalf | width: 50%; |
|  | w-auto | width: auto; |
|  | w-full | width: 100%; |
|  | h-full | height: 100%; |
|  | h-screen | height: 100vh; |
| 背景色 | bg-white | background: #fff; |
| 层级 | z-fu1 | z-index: -1; |
|  | z-0 | z-index: 0; |
|  | z-1 | z-index: 1; |
|  | z-2 | z-index: 2; |
|  | z-3 | z-index: 3; |
|  | z-4 | z-index: 4; |
|  | z-5 | z-index: 5; |
|  | z-6 | z-index: 6; |
|  | z-7 | z-index: 7; |
|  | z-8 | z-index: 8; |
|  | z-9 | z-index: 9; |
| 布局 | txt-flex-center | display: flex; justify-content: center; align-items: center; |
|  | absolutecenter | position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%); |
|  | flex | display: flex; |
|  | flex-nowrap | flex-wrap: nowrap; |
|  | flex-wrap | flex-wrap: wrap; |
|  | justify-between | justify-content: space-between; |
|  | justify-center | justify-content: center; |
|  | justify-start | justify-content: flex-start; |
|  | justify-around | justify-content: space-around; |
| 滑动 | overflow-hidden | overflow: hidden; |
|  | overflow-x-hidden | overflow-x: hidden; |
|  | overflow-y-hidden | overflow-y: hidden; |
|  | overflow-y-auto | overflow-y: auto; |
|  | overflow-x-auto | overflow-x: auto; |






