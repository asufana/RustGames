[![CircleCI](https://circleci.com/gh/asufana/RustGames.svg?style=svg)](https://circleci.com/gh/asufana/RustGames)

# パズドラ

[ゲヱム道館先生の素敵なプログラミング実況動画](https://www.youtube.com/channel/UCj3K2Xy0nQr3Jdc0nd-8zQw) から、今回は [パズドラ](https://www.youtube.com/watch?v=WUHzFk5X-aQ) をRustで習作してみます。



## この実装で学べること

##### 実装面

- 関数を引数に使う



## 実装順序

ゲヱム道館先生のプログラミング実況動画とおおよそ同じ順番に合わせているので、実装順は動画を見てもらうのがわかりやすいです。

##### 1. Rustプロジェクト作成

https://github.com/asufana/RustGames/pull/6/commits/5d81ee521e3fdd1c3c5515eface3e125d2a2cd2d

- 動画 2:13 までの内容
- キーボード入力のところはクレートを使うので、ここに最初から含めてしまいます



##### 2. CircleCI設定

https://github.com/asufana/RustGames/pull/6/commits/dd8e5b2532a7eac25a6c6e3b051cec87921941e3

- CircleCIにもビルドテストを設定しておきます



##### 3. ボードの初期化と表示

https://github.com/asufana/RustGames/pull/6/commits/82ae39601adbf68f089c8870fe57974ffde4ad46

- borad.rs ファイルを生成してボードの構造体を管理



##### 4. ドロップ種別の追加

https://github.com/asufana/RustGames/pull/6/commits/2d7f637e260717871ab9b91ed88e81c8429d50c2

- ◯△▽といったドロップ種別の追加
- 2次元配列を毎回走査するのはイケてないので関数を引数に取るメソッドを整備







