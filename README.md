# Indices

[<img alt="github" src="https://img.shields.io/badge/github-mcmah309/indices-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mcmah309/indices)
[<img alt="crates.io" src="https://img.shields.io/crates/v/indices.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/indices)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-indices-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/indices)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/mcmah309/indices/rust.yml?branch=master&style=for-the-badge" height="20">](https://github.com/mcmah309/indices/actions?query=branch%3Amaster)

Zero allocation macros for retrieving mutiple mutable indices from a mutable slice safely.
```rust
let (four, two, six) = indices!(slice, 4, 2, 6);
```
e.g.
```rust
    fn example1() {
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
The following macros are provided:
```rust
indices!
try_indices!
indices_ordered!
try_indices_ordered!
```