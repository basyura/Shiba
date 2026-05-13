# Windows 64bit shiba.exe ビルド手順の追記

## 目的

Windows ビルドを依頼された場合に、Windows 64bit 用の `shiba.exe` を生成する手順が明確になるよう AGENTS.md に追記する。

## 修正方針

- `Build, Test, and Development Commands` に Windows ビルド依頼時の既定動作を追記する。
- 特に指定がない限り `shiba.msi` ではなく `target/release/shiba.exe` の生成を優先することを明記する。
- 実行コマンドとして `make target/release/shiba.exe` を記載する。
- 生成後の確認コマンドとして `ls -lh target/release/shiba.exe` を記載する。
- リポジトリ直下に `build_win.sh` を追加し、実行すると Windows 64bit 用の `shiba.exe` が生成されるようにする。
- AGENTS.md の Windows ビルド手順を `build_win.sh` 利用に更新する。

## 確認

- AGENTS.md の差分を確認する。
- `build_win.sh` を実行して `target/release/shiba.exe` が生成されることを確認する。
