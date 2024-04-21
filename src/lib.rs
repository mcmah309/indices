

/// Returns mutable references for the requested indicies. Panics if out of bounds or duplicate requested indices.
pub fn two<T>(slice: &mut [T], index0: usize, index1: usize) -> (&mut T, &mut T) {
    let mut indices = [index0, index1];
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
    unsafe {
        let out0 = &mut *ptr.add(index0);
        let out1 = &mut *ptr.add(index1);
        return (out0, out1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut data = [5,4,3,2,1];
        let slice = data.as_mut_slice();
        let (one, two) = two(slice, 1, 3);
        assert_eq!(one, &mut 4);
        assert_eq!(two, &mut 2);
        *one = 10;
        *two = 20;
        assert_eq!(data, [5,10,3,20,1]);
    }

    // #[test]
    // fn it_works2() {
    //     let mut data = [5,4,3,2,1];
    //     let slice = data.as_mut_slice();
    //     let (one, two) = two(slice, 3, 3);
    //     println!("{}",one)
    // }
}
