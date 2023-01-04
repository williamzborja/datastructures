# How are useful

Exists several different types of DS but understand when you need to use an
specific DS is crucial in Computer Science and Interviews.

## How you can solve problem with DS?

- You can understand the expected behavior with edge case e.g: empty, all
  repeated with numbers etc.

- You could solve to problem if you identify the input DS how traverse, update
  and identify patterns for instance two pointers or sliding windows etc.

- You need another extra DS to find the solution.

- Measure the solution efficiency and try to improve that but your limit is the
  input DS for example in average O(n) is the better solution when you receive
  array or string but depend of the problem.

I here going to provide the first example.

## Find Highest Frequency

this example is only a fast overview you likely will see here string, array,
vector, maps all of those will be explain in more detail in the future but right
now I want to show you why DS is so important.

Given a string s return the most common character.

`Input: s = "aabacaad"`

`Output: a`

### Solving

#### Edge Cases

Maybe you try to solve the problem finding edge case like these when string is
empty "" or it doesn't has repeated to understand. which is the expected
behavior you can set Input and Output pairs.

    - Empty: `Input: "" -> Output: "_"`
    - Non-repeated `Input: "abc" -> Output:"a"`

It's really important you never could assume the input or output in interview
context inclusively if could be obviously for example empty case could be set
output `""` instead to `"_"` but for that case they want to manage error
behavior like this.

Same with non-repeated case if suppose that `"a"`is the correct answer you need
to confirm with your interviewer maybe they say the correct answer is `"_"`
because in this problem non-repeated is an error and errors should be managed
with `"_"`

Confirm all cases with your interviewer before to start to solving.

You can delimit the problem and understand how is the expected behavior all info
useful to solve it.

But it has not been resolved yet.

### Input data structure

The input is an string and array of chars and UTF-8 chars traverse and string
have O(n) but all other complexities depend of your language.

Strings are mutable for C/C++, Rust and immutable Python, Java, Golang, Rust
etc.

If you change for example numbers[0] the first element in an immutable array the
complexity is O(n) you change all data and create new string

That was a little example as how input data structure impact directly the
performance in your algo.

### how store information

We need organize info with `"a"->3` and associate letter with frequency exist an
**DS** associative **hash table** or **map** but for most accurate solutions
exist different types of that sorted, unsorted and other like a sets choose the
correct impact totally you algo efficiency.

It looks like this:

`input: "aba"`

```json
{ "a": 2, "b": 1 }
```

And easy understand that the more common letter is **a**

Let's to dive in code.

Code with C++

```c++
char highest_freq(string &input) {
  unordered_map<char, int> freq; // hash map
  for (auto c : input)           // fill map with frequencies
    freq[c] += 1;

  // get max key and value pair
  auto pair = min_element(freq.begin(), freq.end());
  return pair->first; // return key
}
```

With Python

```python
def highest_freq(input: str) -> str:
    freq = {}  # dict or hash map

    for c in input:  # fill map with frequencies
        if c in freq:
            freq[c] += 1
        else:
            freq[c] = 1

    return max(freq, key=freq.get)
```

And finally with Rust

```rust
fn highest_freq(input: &str) -> char {
    let mut freq: HashMap<char, u32> = HashMap::new(); // hash map

    for c in input.chars() { // fill map
        let count = freq.entry(c).or_default();
        *count += 1;
    }
    // find max
    let max_option = freq.iter().max_by(|a, b| a.1.cmp(b.1));
    // return max or empty if max is null
    match max_option {
        Some((c, _)) => return *c,
        None => return ' ',
    }
}
```

For all solution the complexity is the same because we need put **n** elements
in the hash map for that the time and space is `O(n)` and after access a hash
map have `O(1)` and put only one item.

Finally the complexity is not `O(n) + 1` is only `O(n)` remember **tending to
infinity** case

**With this problem all langs had the same complex, but this doesn't mean that
all languages are equally efficient.**

With the next example you could understand why.

`O(1)` is better than `O(n)` algorithm,But:

| Input size: | 2    |
| ----------- | ---- |
| `O(n)`      | 2ms  |
| `O(1)`      | 20ms |

For this case `O(n)` is faster than `O(1)` it is possible remember big O find
better algo in **tending to infinity** case

| Input size: | 200000000 |
| ----------- | --------- |
| `O(n)`      | 2 years   |
| `O(1)`      | 20ms      |

big O forget all implementation details like programming language, compiler,
interpreter, multi-core or single core, faster memory, hardware etc.

If you take O(n) algo and add all constants delete for big O you could have

```
Input size = 2200

Your algo =  O(2200)
Language Python = compiler O(200) + CPU Cache O(300) + optimization O(2) 
+ Interpreter O(5000) + Garbage Collector(3000) + Safe pointers(500)...

Rust = compiler O(300) + CPU Cache O(300)
```

Big O summarize all to O(n).

The final efficiency is `Big O + Language efficiency + Maintainability + ....`
but for Algorithms and data structures interview is only important and **DS**,
**Algorithms** and **BigO**.

But for real work definitely language is really important.

### Conclusion

You saw how choose the DS impact in your algorithm and complexity

this may seem difficult but with only DSA knowledge you can solve thousand of
coding challenges and variants you don't need memorize solutions or learn low
level in C, compilers, complex math or any other.

All language have a DS implementation for that you can choose any language but
only in context of DSA interviews language is not important.

and let's to continue with another problem where DS are key.
