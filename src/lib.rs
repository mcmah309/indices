use error_set::error_set;

error_set! {
    /// The error type returned from the `try_indices!` macro.
    #[derive(Copy,Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
    TryIndicesError = {
        /// The requested index is larger than the length of the input slice.
        IndexOutOfBounds,
        /// The index has been requested twice.
        DuplicateIndex
    };
    /// The error type returned from the `try_indices_ordered!` macro.
    #[derive(Copy,Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
    TryIndicesOrderedError = {
        /// The requested index is larger than the length of the input slice.
        IndexOutOfBounds,
        /// The input indices are either not sorted or out of bounds.
        InvalidIndex,
    };
}

/// Returns mutable references for the requested indices.
/// Panics if any index is out of bounds or duplicated.
#[macro_export]
macro_rules! indices {
    ($slice:expr, $( $index:expr ),*) => {{
        let slice = $slice;
        let mut indices = [$($index),*];
        indices.sort();

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
        indices.sort();

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

#[cfg(test)]
mod tests {
    use crate::{TryIndicesError, TryIndicesOrderedError};

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
