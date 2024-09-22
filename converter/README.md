# converter

## 概要

csvファイルの文字コードをshift-jisとutf-8の間で変換するコンバータです。

デフォルト動作はshift-jis -> utf-8への変換です。(GitHubにアップするマスタデータはshift-jisで、utf-8は変換して生成するという思想)

### フォルダ構成

``` PlainText
.
├── Cargo.lock   // 依存関係の実態（cargoで自動生成されるので手動編集禁止）
├── Cargo.toml   // 依存関係など
├── README.md    // 本ファイル
└── src
    └── main.rs  // converterのソースコード
```

### How to build

1. [Rust](https://www.rust-lang.org/)をインストールする。

2. `converter`に移動する。（current directoryを`converter`にする）

3. `cargo build`でビルドする。`Finished`と出ればビルド成功。ビルド生成物は`target`フォルダに出力される。

### Note

- コマンドラインから`converter.exe --help`と入力するとヘルプが表示される。

- コマンドラインから`converter.exe --inverse`と入力するとデフォルトの逆の変換（shift-jis -> utf-8）を実行する。

- 同一階層に`20XX`フォルダがないか、`20XX`フォルダがあっても`.csv`ファイルが無いと`NotFound`エラー（詳細は`No matching folders with CSV files found. Check location of this program.` ）を発出して何もせず実行終了する。

    - `cargo run`だとユーザー向けのフォルダ階層と異なるので上述のエラーになる。`src/main.rs`の32行目のコメントアウトを解除する。