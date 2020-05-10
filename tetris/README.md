[![CircleCI](https://circleci.com/gh/asufana/RustGames.svg?style=svg)](https://circleci.com/gh/asufana/RustGames)

# テトリス

[ゲヱム道館館長の素敵なプログラミング実況動画](https://www.youtube.com/channel/UCj3K2Xy0nQr3Jdc0nd-8zQw) から、今回は [テトリス](https://www.youtube.com/watch?v=iosmmQvhyzM) をRustで習作してみます。



## この実装で学べること

##### 実装面

- CTRL+Cで終了する
- EXITコードを設定する
- 入力待ちとならないキー入力受付



## 実装順序

ゲヱム道館先生のプログラミング実況動画とおおよそ同じ順番に合わせているので、実装順は動画を見てもらうのがわかりやすいです。

##### 1. Rustプロジェクト作成

https://github.com/asufana/RustGames/pull/8/commits/3e59f39ad0830a967edbaca727d27c7aa728c5f1

- キーボード入力のところはクレートを使うので、ここに最初から含めてしまいます
- CircleCIの設定も追加しておきます



##### 2. CTRL+Cで抜ける制御

https://github.com/asufana/RustGames/pull/8/commits/5de8f22ce55a32eee2dd6e050dcd9e33287cd975

- いままで手を抜いて実装していなかったので追加します
- windowを戻す処理とexitコードを指定します



##### 3. ボードの初期化と表示

https://github.com/asufana/RustGames/pull/8/commits/b99a699d35550def71e20d14f54bd273104672e1

- ボードを用意して描画します



##### 4. ミノの定義と表示

https://github.com/asufana/RustGames/pull/8/commits/9d015b95a9c86024eec4c8b022a7bc4243b52933

- テトリミノを用意して描画します



##### 5. ミノの移動

https://github.com/asufana/RustGames/pull/8/commits/c1feb591c2d87558f31ed3f252850a1f583fe61b

- カーソルキーでミノを移動できるようにします



##### 6. ミノの移動制御

https://github.com/asufana/RustGames/pull/8/commits/24428ed4788f193e7539c48745a70a88c85c7c40

- ミノが枠外に移動しないように制御します



##### 7. ミノを毎秒落下させる

https://github.com/asufana/RustGames/pull/8/commits/08ce105d8b9c3c3001aa5796d9f57ff2e0734773

- 自動的に落下する処理を追加します



##### 8. 列のクリア

https://github.com/asufana/RustGames/pull/8/commits/8e37cecc9dccb017000f49d90734249171e61298

- 揃った列をクリアします

