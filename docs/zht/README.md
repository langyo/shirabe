# shirabe

**重新設計的瀏覽器自動化——透過 CDP 驅動無頭 Chromium 家族，並附 ort 式的零設定後端解析器。**

shirabe 是一個輕量、Rust 原生的瀏覽器自動化函式庫與除錯伺服器。它透過一套手寫的
CDP 引擎驅動任何會說 Chrome DevTools Protocol 的瀏覽器——Google Chrome、Chromium、
Microsoft Edge——並以一套精簡的 HTTP API 對外暴露。它是從 tairitsu 打包器中剝離、
獨立強化而來的瀏覽器底座。

其指導思想與 [ort](https://crates.io/crates/ort) 對待 ONNX Runtime 一致：**你不應該
需要手動安裝瀏覽器。** 一份鎖定的 Chrome for Testing 建置會被拉取到共享快取（建置時
或首次使用時），並透明地被定位與驅動。

完整特性與 HTTP API 表請見根 [README](../../README.md)。

> 仍在開發中，API 未來可能調整。
