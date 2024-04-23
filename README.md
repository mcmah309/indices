# Indices

[<img alt="github" src="https://img.shields.io/badge/github-mcmah309/indices-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mcmah309/indices)
[<img alt="crates.io" src="https://img.shields.io/crates/v/indices.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/indices)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-indices-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/indices)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/mcmah309/indices/rust.yml?branch=master&style=for-the-badge" height="20">](https://github.com/mcmah309/indices/actions?query=branch%3Amaster)

Zero allocation macros and methods for retrieving multiple mutable indices from a mutable slice safely. 
e.g.
```rust
let (two, four, three) = indices!(slice, 2, 4, 3);
```
## Macros
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

## Methods
Methods allow for more dynamic runtime retrieval when the number of indices is unknown at compile time. e.g.
```rust
fn main() {
    struct Node {
        index: usize,
        visted: usize,
        edges: Vec<usize>,
        message: String,
    }

    let mut graph = vec![
        Node {
            index: 0,
            visted: usize::MAX,
            edges: vec![1, 2],
            message: String::new(),
        },
        Node {
            index: 1,
            visted: usize::MAX,
            edges: vec![0, 2],
            message: String::new(),
        },
        Node {
            index: 2,
            visted: usize::MAX,
            edges: vec![3],
            message: String::new(),
        },
        Node {
            index: 4,
            visted: usize::MAX,
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
            current_node[0].visted = current;
            edge_node.message.push_str(&format!(
                "This is Node `{}` Came from Node `{}`.",
                edge_node.index, current_node[0].visted
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