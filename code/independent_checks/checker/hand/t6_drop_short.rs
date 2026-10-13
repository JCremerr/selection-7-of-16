#[allow(unused)]
pub const N: usize = 3;
#[allow(unused)]
pub const I: usize = 1;
#[allow(unused)]
pub fn run(input: [usize; 3]) -> usize {
    select_0(input)
}
/// n = 2, i = 0
fn select_4([a, b]: [usize; 2]) -> usize {
    if a < b {
        a
    } else {
        b
    }
}
/// n = 3, i = 1
fn select_0([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_4([b, c])
    } else {
        select_4([a, c])
    }
}
