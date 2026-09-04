# CHANGELOG.md の追加

## 目的
change logをファイルとして残していなかったので残すようにしたい

## 要望
"Keep a Changelog" 形式でv0.2.1用のCHANGELOG.mdを書いてください

## プラン

### Context
このリポジトリにはこれまで変更履歴をファイルとして残していなかった。今後は "Keep a Changelog" (https://keepachangelog.com/) 形式で `CHANGELOG.md` を残していきたい、という要望。今回は v0.2.1 分の内容を中心に記述する。

### 調査結果
- `git tag -l` → `0.1.0`, `v0.2.0` の2タグのみ存在。v0.2.1 のタグはまだ無い。
- `Cargo.toml` の `version` は現在 `0.2.0`（v0.2.1 へのバージョン更新はまだされていない）。
- `v0.2.0..HEAD` のコミット（＝v0.2.1相当の未リリース分）:
  - `81fd955` READMEにHomebrewでのインストール手順を追加 (2026-07-16)
  - `b3c6bcf` Homebrew Formula の name を rtid に変更し display_name を追加 (2026-07-16)
  - `30aac77` Bump clap from 4.6.1 to 4.6.6 (2026-08-12)
  - `50c88b1` Merge pull request #4（clap bump の取り込みマージ）
  - `e4865ff` Dependabot に github-actions エコシステムを追加 (2026-09-04・最新コミット)
- `0.1.0..v0.2.0` のコミット（v0.2.0相当）:
  - `a4b1007` Dependabot による Rust 依存ライブラリ管理を追加
  - `c1ea322` Bump clap from 2.33.3 to 4.6.1
  - `9ba7eef` clap v4のderiveスタイルへ移行
  - `3362904` GitHub Actions によるビルド・リリースワークフローを追加
  - `3156440` Homebrew Formula の自動公開ワークフローを追加
  - `c74c96a` バージョンを 0.2.0 に更新
- `CHANGELOG.md` は現状リポジトリに存在しない（新規作成）。

### 方針
- Keep a Changelog の標準フォーマット（`## [x.y.z] - YYYY-MM-DD` 見出し、`### Added` / `### Changed` などのカテゴリ分け）に従う。
- 依頼は「v0.2.1用」だが、Keep a Changelog は本来「初回リリースからの全履歴」を残す形式のため、新規作成の機会に v0.1.0・v0.2.0 も含めた完全な履歴として書く。
- v0.2.1 は `Cargo.toml` 上まだ `0.2.0` のままでリリース前のため、日付は最新コミット日 `2026-09-04` を暫定的に採用する。
- バージョン番号のバンプ（`Cargo.toml` の更新）やタグ付けは今回のタスク範囲外。CHANGELOG.md の作成のみ行う。

### 実装ステップ
1. リポジトリルートに `CHANGELOG.md` を新規作成する。
2. 内容に誤り・過不足がないか、コミットログと突き合わせて最終確認する。

## 完了サマリー

完了日時: 2026-09-04T15:07:57+09:00

リポジトリルートに `CHANGELOG.md` を新規作成した。Keep a Changelog 形式に従い、v0.2.1（未リリース分、`v0.2.0..HEAD` のコミットから作成）、v0.2.0、v0.1.0 の3バージョン分の変更履歴を記載した。詳細な調査結果・作業ログは `kanban/0004_add_changelog/log.md` を参照。
