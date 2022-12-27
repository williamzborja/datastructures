# Introduction

## Big O notation

Is the most common way to measure algorithms **complexity** in terms of:
- time(Speed)
- space(Memory)


Big O Notation measure how input size often represented by **n** impact the algorithms effiency in the **worse cases** and the worse input size for any algorithm is  **tending to infinity** for this reason all constant values can be ignored in Big O Notation.

Example.

We have 3 different algorithms that receive the same a input **n** and  each of them perform differents numbers operations/memory:
`` 1, 900, 9999999, n/2``.

At the worse case **tending infinity**, What's the more efficient algorithm?

Initially you could take the algorithm as a better but that only 1 * n operation but n is infinity:

<code>infinity * 99999 = infinity</code>

<code>infinity * 1 = infinity</code> 

<code>infinity/2 = infinity</code>

Big O complex here is ``O(infinity)`` or ``O(n)``

You could try measure efficiency of  your algorithm with a fixed input size for example 3 or 200 but we wouldn't talking about big O notation.

Understand the behavior of your code in terms of time or memory in the worse case is crucial skill in interviews and coding in general.

```rust
const N: usize = 10; // size of array

fn main() {
    let numbers: [i32; N] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    print(&numbers); // time: O(n)  space: O(1)
}

// given and array of n size print all elements
fn print(nums: &[i32; N]) {
    for num in nums {  //  n * 1 = n;
        print!("{} ", num); // n(infinity) * 1 = n(infinity)
    }
    println!("") // n + 1 = n
}
```

## Recursion


## Programming languages


