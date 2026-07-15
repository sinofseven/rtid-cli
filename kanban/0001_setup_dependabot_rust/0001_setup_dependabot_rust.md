# Dependabot による Rust 依存ライブラリ管理

## 目的
長いこと放置していたリポジトリだし、今後は依存ライブラリの管理をちゃんとしたい

## 要望
GihtubのDependabotでRustの依存ライブラリを管理したい

## プラン
- `.github/dependabot.yml` を新規作成する
  - `version: 2`
  - `updates:` に `package-ecosystem: "cargo"` のエントリを追加
    - `directory: "/"`（リポジトリルートに Cargo.toml があるため）
    - `schedule.interval: "weekly"`（放置気味だったリポジトリのため、通知過多を避けて weekly を採用）
  - GitHub Actions ワークフローは存在しないため github-actions エコシステムの設定は追加しない

## 完了サマリー
- 完了日時: 2026-07-15T18:26:46+09:00
- `.github/dependabot.yml` を新規作成し、Cargo エコシステムの依存関係更新を weekly 頻度で監視する設定を追加した。
- 実際の Dependabot ジョブ認識は GitHub 側（Insights > Dependency graph > Dependabot）でのみ確認可能なため、GitHub にプッシュ後に確認すること。
- 詳細な調査内容・判断経緯は `kanban/0001_setup_dependabot_rust/log.md` を参照。
