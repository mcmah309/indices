

pub fn two<T>(slice: &mut [T], in_index1: usize, in_index2: usize) -> (&mut T, &mut T) {
    let mut in_indices = [in_index1, in_index2];
    let mut out_indices = [0, 1];
    // in_indices.sort();
    // let args_len = in_indices.len();
    // let slice_len = slice.len();
    // for i in 0..args_len {
    //     if in_indices[i] >= slice_len {
    //         panic!("")
    //     }
    //     if i != args_len - 1 && in_indices[i] == in_indices[i + 1] {
    //         panic!("")
    //     }
    // }
    let ptr = slice.as_mut_ptr();
    unsafe {
        let out0 = &mut *ptr.add(in_indices[out_indices[0]]);
        let out1 = &mut *ptr.add(in_indices[out_indices[1]]);
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
        let result = two(slice, 1, 3);
        assert_eq!(result, (&mut 4,&mut 2));
    }
}
