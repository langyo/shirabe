# 后端与解析

shirabe 通过一套 CDP 引擎驱动任何会说 Chrome DevTools Protocol 的浏览器——
Google Chrome、Chromium、Microsoft Edge。用 `SHIRABE_BACKEND` 选择后端：

| 取值 | 后端 |
|------|------|
| `chrome`（`auto` 中首选） | Google Chrome |
| `chromium` | Chromium |
| `edge` | Microsoft Edge |
| `auto`（默认） | 依次尝试 Chrome、Chromium、Edge |

## 解析顺序

无论选择哪个后端，shirabe 都按如下顺序解析可执行文件（沿用
[ort](https://crates.io/crates/ort) 的依赖模型）：

1. **后端专属覆盖项** —— `CHROME_PATH` / `CHROMIUM_PATH` / `EDGE_PATH`。
   一旦设置即为权威；路径缺失视为硬错误。
2. **构建时烘焙路径** —— `SHIRABE_BROWSER_PATH`，由 `build.rs` 在开启 `auto-fetch`
   特性时把锁定版本的 Chrome for Testing 下载到共享缓存后写入。
3. **系统二进制** —— `$PATH` 上的可执行文件，以及若干常见安装位置
   （`/usr/bin/google-chrome`、`/Applications/Google Chrome.app/...`、
   `C:\Program Files\Google\Chrome\Application\chrome.exe` 等）。
4. **运行时拉取**（`runtime-fetch` 特性）—— 首次使用时把锁定版本下载进缓存。

## 下载开关

拉取步骤在构建时（`build.rs`）与运行时都遵循下列环境变量：

| 环境变量 | 用途 |
|----------|------|
| `SHIRABE_CHROME_VERSION` | 覆盖锁定的 Chrome for Testing 版本。 |
| `SHIRABE_CHROME_MIRROR` | 从镜像下载（例如对国内友好的镜像），取代默认的 Google 主机。 |
| `SHIRABE_CHROME_SHA256` | 可选的十六进制校验和；下载后据此校验。 |
| `SHIRABE_DOWNLOAD_PROXY` | 经由 `http://`、`https://` 或 `socks5://` 代理下载。 |
| `SHIRABE_DOWNLOAD_TIMEOUT_SECS` | 单次请求超时（默认 600）。 |
| `SHIRABE_SKIP_BROWSER_FETCH` | 跳过构建时与运行时的下载。 |

> 由于 `build.rs` 也读取这些变量，下游 crate 可以在 CI 中用一个 `env:` 块锁定整条
> 工具链。
