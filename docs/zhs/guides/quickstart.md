# 快速开始

## 命令行

```bash
# 零配置：自动发现 Chrome/Chromium/Edge，否则拉取 Chrome for Testing。
shirabe debug --port 3001

# 锁定后端，并让浏览器走代理。
SHIRABE_BACKEND=chromium shirabe debug --port 3001 --proxy http://localhost:7890
```

随后通过 HTTP 驱动运行中的服务器：

```bash
curl -X POST http://localhost:3001/navigate \
  -H "Content-Type: application/json" -d '{"url":"https://example.com"}'

curl -X POST http://localhost:3000/screenshot -d '{}'
```

## 作为库

```rust
use shirabe::{start_debug_server, DebugServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = DebugServerConfig {
        base_url: "about:blank".to_string(),
        dev_port: 0,
        dist_dir: String::new(),
        package_name: String::new(),
        proxy: None,
    };
    start_debug_server(cfg, 3001).await
}
```

可执行文件的发现顺序见[后端与解析](./backends.md)，打包自带原生库的方法见
[打包原生库](./bundling.md)。
