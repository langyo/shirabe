# shirabe

<div align="center">

**Browser automation, reimagined.**

Headless Chrome control via CDP + HTTP API + MCP.
The Rust-native alternative to Playwright.

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](./LICENSE)

</div>

## Why shirabe?

| | Playwright | Puppeteer | **shirabe** |
|---|---|---|---|
| Language | Node.js | Node.js | **Rust** |
| Protocol | CDP | CDP | **CDP** |
| Binary size | ~100 MB | ~100 MB | **~20 MB** |
| Startup | ~2s | ~2s | **<1s** |
| MCP support | ❌ | ❌ | **✅** |
| HTTP API | ❌ | ❌ | **✅** |
| Anti-detection | Extension | Extension | **Built-in** |
| Memory | ~200 MB | ~200 MB | **~50 MB** |

## Quick Start

### CLI

```bash
# Zero-config: finds Chrome automatically
shirabe debug --port 3001

# With proxy
shirabe debug --port 3001 --proxy http://localhost:7890

# Then drive via HTTP API
curl -X POST http://localhost:3001/navigate \
  -H "Content-Type: application/json" \
  -d '{"url":"https://example.com"}'

curl -X POST http://localhost:3001/screenshot \
  -H "Content-Type: application/json" -d '{}'
```

### Library

```rust
use shirabe::{start_debug_server, DebugServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = DebugServerConfig {
        base_url: "about:blank".to_string(),
        proxy: Some("http://localhost:7890".to_string()),
    };
    start_debug_server(cfg, 3001).await?;
    Ok(())
}
```

### As seia dependency (browser search)

```bash
# seia embeds shirabe — zero external dependencies
seia search "rust async" --browser --browser-engine google
```

## HTTP API

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Server health |
| GET | `/info` | Browser status |
| POST | `/navigate` | Navigate to URL |
| POST | `/click` | Click element |
| POST | `/type` | Type text |
| POST | `/evaluate` | Execute JavaScript |
| POST | `/screenshot` | Capture screenshot |
| POST | `/wait-for-selector` | Wait for element |
| GET | `/dom` | Query DOM |
| GET | `/a11y` | Accessibility tree |
| GET | `/console` | Console logs |
| GET | `/network` | Network requests |
| POST | `/batch` | Batch operations |

Plus 10 more endpoints for full browser control.

## Features

- **Zero-config Chrome** — auto-discovers Chrome/Chromium/Playwright cache
- **CDP native** — hand-rolled Chrome DevTools Protocol, no chromiumoxide dependency
- **Anti-detection** — stealth JS injection (navigator.webdriver, window.chrome, plugins)
- **Proxy support** — pass `--proxy` for Chrome to route through your proxy
- **MCP ready** — exposes all browser tools via MCP protocol for AI agents
- **Container-friendly** — `TAIRITSU_SINGLE_PROCESS=1` for sandboxed environments

## License

SySL-1.0.
