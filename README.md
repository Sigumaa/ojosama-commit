# ojosama-commit

commit message を ojosama 風に変換して commit いたしますわ～！

## 壱百満天原サロメお嬢様について

以下を参照してくださいまし。

- [壱百満天原サロメ - にじさんじ公式サイト](https://www.nijisanji.jp/members/salome-hyakumantenbara)

- [壱百満天原サロメ - Twitter](https://twitter.com/1000000lome)

## プログラムの使用について

壱百満天原サロメお嬢様、及びそちらの所属の にじさんじ や、 そちらの関係者、おファンコミュニティの方の迷惑にならないように使ってくださいまし。

本プログラムは、にじさんじ所属の壱百満天原サロメお嬢様のおキャラクターをお題材にした二次創作のお一つですわ。 故に、本プログラムは以下二次創作ガイドラインに従いますわ。

- [ANYCOLOR 二次創作ガイドライン](https://event.nijisanji.app/guidelines/)

本プログラムを使う場合もお上記ガイドラインを守ってお使いくださいまし。

## 環境

こちらのプログラムはお次のお条件で動作確認していますわ。

```bash
$ cargo -V
cargo 1.61.0 (2022-04-29)

$ rustup -V
rustup 1.24.3 (2021-05-31)

$ rustc -V
rustc 1.61.0 (2022-05-18)
```

## 使い方

```bash
$ ojo "コミットメッセージです！"
[hoge huga000] コミットメッセージですわ～～！
 ∞ file changed, ∞ insertions(+)
```

## インストール

こちらのおリポジトリを clone して、以下のおコマンドを実行してくださいまし。

```bash
$ cargo install --path .
```


## 注意事項
こちらを使うためにはRustがインストールされている必要がありますわ。  
Rustについてはこちらのおページを見てインストールしてくださいまし。  
[Rust-Lang](https://www.rust-lang.org/learn)

Rustのおバージョンが古いとエラーが出て動かない可能性がありますわ。  
そちらの場合は以下のおコマンドでRustをupdateして、再度試してみてくださいまし。  

```bash
$ rustup update
```


## 感謝

こちらのプログラムは[jiro4989](https://github.com/jiro4989)さんの[ojosama](https://github.com/jiro4989/ojosama)、および[Ojosama Web API](https://github.com/jiro4989/ojosama-web)を使用していますわ。
お心より感謝いたしますわ。
