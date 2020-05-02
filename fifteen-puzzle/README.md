[![CircleCI](https://circleci.com/gh/asufana/RustGames.svg?style=svg)](https://circleci.com/gh/asufana/RustGames)

# 15パズル

[ゲヱム道館先生の素敵なプログラミング実況動画](https://www.youtube.com/channel/UCj3K2Xy0nQr3Jdc0nd-8zQw) から、今回は [15パズル](https://www.youtube.com/watch?v=YoQt9RHy8rA) をRustで習作してみます。4 x 4 のセルの中に 1 〜 15 の番号がランダムに配置されていて、番号順に揃えるというあれですね。

ゲヱム道館先生の動画ではC実装ですが、Rustに移植します。コンソール上でASCII表示するところは同じです。



## この実装で学べること

##### 実装面

- 二次元配列の操作
- パターンマッチの使い方
- 構造体と構造体への実装方法
- 単体テスト
- deriveアトリビュートによる継承
- pancursesクレートを利用したキーボード入力受付

##### 構造面

- Rustのファイル分割方法
  - main.rs にすべて書くのは見通しが悪いので、main と構造体とでファイルを分けました



## 実装順序

ゲヱム道館先生のプログラミング実況動画とおおよそ同じ順番に合わせているので、実装順は動画を見てもらうのがわかりやすいです。

##### 1. Rustプロジェクト作成

https://github.com/asufana/RustGames/pull/2/commits/7c0aa1a9f51246f567bcc147cc573f1d7b291160

- 動画 1:20 までの内容

- キーボード入力のところはクレートを使うので、ここに最初から含めてしまいます



##### 2. ボードの初期化と表示

https://github.com/asufana/RustGames/pull/2/commits/cebd0294d3ed47c49c0be892483ef2ba68b5354e

- 動画 8:36 までの内容
- borad.rs ファイルを生成してボードの構造体を管理



##### 3. ブランク位置の取得と表示

https://github.com/asufana/RustGames/pull/2/commits/83bc896a74b5156e51586c5b2cbe70a8b4faab07

- 動画 13:47 までの内容
- position.rs ファイルを生成して位置の構造体を管理



##### 4. ブランク位置の移動

https://github.com/asufana/RustGames/pull/2/commits/985664b0c6a934f48ed4d9d7459ea4d282428771

- 動画 24:53 までの内容
- キー入力をパターンマッチで処理し、ブランク位置を移動させる



##### 5. テストの追加

https://github.com/asufana/RustGames/pull/2/commits/e77877256e6adf90e8133cb42ee026cafd0ee31b

- 4 x 4 枠外にブランク位置が移動しないことを確認する単体テストを追加



##### 6. ゲームクリア判定の追加

https://github.com/asufana/RustGames/pull/2/commits/76b624314467e77eb44148a6f57898c0efbc41bf

- 正しく番号順に並んでいることを評価



##### 7. 初期化時のシャッフル

https://github.com/asufana/RustGames/pull/2/commits/12a4519188e4ddb8e93bb0568be68b1fa5636972

- 動画 28:48 までの内容
- ボード構造体の初期化時にシャッフル処理を追加



##### 8. CircleCIでのCIテスト設定

https://github.com/asufana/RustGames/pull/4







