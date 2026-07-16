//! Live-browser screenshot tests.
//!
//! Unlike `tests/mcp_smoke.rs` (which only checks the MCP tool roster with the
//! browser disabled), these tests drive a **real** headless Chromium through
//! the debug server's HTTP API: navigate to a fixture page, screenshot it,
//! and pixel-diff against a committed baseline.
//!
//! # Running
//!
//! Marked `#[ignore]` by default because they need a real browser on the host.
//! The dedicated `live-browser` CI job installs Chromium via
//! `browser-actions/setup-chrome` and sets `CHROME_PATH` before invoking:
//!
//! ```bash
//! cargo test --features mcp --test live_browser -- --ignored --test-threads=1
//! ```
//!
//! Locally, make sure a Chromium-family browser is on `$PATH` (or set
//! `CHROME_PATH`) and `SHIRABE_SKIP_BROWSER_FETCH=1`, then run the same
//! command. Bless a new baseline with `SHIRABE_ACCEPT_SNAPSHOTS=1`.
//!
//! # Why feature `mcp`
//!
//! Gated on `mcp` so the test binary reuses the axum + reqwest + tokio stack
//! that the MCP server already pulls in — no extra dev-deps for HTTP plumbing.

#![cfg(feature = "mcp")]

mod common;

use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

/// A minimal HTML page with deterministic content: a fixed-size coloured box
/// and a heading. Kept inline so the test has no external file dependency.
const FIXTURE_HTML: &str = r#"<!doctype html>
<html><head><meta charset="utf-8"><style>
  body { margin: 0; padding: 0; background: #ffffff; }
  .box { width: 200px; height: 100px; background: #2d7d46; }
  h1 { font-family: monospace; color: #1a1a1a; padding: 8px; }
</style></head>
<body>
  <div class="box"></div>
  <h1>shirabe live-browser snapshot</h1>
</body></html>"#;

/// Boot `shirabe debug` on a free port, drive it through the debug API, and
/// snapshot the rendered page. Validates the whole CDP → screenshot → PNG
/// pipeline end to end.
#[tokio::test]
#[ignore = "needs a real browser; run via the live-browser CI job or locally with CHROME_PATH set"]
async fn renders_fixture_page() -> Result<()> {
    let port = pick_free_port();
    let url = serve_fixture_html().await?;

    let mut server = spawn_debug_server(port).await?;
    let result = async {
        wait_for_browser(port, Duration::from_secs(45)).await?;
        // Navigate to the fixture page we served, then screenshot.
        navigate(port, &url).await?;
        let screenshot = screenshot(port).await?;
        common::compare_browser_png(&screenshot, "live_browser_fixture_page");
        Ok::<(), anyhow::Error>(())
    }
    .await;

    // Always kill the server, even on assertion failure.
    let _ = server.kill().await;
    result?;
    Ok(())
}

/// Bind a transient TCP listener to grab a free loopback port for the debug
/// server. We don't keep the listener — the OS will reissue the port quickly
/// enough for a CI run.
fn pick_free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .expect("bind free port")
        .local_addr()
        .expect("local addr")
        .port()
}

/// Serve `FIXTURE_HTML` on a throwaway HTTP server and return its URL.
/// Uses a plain `std::net::TcpListener` + a hand-rolled HTTP/1.0 response —
/// avoids pulling in a web framework just to ship one static page.
async fn serve_fixture_html() -> Result<String> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    let url = format!("http://127.0.0.1:{port}/");
    // Make the socket non-blocking so accept() can be polled from async land.
    listener.set_nonblocking(true)?;
    tokio::spawn(async move {
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let body = FIXTURE_HTML;
                    let resp = format!(
                        "HTTP/1.0 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(), body
                    );
                    use std::io::Write;
                    let _ = stream.write_all(resp.as_bytes());
                    let _ = stream.flush();
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                Err(_) => break,
            }
        }
    });
    Ok(url)
}

/// Spawn `shirabe debug --port <port> --url about:blank` as a child process.
/// Inherits `SHIRABE_SKIP_BROWSER_FETCH` (set in CI) so the build stays
/// offline; the real browser comes from `CHROME_PATH` or `$PATH`.
/// `kill_on_drop` guarantees teardown even if the test panics.
async fn spawn_debug_server(port: u16) -> Result<tokio::process::Child> {
    let bin = env!("CARGO_BIN_EXE_shirabe");
    let child = tokio::process::Command::new(bin)
        .arg("debug")
        .arg("--port")
        .arg(port.to_string())
        .arg("--url")
        .arg("about:blank")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .context("spawn shirabe debug")?;
    Ok(child)
}

/// Poll `GET /health` until `browser_connected` becomes true or the deadline
/// hits. A 503 here just means the browser hasn't finished booting yet.
async fn wait_for_browser(port: u16, timeout: Duration) -> Result<()> {
    let deadline = Instant::now() + timeout;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    loop {
        if Instant::now() >= deadline {
            bail!("browser did not connect within {:?}", timeout);
        }
        if let Ok(resp) = client
            .get(format!("http://127.0.0.1:{port}/health"))
            .send()
            .await
        {
            if resp.status().is_success() {
                let body: serde_json::Value = resp.json().await.unwrap_or_default();
                if body
                    .get("status")
                    .and_then(|s| s.as_str())
                    .map(|s| s.eq_ignore_ascii_case("ok") || s.eq_ignore_ascii_case("ready"))
                    .unwrap_or(false)
                {
                    return Ok(());
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

async fn navigate(port: u16, url: &str) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;
    let resp = client
        .post(format!("http://127.0.0.1:{port}/navigate"))
        .json(&serde_json::json!({ "url": url }))
        .send()
        .await?;
    if !resp.status().is_success() {
        bail!("navigate failed: HTTP {}", resp.status());
    }
    Ok(())
}

/// `POST /screenshot` and return the base64 PNG string from the response.
async fn screenshot(port: u16) -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;
    let resp = client
        .post(format!("http://127.0.0.1:{port}/screenshot"))
        .json(&serde_json::json!({ "full_page": true }))
        .send()
        .await?;
    if !resp.status().is_success() {
        bail!("screenshot failed: HTTP {}", resp.status());
    }
    let body: serde_json::Value = resp.json().await?;
    let data = body
        .get("data")
        .and_then(|d| d.as_str())
        .context("response missing `data` field")?
        .to_string();
    Ok(data)
}
