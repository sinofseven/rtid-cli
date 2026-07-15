# clap v4対応

## 目的
clapのバージョンを上げたらエラーが出た。この際、モダンに書き直してください

## 要望
clap v4.6.1に合わせて書き直して

## 完了サマリー

- 完了日時: 2026-07-15T18:50:24+09:00
- `Cargo.toml` の `clap` 依存に `features = ["derive"]` を追加
- `src/main.rs` を clap v2 ビルダースタイル（`App::new()` + `crate_authors!()` 等のマクロ）から clap v4 推奨の derive スタイル（`#[derive(Parser)]` + `#[command(author, version, about)]`）へ全面的に書き直し
- `cargo build` のエラーを解消し、`cargo run`／`--version`／`--help` の動作を確認済み
- 詳細は `kanban/0002_migrate_clap_v4/log.md` を参照
