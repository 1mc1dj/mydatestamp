# mydatestamp

DXの遅れた日本では、サラリーマンが書類に印鑑を捺印する文化が残っている。
中途半端に、DXの進んだ会社では、電子書類で処理されるようになっても、印鑑を押す文化があり、それに付き合わされる形で、ExcelやWordなどに、日付印の画像を入力することが求められる。

電子署名付きの画像とかであれば、それなりに意味はあるとは思うのだが、ただの画像ファイルであれば、偽造も簡単だし、バカバカしいとは思うのだが、
付き合ってことなきを得たい場合もあるだろう。

そのために、コマンドで、陰影のsvgファイルを作成するコマンドを作った。

![出力サンプル](./sample.png) 

## 使い方

Rustの開発環境がインストールしておいてください。
cargo buildでビルドして、どっかPATHが通るところに置いてください。

```
$ git clone https://github.com/1mc1dj/mydatestamp.git
$ cd mydatestamp
$ cargo build --release
$ cp target/release/mydatestamp.exe ~/myscripts/ (Pathが通っているところにコピー、Cygwinとかの場合）
```

次に、ホームディレクトリ（$HOMEのパスが通ってるところ）に、.mydatestampというファイルを作って、

```
山本商事
営業3部6課
山下一郎
```
とか書いて、UTF8で保存しておいてください。

そうすれば、以下のように、実行すると、svgファイルが作成されます。

```
$ mydatestamp.exe test.svg
```

