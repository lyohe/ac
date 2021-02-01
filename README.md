My personal repository for [AtCoder](https://atcoder.jp/).

[cargo-atcoder](https://github.com/tanakh/cargo-atcoder) を利用する。環境設定等はそちらを参照。

# 以下コピペ用

## 開始

コンテストが始まったら ID を使って cargo プロジェクトを作る。

```
cargo atcoder new abc000
```

`/src/bin` 以下に提出用のファイルが a ~ f まで生成されるので、解答を書いていく。

## テスト

解答を書き終わったら問題文の ID を指定してサンプルケースをテストできる。

```
cargo atcoder test a
```

### 自分で標準入出力

`--custom` を付けると自分で標準入力できる。

```
cargo atcoder test a --custom
```

普通に ↓ とかでもいい。

```
cargo run --bin a
```

## 提出

提出は ↓ でできる。サンプルケースが通らないと提出できない。

```
cargo atcoder submit a
```