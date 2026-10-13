#[allow(unused)]
pub const N: usize = 3;
#[allow(unused)]
pub const I: usize = 0;
#[allow(unused)]
pub fn run(input: [usize; 3]) -> usize {
    select_0(input)
}
/// n = 1
fn select_1([a]: [usize; 1]) -> usize {
    a
}
/// n = 3, i = 0, a < b
fn select_3([a, b, c]: [usize; 3]) -> usize {
    if a < c {
        select_1([a])
    } else {
        select_1([c])
    }
}
/// n = 3, i = 0
fn select_0([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_3([a, b, c])
    } else {
        select_3([b, a, c])
    }
}
