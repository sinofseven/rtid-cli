# タスク 0003: Dependabot による GitHub Actions の SHA ピン留め更新の管理

- 開始日時: 2026-09-04T15:02:53+09:00

## タスク概要

dependabotでGithub Actionsのshaハッシュのピン留めしたものの更新も管理してください

目的: 今ピン留めまではできているが、その先の更新の追随ができないため

## 調査結果

### `.github/dependabot.yml`

```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
```

`cargo` エコシステムのみが登録されており、`github-actions` エコシステムのエントリが存在しないことを確認した。これが「SHA ピン留めしたものの更新が追随できない」原因である。Dependabot は `package-ecosystem` ごとに独立した監視設定が必要で、github-actions 用のエントリがなければワークフローファイル内の `uses:` は一切監視対象にならない。

### `.github/workflows/build.yml`

全 206 行。以下のジョブで構成される: `build-linux-x86_64`, `build-linux-arm64`, `build-linux-arm`, `build-macos`, `build-windows-x86_64`, `third-party-licenses`, `packaging`。

使用されているアクションはすべて SHA でピン留めされ、末尾に `# vX` 形式のバージョンコメントが付与されている:
- `actions/checkout@df4cb1c069e1874edd31b4311f1884172cec0e10 # v6`（複数箇所で使用）
- `Swatinem/rust-cache@e18b497796c12c097a38f9edb9d0641fb99eee32 # v2`（複数箇所）
- `dtolnay/rust-toolchain@29eef336d9b2848a0b548edc03f92a220660cdb8 # stable`（複数箇所）
- `actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7`（複数箇所）
- `actions/download-artifact@3e5f45b2cfb9172054b4087a40e8e0b5a5461e7c # v8`（2箇所）
- `softprops/action-gh-release@b4309332981a82ec1c5618f44dd2e27cc8bfbfda # v3`（1箇所、packaging ジョブ）

`build-linux-arm` ジョブのみ Docker イメージ（`ghcr.io/rust-cross/rust-musl-cross:${{ env.TARGET }}`）を使った cargo build を実行しており、これは GitHub Actions の `uses:` 形式ではないため dependabot の github-actions エコシステムの対象外（docker エコシステムの対象になりうるが、今回のタスク範囲外）。

### `.github/workflows/publish_formula.yml`

全 45 行。`publish` ジョブ1つのみで、`release: published` トリガーで実行される。

使用されているアクション:
- `sinofseven/action-request-id-token@d98c556ca8df2d4d50b9e59255284ede00a770b1 # v1.0.1`
- `actions/create-github-app-token@bcd2ba49218906704ab6c1aa796996da409d3eb1 # v3`
- `sinofseven/action-workflow-dispatch@d14f3b51612cd9200658a52b8576981c4f96d6f6 # v1.0.0`

これらもすべて SHA ピン留め + バージョンコメント形式。

### kanban 過去タスクの確認

`kanban/0001_setup_dependabot_rust/` が既に存在し、cargo エコシステムの dependabot 設定を追加したタスクであったと推測される（ファイル内容は読んでいないが、ディレクトリ名から把握）。今回のタスクはその延長で github-actions エコシステムを追加するもの。

### Dependabot の github-actions エコシステムの仕様（既知の知識）

`package-ecosystem: "github-actions"` を dependabot.yml に設定すると、`.github/workflows/` 配下の YAML ファイルを走査し、`uses:` で参照されているアクションのバージョンを監視する。SHA ピン留め形式（`uses: owner/repo@<full-sha>`）であっても、末尾に `# vX` のようなバージョンコメントがあれば、Dependabot はそのコメートを手がかりに新バージョンを検出し、SHA とコメントの両方を更新する PR を自動作成する。これは cargo のときと同様、`directory: "/"` を指定するだけで動作する（github-actions エコシステムはリポジトリルートを指定するのが慣例で、`.github/workflows/` は自動的にスキャンされる）。

## 実装プラン（プランモードで承認された内容）

### Context
`.github/workflows/*.yml` 内の GitHub Actions は既に SHA でピン留めされている（例: `actions/checkout@df4cb1c069e1874edd31b4311f1884172cec0e10 # v6`）。しかし `.github/dependabot.yml` には `cargo` エコシステムのみが登録されており、`github-actions` エコシステムが存在しないため、ピン留めされた SHA の新バージョンへの追随（更新PRの自動作成）が行われていない。これが kanban タスク `0003` の「目的」に記載された課題。

Dependabot は `package-ecosystem: "github-actions"` を設定すると、SHA ピン留め形式（`uses: owner/repo@<sha> # vX`）のワークフローも検出し、SHA と末尾のバージョンコメントの両方を更新した PR を自動作成できる。これは既存の cargo 用の設定パターン踏襲で追加できる。

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

### 完了後の作業
- kanban ファイルに `## プラン` セクションとしてこの内容を記載する（完了）
- ログファイルを作成する（このファイル）
- git commit は行わない（ユーザーの明示的な指示がある場合のみ）

## プランニング経緯

初回提案としてこのプラン（`.github/dependabot.yml` に `github-actions` エコシステムのエントリを1つ追加する）を ExitPlanMode で提示した。ユーザーからのリジェクトはなく、初回提案がそのまま承認された。

## 会話内容

1. ユーザーが `/kanban-kit:add-kanban` を実行し、「dependabotでGithub Actionsのshaハッシュのピン留めしたものの更新も管理してください」（要望）「今ピン留めまではできているが、その先の更新の追随ができないため」（目的）を指定した。
2. add-kanban スキルにより `kanban/0003_dependabot_github_actions_sha_update/0003_dependabot_github_actions_sha_update.md` を作成した。
3. ユーザーに続けて `/kanban` を実行するか確認したところ、「はい（すぐ実行する）」が選択された。
4. `kanban` スキルを `args: "0003"` で起動した。
5. タスクファイルを確認し `## 完了サマリー` が存在しないことを確認、目的セクションが存在することを確認した。
6. EnterPlanMode でプランモードに入り、`.github/dependabot.yml`, `.github/workflows/build.yml`, `.github/workflows/publish_formula.yml` を調査した。
7. プランを `/Users/natsume.yuta/.claude/plans/calm-swimming-giraffe.md` に記述し、ExitPlanMode で提示した。
8. ユーザーが承認した。

## 実装フェーズ

### 編集したファイル
- `.github/dependabot.yml`: `updates` 配列に `package-ecosystem: "github-actions"`（`directory: "/"`, `schedule.interval: "weekly"`）のエントリを cargo エントリの後に追加した。

### 実行したコマンド
- `python3 -m pip install --quiet --break-system-packages pyyaml` で yaml モジュールをインストール
- `python3 -c "import yaml; print(yaml.safe_load(open('.github/dependabot.yml')))"` で構文検証を実施し、以下の出力を得て正常にパースできることを確認した:
  ```
  {'version': 2, 'updates': [{'package-ecosystem': 'cargo', 'directory': '/', 'schedule': {'interval': 'weekly'}}, {'package-ecosystem': 'github-actions', 'directory': '/', 'schedule': {'interval': 'weekly'}}]}
  ```

### 判断・意思決定
- cargo エントリと同じ `directory: "/"` / `interval: "weekly"` の形式に揃え、既存パターンを踏襲した。
- ワークフローファイル自体（`build.yml`, `publish_formula.yml`）は変更不要と判断し、変更しなかった。dependabot.yml の設定変更のみで両ファイル内の SHA ピン留めアクションが自動的に監視対象になるため。
- `build-linux-arm` ジョブで使われている Docker イメージ参照（`ghcr.io/rust-cross/rust-musl-cross:${{ env.TARGET }}`）は github-actions エコシステムの対象外（docker エコシステムの範疇）であり、今回のタスク（GitHub Actions の SHA ピン留め更新）のスコープ外と判断し対応しなかった。

### エラー・問題
- ローカル環境に `pyyaml` がインストールされていなかったため、`pip install --break-system-packages pyyaml` でインストールしてから検証した。この操作以外にエラーは発生しなかった。

## 完了日時
2026-09-04T15:03:54+09:00
