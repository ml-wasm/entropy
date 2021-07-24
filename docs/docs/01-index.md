---
slug: /
title: Getting Started with Entropy
---

The goal of this project is to provide a **fast, easy-to-use Dataframes
Javascript library** powered by WebAssembly, written in Rust.

This project is a part of a larger project called [wasml](https://www.github.com/wasml),
which aims to provide a complete machine learning ecosystem in JavaScript.

## Installing the package

_Coming Soon_

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

:::warning Proper error handling not implemented yet

If you are familiar with Rust, currently all the functions just "panic". This
will be fixed in the future.

:::

For more information 

how to work with Series
[Documentation](/).

how to work with For Dataframes 
[Documentation](/).
