# shirabe

**ブラウザ自動化の再設計 —— CDP によるヘッドレス Chromium 系ブラウザの制御と、ort 風のゼロ設定バックエンドリゾルバ。**

shirabe は軽量な Rust ネイティブのブラウザ自動化ライブラリ兼デバッグサーバーです。
Chrome DevTools Protocol を話す任意のブラウザ（Google Chrome・Chromium・Microsoft Edge）
を一つの手書き CDP エンジンで駆動し、小さな HTTP API で公開します。tairitsu パッケージャ
から切り離して単独で強化したブラウザ基盤です。

指导思想は [ort](https://crates.io/crates/ort) が ONNX Runtime に採るものと同じです。
**ブラウザを手作業でインストールする必要はありません。** ピン留めされた Chrome for
Testing ビルドがビルド時（または初回利用時）に共有キャッシュへ取得され、透過的に
特定・駆動されます。バックエンドの切り替え、ネイティブライブラリの同梱、ミラーや
プロキシ経由のダウンロードは、すべて環境変数で行えます。

完全な機能と HTTP API 表はルート [README](../../README.md) を参照してください。

> 開発中であり、API は今後変更される可能性があります。
