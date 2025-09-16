

# 安装wasm打包工具
```shell
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    //编译当前的代码
    wasm-pack build --target web --out-dir pkg
```
# WASM用法(直接导入rust pub函数即可)
```js
 import init, { transform_point, JsPoint } from './pkg/GIS_Wasm_Algorithm.js';
        let wasm;
        async function run() {
            wasm = await init();
            console.log('WASM 模块加载成功')
            // 使全局可用
            window.transform_point = transform_point;
            window.JsPoint = JsPoint;
        }
```

