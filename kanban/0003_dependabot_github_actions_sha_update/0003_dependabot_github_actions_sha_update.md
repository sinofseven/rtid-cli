# Dependabot による GitHub Actions の SHA ピン留め更新の管理

## 目的
今ピン留めまではできているが、その先の更新の追随ができないため

## 要望
dependabotでGithub Actionsのshaハッシュのピン留めしたものの更新も管理してください

## プラン

### Context
`.github/workflows/*.yml` 内の GitHub Actions は既に SHA でピン留めされている（例: `actions/checkout@df4cb1c069e1874edd31b4311f1884172cec0e10 # v6`）。しかし `.github/dependabot.yml` には `cargo` エコシステムのみが登録されており、`github-actions` エコシステムが存在しないため、ピン留めされた SHA の新バージョンへの追随（更新PRの自動作成）が行われていない。

Dependabot は `package-ecosystem: "github-actions"` を設定すると、SHA ピン留め形式（`uses: owner/repo@<sha> # vX`）のワークフローも検出し、SHA と末尾のバージョンコメントの両方を更新した PR を自動作成できる。

### 変更内容
`.github/dependabot.yml` に `github-actions` エコシステムのエントリを追加する。

```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
```

- `directory: "/"` は github-actions エコシステムの規約上、`.github/workflows/` 配下を自動的に対象とするための指定（cargo と同じ書き方に合わせる）。
- `schedule.interval: "weekly"` も既存の cargo 設定に合わせて統一する。

対象ファイルはリポジトリ内に2つ存在する（`.github/workflows/build.yml`, `.github/workflows/publish_formula.yml`）が、これらのファイル自体は変更しない。dependabot.yml の設定変更のみで、両ワークフロー内の SHA ピン留めされた全アクションが Dependabot の監視対象になる。

### 検証方法
- YAML の構文が正しいことを確認する（`python3 -c "import yaml; yaml.safe_load(open('.github/dependabot.yml'))"` などで構文チェック）
- 実際の Dependabot 実行はGitHub側のスケジュール実行に依存するため、ローカルでの動作確認はできない。設定ファイルの妥当性確認までとする。

## 完了サマリー

- 完了日時: 2026-09-04T15:03:54+09:00
- `.github/dependabot.yml` に `package-ecosystem: "github-actions"`（`directory: "/"`, `interval: "weekly"`）のエントリを追加し、`.github/workflows/build.yml` と `.github/workflows/publish_formula.yml` 内の SHA ピン留めされた GitHub Actions が Dependabot の更新監視対象になるようにした。
- YAML 構文の妥当性を `python3` + `pyyaml` で検証済み。
- 詳細は `log.md` を参照。
