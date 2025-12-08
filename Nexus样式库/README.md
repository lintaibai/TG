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

### 👉`nexus.css`样式

| 类名             |    属性   | 样式  |👉含义 |
|-----------|-------|-------|-------|
|    `特有样式` |     |  | 自己拟定的类名，便捷使用 |
| **font字体样式** |  |  |
|  | font-family-pingfang | font-family: PingFangSC, "PingFang SC"; |
| **font字体宽度** |  |  |
|    |    fontw100 ,fontw200, fontw400 ,fontw430,fontw600,fontw700,fontw900  | font-weight: 100;以此类推 |  |
| 文字省略 | txtellipsis | white-space: nowrap; overflow: hidden; text-overflow: ellipsis; |单行省略，2行，3行省略 |
|  | txtellipsis1 | white-space: nowrap; overflow: hidden; text-overflow: ellipsis; |
|  | txtellipsis2 | text-overflow: -o-ellipsis-lastline; overflow: hidden; text-overflow: ellipsis; display: -webkit-box; -webkit-line-clamp: 2; line-clamp: 2; -webkit-box-orient: vertical; |
|  | txtellipsis3 | text-overflow: -o-ellipsis-lastline; overflow: hidden; text-overflow: ellipsis; display: -webkit-box; -webkit-line-clamp: 3; line-clamp: 3; -webkit-box-orient: vertical; |
| 布局 | txt-flex-center | display: flex; justify-content: center; align-items: center; |
|  | absolutecenter | position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%); |
| 层级 | z-fu1 | z-index: -1; |
| 滚动 | .hide-scrollbar-y | overflow-y: auto;scrollbar-width: none;-ms-overflow-style: none;::-webkit-scrollbar { display: none; }	|启用垂直滚动并隐藏滚动条|
|  |  | ::-webkit-scrollbar { display: none; }	|针对 Webkit 浏览器隐藏滚动条|
|  | .hide-scrollbar-x	 | overflow-x: auto;scrollbar-width: none;-ms-overflow-style: none;|启用水平滚动并隐藏滚动条|
|  || ::-webkit-scrollbar { display: none; }	|针对 Webkit 浏览器隐藏滚动条|
|    跟tailwindcss保持一致样式 |     |  |  |
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
| **层级** |  |  |
|z-0,z-1,z-2 ,z-3,z-4,z-5,z-6,z-7,z-8,z-9| z-index | 0~9 |
| 布局 | flex | display: flex; |
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



### 👉`nexuspx.css`样式

| 类名             |    属性   | 样式  |👉含义 |
|-----------|-------|-------|-------|
|    特有样式 |     |  | 自己拟定的类名，便捷使用 |
|    font字体 |     |  |  |
|  | font08, font10, font13, font14, font16, font18, font20, font24, font26, font30, font32, font36, font44 | font-size: 8px; ... 以此类推 |


### 👉`nexusrem.css`样式

| 类名            |    属性   | 样式  |👉含义 |
|-----------|-------|-------|-------|
|    特有样式 |     |  |  |
| **font字体大小** |  |  |
| font08, font10, font13, font14, font16, font18, font20, font24, font26, font30, font32, font36, font44 | font-size | 0.08rem ~ 0.44rem | font-size: 0.08rem;
| **字体样式** |  |  |
| **行高** |  |  |
| lineht20, lineht24, lineht28, lineht30, lineht60 | line-height | 0.2rem ~ 0.6rem |
| **内边距** |  |  |
| pad10, pad20 | padding | 0.1rem, 0.2rem |
| padx20 | padding-left/right | 0.2rem |
| pady20 | padding-top/bottom | 0.2rem |
| padtop20 | padding-top | 0.2rem |
| padbot20 | padding-bottom | 0.2rem |
| padl20 | padding-left | 0.2rem |
| padr20 | padding-right | 0.2rem |
| **外边距** |  |  |
| mgbt10, mgbt20, mgbt40 | margin-bottom | 0.1rem, 0.2rem, 0.4rem |
| **圆角** |  |  |
| redius06, redius10, redius20 | border-radius | 0.06rem, 0.1rem, 0.2rem |
| **间距** |  |  |
| gap20 | gap | 0.2rem |
| gap-x-20 | gap | 0 0.2rem |
| gap-y-20 | gap | 0.2rem 0 |


### 👉`nexusrpx.css`样式

| 模块 (10px =  20rpx) |    类名   | 样式  |👉含义 |
|------|-------|-------|-------|
|    特有样式 |     |  | 自己拟定的类名，uniapp使用 |
|    font字体 |     |  |  |
|  | font08,font10,font13,font14,font16,font18,font20,font24,font26,font30,font44 | font-size: 16rpx; ... 以此类推 |


### 👉`nexusmodule.css`样式

| 模块 (10px =  20rpx)             |    类名   | 样式  |👉含义 |
|-----------|-------|-------|-------|
|    特有 |     |  | 类名模块 |
|  | .mod-xxx| 前缀名 （原生css,无less,无sass,无scss） |


