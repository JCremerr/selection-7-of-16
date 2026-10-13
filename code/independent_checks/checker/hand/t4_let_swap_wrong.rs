#[allow(unused)]
pub const N: usize = 2;
#[allow(unused)]
pub const I: usize = 0;
#[allow(unused)]
pub fn run(input: [usize; 2]) -> usize {
    select_0(input)
}
/// n = 2, i = 0, a < b
fn select_1([a, b]: [usize; 2]) -> usize {
    a
}
/// n = 2, i = 0
fn select_0([a, b]: [usize; 2]) -> usize {
    let (b, a) = if a < b {
        (a, b)
    } else {
        (b, a)
    };
    select_1([a, b])
}
