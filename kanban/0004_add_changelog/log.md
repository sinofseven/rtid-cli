# タスク0004 ログ: CHANGELOG.md の新規作成

開始日時: 2026-09-04T15:07:11+09:00

## タスク概要

"Keep a Changelog" 形式でv0.2.1用のCHANGELOG.mdを書いてください

目的: change logをファイルとして残していなかったので残すようにしたい

## 調査結果

- `git tag -l` を実行した結果、`0.1.0` と `v0.2.0` の2つのタグのみ存在することを確認した。v0.2.1 に相当するタグはまだ作成されていない。
- `Cargo.toml` の `[package]` セクションを確認したところ、`name = "rtid-cli"`、`version = "0.2.0"` であり、v0.2.1 へのバージョン更新はまだ行われていないことを確認した。
- `git log --oneline --no-merges -30` で直近のコミット履歴を確認した:
  ```
  e4865ff Dependabot に github-actions エコシステムを追加
  30aac77 Bump clap from 4.6.1 to 4.6.6
  b3c6bcf Homebrew Formula の name を rtid に変更し display_name を追加
  81fd955 READMEにHomebrewでのインストール手順を追加
  c74c96a バージョンを 0.2.0 に更新
  3156440 Homebrew Formula の自動公開ワークフローを追加
  3362904 GitHub Actions によるビルド・リリースワークフローを追加
  9ba7eef clap v4のderiveスタイルへ移行
  c1ea322 Bump clap from 2.33.3 to 4.6.1
  a4b1007 Dependabot による Rust 依存ライブラリ管理を追加
  f36cb91 fix readme
  0cec76b add embed slide
  03dbc63 impl
  ```
- `git log v0.2.0..HEAD --oneline` で v0.2.0 タグ以降（＝v0.2.1候補分）のコミットを確認した:
  ```
  e4865ff Dependabot に github-actions エコシステムを追加
  50c88b1 Merge pull request #4 from sinofseven/dependabot/cargo/clap-4.6.6
  30aac77 Bump clap from 4.6.1 to 4.6.6
  b3c6bcf Homebrew Formula の name を rtid に変更し display_name を追加
  81fd955 READMEにHomebrewでのインストール手順を追加
  ```
- `git log 0.1.0..v0.2.0 --oneline` で v0.1.0 から v0.2.0 までのコミットを確認した:
  ```
  c74c96a バージョンを 0.2.0 に更新
  3156440 Homebrew Formula の自動公開ワークフローを追加
  3362904 GitHub Actions によるビルド・リリースワークフローを追加
  7ba33ae Merge pull request #1 from sinofseven/dependabot/cargo/clap-4.6.1
  9ba7eef clap v4のderiveスタイルへ移行
  c1ea322 Bump clap from 2.33.3 to 4.6.1
  a4b1007 Dependabot による Rust 依存ライブラリ管理を追加
  ```
- 各コミットの日時を `git log -1 --format=%ai <hash>` で確認した:
  - `81fd955`: 2026-07-16 10:06:14 +0900
  - `b3c6bcf`: 2026-07-16 11:21:38 +0900
  - `30aac77`: 2026-08-12 12:13:57 +0000
  - `e4865ff`: 2026-09-04 15:04:39 +0900（HEAD、最新コミット）
  - `v0.2.0` タグ: 2026-07-16 09:53:45 +0900
  - `0.1.0` タグ: 2020-10-01 17:33:16 +0000
- `ls CHANGELOG.md` を実行したところ "No such file or directory" となり、CHANGELOG.md がリポジトリに存在しないことを確認した（新規作成であることを確認）。
- リポジトリ直下のファイル一覧（`ls`）: `about.hbs`, `about.toml`, `Cargo.lock`, `Cargo.toml`, `CREDITS`, `kanban`, `LICENSE`, `README.md`, `src`

## 実装プラン（完全版）

### Context
このリポジトリにはこれまで変更履歴をファイルとして残していなかった。今後は "Keep a Changelog" (https://keepachangelog.com/) 形式で `CHANGELOG.md` を残していきたい、という要望。今回は v0.2.1 分の内容を中心に記述する。

### 方針
- Keep a Changelog の標準フォーマット（`## [x.y.z] - YYYY-MM-DD` 見出し、`### Added` / `### Changed` などのカテゴリ分け）に従う。
- 依頼は「v0.2.1用」だが、Keep a Changelog は本来「初回リリースからの全履歴」を残す形式のため、新規作成の機会に v0.1.0・v0.2.0 も含めた完全な履歴として書く（後から見て一貫性のあるファイルにするため）。
- v0.2.1 は `Cargo.toml` 上まだ `0.2.0` のままでリリース前のため、日付は最新コミット日 `2026-09-04` を暫定的に採用する。
- バージョン番号のバンプ（`Cargo.toml` の更新）やタグ付けは今回のタスク範囲外のため行わない。CHANGELOG.md の作成のみ行う。

### 記載内容
```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.2.1] - 2026-09-04
### Added
- READMEにHomebrewでのインストール手順を追加
- Dependabot に github-actions エコシステムを追加

### Changed
- Homebrew Formula の name を `rtid` に変更し `display_name` を追加
- clap を 4.6.1 から 4.6.6 に更新

## [0.2.0] - 2026-07-16
### Added
- Dependabot による Rust 依存ライブラリ管理を追加
- GitHub Actions によるビルド・リリースワークフローを追加
- Homebrew Formula の自動公開ワークフローを追加

### Changed
- clap を 2.33.3 から 4.6.1 に更新し、derive スタイルへ移行

## [0.1.0] - 2020-10-01
### Added
- 初回リリース
```

### 実装ステップ
1. リポジトリルートに `CHANGELOG.md` を新規作成し、上記内容を記載する。
2. 内容に誤り・過不足がないか、コミットログと突き合わせて最終確認する。

### 検証
- `cat CHANGELOG.md` で内容を目視確認。
- Markdown の構文（見出しレベル、リンク）に誤りがないか確認。

## プランニング経緯

- 初回提案として上記の内容（v0.1.0・v0.2.0・v0.2.1 の3バージョンをKeep a Changelog形式で記載し、v0.2.1の日付は最新コミット日を暫定採用する案）を ExitPlanMode で提示した。
- ユーザーはリジェクトやフィードバックなく、この初回提案がそのまま承認された。

## 会話内容

- ユーザー（kanban-kit:add-kanban 経由）: "Keep a Changelog" 形式でv0.2.1用のCHANGELOG.mdを書いてください／目的: change logをファイルとして残していなかったので残すようにしたい、というタスクを作成。
- Claude: kanban ファイル `kanban/0004_add_changelog/0004_add_changelog.md` を作成し、`/kanban` 実行の要否をユーザーに確認。
- ユーザー: 「はい（すぐ実行する）」を選択。
- Claude: `kanban-kit:kanban` スキルを args="0004" で起動。
- Claude: EnterPlanMode でプランモードに入り、`git tag -l`、`Cargo.toml` の確認、`git log` による調査を実施。
- Claude: 調査結果をもとにプランを作成し、ExitPlanMode でユーザーに提示。
- ユーザー: プランを承認。
- Claude: フェーズ2（実装）に移行し、このログファイルを作成。

## 実装フェーズ

### 編集したファイル
- `CHANGELOG.md`（新規作成）: プラン通り Keep a Changelog 形式で v0.2.1 / v0.2.0 / v0.1.0 の3バージョン分の内容を記載した。
- `kanban/0004_add_changelog/0004_add_changelog.md`: `## プラン` セクションを追記した。

### 実行したコマンド
- `TZ=Asia/Tokyo date +"%Y-%m-%dT%H:%M:%S+09:00"`（ログの開始日時・完了日時取得）

### 判断・意思決定
- 依頼は「v0.2.1用」だったが、Keep a Changelog は本来プロジェクト全体の履歴を残す形式であるため、新規作成の機会に v0.1.0・v0.2.0 も含めた完全な履歴として記載する判断をした（プランで提示し承認済み）。
- `Cargo.toml` のバージョンはまだ `0.2.0` のままだが、CHANGELOG.md の作成のみがタスク範囲であり、バージョンバンプやタグ付けは行わなかった。
- v0.2.1 の日付は正式なタグが存在しないため、`v0.2.0..HEAD` の最新コミット日（2026-09-04）を暫定的に採用した。

### エラー・問題
- なし

完了日時: 2026-09-04T15:07:57+09:00
