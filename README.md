# Indices

Zero allocation macros for extracting mutiple mutable indices from a slice safely.

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