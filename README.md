[![npm version](https://badge.fury.io/js/%40yshryk%2Figo-wasm-demo.svg)](https://badge.fury.io/js/%40yshryk%2Figo-wasm-demo)

# igo-rs WebAssembly Demo

Rustで書いた [igo-rs crate](https://crates.io/crates/igo-rs) を
[wasm-pack](https://rustwasm.github.io/wasm-pack/) を利用して簡単にWebAssembly化して、
外部APIを使用する事なくWebブラウザ上で日本語形態素解析を行うデモです。

次のURLからオンラインで試す事ができます。このデモページのソースはディレクトリ www 以下に入っています。 
ビルド結果は docs 以下にあります。

https://yshryk.github.io/igo-wasm-demo/


## 🛠️ ビルド方法

### wasmファイルのビルド

```
$ wasm-pack build
```

成功すると、pkg ディレクトリに igo_wasm_demo_bg.wasm ファイルなどが生成されているはずです。

### デモページのビルド

```
$ cd www
$ npm install
$ npm run start
```

Webブラウザで http://localhost:8080/ を開いて結果を確認します。

## ライセンス

ソースコード部分は MIT OR Apache-2.0 です。

辞書ファイル ipadic.zip.br については、IPADICのライセンス(NOTICE.md)を参照してください。


