# Entropy

The goal of this project is to provide a **fast, easy-to-use Dataframes
Javascript library** powered by WebAssembly, written in Rust.

This project is a part of a larger project called [ml-wasm](https://www.github.com/ml-wasm),
which aims to provide a complete machine learning ecosystem in JavaScript.

## Documentation

[Entropy](https://ml-wasm.github.io/entropy/)

## Installing the package

[NPM](https://www.npmjs.com/package/@ml.wasm/entropy)

## How to use

```js
import init, {
  ColumnType,
  SeriesF64,
  SeriesI32,
  SeriesSTR,
  DataFrame,
  readcsv,
} from "../pkg/entropy.js";

(async () => {
  // This init function sets up everything you need to use this library
  await init();

  // All your code goes here...
})();
```
