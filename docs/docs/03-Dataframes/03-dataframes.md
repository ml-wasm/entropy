---
title: Dataframes 
---

The following scripts assume that you have imported the `ColumnType`,  `SeriesI32`,  `SeriesF64`,  `SeriesSTR`,  `Dataframe` object
from the package as explained in [getting started](../).

## Constructors Methods

These methods are used to create new `Dataframe`.

```js
// Create a Dataframe from Series
    let s1 = new SeriesI32("Apple", [1, 2, 3, 4]);
    let s2 = new SeriesF64("Orange", [1.1, 2.1, 3.1, 4.1]);
    let s3 = new SeriesSTR("Banana", ["ba", "na", "na", "na"]);

    let df = new DataFrame([s1.toJson(), s2.toJson(), s3.toJson()]);
    console.table(df.displayTable);

// Create a Dataframe from csv
    let file = await fetch("../data/fruits.csv");
    let df = await readcsv(file);
```

## Interop Methods

Some handy methods to work with the Dataframes.

```js
// Display Dataframe
    console.table(df.displayTable);

// Display Dataframe as String
    console.log(df.display);
```

## Utility Methods

Dataframe getters and setters.

```js
// Construct Dataframe
    let file = await fetch("../data/fruits.csv");
    let df = await readcsv(file);

// Get Dataframe column types
    console.log(df.columns());    
    // {"Orange" => "FLOAT", "Banana" => "STR", "Apple" => "INTEGER"}

// Get number of rows
    console.log(df.rowsCount());
    // 4

// Get number of columns
    console.log(df.columnsCount());
    // 3

// Get Dataframe column names
    console.log(df.columns());  
    // ["Apple", "Orange", "Banana"]
```

Additional Methods for `Dataframe`

```js
// Create a Dataframe from Series
    let s1 = new SeriesI32("Apple", [1, 2, 3, 4]);
    let s2 = new SeriesF64("Orange", [1.1, 2.1, 3.1, 4.1]);
    let s3 = new SeriesSTR("Banana", ["ba", "na", "na", "na"]);

    let df = new DataFrame([s1.toJson(), s2.toJson(), s3.toJson()]);

// Append new Series
    let s4 = new SeriesI32("Fruits", [9, 8, 9, 8]);
    df.append(ColumnType.INTEGER, sertest.toJson());
    console.log(df.display); // String output
```
:::note
While appending new ***Series*** to ***Dataframe***, you have to specify the type via `ColumnType.Integer`,  `ColumnType.Float`  or  `ColumnType.String`.
:::

```js
// Get Series with column name
    console.log(df.loc("Banana")); 
    // String output

// Get Dataframe row
    console.log(df.ilocr(2)); 
    // [3, 3.1, "na"]

// Get Dataframe column
    console.log(df.ilocc(1));
    // [1.1, 2.1, 3.1, 4.1]
```

## Math Methods

Methods to perform simple mathematical operations on the Dataframe.

```js
// Get minimun of all columns
    console.log(df.minColumns());
    // {"Fruits" => 8, "Apple" => 1, "Orange" => 1.1}

// Get maximum of all columns
    console.log(df.maxColumns());
    // {"Fruits" => 9, "Apple" => 4, "Orange" => 4.1}
// Get mean of all columns
    console.log(df.meanColumns());
    // {"Apple" => 2.5, "Fruits" => 8.5, "Orange" => 2.6}

// Get median of all columns
    console.log(df.medianColumns());
    // {"Apple" => 2.5, "Orange" => 2.6, "Fruits" => 8.5}

// Get varience of all columns params (degree_of_freedom)
    console.log(df.varianceColumns(1));
    // {"Apple" => 2, "Orange" => 1.666666666666666, "Fruits" => 0.6666666666666666}

// Get variance of Dataframe column params (column_name, degree_of_freedom)
    console.log(df.variance("Apple", 1));
    // 2

// Get standard deviation of Dataframe params (degree_of_freedom)
    console.log(df.standardDeviationColumns(1));
    // {"Orange" => 1.2909944487358054, "Fruits" => 0.816496580927726, "Apple" => 1.4142135623730951}

// Get standard deviation of Dataframe column params (column_name, degree_of_freedom)
    console.log(df.standardDeviation("Orange", 1));
    // 1.2909944487358054
```

:::note
`SeriesSTR` columns are not taken into account while performing math methods
:::