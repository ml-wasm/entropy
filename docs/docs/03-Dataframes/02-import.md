---
title: Import CSV data  
---

With Entropy you can build `Dataframes` with CSV.

### Import readcsv 
```js
    import init, {
        readcsv
    } from "../pkg/entropy.js";
```

### Import data in JavaScript 

```js
    let file = await fetch("any_path.csv"); 
    // Get the file via fetch
```
:::note
Other methods to import files in JavaScript are also accepted

```js
    const [fileHandle] = await window.showOpenFilePicker();
    const file = await fileHandle.getFile();
```
:::

### Build Dataframe
```js
    let df = await readcsv(file);
    // Dataframe is built with csv
```

:::warning
NULL values are not supported yet.
:::