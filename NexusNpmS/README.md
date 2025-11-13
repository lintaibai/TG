
# NexusNpmS单组件部分

## Tbcode验证码组件

```JS
author:林太白
linlele 开源UI框架
```

## 使用

安装依赖
```JS
npm i tbcode
```

项目引入使用
```JS

方式一

import tbCode from 'tbCode'
app.component('Tbcode', tbcode) 
import 'tbcode/dist/style.css'
<Tbcode/>




方式二 推荐

import Tbcode from 'tbcode';
import 'tbcode/dist/style.css'
app.component('Tbcode', Tbcode) 
<Tbcode/>
```

