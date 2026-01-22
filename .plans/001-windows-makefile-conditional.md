# Windows 用 Makefile 条件分岐の導入

## 1. 課題

Windows では find -E が利用できず、Makefile が失敗する。
Windows のみ find -name を使う条件分岐を入れて、他 OS の挙動は維持する。

## 2. 対象箇所の確認

- 対象ファイルは Makefile の CSS 定義。
- 現状は find -E を前提としている。

## 3. 仕様整理

- Windows では find -E を使わない。
- macOS / Linux では従来の find -E -regex を維持する。

## 4. 実装方針

- OS 変数または uname で Windows を判定し、CSS の定義を分岐する。
- Windows 判定時は find src/assets -type f -name *.css を利用する。

## 5. 影響確認

- CSS の収集結果が従来と一致することを確認する。
- 非 Windows 環境の挙動に影響がないことを確認する。

## 6. 動作確認方針

- Windows 環境で make release が成功することを確認する。
- macOS / Linux 環境では find -E の挙動が変わらないことを確認する。

## 7. 対応内容 (結果)

- Makefile で OS=Windows_NT の場合のみ find -name を使う条件分岐を追加した。
- Windows 環境で clean 後のビルドを実施し、成功を確認した。
