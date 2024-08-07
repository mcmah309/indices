mod errors;

pub use errors::*;

/// Returns mutable references for the requested indices in the provided slice.
/// Panics if any index is out of bounds or duplicate indices.
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
pub fn indices_slices<'a, T, const N: usize>(
    slice: &'a mut [T],
    indices: [&[usize]; N],
) -> [Vec<&'a mut T>; N] {
    const {
        assert!(
            std::mem::size_of::<[std::mem::MaybeUninit<Vec<*mut T>>; N]>()
                == std::mem::size_of::<[Vec<*mut T>; N]>()
        );
    }
    if N == 0 {
        return unsafe { std::mem::zeroed() };
    }
    let slice_length = slice.len();
    let mut all_requested_indices: Vec<usize> = indices.concat();
    let all_requested_indices_length = all_requested_indices.len();
    if slice_length == 0 {
        if all_requested_indices_length != 0 {
            panic!("Requested indices but slice is empty.")
        }
        unsafe {
            let mut array: [std::mem::MaybeUninit<Vec<*mut T>>; N] =
                std::mem::MaybeUninit::uninit().assume_init();
            for i in 0..N {
                let out_vec = Vec::with_capacity(0);
                array[i].write(out_vec);
            }
            return std::mem::transmute_copy::<
                [std::mem::MaybeUninit<Vec<*mut T>>; N],
                [Vec<&'a mut T>; N],
            >(&array);
        }
    }
    if all_requested_indices_length == 0 {
        unsafe {
            let mut array: [std::mem::MaybeUninit<Vec<*mut T>>; N] =
                std::mem::MaybeUninit::uninit().assume_init();
            for i in 0..N {
                let out_vec = Vec::with_capacity(0);
                array[i].write(out_vec);
            }
            return std::mem::transmute_copy::<
                [std::mem::MaybeUninit<Vec<*mut T>>; N],
                [Vec<&'a mut T>; N],
            >(&array);
        }
    }
    insertion_sort(&mut all_requested_indices);
    let indices_len_minus_one = all_requested_indices_length - 1;
    let slice_len_minus_one = slice_length - 1;
    for i in 0..indices_len_minus_one {
        if all_requested_indices[i] == all_requested_indices[i + 1] {
            panic!(
                "Duplicate indices are not allowed. Index `{}` was requested twice.",
                all_requested_indices[i]
            );
        }
    }
    if all_requested_indices[indices_len_minus_one] > slice_len_minus_one {
        panic!(
            "Index out of bounds. Requested index was `{}` while slice length was `{}`.",
            all_requested_indices[indices_len_minus_one],
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
pub fn indices_array<'a, T, const N: usize>(
    slice: &'a mut [T],
    indices: &[usize; N],
) -> [&'a mut T; N] {
    const {
        assert!(
            std::mem::size_of::<[std::mem::MaybeUninit<*mut T>; N]>()
                == std::mem::size_of::<[&'a mut T; N]>()
        );
    }
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

#[doc(hidden)]
#[macro_export]
macro_rules! to_type {
    ( $t:expr ) => { &'a mut T };
}

/// Returns mutable references for the requested indices.
/// Panics if any index is out of bounds or duplicated.
#[macro_export]
macro_rules! indices {
    ($slice:expr, $index1:expr) => {{
        (&mut $slice[$index1],)
    }};

    ($slice:expr, $index1:expr, $index2:expr) => {{
        #[inline(always)]
        fn func<T>(slice: &mut [T], one: usize, two: usize) -> (&mut T, &mut T) {
            if one == two {
                panic!("Duplicate indices are not allowed. Index `{}` was requested twice.", one)
            }
            let slice_len = slice.len();
            if one >= slice_len || two >= slice_len {
                panic!("Index out of bounds.")
            }
            let ptr = slice.as_mut_ptr();
            unsafe {
                (&mut *ptr.add(one), &mut *ptr.add(two))
            }
        }
        func($slice, $index1, $index2)
    }};

    ($slice:expr, $index1:expr, $index2:expr, $index3:expr) => {{
        #[inline(always)]
        fn func<T>(slice: &mut [T], one: usize, two: usize, three: usize) -> (&mut T, &mut T, &mut T) {
            if one == two || one == three || two == three {
                panic!("Duplicate indices are not allowed.")
            }
            let slice_len = slice.len();
            if one >= slice_len || two >= slice_len || three >= slice_len {
                panic!("Index out of bounds.")
            }
            let ptr = slice.as_mut_ptr();
            unsafe {
                (&mut *ptr.add(one), &mut *ptr.add(two), &mut *ptr.add(three))
            }
        }
        func($slice, $index1, $index2, $index3)
    }};

    ($slice:expr, $index1:expr, $index2:expr, $index3:expr, $index4:expr) => {{
        #[inline(always)]
        fn func<T>(slice: &mut [T], one: usize, two: usize, three: usize, four: usize) -> (&mut T, &mut T, &mut T, &mut T) {
            if one == two || one == three || one == four || two == three || two == four || three == four {
                panic!("Duplicate indices are not allowed.")
            }
            let slice_len = slice.len();
            if one >= slice_len || two >= slice_len || three >= slice_len || four >= slice_len {
                panic!("Index out of bounds.")
            }
            let ptr = slice.as_mut_ptr();
            unsafe {
                (&mut *ptr.add(one), &mut *ptr.add(two), &mut *ptr.add(three), &mut *ptr.add(four))
            }
        }
        func($slice, $index1, $index2, $index3, $index4)
    }};

    ($slice:expr, $( $index:expr ),+) => {{
        #[inline(always)]
        fn func<'a, 'b, T>(slice: &'a mut [T], indices: &'b mut [usize]) -> ($($crate::to_type!($index)),+) {
            if slice.is_empty() {
                panic!("Requested indices but slice is empty.")
            }
            $crate::insertion_sort(indices);

            let indices_len_minus_one = indices.len() - 1;
            let slice_len_minus_one = slice.len() - 1;
            for i in 0..indices_len_minus_one {
                if indices[i] == indices[i + 1] {
                    panic!("Duplicate indices are not allowed. Index `{}` was requested twice.", indices[i])
                }
            }
            if indices[indices_len_minus_one] > slice_len_minus_one {
                panic!(
                    "Index out of bounds. Requested index was `{}` while slice length was `{}`.",
                    indices[indices_len_minus_one], slice_len_minus_one + 1
                )
            }

            let ptr = slice.as_mut_ptr();
            (
                $(unsafe { &mut *ptr.add($index) },)*
            )
        }
        let mut indices = [$($index),*];
        func($slice, &mut indices)
    }};
}

/// Returns mutable references for the requested indices.
/// Returns `TryIndicesError` if any index is out of bounds or duplicated.
#[macro_export]
macro_rules! try_indices {
    ($slice:expr, $index1:expr) => {{
        $slice.get_mut($index1).map(|e| (e,)).ok_or($crate::TryIndicesError::IndexOutOfBounds)
    }};

    ($slice:expr, $index1:expr, $index2:expr) => {{
        #[inline(always)]
        fn func<T>(slice: &mut [T], one: usize, two: usize) -> Result<(&mut T, &mut T), $crate::TryIndicesError> {
            if one == two {
                return Err($crate::TryIndicesError::DuplicateIndex);
            }
            let slice_len = slice.len();
            if one >= slice_len || two >= slice_len {
                return Err($crate::TryIndicesError::IndexOutOfBounds);
            }
            let ptr = slice.as_mut_ptr();
            unsafe {
                Ok((&mut *ptr.add(one), &mut *ptr.add(two)))
            }
        }
        func($slice, $index1, $index2)
    }};

    ($slice:expr, $index1:expr, $index2:expr, $index3:expr) => {{
        #[inline(always)]
        fn func<T>(slice: &mut [T], one: usize, two: usize, three: usize) -> Result<(&mut T, &mut T, &mut T), $crate::TryIndicesError> {
            if one == two || one == three || two == three {
                return Err($crate::TryIndicesError::DuplicateIndex);
            }
            let slice_len = slice.len();
            if one >= slice_len || two >= slice_len || three >= slice_len {
                return Err($crate::TryIndicesError::IndexOutOfBounds);
            }
            let ptr = slice.as_mut_ptr();
            unsafe {
                Ok((&mut *ptr.add(one), &mut *ptr.add(two), &mut *ptr.add(three)))
            }
        }
        func($slice, $index1, $index2, $index3)
    }};

    ($slice:expr, $index1:expr, $index2:expr, $index3:expr, $index4:expr) => {{
        #[inline(always)]
        fn func<T>(slice: &mut [T], one: usize, two: usize, three: usize, four: usize) -> Result<(&mut T, &mut T, &mut T, &mut T), $crate::TryIndicesError> {
            if one == two || one == three || one == four || two == three || two == four || three == four {
                return Err($crate::TryIndicesError::DuplicateIndex);
            }
            let slice_len = slice.len();
            if one >= slice_len || two >= slice_len || three >= slice_len || four >= slice_len {
                return Err($crate::TryIndicesError::IndexOutOfBounds);
            }
            let ptr = slice.as_mut_ptr();
            unsafe {
                Ok((&mut *ptr.add(one), &mut *ptr.add(two), &mut *ptr.add(three), &mut *ptr.add(four)))
            }
        }
        func($slice, $index1, $index2, $index3, $index4)
    }};

    ($slice:expr, $( $index:expr ),+) => {{
        #[inline(always)]
        fn func<'a, 'b, T>(slice: &'a mut [T], indices: &'b mut [usize]) -> Result<($($crate::to_type!($index)),+), $crate::TryIndicesError> {
            if slice.is_empty() {
                return Err($crate::TryIndicesError::IndexOutOfBounds);
            }
            $crate::insertion_sort(indices);

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
        }
        let mut indices = [$($index),*];
        func($slice, &mut indices)
    }};
}

/// Returns mutable references for the requested indices
/// Slightly more efficient than `indices!` since assumes the requested indices are already ordered smallest to largest.
/// Panics if the requested indices are not smallest to largest, or if any index is duplicated or out of bounds.
#[macro_export]
macro_rules! indices_ordered {
    ($slice:expr, $index1:expr) => {{
        (&mut $slice[$index1],)
    }};

    ($slice:expr, $( $index:expr ),+) => {{
        #[inline(always)]
        fn func<'a, 'b, T>(slice: &'a mut [T], indices: &'b [usize]) -> ($($crate::to_type!($index)),+) {
            if slice.is_empty() {
                panic!("Requested indices but slice is empty.");
            }

            let indices_len_minus_one = indices.len() - 1;
            let slice_len_minus_one = slice.len() - 1;

            for i in 0..indices_len_minus_one {
                if indices[i] > slice_len_minus_one {
                    panic!("Index out of bounds. Requested index was `{}` while slice length was `{}`.", indices[i], slice_len_minus_one + 1);
                }
                if indices[i] >= indices[i + 1] {
                    panic!("Indices not sorted or duplicate indices detected.");
                }
            }
            if indices[indices_len_minus_one] > slice_len_minus_one {
                panic!("Index out of bounds. Requested index was `{}` while slice length was `{}`.", indices[indices_len_minus_one], slice_len_minus_one + 1);
            }

            let ptr = slice.as_mut_ptr();
            (
                $(unsafe { &mut *ptr.add($index) },)*
            )
        }
        let indices = [$($index),*];
        func($slice, &indices)
    }};
}

/// Returns mutable references for the requested indices.
/// Slightly more efficient than `try_indices!` since assumes the requested indices are already ordered smallest to largest.
/// Returns `TryOrderedIndicesError` if the requested indices are not smallest to largest, or if any index is duplicated or out of bounds.
#[macro_export]
macro_rules! try_indices_ordered {
    ($slice:expr, $index1:expr) => {{
        $slice.get_mut($index1).map(|e| (e,)).ok_or($crate::TryIndicesOrderedError::IndexOutOfBounds)
    }};

    ($slice:expr, $( $index:expr ),+) => {{
        #[inline(always)]
        fn func<'a, 'b, T>(slice: &'a mut [T], indices: &'b [usize]) -> Result<($($crate::to_type!($index)),+), $crate::TryIndicesOrderedError> {
            if slice.is_empty() {
                return Err($crate::TryIndicesOrderedError::IndexOutOfBounds);
            }

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
        }
        let indices = [$($index),*];
        func($slice, &indices)
    }};
}

//************************************************************************//

#[doc(hidden)]
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

    #[should_panic]
    #[test]
    fn indices_empty() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let _result = indices!(slice, 3);
    }

    #[test]
    fn indices_1() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one,) = indices!(slice, 3);
        assert_eq!(one, &mut 2);
        *one = 10;
        assert_eq!(data, [5, 4, 3, 10, 1]);
    }

    #[should_panic]
    #[test]
    fn indices_1_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one,) = indices!(slice, 5);
    }

    #[test]
    fn indices_2() {
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
    fn indices_2_out_of_order() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one, two) = indices!(slice, 3, 1);
        assert_eq!(one, &mut 2);
        assert_eq!(two, &mut 4);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5, 20, 3, 10, 1]);
    }

    #[should_panic]
    #[test]
    fn indices_2_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two) = indices!(slice, 3, 3);
    }

    #[should_panic]
    #[test]
    fn indices_2_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two) = indices!(slice, 3, 5);
    }

    #[test]
    fn indices_3() {
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
    fn indices_3_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two, _three) = indices!(slice, 1, 3, 5);
    }

    #[should_panic]
    #[test]
    fn indices_3_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two, _three) = indices!(slice, 1, 3, 3);
    }

    #[test]
    fn indices_4() {
        let mut data = [5, 4, 3, 2, 1, 6];
        let slice = data.as_mut_slice();
        let (one, two, three, four) = indices!(slice, 0, 2, 4, 5);
        assert_eq!(one, &mut 5);
        assert_eq!(two, &mut 3);
        assert_eq!(three, &mut 1);
        assert_eq!(four, &mut 6);
        *one = 10;
        *two = 20;
        *three = 30;
        *four = 40;
        assert_eq!(data, [10, 4, 20, 2, 30, 40]);
    }

    #[should_panic]
    #[test]
    fn indices_4_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two, _three, _four) = indices!(slice, 1, 3, 4, 5);
    }

    #[should_panic]
    #[test]
    fn indices_4_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two, _three, _four) = indices!(slice, 1, 3, 4, 3);
    }

    #[test]
    fn indices_5() {
        let mut data = [5, 4, 3, 2, 1, 6];
        let slice = data.as_mut_slice();
        let (one, two, three, four, five) = indices!(slice, 0, 1, 2, 4, 5);
        assert_eq!(one, &mut 5);
        assert_eq!(two, &mut 4);
        assert_eq!(three, &mut 3);
        assert_eq!(four, &mut 1);
        assert_eq!(five, &mut 6);
        *one = 10;
        *two = 20;
        *three = 30;
        *four = 40;
        *five = 50;
        assert_eq!(data, [10, 20, 30, 2, 40, 50]);
    }

    #[should_panic]
    #[test]
    fn indices_5_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two, _three, _four, _five) = indices!(slice, 1, 2, 3, 4, 5);
    }

    #[should_panic]
    #[test]
    fn indices_5_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (_one, _two, _three, _four, _five) = indices!(slice, 1, 2, 3, 4, 3);
    }

    #[test]
    fn indices_can_return_mut_from_scope() {
        let mut data = [0, 1, 2, 3, 4];
        let (two, four) = indices_scope_helper(&mut data);
        *two = 200;
        *four = 400;
        assert_eq!(data[2], 200);
        assert_eq!(data[4], 400);
    }

    fn indices_scope_helper(data: &mut [i32]) -> (&mut i32, &mut i32) {
        let (two, four) = indices!(data, 2, 4);
        (two, four)
    }

    //************************************************************************//

    #[test]
    fn try_indices_empty() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 3);
        assert_eq!(result, Err(TryIndicesError::IndexOutOfBounds))
    }

    #[test]
    fn try_indices_1() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let (one,) = try_indices!(slice, 3).unwrap();
        assert_eq!(one, &mut 2);
        *one = 10;
        assert_eq!(data, [5, 4, 3, 10, 1]);
    }

    #[test]
    fn try_indices_1_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 5);
        assert_eq!(result, Err(TryIndicesError::IndexOutOfBounds))
    }

    #[test]
    fn try_indices_2() {
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
    fn try_indices_2_out_of_order() {
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
    fn try_indices_2_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 3, 3);
        assert_eq!(result, Err(TryIndicesError::DuplicateIndex))
    }

    #[test]
    fn try_indices_2_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 3, 5);
        assert_eq!(result, Err(TryIndicesError::IndexOutOfBounds))
    }

    #[test]
    fn try_indices_3() {
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
    fn try_indices_3_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 1, 3, 5);
        assert_eq!(result, Err(TryIndicesError::IndexOutOfBounds))
    }

    #[test]
    fn try_indices_3_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 1, 3, 3);
        assert_eq!(result, Err(TryIndicesError::DuplicateIndex))
    }

    #[test]
    fn try_indices_4() {
        let mut data = [5, 4, 3, 2, 1, 6];
        let slice = data.as_mut_slice();
        let (one, two, three, four) = try_indices!(slice, 0, 2, 4, 5).unwrap();
        assert_eq!(one, &mut 5);
        assert_eq!(two, &mut 3);
        assert_eq!(three, &mut 1);
        assert_eq!(four, &mut 6);
        *one = 10;
        *two = 20;
        *three = 30;
        *four = 40;
        assert_eq!(data, [10, 4, 20, 2, 30, 40]);
    }

    #[test]
    fn try_indices_4_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 1, 3, 4, 5);
        assert_eq!(result, Err(TryIndicesError::IndexOutOfBounds))
    }

    #[test]
    fn try_indices_4_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 1, 3, 4, 3);
        assert_eq!(result, Err(TryIndicesError::DuplicateIndex))
    }

    #[test]
    fn try_indices_5() {
        let mut data = [5, 4, 3, 2, 1, 6];
        let slice = data.as_mut_slice();
        let (one, two, three, four, five) = try_indices!(slice, 0, 1, 2, 4, 5).unwrap();
        assert_eq!(one, &mut 5);
        assert_eq!(two, &mut 4);
        assert_eq!(three, &mut 3);
        assert_eq!(four, &mut 1);
        assert_eq!(five, &mut 6);
        *one = 10;
        *two = 20;
        *three = 30;
        *four = 40;
        *five = 50;
        assert_eq!(data, [10, 20, 30, 2, 40, 50]);
    }

    #[test]
    fn try_indices_5_out_of_bounds() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 1, 2, 3, 4, 5);
        assert_eq!(result, Err(TryIndicesError::IndexOutOfBounds))
    }

    #[test]
    fn try_indices_5_duplicate_indices() {
        let mut data = [5, 4, 3, 2, 1];
        let slice = data.as_mut_slice();
        let result = try_indices!(slice, 1, 2, 3, 4, 3);
        assert_eq!(result, Err(TryIndicesError::DuplicateIndex))
    }

    #[test]
    fn try_indices_can_return_mut_from_scope() {
        let mut data = [0, 1, 2, 3, 4];
        let (two, four) = try_indices_scope_helper(&mut data).unwrap();
        *two = 200;
        *four = 400;
        assert_eq!(data[2], 200);
        assert_eq!(data[4], 400);
    }

    fn try_indices_scope_helper(data: &mut [i32]) -> Result<(&mut i32, &mut i32), TryIndicesError> {
        let (two, four) = try_indices!(data, 2, 4)?;
        Ok((two, four))
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

    #[test]
    fn indices_ordered_can_return_mut_from_scope() {
        let mut data = [0, 1, 2, 3, 4];
        let (two, four) = indices_ordered_scope_helper(&mut data);
        *two = 200;
        *four = 400;
        assert_eq!(data[2], 200);
        assert_eq!(data[4], 400);
    }

    fn indices_ordered_scope_helper(data: &mut [i32]) -> (&mut i32, &mut i32) {
        let (two, four) = indices_ordered!(data, 2, 4);
        (two, four)
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

    #[test]
    fn try_indices_ordered_can_return_mut_from_scope() {
        let mut data = [0, 1, 2, 3, 4];
        let (two, four) = try_indices_ordered_scope_helper(&mut data).unwrap();
        *two = 200;
        *four = 400;
        assert_eq!(data[2], 200);
        assert_eq!(data[4], 400);
    }

    fn try_indices_ordered_scope_helper(
        data: &mut [i32],
    ) -> Result<(&mut i32, &mut i32), TryIndicesOrderedError> {
        let (two, four) = try_indices_ordered!(data, 2, 4)?;
        Ok((two, four))
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

    #[test]
    fn indices_slices_data_not_empty_and_indices_requested_has_indices() {
        let mut data: [i32; 1] = [1];
        let slice = data.as_mut_slice();
        let result = indices_slices(slice, [&mut [0]]);
        assert_eq!(result.len(), 1);
        assert_eq!(*result[0][0], 1)
    }

    #[should_panic]
    #[test]
    fn indices_slices_data_empty_and_indices_requested_has_indices() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let _result = indices_slices(slice, [&mut [0]]);
    }

    #[test]
    fn indices_slices_data_not_empty_and_indices_requested_has_no_indices() {
        let mut data: [i32; 1] = [1];
        let slice = data.as_mut_slice();
        let result = indices_slices(slice, [&mut []]);
        assert_eq!(result.len(), 1);
        assert!(result[0].is_empty())
    }

    #[test]
    fn indices_slices_data_empty_and_indices_requested_has_no_indices() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let result = indices_slices(slice, [&mut []]);
        assert_eq!(result.len(), 1);
        assert!(result[0].is_empty())
    }

    #[test]
    fn indices_slices_data_not_empty_and_indices_requested_empty() {
        let mut data: [i32; 1] = [1];
        let slice = data.as_mut_slice();
        let result = indices_slices(slice, []);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn indices_slices_data_empty_and_indices_requested_empty() {
        let mut data: [i32; 0] = [];
        let slice = data.as_mut_slice();
        let result = indices_slices(slice, []);
        assert_eq!(result.len(), 0);
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
}
