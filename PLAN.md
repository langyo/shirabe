# shirabe — 项目状态与计划 (PLAN)

> 刷新于 2026-07-14。无头浏览器自动化库。

## 1. 项目概述

- **名称**：`shirabe`
- **简介**：手写 CDP 客户端的无头浏览器自动化库，支持 Chrome / Chromium / Edge；不依赖 puppeteer / playwright，独立控制台消息与网络拦截。
- **远程仓库**：https://github.com/celestia-island/shirabe.git
- **技术栈**：Rust / tokio / tungstenite (raw WS) / serde_json / just
- **类别**：library（browser automation）

## 2. 当前状态

- **当前分支**：`dev`
- **工作区**：有未提交改动（4 项）
- **最近提交时间**：2026-07-12
- **最近提交**：`🔧 Pin script recipes to the resolved Git Bash to survive WSL shadowing.`
- **本地领先 `origin/dev`**：0

## 3. 未提交改动

```
 M .github/workflows/checks.yml
 M Cargo.toml
?? tests/common/
?? tests/live_browser.rs
```

## 4. 近期进展

- `🔧 Pin script recipes to the resolved Git Bash to survive WSL shadowing.`
- `🔧 Switch the justfile to Git Bash and fetch devtools recipes on demand.`
- `♻️ Standardize windows-shell to pwsh.exe across celestia repos.`
- `🐛 Replace shebang recipes with [script(...)] to fix the Windows cygpath error.`
- `Merge branch 'master' into dev`

## 5. 后续计划

1. **集成测试**：`tests/live_browser.rs` 端到端（需 CI 上跑 Chromium headless）。
2. **截图/录屏 API**：作为 shirabe 一等公民暴露，避免上游依赖 `headless_chrome` 的硬编码假设。
3. **iframe / shadow DOM 跨域**：CDP 对 `Page.frameNavigated` 的精细支持。

## 6. 跨仓依赖

- 被 entelecheia 的 Web Automation 域 agent（L2）作为底层调用。

---

## 既有详细计划（存档）

CDP 协议映射在 `docs/en/`。本文件只承载"当前态 → 后续计划"两部分。
