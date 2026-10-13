#[allow(unused)]
pub const N: usize = 17;
#[allow(unused)]
pub const I: usize = 4;

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
/// n = 3, i = 0
fn select_3([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_2([a, c])
    } else {
        select_2([b, c])
    }
}
/// n = 4, i = 0
fn select_4([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_3([a, c, d])
    } else {
        select_3([b, c, d])
    }
}
/// n = 6, i = 1, a < f, b < c, b < d, b < e
fn select_5([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_2([b, f])
    } else {
        select_4([a, c, d, e])
    }
}
/// n = 7, i = 1, a < f, a < g, b < c, b < d, b < e
fn select_6([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_3([b, f, g])
    } else {
        select_4([a, c, d, e])
    }
}
/// n = 7, i = 1, b < g, c < d, c < e, c < f
fn select_7([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_5([a, c, d, e, f, b])
    } else {
        select_6([b, c, d, e, f, a, g])
    }
}
/// n = 8, i = 1, a < f, a < g, a < h, b < c, b < d, b < e
fn select_8([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_4([b, f, g, h])
    } else {
        select_4([a, c, d, e])
    }
}
/// n = 8, i = 1, b < g, b < h, c < d, c < e, c < f
fn select_9([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_5([a, c, d, e, f, b])
    } else {
        select_8([b, c, d, e, f, a, g, h])
    }
}
/// n = 8, i = 1, c < h, d < e, d < f, d < g
fn select_10([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_7([b, a, d, e, f, g, c])
    } else {
        select_9([b, c, d, e, f, g, a, h])
    }
}
/// n = 4, i = 1, a < d, b < c
fn select_11([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_2([b, d])
    } else {
        select_2([a, c])
    }
}
/// n = 6, i = 1, b < f, c < d, c < e
fn select_12([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < c {
        select_11([a, b, f, c])
    } else {
        select_5([b, c, a, d, e, f])
    }
}
/// n = 3, i = 1, b < c
fn select_13([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_2([a, c])
    }
}
/// n = 5, i = 2, a < d, a < e, b < c, b < e
fn select_14([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_13([c, a, e])
    } else {
        select_13([d, b, e])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < e, c < f
fn select_15([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_14([a, c, d, b, f])
    } else {
        select_14([b, c, d, a, e])
    }
}
/// n = 7, i = 2, b < g, c < d, c < e, c < g, e < f
fn select_16([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_12([d, b, e, a, f, g])
    } else {
        select_15([a, b, c, d, g, e])
    }
}
/// n = 9, i = 2, c < d, c < e, c < f, e < i, f < g, f < h
fn select_17([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_10([b, d, e, f, a, g, h, i])
    } else {
        select_16([b, a, c, d, e, i, f])
    }
}
/// n = 9, i = 1, a < i, b < g, b < h, c < d, c < e, c < f
fn select_18([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_6([a, c, d, e, f, b, i])
    } else {
        select_8([b, c, d, e, f, a, g, h])
    }
}
/// n = 4, i = 1, b < c, b < d
fn select_19([a, b, c, d]: [usize; 4]) -> usize {
    if c < a {
        select_2([c, d])
    } else {
        select_13([a, b, d])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, d < e, d < f, d < g
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_4([a, e, f, g])
    } else {
        select_19([c, b, d, h])
    }
}
/// n = 9, i = 2, a < i, b < h, c < d, c < h, c < i, d < e, d < f, d < g
fn select_21([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_20([a, c, b, d, e, f, g, i])
    } else {
        select_20([b, c, a, d, e, f, g, h])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < j, d < i, e < f, e < g, e < h
fn select_22([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_18([b, d, e, f, g, h, a, i, j])
    } else {
        select_21([a, b, c, e, f, g, h, j, d])
    }
}
/// n = 12, i = 2, b < l, c < d, c < e, c < f, c < l, d < k, e < i, e < j, f < g, f < h
fn select_23([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_22([a, b, c, d, e, f, i, j, k, l])
    } else {
        select_22([a, b, c, d, f, e, g, h, k, l])
    }
}
/// n = 13, i = 2, b < m, c < e, c < g, d < f, d < h, d < m, f < k, g < l, h < i, h < j
fn select_24([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < d {
        select_17([a, b, c, e, g, d, f, h, l])
    } else {
        select_23([a, b, d, f, c, h, i, j, e, g, k, m])
    }
}
/// n = 6, i = 2, a < c, b < d, d < e, d < f
fn select_25([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_3([a, e, f])
    } else {
        select_13([c, b, d])
    }
}
/// n = 6, i = 2, a < d, b < e, b < f, c < e, c < f
fn select_26([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < c {
        select_25([a, b, d, c, e, f])
    } else {
        select_25([a, c, d, b, e, f])
    }
}
/// n = 7, i = 2, a < g, b < d, c < e, c < g, e < f
fn select_27([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_26([b, a, c, d, e, g])
    } else {
        select_25([b, c, d, e, a, f])
    }
}
/// n = 9, i = 2, a < c, b < d, b < e, d < i, e < f, e < g, e < h
fn select_28([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_6([d, e, f, g, h, a, i])
    } else {
        select_20([a, b, c, e, f, g, h, d])
    }
}
/// n = 9, i = 2, b < d, c < e, c < f, e < i, f < g, f < h
fn select_29([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_27([a, b, c, d, e, i, f])
    } else {
        select_28([b, c, d, e, f, a, g, h, i])
    }
}
/// n = 2, i = 1
fn select_30([a, b]: [usize; 2]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_1([a])
    }
}
/// n = 4, i = 2, a < d, b < c
fn select_31([a, b, c, d]: [usize; 4]) -> usize {
    if c < d {
        select_30([a, c])
    } else {
        select_30([b, d])
    }
}
/// n = 5, i = 2, b < e, c < d, c < e
fn select_32([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_31([a, c, d, b])
    } else {
        select_14([b, c, d, a, e])
    }
}
/// n = 6, i = 2, a < c, a < f, b < d, b < f, d < e
fn select_33([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_13([c, b, f])
    } else {
        select_11([a, d, e, f])
    }
}
/// n = 7, i = 2, a < g, b < f, c < d, c < f, c < g, d < e
fn select_34([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_33([a, c, b, d, e, g])
    } else {
        select_33([b, c, a, d, e, f])
    }
}
/// n = 8, i = 3, a < h, b < e, c < f, c < h, d < e, d < f, d < h, f < g
fn select_35([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_32([a, c, d, e, f])
    } else {
        select_34([a, b, d, f, g, e, h])
    }
}
/// n = 10, i = 3, b < e, c < f, c < g, d < e, d < f, d < g, f < j, g < h, g < i
fn select_36([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_28([b, c, e, f, g, a, h, i, j])
    } else {
        select_35([a, b, c, d, e, f, j, g])
    }
}
/// n = 10, i = 3, c < f, c < g, d < e, d < f, d < g, f < j, g < h, g < i
fn select_37([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_29([b, e, c, a, f, g, h, i, j])
    } else {
        select_36([b, a, c, d, e, f, g, h, i, j])
    }
}
/// n = 14, i = 3, b < n, c < e, c < h, d < f, d < h, f < g, f < i, f < n, g < l, h < m, i < j, i < k
fn select_38([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < c {
        select_23([a, b, f, g, c, i, j, k, e, h, l, n])
    } else {
        select_37([a, b, d, c, e, h, f, g, i, m])
    }
}
/// n = 14, i = 3, b < n, c < e, d < f, d < g, f < h, g < i, g < j, g < n, i < m, j < k, j < l
fn select_39([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < c {
        select_24([a, b, f, g, h, i, c, j, k, l, m, e, n])
    } else {
        select_38([a, b, c, d, e, g, i, f, j, k, l, m, h, n])
    }
}
/// n = 5, i = 2, a < e, b < c, b < d
fn select_40([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < a {
        select_13([d, c, a])
    } else {
        select_14([a, b, d, e, c])
    }
}
/// n = 7, i = 2, a < c, a < f, a < g, b < d, b < f, b < g, d < e
fn select_41([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_33([a, b, c, d, e, f])
    } else {
        select_33([a, b, c, d, e, g])
    }
}
/// n = 7, i = 2, a < e, b < f, b < g, c < d, c < f, c < g
fn select_42([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_40([c, a, b, e, d])
    } else {
        select_41([c, b, d, a, e, f, g])
    }
}
/// n = 3, i = 2
fn select_43([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_30([b, c])
    } else {
        select_30([a, c])
    }
}
/// n = 6, i = 3, a < d, b < e, c < e, e < f
fn select_44([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < d {
        select_13([a, e, f])
    } else {
        select_43([b, c, d])
    }
}
/// n = 6, i = 3, a < d, b < e, b < f, c < e, c < f
fn select_45([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_44([a, b, c, d, e, f])
    } else {
        select_44([a, b, c, d, f, e])
    }
}
/// n = 8, i = 3, a < f, b < d, b < g, b < h, c < e, c < f, e < g, e < h
fn select_46([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_45([b, a, c, d, e, f])
    } else {
        select_41([b, e, d, a, f, g, h])
    }
}
/// n = 8, i = 3, b < e, b < g, b < h, c < d, c < f, f < g, f < h
fn select_47([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_42([d, f, b, e, a, g, h])
    } else {
        select_46([a, b, c, e, f, d, g, h])
    }
}
/// n = 7, i = 2, a < c, b < d, d < e, d < f, d < g
fn select_48([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_4([a, e, f, g])
    } else {
        select_13([c, b, d])
    }
}
/// n = 8, i = 2, b < h, c < d, c < h, d < e, d < f, d < g
fn select_49([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_48([a, c, b, d, e, f, g])
    } else {
        select_20([b, c, a, d, e, f, g, h])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_50([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_49([a, d, b, f, g, h, i, e])
    } else {
        select_49([a, e, c, f, g, h, i, d])
    }
}
/// n = 10, i = 3, b < e, b < g, b < j, c < d, c < f, c < g, f < j, g < h, g < i
fn select_51([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_47([a, b, c, d, e, f, g, j])
    } else {
        select_50([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 14, i = 3, b < d, b < f, c < e, c < g, c < n, e < m, f < n, g < h, g < i, g < m, h < l, i < j, i < k
fn select_52([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_23([a, e, g, h, b, i, j, k, d, f, l, m])
    } else {
        select_51([a, c, b, d, e, f, g, h, i, n])
    }
}
/// n = 10, i = 2, a < c, b < d, b < e, d < i, d < j, e < f, e < g, e < h
fn select_53([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_8([d, e, f, g, h, a, i, j])
    } else {
        select_20([a, b, c, e, f, g, h, d])
    }
}
/// n = 10, i = 2, a < d, b < c, b < e, b < f, e < j, f < g, f < h, f < i
fn select_54([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < e {
        select_28([a, b, d, c, f, g, h, i, e])
    } else {
        select_53([a, b, d, e, f, g, h, i, c, j])
    }
}
/// n = 6, i = 2, a < d, b < c, b < e, e < f
fn select_55([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_11([c, e, f, a])
    } else {
        select_33([a, b, d, e, f, c])
    }
}
/// n = 7, i = 2, a < d, a < e, b < c, b < f, f < g
fn select_56([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_40([b, a, d, e, c])
    } else {
        select_55([a, b, c, e, f, g])
    }
}
/// n = 5, i = 1, a < e, b < c, b < d
fn select_57([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_2([b, e])
    } else {
        select_3([a, c, d])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, c < g, d < e, d < f
fn select_58([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_3([a, e, f])
    } else {
        select_57([c, b, d, h, g])
    }
}
/// n = 5, i = 2, a < e, b < d, c < d, c < e
fn select_59([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_13([b, c, e])
    } else {
        select_13([a, c, d])
    }
}
/// n = 8, i = 3, a < e, a < h, b < d, b < h, c < e, c < g, c < h, d < g, e < f
fn select_60([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_11([b, e, f, h])
    } else {
        select_59([a, d, c, g, h])
    }
}
/// n = 9, i = 3, a < f, a < i, b < d, b < e, b < i, c < e, c < f, c < i, e < g, f < h
fn select_61([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_58([a, b, f, e, d, g, h, i])
    } else {
        select_60([a, b, c, d, f, h, e, i])
    }
}
/// n = 9, i = 3, a < i, b < d, b < g, b < i, c < e, c < g, c < i, e < f, g < h
fn select_62([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_56([e, b, d, a, f, g, h])
    } else {
        select_61([a, b, c, d, g, e, h, f, i])
    }
}
/// n = 11, i = 3, b < d, b < g, b < h, c < e, c < g, c < h, e < f, g < k, h < i, h < j
fn select_63([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_54([e, b, d, f, g, h, a, i, j, k])
    } else {
        select_62([a, b, c, d, e, f, g, k, h])
    }
}
/// n = 15, i = 3, b < e, b < i, c < d, c < f, c < i, d < n, e < g, f < h, f < j, f < n, h < m, i < o, j < k, j < l
fn select_64([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < b {
        select_23([a, d, f, h, b, j, k, l, e, i, m, n])
    } else {
        select_63([a, c, b, d, e, g, i, f, h, j, o])
    }
}
/// n = 15, i = 3, b < e, c < i, d < f, d < g, d < i, f < n, g < h, g < j, g < n, h < m, i < o, j < k, j < l
fn select_65([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if b < c {
        select_52([a, b, d, e, f, c, g, h, j, k, l, m, n, i])
    } else {
        select_64([a, c, d, f, b, g, e, h, i, j, k, l, m, n, o])
    }
}
/// n = 15, i = 3, b < f, c < j, d < h, d < k, d < n, e < g, e < j, e < k, g < n, h < i, j < o, k < l, k < m
fn select_66([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if d < e {
        select_39([a, c, b, d, f, h, e, i, g, k, l, m, n, j])
    } else {
        select_65([a, b, c, e, f, g, d, h, j, k, l, m, i, n, o])
    }
}
/// n = 7, i = 1, a < g, b < f, c < d, c < e
fn select_67([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_57([b, a, c, g, f])
    } else {
        select_5([b, c, a, d, e, f])
    }
}
/// n = 7, i = 1, c < g, d < e, d < f
fn select_68([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_67([a, c, d, e, f, g, b])
    } else {
        select_67([b, c, d, e, f, g, a])
    }
}
/// n = 9, i = 1, b < i, c < g, c < h, d < e, d < f
fn select_69([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_7([a, b, c, d, g, h, i])
    } else {
        select_7([a, b, d, c, e, f, i])
    }
}
/// n = 11, i = 2, b < d, b < f, c < e, c < g, e < j, f < k, g < h, g < i
fn select_70([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_68([a, d, f, c, e, g, k])
    } else {
        select_69([a, e, b, g, h, i, d, f, j])
    }
}
/// n = 10, i = 1, a < i, a < j, b < g, b < h, c < d, c < e, c < f
fn select_71([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_8([a, c, d, e, f, b, i, j])
    } else {
        select_8([b, c, d, e, f, a, g, h])
    }
}
/// n = 10, i = 1, b < j, c < h, c < i, d < e, d < f, d < g
fn select_72([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_18([a, c, d, e, f, g, h, i, b])
    } else {
        select_71([b, c, d, e, f, g, h, i, a, j])
    }
}
/// n = 9, i = 2, c < d, c < e, d < i, e < f, e < g, e < h
fn select_73([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_28([a, c, b, d, e, f, g, h, i])
    } else {
        select_28([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 13, i = 3, b < d, b < g, b < m, c < e, c < g, e < f, e < h, e < m, f < k, g < l, h < i, h < j
fn select_74([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < b {
        select_72([a, f, h, b, d, g, m, i, j, k])
    } else {
        select_73([a, d, c, g, e, f, h, m, l])
    }
}
/// n = 13, i = 3, b < d, b < m, c < e, c < f, e < g, f < h, f < i, f < m, h < l, i < j, i < k
fn select_75([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < b {
        select_70([a, e, f, g, h, b, i, j, k, l, d])
    } else {
        select_74([a, b, c, d, f, h, e, i, j, k, l, g, m])
    }
}
/// n = 6, i = 2, a < f, b < d, b < e, c < d, c < e, c < f
fn select_76([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_2([d, e])
    } else {
        select_59([a, b, c, e, f])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < g, c < h, d < e, d < f
fn select_77([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_6([b, d, a, e, f, g, h])
    } else {
        select_76([a, b, c, g, h, d])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < j, d < h, d < i, e < f, e < g
fn select_78([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < b {
        select_9([a, e, d, b, h, i, f, g])
    } else {
        select_77([a, b, c, e, f, g, d, j])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < j, f < k, g < h, g < i
fn select_79([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_78([a, d, b, f, g, h, i, j, k, e])
    } else {
        select_78([a, e, c, f, g, h, i, j, k, d])
    }
}
/// n = 15, i = 3, b < d, b < i, c < e, c < f, c < i, e < m, f < g, f < h, f < m, g < l, h < j, h < k, i < n, i < o
fn select_80([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < b {
        select_23([a, e, f, g, b, h, j, k, d, i, l, m])
    } else {
        select_79([a, b, c, e, d, f, i, n, o, g, h])
    }
}
/// n = 15, i = 3, b < e, b < j, c < g, c < i, c < m, d < f, d < i, d < j, f < m, g < h, i < k, i < l, j < n, j < o
fn select_81([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if c < d {
        select_75([a, b, c, e, g, d, h, f, i, k, l, m, j])
    } else {
        select_80([a, b, d, e, f, c, g, i, j, k, l, h, m, n, o])
    }
}
/// n = 6, i = 2, a < e, a < f, b < c, b < d, b < f
fn select_82([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_33([a, b, e, c, d, f])
    } else {
        select_33([a, b, e, d, c, f])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, c < d, c < f, c < g
fn select_83([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_82([c, b, a, e, d, f])
    } else {
        select_33([c, a, d, b, e, g])
    }
}
/// n = 5, i = 1, b < c, b < d, b < e
fn select_84([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_4([a, c, d, e])
    }
}
/// n = 6, i = 2, a < d, a < e, a < f, b < c, b < e, b < f
fn select_85([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_19([c, a, e, f])
    } else {
        select_19([d, b, e, f])
    }
}
/// n = 8, i = 2, a < d, a < h, b < c, b < e, b < h, e < f, e < g
fn select_86([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_84([c, e, a, f, g])
    } else {
        select_85([a, b, c, d, e, h])
    }
}
/// n = 8, i = 2, b < e, b < h, c < d, c < f, c < h, f < g
fn select_87([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_83([a, b, c, d, e, h, f])
    } else {
        select_86([b, c, d, e, f, a, g, h])
    }
}
/// n = 10, i = 2, b < e, c < d, c < f, c < g, f < j, g < h, g < i
fn select_88([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_10([a, d, f, g, b, h, i, j])
    } else {
        select_87([a, b, c, d, e, f, j, g])
    }
}
/// n = 4, i = 2, c < d
fn select_89([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_31([a, c, d, b])
    } else {
        select_31([b, c, d, a])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < e
fn select_90([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < f {
        select_89([a, d, b, e])
    } else {
        select_31([a, c, e, f])
    }
}
/// n = 8, i = 3, a < d, a < f, b < e, b < g, c < e, c < f, c < g, g < h
fn select_91([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_90([b, c, a, d, f, e])
    } else {
        select_55([a, b, e, f, g, h])
    }
}
/// n = 5, i = 1, b < e, c < d
fn select_92([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < c {
        select_19([a, b, c, e])
    } else {
        select_19([a, c, b, d])
    }
}
/// n = 7, i = 3, a < e, a < g, b < e, b < f, b < g, c < d, c < g, d < f
fn select_93([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_13([a, b, f])
    } else {
        select_14([b, c, d, e, g])
    }
}
/// n = 9, i = 3, a < d, a < i, b < e, b < f, b < i, c < e, c < f, c < h, c < i, d < h, f < g
fn select_94([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_92([e, a, f, g, i])
    } else {
        select_93([b, c, a, d, e, h, i])
    }
}
/// n = 9, i = 3, a < i, b < f, c < e, c < g, c < i, d < e, d < f, d < g, d < i, g < h
fn select_95([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_91([b, c, d, a, e, f, g, h])
    } else {
        select_94([a, c, d, b, e, g, h, f, i])
    }
}
/// n = 11, i = 3, b < f, c < e, c < g, c < h, d < e, d < f, d < g, d < h, g < k, h < i, h < j
fn select_96([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_54([b, c, e, f, g, h, a, i, j, k])
    } else {
        select_95([a, b, c, d, e, f, g, k, h])
    }
}
/// n = 11, i = 3, c < f, c < g, c < h, d < e, d < f, d < g, d < h, g < k, h < i, h < j
fn select_97([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_88([b, e, c, f, a, g, h, i, j, k])
    } else {
        select_96([b, a, c, d, f, e, g, h, i, j, k])
    }
}
/// n = 5, i = 0
fn select_98([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_4([a, c, d, e])
    } else {
        select_4([b, c, d, e])
    }
}
/// n = 7, i = 1, a < g, b < c, b < d, b < e, b < f
fn select_99([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_2([b, g])
    } else {
        select_98([a, c, d, e, f])
    }
}
/// n = 10, i = 1, a < g, a < h, a < i, a < j, b < c, b < d, b < e, b < f
fn select_100([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_98([b, g, h, i, j])
    } else {
        select_98([a, c, d, e, f])
    }
}
/// n = 10, i = 1, b < h, b < i, b < j, c < d, c < e, c < f, c < g
fn select_101([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_99([a, c, d, e, f, g, b])
    } else {
        select_100([b, c, d, e, f, g, a, h, i, j])
    }
}
/// n = 8, i = 2, a < c, b < d, d < e, d < f, d < g, d < h
fn select_102([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_98([a, e, f, g, h])
    } else {
        select_13([c, b, d])
    }
}
/// n = 10, i = 2, a < c, a < i, a < j, b < d, b < i, b < j, d < e, d < f, d < g, d < h
fn select_103([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_98([a, e, f, g, h])
    } else {
        select_84([c, b, d, i, j])
    }
}
/// n = 10, i = 2, b < i, b < j, c < d, c < i, c < j, d < e, d < f, d < g, d < h
fn select_104([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_102([a, c, b, d, e, f, g, h])
    } else {
        select_103([b, c, a, d, e, f, g, h, i, j])
    }
}
/// n = 12, i = 2, b < l, c < d, c < e, c < l, d < j, d < k, e < f, e < g, e < h, e < i
fn select_105([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < b {
        select_101([a, d, e, f, g, h, i, b, j, k])
    } else {
        select_104([a, b, c, e, f, g, h, i, d, l])
    }
}
/// n = 4, i = 1, c < d
fn select_106([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_11([a, c, d, b])
    } else {
        select_11([b, c, d, a])
    }
}
/// n = 6, i = 2, a < f, b < e, b < f, c < d, c < e
fn select_107([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < c {
        select_11([b, c, d, f])
    } else {
        select_106([a, d, b, e])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, b < g, c < d, c < e, c < f
fn select_108([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_107([a, b, c, d, e, g])
    } else {
        select_107([a, b, c, d, f, g])
    }
}
/// n = 9, i = 2, a < c, a < h, a < i, b < d, b < h, b < i, d < e, d < f, d < g
fn select_109([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < b {
        select_1([b])
    } else {
        select_8([a, d, e, f, g, c, h, i])
    }
}
/// n = 9, i = 2, b < d, b < h, b < i, c < e, c < h, c < i, e < f, e < g
fn select_110([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_108([a, c, b, d, h, i, e])
    } else {
        select_109([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 11, i = 3, b < d, b < g, b < j, c < e, c < g, c < k, d < k, e < f, e < j, g < h, g < i
fn select_111([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_77([a, d, c, g, h, i, e, k])
    } else {
        select_110([a, e, b, f, g, h, i, d, j])
    }
}
/// n = 14, i = 3, b < e, b < g, b < h, c < d, c < g, c < h, c < n, d < f, e < n, g < l, g < m, h < i, h < j, h < k
fn select_112([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < d {
        select_105([a, e, c, g, h, d, i, j, k, l, m, n])
    } else {
        select_111([a, b, c, e, d, f, g, l, m, h, n])
    }
}
/// n = 16, i = 4, b < f, b < j, b < k, c < d, c < e, c < l, d < p, e < g, e < j, e < k, f < h, f < l, f < p, h < i, k < o, l < m, l < n
fn select_113([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if e < f {
        select_97([a, d, b, e, g, j, k, f, h, l, o])
    } else {
        select_112([a, c, f, h, d, i, l, e, g, j, k, m, n, p])
    }
}
/// n = 16, i = 4, b < e, b < f, b < m, c < d, c < g, c < l, d < h, e < p, f < i, f < l, g < j, g < m, g < p, j < k, m < n, m < o
fn select_114([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if d < f {
        select_81([a, d, g, b, h, e, j, k, m, f, n, o, p, i, l])
    } else {
        select_113([a, c, b, e, f, g, i, j, k, l, d, m, n, o, h, p])
    }
}
/// n = 16, i = 4, b < f, b < g, b < m, c < d, c < e, c < h, e < i, f < p, g < j, h < k, h < m, h < p, k < l, m < n, m < o
fn select_115([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if d < g {
        select_66([a, e, d, h, b, i, f, k, l, g, m, n, o, p, j])
    } else {
        select_114([a, b, c, e, f, g, h, i, j, k, l, d, m, n, o, p])
    }
}
/// n = 7, i = 1, d < e, d < f, d < g
fn select_116([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_7([c, a, d, e, f, g, b])
    } else {
        select_7([c, b, d, e, f, g, a])
    }
}
/// n = 9, i = 1, d < h, d < i, e < f, e < g
fn select_117([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_116([a, b, c, d, e, h, i])
    } else {
        select_116([a, b, c, e, d, f, g])
    }
}
/// n = 5, i = 2, a < c, b < d, d < e
fn select_118([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < a {
        select_2([a, e])
    } else {
        select_13([c, b, d])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < f, c < g, d < e
fn select_119([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_118([a, c, b, d, e])
    } else {
        select_41([b, c, a, d, e, f, g])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < e, c < f, c < g
fn select_120([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_119([a, b, c, d, e, f, g])
    } else {
        select_119([a, b, c, e, d, f, g])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < f, c < i, f < g, f < h
fn select_121([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_116([a, d, e, f, b, g, h])
    } else {
        select_120([a, b, c, d, e, f, i])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, c < g, f < j, g < h, g < i
fn select_122([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_117([b, d, e, f, g, h, i, a, j])
    } else {
        select_121([b, a, c, d, e, g, h, i, f])
    }
}
/// n = 15, i = 3, b < e, b < k, b < o, c < d, c < f, c < k, d < o, e < i, f < g, f < h, f < j, j < l, k < m, k < n
fn select_123([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < b {
        select_122([a, d, f, g, h, j, b, e, k, l])
    } else {
        select_112([a, c, b, e, d, i, k, f, g, h, j, m, n, o])
    }
}
/// n = 15, i = 3, b < e, c < g, c < l, c < o, d < f, d < h, d < l, f < o, g < k, h < i, h < j, l < m, l < n
fn select_124([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < b {
        select_123([a, c, d, f, g, h, i, j, k, b, l, e, m, n, o])
    } else {
        select_81([a, b, c, d, e, f, g, k, l, h, m, n, o, i, j])
    }
}
/// n = 8, i = 3, a < h, b < e, b < f, b < h, c < d, c < f, c < h, f < g
fn select_125([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_34([a, d, b, f, g, e, h])
    } else {
        select_34([a, e, c, f, g, d, h])
    }
}
/// n = 10, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < j, g < h, g < i
fn select_126([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_10([d, e, f, g, a, h, i, j])
    } else {
        select_125([a, b, c, d, e, f, j, g])
    }
}
/// n = 14, i = 3, a < n, b < e, b < h, c < d, c < f, c < h, f < g, f < i, f < n, g < l, h < m, i < j, i < k
fn select_127([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < b {
        select_23([d, a, f, g, b, i, j, k, e, h, l, n])
    } else {
        select_126([a, b, c, d, e, h, f, g, i, m])
    }
}
/// n = 15, i = 4, a < o, b < f, b < i, c < d, c < e, c < i, e < g, f < h, i < j, i < k, i < o, j < n, k < l, k < m
fn select_128([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < e {
        select_127([a, f, c, d, h, i, j, e, k, l, m, n, g, o])
    } else {
        select_38([d, a, e, b, g, i, j, f, k, l, m, n, h, o])
    }
}
/// n = 16, i = 4, a < k, b < f, b < l, b < o, c < d, c < e, c < g, e < h, f < i, g < j, g < k, g < l, j < o, k < p, l < m, l < n
fn select_129([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < b {
        select_65([d, e, a, g, h, j, b, f, k, l, m, n, i, o, p])
    } else {
        select_128([a, b, c, d, e, f, h, i, g, j, l, m, n, o, k])
    }
}
/// n = 16, i = 4, b < f, b < m, b < p, c < d, c < e, c < g, e < h, f < i, g < j, g < k, g < m, j < p, k < l, m < n, m < o
fn select_130([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < a {
        select_124([d, e, b, g, h, j, f, k, a, l, i, m, n, o, p])
    } else {
        select_129([a, b, c, d, e, f, g, h, i, j, k, m, n, o, p, l])
    }
}
/// n = 16, i = 4, b < e, b < g, b < n, c < d, c < f, c < h, f < j, g < k, h < i, h < l, h < n, l < m, n < o, n < p
fn select_131([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if e < i {
        select_115([a, b, c, d, f, e, g, h, j, k, l, m, n, o, p, i])
    } else {
        select_130([a, b, c, d, f, g, h, j, k, i, l, m, n, o, p, e])
    }
}
/// n = 17, i = 4, b < e, b < g, b < i, c < d, c < f, c < h, f < l, g < m, h < j, h < n, i < k, i < o, n < p, o < q
fn select_132([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if h < i {
        select_131([a, b, c, d, e, f, g, h, j, l, m, n, p, i, k, o])
    } else {
        select_131([a, c, b, e, d, g, f, i, k, m, l, o, q, h, j, n])
    }
}
/// n = 17, i = 4, b < g, b < j, c < f, c < i, d < e, d < h, d < k, h < m, i < n, j < o, k < l, k < p, p < q
fn select_133([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if b < c {
        select_132([a, b, d, e, g, h, j, k, c, l, f, m, o, p, i, q, n])
    } else {
        select_132([a, c, d, e, f, h, i, k, b, l, g, m, n, p, j, q, o])
    }
}
/// n = 17, i = 4, b < i, c < h, d < g, d < k, e < f, e < j, e < l, j < n, k < o, l < m, l < p, p < q
fn select_134([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if b < c {
        select_133([a, b, d, e, f, g, i, j, k, c, l, m, n, o, h, p, q])
    } else {
        select_133([a, c, d, e, f, g, h, j, k, b, l, m, n, o, i, p, q])
    }
}
/// n = 17, i = 4, b < k, c < j, d < i, e < h, f < g, f < l, f < m, l < o, m < n, m < p, p < q
fn select_135([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if d < e {
        select_134([a, b, c, d, f, g, i, j, k, l, e, m, n, o, h, p, q])
    } else {
        select_134([a, b, c, e, f, g, h, j, k, l, d, m, n, o, i, p, q])
    }
}
/// n = 17, i = 4, b < m, c < l, d < k, e < j, f < i, f < o, g < h, g < n, n < p, o < q
fn select_136([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if f < g {
        select_135([a, b, c, d, e, f, i, j, k, l, m, o, g, h, q, n, p])
    } else {
        select_135([a, b, c, d, e, g, h, j, k, l, m, n, f, i, p, o, q])
    }
}
/// n = 17, i = 4, b < o, c < n, d < m, e < l, f < k, g < j, h < i, h < p, p < q
fn select_137([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if c < d {
        select_136([a, b, e, f, g, c, h, i, n, j, k, l, o, p, d, q, m])
    } else {
        select_136([a, b, e, f, g, d, h, i, m, j, k, l, o, p, c, q, n])
    }
}
/// n = 17, i = 4, b < q, c < p, d < o, e < n, f < m, g < l, h < k, i < j
fn select_138([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if f < i {
        select_137([a, b, c, d, e, g, h, f, m, k, l, n, o, p, q, i, j])
    } else {
        select_137([a, b, c, d, e, g, h, i, j, k, l, n, o, p, q, f, m])
    }
}
/// n = 17, i = 4, d < q, e < p, f < o, g < n, h < m, i < l, j < k
fn select_139([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_138([c, a, d, e, f, g, h, i, j, k, l, m, n, o, p, q, b])
    } else {
        select_138([c, b, d, e, f, g, h, i, j, k, l, m, n, o, p, q, a])
    }
}
/// n = 17, i = 4, f < q, g < p, h < o, i < n, j < m, k < l
fn select_140([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_139([c, d, e, a, f, g, h, i, j, k, l, m, n, o, p, q, b])
    } else {
        select_139([c, d, e, b, f, g, h, i, j, k, l, m, n, o, p, q, a])
    }
}
/// n = 17, i = 4, h < q, i < p, j < o, k < n, l < m
fn select_141([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_140([c, d, e, f, g, a, h, i, j, k, l, m, n, o, p, q, b])
    } else {
        select_140([c, d, e, f, g, b, h, i, j, k, l, m, n, o, p, q, a])
    }
}
/// n = 17, i = 4, j < q, k < p, l < o, m < n
fn select_142([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_141([c, d, e, f, g, h, i, a, j, k, l, m, n, o, p, q, b])
    } else {
        select_141([c, d, e, f, g, h, i, b, j, k, l, m, n, o, p, q, a])
    }
}
/// n = 17, i = 4, l < q, m < p, n < o
fn select_143([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_142([c, d, e, f, g, h, i, j, k, a, l, m, n, o, p, q, b])
    } else {
        select_142([c, d, e, f, g, h, i, j, k, b, l, m, n, o, p, q, a])
    }
}
/// n = 17, i = 4, n < q, o < p
fn select_144([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_143([c, d, e, f, g, h, i, j, k, l, m, a, n, o, p, q, b])
    } else {
        select_143([c, d, e, f, g, h, i, j, k, l, m, b, n, o, p, q, a])
    }
}
/// n = 17, i = 4, p < q
fn select_145([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_144([c, d, e, f, g, h, i, j, k, l, m, n, o, a, p, q, b])
    } else {
        select_144([c, d, e, f, g, h, i, j, k, l, m, n, o, b, p, q, a])
    }
}
/// n = 17, i = 4
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_145([c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, a, b])
    } else {
        select_145([c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, b, a])
    }
}
