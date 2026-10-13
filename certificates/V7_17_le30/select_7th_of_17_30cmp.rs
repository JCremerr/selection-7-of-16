#[allow(unused)]
pub const N: usize = 17;
#[allow(unused)]
pub const I: usize = 6;

#[allow(unused)]
pub fn run(input: [usize; 17]) -> usize {
    select_0(input)
}

/// n = 1
fn select_1([a]: [usize; 1]) -> usize {
    a
}
/// n = 2, i = 0
fn select_2([a, b]: [usize; 2]) -> usize {
    if a < b {
        select_1([a])
    } else {
        select_1([b])
    }
}
/// n = 3, i = 1, b < c
fn select_3([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_2([a, c])
    }
}
/// n = 4, i = 1, b < c, b < d
fn select_4([a, b, c, d]: [usize; 4]) -> usize {
    if c < a {
        select_2([c, d])
    } else {
        select_3([a, b, d])
    }
}
/// n = 5, i = 1, b < c, b < d, b < e
fn select_5([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_4([a, b, c, e])
    } else {
        select_4([a, b, d, e])
    }
}
/// n = 6, i = 1, b < f, c < d, c < e
fn select_6([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < c {
        select_4([a, b, c, f])
    } else {
        select_5([a, c, b, d, e])
    }
}
/// n = 8, i = 2, a < d, a < f, b < c, b < e, e < g, f < h
fn select_7([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_6([d, f, b, c, e, h])
    } else {
        select_6([c, e, a, d, f, g])
    }
}
/// n = 6, i = 2, a < d, a < e, a < f, b < c, b < e, b < f
fn select_8([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_4([c, a, e, f])
    } else {
        select_4([d, b, e, f])
    }
}
/// n = 7, i = 2, a < d, a < g, b < c, b < e, b < g, e < f
fn select_9([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_4([c, e, a, f])
    } else {
        select_8([a, b, c, d, e, g])
    }
}
/// n = 9, i = 2, a < d, a < f, a < i, b < c, b < e, b < i, e < g, f < h
fn select_10([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_9([a, b, c, d, e, g, i])
    } else {
        select_9([b, a, d, c, f, h, i])
    }
}
/// n = 9, i = 2, a < f, b < e, b < i, c < d, c < g, c < i, g < h
fn select_11([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_7([a, c, d, f, g, b, h, e])
    } else {
        select_10([b, c, d, e, g, a, h, f, i])
    }
}
/// n = 5, i = 2, a < c, b < d, d < e
fn select_12([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < a {
        select_2([a, e])
    } else {
        select_3([c, b, d])
    }
}
/// n = 6, i = 2, a < c, a < f, b < d, b < f, d < e
fn select_13([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if f < d {
        select_3([c, b, f])
    } else {
        select_12([a, b, c, d, e])
    }
}
/// n = 7, i = 2, a < e, b < c, b < d, d < f, e < g
fn select_14([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_4([c, d, a, f])
    } else {
        select_13([b, a, c, e, g, d])
    }
}
/// n = 2, i = 1
fn select_15([a, b]: [usize; 2]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_1([a])
    }
}
/// n = 4, i = 2, b < d, c < d
fn select_16([a, b, c, d]: [usize; 4]) -> usize {
    if b < a {
        select_3([a, c, d])
    } else {
        select_15([b, c])
    }
}
/// n = 5, i = 2, c < d, d < e
fn select_17([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < d {
        select_16([b, a, c, d])
    } else {
        select_4([b, d, a, e])
    }
}
/// n = 7, i = 3, a < e, b < e, b < f, c < d, c < f, f < g
fn select_18([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_17([a, d, b, f, g])
    } else {
        select_12([a, c, e, f, g])
    }
}
/// n = 9, i = 3, a < e, a < g, b < d, b < f, c < e, c < f, c < g, f < h, g < i
fn select_19([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_14([b, a, e, g, f, i, h])
    } else {
        select_18([a, c, b, d, e, f, h])
    }
}
/// n = 10, i = 3, a < g, b < d, b < h, b < j, c < e, c < g, c < h, e < f, e < j, h < i
fn select_20([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_10([b, e, f, d, a, h, g, i, j])
    } else {
        select_19([a, b, c, d, g, h, e, i, f])
    }
}
/// n = 10, i = 3, b < e, b < h, b < j, c < d, c < f, c < h, f < g, f < j, h < i
fn select_21([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_11([d, f, b, e, g, a, h, i, j])
    } else {
        select_20([a, b, c, e, f, g, d, h, i, j])
    }
}
/// n = 3, i = 0
fn select_22([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_2([a, c])
    } else {
        select_2([b, c])
    }
}
/// n = 4, i = 0
fn select_23([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_22([a, c, d])
    } else {
        select_22([b, c, d])
    }
}
/// n = 5, i = 0
fn select_24([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_23([a, c, d, e])
    } else {
        select_23([b, c, d, e])
    }
}
/// n = 8, i = 1, a < g, a < h, b < c, b < d, b < e, b < f
fn select_25([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_22([b, g, h])
    } else {
        select_24([a, c, d, e, f])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < i, d < e, d < f, d < g, d < h
fn select_26([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_24([a, e, f, g, h])
    } else {
        select_4([c, b, d, i])
    }
}
/// n = 10, i = 2, a < c, b < d, b < e, d < j, e < f, e < g, e < h, e < i
fn select_27([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_25([d, e, f, g, h, i, a, j])
    } else {
        select_26([a, b, c, e, f, g, h, i, d])
    }
}
/// n = 10, i = 2, c < d, c < e, d < j, e < f, e < g, e < h, e < i
fn select_28([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_27([a, c, b, d, e, f, g, h, i, j])
    } else {
        select_27([b, c, a, d, e, f, g, h, i, j])
    }
}
/// n = 7, i = 1, a < f, a < g, b < c, b < d, b < e
fn select_29([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_22([b, f, g])
    } else {
        select_23([a, c, d, e])
    }
}
/// n = 8, i = 1, a < f, a < g, a < h, b < c, b < d, b < e
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_23([b, f, g, h])
    } else {
        select_23([a, c, d, e])
    }
}
/// n = 9, i = 1, a < i, b < g, b < h, c < d, c < e, c < f
fn select_31([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_29([a, c, d, e, f, b, i])
    } else {
        select_30([b, c, d, e, f, a, g, h])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, d < e, d < f, d < g
fn select_32([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_23([a, e, f, g])
    } else {
        select_4([c, b, d, h])
    }
}
/// n = 9, i = 2, a < i, b < h, c < d, c < h, c < i, d < e, d < f, d < g
fn select_33([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_32([a, c, b, d, e, f, g, i])
    } else {
        select_32([b, c, a, d, e, f, g, h])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < j, d < i, e < f, e < g, e < h
fn select_34([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_31([b, d, e, f, g, h, a, i, j])
    } else {
        select_33([a, b, c, e, f, g, h, j, d])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_35([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_34([a, d, b, f, g, h, i, j, k, e])
    } else {
        select_34([a, e, c, f, g, h, i, j, k, d])
    }
}
/// n = 12, i = 3, b < e, b < h, b < k, b < l, c < d, c < f, c < h, f < g, f < k, f < l, g < i, h < j
fn select_36([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_28([a, d, f, g, b, e, h, k, l, i])
    } else {
        select_35([a, b, c, d, e, h, f, g, k, l, j])
    }
}
/// n = 13, i = 3, b < e, b < f, b < j, b < m, c < d, c < g, c < j, f < h, g < i, g < m, i < k, j < l
fn select_37([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_21([a, c, b, e, d, f, h, j, l, g])
    } else {
        select_36([a, b, c, d, e, g, i, j, k, l, f, m])
    }
}
/// n = 4, i = 1, a < d, b < c
fn select_38([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_2([b, d])
    } else {
        select_2([a, c])
    }
}
/// n = 5, i = 1, a < e, b < c, b < d
fn select_39([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < a {
        select_2([c, d])
    } else {
        select_38([a, b, d, e])
    }
}
/// n = 6, i = 1, a < f, b < c, b < d, b < e
fn select_40([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_39([a, b, c, e, f])
    } else {
        select_39([a, b, d, e, f])
    }
}
/// n = 8, i = 2, a < d, b < c, b < e, e < f, e < g, e < h
fn select_41([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < a {
        select_40([c, e, f, g, h, a])
    } else {
        select_32([a, b, d, e, f, g, h, c])
    }
}
/// n = 9, i = 2, a < c, b < d, b < e, d < i, e < f, e < g, e < h
fn select_42([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_29([d, e, f, g, h, a, i])
    } else {
        select_32([a, b, c, e, f, g, h, d])
    }
}
/// n = 9, i = 2, a < e, b < d, b < h, b < i, c < f, c < h, c < i, f < g
fn select_43([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_41([a, b, d, e, c, f, h, i])
    } else {
        select_42([a, c, e, f, b, d, h, i, g])
    }
}
/// n = 8, i = 2, a < c, a < f, a < g, a < h, b < d, b < f, b < g, b < h, d < e
fn select_44([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_5([c, b, f, g, h])
    } else {
        select_40([d, a, f, g, h, e])
    }
}
/// n = 10, i = 3, a < d, a < g, b < c, b < g, b < i, b < j, c < f, d < e, d < i, d < j, g < h
fn select_45([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_13([c, a, f, g, h, d])
    } else {
        select_44([d, b, e, g, h, c, i, j])
    }
}
/// n = 12, i = 3, a < e, a < i, a < l, b < c, b < d, b < i, b < k, c < g, d < f, d < l, e < h, e < k, i < j
fn select_46([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_43([c, d, a, f, g, i, j, e, l])
    } else {
        select_45([a, b, c, e, h, g, i, j, d, k])
    }
}
/// n = 8, i = 2, a < d, a < g, a < h, b < c, b < g, b < h, c < e, d < f
fn select_47([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_39([c, a, g, h, e])
    } else {
        select_39([d, b, g, h, f])
    }
}
/// n = 8, i = 2, a < e, a < g, a < h, b < c, b < d, b < g, b < h, e < f
fn select_48([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_47([a, b, c, e, d, f, g, h])
    } else {
        select_47([a, b, d, e, c, f, g, h])
    }
}
/// n = 8, i = 3, a < d, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_49([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_12([a, c, d, e, g])
    } else {
        select_12([b, c, d, f, h])
    }
}
/// n = 10, i = 3, a < f, b < f, b < g, b < i, b < j, c < d, c < g, d < e, d < i, d < j, g < h
fn select_50([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_48([b, d, a, e, g, h, i, j])
    } else {
        select_49([a, b, c, f, g, d, h, e])
    }
}
/// n = 12, i = 3, a < h, b < e, b < i, b < l, c < d, c < h, c < i, c < k, d < f, d < l, e < g, e < k, i < j
fn select_51([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_43([a, d, b, f, h, i, j, e, l])
    } else {
        select_50([a, c, b, e, g, h, i, j, d, k])
    }
}
/// n = 12, i = 3, b < f, b < i, b < l, c < d, c < e, c < i, c < k, e < g, e < l, f < h, f < k, i < j
fn select_52([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < a {
        select_46([b, c, d, e, f, g, a, h, i, j, k, l])
    } else {
        select_51([a, b, c, e, f, g, h, d, i, j, k, l])
    }
}
/// n = 8, i = 1, a < h, b < g, c < d, c < e, c < f
fn select_53([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_29([a, c, d, e, f, b, h])
    } else {
        select_29([b, c, d, e, f, a, g])
    }
}
/// n = 8, i = 1, c < h, d < e, d < f, d < g
fn select_54([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_53([a, c, d, e, f, g, h, b])
    } else {
        select_53([b, c, d, e, f, g, h, a])
    }
}
/// n = 9, i = 2, c < d, c < e, d < i, e < f, e < g, e < h
fn select_55([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_42([a, c, b, d, e, f, g, h, i])
    } else {
        select_42([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 11, i = 3, b < d, b < h, b < k, c < e, c < h, e < f, e < g, e < k, g < i, h < j
fn select_56([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_54([a, f, g, b, d, h, k, i])
    } else {
        select_55([a, d, c, h, e, f, g, k, j])
    }
}
/// n = 12, i = 4, b < e, b < i, c < d, c < h, d < f, d < i, d < l, e < g, e < h, e < l, h < j, i < k
fn select_57([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_55([a, f, b, i, e, g, h, l, k])
    } else {
        select_55([a, g, c, h, d, f, i, l, j])
    }
}
/// n = 13, i = 4, b < e, b < h, c < d, c < h, c < j, d < g, d < m, e < f, e < j, e < l, e < m, h < i, h < l, j < k
fn select_58([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < d {
        select_56([a, e, c, f, h, i, d, j, g, k, l])
    } else {
        select_57([a, b, c, d, e, g, f, j, h, k, i, m])
    }
}
/// n = 14, i = 4, b < d, b < i, b < l, c < e, c < i, d < g, e < f, e < h, e < l, e < n, h < j, i < k, i < n, l < m
fn select_59([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < d {
        select_52([a, b, e, f, h, i, j, k, l, m, n, d])
    } else {
        select_58([a, c, b, d, e, f, g, i, k, l, m, n, h])
    }
}
/// n = 15, i = 4, b < d, b < m, c < e, c < f, d < i, e < h, e < l, f < g, f < j, f < l, f < m, j < k, l < n, m < o
fn select_60([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < b {
        select_37([a, f, e, h, g, j, b, k, d, l, i, n, m])
    } else {
        select_59([a, b, c, d, f, g, i, j, e, k, h, m, o, l])
    }
}
/// n = 5, i = 2, a < d, a < e, b < c, b < e
fn select_61([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_3([c, a, e])
    } else {
        select_3([d, b, e])
    }
}
/// n = 9, i = 3, a < c, a < g, b < d, b < g, c < f, d < e, d < h, h < i
fn select_62([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_61([b, c, f, g, d])
    } else {
        select_9([a, d, e, g, h, i, c])
    }
}
/// n = 5, i = 2, a < e, b < d, c < d, c < e
fn select_63([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_3([b, c, e])
    } else {
        select_3([a, c, d])
    }
}
/// n = 6, i = 2, b < f, c < d, c < f, d < e
fn select_64([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_39([b, d, a, e, f])
    } else {
        select_63([a, b, c, f, d])
    }
}
/// n = 8, i = 3, a < e, a < h, b < c, b < d, b < g, d < h, e < f, e < g
fn select_65([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_61([b, e, f, c, g])
    } else {
        select_64([c, d, a, e, g, h])
    }
}
/// n = 10, i = 3, a < c, a < d, a < i, b < e, b < j, d < j, e < f, e < g, e < i, g < h
fn select_66([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_9([a, e, f, c, g, h, i])
    } else {
        select_65([b, a, c, d, e, f, i, j])
    }
}
/// n = 10, i = 3, a < g, b < d, b < j, c < e, c < g, e < f, e < h, e < j, h < i
fn select_67([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_62([a, c, b, e, f, d, g, h, i])
    } else {
        select_66([b, c, d, a, e, f, h, i, j, g])
    }
}
/// n = 5, i = 3, b < e, c < d
fn select_68([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_16([a, b, d, e])
    } else {
        select_16([a, c, e, d])
    }
}
/// n = 4, i = 2, a < d, b < c
fn select_69([a, b, c, d]: [usize; 4]) -> usize {
    if c < d {
        select_15([a, c])
    } else {
        select_15([b, d])
    }
}
/// n = 5, i = 3, a < e, b < e, c < d
fn select_70([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < d {
        select_69([b, c, d, e])
    } else {
        select_15([a, b])
    }
}
/// n = 6, i = 3, b < e, c < d, c < f, e < f
fn select_71([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_61([c, e, a, d, f])
    } else {
        select_70([a, b, c, d, e])
    }
}
/// n = 8, i = 3, a < d, b < e, c < e, c < h, d < h, e < f, e < g
fn select_72([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_16([b, c, d, h])
    } else {
        select_4([a, e, f, g])
    }
}
/// n = 10, i = 4, a < g, b < e, b < f, c < f, c < g, d < e, d < g, d < h, f < h, g < i, g < j
fn select_73([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_71([a, c, d, e, f, h])
    } else {
        select_72([b, a, d, f, g, i, j, h])
    }
}
/// n = 10, i = 4, a < g, b < e, b < f, c < g, c < h, d < f, d < g, f < h, g < i, g < j
fn select_74([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < c {
        select_68([a, d, e, c, f])
    } else {
        select_73([a, b, d, c, e, f, g, h, i, j])
    }
}
/// n = 5, i = 2, a < e, b < c, b < d
fn select_75([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_12([a, b, e, c, d])
    } else {
        select_12([a, b, e, d, c])
    }
}
/// n = 7, i = 3, a < e, b < d, c < e, c < g, d < g, e < f
fn select_76([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_16([a, c, d, g])
    } else {
        select_3([b, e, f])
    }
}
/// n = 11, i = 4, a < e, a < f, b < f, b < g, c < e, c < g, d < f, d < g, d < k, e < k, f < j, g < h, g < i
fn select_77([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_76([b, c, d, e, f, j, k])
    } else {
        select_72([a, b, d, e, g, h, i, k])
    }
}
/// n = 11, i = 4, a < h, a < k, b < e, b < f, c < e, c < h, d < f, d < h, e < k, f < g, h < i, h < j
fn select_78([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_75([c, f, a, g, e])
    } else {
        select_77([b, d, c, a, e, f, h, i, j, g, k])
    }
}
/// n = 12, i = 5, a < i, a < j, b < h, b < i, c < e, c < i, d < f, d < i, e < g, f < h, h < j, i < k, i < l
fn select_79([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_74([a, e, b, d, g, f, i, h, k, l])
    } else {
        select_78([a, f, b, c, h, e, g, i, k, l, j])
    }
}
/// n = 13, i = 5, a < l, a < m, b < h, b < i, c < f, c < i, d < e, d < i, e < h, f < g, f < k, h < l, i < j, i < k, i < m
fn select_80([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < a {
        select_67([e, f, b, g, i, j, h, a, m, k])
    } else {
        select_79([a, b, c, d, f, e, g, h, i, l, j, m])
    }
}
/// n = 11, i = 4, a < k, b < e, b < f, b < i, c < g, c < j, d < f, d < g, f < j, g < h, g < i, g < k
fn select_81([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_66([b, c, e, f, g, h, a, k, i, j])
    } else {
        select_74([a, b, c, d, e, f, g, j, h, k])
    }
}
/// n = 6, i = 2, a < e, a < f, b < c, b < d, b < f
fn select_82([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_13([a, b, e, c, d, f])
    } else {
        select_13([a, b, e, d, c, f])
    }
}
/// n = 7, i = 3, a < d, a < g, b < e, b < g, c < e, c < g, e < f
fn select_83([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_38([a, e, f, g])
    } else {
        select_16([d, b, c, g])
    }
}
/// n = 8, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_84([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_4([d, e, a, g])
    } else {
        select_83([b, a, c, d, f, h, e])
    }
}
/// n = 9, i = 3, a < g, b < e, c < d, c < g, c < h, e < f, e < h, g < i
fn select_85([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_82([c, e, a, f, d, h])
    } else {
        select_84([b, c, a, d, g, e, i, f])
    }
}
/// n = 8, i = 3, a < d, a < h, b < e, b < h, c < e, c < h, e < f, e < g
fn select_86([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_16([d, b, c, h])
    } else {
        select_39([a, e, f, g, h])
    }
}
/// n = 9, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h, f < i
fn select_87([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_4([d, e, a, g])
    } else {
        select_86([b, a, c, d, f, h, i, e])
    }
}
/// n = 10, i = 3, a < g, a < j, b < e, c < d, c < g, c < h, e < f, e < h, e < j, g < i
fn select_88([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_9([c, e, f, d, a, j, h])
    } else {
        select_87([b, c, a, d, g, e, i, f, j])
    }
}
/// n = 12, i = 4, a < j, a < l, b < f, b < g, c < e, c < f, c < j, d < e, d < g, d < j, f < i, g < h, g < l, j < k
fn select_89([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_85([a, b, d, e, f, i, j, g, k])
    } else {
        select_88([a, b, c, e, g, h, j, f, k, l])
    }
}
/// n = 14, i = 5, a < k, a < n, b < e, b < i, c < h, c < i, c < k, d < f, d < i, e < g, e < l, f < h, f < k, i < j, i < l, i < n, k < m
fn select_90([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < f {
        select_81([a, e, c, d, g, f, i, j, l, h, n])
    } else {
        select_89([a, b, f, c, h, e, i, j, g, k, m, n])
    }
}
/// n = 14, i = 5, a < k, a < n, b < g, b < i, c < f, c < i, d < e, d < i, d < k, f < k, g < h, g < l, i < j, i < l, i < n, k < m
fn select_91([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < f {
        select_80([a, c, b, d, e, g, h, f, i, j, l, k, n])
    } else {
        select_90([a, b, d, c, g, f, h, e, i, j, k, l, m, n])
    }
}
/// n = 7, i = 2, a < d, b < c, b < e, e < f, e < g
fn select_92([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_5([c, e, a, f, g])
    } else {
        select_61([a, b, c, d, e])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < e, c < f
fn select_93([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_61([a, c, d, b, f])
    } else {
        select_61([b, c, d, a, e])
    }
}
/// n = 7, i = 3, a < g, b < d, c < e, e < f, e < g
fn select_94([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_16([a, c, d, e])
    } else {
        select_93([a, b, e, f, d, g])
    }
}
/// n = 8, i = 3, b < d, c < e, e < f, e < g, g < h
fn select_95([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < a {
        select_92([b, e, f, d, g, a, h])
    } else {
        select_94([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 3, b < d, b < i, c < e, e < f, e < g, e < i, g < h
fn select_96([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if i < a {
        select_9([b, e, f, d, g, h, i])
    } else {
        select_95([a, b, c, d, e, f, g, h])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < f
fn select_97([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_63([a, d, b, e, f])
    } else {
        select_63([a, e, c, d, f])
    }
}
/// n = 7, i = 3, a < e, b < f, c < d, c < f, f < g
fn select_98([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_97([b, a, c, d, e, f])
    } else {
        select_4([d, f, a, g])
    }
}
/// n = 8, i = 3, a < e, a < h, b < f, b < h, c < d, c < f, c < h, f < g
fn select_99([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_97([b, a, c, d, e, h])
    } else {
        select_64([d, a, b, f, g, h])
    }
}
/// n = 8, i = 3, b < h, c < f, c < h, d < e, d < f, d < h, f < g
fn select_100([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_98([a, c, d, e, b, f, g])
    } else {
        select_99([b, c, d, e, a, f, g, h])
    }
}
/// n = 9, i = 3, a < e, a < i, b < f, b < i, c < d, c < f, c < i, f < g, f < h
fn select_101([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_4([d, f, g, h])
    } else {
        select_99([a, b, c, d, e, f, h, i])
    }
}
/// n = 11, i = 4, a < h, b < f, b < g, c < g, c < h, d < e, d < g, d < h, g < i, h < j, h < k
fn select_102([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < a {
        select_100([f, a, c, d, e, g, i, h])
    } else {
        select_101([b, c, d, e, f, h, j, k, g])
    }
}
/// n = 12, i = 4, a < i, a < l, b < g, c < f, c < i, c < j, d < e, d < g, d < i, g < h, g < j, g < l, i < k
fn select_103([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_96([e, c, b, f, g, h, a, l, j])
    } else {
        select_102([b, c, a, d, e, f, i, g, k, h, l])
    }
}
/// n = 7, i = 2, a < e, b < d, c < f, f < g
fn select_104([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_14([c, a, e, b, f, d, g])
    } else {
        select_14([c, b, d, a, f, e, g])
    }
}
/// n = 6, i = 2, a < d, b < c, c < e, d < f
fn select_105([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_3([a, c, e])
    } else {
        select_3([b, d, f])
    }
}
/// n = 8, i = 3, a < e, b < d, c < e, c < f, c < g, d < f, d < g, e < h
fn select_106([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_4([a, c, f, g])
    } else {
        select_12([b, c, d, e, h])
    }
}
/// n = 8, i = 3, a < f, b < d, b < e, c < e, c < f, c < g, d < g, f < h
fn select_107([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_105([a, b, e, f, d, h])
    } else {
        select_106([a, b, c, d, f, e, g, h])
    }
}
/// n = 8, i = 3, a < f, b < e, c < g, d < e, d < f, d < g, g < h
fn select_108([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_107([c, a, d, b, f, g, e, h])
    } else {
        select_107([c, b, d, a, e, g, f, h])
    }
}
/// n = 8, i = 3, b < f, c < g, d < e, d < f, d < g, g < h
fn select_109([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_104([b, e, c, a, f, g, h])
    } else {
        select_108([a, b, c, d, f, e, g, h])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < e, c < f
fn select_110([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_69([a, c, d, b])
    } else {
        select_8([b, c, d, a, e, f])
    }
}
/// n = 6, i = 3, a < f, b < e, c < e, c < f, d < e, d < f
fn select_111([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_16([b, c, d, f])
    } else {
        select_16([a, c, d, e])
    }
}
/// n = 6, i = 3, b < f, c < e, c < f, d < e, d < f
fn select_112([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_111([a, b, c, d, f, e])
    } else {
        select_38([b, e, a, f])
    }
}
/// n = 7, i = 3, b < g, c < e, c < g, d < e, d < g, e < f
fn select_113([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_39([b, e, a, f, g])
    } else {
        select_111([a, b, c, d, g, e])
    }
}
/// n = 8, i = 3, b < e, b < h, c < f, c < h, d < e, d < f, d < h, f < g
fn select_114([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_112([a, c, b, d, e, h])
    } else {
        select_113([a, b, c, d, f, g, h])
    }
}
/// n = 11, i = 4, b < f, b < h, c < f, c < i, d < e, d < k, e < h, e < i, f < g, f < k, i < j
fn select_115([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_110([a, d, f, g, e, k])
    } else {
        select_114([a, b, c, e, h, i, j, f])
    }
}
/// n = 12, i = 4, b < g, b < h, c < g, c < j, d < e, d < l, e < f, e < h, e < j, g < i, g < l, j < k
fn select_116([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_109([a, b, c, e, f, h, j, k])
    } else {
        select_115([a, b, c, d, e, g, i, h, j, k, l])
    }
}
/// n = 14, i = 5, a < h, a < n, b < j, b < l, c < f, c < j, d < e, d < h, d < j, f < g, f < m, h < i, h < l, j < k, j < m, j < n
fn select_117([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < h {
        select_103([a, b, f, d, e, g, j, k, h, m, i, n])
    } else {
        select_116([e, b, c, a, h, i, j, l, k, f, g, n])
    }
}
/// n = 14, i = 5, a < i, a < n, b < f, b < k, c < g, c < k, d < e, d < i, d < k, g < h, g < m, i < j, k < l, k < m, k < n
fn select_118([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < i {
        select_91([a, c, b, d, e, f, g, h, k, l, i, m, j, n])
    } else {
        select_117([a, b, c, d, e, g, h, i, j, k, l, f, m, n])
    }
}
/// n = 16, i = 5, a < f, b < i, b < n, c < g, c < j, d < e, d < i, d < j, g < h, g < m, i < l, j < k, j < m, j < n, m < o, n < p
fn select_119([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < a {
        select_60([e, b, c, i, g, j, k, h, l, a, f, m, n, o, p])
    } else {
        select_118([b, a, c, d, e, f, g, h, i, l, j, k, m, n])
    }
}
/// n = 7, i = 2, b < d, b < g, c < e, c < g, e < f
fn select_120([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_12([b, a, d, c, e])
    } else {
        select_9([b, c, a, d, e, f, g])
    }
}
/// n = 7, i = 3, a < f, b < e, c < d, c < f, c < g, e < g
fn select_121([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_93([a, e, c, d, g, f])
    } else {
        select_75([b, c, d, f, e])
    }
}
/// n = 8, i = 3, b < f, b < h, c < d, c < e, c < g, e < h, f < g
fn select_122([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_120([d, e, b, a, f, g, h])
    } else {
        select_121([a, b, c, d, f, e, g])
    }
}
/// n = 8, i = 4, b < e, c < d, d < f, d < h, e < g, e < h
fn select_123([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_71([a, b, d, f, e, h])
    } else {
        select_71([a, c, e, g, d, h])
    }
}
/// n = 9, i = 4, b < f, c < d, c < e, d < i, e < h, f < g, f < h, f < i
fn select_124([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_123([a, b, c, d, f, e, g, i])
    } else {
        select_123([a, b, c, e, f, d, g, h])
    }
}
/// n = 9, i = 4, b < d, b < e, c < f, e < i, f < g, f < h, f < i
fn select_125([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_122([a, b, f, h, g, e, i, d])
    } else {
        select_124([a, c, b, d, e, f, h, i, g])
    }
}
/// n = 7, i = 2, a < c, a < f, a < g, b < d, b < f, b < g, d < e
fn select_126([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_4([c, b, f, g])
    } else {
        select_39([d, a, f, g, e])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, b < g, c < d, c < e, c < f
fn select_127([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_12([c, a, d, b, g])
    } else {
        select_126([c, b, d, a, g, e, f])
    }
}
/// n = 8, i = 2, a < h, b < f, b < g, c < d, c < f, c < g, c < h, d < e
fn select_128([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_13([a, c, b, d, e, h])
    } else {
        select_126([b, c, a, d, e, f, g])
    }
}
/// n = 9, i = 2, a < i, b < d, b < g, b < h, c < e, c < g, c < h, c < i, e < f
fn select_129([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_127([a, c, b, d, g, h, i])
    } else {
        select_128([a, b, c, e, f, g, h, i])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < e, b < i, d < h, e < f, e < g
fn select_130([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_40([d, e, a, f, g, h])
    } else {
        select_126([a, b, c, d, h, e, i])
    }
}
/// n = 11, i = 3, a < d, a < g, a < k, b < c, b < g, b < i, b < j, c < f, c < k, d < e, d < i, d < j, g < h
fn select_131([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < c {
        select_126([d, b, e, g, h, i, j])
    } else {
        select_130([c, a, f, g, d, i, j, h, k])
    }
}
/// n = 11, i = 3, a < d, a < h, a < j, a < k, b < c, b < e, b < h, d < g, e < f, e < j, e < k, h < i
fn select_132([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_129([c, e, a, f, h, i, j, k, d])
    } else {
        select_131([b, a, d, e, f, g, h, i, j, k, c])
    }
}
/// n = 6, i = 2, a < f, b < e, b < f, c < d, c < e
fn select_133([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_12([c, a, d, b, f])
    } else {
        select_13([c, b, d, a, f, e])
    }
}
/// n = 7, i = 2, a < g, b < d, c < e, c < g, e < f
fn select_134([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_39([a, e, b, f, g])
    } else {
        select_133([a, c, b, d, e, g])
    }
}
/// n = 8, i = 4, a < f, b < f, c < d, c < h, d < e, f < g, f < h
fn select_135([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_16([e, a, b, f])
    } else {
        select_4([g, c, d, h])
    }
}
/// n = 9, i = 3, a < d, a < g, b < c, b < g, b < i, c < f, d < e, d < i, g < h
fn select_136([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_13([c, a, f, g, h, d])
    } else {
        select_126([d, b, e, g, h, c, i])
    }
}
/// n = 10, i = 4, a < g, b < e, c < d, c < h, d < f, d < j, e < g, e < h, e < j, h < i
fn select_137([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_135([a, b, c, d, f, e, g, h])
    } else {
        select_136([c, e, a, d, f, g, h, i, j])
    }
}
/// n = 10, i = 4, a < h, b < d, b < i, c < e, d < g, e < f, e < h, e < i, i < j
fn select_138([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_134([f, a, b, h, i, j, d])
    } else {
        select_137([a, c, b, d, e, g, h, i, j, f])
    }
}
/// n = 12, i = 4, b < d, b < i, b < k, b < l, c < e, d < g, e < f, e < h, e < i, h < k, h < l, i < j
fn select_139([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_132([b, e, f, d, h, a, g, i, j, k, l])
    } else {
        select_138([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 12, i = 4, b < e, c < i, c < k, c < l, d < f, f < g, f < h, f < i, h < k, h < l, i < j
fn select_140([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < c {
        select_125([a, b, d, e, c, f, g, h, i])
    } else {
        select_139([a, c, d, b, f, g, e, h, i, j, k, l])
    }
}
/// n = 10, i = 3, a < d, b < g, b < h, b < j, c < e, c < g, c < h, e < f, e < j, h < i
fn select_141([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_10([b, e, f, g, a, h, d, i, j])
    } else {
        select_19([b, a, c, d, g, e, h, f, i])
    }
}
/// n = 10, i = 4, a < f, a < h, b < e, b < g, c < f, c < g, c < h, d < e, d < g, d < h, g < i, h < j
fn select_142([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_84([a, b, c, e, g, h, i, j])
    } else {
        select_84([b, a, d, f, h, g, j, i])
    }
}
/// n = 11, i = 4, a < f, b < e, b < i, b < k, c < f, c < g, c < i, d < e, d < g, d < i, g < h, g < k, i < j
fn select_143([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_10([b, g, h, e, a, i, f, j, k])
    } else {
        select_142([a, b, c, d, e, f, i, g, j, h])
    }
}
/// n = 11, i = 4, b < f, b < i, b < k, c < f, c < g, c < i, d < e, d < g, d < i, g < h, g < k, i < j
fn select_144([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_141([e, b, c, a, g, h, f, i, j, k])
    } else {
        select_143([a, b, d, c, f, e, g, h, i, j, k])
    }
}
/// n = 7, i = 3, a < g, b < d, b < f, c < e, e < f, e < g
fn select_145([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_12([b, c, d, e, g])
    } else {
        select_17([a, d, c, e, f])
    }
}
/// n = 6, i = 3, a < e, b < e, b < f, c < d, d < f
fn select_146([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < d {
        select_12([a, c, e, d, f])
    } else {
        select_3([a, b, f])
    }
}
/// n = 7, i = 4, a < g, b < d, c < e, d < g, e < f
fn select_147([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_146([a, d, c, e, g, f])
    } else {
        select_16([a, b, f, d])
    }
}
/// n = 9, i = 5, a < h, b < g, b < i, c < e, d < f, e < h, e < i, f < g, f < i
fn select_148([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_145([a, b, c, g, e, i, h])
    } else {
        select_147([b, d, c, f, e, h, g])
    }
}
/// n = 5, i = 3, b < e, c < e, d < e
fn select_149([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < c {
        select_16([a, c, d, e])
    } else {
        select_16([a, b, d, e])
    }
}
/// n = 6, i = 4, b < f, c < f, d < e
fn select_150([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_149([a, b, c, e, f])
    } else {
        select_16([a, d, f, e])
    }
}
/// n = 8, i = 5, a < h, b < g, c < e, d < f, e < h, f < g
fn select_151([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < h {
        select_150([a, b, f, c, e, g])
    } else {
        select_150([b, a, e, d, f, h])
    }
}
/// n = 9, i = 5, a < h, b < f, c < e, c < i, d < g, f < i, g < h, g < i
fn select_152([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_148([a, c, d, b, g, f, e, h, i])
    } else {
        select_151([a, b, d, c, g, e, f, h])
    }
}
/// n = 6, i = 1, c < d, c < e, c < f
fn select_153([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_40([a, c, d, e, f, b])
    } else {
        select_40([b, c, d, e, f, a])
    }
}
/// n = 8, i = 2, b < h, c < d, c < e, c < h, e < f, e < g
fn select_154([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_153([a, d, e, b, f, g])
    } else {
        select_110([a, b, c, d, e, h])
    }
}
/// n = 7, i = 4, a < g, b < f, b < g, c < f, c < g, d < e, d < g
fn select_155([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_111([a, e, b, c, f, g])
    } else {
        select_63([a, f, d, e, g])
    }
}
/// n = 9, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, g < h, g < i
fn select_156([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_155([a, b, c, d, e, f, g])
    } else {
        select_153([e, f, g, a, h, i])
    }
}
/// n = 10, i = 4, a < j, b < f, b < i, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_157([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_154([e, a, g, h, b, f, i, j])
    } else {
        select_156([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 11, i = 5, a < f, b < h, b < k, c < e, c < i, d < g, g < h, g < i, i < j, i < k
fn select_158([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < i {
        select_152([b, a, c, d, e, f, g, h, i])
    } else {
        select_157([a, b, g, c, e, h, i, j, k, f])
    }
}
/// n = 13, i = 5, a < f, b < h, b < k, b < m, c < e, c < i, c < k, d < g, g < h, g < i, g < k, i < j, i < m, k < l
fn select_159([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if k < f {
        select_144([a, b, g, c, e, h, i, j, k, l, m])
    } else {
        select_158([a, b, c, d, e, f, g, h, i, j, m])
    }
}
/// n = 14, i = 5, a < f, b < j, b < k, b < l, c < e, c < h, c < l, d < g, d < h, g < j, g < l, h < i, h < k, k < m, l < n
fn select_160([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_140([e, a, b, d, f, h, i, g, k, m, j, l])
    } else {
        select_159([a, b, c, d, e, f, g, j, h, i, l, n, k])
    }
}
/// n = 8, i = 4, a < h, b < f, c < f, c < g, c < h, d < e, e < g, e < h
fn select_161([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < e {
        select_71([a, d, c, f, e, g])
    } else {
        select_93([a, b, c, g, f, h])
    }
}
/// n = 9, i = 5, a < i, b < e, c < g, d < f, f < h, f < i, g < h, g < i
fn select_162([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_161([a, b, f, c, g, e, h, i])
    } else {
        select_150([a, d, e, c, g, f])
    }
}
/// n = 8, i = 4, a < h, b < e, c < g, c < h, d < f, f < g, f < h
fn select_163([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_161([a, b, c, d, f, e, g, h])
    } else {
        select_68([a, d, e, c, f])
    }
}
/// n = 10, i = 5, a < e, b < i, b < j, c < f, c < i, d < g, f < j, g < h, g < j, h < i
fn select_164([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_162([b, a, c, d, e, g, f, h, j])
    } else {
        select_163([b, a, c, g, e, h, f, i])
    }
}
/// n = 7, i = 4, b < g, c < f, d < e, d < g, f < g
fn select_165([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_97([b, d, f, a, e, g])
    } else {
        select_150([b, a, c, d, e, f])
    }
}
/// n = 7, i = 3, a < f, a < g, b < d, b < g, c < e, c < g, e < f
fn select_166([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_12([b, e, d, a, f])
    } else {
        select_83([b, a, c, d, e, f, g])
    }
}
/// n = 9, i = 3, a < d, b < f, b < i, c < e, c < f, e < i, f < g, f < h
fn select_167([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_40([e, f, a, g, h, i])
    } else {
        select_166([b, a, c, d, e, i, f])
    }
}
/// n = 11, i = 5, a < e, b < h, b < k, c < g, d < f, d < h, f < k, g < h, g < k, h < i, h < j
fn select_168([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_165([e, b, c, d, f, g, h])
    } else {
        select_167([a, b, d, e, f, h, i, j, k])
    }
}
/// n = 11, i = 5, a < e, b < i, b < k, c < f, c < i, d < g, f < k, g < h, g < i, g < k, i < j
fn select_169([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_164([a, b, c, d, e, f, g, h, i, k])
    } else {
        select_168([a, b, d, c, e, f, g, i, h, j, k])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < f, c < g, d < e
fn select_170([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_12([a, c, b, d, e])
    } else {
        select_126([b, c, a, d, e, f, g])
    }
}
/// n = 9, i = 2, b < e, b < h, b < i, c < d, c < h, c < i, d < f, e < g
fn select_171([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_170([a, b, c, d, f, h, i])
    } else {
        select_170([a, c, b, e, g, h, i])
    }
}
/// n = 7, i = 1, b < f, b < g, c < d, c < e
fn select_172([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_5([a, b, c, f, g])
    } else {
        select_5([a, c, b, d, e])
    }
}
/// n = 6, i = 3, a < e, a < f, b < e, b < f, c < d, c < f
fn select_173([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_16([d, a, b, f])
    } else {
        select_3([e, c, f])
    }
}
/// n = 8, i = 3, a < h, b < e, b < h, c < d, c < f, c < g, c < h, e < f, e < g
fn select_174([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_8([c, e, a, d, f, g])
    } else {
        select_173([a, b, c, d, e, h])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, c < h, c < i, e < h, e < i, f < g
fn select_175([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_172([d, e, f, a, g, h, i])
    } else {
        select_174([a, b, c, d, e, h, i, f])
    }
}
/// n = 11, i = 3, b < d, b < g, b < j, b < k, c < e, c < g, e < f, e < j, e < k, f < h, g < i
fn select_176([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_171([a, b, e, f, g, h, i, j, k])
    } else {
        select_175([a, c, b, d, e, g, i, j, k])
    }
}
/// n = 10, i = 4, a < h, b < f, b < g, c < g, c < h, d < e, d < g, d < h, g < i, h < j
fn select_177([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_99([b, c, d, e, f, h, j, g])
    } else {
        select_100([f, a, c, d, e, g, i, h])
    }
}
/// n = 13, i = 5, a < h, a < j, b < e, b < f, c < d, c < h, c < j, e < g, f < j, f < l, f < m, h < i, h < l, h < m, j < k
fn select_178([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_176([d, h, b, i, f, j, e, k, g, l, m])
    } else {
        select_177([f, e, a, c, d, g, h, j, i, k])
    }
}
/// n = 14, i = 6, a < j, a < l, b < f, c < d, c < e, d < i, e < g, e < l, e < n, f < h, f < j, f < l, j < k, j < n, l < m
fn select_179([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < j {
        select_169([d, a, e, b, i, g, f, h, l, m, j])
    } else {
        select_178([a, c, f, h, d, e, i, j, k, l, m, g, n])
    }
}
/// n = 4, i = 2, c < d
fn select_180([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_69([a, c, d, b])
    } else {
        select_69([b, c, d, a])
    }
}
/// n = 7, i = 3, a < d, b < f, b < g, c < e, e < f, e < g
fn select_181([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_180([b, d, c, e])
    } else {
        select_39([a, b, f, g, d])
    }
}
/// n = 6, i = 2, a < f, b < f, c < d, d < e
fn select_182([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_4([b, d, a, e])
    } else {
        select_63([b, c, a, d, f])
    }
}
/// n = 8, i = 2, a < h, b < e, c < d, c < h, d < f, e < g
fn select_183([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_182([a, c, b, e, g, h])
    } else {
        select_64([b, a, c, d, f, h])
    }
}
/// n = 6, i = 2, a < e, a < f, b < d, b < f, c < d, c < e
fn select_184([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_38([b, c, e, f])
    } else {
        select_38([a, c, d, f])
    }
}
/// n = 7, i = 2, a < g, b < f, b < g, c < d, c < f, d < e
fn select_185([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_39([b, d, a, e, f])
    } else {
        select_184([a, b, c, f, d, g])
    }
}
/// n = 10, i = 3, a < f, a < j, b < e, b < f, b < h, c < d, c < e, d < h, d < j, e < g, f < i
fn select_186([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_105([a, c, e, f, g, i])
    } else {
        select_185([a, d, b, e, g, h, j])
    }
}
/// n = 10, i = 3, a < g, a < j, b < e, b < f, c < d, c < f, c < g, e < j, f < h, g < i
fn select_187([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_183([d, a, b, f, g, h, i, e])
    } else {
        select_186([a, c, b, e, f, g, h, d, i, j])
    }
}
/// n = 12, i = 5, a < h, a < j, b < e, b < i, c < g, c < i, c < j, d < f, e < h, f < g, f < i, f < j, i < k, j < l
fn select_188([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_181([a, c, d, h, f, g, i])
    } else {
        select_187([a, b, c, g, e, i, j, k, l, h])
    }
}
/// n = 12, i = 5, a < f, a < j, b < e, b < i, c < h, c < i, c < j, d < g, g < h, g < i, g < j, i < k, j < l
fn select_189([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_188([a, b, c, d, e, g, h, f, i, j, k, l])
    } else {
        select_188([b, a, c, d, f, g, h, e, j, i, l, k])
    }
}
/// n = 14, i = 6, a < i, a < j, a < l, b < f, c < d, c < e, d < h, e < g, e < l, e < n, f < i, f < j, f < l, j < k, j < n, l < m
fn select_190([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < d {
        select_56([i, j, c, k, e, g, l, d, m, h, n])
    } else {
        select_189([d, e, a, b, g, h, f, i, l, j, m, k])
    }
}
/// n = 15, i = 6, a < f, a < m, b < e, b < k, b < l, c < d, c < g, d < i, e < j, e < m, f < k, f < l, g < h, g < l, g < m, l < n, m < o
fn select_191([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < f {
        select_179([a, b, c, d, g, e, h, j, i, f, k, m, o, l])
    } else {
        select_190([b, a, c, d, g, f, h, i, k, e, j, l, n, m])
    }
}
/// n = 15, i = 6, a < e, a < k, a < n, b < f, b < l, c < d, c < g, d < i, e < j, f < k, f < n, g < h, g < l, g < n, l < m, n < o
fn select_192([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if l < e {
        select_160([d, a, g, b, h, i, f, l, m, k, e, n, j, o])
    } else {
        select_191([b, a, c, d, e, f, g, h, i, j, k, n, l, o, m])
    }
}
/// n = 17, i = 6, a < e, a < l, b < f, b < k, b < m, c < d, c < g, d < i, e < k, f < j, f < p, g < h, g < l, g < m, l < o, m < n, m < p, p < q
fn select_193([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if m < e {
        select_119([d, a, b, g, h, i, f, j, l, m, n, o, p, e, q, k])
    } else {
        select_192([b, a, c, d, f, e, g, h, i, j, k, l, o, m, n])
    }
}
/// n = 8, i = 3, a < d, b < e, c < e, e < f, e < g, e < h
fn select_194([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_23([a, f, g, h])
    } else {
        select_16([d, b, c, e])
    }
}
/// n = 8, i = 3, c < e, d < e, e < f, e < g, e < h
fn select_195([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_194([a, c, d, b, e, f, g, h])
    } else {
        select_194([b, c, d, a, e, f, g, h])
    }
}
/// n = 8, i = 3, b < e, b < f, c < d, c < f, f < g, f < h
fn select_196([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_153([d, e, f, a, g, h])
    } else {
        select_97([a, b, c, d, e, f])
    }
}
/// n = 9, i = 4, b < f, c < d, c < e, e < h, e < i, f < g, f < h, f < i
fn select_197([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_195([a, d, b, e, f, g, h, i])
    } else {
        select_196([a, c, f, g, d, e, h, i])
    }
}
/// n = 6, i = 2, c < d, d < e, d < f
fn select_198([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_16([b, a, c, d])
    } else {
        select_5([b, d, a, e, f])
    }
}
/// n = 7, i = 3, b < e, c < d, c < f, c < g, e < f, e < g
fn select_199([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_8([c, e, a, d, f, g])
    } else {
        select_70([a, b, c, d, e])
    }
}
/// n = 9, i = 4, b < e, c < d, d < f, d < h, d < i, e < g, e < h, e < i
fn select_200([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_199([a, b, d, f, e, h, i])
    } else {
        select_199([a, c, e, g, d, h, i])
    }
}
/// n = 10, i = 4, b < e, c < d, c < h, d < g, d < j, e < f, e < h, e < j, h < i
fn select_201([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_198([a, f, c, h, d, i])
    } else {
        select_200([a, b, c, d, e, g, f, h, j])
    }
}
/// n = 10, i = 4, b < e, b < j, c < h, c < j, d < f, f < g, f < h, f < j, h < i
fn select_202([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_197([a, d, b, e, c, f, g, h, j])
    } else {
        select_201([a, d, c, b, f, g, e, h, i, j])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < g, c < d, c < f, c < g
fn select_203([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_166([a, b, c, e, d, f, g])
    } else {
        select_83([b, a, c, e, f, d, g])
    }
}
/// n = 7, i = 4, a < g, b < e, c < f, d < f, f < g
fn select_204([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_63([a, b, f, e, g])
    } else {
        select_149([a, c, d, e, f])
    }
}
/// n = 8, i = 4, b < g, b < h, c < e, c < h, d < f, f < g, f < h
fn select_205([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_203([b, c, f, a, e, g, h])
    } else {
        select_204([b, c, a, d, e, f, g])
    }
}
/// n = 8, i = 3, a < d, b < f, b < h, c < e, c < f, e < h, f < g
fn select_206([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_39([e, f, a, g, h])
    } else {
        select_166([b, a, c, d, e, h, f])
    }
}
/// n = 8, i = 3, a < f, b < e, b < g, c < d, c < g, g < h
fn select_207([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_206([a, b, c, f, d, g, h, e])
    } else {
        select_206([a, c, b, f, e, g, h, d])
    }
}
/// n = 10, i = 5, a < f, b < h, b < i, c < e, c < i, d < g, g < h, g < i, i < j
fn select_208([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_205([f, b, c, d, e, g, h, i])
    } else {
        select_207([a, b, c, e, h, f, i, j])
    }
}
/// n = 11, i = 5, a < f, a < k, b < g, b < k, c < h, c < j, d < e, d < h, g < j, h < i, h < k
fn select_209([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_202([e, a, b, c, f, h, i, g, j, k])
    } else {
        select_208([a, c, d, b, e, f, g, j, h, i])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < g, b < h, c < d, c < h, d < f, d < g
fn select_210([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_4([a, b, f, g])
    } else {
        select_61([b, c, d, e, h])
    }
}
/// n = 8, i = 3, a < d, a < h, b < f, b < g, b < h, c < e, c < h, e < f, e < g
fn select_211([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < b {
        select_69([c, d, b, e])
    } else {
        select_210([a, b, c, e, d, f, g, h])
    }
}
/// n = 10, i = 3, a < d, b < f, b < i, b < j, c < e, c < f, e < i, e < j, f < g, f < h
fn select_212([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_29([e, f, a, g, h, i, j])
    } else {
        select_211([a, b, c, d, e, i, j, f])
    }
}
/// n = 10, i = 3, a < e, b < f, b < g, c < d, c < g, c < j, f < j, g < h, g < i
fn select_213([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_167([a, b, c, e, d, g, h, i, f])
    } else {
        select_212([a, c, b, e, f, g, h, i, d, j])
    }
}
/// n = 7, i = 2, a < c, b < d, d < e, d < f, d < g
fn select_214([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_23([a, e, f, g])
    } else {
        select_3([c, b, d])
    }
}
/// n = 9, i = 2, a < c, b < d, b < e, d < h, d < i, e < f, e < g
fn select_215([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_214([a, b, c, d, e, h, i])
    } else {
        select_214([a, b, c, e, d, f, g])
    }
}
/// n = 11, i = 3, a < d, a < k, b < f, b < i, b < j, c < e, c < f, e < i, e < j, e < k, f < g, f < h
fn select_216([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_64([d, b, c, e, k, f])
    } else {
        select_215([a, c, d, e, f, g, h, i, j])
    }
}
/// n = 11, i = 3, a < e, a < k, b < d, b < g, b < j, c < f, c < g, f < j, f < k, g < h, g < i
fn select_217([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < f {
        select_167([a, c, b, e, d, g, h, i, f])
    } else {
        select_216([a, b, c, e, f, g, h, i, d, j, k])
    }
}
/// n = 11, i = 3, b < k, c < e, c < g, c < j, d < f, d < g, f < j, f < k, g < h, g < i
fn select_218([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_213([a, d, c, e, b, f, g, h, i, j])
    } else {
        select_217([b, c, d, e, a, f, g, h, i, j, k])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < e, e < g
fn select_219([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_111([a, b, c, e, f, g])
    } else {
        select_16([a, d, f, e])
    }
}
/// n = 7, i = 4, b < f, c < f, c < g, d < e, e < g
fn select_220([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < g {
        select_219([a, b, c, d, e, f, g])
    } else {
        select_12([b, e, f, g, a])
    }
}
/// n = 6, i = 3, a < e, b < c, b < d, e < f
fn select_221([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < c {
        select_61([b, e, f, d, c])
    } else {
        select_16([d, a, c, e])
    }
}
/// n = 9, i = 3, a < d, b < c, b < g, b < h, b < i, c < f, d < e, d < g, d < h, d < i
fn select_222([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < c {
        select_5([e, b, g, h, i])
    } else {
        select_214([c, a, f, d, g, h, i])
    }
}
/// n = 9, i = 3, a < d, b < g, b < h, b < i, c < e, e < f, e < g, e < h, e < i
fn select_223([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_221([c, a, b, d, e, f])
    } else {
        select_222([c, b, a, e, f, d, g, h, i])
    }
}
/// n = 10, i = 4, a < f, b < h, b < i, b < j, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_224([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_220([b, a, c, d, e, f, g])
    } else {
        select_223([a, b, d, f, g, e, h, i, j])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, b < i, c < f, d < e, d < i, e < h, f < g, f < i
fn select_225([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < h {
        select_155([a, b, f, d, e, g, i])
    } else {
        select_219([b, e, a, c, f, h, i])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, c < e, c < i, d < f, d < i, f < h, g < i
fn select_226([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_225([a, c, b, d, f, g, e, h, i])
    } else {
        select_151([a, b, d, c, f, e, g, h])
    }
}
/// n = 11, i = 5, a < f, a < j, a < k, b < h, b < i, c < e, c < i, d < g, d < i, g < h, i < j, i < k
fn select_227([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < f {
        select_224([g, a, b, c, e, h, i, f, j, k])
    } else {
        select_226([b, a, c, d, e, g, f, h, i])
    }
}
/// n = 13, i = 5, a < i, a < m, b < f, b < j, b < m, c < e, c < h, d < g, d < h, g < i, h < j, h < m, j < k, j < l
fn select_228([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_218([e, g, b, h, f, a, j, k, l, m, i])
    } else {
        select_227([b, a, c, d, e, f, g, i, h, j, m])
    }
}
/// n = 14, i = 5, a < h, a < k, b < f, b < i, c < g, c < i, d < e, d < h, d < l, g < k, h < j, i < l, l < m, l < n
fn select_229([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_209([b, c, a, d, e, f, g, h, j, k, i])
    } else {
        select_228([a, d, b, c, f, e, g, i, k, l, m, n, h])
    }
}
/// n = 6, i = 3, a < d, b < c, c < e, d < f
fn select_230([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < c {
        select_3([f, b, c])
    } else {
        select_3([e, a, d])
    }
}
/// n = 9, i = 4, a < e, a < g, b < e, b < h, c < d, d < g, d < h, e < f, h < i
fn select_231([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < h {
        select_230([b, c, d, e, g, f])
    } else {
        select_12([a, b, g, h, i])
    }
}
/// n = 9, i = 4, a < f, b < f, b < h, c < d, d < e, d < h, f < g, h < i
fn select_232([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_231([a, b, c, d, f, g, e, h, i])
    } else {
        select_12([e, b, a, h, i])
    }
}
/// n = 11, i = 4, a < f, a < k, b < f, b < i, b < j, c < d, d < e, d < g, f < h, g < i, g < j, g < k
fn select_233([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_170([e, b, g, a, k, i, j])
    } else {
        select_232([b, a, c, d, e, f, h, g, k])
    }
}
/// n = 7, i = 3, a < d, b < c, b < g, c < f, d < e, d < g
fn select_234([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_3([f, a, d])
    } else {
        select_4([e, b, c, g])
    }
}
/// n = 11, i = 4, a < e, a < g, b < e, b < h, c < d, d < g, d < h, e < f, h < i, h < j, h < k
fn select_235([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_23([a, i, j, k])
    } else {
        select_234([c, b, e, d, g, f, h])
    }
}
/// n = 11, i = 4, a < f, b < f, b < h, c < d, d < e, d < h, f < g, h < i, h < j, h < k
fn select_236([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_214([e, b, a, h, i, j, k])
    } else {
        select_235([a, b, c, d, f, g, e, h, i, j, k])
    }
}
/// n = 13, i = 4, a < f, a < m, b < f, b < i, b < l, c < d, d < e, d < g, d < i, f < h, g < l, g < m, i < j, i < k
fn select_237([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < i {
        select_233([a, b, c, d, e, f, g, h, i, l, m])
    } else {
        select_236([a, b, c, d, e, f, h, i, g, j, k])
    }
}
/// n = 7, i = 1, b < g, c < d, c < e, c < f
fn select_238([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_40([a, c, d, e, f, b])
    } else {
        select_29([b, c, d, e, f, a, g])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < i, d < h, e < f, e < g
fn select_239([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_238([a, d, e, b, f, g, h])
    } else {
        select_170([a, b, c, d, h, e, i])
    }
}
/// n = 9, i = 4, a < e, b < c, b < d, b < i, d < f, e < g, g < h, g < i
fn select_240([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_71([c, a, d, f, e, g])
    } else {
        select_8([b, g, h, c, d, i])
    }
}
/// n = 13, i = 5, a < f, b < e, c < d, c < f, c < j, d < m, e < g, f < i, g < h, g < j, g < m, j < k, j < l
fn select_241([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_239([h, a, c, d, j, k, l, m, f])
    } else {
        select_240([b, c, d, f, e, i, g, h, j])
    }
}
/// n = 14, i = 5, a < f, a < n, b < d, b < f, b < k, c < e, e < g, f < i, g < h, g < j, g < k, j < n, k < l, k < m
fn select_242([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < d {
        select_237([a, b, e, g, h, f, j, i, k, l, m, d, n])
    } else {
        select_241([a, c, b, d, e, f, g, h, i, k, l, m, j])
    }
}
/// n = 6, i = 2, a < e, a < f, b < c, b < d
fn select_243([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < e {
        select_75([a, b, c, d, f])
    } else {
        select_75([b, a, e, f, d])
    }
}
/// n = 7, i = 3, a < f, b < d, b < e, c < e, c < f, f < g
fn select_244([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_12([b, a, e, f, g])
    } else {
        select_180([a, d, c, e])
    }
}
/// n = 7, i = 3, b < d, b < g, c < e, c < g, e < f
fn select_245([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_243([b, e, a, f, d, g])
    } else {
        select_244([a, b, c, d, g, e, f])
    }
}
/// n = 7, i = 3, a < g, b < g, c < e, d < e, d < g, e < f
fn select_246([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_4([b, e, a, f])
    } else {
        select_111([b, c, a, d, e, g])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, c < g, c < i, d < e, e < f, f < i, g < h
fn select_247([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_146([a, g, d, e, h, f])
    } else {
        select_111([a, b, c, f, g, i])
    }
}
/// n = 11, i = 5, a < g, a < k, b < h, b < i, b < k, c < g, c < i, d < e, e < f, e < i, f < k, g < h, i < j
fn select_248([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < g {
        select_246([a, f, c, b, i, j, k])
    } else {
        select_247([b, c, a, d, e, f, g, h, k])
    }
}
/// n = 9, i = 3, a < c, b < d, c < f, d < e, d < g, g < h, g < i
fn select_249([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_5([e, g, a, h, i])
    } else {
        select_234([b, a, c, d, e, f, g])
    }
}
/// n = 8, i = 2, a < c, b < d, c < h, d < e, d < f, d < g
fn select_250([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_23([a, e, f, g])
    } else {
        select_38([b, c, h, d])
    }
}
/// n = 8, i = 4, a < d, b < f, b < g, c < e, d < f, e < g, g < h
fn select_251([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_12([a, b, d, g, h])
    } else {
        select_69([b, c, e, f])
    }
}
/// n = 13, i = 5, a < f, a < h, b < f, b < i, c < g, c < h, c < i, d < e, e < h, e < i, f < g, h < m, i < j, i < k, i < l
fn select_252([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_250([a, b, h, i, j, k, l, m])
    } else {
        select_251([b, c, d, f, e, g, h, m])
    }
}
/// n = 13, i = 5, a < f, a < j, b < f, b < g, c < g, c < i, c < j, d < e, d < g, e < j, e < k, e < l, f < i, g < h, g < k, g < l, j < m
fn select_253([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_249([a, d, f, g, h, i, e, k, l])
    } else {
        select_252([a, b, c, d, e, f, i, j, g, h, k, l, m])
    }
}
/// n = 13, i = 5, a < g, a < k, b < g, b < h, c < h, c < j, c < k, d < e, d < h, e < f, e < k, e < l, g < j, h < i, h < l, k < m
fn select_254([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < h {
        select_248([b, c, a, d, e, f, g, j, k, m, h])
    } else {
        select_253([a, b, c, d, e, g, h, i, j, k, f, l, m])
    }
}
/// n = 14, i = 6, a < h, a < i, b < e, b < k, c < d, c < f, d < k, d < m, e < g, e < i, e < n, f < h, f < k, h < m, i < j, k < l, k < n
fn select_255([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < d {
        select_245([g, i, c, j, f, h, d])
    } else {
        select_254([a, f, d, b, e, g, h, k, l, m, i, n, j])
    }
}
/// n = 16, i = 6, a < h, a < i, b < f, b < j, c < d, c < e, d < i, e < j, e < m, f < g, f < h, f < n, h < l, i < m, j < k, j < n, n < o, n < p
fn select_256([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < d {
        select_242([a, f, c, g, e, h, j, k, l, d, n, o, p, i])
    } else {
        select_255([a, b, c, e, f, d, g, i, h, l, j, k, m, n])
    }
}
/// n = 8, i = 1, c < g, c < h, d < e, d < f
fn select_257([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_153([a, b, c, d, g, h])
    } else {
        select_153([a, b, d, c, e, f])
    }
}
/// n = 9, i = 2, c < d, c < e, c < f, e < i, f < g, f < h
fn select_258([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_257([b, d, e, f, g, h, a, i])
    } else {
        select_154([b, a, c, d, f, g, h, e])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, c < g, e < j, f < i, g < h
fn select_259([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_258([a, b, c, d, g, e, f, j, h])
    } else {
        select_258([a, b, c, d, g, f, e, i, h])
    }
}
/// n = 8, i = 1, b < g, b < h, c < d, c < e, c < f
fn select_260([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_40([a, c, d, e, f, b])
    } else {
        select_30([b, c, d, e, f, a, g, h])
    }
}
/// n = 7, i = 2, c < d, c < e, d < g, e < f
fn select_261([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_6([b, e, d, a, g, f])
    } else {
        select_64([b, a, c, e, f, d])
    }
}
/// n = 11, i = 3, b < d, b < f, c < e, c < f, e < g, f < k, g < h, g < i, g < j
fn select_262([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_260([a, b, g, h, i, j, d, f])
    } else {
        select_261([a, d, c, e, f, k, g])
    }
}
/// n = 6, i = 1, b < c, b < d, b < e, b < f
fn select_263([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_23([c, d, e, f])
    } else {
        select_5([a, b, d, e, f])
    }
}
/// n = 8, i = 3, a < h, b < g, c < e, c < g, c < h, d < e, d < g, d < h, e < f
fn select_264([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_83([a, c, d, b, e, f, h])
    } else {
        select_83([b, c, d, a, e, f, g])
    }
}
/// n = 11, i = 3, a < k, b < e, b < k, c < f, d < e, d < f, d < k, e < j, f < g, f < h, f < i
fn select_265([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_263([a, f, b, g, h, i])
    } else {
        select_264([a, c, b, d, e, j, f, k])
    }
}
/// n = 7, i = 3, a < e, b < e, b < f, b < g, c < d, d < f, d < g
fn select_266([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_4([a, b, f, g])
    } else {
        select_69([b, c, d, e])
    }
}
/// n = 8, i = 3, a < e, b < e, b < h, c < d, d < f, d < h, f < g
fn select_267([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < b {
        select_4([a, f, b, g])
    } else {
        select_266([a, b, c, d, e, f, h])
    }
}
/// n = 13, i = 4, a < f, a < h, b < d, b < g, c < e, d < f, d < m, e < g, e < h, e < m, g < i, h < j, h < k, h < l
fn select_268([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < d {
        select_250([b, a, g, h, j, k, l, i])
    } else {
        select_267([a, d, c, e, f, g, i, m])
    }
}
/// n = 14, i = 5, a < g, a < i, b < h, b < j, c < g, c < i, c < j, d < e, e < f, e < i, e < j, g < h, i < n, j < k, j < l, j < m
fn select_269([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < g {
        select_265([f, a, b, c, i, j, k, l, m, n, g])
    } else {
        select_268([b, a, d, g, e, h, i, j, n, k, l, m, f])
    }
}
/// n = 15, i = 6, a < h, a < i, a < l, b < f, c < d, c < e, d < i, e < k, e < l, f < g, f < h, f < l, h < j, i < k, l < m, l < n, l < o
fn select_270([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < d {
        select_262([g, h, c, j, e, d, l, m, n, o, i])
    } else {
        select_269([d, e, a, b, f, g, i, k, h, l, m, n, o, j])
    }
}
/// n = 17, i = 6, a < h, a < i, b < f, c < d, c < e, d < i, e < k, e < m, f < g, f < h, f < k, h < j, i < m, k < l, k < n, k < o, n < q, o < p
fn select_271([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if k < a {
        select_259([d, g, k, l, a, n, o, p, q, h])
    } else {
        select_270([a, b, c, d, e, f, g, h, i, j, m, k, l, n, o])
    }
}
/// n = 17, i = 6, a < i, a < j, b < f, b < h, c < d, c < e, d < j, e < h, e < m, e < o, f < g, f < i, f < m, h < k, i < l, j < o, m < n, m < p, p < q
fn select_272([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if h < m {
        select_256([a, b, c, d, e, f, g, i, j, h, k, l, o, m, n, p])
    } else {
        select_271([a, b, c, d, e, f, g, i, j, l, m, n, o, h, p, q, k])
    }
}
/// n = 17, i = 6, a < j, a < k, b < e, b < f, c < d, c < g, d < i, e < k, f < m, f < o, g < h, g < j, g < m, j < l, k < o, m < n, m < p, p < q
fn select_273([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if d < f {
        select_229([a, d, b, g, h, i, e, j, f, l, k, m, n, p])
    } else {
        select_272([a, c, b, e, f, g, h, d, j, k, i, l, m, n, o, p, q])
    }
}
/// n = 8, i = 4, b < e, c < g, d < f, f < g, g < h
fn select_274([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_98([b, c, f, a, e, g, h])
    } else {
        select_204([c, b, a, d, e, f, g])
    }
}
/// n = 8, i = 4, b < e, c < g, c < h, d < f, f < g, f < h
fn select_275([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < h {
        select_274([a, b, c, d, e, f, g, h])
    } else {
        select_274([a, b, c, d, e, f, h, g])
    }
}
/// n = 5, i = 1, b < e, c < d
fn select_276([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < c {
        select_4([a, b, c, e])
    } else {
        select_4([a, c, b, d])
    }
}
/// n = 7, i = 3, b < d, c < e, e < f, f < g
fn select_277([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_276([a, b, f, g, d])
    } else {
        select_17([a, d, c, e, f])
    }
}
/// n = 8, i = 4, b < h, c < e, d < f, f < g, f < h
fn select_278([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < g {
        select_275([a, c, b, d, e, f, g, h])
    } else {
        select_277([a, c, f, e, g, b, h])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, c < d, c < e, c < f, c < g
fn select_279([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_61([a, c, d, b, g])
    } else {
        select_8([b, c, d, a, e, f])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < e, c < g, c < h, e < f
fn select_280([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_172([d, b, e, a, f, g, h])
    } else {
        select_279([a, b, c, d, g, h, e])
    }
}
/// n = 5, i = 1, c < d, c < e
fn select_281([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_39([a, c, d, e, b])
    } else {
        select_39([b, c, d, e, a])
    }
}
/// n = 7, i = 3, b < e, b < f, c < d, c < f, f < g
fn select_282([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_97([a, b, c, d, e, f])
    } else {
        select_281([d, e, f, a, g])
    }
}
/// n = 9, i = 3, a < e, b < h, b < i, c < d, c < f, f < g, f < h, f < i
fn select_283([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_280([d, b, f, g, a, e, h, i])
    } else {
        select_282([b, a, c, d, e, f, g])
    }
}
/// n = 10, i = 4, b < e, c < i, c < j, d < f, f < g, f < h, h < i, h < j
fn select_284([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < h {
        select_278([c, a, b, d, e, f, g, h])
    } else {
        select_283([b, c, f, g, e, h, a, i, j])
    }
}
/// n = 7, i = 2, b < g, c < d, c < e, c < g, e < f
fn select_285([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_6([d, b, e, a, f, g])
    } else {
        select_93([a, b, c, d, g, e])
    }
}
/// n = 3, i = 2
fn select_286([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_15([b, c])
    } else {
        select_15([a, c])
    }
}
/// n = 7, i = 4, a < e, b < f, c < f, d < e, d < g, f < g
fn select_287([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_16([a, d, f, g])
    } else {
        select_286([b, c, e])
    }
}
/// n = 8, i = 4, b < f, b < h, c < f, c < h, d < e, d < g, e < h, f < g
fn select_288([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_83([e, b, c, a, f, g, h])
    } else {
        select_287([a, b, c, d, e, f, g])
    }
}
/// n = 8, i = 3, b < h, c < e, c < h, d < e, d < h, e < f, e < g
fn select_289([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_40([b, e, a, f, g, h])
    } else {
        select_111([a, b, c, d, h, e])
    }
}
/// n = 9, i = 4, b < f, b < g, b < i, c < f, c < g, c < i, d < e, d < g, e < i, g < h
fn select_290([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_288([a, b, c, d, e, f, g, i])
    } else {
        select_289([a, e, b, c, g, f, h, i])
    }
}
/// n = 10, i = 4, a < j, b < f, b < i, c < f, c < g, c < i, d < e, d < g, e < i, g < h, g < j
fn select_291([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_285([e, a, g, h, b, f, j])
    } else {
        select_290([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 5, a < h, a < j, b < f, c < e, c < i, d < g, e < j, f < i, g < h, g < i, g < j
fn select_292([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_148([b, a, c, d, e, g, h, f, j])
    } else {
        select_148([a, c, d, b, g, f, e, h, i])
    }
}
/// n = 11, i = 5, a < e, b < h, b < k, c < f, c < i, d < g, f < k, g < h, g < i, g < k, i < j
fn select_293([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < e {
        select_291([a, b, g, c, f, h, i, j, k, e])
    } else {
        select_292([b, a, c, d, f, e, g, h, i, k])
    }
}
/// n = 4, i = 1, c < d
fn select_294([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_38([a, c, d, b])
    } else {
        select_38([b, c, d, a])
    }
}
/// n = 7, i = 3, a < g, b < d, c < e, d < g, e < f
fn select_295([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_294([a, f, b, d])
    } else {
        select_182([a, d, c, e, f, g])
    }
}
/// n = 9, i = 3, a < e, b < g, b < i, c < d, c < g, d < i, e < f, g < h
fn select_296([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_64([a, d, b, g, h, i])
    } else {
        select_295([b, c, a, d, e, f, i])
    }
}
/// n = 5, i = 2, a < e, b < e, c < d
fn select_297([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_12([c, a, d, b, e])
    } else {
        select_12([c, b, d, a, e])
    }
}
/// n = 7, i = 3, a < g, b < d, b < g, c < e, c < g, e < f
fn select_298([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_297([a, b, e, f, g])
    } else {
        select_97([a, b, c, e, d, g])
    }
}
/// n = 9, i = 3, a < f, b < d, b < e, c < e, c < f, c < h, d < h, e < g, f < i
fn select_299([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_13([d, c, a, e, g, h])
    } else {
        select_49([a, b, c, d, e, f, g, i])
    }
}
/// n = 10, i = 4, a < f, b < h, b < j, c < e, c < h, d < f, d < h, d < j, e < j, f < g, h < i
fn select_300([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_298([b, c, d, e, f, g, h])
    } else {
        select_299([a, c, d, e, h, f, i, j, g])
    }
}
/// n = 10, i = 4, a < g, b < f, b < i, c < e, c < i, d < f, d < g, d < i, g < h, i < j
fn select_301([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_296([a, c, b, f, g, h, i, j, e])
    } else {
        select_300([a, b, c, d, e, g, h, i, j, f])
    }
}
/// n = 6, i = 3, a < d, b < e, c < e, e < f
fn select_302([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < d {
        select_3([a, e, f])
    } else {
        select_286([b, c, d])
    }
}
/// n = 8, i = 3, a < h, b < d, b < f, b < g, c < e, e < f, e < g, e < h
fn select_303([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_302([b, a, c, d, e, h])
    } else {
        select_126([b, e, d, a, h, f, g])
    }
}
/// n = 9, i = 3, a < d, a < i, b < c, b < g, b < h, c < f, c < i, d < e, d < g, d < h
fn select_304([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_4([f, a, d, i])
    } else {
        select_5([e, b, c, g, h])
    }
}
/// n = 10, i = 4, a < e, b < c, b < d, d < f, d < i, d < j, e < g, e < i, e < j, g < h
fn select_305([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < g {
        select_303([c, d, a, f, e, i, j, g])
    } else {
        select_304([b, e, g, d, f, h, i, j, c])
    }
}
/// n = 12, i = 4, a < d, a < i, b < c, b < e, d < g, d < k, d < l, e < f, e < i, e < k, e < l, g < h, i < j
fn select_306([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_175([c, a, e, f, d, i, j, k, l])
    } else {
        select_305([a, b, c, e, d, f, g, h, k, l])
    }
}
/// n = 13, i = 5, a < g, a < h, b < g, b < h, b < j, c < d, c < e, d < f, e < j, e < l, e < m, h < i, h < l, h < m, j < k
fn select_307([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < h {
        select_301([e, a, d, b, f, g, j, k, h, i])
    } else {
        select_306([c, a, g, e, h, i, j, k, d, f, l, m])
    }
}
/// n = 14, i = 6, a < i, a < j, b < f, c < d, c < e, d < h, e < g, e < l, e < n, f < i, f < j, f < l, j < k, j < n, l < m
fn select_308([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < j {
        select_293([d, a, e, b, h, g, f, i, l, m, j])
    } else {
        select_307([a, f, c, d, e, h, i, j, k, l, m, g, n])
    }
}
/// n = 14, i = 6, a < i, a < k, b < f, b < j, c < d, c < e, d < h, e < g, e < j, e < n, f < i, f < k, j < l, k < m, k < n
fn select_309([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < f {
        select_284([g, d, a, b, h, j, l, f, i, k])
    } else {
        select_308([a, b, c, d, e, f, g, h, i, k, m, j, l, n])
    }
}
/// n = 7, i = 3, b < d, b < g, c < e, e < f, e < g
fn select_310([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_281([a, f, b, d, g])
    } else {
        select_198([a, d, c, e, f, g])
    }
}
/// n = 7, i = 4, a < g, b < e, c < d, c < g, e < f
fn select_311([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_68([a, c, e, f, d])
    } else {
        select_68([d, a, b, e, g])
    }
}
/// n = 8, i = 4, b < f, c < d, c < e, e < h, f < g, f < h
fn select_312([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_310([d, e, b, a, f, g, h])
    } else {
        select_311([a, b, c, d, f, g, e])
    }
}
/// n = 7, i = 2, c < d, d < e, d < f, d < g
fn select_313([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_214([a, c, b, d, e, f, g])
    } else {
        select_214([b, c, a, d, e, f, g])
    }
}
/// n = 10, i = 4, b < e, c < d, c < h, d < g, e < f, e < h, h < i, h < j
fn select_314([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_313([a, f, c, h, d, i, j])
    } else {
        select_123([a, b, c, d, e, g, f, h])
    }
}
/// n = 10, i = 4, b < e, c < h, d < f, f < g, f < h, h < i, h < j
fn select_315([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_312([a, d, b, e, c, f, g, h])
    } else {
        select_314([a, d, c, b, f, g, e, h, i, j])
    }
}
/// n = 7, i = 3, a < e, b < c, b < d, b < g, e < f, e < g
fn select_316([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < c {
        select_8([b, e, f, d, c, g])
    } else {
        select_16([d, a, c, e])
    }
}
/// n = 9, i = 3, a < e, a < g, b < c, b < d, b < g, e < f, g < h, g < i
fn select_317([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_313([c, d, a, g, e, h, i])
    } else {
        select_316([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 4, a < e, a < i, b < f, b < i, c < f, c < i, d < f, d < i, f < g, f < h
fn select_318([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_39([a, f, g, h, i])
    } else {
        select_149([e, b, c, d, i])
    }
}
/// n = 12, i = 4, a < e, b < f, b < g, c < f, c < g, d < f, d < g, f < k, f < l, g < h, g < i, g < j
fn select_319([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_25([f, g, a, h, i, j, k, l])
    } else {
        select_318([a, b, c, d, e, f, k, l, g])
    }
}
/// n = 13, i = 5, a < g, a < j, b < g, b < j, c < d, c < e, d < f, e < j, e < m, g < h, g < i, g < m, j < k, j < l
fn select_320([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_317([c, g, h, i, d, f, e, j, m])
    } else {
        select_319([d, a, b, e, f, j, g, h, i, m, k, l])
    }
}
/// n = 14, i = 6, a < i, a < l, b < f, c < d, c < e, d < h, e < g, e < l, f < i, f < l, i < j, i < k, l < m, l < n
fn select_321([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < i {
        select_168([d, a, b, e, h, g, f, l, m, n, i])
    } else {
        select_320([a, f, c, d, e, h, i, j, k, l, m, n, g])
    }
}
/// n = 14, i = 6, a < f, b < i, b < j, c < d, c < e, d < h, e < g, e < i, f < j, f < n, i < k, i < n, j < l, j < m
fn select_322([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < f {
        select_315([g, d, a, b, h, i, k, f, j, n])
    } else {
        select_321([b, a, c, d, e, f, g, h, j, l, m, i, k, n])
    }
}
/// n = 8, i = 1, a < h, b < f, b < g, c < d, c < e
fn select_323([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_40([a, b, c, f, g, h])
    } else {
        select_40([a, c, b, d, e, h])
    }
}
/// n = 7, i = 2, a < f, a < g, b < d, b < e, c < d, c < e, c < f, c < g
fn select_324([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_4([b, c, f, g])
    } else {
        select_4([a, c, d, e])
    }
}
/// n = 9, i = 2, a < i, b < g, b < h, c < d, c < g, c < h, c < i, d < e, d < f
fn select_325([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_29([b, d, a, e, f, g, h])
    } else {
        select_324([a, b, c, g, h, d, i])
    }
}
/// n = 9, i = 2, b < h, b < i, c < d, c < e, c < h, c < i, e < f, e < g
fn select_326([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_323([d, b, e, f, g, h, i, a])
    } else {
        select_325([a, b, c, e, f, g, h, i, d])
    }
}
/// n = 10, i = 2, b < i, b < j, c < d, c < e, c < f, c < i, c < j, e < h, f < g
fn select_327([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_326([a, b, c, d, e, f, h, i, j])
    } else {
        select_326([a, b, c, d, f, e, g, i, j])
    }
}
/// n = 14, i = 4, a < c, a < n, b < d, c < f, c < g, d < e, d < h, h < i, h < j, h < k, h < n, j < m, k < l
fn select_328([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < c {
        select_327([e, a, h, i, j, k, l, m, c, n])
    } else {
        select_316([b, c, f, g, d, e, h])
    }
}
/// n = 6, i = 2, a < f, b < d, b < e, c < d, c < e, c < f
fn select_329([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_3([b, c, f])
    } else {
        select_4([a, c, d, e])
    }
}
/// n = 8, i = 4, a < h, b < e, b < g, c < f, d < f, f < g, f < h
fn select_330([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_329([a, b, f, e, g, h])
    } else {
        select_149([a, c, d, e, f])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, b < h, c < f, c < h, d < f, d < h, f < g
fn select_331([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_184([a, b, f, e, g, h])
    } else {
        select_111([a, e, c, d, f, h])
    }
}
/// n = 10, i = 4, a < j, b < e, b < g, c < f, c < g, d < f, d < g, f < j, g < h, g < i
fn select_332([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_153([e, f, g, a, h, i])
    } else {
        select_331([a, b, c, d, e, f, j, g])
    }
}
/// n = 11, i = 5, a < k, b < e, b < h, c < g, d < f, d < h, f < k, g < h, g < k, h < i, h < j
fn select_333([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_330([a, b, c, f, e, g, h, k])
    } else {
        select_332([a, b, d, g, e, f, h, i, j, k])
    }
}
/// n = 8, i = 3, a < e, a < g, b < c, b < d, b < g, e < f, g < h
fn select_334([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_64([d, c, a, g, h, e])
    } else {
        select_9([e, b, d, f, g, h, c])
    }
}
/// n = 9, i = 2, a < c, b < d, c < h, c < i, d < e, d < f, d < g
fn select_335([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_23([a, e, f, g])
    } else {
        select_39([b, c, h, i, d])
    }
}
/// n = 7, i = 3, c < e, d < e, e < f, e < g
fn select_336([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_149([b, a, c, d, e])
    } else {
        select_5([b, e, a, f, g])
    }
}
/// n = 12, i = 4, a < g, b < e, b < f, c < f, c < g, d < f, d < g, f < h, f < i, g < j, g < k, g < l
fn select_337([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_335([b, a, f, g, j, k, l, h, i])
    } else {
        select_336([a, e, c, d, f, h, i])
    }
}
/// n = 13, i = 5, a < g, b < g, b < j, c < d, c < e, d < j, d < m, e < f, e < j, g < h, g < i, g < m, j < k, j < l
fn select_338([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < d {
        select_334([c, g, h, i, e, f, d, m])
    } else {
        select_337([a, e, b, d, f, j, g, k, l, h, i, m])
    }
}
/// n = 14, i = 6, a < i, b < f, c < d, c < e, d < h, d < l, e < g, e < l, f < i, f < l, i < j, i < k, l < m, l < n
fn select_339([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < i {
        select_333([a, d, b, e, h, g, f, l, m, n, i])
    } else {
        select_338([a, f, c, e, d, h, i, j, k, l, m, n, g])
    }
}
/// n = 16, i = 6, a < i, a < p, b < f, c < d, c < e, d < h, e < g, e < l, f < i, f < l, i < j, i < k, l < m, l < n, l < p, n < o
fn select_340([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < d {
        select_328([a, c, i, e, g, j, k, l, m, d, n, o, h, p])
    } else {
        select_339([a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 17, i = 6, a < i, a < j, b < f, c < d, c < e, d < h, e < g, e < i, e < n, f < j, f < n, i < k, j < l, j < m, n < o, n < p, p < q
fn select_341([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if i < n {
        select_322([b, a, c, d, e, f, g, h, i, j, k, l, m, n])
    } else {
        select_340([a, b, c, d, e, f, g, h, j, l, m, n, o, p, q, i])
    }
}
/// n = 17, i = 6, a < e, a < k, b < f, b < j, c < d, c < g, d < i, e < j, e < m, f < m, f < n, g < h, g < k, g < n, k < l, n < o, n < p, p < q
fn select_342([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if e < f {
        select_309([b, a, c, d, g, e, h, i, j, k, f, l, m, n])
    } else {
        select_341([a, b, c, d, g, f, h, i, k, e, l, j, m, n, o, p, q])
    }
}
/// n = 17, i = 6, a < f, a < k, b < d, b < g, c < e, c < h, e < j, f < m, g < m, g < n, h < i, h < k, h < n, k < l, n < o, n < p, p < q
fn select_343([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if d < f {
        select_273([a, b, c, e, d, g, h, i, j, k, f, l, n, o, m, p, q])
    } else {
        select_342([a, b, c, e, f, g, h, i, j, d, k, l, m, n, o, p, q])
    }
}
/// n = 17, i = 6, a < e, a < p, b < g, b < l, c < k, c < m, d < f, d < h, f < j, g < k, h < i, h < l, h < m, l < o, m < n, m < p, p < q
fn select_344([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if c < a {
        select_193([b, c, d, f, g, a, h, i, j, e, k, l, m, n, o, p, q])
    } else {
        select_343([b, a, d, e, f, g, c, h, i, j, l, o, k, m, n, p, q])
    }
}
/// n = 6, i = 2, c < d, c < e, e < f
fn select_345([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_276([b, d, e, f, a])
    } else {
        select_64([b, a, c, e, f, d])
    }
}
/// n = 8, i = 2, b < d, c < e, d < h, e < f, e < g
fn select_346([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_5([a, e, b, f, g])
    } else {
        select_64([a, c, b, d, h, e])
    }
}
/// n = 10, i = 4, a < i, b < d, c < e, c < h, d < g, d < h, e < f, e < i, i < j
fn select_347([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_345([a, g, c, h, e, f])
    } else {
        select_346([f, a, b, i, d, g, h, j])
    }
}
/// n = 8, i = 3, a < h, b < d, b < h, c < e, c < h, e < f, f < g
fn select_348([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_276([a, b, f, g, d])
    } else {
        select_64([d, a, c, e, f, h])
    }
}
/// n = 8, i = 3, b < h, c < h, d < e, d < h, e < f, f < g
fn select_349([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_277([c, a, d, b, e, f, g])
    } else {
        select_348([c, b, d, a, e, f, g, h])
    }
}
/// n = 7, i = 2, a < c, b < d, b < e, d < g, e < f
fn select_350([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_39([e, d, a, g, f])
    } else {
        select_13([a, b, c, e, f, d])
    }
}
/// n = 9, i = 3, a < d, a < i, b < f, b < h, c < e, c < f, e < h, e < i, f < g
fn select_351([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_64([d, b, c, e, i, f])
    } else {
        select_350([a, c, d, e, f, g, h])
    }
}
/// n = 11, i = 4, a < h, a < k, b < h, b < k, c < e, c < g, d < f, d < h, f < g, f < k, g < i, h < j
fn select_352([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < a {
        select_349([e, a, b, d, f, g, i, h])
    } else {
        select_351([c, b, d, e, f, h, j, k, g])
    }
}
/// n = 12, i = 5, a < h, a < l, b < f, b < h, c < d, c < e, d < l, e < g, e < j, f < j, f < l, h < i, j < k
fn select_353([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < d {
        select_347([f, a, c, h, e, g, i, d, j, k])
    } else {
        select_352([a, d, e, b, g, f, j, h, k, i, l])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, c < g, d < e, d < f
fn select_354([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_22([a, e, f])
    } else {
        select_39([c, b, d, h, g])
    }
}
/// n = 9, i = 3, a < f, a < i, b < e, b < f, c < d, c < e, d < i, e < g, f < h
fn select_355([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_182([a, d, b, e, g, i])
    } else {
        select_105([a, c, e, f, g, h])
    }
}
/// n = 10, i = 3, a < d, a < i, b < f, b < h, b < j, c < e, c < f, d < j, e < h, e < i, f < g
fn select_356([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_354([b, e, f, a, d, i, g, h])
    } else {
        select_355([b, c, a, d, e, f, i, g, j])
    }
}
/// n = 8, i = 4, a < e, a < f, b < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_357([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_12([a, d, f, g, h])
    } else {
        select_70([b, c, d, e, f])
    }
}
/// n = 9, i = 4, a < g, b < e, b < f, c < f, c < g, d < e, d < g, d < h, f < h, g < i
fn select_358([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_357([b, a, c, d, e, f, g, i])
    } else {
        select_61([d, f, a, e, h])
    }
}
/// n = 10, i = 4, a < g, a < i, b < g, b < h, c < e, c < f, d < f, d < g, d < i, e < i, f < h, g < j
fn select_359([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_97([a, d, e, b, f, i])
    } else {
        select_358([a, c, d, b, e, f, g, h, j])
    }
}
/// n = 11, i = 4, a < e, a < g, b < h, b < j, b < k, c < f, c < h, d < f, d < g, d < h, f < j, g < k, h < i
fn select_360([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_356([a, b, c, g, f, h, i, j, e, k])
    } else {
        select_359([c, b, a, d, e, g, h, k, f, i])
    }
}
/// n = 9, i = 3, a < c, a < i, b < d, c < e, d < f, d < i, f < g, f < h
fn select_361([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < c {
        select_39([a, f, g, h, i])
    } else {
        select_12([c, b, e, d, i])
    }
}
/// n = 10, i = 4, a < f, a < j, b < f, b < g, c < d, c < g, d < e, d < i, f < h, g < i, g < j
fn select_362([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_361([a, c, f, g, h, d, e, i, j])
    } else {
        select_232([a, b, c, d, e, f, h, g, i])
    }
}
/// n = 12, i = 4, a < g, a < k, a < l, b < f, b < g, b < j, c < d, c < f, d < e, d < h, f < l, g < i, h < j, h < k
fn select_363([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_362([a, b, c, d, e, g, f, i, h, l])
    } else {
        select_233([a, b, c, d, e, g, h, i, f, j, k])
    }
}
/// n = 14, i = 5, a < h, a < k, a < m, b < e, b < i, b < k, c < d, c < f, d < k, e < h, e < l, f < g, f < i, h < j, i < l, i < m, k < n
fn select_364([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < e {
        select_360([f, a, d, b, g, e, i, k, n, h, m])
    } else {
        select_363([a, e, c, f, g, d, h, i, j, l, m, k])
    }
}
/// n = 14, i = 5, a < h, a < n, b < j, b < k, c < h, c < k, c < l, d < e, d < f, e < k, f < g, f < j, h < i, j < l, j < n, k < m
fn select_365([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_353([c, a, d, e, f, b, g, h, i, j, n, k])
    } else {
        select_364([c, b, d, e, a, f, g, h, j, i, k, n, l, m])
    }
}
/// n = 7, i = 2, a < e, b < c, b < d, b < f, f < g
fn select_366([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_281([c, d, f, a, g])
    } else {
        select_82([a, b, c, d, e, f])
    }
}
/// n = 8, i = 3, a < e, b < f, c < d, c < f, f < g, f < h
fn select_367([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_5([d, f, a, g, h])
    } else {
        select_97([b, a, c, d, e, f])
    }
}
/// n = 8, i = 3, a < e, b < d, c < f, f < g, f < h
fn select_368([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_366([b, f, g, h, d, a, e])
    } else {
        select_367([b, c, a, e, d, f, g, h])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < e, b < h, d < g, e < f
fn select_369([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_39([e, d, a, g, f])
    } else {
        select_126([a, b, c, e, f, d, h])
    }
}
/// n = 10, i = 3, a < e, a < g, b < e, b < f, b < i, c < d, c < f, c < g, d < i, f < h, g < j
fn select_370([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_9([b, c, d, e, g, j, f])
    } else {
        select_369([a, c, e, d, f, h, i, g])
    }
}
/// n = 9, i = 4, a < f, a < i, b < g, c < e, c < f, c < h, d < e, d < g, g < h, g < i
fn select_371([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_145([a, c, b, e, g, h, i])
    } else {
        select_68([b, a, d, e, f])
    }
}
/// n = 11, i = 4, a < g, b < f, b < i, c < f, c < g, c < h, d < e, e < h, e < i, h < j, i < k
fn select_372([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_370([b, c, e, a, f, h, i, j, g, k])
    } else {
        select_371([b, d, c, a, g, f, e, h, i])
    }
}
/// n = 11, i = 4, a < g, b < f, b < j, c < f, c < g, c < h, d < e, d < h, e < j, h < i, j < k
fn select_373([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_368([a, b, d, f, g, h, e, i])
    } else {
        select_372([a, b, c, d, e, f, g, h, j, i, k])
    }
}
/// n = 9, i = 5, b < f, b < i, c < f, c < h, d < e, e < h, e < i, f < g
fn select_374([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < h {
        select_123([a, c, d, e, f, i, g, h])
    } else {
        select_71([a, c, b, i, h, f])
    }
}
/// n = 10, i = 5, a < j, b < e, c < f, c < i, d < f, d < h, e < h, e < i, f < g, f < j
fn select_375([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < j {
        select_374([a, c, d, b, e, f, g, h, i])
    } else {
        select_180([b, g, a, j])
    }
}
/// n = 11, i = 4, a < f, a < g, a < h, b < d, b < h, c < e, c < h, d < f, d < j, e < g, e < j, g < i, h < k
fn select_376([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_12([d, c, f, h, k])
    } else {
        select_355([e, a, b, d, h, g, k, i, j])
    }
}
/// n = 6, i = 3, a < f, b < e, c < d, c < f
fn select_377([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < d {
        select_146([a, c, b, e, f, d])
    } else {
        select_297([b, d, a, f, e])
    }
}
/// n = 7, i = 2, a < d, a < g, b < c, b < g, c < e, d < f
fn select_378([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_38([a, c, e, g])
    } else {
        select_38([b, d, f, g])
    }
}
/// n = 11, i = 5, a < h, a < i, a < k, b < f, c < e, c < k, d < f, d < g, e < g, e < h, f < i, f < k, i < j
fn select_379([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_377([b, a, d, g, h, f])
    } else {
        select_378([a, c, e, i, g, j, k])
    }
}
/// n = 12, i = 5, a < h, a < j, b < g, b < i, b < j, c < f, d < e, d < i, e < g, e < h, f < i, f < j, i < k, j < l
fn select_380([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_376([b, d, f, e, a, g, j, i, l, h, k])
    } else {
        select_379([b, c, d, a, e, f, h, g, j, l, i])
    }
}
/// n = 12, i = 5, a < g, a < j, b < f, c < g, c < h, c < k, d < e, d < k, e < h, e < j, f < k, g < i, k < l
fn select_381([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_375([b, d, a, c, e, g, i, h, j, f])
    } else {
        select_380([a, c, b, d, e, f, h, j, k, g, l, i])
    }
}
/// n = 14, i = 5, a < g, a < i, b < g, b < h, b < m, c < e, c < k, d < f, d < m, e < m, f < h, f < i, f < k, g < j, k < l, m < n
fn select_382([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < g {
        select_373([a, b, f, c, e, h, i, k, l, m, n])
    } else {
        select_381([a, c, b, d, f, e, g, h, j, i, m, n])
    }
}
/// n = 14, i = 5, a < i, a < l, b < i, b < j, b < m, c < g, c < m, d < e, d < f, e < h, f < m, g < j, g < l, i < k, m < n
fn select_383([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < g {
        select_365([a, c, b, d, f, e, h, i, k, g, m, j, n, l])
    } else {
        select_382([a, b, d, c, f, g, i, j, l, k, e, h, m, n])
    }
}
/// n = 8, i = 3, a < e, b < d, b < h, c < f, f < g, f < h
fn select_384([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_9([b, f, g, d, a, e, h])
    } else {
        select_98([b, c, a, e, d, f, g])
    }
}
/// n = 12, i = 5, a < j, b < f, b < h, c < f, c < g, c < l, d < e, e < g, e < h, e < j, f < i, j < k, j < l
fn select_385([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < f {
        select_384([b, c, a, g, h, j, k, l])
    } else {
        select_374([a, b, c, d, e, f, i, g, h])
    }
}
/// n = 10, i = 2, b < d, b < j, c < e, c < j, d < i, e < f, e < g, e < h
fn select_386([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_263([a, e, b, f, g, h])
    } else {
        select_170([a, c, b, d, i, e, j])
    }
}
/// n = 8, i = 3, a < h, b < g, b < h, c < e, c < g, d < e, d < g, d < h, e < f
fn select_387([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_63([b, c, d, e, h])
    } else {
        select_64([a, b, d, e, f, g])
    }
}
/// n = 11, i = 3, a < k, b < e, b < k, c < f, c < k, d < e, d < f, e < j, f < g, f < h, f < i
fn select_388([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_263([a, f, b, g, h, i])
    } else {
        select_387([a, c, d, b, e, j, f, k])
    }
}
/// n = 12, i = 3, a < l, b < k, b < l, c < f, c < j, c < l, d < e, d < f, e < j, e < k, f < g, f < h, f < i
fn select_389([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < b {
        select_386([a, e, c, b, f, g, h, i, k, j])
    } else {
        select_388([a, b, c, d, e, f, g, h, i, k, l])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < i, c < h, d < e, d < f, d < g
fn select_390([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_23([a, e, f, g])
    } else {
        select_39([c, b, d, i, h])
    }
}
/// n = 9, i = 2, a < i, b < h, b < i, c < d, c < h, d < e, d < f, d < g
fn select_391([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_250([a, c, b, d, e, f, g, i])
    } else {
        select_390([b, c, a, d, e, f, g, i, h])
    }
}
/// n = 9, i = 3, a < d, b < e, b < i, c < e, c < h, d < h, d < i, e < f, e < g
fn select_392([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_4([a, e, f, g])
    } else {
        select_63([b, c, d, h, i])
    }
}
/// n = 11, i = 5, a < h, b < f, b < k, c < f, c < j, d < e, e < h, e < i, f < g, f < i, h < j, h < k
fn select_393([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_17([a, g, d, e, i])
    } else {
        select_392([a, b, c, h, f, g, i, j, k])
    }
}
/// n = 15, i = 5, a < f, a < o, b < h, b < i, c < f, c < i, c < m, d < e, e < h, e < i, e < n, f < g, f < n, h < m, h < o, i < j, i < k, i < l
fn select_394([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_391([a, h, c, i, j, k, l, m, o])
    } else {
        select_393([b, a, c, d, e, f, g, h, n, m, o])
    }
}
/// n = 15, i = 5, a < g, a < o, b < i, b < j, c < g, c < j, c < n, d < e, e < f, e < i, e < j, g < h, i < n, i < o, j < k, j < l, j < m
fn select_395([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < g {
        select_389([f, a, c, b, i, j, k, l, m, n, o, g])
    } else {
        select_394([a, b, c, d, e, g, h, i, j, k, l, m, n, f, o])
    }
}
/// n = 15, i = 5, a < h, a < k, b < f, c < h, c < i, c < l, d < e, d < l, e < i, e < k, f < g, f < l, h < j, l < m, l < n, l < o
fn select_396([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < f {
        select_385([b, a, c, d, e, h, i, k, j, f, g, l])
    } else {
        select_395([a, d, c, b, f, g, h, j, e, l, m, n, o, i, k])
    }
}
/// n = 9, i = 1, b < i, c < g, c < h, d < e, d < f
fn select_397([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_238([a, b, c, d, g, h, i])
    } else {
        select_238([a, b, d, c, e, f, i])
    }
}
/// n = 12, i = 2, b < d, b < g, c < e, c < f, e < j, f < h, f < i, g < k, g < l
fn select_398([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < c {
        select_257([a, d, c, g, k, l, e, f])
    } else {
        select_397([a, e, b, f, h, i, d, g, j])
    }
}
/// n = 9, i = 2, a < e, a < i, b < c, b < d, b < f, b < i, f < g, f < h
fn select_399([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_130([a, b, e, c, f, g, h, d, i])
    } else {
        select_130([a, b, e, d, f, g, h, c, i])
    }
}
/// n = 12, i = 3, a < f, a < h, b < e, b < g, c < d, c < f, c < g, c < h, g < i, g < j, h < k, h < l
fn select_400([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_399([b, c, d, f, e, h, k, l, g])
    } else {
        select_326([e, a, c, d, g, i, j, f, h])
    }
}
/// n = 13, i = 3, a < e, a < m, b < g, b < i, c < d, c < f, c < g, c < i, f < h, f < m, h < j, i < k, i < l
fn select_401([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_398([d, b, f, g, h, a, i, e, m, j, k, l])
    } else {
        select_400([b, a, c, d, e, g, f, i, h, m, k, l])
    }
}
/// n = 12, i = 4, a < k, a < l, b < e, b < f, c < f, c < g, d < e, d < g, g < h, g < i, g < k, g < l, i < j
fn select_402([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_397([h, i, a, b, e, f, k, l, j])
    } else {
        select_196([a, c, d, e, f, g, h, i])
    }
}
/// n = 8, i = 2, c < d, c < e, d < h, e < f, e < g
fn select_403([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_198([a, b, c, d, e, h])
    } else {
        select_313([a, b, c, e, d, f, g])
    }
}
/// n = 11, i = 5, b < f, b < i, c < f, c < h, d < e, d < j, e < h, e < i, f < g, f < j, j < k
fn select_404([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_196([a, b, c, h, i, f, g, j])
    } else {
        select_403([a, g, d, j, e, h, i, k])
    }
}
/// n = 13, i = 5, a < l, a < m, b < f, b < i, b < j, c < f, c < h, c < j, d < e, d < j, e < h, e < i, f < g, j < k, j < l, j < m
fn select_405([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < f {
        select_402([a, e, b, c, h, i, j, k, f, g, l, m])
    } else {
        select_404([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 15, i = 5, a < f, a < k, b < l, b < m, c < f, c < g, c < i, d < e, d < g, e < i, e < k, f < j, g < h, g < l, g < m, m < n, m < o
fn select_406([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < a {
        select_401([e, b, g, h, i, a, l, f, m, j, n, o, k])
    } else {
        select_405([b, a, c, d, e, f, j, i, k, g, h, l, m])
    }
}
/// n = 15, i = 5, a < h, a < m, b < f, b < n, c < h, c < i, c < k, d < e, d < i, e < k, e < m, f < g, f < o, h < l, i < j, i < n, i < o
fn select_407([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < i {
        select_396([a, b, c, d, e, f, g, h, k, l, m, i, j, n, o])
    } else {
        select_406([a, b, c, d, e, h, i, j, k, l, m, n, f, g, o])
    }
}
/// n = 15, i = 5, a < i, a < n, b < e, b < g, c < i, c < j, c < l, d < f, d < j, f < l, f < n, g < h, g < o, i < m, j < k, j < o
fn select_408([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < j {
        select_383([a, c, d, b, g, e, f, h, i, l, m, n, j, k])
    } else {
        select_407([a, b, c, d, f, g, h, i, j, k, l, m, n, e, o])
    }
}
/// n = 9, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, f < i, g < h
fn select_409([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_6([d, g, f, a, i, h])
    } else {
        select_99([a, b, c, d, e, g, h, f])
    }
}
/// n = 9, i = 3, a < e, b < f, b < g, c < d, c < g, c < i, f < i, g < h
fn select_410([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < c {
        select_214([a, f, e, c, d, g, i])
    } else {
        select_409([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 8, i = 4, a < g, b < e, b < f, c < f, c < g, d < f, d < g, g < h
fn select_411([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_357([b, c, d, a, e, f, g, h])
    } else {
        select_70([c, d, e, a, f])
    }
}
/// n = 10, i = 4, a < e, b < g, b < i, b < j, c < f, c < g, d < f, d < g, f < i, f < j, g < h
fn select_412([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_47([b, f, a, g, e, h, i, j])
    } else {
        select_411([b, a, c, d, e, f, g, h])
    }
}
/// n = 11, i = 4, a < e, b < g, b < h, b < k, c < f, c < h, c < j, d < f, d < g, d < h, f < k, g < j, h < i
fn select_413([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_412([a, b, c, d, e, f, h, i, g, k])
    } else {
        select_412([a, c, b, d, e, g, h, i, f, j])
    }
}
/// n = 12, i = 4, a < e, b < i, b < l, c < g, c < h, c < i, d < f, d < g, d < i, f < h, g < l, h < j, i < k
fn select_414([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_410([a, f, c, g, e, b, h, j, i])
    } else {
        select_413([a, b, c, d, e, g, f, i, k, h, l])
    }
}
/// n = 9, i = 4, a < d, b < c, b < i, c < e, c < f, d < g, d < i, g < h
fn select_415([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_61([b, g, h, i, c])
    } else {
        select_316([a, c, e, f, d, i, g])
    }
}
/// n = 7, i = 3, a < e, a < g, b < c, b < d, d < g, e < f
fn select_416([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_63([c, d, a, g, e])
    } else {
        select_13([e, b, f, d, g, c])
    }
}
/// n = 9, i = 4, a < d, b < c, b < i, c < f, c < g, d < e, d < h, h < i
fn select_417([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_416([b, d, e, h, c, g, i])
    } else {
        select_221([a, c, f, g, d, e])
    }
}
/// n = 9, i = 4, a < e, a < i, b < c, b < d, c < i, d < h, e < f, e < g
fn select_418([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_415([b, a, e, c, f, g, d, h, i])
    } else {
        select_417([b, a, e, d, h, f, g, c, i])
    }
}
/// n = 7, i = 3, a < e, a < g, b < e, b < f, b < g, c < d, c < g, d < f
fn select_419([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_3([a, b, f])
    } else {
        select_61([b, c, d, e, g])
    }
}
/// n = 7, i = 3, a < g, b < f, b < g, c < e, c < f, d < e, d < f, d < g
fn select_420([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_419([c, d, b, a, e, g, f])
    } else {
        select_146([c, d, a, b, e, g])
    }
}
/// n = 8, i = 3, a < d, a < h, b < c, b < g, c < e, c < h, d < f, d < g
fn select_421([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_4([e, a, d, h])
    } else {
        select_4([f, b, c, g])
    }
}
/// n = 9, i = 4, a < f, a < i, b < f, b < g, b < i, c < d, c < e, e < g, e < i, f < h
fn select_422([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_420([d, a, e, b, g, i, f])
    } else {
        select_421([a, c, e, f, g, h, d, i])
    }
}
/// n = 11, i = 5, a < h, a < k, b < h, b < i, b < k, c < g, c < h, d < e, d < g, e < f, g < i, g < k, h < j
fn select_423([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_163([a, e, b, c, f, g, i, k])
    } else {
        select_422([a, b, d, e, g, h, i, j, k])
    }
}
/// n = 11, i = 5, a < k, b < g, b < j, b < k, c < g, c < h, d < e, d < h, e < f, g < i, h < j, h < k
fn select_424([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_418([b, d, h, e, g, a, i, f, j])
    } else {
        select_423([a, b, c, d, e, f, h, g, j, i, k])
    }
}
/// n = 13, i = 5, a < h, b < k, b < m, c < h, c < j, c < k, d < e, d < f, e < g, f < j, f < k, h < i, j < m, k < l
fn select_425([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_414([e, b, c, f, g, a, j, h, k, i, l, m])
    } else {
        select_424([b, c, a, d, e, g, h, f, i, j, k])
    }
}
/// n = 8, i = 2, a < g, a < h, b < d, b < h, c < e, c < g, e < f
fn select_426([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_14([c, b, d, a, e, h, f])
    } else {
        select_354([c, a, e, b, d, h, f, g])
    }
}
/// n = 8, i = 2, b < h, c < g, c < h, d < e, d < g, e < f
fn select_427([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_134([c, a, d, b, e, f, g])
    } else {
        select_426([c, b, d, a, e, f, g, h])
    }
}
/// n = 8, i = 3, a < g, a < h, b < e, c < d, c < f, d < h, e < f, e < g
fn select_428([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_13([c, e, d, a, g, f])
    } else {
        select_76([b, c, a, d, e, g, h])
    }
}
/// n = 9, i = 3, a < g, a < h, b < d, c < e, c < i, d < i, e < f, e < g, e < h
fn select_429([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_6([f, b, a, g, h, d])
    } else {
        select_64([a, d, c, e, f, i])
    }
}
/// n = 10, i = 3, a < e, a < j, b < d, b < h, c < f, c < i, d < j, e < i, f < g, f < h
fn select_430([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_428([c, b, a, e, d, j, f, i])
    } else {
        select_429([b, a, c, e, f, g, d, h, i])
    }
}
/// n = 12, i = 4, a < h, a < l, b < g, b < i, c < g, c < h, d < e, d < k, e < f, e < i, g < l, h < k, i < j
fn select_431([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < c {
        select_427([f, a, c, b, i, j, g, h])
    } else {
        select_430([a, b, d, g, h, e, f, i, k, l])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < e
fn select_432([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_146([a, b, c, d, f, e])
    } else {
        select_12([a, c, f, e, d])
    }
}
/// n = 8, i = 3, a < d, b < f, b < g, b < h, c < e, c < f, e < g, e < h
fn select_433([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_432([b, c, a, d, e, f])
    } else {
        select_126([b, e, f, a, d, g, h])
    }
}
/// n = 8, i = 4, a < d, b < f, c < f, d < e, f < g, g < h
fn select_434([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_16([e, b, c, f])
    } else {
        select_38([a, g, h, d])
    }
}
/// n = 9, i = 3, a < c, a < h, b < d, b < i, c < e, c < i, d < f, d < h, f < g
fn select_435([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_4([e, b, d, i])
    } else {
        select_39([f, a, c, h, g])
    }
}
/// n = 11, i = 5, a < d, a < j, b < f, c < e, c < k, d < g, d < k, e < h, e < j, f < h, f < j, f < k, h < i
fn select_436([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < f {
        select_71([g, b, c, e, f, k])
    } else {
        select_435([a, c, d, e, g, h, i, j, k])
    }
}
/// n = 11, i = 5, a < d, a < j, b < f, b < k, c < e, c < h, d < g, d < k, e < i, e < j, e < k, f < h, f < j, h < i
fn select_437([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_434([a, c, f, d, g, h, e, i])
    } else {
        select_436([a, c, b, d, f, e, g, h, i, j, k])
    }
}
/// n = 12, i = 5, a < d, a < i, a < k, b < e, b < h, c < f, c < l, d < j, d < l, e < g, e < l, f < h, f < i, h < k, i < j
fn select_438([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < d {
        select_433([e, a, f, g, h, i, d, k])
    } else {
        select_437([b, c, a, e, d, f, g, i, j, h, l])
    }
}
/// n = 9, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < g, e < f, h < i
fn select_439([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_49([a, d, c, g, e, h, f, i])
    } else {
        select_244([a, e, b, f, d, h, i])
    }
}
/// n = 12, i = 5, a < f, a < g, b < d, b < j, c < e, c < j, d < h, d < i, e < g, e < i, f < h, f < j, i < k, j < l
fn select_440([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_12([a, d, g, i, k])
    } else {
        select_439([f, b, c, d, e, g, h, j, l])
    }
}
/// n = 12, i = 5, a < f, a < h, b < d, b < j, c < e, c < j, d < i, e < h, e < i, f < g, f < j, i < k, j < l
fn select_441([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < d {
        select_439([b, f, c, g, e, h, d, j, l])
    } else {
        select_440([a, b, c, d, e, f, h, g, i, j, k, l])
    }
}
/// n = 13, i = 5, a < j, a < m, b < i, b < k, c < e, c < h, d < f, d < j, e < g, e < j, f < h, f < i, h < k, i < m, j < l
fn select_442([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < a {
        select_438([b, c, d, a, e, f, g, h, i, m, k, j])
    } else {
        select_441([c, a, d, b, f, e, g, h, i, j, m, l])
    }
}
/// n = 13, i = 5, a < k, a < l, b < h, b < m, c < e, c < j, d < f, d < h, e < g, f < j, f < k, h < i, j < l, k < m
fn select_443([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_431([a, c, f, b, h, i, j, k, e, g, m, l])
    } else {
        select_442([b, a, c, d, e, f, g, j, k, h, l, i, m])
    }
}
/// n = 14, i = 5, a < i, a < n, b < k, b < l, c < e, c < g, d < f, d < i, d < l, e < h, f < k, g < l, i < j, k < n, l < m
fn select_444([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < f {
        select_425([a, b, d, c, e, g, h, i, j, f, l, m, k])
    } else {
        select_443([b, a, c, d, e, f, h, i, j, g, k, l, n])
    }
}
/// n = 8, i = 1, b < h, c < g, d < e, d < f
fn select_445([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_172([a, b, d, e, f, c, h])
    } else {
        select_172([a, c, d, e, f, b, g])
    }
}
/// n = 11, i = 2, b < e, b < f, c < d, c < g, e < k, f < j, g < h, g < i
fn select_446([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_445([a, e, f, c, d, g, j, k])
    } else {
        select_257([a, d, b, g, h, i, e, f])
    }
}
/// n = 11, i = 3, a < e, a < g, b < f, b < h, c < d, c < f, c < g, c < h, g < k, h < i, h < j
fn select_447([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_280([e, b, c, d, g, k, f, h])
    } else {
        select_399([a, c, d, f, e, h, i, j, g])
    }
}
/// n = 12, i = 3, a < e, b < g, b < i, c < d, c < f, c < g, c < i, f < h, h < j, i < k, i < l
fn select_448([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_446([d, f, b, g, a, h, i, k, l, j, e])
    } else {
        select_447([a, b, c, d, e, g, f, i, k, l, h])
    }
}
/// n = 10, i = 3, a < c, a < d, b < e, b < j, d < j, e < f, e < g, g < h, g < i
fn select_449([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_92([a, e, f, c, g, h, i])
    } else {
        select_416([b, a, c, d, e, f, j])
    }
}
/// n = 11, i = 3, a < c, a < g, b < d, b < g, c < f, d < e, d < h, g < k, h < i, h < j
fn select_450([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_5([e, h, a, i, j])
    } else {
        select_136([b, a, c, d, e, f, g, k, h])
    }
}
/// n = 11, i = 3, a < d, b < g, c < e, c < g, e < f, e < h, g < k, h < i, h < j
fn select_451([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_449([a, c, d, b, e, f, h, i, j, g])
    } else {
        select_450([b, c, a, e, f, d, g, h, i, j, k])
    }
}
/// n = 9, i = 4, a < g, a < i, b < d, b < g, c < e, c < h, c < i, d < i, e < f, g < h
fn select_452([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_329([d, g, c, e, h, i])
    } else {
        select_166([a, e, b, f, d, i, g])
    }
}
/// n = 9, i = 3, a < d, a < e, b < f, c < e, c < f, f < g, f < h, f < i
fn select_453([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_180([b, d, c, e])
    } else {
        select_214([a, b, e, f, g, h, i])
    }
}
/// n = 10, i = 3, a < d, a < e, b < d, b < f, c < e, c < f, e < j, f < g, f < h, f < i
fn select_454([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_23([a, g, h, i])
    } else {
        select_13([b, c, d, e, j, f])
    }
}
/// n = 11, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < g, e < f, h < i, h < j, h < k
fn select_455([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < d {
        select_453([e, a, b, f, d, h, i, j, k])
    } else {
        select_454([d, a, c, g, e, h, i, j, k, f])
    }
}
/// n = 12, i = 4, a < g, b < h, c < g, c < h, d < e, d < h, d < l, e < f, g < l, h < i, h < j, h < k
fn select_456([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_452([c, a, d, b, e, f, g, l, h])
    } else {
        select_455([c, b, d, a, e, f, g, h, i, j, k])
    }
}
/// n = 12, i = 4, a < j, a < k, b < g, c < g, c < h, d < e, d < h, d < l, e < f, g < l, h < i, h < j, h < k
fn select_457([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_451([e, b, c, f, h, i, g, a, j, k, l])
    } else {
        select_456([b, a, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 14, i = 4, a < i, b < e, b < n, c < j, c < k, d < f, d < i, e < h, f < g, f < j, f < k, i < n, k < l, k < m
fn select_458([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < b {
        select_448([a, c, f, g, i, b, j, e, k, h, l, m])
    } else {
        select_457([c, a, d, b, e, h, i, f, g, j, k, n])
    }
}
/// n = 9, i = 3, a < h, a < i, b < d, b < g, c < e, d < i, e < f, e < g, e < h
fn select_459([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_9([b, e, f, d, a, h, g])
    } else {
        select_72([b, c, a, d, e, f, h, i])
    }
}
/// n = 8, i = 4, a < g, b < e, b < h, c < d, c < g, e < f, g < h
fn select_460([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_71([d, a, b, e, g, h])
    } else {
        select_68([a, c, e, f, d])
    }
}
/// n = 8, i = 4, a < f, a < h, b < f, b < h, c < d, d < e, d < h, f < g
fn select_461([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_12([f, c, g, d, h])
    } else {
        select_16([e, a, b, h])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, c < d, c < h, d < g, e < f, e < h
fn select_462([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < d {
        select_461([a, c, b, e, f, d, g, h])
    } else {
        select_230([b, d, a, e, g, f])
    }
}
/// n = 9, i = 4, a < i, b < g, c < g, c < i, d < e, d < h, e < f, e < i, g < h
fn select_463([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_460([c, d, b, a, e, f, g, h])
    } else {
        select_462([c, d, a, b, e, f, g, i])
    }
}
/// n = 11, i = 4, a < h, b < g, b < k, c < e, c < j, d < g, d < h, e < f, g < j, h < i, h < k
fn select_464([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_459([c, b, a, g, h, i, k, e, j])
    } else {
        select_463([a, b, d, c, e, f, g, j, h])
    }
}
/// n = 9, i = 5, a < g, a < i, b < e, c < d, c < g, e < f, e < i, g < h
fn select_465([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < i {
        select_123([d, a, b, e, g, f, h, i])
    } else {
        select_71([f, a, c, d, i, g])
    }
}
/// n = 10, i = 5, a < g, a < i, a < j, b < e, c < d, c < g, d < j, e < f, e < i, e < j, g < h
fn select_466([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < j {
        select_465([a, b, c, d, e, f, g, h, i])
    } else {
        select_71([f, c, a, i, d, j])
    }
}
/// n = 8, i = 2, a < c, b < d, d < e, d < f, d < g, d < h
fn select_467([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_24([a, e, f, g, h])
    } else {
        select_3([c, b, d])
    }
}
/// n = 9, i = 2, b < i, c < d, c < i, d < e, d < f, d < g, d < h
fn select_468([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_467([a, c, b, d, e, f, g, h])
    } else {
        select_26([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, a < e, a < g, a < j, b < e, b < h, b < j, c < d, d < g, d < h, d < j, e < f, h < i
fn select_469([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_3([f, c, d])
    } else {
        select_126([a, b, g, h, i, e, j])
    }
}
/// n = 11, i = 5, a < g, a < h, a < k, b < e, c < d, c < j, d < h, d < k, e < f, e < g, e < k, g < j, h < i
fn select_470([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_146([f, c, a, g, d, j])
    } else {
        select_469([d, a, b, e, h, i, f, g, j, k])
    }
}
/// n = 14, i = 5, a < g, a < h, a < j, b < e, c < d, c < j, c < n, d < h, e < f, e < g, e < j, g < n, h < i, j < k, j < l, j < m
fn select_471([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < d {
        select_468([f, g, c, j, d, k, l, m, n])
    } else {
        select_470([a, b, c, d, e, f, g, h, i, n, j])
    }
}
/// n = 14, i = 5, a < g, b < j, b < n, c < g, c < i, c < j, d < e, e < f, e < i, e < j, g < h, i < n, j < k, j < l, j < m
fn select_472([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_466([c, d, a, b, e, f, g, h, i, j])
    } else {
        select_471([c, d, b, a, e, f, i, g, h, j, k, l, m, n])
    }
}
/// n = 15, i = 5, a < h, a < o, b < j, b < k, c < f, d < e, d < h, d < k, e < j, f < g, f < k, h < i, j < o, k < l, k < m, k < n
fn select_473([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < f {
        select_464([c, b, a, e, h, i, j, f, g, o, k])
    } else {
        select_472([a, b, d, c, f, g, h, i, e, k, l, m, n, j])
    }
}
/// n = 15, i = 5, a < h, a < o, b < f, b < m, c < i, c < l, d < e, d < h, d < i, e < l, f < g, f < n, h < k, i < j, i < m, i < n, l < o
fn select_474([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_458([e, a, b, c, h, i, j, k, l, m, f, g, n, o])
    } else {
        select_473([a, c, b, d, e, f, g, h, k, l, i, j, m, n, o])
    }
}
/// n = 15, i = 5, a < i, a < o, b < e, b < g, c < j, c < m, d < f, d < i, d < j, f < m, g < h, g < n, i < l, j < k, j < n, m < o
fn select_475([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < j {
        select_444([a, c, b, d, g, f, e, h, i, l, m, j, k, o])
    } else {
        select_474([a, b, c, d, f, g, h, i, j, k, l, m, e, n, o])
    }
}
/// n = 15, i = 5, a < j, a < n, b < f, b < h, c < g, c < k, d < e, d < j, d < k, g < n, h < i, h < o, j < m, k < l, k < o
fn select_476([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < e {
        select_408([a, b, d, c, f, g, h, i, j, k, l, e, m, n, o])
    } else {
        select_475([a, b, c, d, f, e, h, i, j, k, l, m, g, o, n])
    }
}
/// n = 7, i = 3, a < d, b < f, b < g, c < e, c < f, e < g
fn select_477([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_432([b, c, a, d, e, f])
    } else {
        select_13([b, e, f, a, d, g])
    }
}
/// n = 7, i = 2, b < g, c < d, c < g, d < e, d < f
fn select_478([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_40([b, d, a, e, f, g])
    } else {
        select_63([a, b, c, g, d])
    }
}
/// n = 7, i = 3, a < g, b < e, c < d, c < f, d < g, e < f
fn select_479([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_146([b, a, c, d, e, g])
    } else {
        select_61([c, e, a, d, f])
    }
}
/// n = 10, i = 3, a < e, b < f, b < i, c < d, c < f, c < j, d < i, e < j, f < g, f < h
fn select_480([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_478([a, d, b, f, g, h, i])
    } else {
        select_479([b, a, c, d, e, j, i])
    }
}
/// n = 10, i = 3, a < f, b < e, c < g, c < j, d < f, d < g, e < j, g < h, g < i
fn select_481([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < c {
        select_477([b, d, a, e, c, f, g])
    } else {
        select_480([b, d, c, a, e, g, h, i, f, j])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < f
fn select_482([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < a {
        select_75([c, b, a, e, d])
    } else {
        select_13([c, a, d, b, e, f])
    }
}
/// n = 9, i = 3, a < d, a < e, b < f, b < i, c < e, c < f, d < i, f < g, f < h
fn select_483([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_297([b, d, c, e, i])
    } else {
        select_72([a, c, b, d, f, g, h, i])
    }
}
/// n = 10, i = 3, a < f, a < g, b < d, b < e, b < f, c < e, c < g, c < j, d < j, g < h, g < i
fn select_484([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_482([d, a, c, e, f, j])
    } else {
        select_483([b, c, a, d, f, g, h, i, j])
    }
}
/// n = 10, i = 3, a < e, a < j, b < d, c < e, c < h, c < i, d < h, d < i, d < j, e < f, e < g
fn select_485([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_2([h, i])
    } else {
        select_392([b, a, c, d, e, f, g, i, j])
    }
}
/// n = 12, i = 4, a < g, a < k, a < l, b < e, b < f, c < f, c < g, d < e, d < g, d < h, e < l, f < h, f < k, g < i, g < j
fn select_486([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_392([b, a, c, e, g, i, j, f, l])
    } else {
        select_485([a, b, d, f, g, i, j, e, h, k])
    }
}
/// n = 12, i = 4, a < f, b < h, b < i, b < l, c < f, c < g, c < i, d < e, d < i, e < g, e < h, f < l, i < j, i < k
fn select_487([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_484([b, e, c, a, g, h, i, j, k, f])
    } else {
        select_486([b, a, d, c, f, e, i, g, j, k, h, l])
    }
}
/// n = 12, i = 4, a < g, b < h, b < i, b < l, c < f, c < i, d < e, d < g, d < i, f < h, g < l, i < j, i < k
fn select_488([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_481([e, a, b, c, g, f, i, j, k, l])
    } else {
        select_487([a, b, d, c, f, g, e, h, i, j, k, l])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, c < d, c < e, d < g
fn select_489([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < b {
        select_38([a, b, e, g])
    } else {
        select_75([a, c, d, e, f])
    }
}
/// n = 8, i = 3, a < e, b < f, b < g, b < h, c < d, c < f, d < g, e < h
fn select_490([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_13([b, d, f, a, e, g])
    } else {
        select_489([b, c, a, e, d, f, h])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, c < d, d < f, e < g
fn select_491([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_63([a, c, e, d, g])
    } else {
        select_63([a, b, d, e, f])
    }
}
/// n = 10, i = 3, a < e, b < f, b < i, b < j, c < d, c < f, d < i, e < j, f < g, f < h
fn select_492([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_478([a, d, b, f, g, h, i])
    } else {
        select_491([b, a, c, d, e, i, j])
    }
}
/// n = 10, i = 3, a < f, b < e, c < g, d < f, d < g, d < j, e < j, g < h, g < i
fn select_493([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < c {
        select_490([b, d, a, c, e, f, g, j])
    } else {
        select_492([b, d, c, a, e, g, h, i, f, j])
    }
}
/// n = 9, i = 3, a < e, b < d, c < e, c < h, c < i, d < h, d < i, e < f, e < g
fn select_494([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < a {
        select_2([h, i])
    } else {
        select_72([b, a, c, d, e, f, g, i])
    }
}
/// n = 12, i = 4, a < g, a < l, b < e, b < f, c < e, c < g, c < h, d < f, d < g, d < i, e < i, f < h, f < l, g < j, g < k
fn select_495([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_494([a, b, d, e, g, j, k, f, i])
    } else {
        select_485([a, b, c, f, g, j, k, e, h, l])
    }
}
/// n = 12, i = 4, a < f, b < h, b < i, c < f, c < g, c < i, d < e, d < i, d < l, e < g, e < h, f < l, i < j, i < k
fn select_496([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_484([b, e, c, a, g, h, i, j, k, f])
    } else {
        select_495([b, a, c, d, f, e, i, g, l, j, k, h])
    }
}
/// n = 12, i = 4, a < g, b < h, b < i, c < f, c < i, c < l, d < e, d < g, d < i, f < h, g < l, i < j, i < k
fn select_497([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_493([e, a, b, c, g, f, i, j, k, l])
    } else {
        select_496([a, b, d, c, f, g, e, h, i, j, k, l])
    }
}
/// n = 12, i = 4, a < h, b < g, b < i, c < f, c < i, c < l, d < e, d < h, d < i, h < l, i < j, i < k
fn select_498([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_488([a, c, b, d, e, g, h, f, i, j, k, l])
    } else {
        select_497([a, b, c, d, e, f, h, g, i, j, k, l])
    }
}
/// n = 8, i = 2, a < e, a < h, b < c, b < d, b < h, d < f, e < g
fn select_499([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_4([c, d, a, f])
    } else {
        select_126([b, a, c, e, g, d, h])
    }
}
/// n = 10, i = 2, a < c, a < f, a < j, b < d, b < e, b < j, d < h, e < g, f < i
fn select_500([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_369([a, b, c, d, e, g, h, j])
    } else {
        select_499([b, a, c, f, e, i, g, j])
    }
}
/// n = 10, i = 2, a < e, b < g, b < j, c < d, c < f, c < j, f < h, g < i
fn select_501([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_7([a, c, d, e, f, b, h, g])
    } else {
        select_500([c, b, d, a, g, f, i, e, h, j])
    }
}
/// n = 11, i = 3, a < f, b < d, b < h, b < k, c < e, c < f, c < h, e < g, e < k, g < i, h < j
fn select_502([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_500([b, e, d, a, g, h, i, f, j, k])
    } else {
        select_19([a, b, c, d, f, h, e, j, g])
    }
}
/// n = 11, i = 3, b < e, b < h, b < k, c < d, c < f, c < h, f < g, f < k, g < i, h < j
fn select_503([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_501([d, f, b, e, a, h, g, j, i, k])
    } else {
        select_502([a, b, c, e, f, d, g, h, i, j, k])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < i, e < f, e < g, e < h
fn select_504([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_53([b, d, e, f, g, h, a, i])
    } else {
        select_33([a, b, c, e, f, g, h, i, d])
    }
}
/// n = 9, i = 3, a < e, a < i, b < e, b < f, b < i, c < d, c < f, c < i, f < g, f < h
fn select_505([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_32([a, c, e, f, d, g, h, i])
    } else {
        select_419([a, b, c, d, e, f, i])
    }
}
/// n = 11, i = 3, a < e, b < e, b < f, b < g, c < d, c < f, c < g, f < j, f < k, g < h, g < i
fn select_506([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_260([d, g, f, a, j, k, h, i])
    } else {
        select_505([a, b, c, d, e, g, h, i, f])
    }
}
/// n = 12, i = 4, a < f, a < g, a < h, b < e, b < g, b < h, c < d, c < g, c < h, g < k, g < l, h < i, h < j
fn select_507([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_506([d, b, a, f, e, g, h, i, j, k, l])
    } else {
        select_506([e, c, a, f, d, g, h, i, j, k, l])
    }
}
/// n = 13, i = 4, a < e, a < i, a < m, b < f, b < g, b < i, c < d, c < g, c < i, f < j, g < h, g < j, g < m, i < k, i < l
fn select_508([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_504([d, f, g, h, a, e, i, m, j])
    } else {
        select_507([a, b, c, d, f, e, g, i, k, l, h, m])
    }
}
/// n = 14, i = 4, a < f, a < j, b < e, b < i, b < j, c < d, c < g, c < j, f < l, g < h, g < i, g < l, i < k, j < m, j < n
fn select_509([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < a {
        select_503([d, b, g, h, e, a, f, i, l, k, j])
    } else {
        select_508([b, a, c, d, e, f, g, h, j, l, m, n, i])
    }
}
/// n = 14, i = 4, b < g, b < j, c < f, c < i, c < j, d < e, d < h, d < j, g < l, h < i, h < l, i < k, j < m, j < n
fn select_510([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < h {
        select_498([a, b, c, d, e, f, g, h, j, m, n, i])
    } else {
        select_509([b, c, d, e, f, g, h, a, i, j, k, l, m, n])
    }
}
/// n = 7, i = 2, a < e, a < g, b < c, b < d, b < g, e < f
fn select_511([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_294([c, d, a, g])
    } else {
        select_13([b, a, d, e, f, g])
    }
}
/// n = 8, i = 2, a < h, b < d, b < g, c < e, c < g, c < h, e < f
fn select_512([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_378([a, c, e, b, f, d, h])
    } else {
        select_511([c, b, a, d, e, f, g])
    }
}
/// n = 6, i = 3, a < f, b < f, c < e, d < e, d < f
fn select_513([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_146([c, d, a, b, e, f])
    } else {
        select_146([c, d, b, a, e, f])
    }
}
/// n = 8, i = 3, a < h, b < e, c < f, c < h, d < e, d < f, d < h, f < g
fn select_514([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_513([a, c, b, d, e, h])
    } else {
        select_113([b, a, c, d, f, g, h])
    }
}
/// n = 9, i = 3, b < i, c < f, c < h, c < i, d < e, d < f, d < i, e < h, f < g
fn select_515([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_512([b, e, c, a, f, g, h, i])
    } else {
        select_514([b, a, c, d, e, f, g, i])
    }
}
/// n = 8, i = 4, b < f, c < f, c < g, d < e, e < g, g < h
fn select_516([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_64([a, b, c, g, h, f])
    } else {
        select_71([a, d, c, f, e, g])
    }
}
/// n = 8, i = 3, b < d, b < h, c < e, e < f, e < h, f < g
fn select_517([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_64([a, b, e, f, g, h])
    } else {
        select_71([a, c, b, d, e, h])
    }
}
/// n = 9, i = 4, c < f, d < e, d < g, e < i, f < g, f < i, g < h
fn select_518([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_516([b, a, d, c, f, e, g, h])
    } else {
        select_517([b, e, c, a, f, g, h, i])
    }
}
/// n = 10, i = 4, b < j, c < f, d < e, d < g, d < j, e < i, f < g, f < i, f < j, g < h
fn select_519([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if j < e {
        select_113([a, b, d, f, g, h, j])
    } else {
        select_518([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, b < j, c < e, c < h, c < j, d < f, f < g, f < h, f < j, h < i
fn select_520([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_515([a, b, c, f, g, h, i, e, j])
    } else {
        select_519([a, b, d, c, e, f, h, i, g, j])
    }
}
/// n = 9, i = 2, a < f, b < d, b < i, c < e, c < i, e < g, f < h
fn select_521([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < c {
        select_350([b, a, d, c, f, h, e])
    } else {
        select_369([b, c, d, a, e, g, f, i])
    }
}
/// n = 8, i = 3, a < e, b < e, b < f, b < h, c < d, c < f, d < h, f < g
fn select_522([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_39([d, f, a, g, h])
    } else {
        select_419([a, b, c, d, e, h, f])
    }
}
/// n = 9, i = 3, a < e, b < g, c < f, d < e, d < f, d < g, f < h, g < i
fn select_523([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_522([a, d, b, c, e, g, i, f])
    } else {
        select_522([a, d, c, b, e, f, h, g])
    }
}
/// n = 10, i = 3, b < g, c < f, c < i, d < e, d < f, d < g, e < i, f < h, g < j
fn select_524([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_521([b, e, c, a, f, g, h, j, i])
    } else {
        select_523([a, b, c, d, e, f, g, h, j])
    }
}
/// n = 10, i = 4, b < f, b < h, c < f, c < g, d < e, e < g, e < h, g < i, h < j
fn select_525([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_516([a, b, c, d, e, f, g, i])
    } else {
        select_516([a, c, b, d, e, f, h, j])
    }
}
/// n = 11, i = 4, b < h, c < e, c < g, d < f, e < j, f < g, f < h, f < j, g < i, h < k
fn select_526([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_517([a, e, d, b, f, g, i, j])
    } else {
        select_525([a, b, c, d, f, e, g, h, i, k])
    }
}
/// n = 11, i = 4, b < i, c < e, c < h, d < f, f < g, f < h, f < i, h < j, i < k
fn select_527([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_524([a, b, c, f, g, h, i, j, e, k])
    } else {
        select_526([a, b, c, d, e, f, h, i, j, g, k])
    }
}
/// n = 13, i = 5, a < k, a < m, b < f, b < g, c < e, c < h, c < m, d < g, d < h, g < j, g < m, h < i, h < k, k < l
fn select_528([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_520([f, a, c, d, e, g, j, h, i, m])
    } else {
        select_527([e, a, b, d, f, h, i, g, k, j, l])
    }
}
/// n = 10, i = 3, a < d, a < g, b < e, b < f, c < f, c < g, c < i, e < i, f < h, g < j
fn select_529([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_355([b, c, a, d, g, f, j, h, e])
    } else {
        select_186([a, c, b, e, f, g, h, i, j, d])
    }
}
/// n = 9, i = 4, b < e, b < i, c < g, c < i, d < f, f < g, f < i, g < h
fn select_530([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_204([c, b, a, d, e, f, g])
    } else {
        select_99([b, c, f, a, e, g, h, i])
    }
}
/// n = 12, i = 5, a < e, a < i, b < f, b < h, c < h, c < i, c < k, d < g, f < k, g < h, g < i, g < k, h < j, i < l
fn select_531([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_529([a, b, c, e, f, h, i, j, k, l])
    } else {
        select_530([e, b, c, d, f, g, i, l, h])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < g, c < d, d < f, d < g, d < h
fn select_532([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_39([a, b, f, g, h])
    } else {
        select_12([b, c, e, d, h])
    }
}
/// n = 9, i = 3, a < e, a < i, b < e, b < h, c < d, d < f, d < h, d < i, f < g
fn select_533([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_39([a, f, b, g, i])
    } else {
        select_532([a, b, c, d, e, f, h, i])
    }
}
/// n = 10, i = 5, a < e, a < j, b < g, b < h, b < j, c < g, c < h, c < j, d < f, e < g, f < h, f < j, h < i
fn select_534([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_83([a, b, c, e, h, i, j])
    } else {
        select_70([b, c, d, f, g])
    }
}
/// n = 11, i = 5, a < e, a < k, b < g, b < h, b < k, c < g, c < h, c < k, d < f, f < h, f < j, f < k, g < j, h < i
fn select_535([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_533([a, g, d, f, e, h, i, j, k])
    } else {
        select_534([a, b, c, d, e, f, g, h, i, k])
    }
}
/// n = 12, i = 5, a < e, a < i, b < g, b < h, b < i, c < g, c < h, c < i, d < f, d < i, f < h, f < k, g < k, h < j, i < l
fn select_536([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_198([e, g, d, i, f, l])
    } else {
        select_535([a, b, c, d, e, f, g, h, j, k, i])
    }
}
/// n = 13, i = 6, a < e, a < k, b < i, b < j, b < k, c < g, d < f, d < k, f < h, f < j, g < i, g < j, g < k, j < l, k < m
fn select_537([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_531([a, f, b, c, e, h, g, j, k, l, i, m])
    } else {
        select_536([a, b, g, d, e, f, i, j, k, l, h, m])
    }
}
/// n = 14, i = 6, a < e, a < j, b < f, b < n, c < i, c < j, c < l, d < g, d < j, f < i, f < l, g < h, g < l, g < n, j < k, j < n, l < m
fn select_538([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < f {
        select_202([e, g, b, c, h, j, k, f, i, n])
    } else {
        select_537([a, c, b, d, e, g, f, h, i, l, j, m, k])
    }
}
/// n = 14, i = 6, a < f, a < n, b < e, b < k, c < i, c < j, c < k, d < g, d < k, f < i, g < h, g < j, g < n, j < l, k < m, k < n
fn select_539([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < f {
        select_528([a, b, g, c, h, e, k, j, l, m, f, i, n])
    } else {
        select_538([b, a, c, d, e, f, g, h, i, k, m, j, l, n])
    }
}
/// n = 16, i = 6, a < k, a < l, b < f, b < n, c < e, c < i, d < g, d < i, f < k, g < h, g < l, g < n, i < j, i < n, l < m, n < o, n < p
fn select_540([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < a {
        select_510([e, b, g, i, j, h, f, a, l, n, m, k, o, p])
    } else {
        select_539([b, c, a, d, e, f, g, h, k, l, i, m, j, n])
    }
}
/// n = 11, i = 3, b < d, b < h, c < e, c < h, e < f, e < g, f < j, g < i, h < k
fn select_541([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_445([a, f, g, b, d, h, i, j])
    } else {
        select_403([a, d, c, h, e, f, g, k])
    }
}
/// n = 12, i = 3, b < d, b < i, b < l, c < e, c < f, c < i, e < h, f < g, f < l, h < j, i < k
fn select_542([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_56([a, b, c, d, f, g, e, i, h, k, l])
    } else {
        select_541([a, b, c, d, e, f, h, i, j, g, k])
    }
}
/// n = 9, i = 2, c < d, c < e, d < h, d < i, e < f, e < g
fn select_543([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_313([a, b, c, d, e, h, i])
    } else {
        select_313([a, b, c, e, d, f, g])
    }
}
/// n = 10, i = 2, a < d, a < j, b < c, b < e, b < j, d < i, e < f, e < g, e < h
fn select_544([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_238([c, a, e, f, g, h, j])
    } else {
        select_13([b, a, c, d, i, j])
    }
}
/// n = 9, i = 3, a < i, b < d, b < h, b < i, c < e, c < i, e < f, e < h, f < g
fn select_545([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_9([b, e, a, d, f, g, h])
    } else {
        select_83([b, a, c, d, e, f, i])
    }
}
/// n = 11, i = 3, b < d, b < g, b < k, c < e, c < g, e < f, e < k, f < h, g < i, g < j
fn select_546([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_544([e, b, d, f, g, a, i, j, h, k])
    } else {
        select_545([a, b, c, d, e, f, h, k, g])
    }
}
/// n = 12, i = 3, b < d, b < g, b < h, c < e, c < g, c < h, e < f, f < i, g < l, h < j, h < k
fn select_547([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_543([a, d, c, g, h, j, k, e, l])
    } else {
        select_546([a, b, c, d, e, f, h, i, j, k, g])
    }
}
/// n = 13, i = 3, b < d, b < i, b < j, c < e, c < f, c < i, c < j, e < g, f < h, h < k, i < m, j < l
fn select_548([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < i {
        select_542([a, b, c, d, f, e, g, h, j, k, l, i])
    } else {
        select_547([a, b, c, d, f, h, j, i, k, e, m, l])
    }
}
/// n = 10, i = 3, a < d, a < e, b < f, b < g, c < e, c < f, c < g, c < j, d < j, f < i, g < h
fn select_549([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_350([a, b, e, f, g, h, i])
    } else {
        select_107([b, a, c, d, e, f, j, i])
    }
}
/// n = 9, i = 3, a < e, a < h, a < i, b < e, b < g, b < i, c < d, c < i, d < g, d < h, e < f
fn select_550([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_38([c, e, f, i])
    } else {
        select_184([a, b, d, g, h, i])
    }
}
/// n = 9, i = 3, a < e, a < i, b < f, b < h, b < i, c < d, c < f, c < i, e < h, f < g
fn select_551([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_185([d, a, b, f, g, i, e])
    } else {
        select_550([b, c, a, e, f, g, d, h, i])
    }
}
/// n = 10, i = 3, a < e, b < f, b < g, b < j, c < d, c < f, c < g, e < j, f < i, g < h
fn select_552([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_6([d, g, f, a, i, h])
    } else {
        select_551([a, b, c, d, e, g, h, j, f])
    }
}
/// n = 11, i = 3, a < e, a < f, b < d, b < g, b < h, c < f, c < g, c < h, c < k, e < k, g < j, h < i
fn select_553([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_549([a, b, c, e, f, g, h, i, j, k])
    } else {
        select_552([a, c, b, d, e, g, h, i, j, k])
    }
}
/// n = 11, i = 3, a < f, a < g, b < d, b < e, c < e, c < f, c < g, c < k, d < k, e < h, f < j, g < i
fn select_554([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_369([b, c, d, f, g, i, j, e])
    } else {
        select_128([d, a, c, e, h, f, g, k])
    }
}
/// n = 12, i = 3, a < e, a < h, b < d, b < f, b < g, c < f, c < g, c < h, c < l, e < l, f < j, g < i, h < k
fn select_555([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < h {
        select_552([a, c, b, d, e, f, g, i, j, l])
    } else {
        select_554([b, a, c, e, h, f, g, k, i, j, l])
    }
}
/// n = 12, i = 3, a < f, b < i, c < e, c < g, c < h, d < f, d < g, d < h, d < i, g < k, h < j, i < l
fn select_556([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_553([a, c, d, e, b, f, g, h, j, k, i])
    } else {
        select_555([b, c, d, e, a, g, h, i, j, k, l, f])
    }
}
/// n = 7, i = 1, a < g, b < c, b < d, b < e, b < f
fn select_557([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < a {
        select_23([c, d, e, f])
    } else {
        select_40([a, b, d, e, f, g])
    }
}
/// n = 8, i = 1, b < h, c < d, c < e, c < f, c < g
fn select_558([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_557([a, c, d, e, f, g, b])
    } else {
        select_25([b, c, d, e, f, g, a, h])
    }
}
/// n = 8, i = 2, a < d, a < g, a < h, b < c, b < e, b < g, b < h, e < f
fn select_559([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_6([c, e, a, g, h, f])
    } else {
        select_8([a, b, c, d, g, h])
    }
}
/// n = 11, i = 2, a < d, a < k, b < c, b < e, b < f, b < k, e < j, f < g, f < h, f < i
fn select_560([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_558([c, e, f, a, g, h, i, j])
    } else {
        select_559([a, b, c, d, e, j, f, k])
    }
}
/// n = 12, i = 3, a < e, b < e, b < f, b < g, b < h, c < d, c < f, c < g, c < h, f < l, g < k, h < i, h < j
fn select_561([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_506([a, b, c, d, e, f, h, i, j, g, l])
    } else {
        select_506([a, b, c, d, e, g, h, i, j, f, k])
    }
}
/// n = 14, i = 3, a < g, a < n, b < d, b < i, b < j, c < e, c < g, c < i, c < j, e < f, e < h, e < n, h < k, i < m, j < l
fn select_562([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < b {
        select_560([a, e, f, g, h, b, d, i, j, k, n])
    } else {
        select_561([a, c, b, d, g, i, j, e, f, h, l, m])
    }
}
/// n = 14, i = 3, b < g, b < n, c < e, c < i, c < j, d < f, d < g, d < i, d < j, f < h, f < n, h < k, i < m, j < l
fn select_563([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < f {
        select_556([b, a, c, d, e, g, i, j, f, l, m, h])
    } else {
        select_562([b, c, d, e, f, a, g, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 3, b < n, c < f, c < i, c < j, d < e, d < g, d < i, d < j, g < h, g < n, h < k, i < m, j < l
fn select_564([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < b {
        select_548([a, c, d, f, e, g, b, h, i, j, k, l, m])
    } else {
        select_563([a, b, c, d, f, g, e, h, i, j, k, l, m, n])
    }
}
/// n = 7, i = 2, a < d, a < e, a < f, a < g, b < c, b < e, b < f, b < g
fn select_565([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_5([c, a, e, f, g])
    } else {
        select_5([d, b, e, f, g])
    }
}
/// n = 9, i = 2, a < d, a < h, a < i, b < c, b < e, b < h, b < i, e < f, e < g
fn select_566([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_5([c, e, a, f, g])
    } else {
        select_565([a, b, c, d, e, h, i])
    }
}
/// n = 10, i = 2, a < e, a < i, a < j, b < c, b < d, b < f, b < i, b < j, f < g, f < h
fn select_567([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < e {
        select_257([c, d, a, f, g, h, i, j])
    } else {
        select_566([a, b, d, e, f, g, h, i, j])
    }
}
/// n = 10, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, f < j, g < h, g < i
fn select_568([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_238([d, f, g, a, h, i, j])
    } else {
        select_99([a, b, c, d, e, f, j, g])
    }
}
/// n = 12, i = 3, a < e, a < k, a < l, b < d, b < i, c < f, c < i, f < g, f < h, f < k, f < l, i < j
fn select_569([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_567([a, f, g, h, e, b, d, i, k, l])
    } else {
        select_568([a, c, b, d, e, i, f, g, h, j])
    }
}
/// n = 8, i = 3, b < d, b < g, c < e, c < h, d < h, e < f, e < g
fn select_570([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_230([a, c, e, b, f, d])
    } else {
        select_65([c, b, a, d, e, f, g, h])
    }
}
/// n = 9, i = 4, a < i, b < f, b < i, c < d, c < e, e < h, e < i, f < g, f < h
fn select_571([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_570([d, e, b, a, f, g, h, i])
    } else {
        select_460([a, b, c, d, f, g, e, i])
    }
}
/// n = 8, i = 3, a < d, a < g, b < c, b < g, c < e, d < f, g < h
fn select_572([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_13([c, a, e, g, h, d])
    } else {
        select_13([d, b, f, g, h, c])
    }
}
/// n = 10, i = 4, a < f, a < h, b < f, b < h, c < d, c < h, c < j, d < e, f < g, f < j, h < i
fn select_573([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_126([f, c, g, h, i, d, j])
    } else {
        select_83([d, a, b, e, h, i, f])
    }
}
/// n = 11, i = 4, a < g, a < i, b < e, b < i, c < d, c < h, c < i, d < f, e < g, e < h, h < j, i < k
fn select_574([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_136([e, c, d, a, g, f, h, j, i])
    } else {
        select_573([a, b, c, d, f, e, g, i, k, h])
    }
}
/// n = 11, i = 4, a < i, b < e, b < i, c < d, c < h, c < i, d < g, e < f, e < h, h < j, i < k
fn select_575([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_572([c, e, f, d, a, g, h, j])
    } else {
        select_574([a, b, c, d, e, g, f, h, i, j, k])
    }
}
/// n = 11, i = 4, a < e, b < i, c < h, c < i, d < f, d < i, f < g, f < h, h < j, i < k
fn select_576([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < c {
        select_571([b, d, a, e, c, f, g, h, i])
    } else {
        select_575([b, d, c, a, f, g, e, h, i, j, k])
    }
}
/// n = 13, i = 4, b < e, b < l, b < m, c < h, c < j, d < f, d < h, f < g, f < j, h < i, h < l, h < m, j < k
fn select_577([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_569([b, f, c, g, e, h, a, i, j, k, l, m])
    } else {
        select_576([b, a, c, d, e, f, g, j, h, k, i])
    }
}
/// n = 9, i = 1, c < h, c < i, d < e, d < f, d < g
fn select_578([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_31([a, c, d, e, f, g, h, i, b])
    } else {
        select_31([b, c, d, e, f, g, h, i, a])
    }
}
/// n = 12, i = 3, a < j, a < k, a < l, b < d, b < h, c < e, c < h, e < f, e < g, e < j, e < k, e < l, h < i
fn select_579([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < b {
        select_578([f, g, b, a, j, k, l, d, h])
    } else {
        select_403([a, d, c, h, e, f, g, i])
    }
}
/// n = 7, i = 4, a < f, b < f, b < g, c < f, c < g, d < e, e < g
fn select_580([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_70([b, c, d, e, f])
    } else {
        select_16([a, b, c, g])
    }
}
/// n = 10, i = 5, a < g, a < h, b < e, c < g, c < h, c < j, d < f, e < g, e < j, f < h, f < j, h < i
fn select_581([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_580([a, c, e, d, f, g, j])
    } else {
        select_3([b, h, i])
    }
}
/// n = 7, i = 3, a < e, b < e, c < d, d < f, f < g
fn select_582([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_4([b, f, a, g])
    } else {
        select_146([b, a, c, d, e, f])
    }
}
/// n = 10, i = 5, a < e, b < g, b < h, c < g, c < h, c < j, d < f, e < j, f < h, f < j, h < i
fn select_583([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_581([b, a, c, d, e, f, g, h, i, j])
    } else {
        select_582([a, g, d, f, e, h, i])
    }
}
/// n = 14, i = 5, a < g, a < k, b < e, b < l, b < m, c < g, c < h, c < k, d < f, f < h, f < k, h < i, h < j, h < l, h < m, k < n
fn select_584([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < e {
        select_579([b, a, f, g, h, i, j, k, n, e, l, m])
    } else {
        select_583([b, a, c, d, e, f, g, k, n, h])
    }
}
/// n = 8, i = 3, a < d, b < e, b < f, c < e, c < f, e < h, f < g
fn select_585([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_39([f, e, a, h, g])
    } else {
        select_83([a, b, c, d, f, g, e])
    }
}
/// n = 7, i = 4, b < f, b < g, c < f, c < g, d < e, e < g
fn select_586([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_580([a, b, c, d, e, f, g])
    } else {
        select_12([f, d, a, e, g])
    }
}
/// n = 10, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < j, g < j, h < i
fn select_587([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_585([a, b, c, e, g, h, i, j])
    } else {
        select_586([e, b, c, d, f, g, h])
    }
}
/// n = 11, i = 5, a < e, b < g, b < i, b < k, c < g, c < h, c < i, d < f, f < h, f < i, h < k, i < j
fn select_588([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_587([a, b, c, d, e, f, g, i, j, h])
    } else {
        select_412([a, b, c, f, e, h, i, j, g, k])
    }
}
/// n = 13, i = 6, a < e, b < i, b < j, b < k, c < g, d < f, d < j, f < i, f < k, g < h, g < k, h < j, j < l, k < m
fn select_589([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < h {
        select_588([a, b, f, c, e, g, i, h, k, m, j])
    } else {
        select_412([a, b, d, h, e, f, j, l, i, k])
    }
}
/// n = 15, i = 6, a < e, a < n, a < o, b < i, b < j, b < l, c < g, d < f, d < j, f < i, f < l, g < h, g < j, g < l, j < k, j < n, j < o, l < m
fn select_590([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < h {
        select_584([f, a, b, c, e, g, i, j, h, k, l, n, o, m])
    } else {
        select_589([a, b, c, d, e, f, g, h, i, j, l, k, m])
    }
}
/// n = 15, i = 6, a < e, a < n, a < o, b < f, b < l, c < i, c < j, c < l, d < g, f < i, g < h, g < j, g < l, j < k, l < m, l < n, l < o
fn select_591([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < f {
        select_577([h, a, b, c, e, j, k, l, m, f, i, n, o])
    } else {
        select_590([a, c, d, b, e, f, g, h, i, l, m, j, k, n, o])
    }
}
/// n = 17, i = 6, a < i, a < j, b < e, b < n, b < o, c < f, c < k, d < g, f < i, g < h, g < j, g < k, j < m, k < l, k < n, k < o, n < q, o < p
fn select_592([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if k < a {
        select_564([h, f, b, k, l, e, a, j, n, o, m, p, q, i])
    } else {
        select_591([b, c, a, d, e, f, g, h, i, j, m, k, l, n, o])
    }
}
/// n = 17, i = 6, a < k, a < l, b < e, b < i, b < p, c < f, c < m, d < g, d < i, f < k, g < h, g < l, g < m, i < j, l < o, m < n, m < p, p < q
fn select_593([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if i < m {
        select_540([a, c, b, d, e, f, g, h, i, j, k, l, o, m, n, p])
    } else {
        select_592([a, b, c, d, e, f, g, h, k, l, m, n, o, i, p, q, j])
    }
}
/// n = 17, i = 6, a < k, a < l, b < e, b < p, c < g, c < m, d < f, d < h, f < j, g < k, h < i, h < l, h < m, l < o, m < n, m < p, p < q
fn select_594([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if f < b {
        select_476([a, f, c, h, i, j, g, b, e, l, m, n, o, k, p])
    } else {
        select_593([a, b, c, d, e, g, h, i, f, j, k, l, m, n, o, p, q])
    }
}
/// n = 17, i = 6, a < g, a < p, b < f, b < l, c < e, c < m, d < h, d < i, h < k, i < j, i < l, i < m, l < o, m < n, m < p, p < q
fn select_595([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if f < e {
        select_344([a, b, c, d, g, h, f, i, j, k, e, l, m, n, o, p, q])
    } else {
        select_594([b, a, c, d, g, h, e, i, j, k, f, l, m, n, o, p, q])
    }
}
/// n = 8, i = 4, a < f, b < f, b < h, c < d, d < e, e < h, f < g
fn select_596([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_16([a, b, e, h])
    } else {
        select_69([c, f, g, d])
    }
}
/// n = 8, i = 4, a < h, b < d, c < e, d < f, e < g, g < h
fn select_597([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < d {
        select_596([b, a, c, e, g, d, f, h])
    } else {
        select_221([c, d, a, f, e, g])
    }
}
/// n = 8, i = 4, a < h, b < g, c < g, c < h, d < e, e < f, e < h
fn select_598([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_462([c, d, a, b, e, f, g, h])
    } else {
        select_311([c, d, b, a, e, f, g])
    }
}
/// n = 10, i = 4, a < j, b < g, c < g, c < h, c < j, d < e, e < f, e < h, e < j, h < i
fn select_599([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_100([b, a, c, e, f, h, i, j])
    } else {
        select_598([a, b, c, d, e, f, g, j])
    }
}
/// n = 11, i = 4, a < g, b < h, c < g, c < h, c < i, d < e, d < k, e < f, e < i, h < k, i < j
fn select_600([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_181([a, d, b, g, h, e, k])
    } else {
        select_599([b, a, c, d, e, f, g, i, j, h])
    }
}
/// n = 11, i = 4, a < e, b < h, c < h, c < i, d < f, d < k, f < g, f < i, h < k, i < j
fn select_601([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < c {
        select_597([b, d, a, f, e, g, c, h])
    } else {
        select_600([a, b, c, d, f, g, e, h, i, j, k])
    }
}
/// n = 7, i = 1, c < g, d < e, d < f
fn select_602([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_281([a, b, c, d, g])
    } else {
        select_153([a, b, d, c, e, f])
    }
}
/// n = 9, i = 2, a < i, b < h, b < i, c < d, c < e, c < h, e < f, e < g
fn select_603([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_153([a, d, e, b, f, g])
    } else {
        select_127([a, b, c, d, e, h, i])
    }
}
/// n = 11, i = 3, a < d, a < e, b < c, b < f, b < k, d < j, e < j, e < k, f < g, f < h, h < i
fn select_604([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_602([c, g, h, a, d, e, i])
    } else {
        select_603([d, e, b, c, f, g, h, k, j])
    }
}
/// n = 9, i = 3, a < i, b < d, b < f, c < e, c < f, e < i, f < g, f < h
fn select_605([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_153([d, e, f, a, g, h])
    } else {
        select_166([a, b, c, d, e, i, f])
    }
}
/// n = 11, i = 4, a < e, a < f, b < d, b < g, c < f, c < g, e < k, f < k, g < h, g < i, i < j
fn select_606([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_602([d, h, i, a, e, f, j])
    } else {
        select_605([e, b, c, d, f, g, h, i, k])
    }
}
/// n = 12, i = 4, a < d, a < i, b < e, b < h, c < f, c < h, e < j, e < k, f < g, f < i, f < k, h < j, i < l
fn select_607([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_604([b, f, g, h, e, a, d, i, l, j, k])
    } else {
        select_606([b, a, c, d, e, h, f, g, i, l, j])
    }
}
/// n = 10, i = 3, b < d, b < i, c < e, c < j, d < j, e < f, e < g, e < i, g < h
fn select_608([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_285([a, b, e, f, g, h, i])
    } else {
        select_570([a, b, c, d, e, f, i, j])
    }
}
/// n = 10, i = 3, a < c, a < h, b < d, b < h, c < f, d < e, d < g, g < i, h < j
fn select_609([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < c {
        select_14([a, d, e, g, h, i, j])
    } else {
        select_572([a, b, d, c, e, f, h, j])
    }
}
/// n = 11, i = 3, a < h, b < d, b < j, c < e, c < h, e < f, e < g, e < j, g < i, h < k
fn select_610([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < a {
        select_66([b, c, d, a, e, f, g, i, j, h])
    } else {
        select_609([a, c, b, e, f, d, g, h, i, k])
    }
}
/// n = 13, i = 4, a < d, a < j, b < e, b < h, c < f, c < i, e < i, f < g, f < h, f < j, h < k, i < l, j < m
fn select_611([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_608([d, b, c, e, f, g, j, m, h, i])
    } else {
        select_610([e, a, c, d, f, g, h, i, k, j, l])
    }
}
/// n = 13, i = 4, a < h, b < j, b < m, c < e, c < i, d < f, d < h, f < g, f < i, f < j, h < m, i < k, j < l
fn select_612([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_607([c, a, d, e, b, f, g, h, i, m, j, k])
    } else {
        select_611([c, b, d, e, a, f, g, j, h, i, l, m, k])
    }
}
/// n = 13, i = 4, a < j, b < f, b < m, c < e, c < k, d < g, d < j, f < i, g < h, g < k, j < m, k < l
fn select_613([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_601([c, a, d, b, e, f, i, j, g, h, m])
    } else {
        select_612([a, b, c, d, e, g, h, j, k, f, l, i, m])
    }
}
/// n = 7, i = 2, a < d, a < e, b < c, b < f, f < g
fn select_614([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_4([c, f, a, g])
    } else {
        select_82([b, a, d, e, c, f])
    }
}
/// n = 8, i = 2, a < d, a < e, a < h, b < c, b < f, b < h, f < g
fn select_615([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_82([b, a, d, e, c, h])
    } else {
        select_9([a, b, c, e, f, g, h])
    }
}
/// n = 8, i = 2, a < g, b < f, b < h, c < d, c < e, c < h
fn select_616([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_614([c, a, g, d, e, b, f])
    } else {
        select_615([c, b, f, d, e, a, g, h])
    }
}
/// n = 8, i = 2, b < d, b < g, b < h, c < e, c < g, c < h, e < f
fn select_617([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_110([a, c, b, d, g, h])
    } else {
        select_170([a, b, c, e, f, g, h])
    }
}
/// n = 10, i = 2, b < e, b < j, c < d, c < f, c < j, e < i, f < g, f < h
fn select_618([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_153([a, d, f, b, g, h])
    } else {
        select_617([a, c, b, d, e, i, f, j])
    }
}
/// n = 11, i = 3, a < f, a < h, b < e, b < i, c < d, c < h, c < i, f < g, i < j, i < k
fn select_619([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_616([f, b, c, d, h, e, g, i])
    } else {
        select_618([e, a, c, d, f, i, j, k, g, h])
    }
}
/// n = 9, i = 3, a < d, a < i, b < e, b < i, c < e, c < i, e < f, e < g, e < h
fn select_620([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_16([d, b, c, i])
    } else {
        select_40([a, e, f, g, h, i])
    }
}
/// n = 10, i = 3, a < d, a < j, b < f, b < i, b < j, c < e, c < f, c < j, e < i, f < g, f < h
fn select_621([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_211([a, b, c, d, e, f, i, j])
    } else {
        select_620([a, b, c, d, f, e, g, h, j])
    }
}
/// n = 10, i = 3, a < f, a < j, b < e, b < g, b < j, c < d, c < g, c < j, g < h, g < i
fn select_622([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_621([a, b, c, f, d, g, h, i, e, j])
    } else {
        select_621([a, c, b, f, e, g, h, i, d, j])
    }
}
/// n = 12, i = 4, a < g, a < h, b < f, b < h, b < j, c < d, c < f, c < g, d < e, d < j, h < i, j < k, j < l
fn select_623([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < d {
        select_82([h, c, f, g, i, d])
    } else {
        select_622([a, b, d, e, f, g, j, k, l, h])
    }
}
/// n = 12, i = 4, a < i, a < j, b < e, b < h, b < i, c < d, c < h, c < j, d < f, d < l, e < g, e < l, j < k
fn select_624([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_619([a, d, b, h, f, j, k, i, e, g, l])
    } else {
        select_623([a, c, b, e, g, h, i, j, k, d, f, l])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, c < d, c < f, c < g
fn select_625([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_82([c, b, a, e, d, f])
    } else {
        select_13([c, a, d, b, e, g])
    }
}
/// n = 8, i = 2, a < d, a < h, b < c, b < e, b < h, e < f, e < g
fn select_626([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_5([c, e, a, f, g])
    } else {
        select_8([a, b, c, d, e, h])
    }
}
/// n = 8, i = 2, b < e, b < h, c < d, c < f, c < h, f < g
fn select_627([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_625([a, b, c, d, e, h, f])
    } else {
        select_626([b, c, d, e, f, a, g, h])
    }
}
/// n = 9, i = 2, b < f, b < i, c < d, c < e, c < i, e < g, f < h
fn select_628([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_120([a, c, b, d, f, h, i])
    } else {
        select_285([a, b, c, d, e, g, i])
    }
}
/// n = 11, i = 3, a < d, a < e, a < i, b < c, b < f, b < i, e < g, f < h, h < j, i < k
fn select_629([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_627([d, e, b, c, g, i, k, f])
    } else {
        select_628([c, f, a, d, i, h, k, j, e])
    }
}
/// n = 7, i = 2, b < e, b < f, b < g, c < d, c < f, c < g
fn select_630([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_110([a, b, c, d, f, g])
    } else {
        select_110([a, c, b, e, f, g])
    }
}
/// n = 9, i = 3, a < e, a < i, b < d, b < h, b < i, c < f, c < i, f < g, f < h
fn select_631([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_9([b, f, g, d, a, e, h])
    } else {
        select_99([b, c, a, e, d, f, g, i])
    }
}
/// n = 10, i = 3, a < d, a < i, a < j, b < c, b < g, b < h, c < e, c < i, c < j, d < f, d < g, d < h
fn select_632([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_5([e, a, d, i, j])
    } else {
        select_5([f, b, c, g, h])
    }
}
/// n = 10, i = 3, a < d, a < h, a < i, b < c, b < e, b < j, d < g, d < j, e < f, e < h, e < i
fn select_633([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_127([c, a, e, f, h, i, d])
    } else {
        select_632([a, b, e, d, f, g, c, j, h, i])
    }
}
/// n = 12, i = 4, a < d, a < h, a < l, b < g, b < h, c < e, c < g, c < k, e < f, g < i, g < l, h < j, h < k
fn select_634([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < h {
        select_631([e, a, b, d, f, g, i, l, h])
    } else {
        select_633([c, a, d, g, h, j, i, e, k, l])
    }
}
/// n = 13, i = 5, a < e, a < g, a < m, b < g, b < i, c < f, c < i, c < l, d < f, d < i, d < k, f < h, g < k, g < l, i < j, i < m
fn select_635([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < d {
        select_630([e, c, d, k, l, f, i])
    } else {
        select_634([a, b, c, e, f, h, i, g, j, k, l, m])
    }
}
/// n = 14, i = 5, a < f, a < l, b < e, b < g, b < m, c < g, c < h, d < f, d < h, d < k, f < j, g < k, g < l, h < i, h < m, m < n
fn select_636([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < a {
        select_629([b, h, i, e, g, a, k, f, m, j, n])
    } else {
        select_635([b, c, a, d, e, f, g, j, h, i, k, l, m])
    }
}
/// n = 14, i = 5, a < g, a < l, b < e, b < m, c < g, c < h, c < j, d < f, d < h, f < j, f < l, g < k, h < i, h < m, m < n
fn select_637([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < b {
        select_624([a, f, c, h, b, i, e, j, l, g, k, m])
    } else {
        select_636([a, b, d, c, e, g, f, h, i, k, j, l, m, n])
    }
}
/// n = 14, i = 5, a < h, a < l, b < f, b < m, c < g, c < i, d < e, d < h, d < i, g < l, h < k, i < j, i < m, m < n
fn select_638([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < g {
        select_613([e, a, b, c, f, h, i, j, k, g, m, n, l])
    } else {
        select_637([a, b, d, c, f, g, h, i, j, e, k, l, m, n])
    }
}
/// n = 9, i = 3, a < e, a < f, b < d, b < g, c < f, c < g, c < i, e < i, g < h
fn select_639([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_625([e, b, c, f, d, g, i])
    } else {
        select_120([d, a, c, e, g, h, f])
    }
}
/// n = 8, i = 4, a < g, a < h, b < f, c < e, d < f, d < h, e < h, f < g
fn select_640([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_111([a, b, d, e, f, h])
    } else {
        select_63([a, c, f, e, g])
    }
}
/// n = 10, i = 4, a < f, a < j, b < g, b < i, b < j, c < f, c < g, d < e, d < g, e < j, f < i, g < h
fn select_641([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_246([a, e, c, b, g, h, j])
    } else {
        select_640([b, c, d, a, e, f, i, j])
    }
}
/// n = 11, i = 4, a < g, a < k, b < e, b < g, b < h, c < e, c < f, d < f, d < g, d < i, e < i, f < h, f < k, g < j
fn select_642([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_39([a, b, e, h, k])
    } else {
        select_299([a, c, d, e, f, g, h, i, j])
    }
}
/// n = 11, i = 4, a < e, a < h, b < f, b < g, c < g, c < h, c < j, d < f, d < h, d < i, f < j, g < i, h < k
fn select_643([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_641([b, d, c, a, e, g, h, k, i, f])
    } else {
        select_642([a, c, b, d, g, f, h, j, i, k, e])
    }
}
/// n = 11, i = 4, a < g, b < e, b < i, c < g, c < h, c < i, d < f, d < i, d < k, f < h, g < k, i < j
fn select_644([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_639([f, b, c, e, a, h, i, j, g])
    } else {
        select_643([b, a, c, d, e, f, g, i, k, h, j])
    }
}
/// n = 9, i = 3, a < d, a < i, b < f, b < h, b < i, c < e, c < f, c < i, e < h, f < g
fn select_645([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < a {
        select_38([e, f, g, h])
    } else {
        select_99([a, b, c, e, d, f, g, i])
    }
}
/// n = 10, i = 4, a < d, a < j, b < f, b < g, c < e, c < g, c < j, e < i, f < h, f < i, f < j, g < h
fn select_646([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_98([a, e, b, g, d, f, i])
    } else {
        select_645([a, f, c, d, g, e, i, h, j])
    }
}
/// n = 10, i = 3, a < d, a < g, b < e, b < f, c < e, c < f, c < g, e < i, f < h, g < j
fn select_647([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_87([b, a, c, d, g, e, j, f, i])
    } else {
        select_87([b, a, c, d, g, f, j, e, h])
    }
}
/// n = 11, i = 4, a < d, a < i, b < f, b < g, b < i, c < e, c < h, c < i, e < g, f < h, h < j, i < k
fn select_648([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_19([e, a, b, d, g, i, f, k, h])
    } else {
        select_647([a, f, c, d, e, h, i, j, g, k])
    }
}
/// n = 11, i = 4, a < g, b < e, b < i, c < h, c < i, c < k, d < f, d < g, d < i, f < h, g < k, i < j
fn select_649([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < c {
        select_646([b, a, d, e, f, c, g, k, h, i])
    } else {
        select_648([b, c, d, e, f, a, h, g, i, k, j])
    }
}
/// n = 11, i = 4, a < h, b < g, b < i, c < f, c < i, c < k, d < e, d < h, d < i, h < k, i < j
fn select_650([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_644([a, b, d, c, g, f, h, e, i, j, k])
    } else {
        select_649([a, b, c, d, g, e, h, f, i, j, k])
    }
}
/// n = 8, i = 2, b < h, c < d, c < h, d < e, d < f, d < g
fn select_651([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_214([a, c, b, d, e, f, g])
    } else {
        select_32([b, c, a, d, e, f, g, h])
    }
}
/// n = 9, i = 2, b < d, b < i, c < e, c < i, e < f, e < g, e < h
fn select_652([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_263([a, e, b, f, g, h])
    } else {
        select_110([a, c, b, d, e, i])
    }
}
/// n = 10, i = 3, b < d, b < g, c < e, c < g, e < f, g < h, g < i, g < j
fn select_653([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_651([a, d, c, g, h, i, j, e])
    } else {
        select_652([a, e, b, f, g, h, i, j, d])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, b < g, c < d, c < f, c < g
fn select_654([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_329([d, a, b, f, g, e])
    } else {
        select_329([e, a, c, f, g, d])
    }
}
/// n = 9, i = 3, a < i, b < e, b < f, b < i, c < d, c < f, c < i, f < g, f < h
fn select_655([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_153([d, e, f, a, g, h])
    } else {
        select_654([a, b, c, d, e, f, i])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < j, f < k, g < h, g < i
fn select_656([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_578([d, e, g, f, a, j, k, h, i])
    } else {
        select_655([a, b, c, d, e, g, h, i, f])
    }
}
/// n = 13, i = 4, b < e, b < g, c < d, c < g, c < j, d < h, d < m, e < f, e < j, e < m, h < i, j < k, j < l
fn select_657([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < e {
        select_653([a, b, d, g, h, i, e, f, j, m])
    } else {
        select_656([a, c, e, f, g, d, j, k, l, h, m])
    }
}
/// n = 7, i = 1, c < d, c < e, c < f, c < g
fn select_658([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_153([a, b, c, d, f, g])
    } else {
        select_153([a, b, c, e, f, g])
    }
}
/// n = 7, i = 3, b < f, b < g, c < e, c < g, d < e, d < f, d < g
fn select_659([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_110([a, c, d, f, e, g])
    } else {
        select_110([a, b, d, e, f, g])
    }
}
/// n = 10, i = 3, b < f, c < e, c < g, d < e, d < f, d < g, g < h, g < i, g < j
fn select_660([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_658([a, e, g, b, h, i, j])
    } else {
        select_659([a, b, c, d, e, f, g])
    }
}
/// n = 10, i = 3, a < e, b < e, b < f, b < g, c < d, c < f, c < g, f < j, g < h, g < i
fn select_661([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_172([d, f, g, h, i, a, j])
    } else {
        select_505([a, b, c, d, e, g, h, i, f])
    }
}
/// n = 11, i = 4, a < f, a < g, a < h, b < e, b < g, b < h, c < d, c < g, c < h, g < k, h < i, h < j
fn select_662([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_661([d, b, a, f, e, g, h, i, j, k])
    } else {
        select_661([e, c, a, f, d, g, h, i, j, k])
    }
}
/// n = 13, i = 5, a < f, a < j, b < e, b < h, b < k, c < d, c < h, c < k, e < i, f < g, f < k, f < m, h < i, h < j, h < m, k < l
fn select_663([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < f {
        select_660([d, e, a, h, j, i, f, g, k, m])
    } else {
        select_662([b, c, f, g, d, e, k, h, j, m, l])
    }
}
/// n = 15, i = 5, a < e, a < k, b < g, b < i, c < d, c < f, c < k, e < j, f < i, f < j, f < l, g < h, g < k, g < l, k < o, l < m, l < n
fn select_664([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < a {
        select_657([d, b, f, a, g, h, i, e, j, l, m, n, k])
    } else {
        select_663([b, a, c, d, e, g, h, f, j, i, k, o, l])
    }
}
/// n = 15, i = 5, a < f, a < k, b < e, b < h, c < d, c < g, c < k, f < j, g < j, g < l, h < i, h < k, h < l, k < o, l < m, l < n
fn select_665([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < g {
        select_650([e, a, h, c, d, i, f, g, k, o, l])
    } else {
        select_664([a, b, c, d, f, g, h, i, e, j, k, l, m, n, o])
    }
}
/// n = 7, i = 2, a < f, a < g, b < d, b < e, c < e, c < f, c < g
fn select_666([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_294([a, d, c, e])
    } else {
        select_39([b, c, f, g, d])
    }
}
/// n = 9, i = 2, a < i, b < d, b < h, c < e, c < h, c < i, e < f, e < g
fn select_667([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_666([a, b, c, d, h, e, i])
    } else {
        select_32([b, c, d, e, a, f, g, h])
    }
}
/// n = 7, i = 3, a < d, b < e, c < e, e < f, e < g
fn select_668([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_22([a, f, g])
    } else {
        select_16([d, b, c, e])
    }
}
/// n = 8, i = 3, a < d, a < e, b < f, c < e, c < f, f < g, f < h
fn select_669([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_180([b, d, c, e])
    } else {
        select_668([a, b, c, d, f, g, h])
    }
}
/// n = 9, i = 3, a < d, a < i, b < e, b < i, c < e, c < h, c < i, d < h, e < f, e < g
fn select_670([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_39([a, e, f, g, i])
    } else {
        select_63([b, d, c, h, i])
    }
}
/// n = 9, i = 3, a < i, b < e, c < f, c < i, d < e, d < f, d < i, f < g, f < h
fn select_671([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_669([b, c, d, a, e, f, g, h])
    } else {
        select_670([a, c, d, b, f, g, h, e, i])
    }
}
/// n = 10, i = 3, b < j, c < f, c < i, c < j, d < e, d < f, d < j, e < i, f < g, f < h
fn select_672([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_667([b, e, c, a, f, g, h, i, j])
    } else {
        select_671([b, a, c, d, e, f, g, h, j])
    }
}
/// n = 7, i = 4, a < d, b < f, c < e, e < g, f < g
fn select_673([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_146([a, e, b, f, d, g])
    } else {
        select_70([c, d, b, f, e])
    }
}
/// n = 10, i = 4, a < d, b < f, c < e, c < g, e < j, f < g, f < j, g < h, g < i
fn select_674([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_478([a, e, f, g, h, i, j])
    } else {
        select_673([a, b, c, d, e, f, j])
    }
}
/// n = 8, i = 2, a < h, b < g, c < d, c < g, c < h, d < e, d < f
fn select_675([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_40([b, d, a, e, f, g])
    } else {
        select_329([b, a, c, d, h, g])
    }
}
/// n = 8, i = 4, a < d, a < h, b < f, c < e, c < h, e < g, f < g, f < h
fn select_676([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_419([a, f, c, e, d, g, h])
    } else {
        select_70([b, d, c, e, f])
    }
}
/// n = 11, i = 4, a < d, a < k, b < f, c < e, c < g, c < k, e < j, f < g, f < j, f < k, g < h, g < i
fn select_677([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_675([a, e, f, g, h, i, j, k])
    } else {
        select_676([a, b, c, d, e, f, j, k])
    }
}
/// n = 11, i = 4, b < k, c < f, d < e, d < g, d < k, e < j, f < g, f < j, f < k, g < h, g < i
fn select_678([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_674([a, c, d, b, e, f, g, h, i, j])
    } else {
        select_677([b, c, d, a, e, f, g, h, i, j, k])
    }
}
/// n = 11, i = 4, b < k, c < e, c < h, c < k, d < f, f < g, f < h, f < k, h < i, h < j
fn select_679([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_672([a, b, c, f, g, h, i, j, e, k])
    } else {
        select_678([a, b, d, c, e, f, h, i, j, g, k])
    }
}
/// n = 7, i = 3, a < d, b < e, b < f, c < e, c < f, f < g
fn select_680([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_70([b, c, a, d, e])
    } else {
        select_294([a, e, f, g])
    }
}
/// n = 9, i = 3, a < f, b < d, b < g, b < h, c < e, c < f, e < g, e < h, f < i
fn select_681([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_126([b, e, d, a, f, g, h])
    } else {
        select_680([b, a, c, d, e, f, i])
    }
}
/// n = 9, i = 3, b < h, b < i, c < f, d < e, d < f, e < h, e < i, f < g
fn select_682([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_98([a, c, d, e, b, f, g])
    } else {
        select_681([c, b, d, a, e, f, h, i, g])
    }
}
/// n = 10, i = 3, b < e, c < f, d < e, d < f, e < j, f < g, f < h, f < i
fn select_683([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_263([a, f, b, g, h, i])
    } else {
        select_113([a, c, b, d, e, j, f])
    }
}
/// n = 11, i = 3, b < f, c < g, c < j, d < e, d < f, d < g, e < j, f < k, g < h, g < i
fn select_684([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_682([a, c, b, d, e, f, k, g, j])
    } else {
        select_683([a, b, c, d, f, g, e, h, i, k])
    }
}
/// n = 10, i = 4, b < g, c < e, d < f, e < h, e < i, f < g, f < h, f < i, g < j
fn select_685([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_199([a, d, e, b, f, h, i])
    } else {
        select_516([a, c, b, d, f, e, g, j])
    }
}
/// n = 12, i = 4, b < g, c < e, c < h, d < f, e < k, f < g, f < h, f < k, g < l, h < i, h < j
fn select_686([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_683([a, b, c, f, g, h, e, i, j, l])
    } else {
        select_685([a, b, c, d, e, f, g, h, k, l])
    }
}
/// n = 12, i = 4, b < h, c < e, c < i, d < f, f < g, f < h, f < i, h < l, i < j, i < k
fn select_687([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_684([a, b, c, f, g, h, i, j, k, e, l])
    } else {
        select_686([a, b, c, d, e, f, h, i, j, k, g, l])
    }
}
/// n = 14, i = 5, a < k, a < n, b < f, b < g, c < e, c < h, c < n, d < g, d < h, g < j, g < l, g < n, h < i, h < k, h < l, k < m
fn select_688([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < h {
        select_679([f, a, c, d, e, g, j, h, i, l, n])
    } else {
        select_687([e, a, b, d, f, h, i, k, g, j, l, m])
    }
}
/// n = 8, i = 4, a < e, a < h, b < d, b < h, c < f, f < g, f < h
fn select_689([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_462([a, c, b, d, f, g, e, h])
    } else {
        select_462([b, c, a, e, f, g, d, h])
    }
}
/// n = 7, i = 4, a < e, b < d, c < f, f < g
fn select_690([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_147([a, b, c, d, f, g, e])
    } else {
        select_147([b, a, c, e, f, g, d])
    }
}
/// n = 8, i = 4, b < h, c < e, c < h, d < f, f < g, f < h
fn select_691([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_689([b, c, d, e, a, f, g, h])
    } else {
        select_690([a, c, d, e, b, f, g])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, b < f, c < d, c < g, c < i, f < i, g < h
fn select_692([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < b {
        select_281([d, h, b, e, f])
    } else {
        select_100([e, f, a, c, d, g, h, i])
    }
}
/// n = 11, i = 5, a < i, a < k, b < d, b < e, c < f, e < h, e < k, f < g, f < i, f < k, i < j
fn select_693([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_691([d, a, e, c, h, f, g, k])
    } else {
        select_692([a, b, f, g, d, e, i, j, k])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < f
fn select_694([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < c {
        select_281([a, e, c, d, f])
    } else {
        select_281([a, d, b, e, f])
    }
}
/// n = 8, i = 2, a < g, a < h, b < f, b < h, c < d, c < f, c < g, d < e
fn select_695([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_378([a, c, d, b, e, h, g])
    } else {
        select_378([b, c, d, a, e, h, f])
    }
}
/// n = 9, i = 4, a < f, a < i, b < e, b < i, c < f, c < h, c < i, d < f, d < h, d < i, e < h, f < g
fn select_696([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_38([b, f, g, i])
    } else {
        select_111([a, e, c, d, h, i])
    }
}
/// n = 10, i = 4, a < e, a < j, b < g, b < i, b < j, c < f, c < g, c < j, d < f, d < g, d < j, f < i, g < h
fn select_697([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_695([a, f, b, g, h, i, j, e])
    } else {
        select_696([b, a, c, d, e, g, h, f, j])
    }
}
/// n = 12, i = 5, a < f, a < h, b < e, b < f, c < g, c < h, c < j, d < g, d < h, d < j, e < j, e < l, g < l, h < i, j < k
fn select_698([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_694([g, b, h, i, f, e])
    } else {
        select_697([a, e, c, d, f, g, j, k, l, h])
    }
}
/// n = 8, i = 3, a < d, b < c, b < h, c < f, d < e, d < h, f < g
fn select_699([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_38([a, f, g, d])
    } else {
        select_4([e, b, c, h])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < g, c < d, d < g, d < h, e < f
fn select_700([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_3([c, e, f])
    } else {
        select_63([a, b, d, g, h])
    }
}
/// n = 10, i = 4, a < f, a < h, b < e, b < f, c < d, c < e, d < h, d < j, e < j, f < g, h < i
fn select_701([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_699([b, c, d, f, g, h, i, e])
    } else {
        select_700([a, d, b, e, h, i, j, f])
    }
}
/// n = 6, i = 3, a < f, b < d, c < e, e < f
fn select_702([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < d {
        select_63([a, b, e, d, f])
    } else {
        select_16([a, c, d, e])
    }
}
/// n = 12, i = 6, a < i, a < k, b < g, b < i, c < f, d < e, d < g, e < h, e < k, f < i, f < k, g < h, i < j, k < l
fn select_703([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_701([a, b, d, e, g, i, j, k, l, h])
    } else {
        select_702([a, e, c, h, f, i])
    }
}
/// n = 7, i = 3, a < c, b < d, c < e, d < f, f < g
fn select_704([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_3([e, b, d])
    } else {
        select_38([a, f, g, c])
    }
}
/// n = 9, i = 4, a < f, b < f, b < i, c < g, c < i, d < e, e < g, e < i, g < h
fn select_705([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_185([a, b, c, g, h, i, f])
    } else {
        select_181([a, c, d, f, e, g, i])
    }
}
/// n = 13, i = 6, a < h, a < i, b < i, b < k, b < m, c < e, d < f, e < i, e < k, e < m, f < g, f < k, g < h, g < m, i < j, k < l
fn select_706([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_704([a, d, i, f, j, k, l])
    } else {
        select_705([a, g, b, c, e, h, k, l, m])
    }
}
/// n = 13, i = 6, a < g, a < i, b < i, b < k, b < m, c < f, d < e, d < g, e < h, e < k, f < i, f < k, f < m, h < m, i < j, k < l
fn select_707([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_703([b, a, c, d, e, f, g, h, i, j, k, l])
    } else {
        select_706([a, b, c, d, f, e, h, g, i, j, k, l, m])
    }
}
/// n = 13, i = 6, a < h, a < j, b < i, b < j, b < l, c < e, c < h, d < f, e < g, e < l, f < i, f < j, f < l, j < k, l < m
fn select_708([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_698([a, c, b, f, e, h, i, j, k, l, m, g])
    } else {
        select_707([a, b, d, c, e, f, h, g, j, k, l, m, i])
    }
}
/// n = 14, i = 6, a < h, a < i, b < e, b < n, c < i, c < j, c < l, d < f, d < h, e < j, e < l, f < g, f < l, f < n, i < k, i < n, l < m
fn select_709([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < e {
        select_693([b, d, c, h, f, i, k, g, e, j, n])
    } else {
        select_708([a, c, d, b, f, e, g, h, j, i, k, l, m])
    }
}
/// n = 15, i = 6, a < e, a < o, b < h, b < i, c < i, c < k, c < l, d < f, d < h, e < k, f < g, f < l, f < o, i < j, i < n, i < o, l < m, l < n
fn select_710([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if l < e {
        select_688([a, b, f, c, g, h, i, l, m, j, e, n, k, o])
    } else {
        select_709([b, a, c, d, e, f, g, h, i, k, j, l, m, o])
    }
}
/// n = 16, i = 6, a < k, a < l, b < e, b < n, c < f, c < j, d < g, d < j, e < k, f < i, f < n, f < p, g < h, g < l, g < n, l < m, l < p, n < o
fn select_711([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < a {
        select_665([b, d, f, i, j, e, a, g, h, k, n, l, m, p, o])
    } else {
        select_710([b, c, a, d, e, g, h, j, f, i, k, l, m, p, n])
    }
}
/// n = 16, i = 6, a < k, a < m, b < f, b < j, c < e, c < l, d < g, d < j, e < k, f < i, f < p, g < h, g < l, g < m, l < n, m < o, m < p
fn select_712([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < f {
        select_638([a, b, c, g, h, j, e, m, l, n, o, k, f, i])
    } else {
        select_711([a, c, b, d, e, f, g, h, i, j, k, m, o, l, n, p])
    }
}
/// n = 5, i = 2, b < e, c < d
fn select_713([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < c {
        select_294([a, e, c, d])
    } else {
        select_294([a, d, b, e])
    }
}
/// n = 8, i = 3, a < h, b < d, b < f, c < e, c < g, d < g, e < f, e < h
fn select_714([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_378([b, e, a, d, h, g, f])
    } else {
        select_76([a, b, c, d, e, h, g])
    }
}
/// n = 9, i = 4, a < h, b < f, c < f, c < g, d < e, d < i, e < g, e < h, f < i
fn select_715([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_482([b, a, c, g, h, f])
    } else {
        select_714([a, c, d, f, e, g, i, h])
    }
}
/// n = 9, i = 4, a < h, b < g, c < g, d < e, d < i, e < f, e < h, g < i
fn select_716([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_713([c, a, f, b, h])
    } else {
        select_715([a, c, b, d, e, g, f, h, i])
    }
}
/// n = 9, i = 3, b < e, c < f, d < e, d < f, e < i, f < g, f < h
fn select_717([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_250([b, c, e, f, a, g, h, i])
    } else {
        select_246([a, c, b, d, e, i, f])
    }
}
/// n = 9, i = 4, a < f, b < f, c < g, c < i, d < e, e < g, f < i, g < h
fn select_718([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_246([a, b, c, e, g, h, f])
    } else {
        select_63([c, d, f, e, i])
    }
}
/// n = 12, i = 4, a < f, b < g, b < l, c < f, c < h, d < e, e < g, e < h, f < l, g < k, h < i, h < j
fn select_719([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_717([a, b, c, e, g, h, i, j, k])
    } else {
        select_718([a, c, b, d, e, f, g, k, l])
    }
}
/// n = 12, i = 4, a < h, b < e, b < l, c < f, d < h, d < i, e < g, f < i, h < l, i < j, i < k
fn select_720([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_716([c, a, d, b, e, g, h, f, l])
    } else {
        select_719([a, b, d, c, f, h, e, i, j, k, g, l])
    }
}
/// n = 14, i = 4, a < j, b < f, b < n, c < e, c < k, d < g, d < j, f < i, g < h, g < k, j < n, k < l, k < m
fn select_721([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < g {
        select_720([a, b, c, d, f, e, i, j, g, h, k, n])
    } else {
        select_458([a, b, c, d, f, g, h, i, j, e, k, l, m, n])
    }
}
/// n = 6, i = 2, a < c, b < d, d < e, d < f
fn select_722([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_22([a, e, f])
    } else {
        select_3([c, b, d])
    }
}
/// n = 13, i = 5, a < g, a < j, b < f, c < g, c < h, c < k, d < e, d < k, e < h, e < j, f < k, g < i, k < l, k < m
fn select_723([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if l < a {
        select_722([e, f, h, k, l, m])
    } else {
        select_381([a, b, c, d, e, f, g, h, i, j, k, m])
    }
}
/// n = 15, i = 5, a < g, a < l, b < e, b < m, c < g, c < h, c < j, d < f, d < h, f < j, f < l, g < k, h < i, h < m, m < n, m < o
fn select_724([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < h {
        select_723([a, b, c, d, f, e, g, j, k, l, h, i, m])
    } else {
        select_406([a, b, c, d, f, g, h, i, j, k, l, e, m, n, o])
    }
}
/// n = 15, i = 5, a < h, a < l, b < f, b < m, c < g, c < i, d < e, d < h, d < i, g < l, h < k, i < j, i < m, m < n, m < o
fn select_725([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < g {
        select_721([e, a, b, c, f, h, i, j, k, g, m, n, o, l])
    } else {
        select_724([a, b, d, c, f, g, h, i, j, e, k, l, m, n, o])
    }
}
/// n = 10, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < j, g < h, g < i
fn select_726([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_239([a, d, b, f, g, h, i, j, e])
    } else {
        select_239([a, e, c, f, g, h, i, j, d])
    }
}
/// n = 12, i = 4, b < e, b < g, c < d, c < g, c < j, d < h, d < l, e < f, e < j, e < l, h < i, j < k
fn select_727([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < d {
        select_726([a, c, e, f, g, j, d, h, l, k])
    } else {
        select_653([a, b, d, g, h, i, e, f, j, l])
    }
}
/// n = 14, i = 5, a < f, a < j, b < e, b < h, b < k, c < d, c < h, c < k, e < i, f < g, f < k, f < n, h < i, h < j, h < n, k < l, k < m
fn select_728([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < f {
        select_660([d, e, a, h, j, i, f, g, k, n])
    } else {
        select_507([b, c, f, g, d, e, h, k, l, m, j, n])
    }
}
/// n = 15, i = 5, a < e, a < l, b < g, b < i, c < d, c < f, c < l, e < j, f < i, f < j, f < k, g < h, g < k, g < l, k < m, l < n, l < o
fn select_729([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < a {
        select_727([d, b, f, a, g, h, i, e, j, k, m, l])
    } else {
        select_728([b, a, c, d, e, g, h, f, j, i, l, n, o, k])
    }
}
/// n = 15, i = 5, a < f, a < l, b < e, b < h, c < d, c < g, c < l, f < j, g < j, g < k, h < i, h < k, h < l, k < m, l < n, l < o
fn select_730([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < g {
        select_498([e, a, h, c, d, i, f, g, l, n, o, k])
    } else {
        select_729([a, b, c, d, f, g, h, i, e, j, k, l, m, n, o])
    }
}
/// n = 14, i = 6, a < e, a < n, b < h, b < i, c < i, c < k, c < l, d < f, d < h, e < k, f < g, f < l, f < n, i < j, i < n, l < m
fn select_731([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if l < e {
        select_528([a, b, f, c, g, h, i, l, m, j, e, k, n])
    } else {
        select_709([b, a, c, d, e, f, g, h, i, k, j, l, m, n])
    }
}
/// n = 16, i = 6, a < k, a < l, b < e, b < n, c < f, c < j, d < g, d < j, e < k, f < i, f < n, g < h, g < l, g < n, l < m, n < o, n < p
fn select_732([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < a {
        select_730([b, d, f, i, j, e, a, g, h, k, l, n, m, o, p])
    } else {
        select_731([b, c, a, d, e, g, h, j, f, i, k, l, m, n])
    }
}
/// n = 16, i = 6, a < k, a < l, b < f, b < j, c < e, c < m, d < g, d < j, e < k, f < i, f < p, g < h, g < l, g < m, l < o, m < n, m < p
fn select_733([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if m < f {
        select_725([a, b, c, g, h, j, e, l, m, n, o, k, f, i, p])
    } else {
        select_732([a, c, b, d, e, f, g, h, i, j, k, l, o, m, n, p])
    }
}
/// n = 16, i = 6, a < f, a < l, b < g, b < k, c < e, c < m, d < h, d < k, g < j, g < p, h < i, h < l, h < m, l < o, m < n, m < p
fn select_734([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < e {
        select_712([c, b, a, d, f, g, h, i, j, k, e, l, m, o, n, p])
    } else {
        select_733([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o, p])
    }
}
/// n = 8, i = 3, b < e, b < g, b < h, c < d, c < f, f < g, f < h
fn select_735([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_377([a, b, c, d, e, f])
    } else {
        select_630([d, b, f, a, e, g, h])
    }
}
/// n = 10, i = 4, b < d, b < e, b < i, c < f, e < j, f < g, f < h, f < j, h < i
fn select_736([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_735([a, b, f, g, d, h, e, i])
    } else {
        select_197([a, c, b, d, e, f, g, h, j])
    }
}
/// n = 6, i = 2, a < e, b < c, b < d, e < f
fn select_737([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_105([a, b, c, e, d, f])
    } else {
        select_105([a, b, d, e, c, f])
    }
}
/// n = 8, i = 2, a < d, b < g, b < h, c < e, c < g, c < h, e < f
fn select_738([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_737([c, a, b, d, e, f])
    } else {
        select_47([b, c, e, a, f, d, g, h])
    }
}
/// n = 8, i = 2, c < g, c < h, d < e, d < g, d < h, e < f
fn select_739([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_738([a, c, d, b, e, f, g, h])
    } else {
        select_738([b, c, d, a, e, f, g, h])
    }
}
/// n = 11, i = 4, b < d, b < i, b < k, c < e, d < g, e < f, e < h, e < i, h < k, i < j
fn select_740([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < d {
        select_739([a, f, h, b, i, j, d, k])
    } else {
        select_201([a, c, b, d, e, f, g, i, j, h])
    }
}
/// n = 12, i = 4, b < e, b < l, c < i, c < k, d < f, f < g, f < h, f < i, h < k, h < l, i < j
fn select_741([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < c {
        select_736([a, b, d, e, c, f, g, h, l, i])
    } else {
        select_740([a, c, d, b, f, g, e, h, i, j, k])
    }
}
/// n = 8, i = 3, a < d, b < g, b < h, c < e, e < f, e < g, e < h
fn select_742([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_6([f, a, b, g, h, d])
    } else {
        select_17([b, d, c, e, f])
    }
}
/// n = 9, i = 3, a < h, a < i, b < d, b < g, c < e, e < f, e < g, e < h, e < i
fn select_743([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_172([f, a, b, d, g, h, i])
    } else {
        select_198([a, d, c, e, f, g])
    }
}
/// n = 11, i = 3, a < d, a < k, b < e, b < i, c < f, c < j, e < h, e < j, f < g, f < i, f < k
fn select_744([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_742([a, c, b, d, e, h, f, j])
    } else {
        select_743([b, a, c, d, f, g, k, e, i])
    }
}
/// n = 8, i = 4, a < e, a < f, b < f, b < g, c < d, c < g, g < h
fn select_745([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_416([c, a, e, f, g, h, d])
    } else {
        select_298([d, a, b, e, g, h, f])
    }
}
/// n = 10, i = 4, a < e, a < i, a < j, b < f, b < h, b < j, c < d, c < f, d < j, e < h, f < g, f < i
fn select_746([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_127([d, a, f, g, e, i, j])
    } else {
        select_419([e, b, c, d, h, j, f])
    }
}
/// n = 10, i = 4, a < f, b < i, b < j, c < f, c < g, c < j, d < e, d < g, e < j, g < h, g < i
fn select_747([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_745([a, c, d, e, b, f, g, h])
    } else {
        select_746([b, c, d, e, a, g, h, f, i, j])
    }
}
/// n = 12, i = 4, a < f, a < l, b < h, b < k, c < e, c < g, d < f, d < g, d < h, g < i, g < k, h < j, h < l
fn select_748([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_744([a, c, b, f, g, h, j, i, e, k, l])
    } else {
        select_747([a, b, d, c, e, f, g, i, k, h])
    }
}
/// n = 8, i = 2, a < e, b < c, b < d, b < f, f < g, f < h
fn select_749([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_153([c, d, f, a, g, h])
    } else {
        select_82([a, b, c, d, e, f])
    }
}
/// n = 10, i = 4, a < e, b < g, c < f, c < g, d < f, d < g, g < h, g < i, g < j
fn select_750([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_150([b, c, d, a, e, f])
    } else {
        select_313([a, f, b, g, h, i, j])
    }
}
/// n = 10, i = 4, a < e, b < f, b < j, c < g, d < f, d < g, g < h, g < i, g < j
fn select_751([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_749([a, g, h, i, e, b, f, j])
    } else {
        select_750([a, c, b, d, e, f, g, h, i, j])
    }
}
/// n = 8, i = 4, a < g, b < d, c < e, d < g, d < h, e < f, e < h
fn select_752([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_266([a, d, c, e, g, f, h])
    } else {
        select_16([a, b, f, d])
    }
}
/// n = 8, i = 5, a < h, b < g, b < h, c < e, d < f, e < h, f < g
fn select_753([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_580([a, b, e, d, f, h, g])
    } else {
        select_16([a, c, g, e])
    }
}
/// n = 9, i = 5, a < e, b < g, c < g, c < h, d < f, f < h, f < i, g < i
fn select_754([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_752([a, b, d, g, f, h, e, i])
    } else {
        select_753([b, c, a, d, e, f, h, g])
    }
}
/// n = 10, i = 4, a < d, a < j, b < f, c < e, c < g, c < j, e < i, f < g, f < i, f < j, g < h
fn select_755([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_369([a, c, d, e, g, h, i, j])
    } else {
        select_199([d, b, c, e, f, g, j])
    }
}
/// n = 11, i = 5, a < h, b < e, b < k, c < f, d < g, f < i, f < k, g < h, g < i, g < k, i < j
fn select_756([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_754([b, c, a, d, e, g, f, h, i])
    } else {
        select_755([b, d, f, e, a, g, i, j, h, k])
    }
}
/// n = 12, i = 5, a < h, a < l, b < e, b < k, c < f, c < i, d < g, f < k, g < h, g < i, g < k, i < j, i < l
fn select_757([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_751([b, a, c, g, e, h, i, f, j, l])
    } else {
        select_756([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 13, i = 5, a < h, a < m, b < f, b < i, c < e, c < j, d < g, g < h, g < i, g < j, i < l, j < k, j < m
fn select_758([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < e {
        select_748([a, c, b, g, f, h, i, j, l, k, e, m])
    } else {
        select_757([a, b, c, d, f, e, g, h, j, k, i, m])
    }
}
/// n = 14, i = 5, a < j, a < m, b < f, b < k, c < e, c < h, d < g, d < h, g < j, g < k, h < i, h < m, k < l, m < n
fn select_759([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_741([e, b, a, d, f, h, i, g, m, n, j, k])
    } else {
        select_758([a, b, c, d, e, f, g, j, k, h, i, l, m])
    }
}
/// n = 9, i = 3, a < d, a < h, a < i, b < c, b < e, d < g, e < f, e < h, e < i
fn select_760([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_127([c, a, e, f, h, i, d])
    } else {
        select_304([b, a, d, e, f, g, h, i, c])
    }
}
/// n = 12, i = 3, a < d, a < k, a < l, b < c, b < e, d < g, e < f, e < h, e < k, e < l, h < i, h < j
fn select_761([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < d {
        select_326([c, a, e, f, h, i, j, k, l])
    } else {
        select_760([a, b, c, d, e, f, g, k, l])
    }
}
/// n = 8, i = 4, a < f, b < e, c < g, d < f, d < g, g < h
fn select_762([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_94([b, a, c, f, g, h, e])
    } else {
        select_219([c, a, d, b, e, f, g])
    }
}
/// n = 11, i = 4, a < g, b < f, b < i, c < e, c < i, d < f, d < g, d < i, g < h, i < j, i < k
fn select_763([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_762([b, c, a, d, e, f, g, h])
    } else {
        select_156([a, b, d, c, e, f, i, j, k])
    }
}
/// n = 13, i = 4, a < e, a < m, b < g, b < k, b < l, c < f, c < h, d < f, d < g, d < h, g < j, h < i, h < k, h < l, h < m
fn select_764([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_761([b, c, f, g, h, i, j, a, e, m, k, l])
    } else {
        select_763([b, c, a, d, e, f, g, j, h, i, m])
    }
}
/// n = 14, i = 5, a < f, a < n, b < e, b < i, b < m, c < h, c < j, d < g, g < h, g < i, g < j, i < l, j < k, j < m, j < n
fn select_765([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < e {
        select_764([a, b, c, g, f, h, i, j, k, l, e, m, n])
    } else {
        select_293([a, c, b, d, f, e, g, h, i, l, j])
    }
}
/// n = 11, i = 3, a < e, a < h, a < k, b < c, b < d, b < h, b < j, d < f, d < k, e < g, e < j, h < i
fn select_766([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < e {
        select_512([c, d, a, f, h, i, k, e])
    } else {
        select_131([a, b, d, e, g, f, h, i, c, j, k])
    }
}
/// n = 8, i = 2, a < g, a < h, b < e, b < f, c < d, c < e, c < f, c < g, c < h
fn select_767([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_8([a, c, d, b, g, h])
    } else {
        select_8([b, c, d, a, e, f])
    }
}
/// n = 10, i = 4, a < i, a < j, b < e, b < h, c < f, d < f, f < g, f < h, f < i, f < j
fn select_768([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_149([a, c, d, e, f])
    } else {
        select_767([a, b, f, g, e, h, i, j])
    }
}
/// n = 11, i = 4, a < k, b < e, b < j, c < f, d < f, f < g, f < h, f < j, f < k, h < i
fn select_769([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_626([b, f, g, e, h, a, i, j])
    } else {
        select_768([a, b, c, d, e, f, g, j, h, k])
    }
}
/// n = 13, i = 5, a < d, a < e, a < k, b < g, c < g, e < f, e < m, g < h, g < i, g < k, g < m, i < j, k < l
fn select_770([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < d {
        select_766([a, g, h, i, e, j, f, k, l, m, d])
    } else {
        select_769([d, e, b, c, f, g, h, k, l, m, i])
    }
}
/// n = 9, i = 2, b < e, b < h, b < i, c < d, c < f, c < h, c < i, f < g
fn select_771([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_630([a, b, c, d, e, h, i])
    } else {
        select_280([a, b, c, d, f, g, h, i])
    }
}
/// n = 6, i = 4, a < f, b < f, c < f, d < e
fn select_772([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_70([b, c, d, e, f])
    } else {
        select_70([a, c, d, e, f])
    }
}
/// n = 8, i = 4, b < f, c < f, d < e, d < g, d < h, f < g, f < h
fn select_773([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_8([d, f, a, e, g, h])
    } else {
        select_772([a, b, c, d, e, f])
    }
}
/// n = 10, i = 4, b < e, b < i, b < j, c < f, d < f, f < g, f < i, f < j, g < h
fn select_774([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_170([a, b, f, g, h, i, j])
    } else {
        select_773([a, c, d, b, e, f, i, j])
    }
}
/// n = 11, i = 4, b < e, b < j, b < k, c < f, d < f, f < g, f < h, f < j, f < k, h < i
fn select_775([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_280([a, b, f, g, h, i, j, k])
    } else {
        select_774([a, b, c, d, e, f, h, i, j, k])
    }
}
/// n = 14, i = 5, a < d, a < e, a < k, a < m, b < g, c < g, e < f, e < n, g < h, g < i, g < k, g < n, i < j, i < m, k < l
fn select_776([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < e {
        select_771([h, i, a, d, j, k, l, e, m])
    } else {
        select_775([d, e, b, c, f, g, h, k, l, i, n])
    }
}
/// n = 15, i = 5, a < f, a < l, b < e, b < k, b < n, c < g, d < g, g < h, g < i, g < k, g < l, i < j, i < n, k < m, l < o
fn select_777([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < b {
        select_770([a, c, d, f, b, e, g, h, i, j, l, o, k])
    } else {
        select_776([b, c, d, e, a, f, g, h, i, j, k, m, n, l])
    }
}
/// n = 16, i = 5, a < f, a < n, b < e, b < k, b < m, c < h, c < j, d < g, d < h, g < j, g < k, h < i, h < m, h < n, k < l, m < o, n < p
fn select_778([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < h {
        select_765([a, b, c, d, e, f, g, j, k, h, i, l, m, n])
    } else {
        select_777([a, b, c, d, e, f, h, i, g, j, m, n, o, k, p])
    }
}
/// n = 16, i = 5, a < f, a < h, b < i, b < k, c < e, c < m, c < o, d < g, d < h, d < i, g < k, g < m, h < l, i < j, i < o, m < n, o < p
fn select_779([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < i {
        select_759([b, c, a, d, f, e, g, h, l, k, m, n, i, j])
    } else {
        select_778([a, c, b, d, e, f, g, i, j, k, m, n, o, h, p, l])
    }
}
/// n = 8, i = 3, a < f, b < d, b < g, c < e, c < f, e < g, f < h
fn select_780([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_182([b, e, a, f, h, g])
    } else {
        select_71([a, c, b, d, e, g])
    }
}
/// n = 9, i = 3, a < e, b < f, c < d, c < h, c < i, e < i, f < g, f < h
fn select_781([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_9([c, f, g, d, a, e, h])
    } else {
        select_780([b, c, a, d, e, f, i, g])
    }
}
/// n = 10, i = 4, a < f, b < g, c < e, c < f, c < i, d < e, d < g, d < j, f < j, g < h, g < i
fn select_782([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_82([c, g, a, h, e, i])
    } else {
        select_358([b, c, a, d, e, f, g, j, h])
    }
}
/// n = 11, i = 4, a < h, b < f, b < g, c < e, c < g, c < k, d < e, d < f, d < h, f < i, g < j, h < k
fn select_783([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_781([a, b, c, e, h, f, i, g, k])
    } else {
        select_782([a, b, d, c, e, h, g, j, f, k])
    }
}
/// n = 11, i = 4, a < h, b < g, b < i, c < f, c < g, c < k, d < e, d < f, d < h, e < i, g < j, h < k
fn select_784([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_781([a, e, c, f, h, b, i, g, k])
    } else {
        select_783([a, b, c, d, f, e, g, h, i, j, k])
    }
}
/// n = 5, i = 2, c < d, c < e
fn select_785([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_75([a, c, d, e, b])
    } else {
        select_75([b, c, d, e, a])
    }
}
/// n = 7, i = 3, a < e, a < g, b < c, b < d, b < g, e < f
fn select_786([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < c {
        select_75([e, b, d, g, f])
    } else {
        select_785([c, d, a, e, g])
    }
}
/// n = 11, i = 5, a < f, b < f, b < g, b < k, c < d, c < e, d < j, e < g, e < j, f < h, f < i, j < k
fn select_787([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < f {
        select_73([a, e, d, b, g, j, f, k, h, i])
    } else {
        select_786([c, f, h, i, e, g, d])
    }
}
/// n = 8, i = 3, a < d, a < g, b < c, b < g, b < h, c < f, d < e, d < h
fn select_788([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_61([a, c, f, g, d])
    } else {
        select_8([b, d, e, g, c, h])
    }
}
/// n = 11, i = 5, a < f, a < g, a < k, b < e, b < k, c < d, c < f, d < i, d < j, d < k, e < g, e < j, f < i, g < h
fn select_789([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_788([b, a, f, e, j, i, k, g])
    } else {
        select_788([c, a, g, d, j, h, f, k])
    }
}
/// n = 11, i = 5, a < g, b < f, b < h, b < k, c < f, c < g, d < e, d < k, e < h, e < i, e < j, f < i, g < j, g < k
fn select_790([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_83([b, d, g, f, e, j, k])
    } else {
        select_358([a, b, c, e, h, f, g, i, j])
    }
}
/// n = 13, i = 6, a < e, b < g, b < l, c < d, c < f, d < h, e < j, e < l, f < h, f < i, f < l, g < i, g < j, g < m, h < m, j < k
fn select_791([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < d {
        select_789([e, b, c, f, g, d, j, k, h, i, l])
    } else {
        select_790([a, f, d, b, g, h, e, i, m, j, l])
    }
}
/// n = 13, i = 6, a < d, a < i, b < g, b < l, c < e, c < f, d < l, e < h, f < h, f < j, f < l, g < i, g < j, g < m, h < m, i < k
fn select_792([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < d {
        select_787([a, g, c, e, f, i, j, d, k, h, m])
    } else {
        select_791([a, b, c, e, d, f, g, h, j, i, k, l, m])
    }
}
/// n = 13, i = 6, a < d, a < j, b < g, b < h, c < e, c < f, e < i, f < h, f < i, f < k, g < j, g < k, g < m, i < m, j < l
fn select_793([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < d {
        select_784([e, a, g, f, h, k, j, i, d, l, m])
    } else {
        select_792([a, b, c, d, e, f, g, i, j, k, l, h, m])
    }
}
/// n = 11, i = 2, a < c, a < e, a < k, b < d, b < f, b < k, d < i, e < j, f < g, f < h
fn select_794([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_499([b, a, c, e, d, j, i, k])
    } else {
        select_130([a, b, c, d, f, g, h, i, k])
    }
}
/// n = 11, i = 3, a < e, a < f, b < d, b < g, c < e, c < f, c < g, c < j, d < j, f < k, g < h, g < i
fn select_795([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_9([b, c, e, d, f, k, g])
    } else {
        select_325([d, a, c, g, h, i, e, f, j])
    }
}
/// n = 12, i = 3, a < g, b < d, b < h, b < k, c < e, c < g, c < h, c < l, d < l, e < f, e < k, h < i, h < j
fn select_796([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_794([e, b, f, d, a, h, i, j, l, g, k])
    } else {
        select_795([a, b, c, d, g, e, h, i, j, l, f])
    }
}
/// n = 6, i = 1, a < e, a < f, b < c, b < d
fn select_797([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_22([b, e, f])
    } else {
        select_22([a, c, d])
    }
}
/// n = 8, i = 2, a < c, a < g, a < h, b < d, b < g, b < h, d < e, d < f
fn select_798([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_4([c, b, g, h])
    } else {
        select_797([a, d, e, f, g, h])
    }
}
/// n = 9, i = 2, a < c, a < h, a < i, b < d, b < e, b < h, b < i, d < g, e < f
fn select_799([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_798([a, b, c, d, e, g, h, i])
    } else {
        select_798([a, b, c, e, d, f, h, i])
    }
}
/// n = 11, i = 2, a < c, a < k, b < d, b < e, b < f, b < k, d < j, e < i, f < g, f < h
fn select_800([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_53([d, e, f, a, g, h, i, j])
    } else {
        select_799([a, b, c, d, e, i, j, f, k])
    }
}
/// n = 8, i = 3, a < d, b < e, b < f, c < e, c < f, f < g, f < h
fn select_801([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_39([e, f, g, h, a])
    } else {
        select_86([a, b, c, d, f, g, h, e])
    }
}
/// n = 12, i = 3, a < f, b < d, b < l, c < e, c < f, e < g, e < h, e < l, g < k, h < i, h < j
fn select_802([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_800([b, e, d, a, g, h, i, j, k, f, l])
    } else {
        select_801([b, a, c, d, f, e, g, h])
    }
}
/// n = 13, i = 4, a < d, a < f, b < c, b < e, b < i, b < l, e < g, e < m, f < h, f < i, f < m, h < l, i < j, i < k
fn select_803([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < c {
        select_796([d, f, b, h, e, g, c, i, j, k, m, l])
    } else {
        select_802([c, e, a, g, f, d, h, i, j, k, l, m])
    }
}
/// n = 8, i = 3, b < d, b < g, b < h, c < e, e < f, e < g, e < h
fn select_804([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_110([a, b, e, f, g, h])
    } else {
        select_199([a, c, b, d, e, g, h])
    }
}
/// n = 12, i = 5, b < e, c < d, d < f, d < j, d < k, e < g, e < j, e < l, f < h, f < l, g < i, g < k
fn select_805([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_804([a, f, b, h, e, j, g, l])
    } else {
        select_804([a, g, c, i, d, j, f, k])
    }
}
/// n = 10, i = 4, a < d, a < f, b < c, b < e, b < j, d < j, e < g, e < i, f < h, f < i, f < j
fn select_806([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_196([c, a, e, g, d, f, h, i])
    } else {
        select_655([d, b, f, h, c, e, g, i, j])
    }
}
/// n = 13, i = 6, a < f, b < e, b < g, c < d, c < h, c < i, e < h, e < l, e < m, f < g, f < i, f < l, h < k, i < j, i < m
fn select_807([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_805([d, a, b, e, f, h, i, k, j, l, m, g])
    } else {
        select_806([b, c, d, g, i, e, j, l, m, h])
    }
}
/// n = 16, i = 6, a < d, a < j, b < f, c < e, c < i, e < j, e < l, e < m, f < g, f < i, f < l, g < h, g < m, g < p, j < k, l < p, m < n, m < o
fn select_808([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < a {
        select_803([c, g, h, i, a, e, d, l, m, n, o, p, j])
    } else {
        select_807([b, c, a, d, e, f, i, j, g, h, k, l, m])
    }
}
/// n = 16, i = 6, a < e, a < j, b < g, b < i, c < d, c < f, f < h, f < i, f < l, g < j, g < l, g < m, h < m, h < p, j < k, l < p, m < n, m < o
fn select_809([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if d < h {
        select_793([a, b, c, e, d, f, g, i, h, j, l, k, m])
    } else {
        select_808([a, c, b, e, g, f, h, d, i, j, k, l, m, n, o, p])
    }
}
/// n = 8, i = 3, a < g, b < e, c < d, c < g, e < f, g < h
fn select_810([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_75([c, e, a, f, d])
    } else {
        select_84([b, c, a, d, g, e, h, f])
    }
}
/// n = 6, i = 1, d < e, d < f
fn select_811([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_6([c, a, d, e, f, b])
    } else {
        select_6([c, b, d, e, f, a])
    }
}
/// n = 9, i = 3, a < d, a < i, b < c, b < e, d < h, e < f, e < g, e < i
fn select_812([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_811([c, f, g, a, d, i])
    } else {
        select_41([d, b, c, h, e, f, g, i])
    }
}
/// n = 9, i = 3, b < e, b < i, c < d, c < f, e < h, f < g, f < i
fn select_813([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_810([a, b, c, d, e, h, f, g])
    } else {
        select_812([b, c, d, e, f, a, g, h, i])
    }
}
/// n = 8, i = 3, c < g, c < h, d < e, e < f, e < g, e < h
fn select_814([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_742([a, c, d, b, e, f, g, h])
    } else {
        select_742([b, c, d, a, e, f, g, h])
    }
}
/// n = 11, i = 5, a < e, a < k, b < g, c < g, d < f, d < h, d < k, f < j, g < h, g < j, g < k, h < i
fn select_815([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_773([e, b, c, d, f, g, h, k])
    } else {
        select_369([a, d, e, f, h, i, j, k])
    }
}
/// n = 11, i = 5, a < f, a < k, b < e, b < i, b < k, c < g, d < g, g < h, g < i, g < k, i < j
fn select_816([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_645([a, b, g, f, h, i, j, e, k])
    } else {
        select_815([a, c, d, b, f, e, g, i, j, h, k])
    }
}
/// n = 13, i = 5, a < f, a < i, b < g, b < m, c < e, c < i, c < k, d < g, d < i, g < h, g < k, i < j, i < m, k < l
fn select_817([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_814([e, f, b, d, i, j, g, m])
    } else {
        select_816([a, c, b, d, e, f, g, h, k, l, i])
    }
}
/// n = 13, i = 5, a < f, a < k, a < m, b < e, b < g, c < h, c < m, d < g, d < h, g < j, g < m, h < i, h < k, k < l
fn select_818([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_813([e, c, g, j, h, a, f, i, m])
    } else {
        select_817([b, c, a, d, f, e, h, i, g, j, k, l, m])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_819([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_651([a, d, b, f, g, h, i, e])
    } else {
        select_651([a, e, c, f, g, h, i, d])
    }
}
/// n = 10, i = 3, b < e, b < i, b < j, c < d, c < f, f < g, f < i, f < j, g < h
fn select_820([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_55([a, d, f, g, b, e, i, j, h])
    } else {
        select_819([a, b, c, d, e, f, g, i, j])
    }
}
/// n = 14, i = 4, b < d, b < g, b < k, c < e, c < g, d < f, d < n, e < h, e < k, e < n, h < i, h < j, j < l, k < m
fn select_821([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < h {
        select_820([a, d, c, g, f, e, k, m, h, n])
    } else {
        select_56([a, b, e, g, h, i, j, k, l, m, d])
    }
}
/// n = 6, i = 3, a < e, a < f, b < d, b < f, c < d, c < e
fn select_822([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_69([a, c, d, f])
    } else {
        select_69([b, c, e, f])
    }
}
/// n = 7, i = 4, a < f, a < g, b < e, c < d, c < f, e < g
fn select_823([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_822([a, c, e, d, g, f])
    } else {
        select_70([b, d, a, f, e])
    }
}
/// n = 10, i = 4, a < d, b < f, c < e, c < g, e < h, e < i, e < j, f < g, f < h, f < i, f < j
fn select_824([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_823([c, b, a, d, f, e, g])
    } else {
        select_222([b, e, a, f, g, d, h, i, j])
    }
}
/// n = 10, i = 4, c < f, d < e, d < g, e < h, e < i, e < j, f < g, f < h, f < i, f < j
fn select_825([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_824([a, c, d, b, e, f, g, h, i, j])
    } else {
        select_824([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 15, i = 6, a < f, a < i, b < e, b < i, b < l, c < h, c < i, d < g, d < k, g < h, g < n, g < o, h < j, h < l, i < k, i < n, i < o, l < m
fn select_826([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_825([e, f, c, d, g, i, k, h, n, o])
    } else {
        select_816([a, b, c, g, e, f, h, j, l, m, i])
    }
}
/// n = 16, i = 6, a < f, a < p, b < e, b < h, b < l, c < h, c < i, d < g, d < k, g < i, g < m, g < p, h < k, h < m, h < p, i < j, i < l, l < n, m < o
fn select_827([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < a {
        select_821([e, h, d, a, g, f, k, i, j, l, m, n, o, p])
    } else {
        select_826([a, b, c, d, e, f, g, i, h, j, k, l, n, m, p])
    }
}
/// n = 17, i = 6, a < f, a < i, b < e, b < h, b < n, c < h, c < j, d < g, d < l, g < i, g < j, g < o, h < l, h < o, i < m, j < k, j < n, n < p, o < q
fn select_828([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if i < h {
        select_818([b, a, c, g, f, e, i, j, k, m, n, p, h])
    } else {
        select_827([a, b, c, d, e, f, g, h, j, k, l, n, o, p, q, i])
    }
}
/// n = 17, i = 6, a < f, a < j, b < k, b < q, c < e, c < g, c < o, d < h, d < i, g < i, g < n, h < j, h < k, h < n, j < m, k < l, k < o, n < q, o < p
fn select_829([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if g < b {
        select_809([a, d, c, e, f, g, h, b, i, j, m, n, k, l, o, q])
    } else {
        select_828([a, c, b, d, e, f, h, g, j, k, l, i, m, o, n, p, q])
    }
}
/// n = 17, i = 6, a < f, a < k, b < l, b < n, c < e, c < g, c < p, d < h, d < j, g < j, h < i, h < k, h < l, i < n, k < o, l < m, l < p, p < q
fn select_830([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if i < g {
        select_779([a, b, c, h, e, f, i, k, l, m, n, o, g, j, p, q])
    } else {
        select_829([a, b, c, d, e, f, g, h, j, k, l, m, o, i, p, q, n])
    }
}
/// n = 10, i = 4, a < h, b < f, b < h, c < f, c < g, d < e, d < h, e < g, g < i, h < j
fn select_831([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_185([a, b, c, g, i, f, h])
    } else {
        select_780([a, c, d, f, e, h, g, j])
    }
}
/// n = 10, i = 4, a < h, b < h, c < e, c < g, d < f, d < h, f < g, g < i, h < j
fn select_832([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_277([b, e, d, a, f, g, i])
    } else {
        select_831([b, a, c, d, f, e, g, h, i, j])
    }
}
/// n = 11, i = 5, a < h, b < e, b < h, c < d, c < f, e < j, f < g, f < j, h < i, j < k
fn select_833([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < d {
        select_347([e, a, c, h, f, g, i, d, j, k])
    } else {
        select_832([a, d, f, b, g, e, j, h, k, i])
    }
}
/// n = 4, i = 3
fn select_834([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_286([b, c, d])
    } else {
        select_286([a, c, d])
    }
}
/// n = 7, i = 4, a < e, b < f, c < f, d < f, f < g
fn select_835([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_3([a, f, g])
    } else {
        select_834([b, c, d, e])
    }
}
/// n = 8, i = 4, b < e, b < h, c < f, d < f, f < g, f < h
fn select_836([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_835([b, a, c, d, e, f, g])
    } else {
        select_82([b, f, a, g, e, h])
    }
}
/// n = 8, i = 3, b < d, b < g, c < e, c < g, e < f, g < h
fn select_837([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < a {
        select_92([e, b, d, f, g, a, h])
    } else {
        select_298([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 4, b < g, c < e, c < h, d < f, f < g, f < h, h < i
fn select_838([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_836([a, c, b, d, e, f, g, h])
    } else {
        select_837([a, c, f, e, b, g, h, i])
    }
}
/// n = 9, i = 4, a < e, b < c, b < d, d < f, d < i, e < g, e < i, g < h
fn select_839([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < g {
        select_145([c, d, a, f, e, i, g])
    } else {
        select_421([b, e, g, d, h, f, i, c])
    }
}
/// n = 8, i = 3, c < h, d < e, e < f, e < h, f < g
fn select_840([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_277([b, a, d, c, e, f, g])
    } else {
        select_517([b, c, d, a, e, f, g, h])
    }
}
/// n = 11, i = 5, a < h, b < f, c < d, c < e, e < g, e < j, f < h, f < j, h < i, j < k
fn select_841([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < e {
        select_839([b, c, d, e, f, g, h, i, j])
    } else {
        select_840([d, g, a, b, f, j, k, h])
    }
}
/// n = 13, i = 6, a < j, a < k, b < g, b < h, c < g, c < i, d < e, d < f, f < h, f < i, g < j, g < m, i < k, i < m, j < l
fn select_842([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_838([e, a, b, c, h, i, k, g, m])
    } else {
        select_841([a, c, d, e, f, g, h, j, l, i, m])
    }
}
/// n = 13, i = 6, a < j, b < i, b < l, c < f, c < h, d < e, d < g, f < i, f < m, g < h, g < j, i < k, j < l, j < m
fn select_843([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_833([b, f, d, e, a, g, h, i, k, j, m])
    } else {
        select_842([b, c, a, d, e, g, f, h, j, i, l, k, m])
    }
}
/// n = 8, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, g < h
fn select_844([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_276([d, f, g, h, a])
    } else {
        select_99([a, b, c, d, e, g, h, f])
    }
}
/// n = 10, i = 5, b < f, b < i, c < f, c < h, c < j, d < e, e < h, e < i, f < g, i < j
fn select_845([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < i {
        select_123([a, b, d, e, f, h, g, i])
    } else {
        select_199([a, b, c, h, i, f, j])
    }
}
/// n = 10, i = 4, a < d, b < h, c < e, d < g, e < f, e < h, h < i, h < j
fn select_846([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_94([b, d, c, g, e, f, h])
    } else {
        select_249([a, c, d, e, f, g, h, i, j])
    }
}
/// n = 13, i = 5, a < j, b < f, b < i, c < f, c < g, c < j, c < k, d < e, e < g, e < i, e < j, f < h, i < k, j < l, j < m
fn select_847([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < j {
        select_845([a, b, c, d, e, f, h, g, i, k])
    } else {
        select_846([b, a, d, f, e, g, h, j, l, m])
    }
}
/// n = 14, i = 6, a < i, b < g, b < h, c < e, c < m, d < f, e < h, e < j, e < n, f < g, f < i, f < j, g < n, h < k, i < l, i < m
fn select_848([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < e {
        select_844([b, c, i, l, g, m, e, j])
    } else {
        select_847([a, b, e, d, f, h, j, k, g, i, n, l, m])
    }
}
/// n = 7, i = 3, a < g, b < e, b < g, c < d, c < f, c < g, e < f
fn select_849([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_61([c, e, a, d, f])
    } else {
        select_173([a, b, c, d, e, g])
    }
}
/// n = 8, i = 3, b < e, b < f, c < d, c < f, c < h, e < h, f < g
fn select_850([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_6([d, e, f, a, g, h])
    } else {
        select_849([a, b, c, d, e, h, f])
    }
}
/// n = 7, i = 2, b < d, c < e, e < f, e < g
fn select_851([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_297([a, c, b, d, e])
    } else {
        select_214([b, c, d, e, a, f, g])
    }
}
/// n = 11, i = 5, a < j, b < e, c < f, c < i, d < f, d < h, d < k, e < h, e < i, f < g, f < j, i < k
fn select_852([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_850([a, c, d, h, i, f, g, k])
    } else {
        select_851([g, a, b, j, e, h, i])
    }
}
/// n = 9, i = 4, a < f, a < g, b < d, c < e, d < f, d < i, e < g, e < i, g < h
fn select_853([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_12([a, c, f, e, i])
    } else {
        select_105([a, b, d, g, i, h])
    }
}
/// n = 9, i = 4, a < g, b < d, c < e, d < f, d < h, e < g, e < h, g < i
fn select_854([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_12([f, c, a, e, h])
    } else {
        select_853([a, b, c, d, e, f, g, i, h])
    }
}
/// n = 12, i = 5, a < h, a < i, b < f, c < e, d < f, d < g, d < j, e < g, e < h, e < k, f < i, f < k, h < j, i < l
fn select_855([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_71([b, a, d, g, h, j])
    } else {
        select_854([a, c, b, e, f, g, i, k, l])
    }
}
/// n = 8, i = 4, a < e, b < f, c < f, d < e, d < g, d < h, f < g, f < h
fn select_856([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_70([b, c, d, e, f])
    } else {
        select_4([a, d, g, h])
    }
}
/// n = 11, i = 5, a < g, a < h, b < e, c < f, d < f, e < g, e < i, e < j, f < h, f < i, f < j, h < k
fn select_857([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_3([b, h, k])
    } else {
        select_856([a, c, d, e, g, f, i, j])
    }
}
/// n = 12, i = 5, a < g, a < i, b < f, b < j, c < e, d < f, d < h, e < g, e < h, e < k, f < i, f < k, h < j, i < l
fn select_858([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_181([a, b, d, g, h, f, j])
    } else {
        select_857([a, c, b, d, e, f, g, i, h, k, l])
    }
}
/// n = 14, i = 6, a < h, a < k, b < f, c < g, d < e, d < l, e < i, e < k, e < m, f < j, f < k, g < h, g < i, g < j, h < m, j < l, k < n
fn select_859([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < f {
        select_855([a, b, c, e, g, f, i, h, k, m, j, n])
    } else {
        select_858([a, d, c, f, g, e, h, j, k, l, i, n])
    }
}
/// n = 14, i = 6, a < e, b < h, b < i, c < g, d < f, d < n, e < l, f < i, f < j, f < m, g < h, g < j, g < l, h < m, i < k, l < n
fn select_860([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < e {
        select_852([a, c, b, f, g, i, k, j, h, e, m])
    } else {
        select_859([b, a, c, d, f, e, g, h, j, l, i, n, m, k])
    }
}
/// n = 14, i = 6, a < e, a < j, b < h, b < i, c < g, d < f, d < m, f < i, f < k, f < n, g < h, g < j, g < k, h < n, i < l, j < m
fn select_861([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < e {
        select_848([a, b, d, c, f, g, h, i, j, k, l, e, m, n])
    } else {
        select_860([a, b, c, d, e, f, g, h, i, k, l, j, n, m])
    }
}
/// n = 14, i = 6, a < e, a < f, b < j, b < k, c < g, d < h, d < i, f < i, g < k, g < m, h < j, h < m, h < n, j < l, k < n
fn select_862([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < g {
        select_843([c, b, d, a, e, h, f, i, j, g, l, k, m])
    } else {
        select_861([a, b, c, d, e, h, g, k, j, f, m, l, i, n])
    }
}
/// n = 7, i = 3, a < f, b < f, b < g, c < d, d < e, d < g
fn select_863([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_294([a, e, b, g])
    } else {
        select_12([b, c, f, d, e])
    }
}
/// n = 8, i = 4, a < h, b < g, b < h, c < f, c < h, d < e, e < f, e < g
fn select_864([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_863([a, c, d, e, g, h, f])
    } else {
        select_863([a, b, d, e, f, h, g])
    }
}
/// n = 9, i = 4, a < e, a < h, b < e, b < g, c < d, c < i, d < g, d < h, e < f, e < i
fn select_865([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_4([f, c, d, i])
    } else {
        select_61([a, b, g, h, e])
    }
}
/// n = 9, i = 4, b < f, b < h, c < f, c < g, d < e, d < i, e < g, e < h, f < i
fn select_866([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_864([a, b, c, d, e, g, h, f])
    } else {
        select_865([b, c, d, e, f, a, g, h, i])
    }
}
/// n = 8, i = 4, a < h, b < f, c < f, c < h, d < e, d < g, e < h, f < g
fn select_867([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < e {
        select_71([a, c, d, e, f, g])
    } else {
        select_63([a, b, c, f, h])
    }
}
/// n = 8, i = 4, b < f, c < f, c < g, d < e, d < h, e < g, f < h
fn select_868([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < g {
        select_867([a, b, c, d, e, f, h, g])
    } else {
        select_105([b, e, g, f, a, h])
    }
}
/// n = 10, i = 5, b < e, b < i, c < g, c < j, d < f, d < i, e < j, f < h, f < j, g < h, g < i
fn select_869([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_866([a, b, f, c, g, e, h, i, j])
    } else {
        select_868([a, e, d, c, g, f, i, j])
    }
}
/// n = 8, i = 4, a < f, a < g, b < e, c < d, d < f, d < h, e < g, e < h
fn select_870([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_12([a, c, g, d, h])
    } else {
        select_12([a, b, f, e, h])
    }
}
/// n = 9, i = 4, a < g, b < e, c < d, c < h, d < g, d < i, e < f, e < i, f < h
fn select_871([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_13([f, c, a, d, i, h])
    } else {
        select_870([a, b, c, d, e, g, f, i])
    }
}
/// n = 8, i = 3, a < h, b < f, c < f, c < g, d < e, e < g, e < h
fn select_872([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < a {
        select_121([b, d, c, a, e, f, g])
    } else {
        select_479([b, d, a, c, e, h, f])
    }
}
/// n = 9, i = 4, a < h, b < f, c < d, c < e, e < h, e < i, f < g, f < i
fn select_873([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_871([a, b, c, e, f, g, h, d, i])
    } else {
        select_872([d, a, e, b, f, h, i, g])
    }
}
/// n = 12, i = 6, a < f, a < l, b < e, b < i, c < d, c < g, e < k, e < l, f < i, g < h, g < l, h < k, i < j
fn select_874([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < i {
        select_869([d, a, b, g, f, h, e, k, l, i])
    } else {
        select_873([e, f, c, d, g, i, j, l, h])
    }
}
/// n = 6, i = 2, c < f, d < e, d < f
fn select_875([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_482([c, a, d, e, b, f])
    } else {
        select_482([c, b, d, e, a, f])
    }
}
/// n = 7, i = 3, a < e, b < c, b < d, d < f, e < g
fn select_876([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_221([a, b, c, d, e, g])
    } else {
        select_180([a, c, d, f])
    }
}
/// n = 8, i = 3, a < d, b < c, b < e, d < h, e < f, e < g
fn select_877([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_785([a, c, e, f, g])
    } else {
        select_876([a, b, c, e, d, g, h])
    }
}
/// n = 9, i = 4, a < g, b < g, b < h, c < d, c < e, e < f, e < h, g < i
fn select_878([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_875([d, f, a, b, h, g])
    } else {
        select_877([a, c, d, g, e, f, h, i])
    }
}
/// n = 7, i = 4, b < e, c < f, d < f, f < g
fn select_879([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_835([b, a, c, d, e, f, g])
    } else {
        select_75([b, f, a, g, e])
    }
}
/// n = 6, i = 3, c < f, d < e, e < f
fn select_880([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_71([b, d, c, a, e, f])
    } else {
        select_68([b, a, d, e, c])
    }
}
/// n = 9, i = 4, a < g, b < f, c < g, c < i, d < e, d < f, f < i, g < h
fn select_881([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_879([b, d, a, c, e, g, h])
    } else {
        select_880([a, e, c, b, f, i])
    }
}
/// n = 9, i = 4, b < g, c < g, c < h, d < e, d < f, f < h, g < i
fn select_882([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_878([b, c, d, e, f, a, g, h, i])
    } else {
        select_881([b, a, c, d, e, f, g, i, h])
    }
}
/// n = 12, i = 6, a < f, a < i, b < e, b < j, c < d, c < g, e < l, f < j, g < h, g < i, h < l, j < k
fn select_883([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_874([a, b, c, d, e, f, g, h, j, k, l, i])
    } else {
        select_882([d, f, b, g, h, i, j, e, k])
    }
}
/// n = 9, i = 3, a < d, a < i, b < g, b < h, b < i, c < e, c < i, e < f, e < g, e < h
fn select_884([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_6([f, a, b, g, h, d])
    } else {
        select_64([d, b, c, e, f, i])
    }
}
/// n = 9, i = 3, a < e, a < i, b < g, b < h, b < i, c < d, c < f, c < i, f < g, f < h
fn select_885([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_298([b, a, c, e, d, f, i])
    } else {
        select_884([a, b, c, e, f, d, g, h, i])
    }
}
/// n = 8, i = 4, a < f, b < e, c < f, c < h, d < f, d < h, e < h, f < g
fn select_886([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_302([b, c, d, e, f, g])
    } else {
        select_16([a, c, d, h])
    }
}
/// n = 7, i = 2, a < c, b < d, c < g, d < e, d < f
fn select_887([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_22([a, e, f])
    } else {
        select_38([b, c, g, d])
    }
}
/// n = 9, i = 4, a < g, b < e, b < f, c < f, c < g, d < f, d < g, f < h, g < i
fn select_888([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_886([a, b, c, d, e, g, i, f])
    } else {
        select_887([a, b, g, f, e, h, i])
    }
}
/// n = 10, i = 4, a < e, b < h, b < j, c < f, c < h, d < f, d < h, f < g, f < j, h < i
fn select_889([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_499([b, f, g, a, h, e, i, j])
    } else {
        select_888([b, a, c, d, e, f, h, g, i])
    }
}
/// n = 9, i = 3, a < d, b < e, b < f, c < e, c < f, e < i, f < g, f < h
fn select_890([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_40([e, f, a, g, h, i])
    } else {
        select_83([a, b, c, d, e, i, f])
    }
}
/// n = 11, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < j, f < k, g < j, g < k, h < i
fn select_891([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_586([e, b, c, d, f, g, h])
    } else {
        select_890([a, b, c, e, h, g, j, k, i])
    }
}
/// n = 12, i = 5, a < e, b < g, b < j, c < g, c < h, c < j, d < f, f < h, f < j, f < l, g < l, h < i, j < k
fn select_892([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_889([a, b, c, f, e, h, i, j, k, g])
    } else {
        select_891([a, b, c, d, e, f, g, j, k, h, l])
    }
}
/// n = 13, i = 5, a < e, a < m, b < g, b < j, c < g, c < j, c < m, d < f, f < h, f < j, f < l, g < l, h < i, h < m, j < k
fn select_893([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_885([a, c, h, i, e, b, g, j, m])
    } else {
        select_892([a, c, b, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 9, i = 4, b < g, b < i, c < e, c < f, d < f, d < g, d < i, e < i, g < h
fn select_894([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_281([a, h, c, e, f])
    } else {
        select_100([a, e, b, d, f, g, h, i])
    }
}
/// n = 6, i = 2, c < e, c < f, d < e, d < f
fn select_895([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_198([a, b, c, d, e, f])
    } else {
        select_198([a, b, d, c, e, f])
    }
}
/// n = 12, i = 5, a < e, a < h, b < f, b < l, c < g, c < h, c < i, d < g, d < h, d < i, f < i, f < k, g < k, h < l, i < j
fn select_896([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_895([e, g, b, h, f, l])
    } else {
        select_535([a, c, d, b, e, f, g, i, j, k, h])
    }
}
/// n = 14, i = 6, a < e, a < h, b < f, b < j, c < g, c < n, d < h, d < i, d < k, f < k, f < m, f < n, g < i, g < k, h < j, h < n, i < m, k < l
fn select_897([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_894([e, c, b, h, f, j, g, i, n])
    } else {
        select_896([a, b, d, g, e, f, i, h, k, l, m, j])
    }
}
/// n = 15, i = 6, a < e, a < h, b < h, b < j, b < m, c < g, c < l, d < f, d < i, f < j, f < m, g < i, g < m, g < o, h < l, i < k, j < o, m < n
fn select_898([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_893([a, f, b, c, e, g, j, i, k, m, n, o, h])
    } else {
        select_897([a, c, d, b, e, g, f, h, j, l, m, n, o, i])
    }
}
/// n = 15, i = 6, a < e, a < f, b < k, b < m, c < h, c < i, d < g, d < j, f < i, g < k, g < m, h < j, h < m, h < o, j < l, k < o, m < n
fn select_899([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < b {
        select_883([c, d, a, e, g, h, f, b, i, j, l, k])
    } else {
        select_898([a, b, c, d, e, g, h, f, j, k, l, i, m, n, o])
    }
}
/// n = 15, i = 6, a < e, a < g, b < f, b < n, c < h, c < k, d < i, d < j, f < m, g < j, h < n, i < k, i < m, i < n, k < l, n < o
fn select_900([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < h {
        select_862([a, c, b, d, e, g, f, i, j, k, h, l, m, n])
    } else {
        select_899([a, b, d, c, e, g, h, i, j, k, f, l, n, o, m])
    }
}
/// n = 10, i = 2, b < e, b < g, b < j, c < d, c < f, c < j, f < h, g < i
fn select_901([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_54([a, e, g, c, d, f, j, i])
    } else {
        select_54([a, d, f, b, e, g, j, h])
    }
}
/// n = 9, i = 3, a < c, a < d, a < h, b < e, b < i, d < i, e < f, e < h, f < g
fn select_902([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < e {
        select_63([c, d, b, i, e])
    } else {
        select_47([a, e, f, d, g, i, c, h])
    }
}
/// n = 11, i = 3, a < c, a < d, a < j, b < e, b < k, d < k, e < f, e < g, e < j, f < i, g < h
fn select_903([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_369([a, e, c, f, g, h, i, j])
    } else {
        select_902([a, b, c, d, e, f, i, j, k])
    }
}
/// n = 11, i = 3, a < c, a < d, a < j, b < e, b < k, d < k, e < f, e < g, e < h, e < j, h < i
fn select_904([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_903([a, b, c, d, e, f, h, i, g, j, k])
    } else {
        select_903([a, b, c, d, e, g, h, i, f, j, k])
    }
}
/// n = 15, i = 4, a < c, a < j, a < l, b < d, b < o, c < f, c < g, d < e, d < h, d < j, d < l, g < o, h < i, h < k, k < m, l < n
fn select_905([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < c {
        select_901([e, a, h, i, j, k, l, m, n, c])
    } else {
        select_904([c, b, f, g, d, e, j, l, n, h, o])
    }
}
/// n = 10, i = 3, a < d, a < j, b < g, b < h, b < i, c < e, e < f, e < g, e < h, e < i, e < j
fn select_906([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_316([c, a, b, d, e, f, j])
    } else {
        select_222([c, b, a, e, f, d, g, h, i])
    }
}
/// n = 10, i = 3, a < d, b < h, b < i, b < j, c < e, e < f, e < g, e < h, e < i, e < j
fn select_907([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_238([g, f, b, h, i, j, a])
    } else {
        select_906([a, b, c, d, e, g, h, i, j, f])
    }
}
/// n = 7, i = 3, b < g, c < f, c < g, d < e, d < g, e < f
fn select_908([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_513([b, c, a, d, e, g])
    } else {
        select_133([b, c, e, a, f, g])
    }
}
/// n = 9, i = 5, a < i, b < g, b < i, c < g, c < i, d < e, e < f, e < i, g < h
fn select_909([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_145([a, g, d, h, e, f, i])
    } else {
        select_111([a, f, b, c, g, i])
    }
}
/// n = 11, i = 5, a < i, a < k, b < g, b < k, c < g, c < j, c < k, d < e, e < f, e < i, e < k, g < h, i < j
fn select_910([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < g {
        select_908([f, b, c, a, i, j, k])
    } else {
        select_909([a, b, c, d, e, f, g, h, k])
    }
}
/// n = 15, i = 5, a < e, a < n, a < o, b < i, b < j, c < g, c < j, d < g, d < j, d < m, e < f, e < i, g < h, i < m, j < k, j < l, j < n, j < o
fn select_911([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < e {
        select_907([g, a, b, h, j, k, l, e, n, o])
    } else {
        select_910([b, c, d, a, e, f, g, h, i, m, j])
    }
}
/// n = 16, i = 5, a < g, a < m, b < e, b < l, b < n, c < h, c < k, d < g, d < h, d < p, e < f, e < k, g < j, h < i, h < l, h < m, h < n, k < p, n < o
fn select_912([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < a {
        select_905([b, d, e, h, i, f, k, a, m, l, g, n, j, o, p])
    } else {
        select_911([b, c, a, d, e, f, g, j, k, h, i, m, p, l, n])
    }
}
/// n = 7, i = 3, b < e, c < d, c < g, d < f, e < g
fn select_913([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < b {
        select_294([a, f, b, e])
    } else {
        select_64([a, e, c, d, f, g])
    }
}
/// n = 8, i = 4, a < h, b < f, b < h, c < d, c < e, e < h, f < g
fn select_914([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_460([a, b, c, d, f, g, e, h])
    } else {
        select_913([d, e, b, f, a, g, h])
    }
}
/// n = 11, i = 5, a < g, a < h, b < i, b < k, c < f, c < g, d < e, d < i, e < h, e < k, f < i, f < k, i < j
fn select_915([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_914([b, d, c, g, f, e, h, i])
    } else {
        select_351([a, b, d, g, e, i, j, k, h])
    }
}
/// n = 9, i = 4, a < i, b < e, c < d, c < h, d < f, d < i, e < g, e < h
fn select_916([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_345([a, g, c, h, d, f])
    } else {
        select_851([f, a, b, i, e, g, h])
    }
}
/// n = 12, i = 6, a < g, a < h, b < f, b < i, c < d, c < e, d < l, e < h, e < j, f < j, f < l, g < i, g < l, i < k
fn select_917([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < i {
        select_915([e, d, a, b, f, g, h, j, i, k, l])
    } else {
        select_916([f, g, c, e, i, h, k, d, j])
    }
}
/// n = 9, i = 4, a < g, a < i, b < f, b < g, c < d, c < f, d < e, f < i, g < h
fn select_918([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_166([a, d, b, e, f, i, g])
    } else {
        select_13([g, c, h, f, i, d])
    }
}
/// n = 6, i = 3, b < d, c < e, e < f
fn select_919([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < f {
        select_68([a, b, c, e, d])
    } else {
        select_180([a, b, e, f])
    }
}
/// n = 9, i = 4, a < e, b < d, b < g, c < f, d < h, e < i, f < g, f < h, f < i
fn select_920([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_863([a, d, c, f, g, e, h])
    } else {
        select_870([b, a, c, f, e, g, d, i])
    }
}
/// n = 9, i = 4, a < e, b < d, b < g, c < f, c < g, d < h, e < i, f < h, f < i
fn select_921([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_919([d, a, c, e, g, f])
    } else {
        select_920([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 12, i = 5, a < g, a < i, b < e, b < h, c < d, c < g, d < f, d < h, e < i, e < k, e < l, f < k, g < l, i < j
fn select_922([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_918([e, a, c, d, h, g, i, j, l])
    } else {
        select_921([a, d, b, f, g, e, h, k, l])
    }
}
/// n = 11, i = 4, a < g, a < k, b < g, b < h, c < e, c < f, d < f, d < g, d < k, e < k, f < h, g < i, g < j
fn select_923([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_97([a, d, e, b, f, k])
    } else {
        select_73([a, c, d, b, e, f, g, h, i, j])
    }
}
/// n = 9, i = 2, a < d, b < c, b < e, c < h, d < i, e < f, e < g
fn select_924([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_40([c, e, a, f, g, h])
    } else {
        select_378([a, b, c, d, h, i, e])
    }
}
/// n = 12, i = 4, a < d, a < l, b < e, b < g, c < f, c < l, d < k, e < j, e < l, f < g, f < j, f < k, g < h, g < i
fn select_925([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_924([a, b, e, d, g, h, i, j, k])
    } else {
        select_714([d, b, c, e, f, g, l, k])
    }
}
/// n = 13, i = 5, a < f, a < h, b < g, b < i, c < g, c < h, c < i, d < e, d < f, e < i, e < l, e < m, g < l, h < m, i < j, i < k
fn select_926([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_923([b, e, a, c, f, h, i, m, j, k, g])
    } else {
        select_925([a, b, d, h, g, e, i, j, k, l, m, f])
    }
}
/// n = 15, i = 6, a < g, a < h, b < e, b < f, c < d, c < i, c < l, d < j, e < l, f < h, f < i, g < j, g < l, g < m, i < m, j < k, l < n, l < o
fn select_927([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if d < e {
        select_922([d, a, b, f, g, i, e, h, j, k, m, l])
    } else {
        select_926([f, e, c, a, g, h, d, i, l, n, o, j, m])
    }
}
/// n = 15, i = 6, a < j, b < i, b < l, c < g, c < h, d < e, d < f, e < l, f < h, f < i, g < j, g < l, g < m, i < m, j < k, l < n, l < o
fn select_928([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < b {
        select_917([c, a, d, e, f, b, g, h, j, i, k, l])
    } else {
        select_927([c, d, b, a, e, f, g, h, i, j, k, l, m, n, o])
    }
}
/// n = 17, i = 6, a < j, a < n, b < i, b < k, c < e, c < f, c < o, d < g, d < h, f < h, f < i, g < j, g < k, g < q, i < q, j < m, k < l, k < n, k < o, o < p
fn select_929([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if k < e {
        select_912([a, c, b, g, f, h, j, k, l, m, i, e, n, o, p, q])
    } else {
        select_928([a, b, d, c, e, f, g, h, i, j, m, k, q, l, o])
    }
}
/// n = 9, i = 2, a < d, a < i, b < c, b < e, b < f, b < i, e < h, f < g
fn select_930([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_626([a, b, c, d, e, f, h, i])
    } else {
        select_626([a, b, c, d, f, e, g, i])
    }
}
/// n = 11, i = 3, a < d, a < h, b < f, b < g, b < j, c < e, c < f, c < g, c < h, e < j, g < i, h < k
fn select_931([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_280([d, b, c, e, h, k, f, g])
    } else {
        select_930([a, c, f, d, e, g, i, j, h])
    }
}
/// n = 12, i = 3, a < h, a < l, b < d, b < i, b < k, c < e, c < f, c < h, c < i, e < l, f < g, f < k, i < j
fn select_932([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_901([e, b, f, g, d, a, i, h, j, k])
    } else {
        select_931([b, a, c, d, e, h, f, i, g, l, j])
    }
}
/// n = 10, i = 2, a < d, a < e, a < j, b < c, b < f, b < j, e < i, f < g, f < h
fn select_933([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_9([b, a, d, c, e, i, j])
    } else {
        select_626([a, b, c, d, f, g, h, j])
    }
}
/// n = 10, i = 3, a < d, a < f, b < e, b < g, c < e, c < f, c < g, f < j, g < h, g < i
fn select_934([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_18([b, c, a, d, e, f, j])
    } else {
        select_801([a, b, c, d, e, g, h, i])
    }
}
/// n = 11, i = 3, a < g, a < k, b < d, b < h, b < j, c < e, c < g, c < h, e < f, e < j, e < k, h < i
fn select_935([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_933([b, e, f, d, h, a, g, k, i, j])
    } else {
        select_934([b, a, c, d, g, h, e, f, k, i])
    }
}
/// n = 9, i = 3, a < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_936([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_68([b, a, c, d, e])
    } else {
        select_313([a, d, b, f, g, h, i])
    }
}
/// n = 7, i = 2, a < c, a < g, b < d, b < g, d < e, d < f
fn select_937([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_22([a, e, f])
    } else {
        select_4([c, b, d, g])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < h, c < d, c < f, c < h, f < g
fn select_938([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_937([a, c, e, f, d, g, h])
    } else {
        select_419([a, b, c, d, e, f, h])
    }
}
/// n = 11, i = 3, a < d, a < f, b < e, b < g, c < e, c < f, c < g, f < k, g < h, g < i, g < j
fn select_939([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_263([e, g, a, h, i, j])
    } else {
        select_938([b, c, a, d, e, f, k, g])
    }
}
/// n = 12, i = 4, a < d, a < i, b < e, b < h, c < f, e < g, e < j, e < k, f < h, f < i, f < j, f < k, i < l
fn select_940([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_936([a, c, e, g, d, f, h, j, k])
    } else {
        select_939([a, b, f, d, h, i, e, g, j, k, l])
    }
}
/// n = 14, i = 4, a < e, a < j, b < d, b < k, b < m, c < f, e < g, e < n, f < h, f < j, f < k, f < n, h < i, h < m, k < l
fn select_941([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < e {
        select_935([a, b, f, d, h, i, j, k, l, m, e])
    } else {
        select_940([b, a, c, d, e, f, g, j, k, h, n, l])
    }
}
/// n = 14, i = 4, a < e, a < k, b < d, b < l, b < n, c < f, e < h, f < g, f < i, f < k, f < l, i < j, i < n, l < m
fn select_942([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < e {
        select_932([a, b, f, d, g, i, j, k, l, m, n, e])
    } else {
        select_941([a, b, c, d, e, f, h, i, j, k, l, m, n, g])
    }
}
/// n = 9, i = 3, a < e, b < g, c < d, c < g, e < f, g < h, g < i
fn select_943([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_198([a, d, b, g, h, i])
    } else {
        select_919([b, c, a, d, e, f])
    }
}
/// n = 13, i = 5, a < i, b < f, b < k, c < f, c < g, c < k, d < e, e < g, e < i, e < k, f < h, i < j, k < l, k < m
fn select_944([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_943([a, b, c, g, i, j, k, l, m])
    } else {
        select_314([a, d, b, f, e, g, h, k, l, m])
    }
}
/// n = 15, i = 5, a < f, a < o, b < i, b < m, b < n, c < f, c < g, c < j, d < e, e < g, e < i, e < j, f < h, i < l, j < k, j < m, j < n, j < o
fn select_945([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < a {
        select_761([b, c, g, i, j, k, l, a, f, o, m, n])
    } else {
        select_944([b, a, c, d, e, f, g, h, i, l, j, k, o])
    }
}
/// n = 9, i = 4, a < f, b < e, c < f, c < g, c < h, d < f, d < g, d < h, e < g, e < h, f < i
fn select_946([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_106([a, b, d, e, f, g, h, i])
    } else {
        select_106([a, b, c, e, f, g, h, i])
    }
}
/// n = 9, i = 4, a < g, b < e, c < f, c < g, c < i, d < f, d < g, d < i, e < i, g < h
fn select_947([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_946([a, b, c, d, e, g, f, i, h])
    } else {
        select_182([b, f, a, g, h, e])
    }
}
/// n = 10, i = 5, a < g, b < e, c < f, d < g, d < h, d < j, e < j, f < h, f < j, g < i
fn select_948([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_947([a, b, d, f, e, h, g, i, j])
    } else {
        select_98([b, c, g, i, e, f, h])
    }
}
/// n = 9, i = 4, a < e, b < f, c < f, c < i, d < e, d < g, d < h, e < i, f < g, f < h
fn select_949([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_146([b, c, d, e, f, i])
    } else {
        select_4([a, d, g, h])
    }
}
/// n = 10, i = 5, a < h, b < g, b < h, b < i, c < f, c < j, d < e, d < h, e < i, f < g, f < i, h < j
fn select_950([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_358([a, f, d, b, g, e, h, i, j])
    } else {
        select_949([a, e, c, b, h, f, g, i, j])
    }
}
/// n = 7, i = 3, a < d, a < g, b < c, b < g, c < e, d < f
fn select_951([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_61([a, c, e, g, d])
    } else {
        select_61([b, d, f, g, c])
    }
}
/// n = 8, i = 3, a < g, a < h, b < f, b < g, c < d, c < f, d < e, d < h
fn select_952([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_951([b, c, d, a, e, g, f])
    } else {
        select_421([a, c, d, b, e, g, f, h])
    }
}
/// n = 8, i = 3, a < h, b < d, b < g, c < e, c < g, e < f, e < h
fn select_953([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_951([b, c, e, d, f, a, g])
    } else {
        select_952([a, b, c, e, f, g, d, h])
    }
}
/// n = 10, i = 5, b < g, b < h, b < j, c < f, c < i, d < e, d < g, e < j, f < h, f < j, g < i
fn select_954([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_950([a, b, c, d, e, f, h, g, j, i])
    } else {
        select_953([e, g, c, a, f, h, i, j])
    }
}
/// n = 13, i = 6, a < i, b < e, b < h, c < g, d < f, d < l, e < m, f < i, f < j, f < m, g < h, g < j, g < m, h < l, i < k
fn select_955([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < h {
        select_948([a, b, c, f, e, g, i, j, k, m])
    } else {
        select_954([a, g, d, b, e, f, h, j, l, m])
    }
}
/// n = 16, i = 6, a < i, a < p, b < e, b < h, b < o, c < g, d < f, d < n, f < i, f < j, f < l, g < h, g < j, g < l, h < n, i < k, l < m, l < o, l < p
fn select_956([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < e {
        select_945([a, b, f, c, g, i, j, k, h, l, m, n, e, o, p])
    } else {
        select_955([a, b, c, d, e, f, g, h, i, j, k, n, l])
    }
}
/// n = 17, i = 6, a < i, a < o, b < e, b < h, b < p, c < f, c < j, d < g, d < n, f < h, f < l, g < i, g < j, g < l, h < n, i < m, j < k, j < o, j < p, p < q
fn select_957([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < f {
        select_942([a, b, c, e, i, j, k, m, f, l, o, p, q, h])
    } else {
        select_956([a, b, c, d, e, g, f, h, i, l, m, j, k, n, p, o])
    }
}
/// n = 17, i = 6, a < j, a < o, b < e, b < g, b < p, c < f, c < k, d < h, d < i, f < m, g < i, h < j, h < k, h < m, j < n, k < l, k < o, k < p, p < q
fn select_958([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if g < f {
        select_929([a, c, b, d, e, g, h, i, f, j, k, l, n, o, p, q, m])
    } else {
        select_957([a, b, c, d, e, f, h, g, j, k, l, m, n, i, o, p, q])
    }
}
/// n = 17, i = 6, a < f, a < k, b < e, b < h, b < p, c < g, c < l, d < i, d < j, g < n, h < j, i < k, i < l, i < n, k < o, l < m, l < p, p < q
fn select_959([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if f < l {
        select_900([b, c, a, d, e, g, h, f, i, j, k, o, n, l, m])
    } else {
        select_958([a, b, c, d, e, g, h, i, j, k, l, m, n, o, f, p, q])
    }
}
/// n = 17, i = 6, a < g, a < l, b < f, b < m, c < e, c < h, c < p, d < i, d < k, h < k, i < j, i < l, i < m, l < o, m < n, m < p, p < q
fn select_960([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < f {
        select_830([a, b, c, d, e, g, h, i, j, k, l, m, n, f, o, p, q])
    } else {
        select_959([a, c, b, d, e, g, f, h, i, k, l, m, n, j, o, p, q])
    }
}
/// n = 17, i = 6, a < k, b < h, b < p, c < g, c < l, d < f, d < m, e < i, e < k, i < j, i < l, i < m, l < o, m < n, m < p, p < q
fn select_961([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_734([c, a, d, e, f, g, b, i, j, h, k, l, m, n, o, p])
    } else {
        select_960([c, d, b, e, h, f, g, a, i, j, k, l, m, n, o, p, q])
    }
}
/// n = 17, i = 6, b < i, b < p, c < h, c < l, d < g, d < m, e < f, e < j, j < k, j < l, j < m, l < o, m < n, m < p, p < q
fn select_962([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if f < a {
        select_595([b, c, d, e, g, h, i, f, j, k, a, l, m, n, o, p, q])
    } else {
        select_961([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o, p, q])
    }
}
/// n = 17, i = 6, b < i, b < o, c < h, c < n, d < g, d < j, e < f, e < k, j < m, j < o, k < l, k < n, n < p, o < q
fn select_963([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < k {
        select_962([a, c, b, e, d, g, f, i, h, j, m, o, k, l, q, n, p])
    } else {
        select_962([a, b, c, d, e, f, g, h, i, k, l, n, j, m, p, o, q])
    }
}
/// n = 17, i = 6, b < i, b < p, c < h, c < k, d < g, d < j, e < f, e < l, j < n, k < o, l < m, l < p, p < q
fn select_964([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < k {
        select_963([a, b, c, e, d, g, f, h, i, l, j, n, m, k, p, o, q])
    } else {
        select_963([a, b, d, e, c, h, f, g, i, l, k, o, m, j, p, n, q])
    }
}
/// n = 17, i = 6, b < k, c < j, d < i, d < p, e < h, e < l, f < g, f < m, l < o, m < n, m < p, p < q
fn select_965([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if b < c {
        select_964([a, d, b, e, f, g, h, k, i, l, c, m, n, o, j, p, q])
    } else {
        select_964([a, d, c, e, f, g, h, j, i, l, b, m, n, o, k, p, q])
    }
}
/// n = 17, i = 6, d < k, e < j, e < p, f < i, f < l, g < h, g < m, l < o, m < n, m < p, p < q
fn select_966([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_965([c, a, d, e, f, g, h, i, j, k, b, l, m, n, o, p, q])
    } else {
        select_965([c, b, d, e, f, g, h, i, j, k, a, l, m, n, o, p, q])
    }
}
/// n = 17, i = 6, f < k, f < p, g < j, g < l, h < i, h < m, l < o, m < n, m < p, p < q
fn select_967([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_966([c, d, e, a, f, g, h, i, j, k, b, l, m, n, o, p, q])
    } else {
        select_966([c, d, e, b, f, g, h, i, j, k, a, l, m, n, o, p, q])
    }
}
/// n = 17, i = 6, f < m, g < l, h < k, h < p, i < j, i < n, n < o, n < p, p < q
fn select_968([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if f < g {
        select_967([a, b, c, d, e, h, f, i, j, m, k, g, n, o, l, p, q])
    } else {
        select_967([a, b, c, d, e, h, g, i, j, l, k, f, n, o, m, p, q])
    }
}
/// n = 17, i = 6, h < m, i < l, i < p, j < k, j < n, n < o, n < p, p < q
fn select_969([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_968([c, d, e, f, g, a, h, i, j, k, l, m, b, n, o, p, q])
    } else {
        select_968([c, d, e, f, g, b, h, i, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 6, h < m, i < l, i < o, j < k, j < n, n < p, o < q
fn select_970([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if n < o {
        select_969([a, b, c, d, e, f, g, h, i, j, k, l, m, n, p, o, q])
    } else {
        select_969([a, b, c, d, e, f, g, h, j, i, l, k, m, o, q, n, p])
    }
}
/// n = 17, i = 6, h < o, i < n, j < m, k < l, k < p, p < q
fn select_971([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if h < j {
        select_970([a, b, c, d, e, f, g, i, h, k, l, o, n, p, j, q, m])
    } else {
        select_970([a, b, c, d, e, f, g, i, j, k, l, m, n, p, h, q, o])
    }
}
/// n = 17, i = 6, j < o, k < n, l < m, l < p, p < q
fn select_972([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_971([c, d, e, f, g, h, i, a, j, k, l, m, n, o, b, p, q])
    } else {
        select_971([c, d, e, f, g, h, i, b, j, k, l, m, n, o, a, p, q])
    }
}
/// n = 17, i = 6, l < o, m < n, m < p, p < q
fn select_973([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_972([c, d, e, f, g, h, i, j, k, a, l, m, n, o, b, p, q])
    } else {
        select_972([c, d, e, f, g, h, i, j, k, b, l, m, n, o, a, p, q])
    }
}
/// n = 17, i = 6, n < o, n < p, p < q
fn select_974([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_973([c, d, e, f, g, h, i, j, k, l, m, a, n, o, b, p, q])
    } else {
        select_973([c, d, e, f, g, h, i, j, k, l, m, b, n, o, a, p, q])
    }
}
/// n = 17, i = 6, n < q, o < p
fn select_975([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if n < o {
        select_974([a, b, c, d, e, f, g, h, i, j, k, l, m, n, q, o, p])
    } else {
        select_974([a, b, c, d, e, f, g, h, i, j, k, l, m, o, p, n, q])
    }
}
/// n = 17, i = 6, p < q
fn select_976([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_975([c, d, e, f, g, h, i, j, k, l, m, n, o, a, p, q, b])
    } else {
        select_975([c, d, e, f, g, h, i, j, k, l, m, n, o, b, p, q, a])
    }
}
/// n = 17, i = 6
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_976([c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, a, b])
    } else {
        select_976([c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, b, a])
    }
}
