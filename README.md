# Indices

[<img alt="github" src="https://img.shields.io/badge/github-mcmah309/indices-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mcmah309/indices)
[<img alt="crates.io" src="https://img.shields.io/crates/v/indices.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/indices)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-indices-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/indices)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/mcmah309/indices/rust.yml?branch=master&style=for-the-badge" height="20">](https://github.com/mcmah309/indices/actions?query=branch%3Amaster)

Indices provides macros and methods for **safely** retrieving **multiple mutable elements** from **a mutable slice**,
addressing scenarios where slice elements would typically require `RefCell` or `Cell` (interior mutability approach).

e.g.
```rust
let (four, one, two) = indices!(slice, 4, 1, 2);
```
Which expands to
```rust
#[inline(always)]
fn func<T>(slice: &mut [T], one: usize, two: usize, three: usize,) -> (&mut T, &mut T, &mut T) {
    if one == two || one == three || two == three {
        panic!("Duplicate indices are not allowed.");
    }
    let slice_len = slice.len();
    if one >= slice_len || two >= slice_len || three >= slice_len {
        panic!("Index out of bounds.");
    }
    let ptr = slice.as_mut_ptr();
    unsafe { (&mut *ptr.add(one), &mut *ptr.add(two), &mut *ptr.add(three)) }
}
let (four, one, two) = func(slice, 4, 1, 2);
```
Which will be optimized by the rust compiler to essentially the following <ins>pseudo</ins> code
```rust
if 4 >= slice.len() {
    panic!("Index out of bounds.");
}
let (four, one, two) = (slice.get_unchecked_mut(4), slice.get_unchecked_mut(1), slice.slice.get_unchecked_mut(2))
```
The above code is safe, correct, and more performant than using `RefCell` or `Cell`. `indices!` follows the above expansion pattern for up to 4 requested indices.
At which point, the macro will switch to a more optimized approach for many requested indices.

There is also `try_indices!`, `indices_ordered!`, and `try_indices_ordered!`.

### Examples
<details>

<summary>Macro Example</summary>

All macros are zero allocation and allow retrieving a variable number of indices at runtime. Prefer macros when the number
of indices are known at compile time. e.g.
```rust
fn main() {
    struct Person {
        first: String,
        last: String,
    }
    let mut data = [
        Person { first: "John".to_string(), last: "Doe".to_string() },
        Person { first: "Jane".to_string(), last: "Smith".to_string() },
        Person { first: "Alice".to_string(), last: "Johnson".to_string() },
        Person { first: "Bob".to_string(), last: "Brown".to_string() },
        Person { first: "Charlie".to_string(), last: "White".to_string() },
    ];
    fn modify(data_slice: &mut [Person], index: usize){
        let (four, func_provided, three) = indices!(data_slice, 4, index, 3);
        four.last = "Black".to_string();
        func_provided.first = "Jack".to_string();
        three.last = "Jones".to_string();
    }
    let slice = data.as_mut_slice();
    modify(slice, 1);
    assert_eq!(data[4].last, "Black");
    assert_eq!(data[1].first, "Jack");
    assert_eq!(data[3].last, "Jones");
}
```

</details>

<details>

<summary>Method Example</summary>

Methods allow for more dynamic runtime retrieval when the number of indices is unknown at compile time. e.g.
```rust
fn main() {
    struct Node {
        index: usize,
        visited: usize,
        edges: Vec<usize>,
        message: String,
    }

    let mut graph = vec![
        Node {
            index: 0,
            visited: usize::MAX,
            edges: vec![1, 2],
            message: String::new(),
        },
        Node {
            index: 1,
            visited: usize::MAX,
            edges: vec![0, 2],
            message: String::new(),
        },
        Node {
            index: 2,
            visited: usize::MAX,
            edges: vec![3],
            message: String::new(),
        },
        Node {
            index: 4,
            visited: usize::MAX,
            edges: vec![1],
            message: String::new(),
        },
    ];

    fn traverse_graph(graph: &mut [Node], current: usize, start: usize) -> bool {
        if current == start {
            return true;
        }
        let edges = graph[current].edges.clone();
        let [mut current_node, mut edge_nodes] = indices_slices(graph, [&[current], &edges]);
        for edge_node in edge_nodes.iter_mut() {
            current_node[0].visited = current;
            edge_node.message.push_str(&format!(
                "This is Node `{}` Came from Node `{}`.",
                edge_node.index, current_node[0].visited
            ));
        }
        for edge in edges {
            if traverse_graph(graph, edge, start) {
                return true;
            }
        }
        return false;
    }
    traverse_graph(&mut *graph, 2, 0);
    let answers = [
        "This is Node `0` Came from Node `1`.",
        "This is Node `1` Came from Node `3`.",
        "This is Node `2` Came from Node `1`.",
        "This is Node `4` Came from Node `2`.",
    ];
    for (index, node) in graph.iter().enumerate() {
        assert_eq!(&node.message, answers[index]);
    }
}
```

</details>
