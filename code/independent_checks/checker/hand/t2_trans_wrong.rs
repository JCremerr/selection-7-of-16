#[allow(unused)]
pub const N: usize = 3;
#[allow(unused)]
pub const I: usize = 0;
#[allow(unused)]
pub fn run(input: [usize; 3]) -> usize {
    select_0(input)
}

/// n = 2, i = 0
fn select_5([a, b]: [usize; 2]) -> usize {
    a
}
/// n = 3, i = 0, a < c
fn select_2([a, b, c]: [usize; 3]) -> usize {
    if b < c {
        select_5([a, b])
    } else {
        select_5([a, b])
    }
}
/// n = 3, i = 0
fn select_0([a, b, c]: [usize; 3]) -> usize {
    if a < c {
        select_2([a, b, c])
    } else {
        select_2([c, b, a])
    }
}
