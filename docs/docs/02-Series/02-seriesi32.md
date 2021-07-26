---
title: Series - Integer
---

`SeriesI32` is a Series where the type of the vector is `IntegersVector` and the Series name is a string.

The following scripts assume that you have imported the `SeriesI32` object
from the package and set up the threads as explained in [getting started](../).

:::note 
Use `.display` on ***SeriesI32*** to view the Series.

Use `.data` on ***SeriesI32*** to view ***IntegersVector*** of the Series.
:::

## Constructors Methods

These methods are used to create new `SeriesI32`.

```js
// Create an SeriesI32 from a given JavaScript array
    let s = new SeriesI32("Fibonacci", [0, 1, 1, 2]);
    console.log(s.display); 
    /*
    ################
    # Fibonacci    #
    ################
    # 0            #
    # 1            #
    # 1            #
    # 2            #
    ################
    */
```

## Interop Methods

Some handy methods to work with the Series.

```js
// Return Json representation of Series
    let s = new SeriesI32("Fibonacci", [0, 1, 1, 2]);
    console.log(s.toJson()); // {name: "Fibonacci", data: Array(4)}

// Return String Series Representation
    console.log(s.display)
    /*
    ################
    # Fibonacci    #
    ################
    # 0            #
    # 1            #
    # 1            #
    # 2            #
    ################
    */

// Return IntegersVector of Series
    console.log(s.data())
    // [0, 1, 1, 2]
```

## Utility Methods

Basic getters and setters.

```js
// Create an SeriesI32 from a given JavaScript array
    let s = new SeriesI32("Fibonacci", [0, 1, 1, 2]);

// Get the length of the array
    console.log(s.len()); 
    // 4

// Set the given value at the specified index
    s.set(0, 9); 
    // [9, 1, 1, 2]
    console.log(s.display) 
    // Use to view Series in console

// Get the value at the specified index
    console.log(s.get(3)); 
    // 2

// Swap the values at the specified indices
    s.swap(0, 1); 
    // [1, 9, 1, 2] 
    console.log(s.display); 
    // Use to view Series in console
```

More complex methods used to manipulate the `SeriesI32`. Each of these
methods has two versions. The "pure" version returns the result of performing
the operation while the "impure" version actually changes the `IntegersVector`.

| Pure    | Impure   |
| ------- | -------- |
| reverse | reversed |
| append  | appended |
| extend  | extended |
| insert  | inserted |
| splice  | spliced  |

```js
// Create an SeriesI32 from a given JavaScript array
    let s = new SeriesI32("Fibonacci", [0, 1, 1, 2]);

// Reverse the Series Data
    s.reverse(); 
    // [2, 1, 1, 0] (without return)
    console.log(s.reversed().data); 
    // [2, 1, 1, 0]

// Append an element to the Series
    s.append(88); 
    // [2, 1, 1, 0, 88] (without return)
    console.log(s.appended(33).data) 
    // [2, 1, 1, 0, 88, 33] 

// Extend the IntegersVector with another
    s.extend([44, 44]); 
    // [2, 1, 1, 0, 88, 44, 44] (without return)   
    console.log(s.extended([55, 55]).data); 
    // [2, 1, 1, 0, 88, 44, 44, 55, 55]

// Insert the given element at the specified index
    s.insert(0, 99); 
    // [99, 2, 1, 1, 0, 88, 44, 44] (without return)
    console.log(s.inserted(0, 11).data);
    // [11, 99, 2, 1, 1, 0, 88, 44, 44]

// Removes an element from the specified index
    console.log(s.splice(0)); 
    // 99

// Remove the value at the specified index and return the array
    console.log(s.spliced(6)[0].data); 
    // [2, 1, 1, 0, 88, 44]
```
## Math Methods

Methods to perform simple mathematical operations on the Series.

```js
// Create an SeriesI32 from a given JavaScript array
    let s = new SeriesI32("Fibonacci", [0, 1, 1, 2]);

// SeriesI32 max
    console.log(s.max());
    // 2

// SeriesI32 min
    console.log(s.min());
    // 0

// SeriesI32 sum
    console.log(s.sum());
    // 4

// SeriesI32 product
    console.log(s.product());
    // 0

// SeriesI32 mean
    console.log(s.mean());
    // 1

// SeriesI32 median
    console.log(s.median());
    // 1

// SeriesI32 standard deviation  params (degree_of_freedom)
    console.log(s.std_dev(1));
    // 0.816496580927726

// SeriesI32 variance params (degree_of_freedom)
    console.log(s.variance(1));
    // 0.6666666666666666

```