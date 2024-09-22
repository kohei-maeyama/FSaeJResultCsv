# FSaeJResultCsv

## はじめに

このレポジトリでは、[学生フォーミュラ日本大会](https://www.jsae.or.jp/formula/)の開催結果をcsv形式に変換して配布しています。

csvファイルは、公益社団法人自動車技術会と無関係な個人が提供する**非公式**な配布物です。

元データは学生フォーミュラ日本大会の[過去の大会結果](https://www.jsae.or.jp/formula/past-result/)ページで公開されています。

## データについて

2018年から最新年の結果を掲載しています。

毎年、最新年の結果を追加する予定です。

2018年以前の結果は掲載予定ありません。（要望が多く出れば対応します）

基本的にpdfの内容を変更せずにcsv化しましたが、以下の変更をしています。

- デザインファイナルの結果はデザインの結果に統合
- コスト&製造審査のAdjusted Costはカンマを含まない様式に変更
- 注釈の消去（例: 2023年エンデュランスの(\*1)~(\*4)など）

## 文字コード

csvファイルはshift-jisでエンコーディングされています。

このため、Microsoft Excelでオープンしても文字化けしません。

### UTF-8対応

utf-8に変換するソフトをGitHubのReleasesページに公開しています。（Windows・Linux）

ソースコード(言語: Rust)は`converter`フォルダにあります。

#### 使い方

Linuxユーザーは`converter.exe`を`converter`に読み替えてください。

1. `converter.exe`を本README.mdのある階層に置く。

2. プログラムを実行する。（ダブルクリックする）

## 課題提起・提案・要望

データの誤りに気がついた場合や、配布のやり方に対する提案・要望は、GitHub Issueに投稿頂けますと幸いです。

Pull Requestは作成しないようにしてください。
