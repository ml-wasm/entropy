---
title: Series - Float
---

`SeriesF32` is a Series where the type of the vector is `FloatsVector` and the Series name is a string.

The following scripts assume that you have imported the `SeriesF32` object
from the package and set up the threads as explained in [getting started](../).

:::note 
Use `.display` on ***SeriesF32*** to view the Series.

Use `.data` on ***SeriesF32*** to view ***FloatsVector*** of the Series.
:::

## Constructors Methods

These methods are used to create new `SeriesF32`.

```js
// Create an SeriesF32 from a given JavaScript array
    let s = new SeriesF32("Grades", [9.5, 9.0, 7.8, 8.5]);
    console.log(s.display); 
    /*
    ################
    # Grades    #
    ################
    # 9.5          #
    # 9.0          #
    # 7.8          #
    # 8.5          #
    ################
    */
```

## Interop Methods

Some handy methods to work with the Series.

```js
// Return Json representation of Series
    let s = new SeriesF32("Grades", [9.5, 9.0, 7.8, 8.5]);
    console.log(s.toJson());
    // {name: "Grades", data: Array(4)}

// Return String Series Representation
    console.log(s.display)
    /*
    ################
    # Grades   	   #
    ################
    # 9.5          #
    # 9.0          #
    # 7.8          #
    # 8.5          #
    ################
    */

// Return FloatsVector of Series
    console.log(s.data())
    // [9.5, 9.0, 7.8, 8.5]
```

## Utility Methods

Basic getters and setters.

```js
// Create an SeriesF32 from a given JavaScript array
    let s = new SeriesF32("Grades", [9.5, 9.0, 7.8, 8.5]);

// Get the length of the array
    console.log(s.len()); 
    // 4

// Set the given value at the specified index
    s.set(1, 10); 
    // [9.5, 10, 7.8, 8.5]
    console.log(s.display) 
    // Use to view Series in console

// Get the value at the specified index
    console.log(s.get(3)); 
    // 8.5

// Swap the values at the specified indices
    s.swap(0, 1); 
    // [10, 9.5, 7.8, 8.5]
    console.log(s.display); 
    // Use to view Series in console
```

More complex methods used to manipulate the `SeriesF32`. Each of these
methods has two versions. The "pure" version returns the result of performing
the operation while the "impure" version actually changes the `FloatsVector`.

| Pure    | Impure   |
| ------- | -------- |
| reverse | reversed |
| append  | appended |
| extend  | extended |
| insert  | inserted |
| splice  | spliced  |

```js
// Create an SeriesF32 from a given JavaScript array
    let s = new SeriesF32("Grades", [9.5, 9.0, 7.8, 8.5]);

// Reverse the Series Data
    s.reverse(); 
    // [8.5, 7.8, 9.0, 9.5] (without return)
    console.log(s.reversed().data); 
    // [8.5, 7.8, 9.0, 9.5]

// Append an element to the Series
    s.append(10); 
    // [8.5, 7.8, 9.0, 9.5, 10] (without return)
    console.log(s.appended(33).data) 
    // [8.5, 7.8, 9.0, 9.5, 10] 

// Extend the FloatsVector with another
    s.extend([7.5, 8.5]); 
    // [8.5, 7.8, 9.0, 9.5, 10, 7.5, 8.5]  (without return)   
    console.log(s.extended([7.5, 8.5]).data); 
    // [8.5, 7.8, 9.0, 9.5, 10, 7.5, 8.5]

// Insert the given element at the specified index
    s.insert(2, 6); 
    // [8.5, 7.8, 6, 9.0, 9.5, 10, 7.5, 8.5] (without return)
    console.log(s.inserted(2, 6).data);
    // [8.5, 7.8, 6.0, 9.0, 9.5, 10, 7.5, 8.5]

// Removes an element from the specified index
    console.log(s.splice(0)); 
    // 8.5

// Remove the value at the specified index and return the array
    console.log(s.spliced(6)[0].data); 
    // [7.8, 6.0, 9.0, 9.5, 10, 7.5]
```
## Math Methods

Methods to perform simple mathematical operations on the Series.

```js
// Create an SeriesF32 from a given JavaScript array
    let s = new SeriesF32("Grades", [9.5, 9.0, 7.8, 8.5]);

// SeriesF32 max
    console.log(s.max());
    // 9.5

// SeriesF32 min
    console.log(s.min());
    // 7.8

// SeriesF32 sum
    console.log(s.sum());
    // 34.8

// SeriesF32 product
    console.log(s.product());
    // 5668.65

// SeriesF32 mean
    console.log(s.mean());
    // 8.7

// SeriesF32 median
    console.log(s.median());
    // 8.75

// SeriesF32 standard deviation  params (degree_of_freedom)
    console.log(s.std_dev(1));
    // 0.7257180352359078

// SeriesF32 variance params (degree_of_freedom)
    console.log(s.variance(1));
    // 0.5266666666666663

```