# rust atcoder

## atcoder contest の際

- 新しいコンテストの作成
- contests 配下で実行

```
cargo compete new
```

- member 追加（これしないと rust-analyzer 効かない）

```
cargo member include
```

- テストケースを試す

```
cargo compete test
```

- 提出する

```
cargo compete submit
```

## AtCoderProblems するとき

- やっぱコンテストやる時と同じで

- problems 配下で実行

```
cargo new --bin 作りたいフォルダ名
```

```
cargo member include 作ったフォルダ名
```

- この後 src 配下に bin ディレクトリを入れる（main.rs も移動させてもよし）

## virtual contest する時

<!-- https://github.com/qryxip/cargo-compete/pull/166 -->

```
cargo compete new 'contestID'
```

```
cargo member include 作られたフォルダ名
```
