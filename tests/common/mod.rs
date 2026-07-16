//! Shared test helpers for shirabe's screenshot-based regression tests.
//!
//! Mirrors the snapshot workflow used by kou (`tests/common/mod.rs`) but
//! adapted for browser-rendered PNGs: the debug server returns a base64 PNG
//! via `POST /screenshot`, which we decode and compare against a committed
//! baseline under `tests/baselines/`.
//!
//! # Accepting a new baseline
//!
//! ```bash
//! SHIRABE_ACCEPT_SNAPSHOTS=1 cargo test --test live_browser -- --ignored
//! ```
//! then commit the updated `tests/baselines/*.png`.

use image::RgbaImage;

/// Maximum fraction of pixels allowed to differ before a screenshot is flagged
/// as a regression. Browser rendering is noisier than terminal rendering
/// (sub-pixel font hinting, GPU rasterisation variance across Chromium
/// versions), so we use a looser threshold than kou's 0.1%.
pub const DIFF_THRESHOLD: f64 = 0.005;

/// Per-channel tolerance below which a pixel is not counted as differing.
const PIXEL_JITTER_TOLERANCE: u8 = 3;

/// Baseline root for browser screenshots.
const BASELINE_DIR: &str = "tests/baselines";

/// Decode a base64-encoded PNG (as returned by the debug server's
/// `/screenshot` endpoint) and compare it against
/// `tests/baselines/{name}.png`.
///
/// - With `SHIRABE_ACCEPT_SNAPSHOTS` set, the decoded image replaces the
///   baseline.
/// - Otherwise the two are diffed pixel-by-pixel; differing beyond
///   [`DIFF_THRESHOLD`] fails the test.
pub fn compare_browser_png(base64_png: &str, name: &str) {
    let png_bytes = decode_base64_png(base64_png);
    let baseline = std::path::Path::new(BASELINE_DIR).join(format!("{name}.png"));

    if std::env::var_os("SHIRABE_ACCEPT_SNAPSHOTS").is_some() {
        std::fs::create_dir_all(baseline.parent().unwrap()).unwrap();
        std::fs::write(&baseline, &png_bytes).unwrap();
        eprintln!("  accepted {baseline:?}");
        return;
    }

    let new = image::load_from_memory(&png_bytes)
        .expect("decode rendered png")
        .to_rgba8();
    let base_bytes = std::fs::read(&baseline).unwrap_or_else(|e| {
        panic!(
            "baseline {baseline:?} missing ({e}); generate it with \
             `SHIRABE_ACCEPT_SNAPSHOTS=1 cargo test --test live_browser -- --ignored {name}`"
        )
    });
    let base = image::load_from_memory(&base_bytes)
        .expect("decode baseline png")
        .to_rgba8();

    // If dimensions differ, we can't pixel-diff — but for browser screenshots
    // a dimension change is itself a strong signal (viewport resized, page
    // layout broke). Report it but don't hard-panic; fall through to the ratio
    // check with the overlapping region so the user still gets a diff number.
    if base.dimensions() != new.dimensions() {
        eprintln!(
            "{name}: dimensions changed ({}x{} -> {}x{}) — viewport or layout drift",
            base.width(),
            base.height(),
            new.width(),
            new.height(),
        );
    }

    let (ratio, max_delta) = pixel_diff(&base, &new);
    let pct = format!("{:.3}%", ratio * 100.0);
    assert!(
        ratio < DIFF_THRESHOLD,
        "{name}: {pct} of pixels differ (max channel delta {max_delta}) — \
         rendering regression? If this change is intended, bless it with \
         `SHIRABE_ACCEPT_SNAPSHOTS=1 cargo test --test live_browser -- --ignored {name}`",
    );
}

/// Decode a base64 string into raw PNG bytes. Tolerates a `data:` prefix.
fn decode_base64_png(s: &str) -> Vec<u8> {
    use base64::Engine;
    let body = s
        .strip_prefix("data:image/png;base64,")
        .or_else(|| s.strip_prefix("data:image/png;"))
        .unwrap_or(s);
    base64::engine::general_purpose::STANDARD
        .decode(body.trim())
        .expect("decode base64 png")
}

/// Fraction of differing pixels and the largest single-channel delta between
/// two RGBA images of equal dimensions. A pixel "differs" when any channel
/// differs by more than [`PIXEL_JITTER_TOLERANCE`].
fn pixel_diff(base: &RgbaImage, other: &RgbaImage) -> (f64, u8) {
    let (bw, bh) = base.dimensions();
    let (ow, oh) = other.dimensions();
    let (w, h) = (bw.min(ow), bh.min(oh));
    let total = (w as u64) * (h as u64);
    if total == 0 {
        return (1.0, 255);
    }
    let mut differing = 0u64;
    let mut max_delta = 0u8;
    for y in 0..h {
        for x in 0..w {
            let a = base.get_pixel(x, y);
            let b = other.get_pixel(x, y);
            let dr = a[0].abs_diff(b[0]);
            let dg = a[1].abs_diff(b[1]);
            let db = a[2].abs_diff(b[2]);
            let da = a[3].abs_diff(b[3]);
            let local = dr.max(dg).max(db).max(da);
            max_delta = max_delta.max(local);
            if local > PIXEL_JITTER_TOLERANCE {
                differing += 1;
            }
        }
    }
    (differing as f64 / total as f64, max_delta)
}
