# shirabe

**重新设计的浏览器自动化——通过 CDP 驱动无头 Chromium 家族，附带 ort 式的零配置后端解析器。**

shirabe 是一个轻量、Rust 原生的浏览器自动化库与调试服务器。它通过一套手写的
CDP 引擎驱动任何会说 Chrome DevTools Protocol 的浏览器——Google Chrome、Chromium、
Microsoft Edge——并以一套精简的 HTTP API 对外暴露。它是从 tairitsu 打包器中剥离、
独立强化而来的浏览器底座。

其指导思想与 [ort](https://crates.io/crates/ort) 对待 ONNX Runtime 一致：**你不应
该需要手动安装浏览器。** 一份锁定的 Chrome for Testing 构建会被拉取到共享缓存
（构建时或首次使用时），并透明地被定位与驱动。锁定不同的后端、随产品携带原生库、
经由镜像或代理下载——全部通过环境变量完成。

详见根 [README](../../README.md) 的完整特性与 HTTP API 表。

> 仍在开发中，API 未来可能调整。
