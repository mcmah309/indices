use indices::indices_ordered;

fn main() {
    one();
    two();
    three();
    four();
    five();
}

fn one() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four,) = indices_ordered!(&mut slice, 4);
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}");
}

fn two() {
    let mut slice = [0, 1, 2, 3, 4];
    let (one, four) = indices_ordered!(&mut slice, 1, 4);
    slice[1] *= 2;
    *one *= 2;
    slice[1] *= 2;
    *one *= 2;
    println!("{one}, {four}");
}

fn three() {
    let mut slice = [0, 1, 2, 3, 4];
    let (one, two, four) = indices_ordered!(&mut slice, 1, 2, 4);
    slice[1] *= 2;
    *one *= 2;
    slice[1] *= 2;
    *one *= 2;
    println!("{one}, {two}, {four}");
}

fn four() {
    let mut slice = [0, 1, 2, 3, 4];
    let (one, two, three, four) = indices_ordered!(&mut slice, 1, 2, 3, 4);
    slice[1] *= 2;
    *one *= 2;
    slice[1] *= 2;
    *one *= 2;
    println!("{one}, {two}, {three}, {four}");
}

fn five() {
    let mut slice = [0, 1, 2, 3, 4];
    let (zero, one, two, three, four) = indices_ordered!(&mut slice, 0, 1, 2, 3, 4);
    slice[0] *= 2;
    *zero *= 2;
    slice[0] *= 2;
    *zero *= 2;
    println!("{zero}, {one}, {two}, {three}, {four}");
}
