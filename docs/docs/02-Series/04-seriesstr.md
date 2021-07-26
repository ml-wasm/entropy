---
title: Series - String
---


`SeriesSTR` is a Series where the type of the vector is `SeriesVector` and the Series name is a string.

The following scripts assume that you have imported the `SeriesSTR` object
from the package and set up the threads as explained in [getting started](../).

:::note 
Use `.display` on ***SeriesSTR*** to view the Series.

Use `.data` on ***SeriesSTR*** to view ***SeriesVector*** of the Series.
:::

## Constructors Methods

These methods are used to create new `SeriesSTR`.

```js
// Create an SeriesSTR from a given JavaScript array
    let s = new SeriesSTR("Fruits", ["Apples", "Oranges", "Mangoes", "Banana"]);
    console.log(s.display); 
    /*
    ################
    # Fruits       #
    ################
    # "Apples"     #
    # "Oranges"    #
    # "Mangoes"    #
    # "Banana"     #
    ################
    */
```

## Interop Methods

Some handy methods to work with the Series.

```js
// Return Json representation of Series
    let s = new SeriesSTR("Fruits", ["Apples", "Oranges", "Mangoes", "Banana"]);
    console.log(s.toJson());
	// { name: "Fruits", data: Array(4)}

// Return String Series Representation
    console.log(s.display)
    /*
    ################
    # Fruits       #
    ################
    # "Apples"     #
    # "Oranges"    #
    # "Mangoes"    #
    # "Banana"     #
    ################
    */

// Return SeriesVector of Series
    console.log(s.data())
    // ["Apples", "Oranges", "Mangoes", "Banana"]
```

## Utility Methods

Basic getters and setters.

```js
// Create an SeriesSTR from a given JavaScript array
    let s = new SeriesSTR("Fruits", ["Apples", "Oranges", "Mangoes", "Banana"]);

// Get the length of the array
    console.log(s.len()); 
    // 4

// Set the given value at the specified index
    s.set(0, "Watermelon"); 
    // ["Watermelon", "Oranges", "Mangoes", "Banana"]
    console.log(s.display) 
    // Use to view Series in console

// Get the value at the specified index
    console.log(s.get(2));
    // Mangoes

// Swap the values at the specified indices
    s.swap(0, 1); 
    // ["Oranges", "Watermelon", "Mangoes", "Banana"] 
    console.log(s.display); 
    // Use to view Series in console
```

More complex methods used to manipulate the `SeriesSTR`. Each of these
methods has two versions. The "pure" version returns the result of performing
the operation while the "impure" version actually changes the `SeriesVector`.

| Pure    | Impure   |
| ------- | -------- |
| reverse | reversed |
| append  | appended |
| extend  | extended |
| insert  | inserted |
| splice  | spliced  |

```js
// Create an SeriesSTR from a given JavaScript array
    let s = new SeriesSTR("Fruits", ["Apples", "Oranges", "Mangoes", "Banana"]);

// Reverse the Series Data
    s.reverse(); 
    // ["Banana", "Mangoes", "Oranges", "Apples"] (without return)
    console.log(s.reversed().data); 
    // ["Banana", "Mangoes", "Oranges", "Apples"]

// Append an element to the Series
    s.append("Watermelon"); 
    // ["Apples", "Oranges", "Mangoes", "Banana", "Watermelon"] (without return)
    console.log(s.appended(33).data) 
    // ["Apples", "Oranges", "Mangoes", "Banana", "Watermelon"]

// Extend the SeriesVector with another
    s.extend(["Cherry", "Strawberry"]); 
    // ["Apples", "Oranges", "Mangoes", "Banana", "Watermelon", "Cherry", "Strawberry"] (without return)   
    console.log(s.extended([55, 55]).data); 
    // ["Apples", "Oranges", "Mangoes", "Banana", "Watermelon", "Cherry", "Strawberry"]

// Insert the given element at the specified index
    s.insert(0, "Peaches"); 
    // ["Apples", "Oranges", "Mangoes", "Banana", "Watermelon", "Cherry", "Strawberry", "Peaches"] 
	// (without return)
    console.log(s.inserted(0, 11).data);
    // ["Apples", "Oranges", "Mangoes", "Banana", "Watermelon", "Cherry", "Strawberry", "Peaches"]

// Removes an element from the specified index
    console.log(s.splice(2)); 
    // "Mangoes"

// Remove the value at the specified index and return the array
    console.log(s.spliced(4)[0].data);
    // ["Apples", "Oranges", "Banana", "Watermelon", "Strawberry", "Peaches"]
```