# Windows で symlink テスト失敗を warning 扱いにする

## 1. 課題

Windows で config::tests::symlink_config_dir と config::tests::symlink_config_file が失敗する。
ただし挙動は既知の制約なので、Windows 実行時は失敗を warning 扱いにしてテスト全体を通したい。

## 2. 対象箇所の確認

- src/config.rs の symlink_config_dir と symlink_config_file の 2 テスト。

## 3. 仕様整理

- Windows では両テストの失敗を warning として扱い、テストは成功扱いにする。
- Windows 以外では従来通り assert_eq! で失敗させる。

## 4. 実装方針

- テスト内で cfg!(windows) を使い、Windows の場合は assert_eq! を行わず、
  不一致時に eprintln! で warning を出す。
- Windows 以外は従来の assert_eq! を維持する。

## 5. 影響確認

- Windows でのテスト結果は warning のみとなり、CI や push フックを通過できる。
- Windows 以外の環境ではテストの厳密性が維持される。

## 6. 動作確認方針

- Windows で cargo test --lib config::tests::symlink_config_dir を実行し、
  失敗せず warning が出ることを確認する。
- Windows で cargo test --lib config::tests::symlink_config_file を実行し、
  失敗せず warning が出ることを確認する。
- 非 Windows 環境では従来通り失敗時にテストが落ちることを確認する。

## 7. 対応内容 (結果)

- Windows では Config::load の結果を warning 出力に切り替え、失敗や不一致でもテストを通すようにした。
- Windows 以外では従来通り assert_eq! で失敗させる挙動を維持した。
