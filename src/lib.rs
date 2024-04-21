

/// A macro that returns mutable references for the requested indices.
/// Panics if any index is out of bounds or if any index is duplicated.
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

/// A macro that returns mutable references for the requested indices, assumes the requested indices are already ordered
/// smallest to largest. Panics if any index is out of bounds, if any index is duplicated, or if not sorted.
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

#[cfg(test)]
mod tests {
    
    #[test]
    fn indices_works() {
        let mut data = [5,4,3,2,1];
        let slice = data.as_mut_slice();
        let (one, two) = indices!(slice, 1, 3);
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 2);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5,10,3,20,1]);
    }

    #[test]
    fn indices_out_of_order() {
        let mut data = [5,4,3,2,1];
        let slice = data.as_mut_slice();
        let (one, two) = indices!(slice, 3, 1);
        assert_eq!(one, &mut 2);
        assert_eq!(two, &mut 4);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5,20,3,10,1]);
    }

    #[test]
    fn indices_more_than_two_indices() {
        let mut data = [5,4,3,2,1];
        let slice = data.as_mut_slice();
        let (one, two, three) = indices!(slice, 3, 1, 2);
        assert_eq!(one, &mut 2);
        assert_eq!(two, &mut 4);
        assert_eq!(three, &mut 3);
        *one = 10;
        *two = 20;
        *three = 30;
        assert_eq!(data, [5,20,30,10,1]);
    }

    //************************************************************************//

    #[test]
    fn indices_ordered_works() {
        let mut data = [5,4,3,2,1];
        let slice = data.as_mut_slice();
        let (one, two) = indices_ordered!(slice, 1, 3);
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 2);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5,10,3,20,1]);
    }

    // #[test]
    // fn indices_ordered_out_of_order() {
    //     let mut data = [5,4,3,2,1];
    //     let slice = data.as_mut_slice();
    //     let (one, two) = indices_ordered!(slice, 3, 1);
    //     assert_eq!(one, &mut 2);
    //     assert_eq!(two, &mut 4);
    //     *one = 10;
    //     *two = 20;
    //     assert_eq!(data, [5,20,3,10,1]);
    // }

    #[test]
    fn indices_ordered_more_than_two_indices() {
        let mut data = [5,4,3,2,1];
        let slice = data.as_mut_slice();
        let (one, two, three) = indices_ordered!(slice, 1, 2, 4);
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 3);
        assert_eq!(three, &mut 1);
        *one = 10;
        *two = 20;
        *three = 30;
        assert_eq!(data, [5,10,20,2,30]);
    }
}
