mod errors;

pub use errors::*;

/// Returns mutable references for the requested indices in the provided slice.
/// Panics if any index is out of bounds or duplicate indices.
#[inline(always)]
pub fn indices_slice<'a, T>(slice: &'a mut [T], indices: &[usize]) -> Vec<&'a mut T> {
    let slice_length = slice.len();
    let indices_length = indices.len();
    if slice_length == 0 {
        if indices_length != 0 {
            panic!("Requested indices but slice is empty.")
        }
        return Vec::new();
    }
    if indices_length == 0 {
        return Vec::new();
    }
    let mut check: Vec<usize> = indices.to_vec();
    insertion_sort(&mut check);
    let indices_len_minus_one = indices_length - 1;
    let slice_len_minus_one = slice_length - 1;
    for i in 0..indices_len_minus_one {
        if check[i] == check[i + 1] {
            panic!(
                "Duplicate indices are not allowed. Index `{}` was requested twice.",
                check[i]
            );
        }
    }
    if check[indices_len_minus_one] > slice_len_minus_one {
        panic!(
            "Index out of bounds. Requested index was `{}` while slice length was `{}`.",
            check[indices_len_minus_one],
            slice_len_minus_one + 1
        );
    }
    let mut vector: Vec<std::mem::MaybeUninit<*mut T>> =
        vec![std::mem::MaybeUninit::uninit(); indices_length];
    let ptr = slice.as_mut_ptr();
    unsafe {
        for (i, index) in indices.iter().enumerate() {
            vector[i].write(ptr.add(*index));
        }
        std::mem::transmute::<_, Vec<&'a mut T>>(vector)
    }
}

/// Returns mutable references for the requested indices in the provided slices.
/// Panics if any index is out of bounds or duplicate indices.
#[inline(always)]
pub fn indices_slices<'a, T, const N: usize>(
    slice: &'a mut [T],
    indices: [&[usize]; N],
) -> [Vec<&'a mut T>; N] {
    assert!(
        std::mem::size_of::<[std::mem::MaybeUninit<Vec<*mut T>>; N]>()
            == std::mem::size_of::<[Vec<&'a mut T>; N]>()
    );
    let slice_length = slice.len();
    let mut check: Vec<usize> = indices.concat();
    let indices_length = check.len();
    if slice_length == 0 {
        if indices_length != 0 {
            panic!("Requested indices but slice is empty.")
        }
        return unsafe { std::mem::zeroed() };
    }
    if indices_length == 0 {
        return unsafe { std::mem::zeroed() };
    }
    insertion_sort(&mut check);
    let indices_len_minus_one = indices_length - 1;
    let slice_len_minus_one = slice_length - 1;
    for i in 0..indices_len_minus_one {
        if check[i] == check[i + 1] {
            panic!(
                "Duplicate indices are not allowed. Index `{}` was requested twice.",
                check[i]
            );
        }
    }
    if check[indices_len_minus_one] > slice_len_minus_one {
        panic!(
            "Index out of bounds. Requested index was `{}` while slice length was `{}`.",
            check[indices_len_minus_one],
            slice_len_minus_one + 1
        );
    }
    let ptr = slice.as_mut_ptr();
    unsafe {
        let mut array: [std::mem::MaybeUninit<Vec<*mut T>>; N] =
            std::mem::MaybeUninit::uninit().assume_init();
        for (i, indice) in indices.iter().enumerate() {
            let mut out_vec = Vec::with_capacity(indice.len());
            for index in *indice {
                out_vec.push(ptr.add(*index));
            }
            array[i].write(out_vec);
        }
        std::mem::transmute_copy::<[std::mem::MaybeUninit<Vec<*mut T>>; N], [Vec<&'a mut T>; N]>(
            &array,
        )
    }
}

//************************************************************************//

/// Returns mutable references for the requested indices in the provided array.
/// Panics if any index is out of bounds or duplicate indices.
#[inline(always)]
pub fn indices_array<'a, T, const N: usize>(
    slice: &'a mut [T],
    indices: &[usize; N],
) -> [&'a mut T; N] {
    assert!(
        std::mem::size_of::<[std::mem::MaybeUninit<*mut T>; N]>()
            == std::mem::size_of::<[&'a mut T; N]>()
    );
    let slice_length = slice.len();
    let indices_length = N;
    if slice_length == 0 {
        if indices_length != 0 {
            panic!("Requested indices but slice is empty.")
        }
        return unsafe { std::mem::zeroed() };
    }
    if indices_length == 0 {
        return unsafe { std::mem::zeroed() };
    }
    let mut check: Vec<usize> = indices.to_vec();
    insertion_sort(&mut check);
    let indices_len_minus_one = indices_length - 1;
    let slice_len_minus_one = slice_length - 1;
    for i in 0..indices_len_minus_one {
        if check[i] == check[i + 1] {
            panic!(
                "Duplicate indices are not allowed. Index `{}` was requested twice.",
                check[i]
            );
        }
    }
    if check[indices_len_minus_one] > slice_len_minus_one {
        panic!(
            "Index out of bounds. Requested index was `{}` while slice length was `{}`.",
            check[indices_len_minus_one],
            slice_len_minus_one + 1
        );
    }

    let ptr = slice.as_mut_ptr();
    unsafe {
        let mut array: [std::mem::MaybeUninit<*mut T>; N] =
            std::mem::MaybeUninit::uninit().assume_init();
        for (i, index) in indices.iter().enumerate() {
            array[i].write(ptr.add(*index));
        }
        std::mem::transmute_copy::<[std::mem::MaybeUninit<*mut T>; N], [&'a mut T; N]>(&array)
    }
}

//************************************************************************//

/// Returns mutable references for the requested indices.
/// Panics if any index is out of bounds or duplicated.
#[macro_export]
macro_rules! indices {
    ($slice:expr, $( $index:expr ),+) => {{
        let slice = $slice;
        if slice.is_empty() {
            panic!("Requested indices but slice is empty.")
        }
        let mut indices = [$($index),*];
        $crate::insertion_sort(&mut indices);

        let indices_len_minus_one = indices.len() - 1;
        let slice_len_minus_one = slice.len() - 1;
        for i in 0..indices_len_minus_one {
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
/// Returns `TryIndicesError` if any index is out of bounds or duplicated.
#[macro_export]
macro_rules! try_indices {
    ($slice:expr, $( $index:expr ),+) => {{
        (|| {
        let slice = $slice;
        if slice.is_empty() {
            return Err($crate::TryIndicesError::IndexOutOfBounds);
        }
        let mut indices = [$($index),*];
        $crate::insertion_sort(&mut indices);

        let indices_len_minus_one = indices.len() - 1;
        let slice_len_minus_one = slice.len() - 1;
        for i in 0..indices_len_minus_one {
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
    ($slice:expr, $( $index:expr ),+) => {{
        let slice = $slice;
        if slice.is_empty() {
            panic!("Requested indices but slice is empty.")
        }
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
    ($slice:expr, $( $index:expr ),+) => {{
        (|| {
        let slice = $slice;
        if slice.is_empty() {
            return Err($crate::TryIndicesOrderedError::IndexOutOfBounds);
        }
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

#[inline(always)]
pub fn insertion_sort<T: PartialOrd>(s: &mut [T]) {
    for i in 1..s.len() {
        let mut j = i;
        while j > 0 && s[j - 1] > s[j] {
            s.swap(j - 1, j);
            j -= 1;
        }
    }
}

//************************************************************************//

#[cfg(test)]
mod tests {
    use crate::{
        indices_array, indices_slice, indices_slices, TryIndicesError, TryIndicesOrderedError,
    };

    #[test]
    fn indices_slice_works() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [one, two] = indices_slice(slice, &mut [1, 3]).try_into().unwrap();
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 2);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 10, 3, 20, 1]);
    }

    #[test]
    fn indices_slice_out_of_order() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [one, two] = indices_slice(slice, &mut [3, 1]).try_into().unwrap();
        assert_eq!(one, &mut 2);
        assert_eq!(two, &mut 4);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 20, 3, 10, 1]);
    }

    #[test]
    fn indices_slice_more_than_two_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [one, two, three] = indices_slice(slice, &mut [3, 1, 2]).try_into().unwrap();
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
    fn indices_slice_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let _result = indices_slice(slice, &mut [3, 3]);
    }

    #[should_panic]
    #[test]
    fn indices_slice_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let _result = indices_slice(slice, &mut [3, 5]);
    }

    #[should_panic]
    #[test]
    fn indices_slice_empty_requested_indices() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let _result = indices_slice(slice, &mut [3]);
    }

    #[test]
    fn indices_slice_empty_requested_empty() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let result = indices_slice(slice, &mut []);
        assert!(result.is_empty())
    }

    #[test]
    fn indices_slice_not_empty_slice_requested_empty() {
        let mut data: [i32; 1] = [1];
        let slice = data.as_mut_slice();
        let result = indices_slice(slice, &mut []);
        assert!(result.is_empty())
    }

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

    #[should_panic]
    #[test]
    fn indices_array_empty_requested_indices() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let _result = indices_array(slice, &mut [3]);
    }

    #[test]
    fn indices_array_empty_requested_empty() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let result = indices_array(slice, &mut []);
        assert!(result.is_empty())
    }

    #[test]
    fn indices_array_not_empty_slice_requested_empty() {
        let mut data: [i32; 1] = [1];
        let slice = data.as_mut_slice();
        let result = indices_array(slice, &mut []);
        assert!(result.is_empty())
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

    #[should_panic]
    #[test]
    fn indices_empty_requested_indices() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let _result = indices!(slice, 3);
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

    #[test]
    fn try_indices_empty_requested_indices() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 3);
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

    #[should_panic]
    #[test]
    fn indices_ordered_empty_requested_indices() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let _result = indices_ordered!(slice, 3);
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

    #[test]
    fn try_indices_ordered_empty_requested_indices() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let result = try_indices_ordered!(slice, 3);
        assert_eq!(result, Err(TryIndicesOrderedError::IndexOutOfBounds))
    }

    //************************************************************************//

    #[test]
    fn indices_slices_works() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [mut one, mut two] = indices_slices(slice, [&[1, 3], &[4, 2]]);
        assert_eq!(one, [&mut 4, &mut 2]);
        assert_eq!(two, [&mut 1, &mut 3]);
        *one[0] = 10;
        *two[0] = 20;
        assert_eq!(data, [5, 10, 3, 2, 20]);
    }

    #[test]
    fn indices_slices_more_than_two_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [one, two] = indices_slices(slice, [&mut [3, 1, 2], &mut [0]]);
        assert_eq!(one[0], &mut 2);
        assert_eq!(one[1], &mut 4);
        assert_eq!(one[2], &mut 3);
        assert_eq!(one.len(), 3);
        assert_eq!(two[0], &mut 5);
        assert_eq!(two.len(), 1);
    }

    #[should_panic]
    #[test]
    fn indices_slices_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [_one, _two] = indices_slices(slice, [&mut [3, 3], &[1, 2]]);
    }

    #[should_panic]
    #[test]
    fn indices_slices_duplicate_indices_different_slice() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [_one, _two] = indices_slices(slice, [&mut [3, 1], &mut [2, 3]]);
    }

    #[should_panic]
    #[test]
    fn indices_slices_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let [_one, _two] = indices_slices(slice, [&mut [3, 5], &mut [1, 0]]);
    }

    #[should_panic]
    #[test]
    fn indices_slices_empty_requested_indices() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let _result = indices_slices(slice, [&mut [3]]);
    }

    #[test]
    fn indices_slices_empty_requested_empty() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let result = indices_slices(slice, [&mut []]);
        assert_eq!(result.len(), 1);
        assert!(result[0].is_empty())
    }

    #[test]
    fn indices_slices_not_empty_slice_requested_empty() {
        let mut data: [i32; 1] = [1];
        let slice = data.as_mut_slice();
        let result = indices_slices(slice, [&mut []]);
        assert_eq!(result.len(), 1);
        assert!(result[0].is_empty())
    }
}

#[cfg(test)]
mod example_tests {
    use crate::indices_slices;

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

    #[test]
    fn graph_example() {
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
}
