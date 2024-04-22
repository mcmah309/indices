mod errors;

pub use errors::*;

const fn create_array<const N: usize>() -> [usize; N] {
    let mut arr: [usize; N] = [0; N];
    let mut i = 0;
    while i < N {
        arr[i] = i;
        i += 1;
    }
    arr
}

// fn create_vec(size: usize) -> Vec<usize> {
//     let mut vector = Vec::with_capacity(size);
//     let mut i = 0;
//     while i < size {
//         vector.push(i);
//         i += 1;
//     }
//     vector
// }

#[inline(always)]
fn insertion_sort<T: PartialOrd>(s: &mut [T]) {
    for i in 1..s.len() {
        let mut j = i;
        while j > 0 && s[j - 1] > s[j] {
            s.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[inline(always)]
fn tracked_insertion_sort<T: PartialOrd, U>(s: &mut [T], follower: &mut [U]) {
    debug_assert!(s.len() == follower.len());
    for i in 1..s.len() {
        let mut j = i;
        while j > 0 && s[j - 1] > s[j] {
            s.swap(j - 1, j);
            follower.swap(j - 1, j);
            j -= 1;
        }
    }
}

//************************************************************************//

// pub fn indices_vec<'a, T>(slice: &'a mut [T], indices: &mut [usize]) -> Vec<&'a mut T> {
//     let mut follower = create_vec(indices.len());
//     tracked_insertion_sort(indices, &mut follower);
//     let indices_len_minus_one = indices.len() - 1;
//     let slice_len_minus_one = slice.len() - 1;
//     check_panic(&indices, slice_len_minus_one, indices_len_minus_one);
//     let size = indices.len();
//     let mut vector: Vec<std::mem::MaybeUninit<&'a mut T>> = Vec::with_capacity(size);
//     for _ in 0..size {
//         vector.push(std::mem::MaybeUninit::uninit());
//     }
//     let ptr = slice.as_mut_ptr();
//     unsafe {
//         for (i, index) in follower.iter().enumerate() {
//             vector[*index] = std::mem::MaybeUninit::new(&mut *ptr.add(i));
//         }
//         let initialized_vec: Vec<&'a mut T> = {
//             // Transmute to the initialized type
//             let ptr = vector.as_mut_ptr() as *mut &'a mut T;
//             Vec::from_raw_parts(ptr, size, size)
//         };
//         initialized_vec
//     }
// }

pub fn indices_array<'a, T, const N: usize>(
    slice: &'a mut [T],
    indices: &mut [usize; N],
) -> [&'a mut T; N] {
    let mut follower: [usize; N] = create_array();
    tracked_insertion_sort(indices, &mut follower);
    let indices_len_minus_one = N - 1;
    let slice_len_minus_one = slice.len() - 1;
    for i in 0..indices_len_minus_one {
        if indices[i] > slice_len_minus_one {
            panic!(
                "Index out of bounds. Requested index was `{}` while slice length was `{}`.",
                indices[i],
                slice_len_minus_one + 1
            );
        }
        if indices[i] == indices[i + 1] {
            panic!(
                "Duplicate indices are not allowed. Index `{}` was requested twice.",
                indices[i]
            );
        }
    }
    if indices[indices_len_minus_one] > slice_len_minus_one {
        panic!(
            "Index out of bounds. Requested index was `{}` while slice length was `{}`.",
            indices[indices_len_minus_one],
            slice_len_minus_one + 1
        );
    }
    let ptr = slice.as_mut_ptr();
    unsafe {
        let mut array: [std::mem::MaybeUninit<*mut T>; N] =
            std::mem::MaybeUninit::uninit().assume_init();
        for (i, index) in follower.iter().enumerate() {
            array[*index] = std::mem::MaybeUninit::new(ptr.add(indices[i]));
        }
        std::mem::transmute_copy::<_, [&'a mut T; N]>(&array)
    }
}

//************************************************************************//

/// Returns mutable references for the requested indices.
/// Panics if any index is out of bounds or duplicated.
#[macro_export]
macro_rules! indices {
    ($slice:expr, $( $index:expr ),*) => {{
        let slice = $slice;
        let mut indices = [$($index),*];
        $crate::insertion_sort(&mut indices);

        let indices_len_minus_one = indices.len() - 1;
        let slice_len_minus_one = slice.len() - 1;
        for i in 0..indices_len_minus_one {
            if indices[i] > slice_len_minus_one {
                panic!("Index out of bounds. Requested index was `{}` while slice length was `{}`.", indices[i], slice_len_minus_one + 1)
            }
            if indices[i] == indices[i + 1] {
                panic!("Duplicate indices are not allowed. Index `{}` was requested twice.", indices[i])
            }
        }
        if indices[indices_len_minus_one] > slice_len_minus_one {
            panic!("Index out of bounds. Requested index was `{}` while slice length was `{}`.", indices[indices_len_minus_one], slice_len_minus_one + 1)
        }

        let ptr = slice.as_mut_ptr();
        (
            $(unsafe { &mut *ptr.add($index) },)*
        )
    }};
}

/// Returns mutable references for the requested indices.
/// Panics if any index is out of bounds or duplicated.
#[macro_export]
macro_rules! try_indices {
    ($slice:expr, $( $index:expr ),*) => {{
        (|| {
        let slice = $slice;
        let mut indices = [$($index),*];
        $crate::insertion_sort(&mut indices);

        let indices_len_minus_one = indices.len() - 1;
        let slice_len_minus_one = slice.len() - 1;
        for i in 0..indices_len_minus_one {
            if indices[i] > slice_len_minus_one {
                return Err($crate::TryIndicesError::IndexOutOfBounds);
            }
            if indices[i] == indices[i + 1] {
                return Err($crate::TryIndicesError::DuplicateIndex);
            }
        }
        if indices[indices_len_minus_one] > slice_len_minus_one {
            return Err($crate::TryIndicesError::IndexOutOfBounds);
        }

        let ptr = slice.as_mut_ptr();
        Ok((
            $(unsafe { &mut *ptr.add($index) },)*
        ))
    })()
    }};
}

/// Returns mutable references for the requested indices
/// Slightly more efficient than `indices!` since assumes the requested indices are already ordered smallest to largest.
/// Panics if the requested indicies are not smallest to largest, or if any index is duplicated or out of bounds.
#[macro_export]
macro_rules! indices_ordered {
    ($slice:expr, $( $index:expr ),*) => {{
        let slice = $slice;
        let indices = [$($index),*];

        let indices_len_minus_one = indices.len() - 1;
        let slice_len_minus_one = slice.len() - 1;
        for i in 0..indices_len_minus_one {
            if indices[i] > slice_len_minus_one {
                panic!("Index out of bounds. Requested index was `{}` while slice length was `{}`.", indices[i], slice_len_minus_one + 1)
            }
            if indices[i] >= indices[i + 1] {
                panic!("Indices not sorted or duplicate indices detected.")
            }
        }
        if indices[indices_len_minus_one] > slice_len_minus_one {
            panic!("Index out of bounds. Requested index was `{}` while slice length was `{}`.", indices[indices_len_minus_one], slice_len_minus_one + 1)
        }

        let ptr = slice.as_mut_ptr();
        (
            $(unsafe { &mut *ptr.add($index) },)*
        )
    }};
}

/// Returns mutable references for the requested indices.
/// Slightly more efficient than `try_indices!` since assumes the requested indices are already ordered smallest to largest.
/// Returns `TryOrderedIndicesError` if the requested indicies are not smallest to largest, or if any index is duplicated or out of bounds.
#[macro_export]
macro_rules! try_indices_ordered {
    ($slice:expr, $( $index:expr ),*) => {{
        (|| {
        let slice = $slice;
        let indices = [$($index),*];

        let indices_len_minus_one = indices.len() - 1;
        let slice_len_minus_one = slice.len() - 1;
        for i in 0..indices_len_minus_one {
            if indices[i] > slice_len_minus_one {
                return Err($crate::TryIndicesOrderedError::IndexOutOfBounds);
            }
            if indices[i] >= indices[i + 1] {
                return Err($crate::TryIndicesOrderedError::InvalidIndex);
            }
        }
        if indices[indices_len_minus_one] > slice_len_minus_one {
            return Err($crate::TryIndicesOrderedError::IndexOutOfBounds);
        }

        let ptr = slice.as_mut_ptr();
        Ok((
            $(unsafe { &mut *ptr.add($index) },)*
        ))
    })()
    }};
}

//************************************************************************//

#[cfg(test)]
mod tests {
    use crate::{indices_array, TryIndicesError, TryIndicesOrderedError};

    // #[test]
    // fn indices_vec_works() {
    //     let mut data = [5, 4, 3, 2, 1];
    //     let slice = data.as_mut_slice();
    //     let [one, two] = indices_vec(slice, &mut [1, 3]).try_into().unwrap();
    //     assert_eq!(one, &mut 4);
    //     assert_eq!(two, &mut 2);
    //     *one = 10;
    //     *two = 20;
    //     assert_eq!(data, [5, 10, 3, 20, 1]);
    // }

    // #[test]
    // fn indices_vec_out_of_order() {
    //     let mut data = [5, 4, 3, 2, 1];
    //     let slice = data.as_mut_slice();
    //     let [one, two] = indices_vec(slice, &mut [3, 1]).try_into().unwrap();
    //     assert_eq!(one, &mut 2);
    //     assert_eq!(two, &mut 4);
    //     *one = 10;
    //     *two = 20;
    //     assert_eq!(data, [5, 20, 3, 10, 1]);
    // }

    // #[test]
    // fn indices_vec_more_than_two_indices() {
    //     let mut data = [5, 4, 3, 2, 1];
    //     let slice = data.as_mut_slice();
    //     let [one, two, three] = indices_vec(slice, &mut [3, 1, 2]).try_into().unwrap();
    //     assert_eq!(one, &mut 2);
    //     assert_eq!(two, &mut 4);
    //     assert_eq!(three, &mut 3);
    //     *one = 10;
    //     *two = 20;
    //     *three = 30;
    //     assert_eq!(data, [5, 20, 30, 10, 1]);
    // }

    // #[should_panic]
    // #[test]
    // fn indices_vec_duplicate_indices() {
    //     let mut data = [5, 4, 3, 2, 1];
    //     let slice = data.as_mut_slice();
    //     let [_one, _two] = indices_vec(slice, &mut [3, 3]).try_into().unwrap();
    // }

    // #[should_panic]
    // #[test]
    // fn indices_vec_out_of_bounds() {
    //     let mut data = [5, 4, 3, 2, 1];
    //     let slice = data.as_mut_slice();
    //     let [_one, _two] = indices_vec(slice, &mut [3, 5]).try_into().unwrap();
    // }

    //************************************************************************//

    #[test]
    fn indices_array_works() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [one, two] = indices_array(slice, &mut [1, 3]);
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 2);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 10, 3, 20, 1]);
    }

    #[test]
    fn indices_array_out_of_order() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [one, two] = indices_array(slice, &mut [3, 1]);
        assert_eq!(one, &mut 2);
        assert_eq!(two, &mut 4);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 20, 3, 10, 1]);
    }

    #[test]
    fn indices_array_more_than_two_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [one, two, three] = indices_array(slice, &mut [3, 1, 2]);
        assert_eq!(one, &mut 2);
        assert_eq!(two, &mut 4);
        assert_eq!(three, &mut 3);
        *one = 10;
        *two = 20;
        *three = 30;
        assert_eq!(data, [5, 20, 30, 10, 1]);
    }

    #[should_panic]
    #[test]
    fn indices_array_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [_one, _two] = indices_array(slice, &mut [3, 3]);
    }

    #[should_panic]
    #[test]
    fn indices_array_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [_one, _two] = indices_array(slice, &mut [3, 5]);
    }

    //************************************************************************//

    #[test]
    fn indices_works() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two) = indices!(slice, 1, 3);
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 2);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 10, 3, 20, 1]);
    }

    #[test]
    fn indices_out_of_order() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two) = indices!(slice, 3, 1);
        assert_eq!(one, &mut 2);
        assert_eq!(two, &mut 4);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 20, 3, 10, 1]);
    }

    #[test]
    fn indices_more_than_two_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two, three) = indices!(slice, 3, 1, 2);
        assert_eq!(one, &mut 2);
        assert_eq!(two, &mut 4);
        assert_eq!(three, &mut 3);
        *one = 10;
        *two = 20;
        *three = 30;
        assert_eq!(data, [5, 20, 30, 10, 1]);
    }

    #[should_panic]
    #[test]
    fn indices_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two) = indices!(slice, 3, 3);
    }

    #[should_panic]
    #[test]
    fn indices_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two) = indices!(slice, 3, 5);
    }

    //************************************************************************//

    #[test]
    fn try_indices_works() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two) = try_indices!(slice, 1, 3).unwrap();
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 2);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 10, 3, 20, 1]);
    }

    #[test]
    fn try_indices_out_of_order() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two) = try_indices!(slice, 3, 1).unwrap();
        assert_eq!(one, &mut 2);
        assert_eq!(two, &mut 4);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 20, 3, 10, 1]);
    }

    #[test]
    fn try_indices_more_than_two_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two, three) = try_indices!(slice, 3, 1, 2).unwrap();
        assert_eq!(one, &mut 2);
        assert_eq!(two, &mut 4);
        assert_eq!(three, &mut 3);
        *one = 10;
        *two = 20;
        *three = 30;
        assert_eq!(data, [5, 20, 30, 10, 1]);
    }

    #[test]
    fn try_indices_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 3, 3);
        assert_eq!(result, Err(TryIndicesError::DuplicateIndex))
    }

    #[test]
    fn try_indices_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 3, 5);
        assert_eq!(result, Err(TryIndicesError::IndexOutOfBounds))
    }

    //************************************************************************//

    #[test]
    fn indices_ordered_works() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two) = indices_ordered!(slice, 1, 3);
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 2);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 10, 3, 20, 1]);
    }

    #[test]
    fn indices_ordered_more_than_two_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two, three) = indices_ordered!(slice, 1, 2, 4);
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 3);
        assert_eq!(three, &mut 1);
        *one = 10;
        *two = 20;
        *three = 30;
        assert_eq!(data, [5, 10, 20, 2, 30]);
    }

    #[should_panic]
    #[test]
    fn indices_ordered_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two) = indices_ordered!(slice, 3, 3);
    }

    #[should_panic]
    #[test]
    fn indices_ordered_out_of_order() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two) = indices_ordered!(slice, 3, 1);
    }

    #[should_panic]
    #[test]
    fn indices_ordered_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two) = indices_ordered!(slice, 3, 5);
    }

    //************************************************************************//

    #[test]
    fn try_indices_ordered_works() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two) = try_indices_ordered!(slice, 1, 3).unwrap();
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 2);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 10, 3, 20, 1]);
    }

    #[test]
    fn try_indices_ordered_more_than_two_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two, three) = try_indices_ordered!(slice, 1, 2, 4).unwrap();
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 3);
        assert_eq!(three, &mut 1);
        *one = 10;
        *two = 20;
        *three = 30;
        assert_eq!(data, [5, 10, 20, 2, 30]);
    }

    #[test]
    fn try_indices_ordered_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices_ordered!(slice, 3, 3);
        assert_eq!(result, Err(TryIndicesOrderedError::InvalidIndex));
    }

    #[test]
    fn try_indices_ordered_out_of_order() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices_ordered!(slice, 3, 1);
        assert_eq!(result, Err(TryIndicesOrderedError::InvalidIndex));
    }

    #[test]
    fn try_indices_ordered_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices_ordered!(slice, 3, 5);
        assert_eq!(result, Err(TryIndicesOrderedError::IndexOutOfBounds));
    }
}

#[cfg(test)]
mod example_tests {
    #[test]
    fn example1() {
        struct Person {
            first: String,
            last: String,
        }
        let mut data = [
            Person {
                first: "John".to_string(),
                last: "Doe".to_string(),
            },
            Person {
                first: "Jane".to_string(),
                last: "Smith".to_string(),
            },
            Person {
                first: "Alice".to_string(),
                last: "Johnson".to_string(),
            },
            Person {
                first: "Bob".to_string(),
                last: "Brown".to_string(),
            },
            Person {
                first: "Charlie".to_string(),
                last: "White".to_string(),
            },
        ];
        fn modify(data_slice: &mut [Person], index: usize) {
            let (four, function_provided, three) = indices!(data_slice, 4, index, 3);
            four.last = "Black".to_string();
            function_provided.first = "Jack".to_string();
            three.last = "Jones".to_string();
        }
        let slice = data.as_mut_slice();
        modify(slice, 1);
        assert_eq!(data[4].last, "Black");
        assert_eq!(data[1].first, "Jack");
        assert_eq!(data[3].last, "Jones");
    }

    // #[test]
    // fn graph_example(){
    //         struct Node {
    //             index: usize,
    //             name: String,
    //             edges: Vec<usize>,
    //         }

    //         let mut graph = vec![
    //             Node { index: 0, name: "Node 0".to_string(), edges: vec![1, 3] },
    //             Node { index: 1, name: "Node 1".to_string(), edges: vec![0, 2] },
    //             Node { index: 2, name: "Node 2".to_string(), edges: vec![1] },
    //             Node { index: 3, name: "Node 3".to_string(), edges: vec![2] },
    //         ];

    //         fn modify_graph(graph: &mut [Node], node_index: usize) {
    //             if let Some(node) = graph.get_mut(node_index) {
    //                 for &edge_index in &node.edges {
    //                     if let Some(adjacent_node) = graph.get_mut(edge_index) {
    //                         println!("Modifying edge from Node {} to Node {}.", node_index, edge_index);
    //                         // Example modification: rename nodes to indicate a processed connection
    //                         adjacent_node.name = format!("{} (connected from {})", adjacent_node.name, node_index);
    //                     }
    //                 }
    //             }
    //         }

    //         // Modify graph starting from node 0
    //         modify_graph(&mut graph, 0);

    //         for node in &graph {
    //             println!("Node {}: {}", node.index, node.name);
    //         }
    // }
}
