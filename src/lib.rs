

/// A macro that returns mutable references for the requested indices.
/// Panics if any index is out of bounds or if any index is duplicated.
macro_rules! indices {
    ($slice:expr, $( $index:expr ),*) => {{
        let slice = $slice;
        let mut indices = [$($index),*];
        indices.sort();

        let args_len = indices.len();
        let slice_len = slice.len();
        for i in 0..args_len {
            if indices[i] > slice_len - 1 {
                panic!("Index out of bounds. Requested index was `{}` while slice length was `{}`.", indices[i], slice_len)
            }
            if i != args_len - 1 && indices[i] == indices[i + 1] { // todo replace with unchecked
                panic!("Duplicate indices are not allowed. Index `{}` was requested twice.", indices[i])
            }
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
    fn it_works() {
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
    fn out_of_order() {
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
    fn more_than_two_indices() {
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
}
