use indices::try_indices;

fn main() {
    one();
    two();
    three();
    four();
    five();
}

fn one() -> Result<&'static mut i32, indices::TryIndicesError> {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (four,) = try_indices!(slice, 4)?;
    Ok(four)
}

fn two() -> Result<(&'static mut i32, &'static mut i32), indices::TryIndicesError> {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (one, four) = try_indices!(slice, 1, 4)?;
    Ok((one, four))
}

fn three() -> Result<&'static mut i32, indices::TryIndicesError> {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (one, _two, _three) = try_indices!(slice, 1, 2, 3)?;
    Ok(one)
}

fn four() -> Result<(&'static mut i32, &'static mut i32, &'static mut i32, &'static mut i32), indices::TryIndicesError> {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (one, two, three, four) = try_indices!(slice, 1, 2, 3, 4)?;
    Ok((one, two, three, four))
}

fn five() -> Result<(&'static mut i32, &'static mut i32, &'static mut i32, &'static mut i32, &'static mut i32), indices::TryIndicesError> {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (zero, one, two, three, four) = try_indices!(slice, 0, 1, 2, 3, 4)?;
    Ok((zero, one, two, three, four))
}
