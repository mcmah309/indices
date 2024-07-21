use indices::try_indices_ordered;

fn main() {
    one();
    two();
    three();
    four();
    five();
}

fn one() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four,) = try_indices_ordered!(&mut slice, 4).unwrap();
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}");
}

fn two() {
    let mut slice = [0, 1, 2, 3, 4];
    let (one, four) = try_indices_ordered!(&mut slice, 1, 4).unwrap();
    slice[1] *= 2;
    *one *= 2;
    slice[1] *= 2;
    *one *= 2;
    println!("{one}, {four}");
}

fn three() {
    let mut slice = [0, 1, 2, 3, 4];
    let (one, two, four) = try_indices_ordered!(&mut slice, 1, 2, 4).unwrap();
    slice[1] *= 2;
    *one *= 2;
    slice[1] *= 2;
    *one *= 2;
    println!("{one}, {two}, {four}");
}

fn four() {
    let mut slice = [0, 1, 2, 3, 4];
    let (one, two, three, four) = try_indices_ordered!(&mut slice, 1, 2, 3, 4).unwrap();
    slice[1] *= 2;
    *one *= 2;
    slice[1] *= 2;
    *one *= 2;
    println!("{one}, {two}, {three}, {four}");
}

fn five() {
    let mut slice = [0, 1, 2, 3, 4];
    let (zero, one, two, three, four) = try_indices_ordered!(&mut slice, 0, 1, 2, 3, 4).unwrap();
    slice[0] *= 2;
    *zero *= 2;
    slice[0] *= 2;
    *zero *= 2;
    println!("{zero}, {one}, {two}, {three}, {four}");
}
