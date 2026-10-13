#[allow(unused)]
pub const N: usize = 17;
#[allow(unused)]
pub const I: usize = 7;

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
/// n = 5, i = 2, a < c, b < d, d < e
fn select_4([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < a {
        select_2([a, e])
    } else {
        select_3([c, b, d])
    }
}
/// n = 3, i = 0
fn select_5([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_2([a, c])
    } else {
        select_2([b, c])
    }
}
/// n = 6, i = 2, a < c, b < d, d < e, d < f
fn select_6([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_5([a, e, f])
    } else {
        select_3([c, b, d])
    }
}
/// n = 6, i = 2, a < d, b < c, b < e, e < f
fn select_7([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < e {
        select_4([a, b, d, c, e])
    } else {
        select_6([a, b, d, e, c, f])
    }
}
/// n = 2, i = 1
fn select_8([a, b]: [usize; 2]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_1([a])
    }
}
/// n = 5, i = 1, a < e, b < c, b < d
fn select_9([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_2([b, e])
    } else {
        select_5([a, c, d])
    }
}
/// n = 6, i = 2, a < c, a < f, b < d, b < f, d < e
fn select_10([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < b {
        select_1([b])
    } else {
        select_9([d, a, c, f, e])
    }
}
/// n = 7, i = 3, a < d, a < g, b < e, b < g, c < e, c < g, e < f
fn select_11([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < b {
        select_8([b, c])
    } else {
        select_10([a, c, d, e, f, g])
    }
}
/// n = 8, i = 3, a < h, b < d, b < h, c < e, c < h, e < f, f < g
fn select_12([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_7([b, e, a, d, f, g])
    } else {
        select_11([b, a, c, d, e, f, h])
    }
}
/// n = 8, i = 3, a < h, b < d, b < h, c < e, c < h, e < f, e < g
fn select_13([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_12([a, b, c, d, e, f, g, h])
    } else {
        select_12([a, b, c, d, e, g, f, h])
    }
}
/// n = 3, i = 2
fn select_14([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_8([b, c])
    } else {
        select_8([a, c])
    }
}
/// n = 5, i = 3, a < e, b < e, c < d
fn select_15([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_14([a, b, d])
    } else {
        select_8([c, e])
    }
}
/// n = 6, i = 3, a < e, b < e, b < f, c < d, d < f
fn select_16([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < f {
        select_15([a, b, c, d, e])
    } else {
        select_1([f])
    }
}
/// n = 7, i = 3, a < e, b < e, b < f, c < d, c < f, f < g
fn select_17([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_6([a, c, e, f, d, g])
    } else {
        select_16([a, b, c, d, e, f])
    }
}
/// n = 4, i = 2, b < d, c < d
fn select_18([a, b, c, d]: [usize; 4]) -> usize {
    if b < a {
        select_3([a, c, d])
    } else {
        select_8([b, c])
    }
}
/// n = 8, i = 3, a < d, a < h, b < e, b < h, c < e, c < h, e < f, e < g
fn select_19([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_18([d, b, c, h])
    } else {
        select_9([a, e, f, g, h])
    }
}
/// n = 9, i = 4, a < g, a < i, b < d, b < g, c < e, c < i, d < h, d < i, e < f, e < h
fn select_20([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_17([a, b, e, f, g, d, h])
    } else {
        select_19([a, c, d, g, e, f, h, i])
    }
}
/// n = 5, i = 2, a < e, b < d, c < d, c < e
fn select_21([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_3([b, c, e])
    } else {
        select_3([a, c, d])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < g, c < d, d < f, d < g, d < h
fn select_22([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_9([a, b, f, g, h])
    } else {
        select_4([b, c, e, d, h])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, c < d, c < i, d < g, d < h, e < f, e < h, e < i
fn select_23([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_21([a, f, c, d, i])
    } else {
        select_22([a, d, b, e, g, f, h, i])
    }
}
/// n = 10, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < g, d < i, e < f, e < i, h < j
fn select_24([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_6([d, c, g, h, e, j])
    } else {
        select_23([a, c, b, d, e, f, g, i, h])
    }
}
/// n = 10, i = 4, a < g, a < j, b < h, b < j, c < g, c < h, d < e, d < h, e < f, e < j, h < i
fn select_25([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_20([c, a, d, b, e, f, g, j, h])
    } else {
        select_24([c, b, d, a, e, f, g, h, j, i])
    }
}
/// n = 10, i = 4, a < g, b < g, b < j, c < h, c < j, d < e, d < h, e < f, e < j, h < i
fn select_26([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_13([b, e, c, f, h, a, i, j])
    } else {
        select_25([b, c, a, d, e, f, g, h, i, j])
    }
}
/// n = 6, i = 3, a < d, b < e, c < e, e < f
fn select_27([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < d {
        select_3([a, e, f])
    } else {
        select_14([b, c, d])
    }
}
/// n = 4, i = 1, a < d, b < c
fn select_28([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_2([b, d])
    } else {
        select_2([a, c])
    }
}
/// n = 4, i = 1, c < d
fn select_29([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_28([a, c, d, b])
    } else {
        select_28([b, c, d, a])
    }
}
/// n = 8, i = 4, a < e, b < e, b < h, c < f, d < f, f < g, f < h
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_27([b, c, d, e, f, g])
    } else {
        select_29([a, g, b, h])
    }
}
/// n = 5, i = 2, a < d, a < e, b < c, b < e
fn select_31([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_3([c, a, e])
    } else {
        select_3([d, b, e])
    }
}
/// n = 5, i = 2, a < e, b < c, b < d
fn select_32([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < a {
        select_3([d, c, a])
    } else {
        select_31([a, b, d, e, c])
    }
}
/// n = 6, i = 3, a < e, b < c, b < d, e < f
fn select_33([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < c {
        select_32([e, b, c, d, f])
    } else {
        select_3([d, a, e])
    }
}
/// n = 7, i = 3, a < c, a < d, b < e, e < f, e < g
fn select_34([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < c {
        select_32([a, e, f, g, d])
    } else {
        select_33([b, a, c, d, e, g])
    }
}
/// n = 9, i = 3, a < c, a < h, a < i, b < d, c < e, d < f, d < h, d < i, f < g
fn select_35([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < c {
        select_9([f, a, h, i, g])
    } else {
        select_6([c, b, e, d, h, i])
    }
}
/// n = 9, i = 3, a < c, a < h, a < i, b < d, c < g, d < e, d < f, d < h, d < i
fn select_36([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_35([a, b, c, d, g, e, f, h, i])
    } else {
        select_35([a, b, c, d, g, f, e, h, i])
    }
}
/// n = 9, i = 3, a < d, b < h, b < i, c < e, e < f, e < g, e < h, e < i
fn select_37([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_34([a, c, b, d, e, f, g])
    } else {
        select_36([b, c, a, e, f, g, d, h, i])
    }
}
/// n = 11, i = 4, a < e, b < f, b < k, c < e, c < g, d < f, d < g, f < j, g < h, g < i, g < k
fn select_38([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_30([a, c, b, d, e, f, j, g])
    } else {
        select_37([a, b, d, e, g, h, i, f, k])
    }
}
/// n = 12, i = 4, a < g, b < g, b < i, c < e, c < h, d < h, d < i, e < f, h < j, i < k, i < l
fn select_39([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_26([a, b, d, c, e, f, g, h, j, i])
    } else {
        select_38([a, c, b, d, g, h, i, k, l, j, e])
    }
}
/// n = 6, i = 3, a < f, b < e, c < e, c < f, d < e, d < f
fn select_40([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_21([a, b, d, e, f])
    } else {
        select_21([a, b, c, e, f])
    }
}
/// n = 7, i = 3, b < g, c < e, c < g, d < e, d < g, e < f
fn select_41([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_9([b, e, a, f, g])
    } else {
        select_40([a, b, c, d, g, e])
    }
}
/// n = 4, i = 1, b < c, b < d
fn select_42([a, b, c, d]: [usize; 4]) -> usize {
    if c < a {
        select_2([c, d])
    } else {
        select_3([a, b, d])
    }
}
/// n = 5, i = 1, b < e, c < d
fn select_43([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < c {
        select_42([a, b, c, e])
    } else {
        select_42([a, c, b, d])
    }
}
/// n = 7, i = 2, a < g, b < e, c < d, c < g, e < f
fn select_44([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_43([a, c, e, f, d])
    } else {
        select_43([d, a, b, e, g])
    }
}
/// n = 6, i = 2, b < f, c < d, c < f, d < e
fn select_45([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_9([b, d, a, e, f])
    } else {
        select_21([a, b, c, f, d])
    }
}
/// n = 4, i = 2, a < d, b < c
fn select_46([a, b, c, d]: [usize; 4]) -> usize {
    if c < d {
        select_8([a, c])
    } else {
        select_8([b, d])
    }
}
/// n = 5, i = 2, b < e, c < d, c < e
fn select_47([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_46([a, c, d, b])
    } else {
        select_31([b, c, d, a, e])
    }
}
/// n = 7, i = 3, b < e, c < f, d < e, d < f, f < g
fn select_48([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < b {
        select_45([a, b, d, f, g, e])
    } else {
        select_47([a, c, d, e, f])
    }
}
/// n = 8, i = 3, b < h, c < f, d < e, d < f, e < h, f < g
fn select_49([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_44([b, c, e, a, f, g, h])
    } else {
        select_48([b, a, c, d, e, f, g])
    }
}
/// n = 9, i = 3, b < h, b < i, c < f, c < i, d < e, d < f, d < i, e < h, f < g
fn select_50([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if i < e {
        select_41([a, b, c, d, f, g, i])
    } else {
        select_49([a, b, c, d, e, f, g, h])
    }
}
/// n = 6, i = 3, a < f, b < d, c < e, e < f
fn select_51([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_27([b, a, c, d, e, f])
    } else {
        select_4([b, e, d, a, f])
    }
}
/// n = 6, i = 3, b < e, c < d, c < f, e < f
fn select_52([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_31([c, e, a, d, f])
    } else {
        select_15([a, b, c, d, e])
    }
}
/// n = 8, i = 4, a < f, b < e, b < h, c < f, c < g, d < e, d < g, g < h
fn select_53([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_51([b, a, d, f, g, h])
    } else {
        select_52([a, c, b, e, g, h])
    }
}
/// n = 5, i = 3, b < e, c < d
fn select_54([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_18([a, b, d, e])
    } else {
        select_18([a, c, e, d])
    }
}
/// n = 7, i = 4, a < g, b < f, c < e, c < g, d < e, d < f
fn select_55([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_54([a, b, c, e, f])
    } else {
        select_54([b, a, d, e, g])
    }
}
/// n = 9, i = 5, a < g, b < g, b < h, c < f, c < h, d < e, d < h, f < i, h < i
fn select_56([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_53([a, f, b, d, e, g, h, i])
    } else {
        select_55([a, e, b, c, h, f, g])
    }
}
/// n = 11, i = 5, a < g, b < g, b < k, c < f, c < h, d < e, d < h, f < i, f < k, h < i, h < k, i < j
fn select_57([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_50([e, b, f, h, a, i, j, g, k])
    } else {
        select_56([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 8, i = 4, a < e, a < f, b < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_58([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_4([a, d, f, g, h])
    } else {
        select_15([b, c, d, e, f])
    }
}
/// n = 8, i = 4, a < f, b < f, b < g, c < e, c < g, d < e, d < g, g < h
fn select_59([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_58([a, c, d, b, f, e, g, h])
    } else {
        select_4([e, b, a, g, h])
    }
}
/// n = 6, i = 4, a < f, b < f, c < e, d < e
fn select_60([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_14([a, b, e])
    } else {
        select_14([c, d, f])
    }
}
/// n = 9, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, h < i
fn select_61([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_59([a, f, b, c, g, e, h, i])
    } else {
        select_60([b, c, d, e, f, g])
    }
}
/// n = 6, i = 2, a < d, a < e, a < f, b < c, b < e, b < f
fn select_62([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_31([a, b, c, d, e])
    } else {
        select_31([a, b, c, d, f])
    }
}
/// n = 7, i = 2, a < c, a < f, a < g, b < d, b < f, b < g, d < e
fn select_63([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_1([e])
    } else {
        select_62([a, b, d, c, f, g])
    }
}
/// n = 6, i = 1, a < e, a < f, b < c, b < d
fn select_64([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_5([b, e, f])
    } else {
        select_5([a, c, d])
    }
}
/// n = 8, i = 2, a < c, a < g, a < h, b < d, b < g, b < h, d < e, d < f
fn select_65([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_42([c, b, g, h])
    } else {
        select_64([a, d, e, f, g, h])
    }
}
/// n = 8, i = 2, a < d, a < g, a < h, b < c, b < e, b < g, b < h, e < f
fn select_66([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_63([a, b, d, c, e, g, h])
    } else {
        select_65([a, b, d, e, c, f, g, h])
    }
}
/// n = 11, i = 5, a < g, b < e, b < j, b < k, c < g, c < h, d < f, f < h, h < i, h < j, h < k
fn select_67([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_61([b, a, c, d, e, f, g, h, i])
    } else {
        select_66([b, h, i, e, a, g, j, k])
    }
}
/// n = 12, i = 5, a < g, b < e, b < k, b < l, c < g, c < h, d < f, f < h, h < i, h < j, h < k, h < l
fn select_68([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < e {
        select_37([a, b, f, g, h, i, j, k, l])
    } else {
        select_67([a, b, c, d, e, f, g, h, j, k, l])
    }
}
/// n = 14, i = 5, a < g, b < g, b < i, c < e, c < h, c < n, d < f, d < h, f < i, f < l, h < l, i < j, i < k, i < n, l < m
fn select_69([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_57([a, b, d, c, e, f, g, h, l, m, i])
    } else {
        select_68([a, c, b, d, e, f, g, i, j, k, h, n])
    }
}
/// n = 14, i = 5, a < h, b < e, b < n, c < h, c < j, d < f, d < g, f < i, g < i, g < j, i < k, j < l, j < m, j < n
fn select_70([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < b {
        select_39([a, c, f, g, b, e, h, i, j, k, l, m])
    } else {
        select_69([a, c, b, d, e, g, h, f, j, l, m, i, k, n])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < e
fn select_71([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < d {
        select_32([a, c, d, e, f])
    } else {
        select_3([a, b, e])
    }
}
/// n = 7, i = 3, a < f, b < f, b < g, c < d, c < e, e < g
fn select_72([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_71([b, a, c, d, e, f])
    } else {
        select_47([d, b, e, a, g])
    }
}
/// n = 6, i = 2, a < e, a < f, b < c, b < d, b < f
fn select_73([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < f {
        select_32([a, b, c, d, e])
    } else {
        select_31([a, b, d, e, f])
    }
}
/// n = 8, i = 3, a < d, a < g, b < c, b < g, b < h, c < f, d < e, d < h
fn select_74([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_31([a, c, f, g, d])
    } else {
        select_62([b, d, e, g, c, h])
    }
}
/// n = 9, i = 3, a < c, a < h, a < i, b < d, b < h, c < g, d < e, d < f, d < i
fn select_75([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < c {
        select_73([a, d, e, f, h, i])
    } else {
        select_74([b, a, c, d, f, g, h, i])
    }
}
/// n = 5, i = 2, b < d, b < e, c < d, c < e
fn select_76([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_8([b, c])
    } else {
        select_42([a, c, d, e])
    }
}
/// n = 5, i = 1, c < d, c < e
fn select_77([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_9([a, c, d, e, b])
    } else {
        select_9([b, c, d, e, a])
    }
}
/// n = 7, i = 2, a < e, a < f, a < g, b < c, b < d, b < f, b < g
fn select_78([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_77([c, d, a, f, g])
    } else {
        select_62([a, b, d, e, f, g])
    }
}
/// n = 9, i = 3, a < c, a < d, a < h, b < e, b < i, d < i, e < f, e < g, e < h
fn select_79([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_76([c, b, d, e, i])
    } else {
        select_78([a, e, f, g, c, d, h])
    }
}
/// n = 10, i = 3, a < h, a < j, b < d, b < i, c < e, c < h, e < f, e < g, e < i, e < j
fn select_80([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_75([a, c, b, e, f, g, d, h, j])
    } else {
        select_79([b, c, d, a, e, f, g, i, h])
    }
}
/// n = 10, i = 3, a < i, a < j, b < d, b < e, c < f, c < i, f < g, f < h, f < j
fn select_81([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_72([a, c, b, e, d, i, f])
    } else {
        select_80([a, b, c, e, f, g, h, i, d, j])
    }
}
/// n = 7, i = 3, a < e, a < g, b < e, b < f, c < d, d < f, d < g
fn select_82([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_28([a, b, f, g])
    } else {
        select_4([b, c, e, d, g])
    }
}
/// n = 8, i = 4, a < d, a < h, b < f, c < e, c < h, e < g, f < g, f < h
fn select_83([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_82([a, e, b, f, d, g, h])
    } else {
        select_16([d, c, b, f, e, h])
    }
}
/// n = 7, i = 2, b < g, c < d, c < g, d < e, d < f
fn select_84([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_42([a, d, e, f])
    } else {
        select_45([a, b, c, d, f, g])
    }
}
/// n = 10, i = 4, a < i, a < j, b < e, b < j, c < d, c < i, d < f, d < h, d < j, e < g, e < h, e < i
fn select_85([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_84([f, a, b, e, g, h, j])
    } else {
        select_84([g, a, c, d, f, h, i])
    }
}
/// n = 11, i = 5, a < g, a < k, b < e, c < d, c < g, d < f, d < j, d < k, e < h, e < k, h < i, h < j
fn select_86([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_83([d, b, a, f, g, e, h, k])
    } else {
        select_85([a, c, e, h, d, i, f, j, k, g])
    }
}
/// n = 11, i = 5, a < g, a < k, b < d, b < g, c < e, d < f, d < k, e < h, e < k, h < i, h < j
fn select_87([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < d {
        select_72([a, b, h, j, i, g, d])
    } else {
        select_86([a, c, b, d, e, f, g, h, j, i, k])
    }
}
/// n = 13, i = 5, a < d, a < g, b < g, b < k, c < e, d < f, e < h, e < k, h < i, h < j, k < l, k < m
fn select_88([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if k < d {
        select_81([a, h, b, i, j, k, l, m, g, d])
    } else {
        select_87([b, a, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 7, i = 2, b < d, b < g, c < e, c < g, e < f
fn select_89([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_47([a, c, b, d, g])
    } else {
        select_45([a, b, c, e, f, g])
    }
}
/// n = 8, i = 3, a < d, a < e, b < c, b < f, f < g, g < h
fn select_90([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_47([e, d, b, c, f])
    } else {
        select_89([c, a, f, e, g, h, d])
    }
}
/// n = 5, i = 1, b < c, b < d, b < e
fn select_91([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < a {
        select_5([c, d, e])
    } else {
        select_42([a, b, d, e])
    }
}
/// n = 6, i = 1, c < d, c < e, c < f
fn select_92([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_91([b, c, d, e, f])
    } else {
        select_77([a, b, c, e, f])
    }
}
/// n = 8, i = 1, c < g, c < h, d < e, d < f
fn select_93([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_92([a, b, c, d, g, h])
    } else {
        select_92([a, b, d, c, e, f])
    }
}
/// n = 6, i = 1, b < f, c < d, c < e
fn select_94([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_9([a, c, d, e, b])
    } else {
        select_64([b, c, d, e, a, f])
    }
}
/// n = 7, i = 2, c < d, c < e, e < f, e < g
fn select_95([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_94([b, d, e, f, g, a])
    } else {
        select_84([b, a, c, e, f, g, d])
    }
}
/// n = 10, i = 3, a < d, a < e, b < c, b < f, f < g, f < h, h < i, h < j
fn select_96([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_93([c, g, a, h, i, j, d, e])
    } else {
        select_95([d, e, b, c, f, g, h])
    }
}
/// n = 10, i = 3, a < e, a < f, b < c, b < d, b < g, g < h, h < i, h < j
fn select_97([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < g {
        select_90([a, b, d, e, f, c, g, h])
    } else {
        select_96([a, b, d, e, f, g, c, h, i, j])
    }
}
/// n = 4, i = 0
fn select_98([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_5([a, c, d])
    } else {
        select_5([b, c, d])
    }
}
/// n = 5, i = 0
fn select_99([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_98([a, c, d, e])
    } else {
        select_98([b, c, d, e])
    }
}
/// n = 7, i = 1, a < g, b < c, b < d, b < e, b < f
fn select_100([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_2([b, g])
    } else {
        select_99([a, c, d, e, f])
    }
}
/// n = 7, i = 2, a < e, a < g, b < c, b < d, b < g, e < f
fn select_101([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_29([c, d, a, g])
    } else {
        select_10([b, a, d, e, f, g])
    }
}
/// n = 10, i = 2, a < c, a < d, b < e, b < f, e < j, f < g, f < h, f < i
fn select_102([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_100([e, f, a, g, h, i, j])
    } else {
        select_101([b, a, c, d, e, j, f])
    }
}
/// n = 10, i = 2, a < e, a < f, b < c, b < d, b < g, g < h, g < i, g < j
fn select_103([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_102([a, b, e, f, c, g, h, i, j, d])
    } else {
        select_102([a, b, e, f, d, g, h, i, j, c])
    }
}
/// n = 6, i = 1, b < c, b < d, b < e, b < f
fn select_104([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_98([c, d, e, f])
    } else {
        select_91([a, b, d, e, f])
    }
}
/// n = 10, i = 2, a < d, a < e, a < j, b < c, b < f, b < j, f < g, f < h, f < i
fn select_105([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_104([c, f, a, g, h, i])
    } else {
        select_78([b, a, d, e, c, f, j])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < i, d < e, d < f, d < g, d < h
fn select_106([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_99([a, e, f, g, h])
    } else {
        select_42([c, b, d, i])
    }
}
/// n = 8, i = 4, a < h, b < f, b < g, c < f, c < g, c < h, d < e, e < g, e < h
fn select_107([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_52([a, d, c, f, e, g])
    } else {
        select_16([b, c, d, e, f, h])
    }
}
/// n = 11, i = 4, b < f, b < k, c < f, c < g, c < k, d < e, e < g, e < k, g < h, g < i, g < j
fn select_108([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_106([b, e, f, g, a, h, i, j, k])
    } else {
        select_107([a, b, c, d, e, f, k, g])
    }
}
/// n = 14, i = 5, a < d, a < e, a < k, b < g, b < h, b < k, c < f, e < g, f < h, f < k, h < i, h < j, k < l, k < m, k < n
fn select_109([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < e {
        select_105([h, a, d, i, j, k, l, m, n, e])
    } else {
        select_108([d, e, b, c, f, g, k, l, m, n, h])
    }
}
/// n = 14, i = 5, a < d, a < e, a < n, b < g, b < h, b < i, c < f, e < g, f < h, f < i, h < l, h < m, i < j, i < k, i < n
fn select_110([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_103([h, i, j, k, l, m, a, d, e, n])
    } else {
        select_109([a, b, c, d, e, f, g, h, l, m, i, j, k, n])
    }
}
/// n = 14, i = 5, a < d, a < e, a < n, b < j, b < k, c < f, e < j, f < g, f < k, g < h, g < i, k < l, k < m, k < n
fn select_111([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_97([a, g, h, i, d, e, b, k, l, m])
    } else {
        select_110([a, b, c, d, e, f, j, g, k, l, m, h, i, n])
    }
}
/// n = 14, i = 5, a < g, b < e, b < n, c < g, c < k, d < f, f < h, f < k, h < i, h < j, k < l, k < m, k < n
fn select_112([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_88([a, c, d, b, f, e, g, h, i, j, k, l, m])
    } else {
        select_111([b, c, d, e, a, f, h, i, j, g, k, l, m, n])
    }
}
/// n = 14, i = 5, a < h, b < f, b < n, c < h, c < k, d < e, d < g, g < i, g < k, i < j, k < l, k < m, k < n
fn select_113([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < i {
        select_70([a, b, c, d, f, e, g, h, i, k, j, l, m, n])
    } else {
        select_112([a, b, c, d, f, g, h, i, e, j, k, l, m, n])
    }
}
/// n = 14, i = 5, a < j, b < f, b < n, c < j, c < k, d < e, d < g, g < h, g < i, g < k, k < l, k < m, k < n
fn select_114([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_113([a, b, c, d, e, f, g, j, h, i, k, l, m, n])
    } else {
        select_113([a, b, c, d, e, f, g, j, i, h, k, l, m, n])
    }
}
/// n = 7, i = 2, a < d, b < c, b < e, e < f, e < g
fn select_115([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_91([c, e, a, f, g])
    } else {
        select_31([a, b, c, d, e])
    }
}
/// n = 9, i = 2, a < d, b < c, b < e, e < f, e < g, e < h, e < i
fn select_116([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < a {
        select_100([c, e, f, g, h, i, a])
    } else {
        select_106([a, b, d, e, f, g, h, i, c])
    }
}
/// n = 10, i = 2, a < d, b < c, b < e, b < f, e < j, f < g, f < h, f < i
fn select_117([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_115([a, b, c, d, e, f, j])
    } else {
        select_116([a, b, c, d, f, e, g, h, i])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, e < j, f < g, f < h, f < i
fn select_118([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_117([a, c, d, b, e, f, g, h, i, j])
    } else {
        select_117([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 7, i = 1, a < g, b < f, c < d, c < e
fn select_119([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_64([a, c, d, e, b, g])
    } else {
        select_64([b, c, d, e, a, f])
    }
}
/// n = 8, i = 1, a < h, b < g, c < d, c < e, c < f
fn select_120([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_119([a, b, c, d, f, g, h])
    } else {
        select_119([a, b, c, e, f, g, h])
    }
}
/// n = 8, i = 1, c < h, d < e, d < f, d < g
fn select_121([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_120([a, c, d, e, f, g, h, b])
    } else {
        select_120([b, c, d, e, f, g, h, a])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < e, c < f
fn select_122([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_31([a, c, d, b, f])
    } else {
        select_31([b, c, d, a, e])
    }
}
/// n = 7, i = 2, b < g, c < d, c < e, c < g, e < f
fn select_123([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_94([d, b, e, a, f, g])
    } else {
        select_122([a, b, c, d, g, e])
    }
}
/// n = 9, i = 2, c < d, c < e, c < f, e < i, f < g, f < h
fn select_124([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_121([b, d, e, f, a, g, h, i])
    } else {
        select_123([b, a, c, d, e, i, f])
    }
}
/// n = 8, i = 2, b < h, c < d, c < h, d < e, d < f, d < g
fn select_125([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_91([a, d, e, f, g])
    } else {
        select_84([a, b, c, d, f, g, h])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_126([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_125([a, d, b, f, g, h, i, e])
    } else {
        select_125([a, e, c, f, g, h, i, d])
    }
}
/// n = 10, i = 3, b < e, b < j, c < d, c < f, f < g, f < h, f < j, h < i
fn select_127([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_124([a, d, f, g, h, b, e, j, i])
    } else {
        select_126([a, b, c, d, e, f, g, h, j])
    }
}
/// n = 12, i = 3, b < e, c < d, c < f, f < g, f < h, f < i, h < l, i < j, i < k
fn select_128([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < b {
        select_118([a, d, f, g, h, i, b, j, k, l])
    } else {
        select_127([a, b, c, d, e, f, g, h, l, i])
    }
}
/// n = 12, i = 3, b < e, c < d, c < f, f < g, f < h, f < i, f < j, j < k, j < l
fn select_129([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_128([a, b, c, d, e, f, i, g, j, k, l, h])
    } else {
        select_128([a, b, c, d, e, f, i, h, j, k, l, g])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < f
fn select_130([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_47([a, b, c, d, f])
    } else {
        select_47([a, c, b, e, f])
    }
}
/// n = 8, i = 2, b < e, c < d, c < f, f < g, f < h
fn select_131([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < b {
        select_92([a, d, f, b, g, h])
    } else {
        select_130([a, b, c, d, e, f])
    }
}
/// n = 8, i = 2, d < e, d < f, f < g, f < h
fn select_132([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_131([c, a, d, e, b, f, g, h])
    } else {
        select_131([c, b, d, e, a, f, g, h])
    }
}
/// n = 10, i = 1, a < g, a < h, a < i, a < j, b < c, b < d, b < e, b < f
fn select_133([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_99([b, g, h, i, j])
    } else {
        select_99([a, c, d, e, f])
    }
}
/// n = 10, i = 1, b < h, b < i, b < j, c < d, c < e, c < f, c < g
fn select_134([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_100([a, c, d, e, f, g, b])
    } else {
        select_133([b, c, d, e, f, g, a, h, i, j])
    }
}
/// n = 9, i = 3, b < i, c < e, c < i, d < e, d < i, e < f, e < g, e < h
fn select_135([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_100([b, e, a, f, g, h, i])
    } else {
        select_40([a, b, c, d, i, e])
    }
}
/// n = 12, i = 3, c < e, c < f, d < e, d < f, e < j, e < k, e < l, f < g, f < h, f < i
fn select_136([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_134([b, f, e, a, j, k, l, g, h, i])
    } else {
        select_135([b, a, c, d, f, g, h, i, e])
    }
}
/// n = 6, i = 3, a < e, a < f, b < e, b < f, c < d, c < f
fn select_137([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_18([d, a, b, f])
    } else {
        select_3([e, c, f])
    }
}
/// n = 6, i = 3, b < f, c < e, c < f, d < e, d < f
fn select_138([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < a {
        select_137([c, d, b, a, e, f])
    } else {
        select_15([c, d, a, b, e])
    }
}
/// n = 8, i = 4, b < g, b < h, c < f, c < h, d < e, d < g, e < h, f < g
fn select_139([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_138([a, b, c, e, f, h])
    } else {
        select_138([a, b, d, f, e, g])
    }
}
/// n = 9, i = 3, a < d, b < e, c < e, e < f, e < g, e < h, e < i
fn select_140([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_99([a, f, g, h, i])
    } else {
        select_18([d, b, c, e])
    }
}
/// n = 10, i = 3, a < d, a < j, b < e, b < j, c < e, c < j, e < f, e < g, e < h, e < i
fn select_141([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_99([a, f, g, h, i])
    } else {
        select_76([d, b, c, e, j])
    }
}
/// n = 10, i = 3, b < j, c < e, c < j, d < e, d < j, e < f, e < g, e < h, e < i
fn select_142([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_140([a, c, d, b, e, f, g, h, i])
    } else {
        select_141([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 11, i = 4, b < g, b < k, c < f, c < g, d < e, d < g, d < k, f < k, g < h, g < i, g < j
fn select_143([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_139([a, b, c, d, e, f, k, g])
    } else {
        select_142([a, f, b, d, g, e, h, i, j, k])
    }
}
/// n = 13, i = 4, b < g, b < h, c < f, c < g, c < h, d < e, d < g, d < h, g < l, g < m, h < i, h < j, h < k
fn select_144([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_136([a, f, b, d, g, h, i, j, k, e, l, m])
    } else {
        select_143([a, b, d, c, f, e, h, i, j, k, g])
    }
}
/// n = 13, i = 4, b < i, b < m, c < f, c < g, c < i, d < e, d < g, d < i, g < h, g < m, i < j, i < k, i < l
fn select_145([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < b {
        select_132([a, e, f, g, h, b, i, m])
    } else {
        select_144([a, b, c, d, e, f, g, i, j, k, l, h, m])
    }
}
/// n = 6, i = 3, b < d, c < e, e < f
fn select_146([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_27([b, a, c, d, e, f])
    } else {
        select_32([b, e, a, f, d])
    }
}
/// n = 6, i = 2, a < f, b < d, b < e, c < d, c < e, c < f
fn select_147([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_3([b, c, f])
    } else {
        select_42([a, c, d, e])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < e, c < f
fn select_148([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_9([d, b, e, f, a])
    } else {
        select_147([a, b, c, e, f, d])
    }
}
/// n = 7, i = 3, b < e, c < d, c < f, c < g, e < f, e < g
fn select_149([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_62([c, e, a, d, f, g])
    } else {
        select_15([a, b, c, d, e])
    }
}
/// n = 8, i = 3, b < d, b < g, b < h, c < e, e < f, e < g, e < h
fn select_150([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_148([a, b, e, f, g, h])
    } else {
        select_149([a, c, b, d, e, g, h])
    }
}
/// n = 8, i = 3, b < e, b < g, b < h, c < d, c < f, f < g, f < h
fn select_151([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_146([a, b, c, e, d, f])
    } else {
        select_150([a, b, c, e, f, d, g, h])
    }
}
/// n = 7, i = 2, a < f, a < g, b < d, b < e, b < g, c < d, c < e, c < f
fn select_152([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_28([b, c, f, g])
    } else {
        select_9([a, c, d, e, g])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, b < g, c < d, c < e, c < f
fn select_153([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_9([d, b, e, f, a])
    } else {
        select_152([a, b, c, e, f, d, g])
    }
}
/// n = 10, i = 2, a < c, a < i, a < j, b < d, b < i, b < j, d < e, d < f, d < g, d < h
fn select_154([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_99([a, e, f, g, h])
    } else {
        select_91([c, b, d, i, j])
    }
}
/// n = 10, i = 2, b < d, b < i, b < j, c < e, c < i, c < j, e < f, e < g, e < h
fn select_155([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_153([a, c, b, d, i, j, e])
    } else {
        select_154([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < g, c < e, c < f, d < e, d < f, d < g
fn select_156([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_21([b, c, d, f, g])
    } else {
        select_21([a, c, d, e, g])
    }
}
/// n = 7, i = 3, a < g, b < f, b < g, c < e, c < f, d < e, d < f, d < g
fn select_157([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_156([a, b, c, d, f, e, g])
    } else {
        select_28([b, e, a, f])
    }
}
/// n = 10, i = 3, b < e, b < j, c < f, c < j, d < e, d < f, d < j, f < g, f < h, f < i
fn select_158([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_106([b, c, e, f, a, g, h, i, j])
    } else {
        select_157([a, c, b, d, e, j, f])
    }
}
/// n = 13, i = 4, b < f, b < i, c < e, c < i, c < m, d < f, d < h, d < i, e < h, f < g, f < m, i < j, i < k, i < l
fn select_159([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < e {
        select_155([a, f, c, g, i, j, k, l, e, m])
    } else {
        select_158([a, e, b, d, h, i, j, k, l, f])
    }
}
/// n = 13, i = 4, b < h, b < i, b < m, c < f, c < i, d < e, d < i, d < m, e < h, f < g, f < m, i < j, i < k, i < l
fn select_160([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < b {
        select_151([a, d, f, g, e, b, i, m])
    } else {
        select_159([a, c, d, b, e, f, g, h, i, j, k, l, m])
    }
}
/// n = 8, i = 5, a < g, a < h, b < g, b < h, c < f, d < e, e < g, f < h
fn select_161([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_15([a, b, d, e, h])
    } else {
        select_15([a, b, c, f, g])
    }
}
/// n = 5, i = 2, a < e, b < e, c < d
fn select_162([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < d {
        select_21([b, c, a, d, e])
    } else {
        select_3([b, d, a])
    }
}
/// n = 6, i = 3, a < f, b < f, c < d, d < e
fn select_163([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < f {
        select_162([a, b, d, e, f])
    } else {
        select_1([c])
    }
}
/// n = 8, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h
fn select_164([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_161([b, c, a, d, f, e, h, g])
    } else {
        select_163([a, g, d, f, h, e])
    }
}
/// n = 10, i = 5, a < f, a < i, a < j, b < g, b < h, c < g, c < h, d < e, d < h, h < i, h < j
fn select_165([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_150([g, a, d, f, h, e, i, j])
    } else {
        select_164([a, b, c, d, f, e, g, h])
    }
}
/// n = 15, i = 6, a < g, a < k, b < f, b < k, b < o, c < h, c < i, d < e, d < h, f < j, g < i, g < o, h < j, h < k, h < o, k < l, k < m, k < n
fn select_166([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < h {
        select_165([b, c, g, d, e, f, i, h, k, o])
    } else {
        select_159([e, a, b, h, f, g, i, j, k, l, m, n, o])
    }
}
/// n = 15, i = 6, a < f, a < k, a < o, b < g, b < k, c < h, c < j, d < e, d < h, g < j, g < o, h < i, h < k, h < o, k < l, k < m, k < n
fn select_167([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_160([e, a, b, h, i, g, j, f, k, l, m, n, o])
    } else {
        select_166([b, a, c, d, e, f, g, h, j, i, k, l, m, n, o])
    }
}
/// n = 16, i = 6, a < g, a < m, b < f, b < k, b < m, c < h, c < j, d < e, d < h, g < j, h < i, h < k, h < m, k < l, m < n, m < o, m < p
fn select_168([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < g {
        select_145([e, a, b, h, i, f, k, l, m, n, o, p, g])
    } else {
        select_167([b, a, c, d, e, f, g, h, i, j, m, n, o, p, k])
    }
}
/// n = 16, i = 6, a < f, a < k, b < g, b < l, c < h, c < j, d < e, d < h, g < j, h < i, h < k, h < l, k < p, l < m, l < n, l < o
fn select_169([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < a {
        select_129([e, g, h, i, j, l, m, n, o, a, f, k])
    } else {
        select_168([b, a, c, d, e, f, g, h, i, j, k, p, l, m, n, o])
    }
}
/// n = 7, i = 2, a < d, a < e, b < c, b < f, f < g
fn select_170([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_32([b, a, d, e, c])
    } else {
        select_7([a, b, c, e, f, g])
    }
}
/// n = 7, i = 2, a < g, b < f, c < d, c < e
fn select_171([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_170([c, a, g, d, e, b, f])
    } else {
        select_170([c, b, f, d, e, a, g])
    }
}
/// n = 8, i = 2, a < f, b < e, c < d, c < g, g < h
fn select_172([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if h < b {
        select_7([a, c, d, f, g, h])
    } else {
        select_171([a, b, c, d, g, e, f])
    }
}
/// n = 9, i = 3, a < c, a < d, b < e, b < i, d < i, e < f, e < g, g < h
fn select_173([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_94([f, g, a, c, d, h])
    } else {
        select_84([c, d, b, e, f, g, i])
    }
}
/// n = 7, i = 2, a < d, a < g, b < c, b < e, b < g, e < f
fn select_174([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if g < e {
        select_31([a, b, c, d, g])
    } else {
        select_7([a, b, c, d, e, f])
    }
}
/// n = 9, i = 3, a < c, a < g, b < d, b < g, c < f, d < e, d < h, h < i
fn select_175([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_31([b, c, f, g, d])
    } else {
        select_174([a, d, e, g, h, i, c])
    }
}
/// n = 9, i = 3, a < g, b < d, c < e, c < g, e < f, e < h, h < i
fn select_176([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_173([b, c, d, a, e, f, h, i, g])
    } else {
        select_175([a, c, b, e, f, d, g, h, i])
    }
}
/// n = 9, i = 3, b < e, c < d, c < f, f < g, f < h, h < i
fn select_177([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_172([b, d, f, g, a, e, h, i])
    } else {
        select_176([a, b, c, e, f, g, d, h, i])
    }
}
/// n = 9, i = 3, d < e, d < f, f < g, f < h, h < i
fn select_178([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_177([c, a, d, e, b, f, g, h, i])
    } else {
        select_177([c, b, d, e, a, f, g, h, i])
    }
}
/// n = 8, i = 2, a < d, a < h, b < c, b < e, b < h, e < f, e < g
fn select_179([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_91([c, e, a, f, g])
    } else {
        select_62([a, b, c, d, e, h])
    }
}
/// n = 9, i = 2, a < i, b < d, b < h, b < i, c < e, c < h, e < f, e < g
fn select_180([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < c {
        select_10([b, a, d, c, e, i])
    } else {
        select_179([b, c, a, d, e, f, g, h])
    }
}
/// n = 8, i = 3, a < d, b < e, c < e, c < h, d < h, e < f, e < g
fn select_181([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_5([a, f, g])
    } else {
        select_21([b, d, c, h, e])
    }
}
/// n = 10, i = 3, a < d, b < g, b < j, c < e, c < g, e < f, e < j, g < h, g < i
fn select_182([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_180([a, e, b, f, g, h, i, j, d])
    } else {
        select_181([a, b, c, d, g, h, i, e])
    }
}
/// n = 7, i = 2, b < d, c < e, e < f, e < g
fn select_183([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_91([a, e, b, f, g])
    } else {
        select_47([a, c, b, d, e])
    }
}
/// n = 8, i = 3, a < d, b < e, c < e, e < f, e < g, e < h
fn select_184([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_98([a, f, g, h])
    } else {
        select_18([d, b, c, e])
    }
}
/// n = 8, i = 3, a < e, b < f, c < d, c < f, f < g, f < h
fn select_185([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_51([b, a, c, e, d, f])
    } else {
        select_184([a, b, c, e, f, d, g, h])
    }
}
/// n = 8, i = 3, a < e, b < g, b < h, c < d, c < f, f < g, f < h
fn select_186([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < b {
        select_183([d, a, f, e, b, g, h])
    } else {
        select_185([a, b, c, d, e, f, g, h])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < f
fn select_187([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_21([a, d, b, e, f])
    } else {
        select_21([a, e, c, d, f])
    }
}
/// n = 9, i = 3, a < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_188([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_104([d, f, a, g, h, i])
    } else {
        select_187([b, a, c, d, e, f])
    }
}
/// n = 10, i = 3, a < e, b < g, b < j, c < d, c < f, c < g, f < j, g < h, g < i
fn select_189([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_186([a, b, c, d, e, f, g, j])
    } else {
        select_188([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 12, i = 3, a < f, b < h, c < e, c < g, d < g, d < h, g < i, g < j, h < k, h < l
fn select_190([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < c {
        select_182([a, d, b, f, c, e, h, k, l, g])
    } else {
        select_189([a, d, c, e, f, b, g, i, j, h])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < e, e < g
fn select_191([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_51([a, b, d, f, e, g])
    } else {
        select_21([a, b, c, f, g])
    }
}
/// n = 10, i = 4, a < f, b < g, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_192([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_191([b, a, c, d, e, f, g])
    } else {
        select_140([a, b, d, f, g, e, h, i, j])
    }
}
/// n = 4, i = 3
fn select_193([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_14([b, c, d])
    } else {
        select_14([a, c, d])
    }
}
/// n = 8, i = 4, a < e, b < f, c < f, d < f, f < g, f < h
fn select_194([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_42([a, f, g, h])
    } else {
        select_193([b, c, d, e])
    }
}
/// n = 8, i = 4, a < h, b < e, c < f, d < f, f < g, f < h
fn select_195([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_194([b, a, c, d, e, f, g, h])
    } else {
        select_7([b, f, g, e, a, h])
    }
}
/// n = 5, i = 3, b < e, c < e, d < e
fn select_196([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < a {
        select_18([a, c, d, e])
    } else {
        select_14([b, c, d])
    }
}
/// n = 7, i = 4, b < e, c < f, d < f, f < g
fn select_197([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_47([a, b, f, g, e])
    } else {
        select_196([a, c, d, e, f])
    }
}
/// n = 8, i = 4, a < h, b < e, c < f, c < g, d < f, d < g, g < h
fn select_198([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_195([a, b, c, d, e, g, f, h])
    } else {
        select_197([a, b, c, d, e, f, g])
    }
}
/// n = 8, i = 2, a < e, b < c, b < d, d < f, e < g, e < h
fn select_199([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_29([a, c, d, f])
    } else {
        select_6([b, a, c, e, g, h])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, c < g, d < e, d < f
fn select_200([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_5([a, e, f])
    } else {
        select_9([c, b, d, h, g])
    }
}
/// n = 10, i = 3, a < f, a < j, b < d, b < e, b < j, c < e, c < f, c < j, e < g, f < h, f < i
fn select_201([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_45([d, a, c, e, g, j])
    } else {
        select_200([b, a, e, f, h, i, g, j])
    }
}
/// n = 10, i = 3, a < d, a < j, b < g, b < j, c < e, c < g, c < j, e < f, g < h, g < i
fn select_202([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_199([b, e, f, a, g, d, h, i])
    } else {
        select_201([b, a, c, d, e, g, f, h, i, j])
    }
}
/// n = 11, i = 4, a < g, b < e, b < k, c < h, c < k, d < f, d < g, f < h, f < k, h < i, h < j
fn select_203([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_198([c, b, a, d, e, g, f, h])
    } else {
        select_202([b, c, f, e, a, g, h, i, j, k])
    }
}
/// n = 13, i = 4, a < g, b < e, b < i, c < h, c < i, d < f, d < g, d < i, f < h, h < j, h < k, i < l, i < m
fn select_204([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_192([a, c, d, b, e, g, i, f, l, m])
    } else {
        select_203([a, b, c, d, e, f, g, h, j, k, i])
    }
}
/// n = 13, i = 4, b < f, b < i, c < h, c < i, d < e, d < g, d < i, g < h, h < j, h < k, i < l, i < m
fn select_205([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < a {
        select_190([e, g, b, c, f, a, i, h, l, m, j, k])
    } else {
        select_204([a, b, c, d, f, g, e, h, i, j, k, l, m])
    }
}
/// n = 13, i = 4, b < j, b < m, c < f, c < h, d < e, d < g, d < h, g < j, h < i, h < m, j < k, j < l
fn select_206([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_178([a, e, g, c, f, h, i, b, m])
    } else {
        select_205([a, c, b, d, e, f, g, j, h, k, l, i, m])
    }
}
/// n = 6, i = 1, a < f, b < c, b < d, b < e
fn select_207([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_2([b, f])
    } else {
        select_98([a, c, d, e])
    }
}
/// n = 8, i = 2, a < c, b < d, b < e, d < h, e < f, e < g
fn select_208([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_207([d, e, a, f, g, h])
    } else {
        select_10([a, b, c, d, h, e])
    }
}
/// n = 10, i = 2, a < c, a < e, b < d, b < f, d < i, e < j, f < g, f < h
fn select_209([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_208([a, b, c, d, f, g, h, i])
    } else {
        select_199([b, a, c, e, f, j, g, h])
    }
}
/// n = 11, i = 3, a < d, a < f, b < e, b < g, c < e, c < f, c < g, e < j, f < k, g < h, g < i
fn select_210([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_64([e, g, h, i, a, j])
    } else {
        select_201([b, a, c, d, f, g, k, h, i, e])
    }
}
/// n = 11, i = 3, a < d, b < g, b < h, c < e, c < g, c < h, e < f, g < k, h < i, h < j
fn select_211([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_209([e, b, f, g, a, h, i, j, k, d])
    } else {
        select_210([a, b, c, d, g, e, h, i, j, k, f])
    }
}
/// n = 11, i = 3, a < f, b < e, c < g, c < h, d < g, d < h, g < k, h < i, h < j
fn select_212([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_186([a, d, b, e, f, c, g, h])
    } else {
        select_211([a, d, c, f, b, e, g, h, i, j, k])
    }
}
/// n = 9, i = 4, a < f, b < h, b < i, c < f, c < g, d < e, d < g, g < h, g < i
fn select_213([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < e {
        select_186([a, b, d, e, f, g, h, i])
    } else {
        select_47([b, a, c, g, f])
    }
}
/// n = 12, i = 4, a < g, b < e, c < h, c < i, d < f, d < g, f < h, f < i, h < l, i < j, i < k
fn select_214([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_211([a, c, f, g, b, e, h, i, j, k, l])
    } else {
        select_213([a, c, d, b, e, g, f, h, i])
    }
}
/// n = 12, i = 4, b < f, c < h, c < i, d < e, d < g, g < h, g < i, h < l, i < j, i < k
fn select_215([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_212([b, e, c, g, a, f, h, i, j, k, l])
    } else {
        select_214([a, b, c, d, f, g, e, h, i, j, k, l])
    }
}
/// n = 7, i = 1, a < f, a < g, b < c, b < d, b < e
fn select_216([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_5([b, f, g])
    } else {
        select_98([a, c, d, e])
    }
}
/// n = 7, i = 1, b < g, c < d, c < e, c < f
fn select_217([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_207([a, c, d, e, f, b])
    } else {
        select_216([b, c, d, e, f, a, g])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < f, c < g, d < e
fn select_218([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_64([b, d, a, e, f, g])
    } else {
        select_147([a, b, c, f, g, d])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < i, d < h, e < f, e < g
fn select_219([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_217([a, d, e, b, f, g, h])
    } else {
        select_218([a, b, c, d, h, e, i])
    }
}
/// n = 10, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < j, g < h, g < i
fn select_220([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_219([a, d, b, f, g, h, i, j, e])
    } else {
        select_219([a, e, c, f, g, h, i, j, d])
    }
}
/// n = 7, i = 2, a < c, b < d, d < e, d < f, d < g
fn select_221([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_98([a, e, f, g])
    } else {
        select_3([c, b, d])
    }
}
/// n = 7, i = 2, c < d, d < e, d < f, d < g
fn select_222([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_221([a, c, b, d, e, f, g])
    } else {
        select_221([b, c, a, d, e, f, g])
    }
}
/// n = 8, i = 3, a < e, b < e, b < f, c < d, c < f, f < g, f < h
fn select_223([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_91([d, f, a, g, h])
    } else {
        select_137([a, b, c, d, e, f])
    }
}
/// n = 9, i = 4, a < f, a < g, b < e, b < g, c < d, c < g, g < h, g < i
fn select_224([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_223([d, b, a, f, e, g, h, i])
    } else {
        select_223([e, c, a, f, d, g, h, i])
    }
}
/// n = 10, i = 4, a < f, a < g, b < e, b < g, c < d, c < g, g < h, g < i, g < j
fn select_225([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_222([e, f, c, g, h, i, j])
    } else {
        select_224([a, b, c, d, e, f, g, i, j])
    }
}
/// n = 12, i = 5, a < g, a < h, b < e, b < g, b < j, c < d, c < h, e < f, e < l, h < i, h < j, h < l, j < k
fn select_226([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_220([d, b, h, i, g, j, e, f, l, k])
    } else {
        select_225([a, c, e, f, d, g, h, i, j, l])
    }
}
/// n = 10, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_227([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_125([a, f, d, g, h, i, j, e])
    } else {
        select_135([a, e, b, c, g, h, i, j, f])
    }
}
/// n = 9, i = 3, b < e, c < f, d < e, d < f, f < g, f < h, f < i
fn select_228([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_104([a, f, b, g, h, i])
    } else {
        select_138([a, c, b, d, e, f])
    }
}
/// n = 10, i = 4, b < g, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_229([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_228([a, e, b, c, f, g, h, i, j])
    } else {
        select_228([a, f, b, d, e, g, h, i, j])
    }
}
/// n = 12, i = 6, a < h, b < g, b < j, c < f, c < h, d < e, d < g, g < i, g < k, g < l, h < j, h < k, h < l
fn select_230([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_227([f, b, h, d, e, j, g, i, k, l])
    } else {
        select_229([e, a, c, g, i, f, h, j, k, l])
    }
}
/// n = 13, i = 6, a < f, a < m, b < g, c < h, c < j, d < e, d < h, g < j, g < k, g < m, h < i, h < k, h < m, k < l
fn select_231([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_226([c, g, d, e, a, f, j, h, i, k, l, m])
    } else {
        select_230([b, c, a, d, e, f, h, g, i, j, k, m])
    }
}
/// n = 15, i = 6, a < f, a < o, b < g, b < l, c < h, c < j, d < e, d < h, g < j, g < o, h < i, h < k, h < o, k < l, l < m, l < n
fn select_232([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < g {
        select_215([e, a, b, h, i, f, k, g, l, m, n, j])
    } else {
        select_231([a, b, c, d, e, f, g, h, i, j, k, l, o])
    }
}
/// n = 16, i = 6, a < f, a < l, b < g, b < n, c < h, c < j, d < e, d < h, g < j, h < i, h < k, h < l, k < n, l < m, n < o, n < p
fn select_233([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < g {
        select_206([e, b, a, h, i, f, k, l, m, n, o, p, g])
    } else {
        select_232([a, b, c, d, e, f, g, h, i, j, k, n, o, p, l])
    }
}
/// n = 16, i = 6, a < f, a < l, b < g, b < m, c < h, c < k, d < e, d < h, g < k, h < i, h < j, h < l, h < m, l < p, m < n, m < o
fn select_234([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if m < i {
        select_169([a, b, c, d, e, f, g, h, j, k, l, m, i, n, o, p])
    } else {
        select_233([a, b, c, d, e, f, g, h, j, k, i, l, p, m, n, o])
    }
}
/// n = 16, i = 6, a < f, a < o, b < g, b < h, c < i, c < n, d < e, d < h, d < i, g < n, h < l, h < m, i < j, i < k, i < o, o < p
fn select_235([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < i {
        select_114([g, a, c, d, e, f, h, l, m, n, i, j, k, o])
    } else {
        select_234([a, b, c, d, e, f, g, i, j, k, n, o, h, l, m, p])
    }
}
/// n = 7, i = 3, a < e, b < d, c < e, c < g, d < g, e < f
fn select_236([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_18([a, c, d, g])
    } else {
        select_3([b, e, f])
    }
}
/// n = 8, i = 3, a < g, a < h, b < e, c < d, c < f, d < h, e < f, e < g
fn select_237([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_10([c, e, d, a, g, f])
    } else {
        select_236([b, c, a, d, e, g, h])
    }
}
/// n = 6, i = 2, a < f, b < e, b < f, c < d, c < e
fn select_238([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_3([d, b, e])
    } else {
        select_162([a, b, c, d, f])
    }
}
/// n = 8, i = 3, a < g, a < h, b < e, b < f, c < d, c < f, d < g, e < h
fn select_239([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_238([a, d, b, f, e, g])
    } else {
        select_238([a, e, c, f, d, h])
    }
}
/// n = 8, i = 3, a < h, b < g, b < h, c < f, c < h, d < e, d < f, e < g
fn select_240([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_237([b, a, d, e, c, f, h, g])
    } else {
        select_239([b, c, d, e, a, f, g, h])
    }
}
/// n = 6, i = 2, b < d, c < e, e < f
fn select_241([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_162([a, c, b, d, e])
    } else {
        select_6([b, c, d, e, a, f])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < g, c < d, d < g, d < h, e < f
fn select_242([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_3([c, e, f])
    } else {
        select_21([a, b, d, g, h])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, b < g, c < e, c < f, d < f, d < g, f < i, g < h
fn select_243([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_51([a, b, d, e, f, i])
    } else {
        select_27([c, a, d, e, g, h])
    }
}
/// n = 9, i = 4, a < f, a < i, b < g, b < i, c < f, c < g, d < e, d < g, e < i, g < h
fn select_244([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_242([b, c, e, a, g, h, f, i])
    } else {
        select_243([b, c, a, d, f, e, g, h, i])
    }
}
/// n = 9, i = 4, a < h, a < i, b < f, b < i, c < f, c < g, d < e, d < g, e < i, g < h
fn select_245([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_241([e, b, g, f, a, h])
    } else {
        select_244([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 9, i = 4, a < g, b < h, b < i, c < g, c < i, d < e, d < f, e < i, f < h
fn select_246([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_240([e, c, b, f, a, h, g, i])
    } else {
        select_245([b, c, a, d, e, g, f, h, i])
    }
}
/// n = 8, i = 3, a < g, a < h, b < d, b < f, c < e, e < f, e < g, e < h
fn select_247([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_4([d, c, a, e, f])
    } else {
        select_22([b, a, c, e, d, g, h, f])
    }
}
/// n = 8, i = 4, a < g, a < h, b < f, b < h, c < f, c < g, d < e, e < g, e < h
fn select_248([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_156([a, b, c, e, f, g, h])
    } else {
        select_18([a, d, f, e])
    }
}
/// n = 10, i = 4, a < f, a < i, a < j, b < e, b < h, c < g, c < j, d < f, d < g, e < j, g < h, g < i
fn select_249([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_247([b, a, c, f, g, i, e, h])
    } else {
        select_248([c, a, d, b, e, f, g, j])
    }
}
/// n = 7, i = 2, a < f, a < g, b < e, b < g, c < d, c < f
fn select_250([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_43([e, a, c, d, g])
    } else {
        select_43([d, a, b, e, f])
    }
}
/// n = 9, i = 3, a < c, a < i, b < d, b < e, c < g, d < h, d < i, e < f, e < h
fn select_251([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_4([c, b, g, d, i])
    } else {
        select_250([d, a, e, f, c, h, i])
    }
}
/// n = 8, i = 4, a < e, a < h, b < e, b < g, c < d, d < g, d < h, e < f
fn select_252([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_31([a, b, g, h, e])
    } else {
        select_3([f, c, d])
    }
}
/// n = 9, i = 4, a < e, a < g, b < e, b < h, c < d, d < g, d < h, e < f, h < i
fn select_253([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < i {
        select_252([a, b, c, d, e, f, h, g])
    } else {
        select_1([i])
    }
}
/// n = 10, i = 4, a < e, a < j, b < e, b < g, b < i, c < d, d < f, d < g, e < h, f < i, f < j
fn select_254([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_10([b, f, g, a, j, i])
    } else {
        select_253([b, a, c, d, e, h, g, f, j])
    }
}
/// n = 11, i = 4, a < g, a < j, a < k, b < f, b < g, b < h, c < d, c < f, d < e, d < h, e < j, f < k, g < i
fn select_255([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_251([a, c, g, f, d, h, i, e, k])
    } else {
        select_254([a, b, c, d, g, e, h, i, f, j])
    }
}
/// n = 12, i = 5, a < h, a < i, a < l, b < f, b < l, c < d, c < e, d < h, e < g, e < j, f < i, f < j, g < l, i < k
fn select_256([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < f {
        select_249([a, e, b, d, g, h, f, j, i, l])
    } else {
        select_255([a, f, c, e, g, d, i, j, k, l, h])
    }
}
/// n = 12, i = 5, a < i, a < j, a < l, b < e, b < l, c < d, c < f, d < i, e < j, f < g, f < h, h < l, j < k
fn select_257([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_246([d, b, a, f, h, g, i, e, l])
    } else {
        select_256([a, b, c, d, f, e, h, i, j, g, k, l])
    }
}
/// n = 8, i = 2, a < d, b < c, c < e, c < f, d < g, d < h
fn select_258([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_42([a, c, e, f])
    } else {
        select_42([b, d, g, h])
    }
}
/// n = 9, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h, f < i
fn select_259([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_258([a, b, e, f, d, g, h, i])
    } else {
        select_181([b, a, c, d, f, h, i, e])
    }
}
/// n = 10, i = 3, a < g, a < j, b < e, c < d, c < g, c < h, e < f, e < h, e < j, g < i
fn select_260([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_174([c, e, f, d, a, j, h])
    } else {
        select_259([b, c, a, d, g, e, i, f, j])
    }
}
/// n = 11, i = 4, a < k, b < e, b < k, c < d, c < h, d < g, d < j, d < k, e < f, e < h, e < j, h < i
fn select_261([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_179([d, e, f, g, h, a, i, j])
    } else {
        select_85([a, b, c, d, e, g, f, j, h, k])
    }
}
/// n = 13, i = 4, a < h, b < d, b < j, c < e, c < h, d < g, d < l, e < f, e < j, e < l, e < m, h < i, h < m, j < k
fn select_262([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < d {
        select_260([b, a, e, f, h, i, j, m, k, d])
    } else {
        select_261([a, c, b, d, e, f, g, j, k, l, h])
    }
}
/// n = 7, i = 3, a < f, a < g, b < d, b < g, c < e, c < g, e < f
fn select_263([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_28([a, b, d, f])
    } else {
        select_47([d, a, c, e, g])
    }
}
/// n = 9, i = 4, a < g, a < i, b < f, b < g, c < d, c < f, d < e, f < i, g < h
fn select_264([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_263([a, d, b, e, f, i, g])
    } else {
        select_10([g, c, h, f, i, d])
    }
}
/// n = 7, i = 3, a < g, b < d, b < f, c < e, e < f, e < g
fn select_265([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_10([b, e, d, a, g, f])
    } else {
        select_27([b, a, c, d, e, g])
    }
}
/// n = 8, i = 4, a < e, b < g, c < f, c < h, d < f, d < g, g < h
fn select_266([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_265([a, c, b, f, g, h, e])
    } else {
        select_191([b, c, d, a, e, f, g])
    }
}
/// n = 10, i = 5, a < h, a < i, b < f, c < d, c < e, d < h, e < g, f < i, i < j
fn select_267([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_264([a, f, c, e, g, d, i, j, h])
    } else {
        select_266([e, b, a, d, g, h, f, i])
    }
}
/// n = 7, i = 3, a < d, b < e, b < f, c < e, c < f, f < g
fn select_268([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_15([b, c, a, d, e])
    } else {
        select_29([a, e, f, g])
    }
}
/// n = 7, i = 3, a < d, b < c, b < g, c < f, d < e, d < g
fn select_269([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_3([f, a, d])
    } else {
        select_42([e, b, c, g])
    }
}
/// n = 8, i = 4, a < f, a < h, b < e, b < f, c < d, c < e, d < h, f < g
fn select_270([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_269([b, c, d, f, g, h, e])
    } else {
        select_16([d, a, b, e, h, f])
    }
}
/// n = 10, i = 4, a < e, a < f, b < f, b < g, b < i, c < d, c < e, d < g, d < i, f < h, i < j
fn select_271([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < f {
        select_268([a, b, d, e, g, i, j])
    } else {
        select_270([b, a, c, d, e, f, h, g])
    }
}
/// n = 8, i = 4, a < f, b < e, c < f, c < g, d < e, d < h, e < g, f < h
fn select_272([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_18([b, d, f, h])
    } else {
        select_18([a, c, e, g])
    }
}
/// n = 9, i = 4, a < f, b < e, b < g, c < f, c < g, d < e, d < g, d < i, f < i, g < h
fn select_273([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_181([a, b, d, f, g, e, h, i])
    } else {
        select_272([a, b, c, d, e, f, g, i])
    }
}
/// n = 11, i = 5, a < g, a < h, a < j, b < e, b < j, c < d, c < f, e < h, f < g, f < j, h < i, j < k
fn select_274([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < d {
        select_271([e, a, c, f, d, h, g, i, j, k])
    } else {
        select_273([d, f, b, a, g, e, j, k, h])
    }
}
/// n = 12, i = 5, a < h, a < i, a < k, b < e, b < k, c < d, c < f, d < g, e < i, f < h, f < k, i < j, k < l
fn select_275([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < k {
        select_267([a, b, c, f, d, e, g, h, i, j])
    } else {
        select_274([a, b, c, d, e, f, h, i, j, k, l])
    }
}
/// n = 14, i = 5, a < d, a < e, b < h, b < j, b < k, c < f, c < h, d < j, e < g, e < n, f < k, h < i, h < m, h < n, k < l, k < m
fn select_276([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < d {
        select_262([f, a, b, e, h, i, g, k, l, d, j, n, m])
    } else {
        select_275([b, c, a, e, f, d, g, j, k, l, h, i])
    }
}
/// n = 14, i = 5, a < d, a < e, b < i, b < k, b < l, c < f, c < i, d < k, e < g, e < h, f < l, i < j, i < n, l < m, l < n
fn select_277([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < i {
        select_257([b, c, a, d, f, e, h, g, k, l, m, i])
    } else {
        select_276([a, b, c, d, e, f, h, i, j, k, l, m, n, g])
    }
}
/// n = 14, i = 5, a < j, b < i, c < e, c < f, d < i, d < j, d < k, e < k, f < g, f < h, i < l, i < n, j < m, j < n
fn select_278([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_277([c, d, a, e, f, b, g, h, j, m, k, i, l, n])
    } else {
        select_277([c, d, b, e, f, a, g, h, i, l, k, j, m, n])
    }
}
/// n = 8, i = 3, a < d, b < f, b < g, b < h, c < e, c < f, e < g, e < h
fn select_279([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_71([b, c, a, d, e, f])
    } else {
        select_63([b, e, f, a, d, g, h])
    }
}
/// n = 8, i = 3, c < f, c < g, c < h, d < e, d < f, e < g, e < h
fn select_280([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_279([a, c, d, b, e, f, g, h])
    } else {
        select_279([b, c, d, a, e, f, g, h])
    }
}
/// n = 8, i = 2, a < d, a < g, a < h, b < c, b < g, b < h, c < e, d < f
fn select_281([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_9([c, a, g, h, e])
    } else {
        select_9([d, b, g, h, f])
    }
}
/// n = 7, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g
fn select_282([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_4([a, c, e, f, g])
    } else {
        select_46([b, c, d, e])
    }
}
/// n = 7, i = 3, a < f, b < d, b < e, c < e, c < f, f < g
fn select_283([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < d {
        select_282([b, a, c, e, d, f, g])
    } else {
        select_46([c, d, a, e])
    }
}
/// n = 9, i = 3, a < d, b < f, b < h, b < i, c < e, c < f, e < h, e < i, f < g
fn select_284([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_281([b, e, a, f, d, g, h, i])
    } else {
        select_283([b, a, c, d, e, f, g])
    }
}
/// n = 9, i = 3, c < f, c < h, c < i, d < e, d < f, e < h, e < i, f < g
fn select_285([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_284([a, c, d, b, e, f, g, h, i])
    } else {
        select_284([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 9, i = 3, c < f, c < i, d < g, d < i, e < f, e < g, e < i, g < h
fn select_286([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_280([a, b, e, c, d, f, g, i])
    } else {
        select_285([a, b, e, d, c, g, h, f, i])
    }
}
/// n = 8, i = 3, c < e, c < f, d < e, d < f, f < g, f < h
fn select_287([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_92([b, e, f, a, g, h])
    } else {
        select_138([b, a, c, d, e, f])
    }
}
/// n = 9, i = 3, c < e, c < f, d < e, d < f, f < g, f < h, f < i
fn select_288([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_92([b, e, f, g, h, i])
    } else {
        select_287([a, b, c, d, e, f, h, i])
    }
}
/// n = 8, i = 2, c < d, c < e, e < f, e < g, e < h
fn select_289([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_217([b, d, e, f, g, h, a])
    } else {
        select_125([b, a, c, e, f, g, h, d])
    }
}
/// n = 11, i = 5, a < g, a < h, b < g, b < h, c < d, c < j, d < e, d < f, d < k, h < i, h < j, h < k
fn select_290([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < h {
        select_288([e, f, a, b, g, h, i, j, k])
    } else {
        select_289([g, i, c, j, d, e, f, k])
    }
}
/// n = 13, i = 5, a < g, a < i, b < g, b < i, b < l, c < d, c < k, d < e, d < f, d < h, h < l, i < j, i < k, l < m
fn select_291([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_286([e, f, a, h, b, g, l, m, i])
    } else {
        select_290([a, b, c, d, e, f, g, i, j, k, h])
    }
}
/// n = 8, i = 3, a < c, a < d, b < e, e < f, e < g, g < h
fn select_292([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < g {
        select_33([b, a, c, d, e, f])
    } else {
        select_7([a, e, f, d, g, h])
    }
}
/// n = 7, i = 3, a < e, b < c, b < d, b < g, e < f, e < g
fn select_293([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < c {
        select_62([b, e, f, d, c, g])
    } else {
        select_18([d, a, c, e])
    }
}
/// n = 9, i = 3, a < c, a < d, a < i, b < e, e < f, e < g, e < i, g < h
fn select_294([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_174([a, e, f, d, g, h, i])
    } else {
        select_293([b, a, c, d, e, f, i])
    }
}
/// n = 9, i = 3, a < c, a < d, a < i, b < e, b < f, e < h, f < g, f < i
fn select_295([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_292([a, b, c, d, e, h, f, g])
    } else {
        select_294([a, b, c, d, f, g, e, h, i])
    }
}
/// n = 9, i = 4, a < d, b < c, b < i, c < e, c < f, d < g, d < i, g < h
fn select_296([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_31([b, g, h, i, c])
    } else {
        select_293([a, c, e, f, d, i, g])
    }
}
/// n = 6, i = 2, c < d, d < e, d < f
fn select_297([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_6([a, c, b, d, e, f])
    } else {
        select_6([b, c, a, d, e, f])
    }
}
/// n = 8, i = 3, a < e, b < c, b < d, b < g, b < h, e < f, e < g, e < h
fn select_298([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < c {
        select_62([b, e, f, d, g, h])
    } else {
        select_297([c, d, a, e, g, h])
    }
}
/// n = 10, i = 4, a < d, b < c, b < i, c < f, c < g, c < j, d < e, d < h, d < j, h < i
fn select_299([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < c {
        select_76([e, b, h, c, i])
    } else {
        select_298([a, c, f, g, d, e, h, j])
    }
}
/// n = 10, i = 4, a < e, a < i, b < c, b < d, c < i, d < f, d < j, e < g, e < h, e < j
fn select_300([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_296([b, a, e, c, g, h, d, f, i])
    } else {
        select_299([b, a, e, d, f, g, h, c, i, j])
    }
}
/// n = 10, i = 4, a < e, a < i, b < c, b < d, b < i, d < f, d < j, e < g, e < h, e < j
fn select_301([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < c {
        select_295([e, b, g, h, i, d, f, c, j])
    } else {
        select_300([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 7, i = 3, a < e, a < g, b < c, b < d, d < g, e < f
fn select_302([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_21([c, d, a, g, e])
    } else {
        select_10([e, b, f, d, g, c])
    }
}
/// n = 9, i = 4, a < d, b < c, b < i, c < f, c < g, d < e, d < h, h < i
fn select_303([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_302([b, d, e, h, c, g, i])
    } else {
        select_33([a, c, f, g, d, e])
    }
}
/// n = 11, i = 4, a < d, b < c, b < i, c < f, c < k, d < e, d < i, d < k, e < g, f < h, i < j
fn select_304([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < c {
        select_10([e, b, g, i, j, c])
    } else {
        select_35([c, a, f, d, h, i, j, e, k])
    }
}
/// n = 11, i = 4, a < d, b < c, b < i, c < e, c < f, c < k, d < g, d < i, d < k, g < h, i < j
fn select_305([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_304([a, b, c, d, g, e, h, f, i, j, k])
    } else {
        select_304([a, b, c, d, g, f, h, e, i, j, k])
    }
}
/// n = 11, i = 4, a < e, a < i, b < c, b < d, c < h, d < i, d < k, e < f, e < g, e < k, i < j
fn select_306([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_303([b, a, e, c, h, f, g, d, i])
    } else {
        select_305([b, a, e, d, f, g, c, h, i, j, k])
    }
}
/// n = 11, i = 4, a < f, a < i, b < c, b < d, b < e, e < i, e < k, f < g, f < h, f < k, i < j
fn select_307([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_306([a, b, c, e, f, g, h, d, i, j, k])
    } else {
        select_306([a, b, d, e, f, g, h, c, i, j, k])
    }
}
/// n = 13, i = 5, a < d, a < k, b < c, b < l, c < g, c < h, c < k, d < e, d < f, d < i, h < j, i < l, l < m
fn select_308([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_301([a, c, g, h, d, j, e, f, k, i])
    } else {
        select_307([b, d, e, f, i, c, g, k, l, m, h])
    }
}
/// n = 13, i = 5, a < i, b < d, b < i, b < l, c < e, c < k, d < h, d < k, e < f, e < g, e < j, j < l, l < m
fn select_309([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < d {
        select_291([a, b, c, e, f, g, i, j, d, h, k, l, m])
    } else {
        select_308([c, b, d, e, f, g, h, a, j, i, k, l, m])
    }
}
/// n = 8, i = 1, a < g, a < h, b < c, b < d, b < e, b < f
fn select_310([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_5([b, g, h])
    } else {
        select_99([a, c, d, e, f])
    }
}
/// n = 8, i = 1, b < h, c < d, c < e, c < f, c < g
fn select_311([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_100([a, c, d, e, f, g, b])
    } else {
        select_310([b, c, d, e, f, g, a, h])
    }
}
/// n = 10, i = 1, b < j, c < h, c < i, d < e, d < f, d < g
fn select_312([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_217([a, b, c, d, h, i, j])
    } else {
        select_311([a, b, d, c, e, f, g, j])
    }
}
/// n = 13, i = 3, a < d, a < e, a < h, b < c, b < f, b < h, f < g, f < i, g < l, h < m, i < j, i < k
fn select_313([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_312([c, g, i, a, d, e, h, j, k, l])
    } else {
        select_124([d, e, b, c, h, f, g, i, m])
    }
}
/// n = 12, i = 3, a < c, a < d, b < e, e < f, e < g, e < h, g < l, h < i, h < j, h < k
fn select_314([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_311([f, g, h, a, i, j, k, l])
    } else {
        select_294([a, b, c, d, e, f, g, l, h])
    }
}
/// n = 9, i = 3, c < e, d < e, e < f, e < g, e < h, e < i
fn select_315([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_140([a, c, d, b, e, f, g, h, i])
    } else {
        select_140([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 8, i = 4, a < f, b < f, c < d, c < e, c < h, f < g, f < h
fn select_316([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_62([c, f, g, e, d, h])
    } else {
        select_196([e, a, b, d, f])
    }
}
/// n = 10, i = 5, a < g, b < g, c < d, d < e, d < f, d < j, g < h, g < i, g < j
fn select_317([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_316([a, b, d, e, f, g, i, j])
    } else {
        select_293([c, g, h, i, d, f, j])
    }
}
/// n = 13, i = 5, a < g, a < j, b < g, b < j, c < d, d < e, d < f, d < j, g < h, g < i, j < k, j < l, j < m
fn select_318([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < g {
        select_315([e, f, a, b, j, g, k, l, m])
    } else {
        select_317([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 14, i = 5, a < g, a < j, b < g, b < j, c < d, c < j, d < e, d < f, d < n, g < h, g < i, j < k, j < l, j < n, l < m
fn select_319([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < d {
        select_314([g, c, h, i, j, k, l, d, e, f, n, m])
    } else {
        select_318([a, b, c, d, e, f, g, h, i, j, k, l, n])
    }
}
/// n = 15, i = 5, a < g, a < m, b < g, b < h, c < d, c < h, d < e, d < f, d < l, g < j, g < k, h < i, h < l, h < m, l < n, m < o
fn select_320([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < a {
        select_313([d, h, i, e, f, a, m, l, g, j, k, o, n])
    } else {
        select_319([a, b, c, d, e, f, g, j, k, h, i, m, o, l])
    }
}
/// n = 6, i = 2, a < f, b < f, c < d, d < e
fn select_321([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_3([b, d, e])
    } else {
        select_162([a, b, c, d, f])
    }
}
/// n = 8, i = 3, a < f, b < e, b < h, c < e, c < h, d < f, d < h, f < g
fn select_322([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_238([a, d, c, e, h, f])
    } else {
        select_321([b, c, d, f, g, e])
    }
}
/// n = 9, i = 4, a < g, a < i, b < g, b < i, c < f, c < g, d < e, e < f, e < i, g < h
fn select_323([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_242([b, c, d, e, g, h, f, i])
    } else {
        select_162([a, b, c, f, i])
    }
}
/// n = 10, i = 4, a < g, a < j, b < f, b < i, c < g, c < i, c < j, d < e, d < g, e < i, f < j, g < h
fn select_324([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_322([a, b, e, c, f, g, h, i])
    } else {
        select_323([a, c, d, b, f, e, g, h, j])
    }
}
/// n = 7, i = 4, a < g, b < f, b < g, c < f, c < g, d < e, d < g
fn select_325([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_40([a, e, b, c, f, g])
    } else {
        select_21([a, f, d, e, g])
    }
}
/// n = 9, i = 4, a < g, b < e, b < f, c < f, c < g, d < e, d < g, d < h, f < h, g < i
fn select_326([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_58([b, a, c, d, e, f, g, i])
    } else {
        select_31([d, f, a, e, h])
    }
}
/// n = 11, i = 5, a < h, a < k, b < e, b < g, c < g, c < h, c < k, d < f, d < k, e < k, f < h, f < i, g < i, h < j
fn select_327([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_325([a, d, e, c, g, f, k])
    } else {
        select_326([a, b, c, f, e, g, h, i, j])
    }
}
/// n = 11, i = 5, a < i, a < k, b < e, b < h, c < h, c < i, c < k, d < f, d < k, e < k, f < g, f < i, i < j
fn select_328([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_324([a, b, c, f, g, e, i, j, h, k])
    } else {
        select_327([a, b, c, d, e, f, h, i, g, j, k])
    }
}
/// n = 8, i = 3, a < e, b < c, b < d, b < h, d < f, e < g, e < h
fn select_329([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_47([c, a, d, f, e])
    } else {
        select_62([b, e, g, c, d, h])
    }
}
/// n = 10, i = 3, a < d, a < i, a < j, b < c, b < g, b < h, c < e, c < i, c < j, d < f, d < g, d < h
fn select_330([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_91([e, a, d, i, j])
    } else {
        select_91([f, b, c, g, h])
    }
}
/// n = 10, i = 3, a < d, a < h, a < i, b < c, b < e, b < j, d < g, d < j, e < f, e < h, e < i
fn select_331([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_153([c, a, e, f, h, i, d])
    } else {
        select_330([a, b, e, d, f, g, c, j, h, i])
    }
}
/// n = 10, i = 3, a < f, a < i, a < j, b < c, b < d, b < e, b < h, e < i, e < j, f < g, f < h
fn select_332([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < e {
        select_329([a, b, d, c, f, e, g, h])
    } else {
        select_331([a, b, d, f, e, c, g, i, j, h])
    }
}
/// n = 8, i = 2, a < d, a < e, a < h, b < c, b < f, b < h, f < g
fn select_333([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_73([b, a, d, e, c, h])
    } else {
        select_174([a, b, c, e, f, g, h])
    }
}
/// n = 9, i = 4, a < f, a < i, b < g, b < i, c < f, c < g, c < i, d < e, d < g, d < i, g < h
fn select_334([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_325([b, a, c, d, e, f, i])
    } else {
        select_41([e, a, b, c, g, h, i])
    }
}
/// n = 12, i = 5, a < h, a < k, b < e, b < f, b < k, c < g, c < h, c < k, d < f, d < h, d < k, e < g, h < i, h < j, k < l
fn select_335([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_333([h, b, f, i, j, k, l, e])
    } else {
        select_334([e, a, c, d, f, g, k, l, h])
    }
}
/// n = 13, i = 5, a < f, a < k, b < g, b < k, c < e, c < f, c < k, d < g, d < j, d < k, e < j, e < l, g < h, g < i, g < l, k < m
fn select_336([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_332([c, g, h, i, a, e, j, l, f, k])
    } else {
        select_335([b, c, d, a, e, f, j, g, h, i, k, m])
    }
}
/// n = 14, i = 6, a < j, a < m, b < i, b < j, b < m, c < e, c < h, c < m, d < f, d < j, e < g, e < i, f < h, f < m, j < k, j < l, m < n
fn select_337([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < j {
        select_328([a, e, b, d, g, f, h, i, m, n, j])
    } else {
        select_336([f, a, c, b, e, h, j, k, l, i, m, g, n])
    }
}
/// n = 16, i = 6, a < e, a < j, b < j, b < k, c < h, c < j, c < k, d < f, d < i, d < k, e < i, f < g, f < h, f < o, j < m, j < n, k < l, k < o, o < p
fn select_338([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < e {
        select_320([a, b, d, f, g, h, j, k, l, m, n, o, e, p, i])
    } else {
        select_337([b, c, d, a, f, e, g, i, h, j, m, n, k, l])
    }
}
/// n = 16, i = 6, a < m, b < e, b < h, c < h, c < i, c < m, d < f, d < l, d < m, e < l, f < g, f < i, f < o, h < j, h < k, m < n, m < o, o < p
fn select_339([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < a {
        select_309([e, d, c, f, h, j, k, g, l, a, i, m, n])
    } else {
        select_338([b, a, c, d, e, f, g, i, l, h, m, n, j, k, o, p])
    }
}
/// n = 16, i = 6, a < m, b < f, b < i, c < g, c < l, c < m, d < e, d < i, d < m, f < l, g < h, g < o, i < j, i < k, m < n, m < o, o < p
fn select_340([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if e < g {
        select_278([a, e, b, c, f, i, j, k, g, m, l, h, n, o])
    } else {
        select_339([a, b, d, c, f, g, h, i, e, j, k, l, m, n, o, p])
    }
}
/// n = 7, i = 3, b < d, b < g, c < e, e < f, e < g
fn select_341([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_47([a, b, e, f, g])
    } else {
        select_52([a, c, b, d, e, g])
    }
}
/// n = 7, i = 3, c < g, d < e, e < f, e < g
fn select_342([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_146([b, a, d, c, e, f])
    } else {
        select_341([b, c, d, a, e, f, g])
    }
}
/// n = 7, i = 3, a < g, b < d, c < e, c < g, e < f
fn select_343([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_7([a, e, f, g, b, d])
    } else {
        select_17([a, c, b, d, g, e, f])
    }
}
/// n = 8, i = 3, a < d, b < g, b < h, c < e, c < g, e < f, e < h
fn select_344([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_174([b, e, f, g, a, d, h])
    } else {
        select_17([b, c, a, d, g, e, f])
    }
}
/// n = 8, i = 3, a < e, b < g, b < h, c < d, c < f, c < g, f < h
fn select_345([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_343([b, a, c, e, d, f, g])
    } else {
        select_344([a, b, c, e, f, d, g, h])
    }
}
/// n = 8, i = 3, c < g, c < h, d < e, d < f, d < g, f < h
fn select_346([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_345([a, c, d, e, b, f, g, h])
    } else {
        select_345([b, c, d, e, a, f, g, h])
    }
}
/// n = 8, i = 3, d < g, d < h, e < f, e < g, f < h
fn select_347([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_342([b, c, d, a, e, f, g])
    } else {
        select_346([b, c, d, e, a, f, g, h])
    }
}
/// n = 9, i = 4, a < h, a < i, b < f, c < e, c < g, d < f, d < i, e < i, f < g, f < h
fn select_348([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_40([a, b, d, e, f, i])
    } else {
        select_147([a, c, f, e, g, h])
    }
}
/// n = 8, i = 4, a < h, b < g, b < h, c < f, d < e, d < g, e < h, f < g
fn select_349([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_191([b, c, a, d, e, f, h])
    } else {
        select_187([b, d, f, a, e, g])
    }
}
/// n = 9, i = 4, a < g, b < e, b < h, c < g, c < h, d < f, d < i, f < h, g < i
fn select_350([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_348([b, a, d, c, f, g, i, e, h])
    } else {
        select_349([a, c, d, b, e, f, h, g])
    }
}
/// n = 4, i = 2, c < d
fn select_351([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_46([a, c, d, b])
    } else {
        select_46([b, c, d, a])
    }
}
/// n = 7, i = 3, a < d, b < f, b < g, c < e, e < f, e < g
fn select_352([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_351([b, d, c, e])
    } else {
        select_9([a, b, f, g, d])
    }
}
/// n = 8, i = 3, b < h, c < f, c < g, d < e, e < f, e < g, e < h
fn select_353([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_352([a, c, d, b, e, f, g])
    } else {
        select_247([c, b, d, a, e, h, f, g])
    }
}
/// n = 8, i = 4, a < h, b < e, b < g, c < f, d < f, f < g, f < h
fn select_354([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_147([a, b, f, e, g, h])
    } else {
        select_196([a, c, d, e, f])
    }
}
/// n = 10, i = 4, a < i, a < j, b < f, b < h, c < e, c < g, d < f, d < g, f < j, g < h, g < i
fn select_355([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_353([e, a, b, d, g, f, h, i])
    } else {
        select_354([a, c, b, d, e, f, g, j])
    }
}
/// n = 10, i = 4, a < f, a < j, b < h, b < i, c < e, c < g, d < g, d < h, g < i, h < j
fn select_356([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_350([b, c, d, a, e, f, h, g, j])
    } else {
        select_355([a, b, c, d, e, h, g, i, f, j])
    }
}
/// n = 9, i = 4, a < e, b < f, c < f, c < i, d < f, d < i, e < i, f < g, f < h
fn select_357([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_5([a, g, h])
    } else {
        select_40([b, e, c, d, i, f])
    }
}
/// n = 9, i = 4, a < h, a < i, b < e, c < f, d < f, d < i, e < i, f < g, f < h
fn select_358([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_7([b, f, g, e, a, h])
    } else {
        select_357([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 6, i = 3, b < f, c < e, d < e, d < f
fn select_359([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < c {
        select_47([a, c, d, f, e])
    } else {
        select_47([a, b, d, e, f])
    }
}
/// n = 6, i = 3, a < f, b < f, c < e, d < e, d < f
fn select_360([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_21([b, c, d, e, f])
    } else {
        select_162([a, b, c, e, f])
    }
}
/// n = 8, i = 4, a < h, b < g, b < h, c < e, d < f, e < h, f < g
fn select_361([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_359([a, b, c, f, e, g])
    } else {
        select_360([a, b, d, e, f, h])
    }
}
/// n = 9, i = 4, a < h, a < i, b < f, c < g, c < i, d < e, d < g, f < i, g < h
fn select_362([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_358([a, b, d, c, f, g, e, h, i])
    } else {
        select_361([a, c, b, d, f, e, g, i])
    }
}
/// n = 9, i = 2, a < d, a < i, b < c, b < i, c < e, c < f, d < g, d < h
fn select_363([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_9([a, c, e, f, i])
    } else {
        select_9([b, d, g, h, i])
    }
}
/// n = 7, i = 3, c < f, c < g, d < e, d < f, e < g
fn select_364([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_359([b, a, c, d, f, e])
    } else {
        select_130([b, c, e, a, f, g])
    }
}
/// n = 13, i = 5, a < i, a < m, b < e, b < m, c < g, c < j, d < g, d < h, e < f, e < h, g < i, g < m, h < j, i < k, i < l
fn select_365([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_363([a, b, e, i, f, h, k, l, m])
    } else {
        select_364([a, f, c, d, h, g, j])
    }
}
/// n = 13, i = 5, a < g, a < j, b < h, b < k, c < e, c < g, d < h, d < i, e < f, e < i, h < j, i < k, j < l, j < m
fn select_366([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_362([b, a, d, e, f, g, i, k, h])
    } else {
        select_365([a, c, b, d, e, f, h, i, j, k, l, m, g])
    }
}
/// n = 14, i = 5, a < j, b < h, b < k, c < e, c < f, d < h, d < i, e < l, f < g, f < i, h < j, h < l, i < k, j < m, j < n
fn select_367([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < a {
        select_356([e, b, f, d, g, a, i, h, k, l])
    } else {
        select_366([a, b, c, d, f, g, e, h, i, j, k, m, n])
    }
}
/// n = 14, i = 5, a < k, b < i, c < i, c < j, d < e, d < f, e < l, f < g, f < h, h < j, i < k, i < l, k < m, k < n
fn select_368([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < b {
        select_347([a, e, g, c, h, b, j, i])
    } else {
        select_367([a, c, d, b, e, f, g, i, h, k, j, l, m, n])
    }
}
/// n = 7, i = 3, b < e, b < f, c < d, c < f, c < g, e < g
fn select_369([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_71([a, b, c, d, f, e])
    } else {
        select_73([e, c, d, f, a, g])
    }
}
/// n = 8, i = 3, c < g, c < h, d < e, d < g, e < f, e < h
fn select_370([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < c {
        select_95([a, b, e, f, c, g, h])
    } else {
        select_287([a, b, c, d, g, e, f, h])
    }
}
/// n = 9, i = 4, b < f, b < i, c < d, c < e, c < h, e < i, f < g, f < h
fn select_371([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_369([a, f, c, d, g, h, e])
    } else {
        select_370([a, d, e, b, f, h, i, g])
    }
}
/// n = 9, i = 3, a < i, b < d, b < f, c < e, c < f, e < i, f < g, f < h
fn select_372([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_92([d, e, f, a, g, h])
    } else {
        select_263([a, b, c, d, e, i, f])
    }
}
/// n = 8, i = 2, a < h, b < h, c < d, d < e, d < f, d < g
fn select_373([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_104([b, d, a, e, f, g])
    } else {
        select_21([b, c, a, d, h])
    }
}
/// n = 9, i = 4, a < f, b < d, b < i, c < e, c < i, d < h, e < g, e < h, f < g, f < i
fn select_374([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_236([a, b, e, d, f, g, h])
    } else {
        select_11([b, c, f, d, e, g, i])
    }
}
/// n = 11, i = 4, a < e, a < g, b < d, b < g, c < f, c < g, d < h, e < i, f < h, f < i, g < j, g < k
fn select_375([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_373([e, f, b, g, d, j, k, i])
    } else {
        select_374([b, a, c, e, f, d, h, i, g])
    }
}
/// n = 12, i = 5, a < d, a < h, b < g, b < h, c < e, c < g, d < l, e < f, e < k, f < j, g < j, g < l, h < i, h < k
fn select_376([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_372([d, h, c, i, g, e, f, k, l])
    } else {
        select_375([a, e, b, f, d, g, h, j, l, i, k])
    }
}
/// n = 12, i = 5, a < i, b < d, b < e, c < f, c < i, d < l, e < g, e < j, f < h, f < j, h < k, i < k, i < l
fn select_377([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_371([d, c, e, g, a, f, h, j, i])
    } else {
        select_376([b, a, c, d, f, h, i, e, g, k, j, l])
    }
}
/// n = 5, i = 3, c < e, d < e
fn select_378([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_15([c, d, a, b, e])
    } else {
        select_15([c, d, b, a, e])
    }
}
/// n = 6, i = 4, c < f, d < f, e < f
fn select_379([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_378([a, b, d, e, f])
    } else {
        select_196([b, c, d, e, f])
    }
}
/// n = 8, i = 4, c < g, c < h, d < f, e < f, f < g, f < h
fn select_380([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_148([b, c, f, a, g, h])
    } else {
        select_379([b, c, a, d, e, f])
    }
}
/// n = 7, i = 4, b < f, c < f, d < e, d < g, f < g
fn select_381([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < e {
        select_52([a, c, d, e, f, g])
    } else {
        select_18([a, b, c, f])
    }
}
/// n = 6, i = 4, b < f, c < f, d < e
fn select_382([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_60([b, c, a, d, e, f])
    } else {
        select_15([b, c, e, a, f])
    }
}
/// n = 7, i = 4, c < g, d < f, e < f, f < g
fn select_383([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < a {
        select_381([b, d, e, c, a, f, g])
    } else {
        select_382([b, d, e, a, c, f])
    }
}
/// n = 8, i = 4, c < g, d < f, d < h, e < f, e < g, g < h
fn select_384([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_380([a, b, d, c, e, g, f, h])
    } else {
        select_383([a, b, c, d, e, f, g])
    }
}
/// n = 7, i = 3, b < g, c < f, c < g, d < e, d < g, e < f
fn select_385([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_360([b, c, a, d, e, g])
    } else {
        select_238([b, c, e, a, f, g])
    }
}
/// n = 9, i = 4, b < i, c < g, d < g, d < i, e < f, e < h, f < i, g < h
fn select_386([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < f {
        select_384([a, b, d, e, c, f, g, h])
    } else {
        select_385([a, b, d, f, c, g, i])
    }
}
/// n = 6, i = 3, c < e, d < e, e < f
fn select_387([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_27([a, c, d, b, e, f])
    } else {
        select_27([b, c, d, a, e, f])
    }
}
/// n = 9, i = 4, b < f, c < f, c < g, d < e, d < i, e < g, f < i, g < h
fn select_388([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_381([a, b, c, d, e, f, i])
    } else {
        select_387([a, b, c, e, g, h])
    }
}
/// n = 8, i = 4, b < f, c < f, c < g, d < e, e < g, g < h
fn select_389([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_45([a, b, c, g, h, f])
    } else {
        select_52([a, d, c, f, e, g])
    }
}
/// n = 9, i = 4, b < g, c < f, d < g, d < i, e < f, e < g, f < i, g < h
fn select_390([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < b {
        select_388([a, c, e, d, b, f, g, h, i])
    } else {
        select_389([a, c, e, b, d, f, g, h])
    }
}
/// n = 9, i = 3, a < e, b < d, b < f, c < e, c < f, e < i, f < g, f < h
fn select_391([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_91([d, f, a, g, h])
    } else {
        select_11([b, a, c, d, e, i, f])
    }
}
/// n = 9, i = 4, a < d, b < f, c < e, c < g, e < i, f < g, f < i, g < h
fn select_392([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_9([e, g, a, h, i])
    } else {
        select_83([a, b, c, d, e, f, i, g])
    }
}
/// n = 12, i = 4, a < e, a < g, b < d, b < h, c < f, e < l, f < g, f < h, f < l, g < k, h < i, h < j
fn select_393([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_391([a, b, f, d, g, h, i, j, k])
    } else {
        select_392([b, c, a, d, e, f, g, k, l])
    }
}
/// n = 14, i = 5, a < g, a < i, b < e, b < f, c < h, d < g, d < h, e < m, f < j, f < n, g < n, h < i, h < j, h < m, i < k, i < l
fn select_394([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < h {
        select_390([e, c, a, f, d, g, h, j, n])
    } else {
        select_393([b, a, c, g, e, h, f, i, k, l, j, m])
    }
}
/// n = 14, i = 5, a < i, b < h, b < j, c < e, c < f, d < h, d < i, e < n, f < g, f < k, h < k, i < j, i < n, j < l, j < m
fn select_395([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < i {
        select_386([e, a, b, d, f, g, h, k, i])
    } else {
        select_394([b, c, a, d, e, f, h, i, j, g, l, m, n, k])
    }
}
/// n = 14, i = 5, a < k, b < i, c < f, c < g, d < e, d < i, e < j, f < l, g < h, g < j, i < k, i < l, k < m, k < n
fn select_396([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < a {
        select_377([b, c, d, f, g, e, h, a, i, j, k, l])
    } else {
        select_395([b, a, c, d, f, g, h, e, i, k, j, m, n, l])
    }
}
/// n = 14, i = 5, a < k, b < j, c < f, c < g, d < e, d < j, f < l, g < h, g < i, j < k, j < l, k < m, k < n
fn select_397([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < e {
        select_368([a, b, d, c, f, g, i, h, j, e, k, l, m, n])
    } else {
        select_396([a, b, c, d, e, f, g, i, j, h, k, l, m, n])
    }
}
/// n = 8, i = 3, c < f, d < e, d < f, f < g, f < h
fn select_398([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_185([a, c, d, e, b, f, g, h])
    } else {
        select_185([b, c, d, e, a, f, g, h])
    }
}
/// n = 10, i = 4, a < g, a < h, b < f, b < h, c < d, c < e, c < h, f < g, h < i, h < j
fn select_399([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_223([f, a, c, e, g, h, i, j])
    } else {
        select_398([d, e, a, b, f, h, i, j])
    }
}
/// n = 10, i = 4, a < g, a < h, b < f, b < h, c < d, c < e, c < h, h < i, h < j
fn select_400([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_399([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_399([b, a, c, d, e, g, f, h, i, j])
    }
}
/// n = 10, i = 4, a < g, a < j, b < f, b < h, c < d, c < e, c < h, h < i, h < j
fn select_401([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_132([d, e, f, h, i, a, g, j])
    } else {
        select_400([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 11, i = 4, a < k, b < j, c < g, c < h, d < e, d < f, d < h, h < i, h < j, h < k
fn select_402([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_401([a, c, d, e, f, g, b, h, i, k])
    } else {
        select_401([b, c, d, e, f, g, a, h, i, j])
    }
}
/// n = 9, i = 2, a < e, a < g, b < c, b < d, b < f, f < h, g < i
fn select_403([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_77([c, d, f, a, h])
    } else {
        select_333([b, a, e, c, d, g, i, f])
    }
}
/// n = 9, i = 2, a < g, b < f, c < d, c < e, c < h, h < i
fn select_404([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_403([a, c, d, e, g, h, b, i, f])
    } else {
        select_403([b, c, d, e, f, h, a, i, g])
    }
}
/// n = 7, i = 1, c < g, d < e, d < f
fn select_405([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_119([a, c, d, e, f, g, b])
    } else {
        select_119([b, c, d, e, f, g, a])
    }
}
/// n = 10, i = 3, a < c, a < d, b < e, b < j, d < j, e < f, e < g, e < h, h < i
fn select_406([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_405([f, g, h, a, c, d, i])
    } else {
        select_125([c, d, b, e, f, g, h, j])
    }
}
/// n = 8, i = 2, a < e, a < h, b < c, b < d, b < f, b < h, f < g
fn select_407([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_77([c, d, f, a, g])
    } else {
        select_78([a, b, c, d, e, f, h])
    }
}
/// n = 10, i = 3, a < c, a < h, b < d, b < h, c < g, d < e, d < f, d < i, i < j
fn select_408([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_31([b, c, g, h, d])
    } else {
        select_407([a, d, e, f, h, i, j, c])
    }
}
/// n = 10, i = 3, a < h, b < d, c < e, c < h, e < f, e < g, e < i, i < j
fn select_409([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < a {
        select_406([b, c, d, a, e, f, g, i, j, h])
    } else {
        select_408([a, c, b, e, f, g, d, h, i, j])
    }
}
/// n = 10, i = 3, b < e, c < d, c < f, f < g, f < h, f < i, i < j
fn select_410([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_404([b, d, f, g, h, a, e, i, j])
    } else {
        select_409([a, b, c, e, f, g, h, d, i, j])
    }
}
/// n = 9, i = 1, a < g, a < h, a < i, b < c, b < d, b < e, b < f
fn select_411([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_98([b, g, h, i])
    } else {
        select_99([a, c, d, e, f])
    }
}
/// n = 9, i = 1, b < h, b < i, c < d, c < e, c < f, c < g
fn select_412([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_100([a, c, d, e, f, g, b])
    } else {
        select_411([b, c, d, e, f, g, a, h, i])
    }
}
/// n = 8, i = 3, b < h, c < e, c < h, d < e, d < h, e < f, e < g
fn select_413([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_207([b, e, a, f, g, h])
    } else {
        select_40([a, b, c, d, h, e])
    }
}
/// n = 11, i = 3, c < e, c < f, d < e, d < f, e < j, e < k, f < g, f < h, f < i
fn select_414([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_412([b, e, f, a, g, h, i, j, k])
    } else {
        select_413([b, a, c, d, e, j, k, f])
    }
}
/// n = 12, i = 3, c < h, c < l, d < e, d < h, e < f, e < g, e < l, g < i, h < j, h < k
fn select_415([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < c {
        select_124([a, b, e, f, g, c, h, l, i])
    } else {
        select_414([a, b, c, d, h, e, f, g, l, j, k])
    }
}
/// n = 7, i = 2, a < e, b < f, b < g, c < d, c < f, c < g
fn select_416([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_32([c, a, b, e, d])
    } else {
        select_63([c, b, d, a, e, f, g])
    }
}
/// n = 8, i = 3, a < h, b < e, b < h, c < d, c < f, c < g, c < h, e < f, e < g
fn select_417([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_62([c, e, a, d, f, g])
    } else {
        select_137([a, b, c, d, e, h])
    }
}
/// n = 9, i = 4, a < g, b < f, b < h, b < i, c < f, c < g, c < h, c < i, d < e, d < g, e < h, e < i
fn select_418([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_417([a, d, c, f, e, h, i, g])
    } else {
        select_71([b, c, d, e, g, f])
    }
}
/// n = 9, i = 4, b < g, b < h, b < i, c < f, c < g, c < h, c < i, d < e, d < f, e < h, e < i
fn select_419([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_416([f, e, b, g, a, h, i])
    } else {
        select_418([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 7, i = 3, a < e, a < f, a < g, b < d, b < f, b < g, c < d, c < e, c < g
fn select_420([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_31([a, c, d, f, g])
    } else {
        select_31([b, c, e, f, g])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, b < g, c < d, c < e, c < g
fn select_421([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_46([b, d, a, e])
    } else {
        select_420([a, b, c, e, d, f, g])
    }
}
/// n = 10, i = 3, a < d, a < f, a < j, b < e, b < g, b < j, c < e, c < f, c < g, c < j, g < h, g < i
fn select_422([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_179([a, b, e, f, g, h, i, j])
    } else {
        select_421([b, c, a, d, f, e, j])
    }
}
/// n = 7, i = 3, c < e, d < e, e < f, e < g
fn select_423([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_196([b, a, c, d, e])
    } else {
        select_91([b, e, a, f, g])
    }
}
/// n = 11, i = 5, a < g, b < f, b < h, c < g, c < h, d < g, d < h, e < f, e < h, e < k, g < k, h < i, h < j
fn select_424([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_181([a, b, e, g, h, i, j, k])
    } else {
        select_423([a, f, c, d, h, i, j])
    }
}
/// n = 12, i = 5, b < f, b < i, b < l, c < g, c < i, d < g, d < i, e < f, e < h, e < i, e < l, g < h, g < l, i < j, i < k
fn select_425([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_422([g, b, e, a, f, h, i, j, k, l])
    } else {
        select_424([a, b, c, d, e, f, g, i, j, k, h])
    }
}
/// n = 12, i = 5, b < i, b < l, c < g, c < i, c < l, d < g, d < h, d < i, d < l, e < f, e < i, f < h, f < l, i < j, i < k
fn select_426([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_419([a, c, d, f, b, h, g, i, l])
    } else {
        select_425([a, c, b, e, d, g, f, h, i, j, k, l])
    }
}
/// n = 15, i = 5, b < f, b < l, c < i, c < l, d < g, d < i, d < l, e < g, e < h, e < i, e < l, f < h, i < j, i < k, k < m, l < n, l < o
fn select_427([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_415([a, g, b, c, i, j, k, l, m, n, o, f])
    } else {
        select_426([a, c, d, e, b, f, g, h, l, n, o, i])
    }
}
/// n = 15, i = 5, b < j, c < f, c < i, d < g, d < i, d < j, e < g, e < h, e < i, e < j, f < h, i < k, i < l, j < m, j < n, n < o
fn select_428([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < b {
        select_410([a, f, d, g, h, i, k, l, b, j])
    } else {
        select_427([a, c, b, d, e, f, g, h, j, m, n, i, o, k, l])
    }
}
/// n = 16, i = 6, a < m, b < f, b < h, c < h, c < i, c < m, d < e, d < g, f < l, g < i, g < l, g < m, h < j, h < k, m < n, m < o, o < p
fn select_429([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < g {
        select_402([a, f, d, h, j, k, e, g, i, l, m])
    } else {
        select_428([e, a, b, c, g, f, i, l, h, m, j, k, n, o, p])
    }
}
/// n = 16, i = 6, a < m, b < g, b < i, c < f, c < h, d < e, d < i, d < m, g < l, h < l, h < m, i < j, i < k, m < n, m < o, o < p
fn select_430([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if e < h {
        select_397([a, e, b, c, f, g, i, j, k, h, m, l, n, o])
    } else {
        select_429([a, b, d, c, f, g, h, i, e, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 6, a < g, a < o, b < m, c < l, c < m, d < h, d < i, e < f, e < i, e < m, h < l, i < j, i < k, m < n, m < o, o < p
fn select_431([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if c < a {
        select_340([b, d, c, e, f, h, a, g, i, j, k, l, m, n, o, p])
    } else {
        select_430([b, d, a, e, f, g, h, c, i, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 6, b < g, b < o, c < j, c < n, d < h, d < i, e < f, e < i, e < j, h < n, i < k, i < l, j < m, j < o, o < p
fn select_432([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < a {
        select_235([b, d, c, e, f, g, h, i, j, a, m, k, l, n, o, p])
    } else {
        select_431([b, a, c, d, e, f, g, h, i, k, l, n, j, m, o, p])
    }
}
/// n = 8, i = 3, a < d, a < f, b < c, b < e, b < h, e < g, f < h
fn select_433([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_130([c, a, e, g, d, f])
    } else {
        select_148([d, f, b, c, e, h])
    }
}
/// n = 7, i = 3, a < c, b < d, c < g, d < e, d < f
fn select_434([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if g < d {
        select_8([b, g])
    } else {
        select_32([a, d, e, f, c])
    }
}
/// n = 9, i = 3, a < d, a < h, a < i, b < c, b < e, c < f, d < g, e < h, e < i
fn select_435([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < e {
        select_434([a, b, d, c, e, f, g])
    } else {
        select_35([a, b, d, e, g, c, f, h, i])
    }
}
/// n = 9, i = 3, a < d, a < g, b < c, b < g, c < e, d < f, g < h, g < i
fn select_436([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_221([d, b, f, g, c, h, i])
    } else {
        select_269([b, a, d, c, e, f, g])
    }
}
/// n = 10, i = 3, a < e, a < h, a < j, b < c, b < d, b < h, c < j, d < f, e < g, h < i
fn select_437([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < h {
        select_435([a, b, d, e, c, f, g, h, j])
    } else {
        select_436([a, b, d, e, f, g, h, c, i])
    }
}
/// n = 10, i = 3, a < d, b < h, b < j, c < e, c < f, c < h, e < j, f < g, h < i
fn select_438([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_433([a, c, e, d, f, b, g, h])
    } else {
        select_437([b, c, e, f, a, g, d, h, i, j])
    }
}
/// n = 8, i = 4, a < d, a < e, b < f, c < f, f < g, f < h
fn select_439([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_73([a, f, g, h, e, d])
    } else {
        select_196([e, b, c, d, f])
    }
}
/// n = 8, i = 4, a < f, b < f, c < d, c < h, d < e, f < g, f < h
fn select_440([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_31([c, f, g, d, h])
    } else {
        select_14([a, b, e])
    }
}
/// n = 9, i = 4, a < d, a < i, b < f, c < f, d < e, f < g, f < h, f < i
fn select_441([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_29([g, h, a, i])
    } else {
        select_440([b, c, a, d, e, f, h, i])
    }
}
/// n = 9, i = 4, a < i, b < e, c < f, d < f, f < g, f < h, f < i
fn select_442([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_439([b, c, d, a, e, f, g, h])
    } else {
        select_441([a, c, d, b, e, f, g, h, i])
    }
}
/// n = 8, i = 4, a < e, b < f, c < d, c < g, f < g, g < h
fn select_443([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_7([a, c, d, e, g, h])
    } else {
        select_52([e, b, c, d, f, g])
    }
}
/// n = 6, i = 4, a < f, b < f, c < f, d < e
fn select_444([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_193([a, b, c, e])
    } else {
        select_8([d, f])
    }
}
/// n = 8, i = 5, a < e, b < g, c < g, d < f, f < h, g < h
fn select_445([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_16([a, g, d, f, e, h])
    } else {
        select_444([b, c, e, d, f, g])
    }
}
/// n = 9, i = 5, b < e, c < g, d < f, f < h, g < h, h < i
fn select_446([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_443([b, d, g, a, e, f, h, i])
    } else {
        select_445([b, a, c, d, e, f, g, h])
    }
}
/// n = 10, i = 5, a < j, b < f, c < g, d < e, d < h, g < h, h < i, h < j
fn select_447([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_442([a, b, d, g, f, h, e, i, j])
    } else {
        select_446([a, b, c, d, f, e, g, h, i])
    }
}
/// n = 12, i = 5, a < f, b < e, c < j, c < l, d < g, g < h, h < i, h < j, i < l, j < k
fn select_448([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_438([b, c, h, e, i, a, f, j, k, l])
    } else {
        select_447([c, b, d, a, f, e, g, h, i, j])
    }
}
/// n = 13, i = 5, a < f, b < e, c < j, c < k, d < g, g < h, g < i, h < k, i < j, j < l, k < m
fn select_449([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_448([a, b, c, d, e, f, g, h, i, k, m, j])
    } else {
        select_448([a, b, c, d, e, f, g, i, h, j, l, k])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < e, b < i, d < h, e < f, e < g
fn select_450([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_207([d, e, a, f, g, h])
    } else {
        select_63([a, b, c, d, h, e, i])
    }
}
/// n = 10, i = 2, a < c, a < j, b < d, b < e, b < f, b < j, d < i, e < h, f < g
fn select_451([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_450([a, b, c, e, d, f, i, h, j])
    } else {
        select_450([a, b, c, e, f, d, g, h, j])
    }
}
/// n = 9, i = 3, a < d, b < e, b < f, c < e, c < f, e < i, f < g, f < h
fn select_452([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_207([e, f, a, g, h, i])
    } else {
        select_11([a, b, c, d, e, i, f])
    }
}
/// n = 12, i = 3, a < h, b < d, b < k, c < e, c < h, e < f, e < g, e < k, f < j, g < i, h < l
fn select_453([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_451([b, e, d, a, f, g, i, j, h, k])
    } else {
        select_452([b, a, c, d, h, e, f, g, l])
    }
}
/// n = 12, i = 3, a < i, b < d, b < k, c < e, c < i, e < f, e < g, e < h, e < k, h < j, i < l
fn select_454([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_453([a, b, c, d, e, f, h, i, j, g, k, l])
    } else {
        select_453([a, b, c, d, e, g, h, i, j, f, k, l])
    }
}
/// n = 8, i = 2, a < e, a < h, b < c, b < d, b < h, d < f, e < g
fn select_455([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_42([c, d, a, f])
    } else {
        select_63([b, a, c, e, g, d, h])
    }
}
/// n = 8, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_456([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_42([d, e, a, g])
    } else {
        select_11([b, a, c, d, f, h, e])
    }
}
/// n = 9, i = 3, a < d, b < g, b < i, c < e, c < g, e < f, e < i, g < h
fn select_457([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_455([b, e, f, a, g, d, h, i])
    } else {
        select_456([b, a, c, d, e, g, f, h])
    }
}
/// n = 11, i = 4, a < e, b < h, b < k, c < f, c < h, d < f, d < h, f < g, f < k, h < i, h < j
fn select_458([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_180([a, f, b, g, h, i, j, k, e])
    } else {
        select_357([a, b, c, d, e, h, i, j, f])
    }
}
/// n = 12, i = 4, a < e, b < g, b < i, c < h, c < i, d < f, d < i, f < g, f < h, h < j, i < k, i < l
fn select_459([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_457([a, c, f, e, b, g, h, j, i])
    } else {
        select_458([a, c, b, d, e, f, g, i, k, l, h])
    }
}
/// n = 10, i = 4, a < e, b < h, c < f, c < h, d < f, d < h, f < g, h < i, h < j
fn select_460([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_183([a, f, b, g, h, i, j])
    } else {
        select_197([b, a, c, d, e, f, g])
    }
}
/// n = 11, i = 5, a < e, b < i, c < g, d < f, d < i, f < h, g < h, g < i, i < j, i < k
fn select_461([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_460([a, b, d, g, e, f, h, i, j, k])
    } else {
        select_195([b, a, c, f, e, g, h, i])
    }
}
/// n = 13, i = 5, a < f, b < e, b < j, c < i, c < j, d < g, g < h, g < j, h < i, i < k, j < l, j < m
fn select_462([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_459([a, b, c, g, f, h, e, i, j, k, l, m])
    } else {
        select_461([a, c, d, b, f, e, g, h, j, l, m])
    }
}
/// n = 14, i = 5, a < f, b < e, b < n, c < i, c < l, d < g, g < h, g < i, h < l, i < j, i < k, i < n, l < m
fn select_463([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_454([h, b, c, e, i, j, k, a, l, f, n, m])
    } else {
        select_462([b, a, c, d, f, e, g, h, l, i, m, j, k])
    }
}
/// n = 14, i = 5, a < f, b < e, b < n, c < j, c < l, d < g, g < h, g < i, g < j, i < l, j < k, j < n, l < m
fn select_464([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_449([a, b, c, d, e, f, g, h, i, l, j, m, k])
    } else {
        select_463([a, b, c, d, e, f, g, i, j, h, k, l, m, n])
    }
}
/// n = 9, i = 2, a < d, a < e, a < i, b < c, b < f, b < i, f < g, f < h
fn select_465([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_91([c, f, a, g, h])
    } else {
        select_78([b, a, d, e, c, f, i])
    }
}
/// n = 9, i = 2, a < e, a < f, a < i, b < c, b < d, b < g, b < i, g < h
fn select_466([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < g {
        select_333([a, b, d, e, f, c, g, i])
    } else {
        select_465([a, b, d, e, f, g, c, h, i])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < g, c < d, c < f, c < g
fn select_467([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_47([d, b, a, f, g])
    } else {
        select_187([a, b, c, d, e, g])
    }
}
/// n = 9, i = 3, a < e, a < f, b < f, b < g, c < d, c < g, g < h, g < i
fn select_468([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_91([d, g, a, h, i])
    } else {
        select_467([b, c, a, e, d, f, g])
    }
}
/// n = 10, i = 3, a < e, b < d, b < i, b < j, c < f, c < i, f < g, f < h, f < j
fn select_469([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_466([b, f, g, h, d, i, a, e, j])
    } else {
        select_468([b, c, a, e, d, i, f, g, h])
    }
}
/// n = 7, i = 3, a < d, b < f, b < g, c < e, c < f, e < g
fn select_470([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_71([b, c, a, d, e, f])
    } else {
        select_10([b, e, f, a, d, g])
    }
}
/// n = 8, i = 4, a < d, b < f, c < e, c < g, e < h, f < g, f < h
fn select_471([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_7([a, c, g, d, e, h])
    } else {
        select_52([d, b, c, e, f, g])
    }
}
/// n = 10, i = 4, a < d, b < f, b < g, c < e, c < h, e < g, e < j, f < h, f < j, h < i
fn select_472([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_89([a, e, f, g, h, i, j])
    } else {
        select_471([a, c, b, d, f, e, g, j])
    }
}
/// n = 10, i = 4, a < d, b < f, b < h, c < e, c < i, e < g, e < h, f < i, i < j
fn select_473([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_470([a, b, e, d, g, h, f])
    } else {
        select_472([a, b, c, d, e, f, h, i, j, g])
    }
}
/// n = 11, i = 4, a < e, b < d, b < h, b < j, c < f, c < h, f < g, f < j, f < k, h < i, h < k
fn select_474([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < d {
        select_469([a, f, b, g, e, h, d, i, j, k])
    } else {
        select_473([a, b, c, e, f, d, g, j, h, i])
    }
}
/// n = 7, i = 3, a < d, b < e, c < e, e < f, e < g
fn select_475([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_5([a, f, g])
    } else {
        select_18([d, b, c, e])
    }
}
/// n = 8, i = 3, a < h, b < d, b < g, c < e, e < f, e < g, e < h
fn select_476([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_174([b, e, f, d, a, h, g])
    } else {
        select_475([b, a, c, d, e, f, h])
    }
}
/// n = 8, i = 3, b < d, b < h, c < e, e < f, e < g, e < h
fn select_477([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_174([b, e, g, d, f, a, h])
    } else {
        select_476([a, b, c, d, e, g, h, f])
    }
}
/// n = 9, i = 3, b < d, b < h, b < i, c < e, e < f, e < h, e < i, f < g
fn select_478([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_218([a, b, e, f, g, h, i])
    } else {
        select_149([a, c, b, d, e, h, i])
    }
}
/// n = 9, i = 3, b < d, b < h, b < i, c < e, e < f, e < g, e < h, e < i
fn select_479([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_478([a, b, c, d, e, f, g, h, i])
    } else {
        select_478([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 12, i = 5, a < c, b < d, c < f, c < h, c < k, d < e, d < g, d < k, d < l, g < j, h < i, h < l
fn select_480([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_477([e, g, a, j, c, f, k, h])
    } else {
        select_479([f, h, b, i, d, e, k, g, l])
    }
}
/// n = 8, i = 4, a < e, a < h, b < d, b < h, c < f, f < g, f < h
fn select_481([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_467([b, a, f, g, e, d, h])
    } else {
        select_27([a, c, d, e, f, g])
    }
}
/// n = 12, i = 6, a < i, a < j, b < d, c < e, d < g, d < i, d < l, e < f, e < j, g < h, j < k, j < l
fn select_482([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < j {
        select_481([a, g, c, h, i, e, f, j])
    } else {
        select_150([f, j, b, k, d, i, g, l])
    }
}
/// n = 13, i = 6, a < h, a < k, b < e, c < d, c < h, d < f, e < g, e < k, e < l, g < i, g < m, h < j, h < l, h < m
fn select_483([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < d {
        select_480([b, c, e, h, j, k, d, g, i, f, l, m])
    } else {
        select_482([a, b, c, e, d, f, g, i, k, h, j, l])
    }
}
/// n = 14, i = 6, a < h, a < k, b < e, b < l, c < d, c < h, d < f, e < g, e < k, e < n, g < i, h < j, h < l, h < n, l < m
fn select_484([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if l < g {
        select_474([d, h, b, j, f, e, k, l, m, n, g])
    } else {
        select_483([a, b, c, d, e, f, g, h, i, j, k, n, l])
    }
}
/// n = 9, i = 5, a < i, b < e, c < g, d < f, f < h, f < i, g < h, g < i
fn select_485([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_195([a, b, c, f, e, g, h, i])
    } else {
        select_195([a, b, d, g, e, f, h, i])
    }
}
/// n = 7, i = 3, a < g, b < f, c < e, c < g, d < e, d < f, d < g
fn select_486([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < b {
        select_122([a, b, d, e, f, g])
    } else {
        select_47([a, c, d, f, e])
    }
}
/// n = 8, i = 4, a < h, b < e, c < g, c < h, d < f, f < g, f < h
fn select_487([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_486([a, b, c, f, g, e, h])
    } else {
        select_378([a, c, d, e, f])
    }
}
/// n = 10, i = 5, a < e, b < i, b < j, c < f, c < i, d < g, f < j, g < h, g < j, h < i
fn select_488([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_485([b, a, c, d, e, g, f, h, j])
    } else {
        select_487([b, a, c, g, e, h, f, i])
    }
}
/// n = 9, i = 2, a < i, b < g, b < h, b < i, c < d, c < e, c < g, c < h, e < f
fn select_489([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < c {
        select_9([b, c, d, e, i])
    } else {
        select_405([a, d, e, b, g, h, f])
    }
}
/// n = 9, i = 4, a < i, b < f, b < g, c < f, c < g, c < i, d < e, d < g, e < i, g < h
fn select_490([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_297([a, f, d, g, e, h])
    } else {
        select_107([a, b, c, d, e, f, g, i])
    }
}
/// n = 9, i = 4, a < i, b < g, b < i, c < e, c < g, d < f, d < g, f < i, g < h
fn select_491([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_52([a, e, d, f, b, g])
    } else {
        select_490([a, c, b, d, f, e, g, h, i])
    }
}
/// n = 11, i = 4, a < e, b < i, b < j, b < k, c < g, c < k, d < f, d < g, f < k, g < h, g < i, g < j
fn select_492([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_489([f, b, g, h, a, e, i, j, k])
    } else {
        select_491([b, c, a, d, e, f, g, h, k])
    }
}
/// n = 12, i = 5, a < f, b < e, b < k, b < l, c < i, c < l, d < g, g < h, g < i, h < l, i < j, i < k
fn select_493([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_488([a, c, b, d, f, e, g, h, l, i])
    } else {
        select_492([a, b, c, g, f, h, i, j, e, k, l])
    }
}
/// n = 8, i = 5, a < h, b < g, c < g, d < e, e < f, g < h
fn select_494([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_16([a, g, d, e, h, f])
    } else {
        select_196([a, b, c, f, g])
    }
}
/// n = 7, i = 4, a < g, b < e, c < d, c < g, e < f
fn select_495([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_54([a, c, e, f, d])
    } else {
        select_54([d, a, b, e, g])
    }
}
/// n = 8, i = 5, b < h, c < e, d < f, e < h, f < g
fn select_496([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_494([b, a, c, d, f, g, e, h])
    } else {
        select_495([b, d, e, a, f, g, h])
    }
}
/// n = 12, i = 5, b < h, b < i, c < f, d < e, d < i, e < g, f < h, f < i, i < j, i < k, i < l
fn select_497([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_496([a, b, c, d, f, e, g, h])
    } else {
        select_227([a, b, f, d, e, h, i, j, k, l])
    }
}
/// n = 10, i = 2, a < d, a < j, b < c, b < e, b < j, e < f, e < g, e < h, e < i
fn select_498([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_100([c, e, f, g, h, i, a])
    } else {
        select_154([a, b, d, e, f, g, h, i, c, j])
    }
}
/// n = 9, i = 4, a < e, a < i, b < f, b < i, c < f, c < i, d < f, d < i, f < g, f < h
fn select_499([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_9([a, f, g, h, i])
    } else {
        select_196([e, b, c, d, i])
    }
}
/// n = 10, i = 4, a < i, a < j, b < e, b < h, b < j, c < f, c < j, d < f, d < j, f < g, f < h, f < i
fn select_500([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_174([b, f, g, e, a, i, h])
    } else {
        select_499([b, a, c, d, e, f, g, i, j])
    }
}
/// n = 13, i = 4, a < m, b < e, b < h, b < l, c < f, c < h, d < f, d < h, f < g, f < l, f < m, h < i, h < j, h < k
fn select_501([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_498([f, b, e, g, h, a, i, j, k, l])
    } else {
        select_500([a, b, c, d, e, f, g, l, m, h])
    }
}
/// n = 8, i = 5, a < h, b < e, c < g, d < f, f < h, g < h
fn select_502([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_191([a, b, f, c, g, e, h])
    } else {
        select_382([a, d, e, c, g, f])
    }
}
/// n = 8, i = 3, c < e, d < e, e < f, e < g, e < h
fn select_503([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_184([a, c, d, b, e, f, g, h])
    } else {
        select_184([b, c, d, a, e, f, g, h])
    }
}
/// n = 12, i = 5, a < f, b < h, b < l, c < e, c < h, d < g, f < l, g < h, g < l, h < i, h < j, h < k
fn select_504([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_502([b, c, a, d, e, g, f, l])
    } else {
        select_503([a, e, b, g, h, i, j, k])
    }
}
/// n = 14, i = 5, a < f, b < e, b < j, b < n, c < h, c < j, d < g, g < h, g < j, h < i, h < n, j < k, j < l, j < m
fn select_505([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < f {
        select_501([a, b, c, g, e, h, i, j, k, l, m, n, f])
    } else {
        select_504([a, c, b, d, e, f, g, j, k, l, m, h])
    }
}
/// n = 15, i = 6, a < e, a < h, a < m, b < i, b < j, c < h, c < k, d < f, d < i, f < g, h < j, h < n, h < o, i < k, i < n, i < o, k < l, k < m
fn select_506([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_497([e, b, c, d, f, h, g, j, i, k, n, o])
    } else {
        select_505([f, a, c, b, e, g, i, k, l, h, j, n, o, m])
    }
}
/// n = 15, i = 6, a < e, a < h, a < n, b < h, b < l, c < i, c < k, d < f, d < i, f < g, h < k, h < o, i < j, i < l, i < o, l < m, l < n
fn select_507([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < h {
        select_493([f, a, b, c, e, g, i, j, l, m, n, h])
    } else {
        select_506([a, c, b, d, e, f, g, h, i, k, l, m, n, j, o])
    }
}
/// n = 15, i = 6, a < e, a < o, b < g, b < l, c < i, c < k, d < f, d < i, f < h, g < k, g < n, i < j, i < l, i < n, l < m, l < o
fn select_508([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < a {
        select_484([c, b, d, f, g, h, a, i, e, j, k, l, m, n])
    } else {
        select_507([a, b, c, d, e, f, h, g, i, j, k, l, m, o, n])
    }
}
/// n = 15, i = 6, a < e, a < o, b < g, b < m, c < i, c < l, d < f, d < i, f < h, g < l, i < j, i < k, i < m, m < n, m < o
fn select_509([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < g {
        select_464([f, a, b, c, e, h, i, k, j, m, n, g, l, o])
    } else {
        select_508([a, b, c, d, e, f, g, h, i, k, l, m, n, j, o])
    }
}
/// n = 7, i = 1, e < f, e < g
fn select_510([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_405([c, d, a, e, f, g, b])
    } else {
        select_405([c, d, b, e, f, g, a])
    }
}
/// n = 8, i = 2, c < d, c < e, c < f, f < g, f < h
fn select_511([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_91([b, c, d, e, f])
    } else {
        select_510([a, b, d, e, f, g, h])
    }
}
/// n = 8, i = 2, a < c, b < d, d < e, d < f, d < g, d < h
fn select_512([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_99([a, e, f, g, h])
    } else {
        select_3([c, b, d])
    }
}
/// n = 9, i = 2, b < i, c < d, c < i, d < e, d < f, d < g, d < h
fn select_513([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_512([a, c, b, d, e, f, g, h])
    } else {
        select_106([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 7, i = 3, b < e, b < g, c < d, c < f, d < g, e < f
fn select_514([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < d {
        select_16([a, c, b, e, d, f])
    } else {
        select_10([d, b, a, e, f, g])
    }
}
/// n = 10, i = 3, b < e, b < f, c < d, c < f, c < j, e < j, f < g, f < h, f < i
fn select_515([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_513([a, e, c, f, d, g, h, i, j])
    } else {
        select_514([a, b, c, d, e, j, f])
    }
}
/// n = 10, i = 3, b < e, b < i, c < d, c < f, c < j, e < j, f < g, f < h, f < i
fn select_516([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_511([a, d, f, g, h, b, e, i])
    } else {
        select_515([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 7, i = 3, a < e, b < e, c < f, d < f, f < g
fn select_517([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < a {
        select_321([a, b, d, f, g, e])
    } else {
        select_162([c, d, b, e, f])
    }
}
/// n = 8, i = 3, a < d, b < c, b < h, c < f, d < e, d < h, f < g
fn select_518([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_3([a, f, g])
    } else {
        select_31([b, d, e, c, h])
    }
}
/// n = 9, i = 3, a < f, a < h, b < f, b < g, c < d, d < e, d < g, d < h, f < i
fn select_519([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_518([c, a, b, d, e, f, i, h])
    } else {
        select_518([c, b, a, d, e, f, i, g])
    }
}
/// n = 10, i = 3, a < f, a < i, b < f, b < g, b < h, c < d, d < e, d < g, d < h, d < i, f < j
fn select_520([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_42([e, b, g, h])
    } else {
        select_519([a, b, c, d, e, f, h, i, j])
    }
}
/// n = 10, i = 3, b < f, b < j, c < f, c < g, c < h, d < e, e < g, e < h, e < j, f < i
fn select_521([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_517([a, d, b, c, e, f, i])
    } else {
        select_520([b, c, d, e, a, f, g, h, j, i])
    }
}
/// n = 7, i = 3, a < g, b < e, c < d, c < f, d < g, e < f
fn select_522([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_16([b, a, c, d, e, g])
    } else {
        select_31([c, e, a, d, f])
    }
}
/// n = 9, i = 4, a < i, b < g, c < f, c < g, d < e, d < g, d < i, f < i, g < h
fn select_523([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < e {
        select_490([a, b, d, c, f, e, g, h, i])
    } else {
        select_522([a, e, c, f, b, g, i])
    }
}
/// n = 11, i = 4, a < h, a < k, b < g, c < f, c < h, c < j, d < e, d < g, d < h, g < j, g < k, h < i
fn select_524([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_521([e, a, c, b, g, h, f, j, i, k])
    } else {
        select_523([b, a, c, d, e, f, h, i, g])
    }
}
/// n = 12, i = 4, a < l, b < h, b < k, c < f, c < g, c < j, d < e, d < g, d < h, g < i, g < k, h < j, h < l
fn select_525([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_516([e, b, c, f, h, g, a, i, k, j])
    } else {
        select_524([a, b, c, d, e, f, h, g, i, j, l])
    }
}
/// n = 10, i = 4, a < g, b < f, b < g, c < d, c < e, d < j, e < h, e < i, f < h, f < i, f < j
fn select_526([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_48([e, a, d, b, g, f, j])
    } else {
        select_284([a, f, c, g, e, d, j, h, i])
    }
}
/// n = 8, i = 4, b < f, c < f, c < g, d < e, d < h, e < g, f < h
fn select_527([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_381([a, b, c, d, e, f, h])
    } else {
        select_378([a, b, c, e, g])
    }
}
/// n = 8, i = 4, b < g, b < h, c < f, d < e, d < h, e < g, f < h
fn select_528([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < f {
        select_527([a, c, b, d, e, f, g, h])
    } else {
        select_52([a, f, d, e, b, h])
    }
}
/// n = 11, i = 5, a < h, a < i, b < e, b < h, c < d, c < f, d < k, e < i, f < g, f < j, h < j, h < k
fn select_529([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_526([e, a, c, d, f, h, i, g, j, k])
    } else {
        select_528([d, a, f, b, e, g, i, h])
    }
}
/// n = 8, i = 3, a < f, a < g, a < h, b < e, b < h, c < d, c < f, c < h, e < g
fn select_530([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_47([d, b, a, f, h])
    } else {
        select_263([a, c, b, d, e, g, h])
    }
}
/// n = 8, i = 4, a < g, a < h, b < f, c < e, d < f, d < h, e < h, f < g
fn select_531([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_40([a, b, d, e, f, h])
    } else {
        select_21([a, c, f, e, g])
    }
}
/// n = 9, i = 4, b < g, b < h, b < i, c < e, c < h, d < f, d < i, e < i, f < g, f < h
fn select_532([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_530([b, c, f, a, e, g, i, h])
    } else {
        select_531([b, a, c, d, e, f, g, i])
    }
}
/// n = 9, i = 3, b < e, b < i, c < f, c < i, d < e, d < f, d < i, f < g, f < h
fn select_533([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_138([a, c, b, d, e, i])
    } else {
        select_413([a, b, c, d, f, g, h, i])
    }
}
/// n = 9, i = 3, a < d, b < f, b < i, c < e, c < f, e < i, f < g, f < h
fn select_534([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_51([b, a, c, d, e, i])
    } else {
        select_84([a, e, b, f, g, h, i])
    }
}
/// n = 11, i = 4, a < g, a < k, b < f, b < g, b < k, c < d, c < e, d < j, e < h, e < i, e < k, f < h, f < i, f < j
fn select_535([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_533([d, a, e, b, g, f, h, i, k])
    } else {
        select_534([a, f, c, g, d, e, h, i, j])
    }
}
/// n = 12, i = 5, a < h, a < i, a < k, b < e, b < h, c < d, c < f, d < l, e < i, e < k, f < g, f < j, f < k, h < j, h < l
fn select_536([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_532([d, a, f, b, g, e, i, k, h])
    } else {
        select_535([e, a, c, d, f, h, i, g, j, l, k])
    }
}
/// n = 12, i = 5, a < f, a < i, b < d, b < i, b < k, c < e, c < g, e < l, f < k, g < h, g < j, g < k, i < j, i < l
fn select_537([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < f {
        select_529([a, b, c, e, d, g, h, i, f, j, l])
    } else {
        select_536([b, a, c, e, f, g, h, i, d, j, k, l])
    }
}
/// n = 13, i = 5, a < e, a < i, b < d, b < i, b < j, c < f, c < g, f < m, g < h, g < j, g < l, i < l, i < m, j < k
fn select_538([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < e {
        select_525([f, a, g, b, d, h, j, i, k, l, e, m])
    } else {
        select_537([a, b, c, d, f, e, g, h, i, l, j, m])
    }
}
/// n = 9, i = 3, a < e, a < i, b < e, b < f, b < i, c < d, c < f, c < i, f < g, f < h
fn select_539([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_137([a, b, c, d, e, i])
    } else {
        select_84([d, a, b, f, g, h, i])
    }
}
/// n = 12, i = 3, a < e, b < e, b < f, b < g, c < d, c < f, c < g, f < k, f < l, g < h, g < i, g < j
fn select_540([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_412([d, f, g, a, h, i, j, k, l])
    } else {
        select_539([a, b, c, d, e, f, k, l, g])
    }
}
/// n = 12, i = 3, a < h, b < h, b < i, b < l, c < d, c < e, c < i, e < f, e < g, e < l, i < j, i < k
fn select_541([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < b {
        select_511([a, d, e, f, g, b, i, l])
    } else {
        select_540([a, b, c, d, h, i, e, f, g, l, j, k])
    }
}
/// n = 10, i = 4, a < g, b < e, c < d, c < h, d < g, d < j, e < f, e < h, e < j, h < i
fn select_542([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_94([f, d, h, a, i, j])
    } else {
        select_23([a, b, c, d, e, f, g, j, h])
    }
}
/// n = 11, i = 4, a < g, b < e, c < d, c < g, c < h, d < k, e < f, e < h, e < k, h < i, h < j
fn select_543([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_219([f, a, c, d, h, i, j, k, g])
    } else {
        select_293([b, c, d, g, e, f, h])
    }
}
/// n = 12, i = 4, a < g, b < h, c < g, c < i, d < e, e < f, e < h, e < i, h < l, i < j, i < k
fn select_544([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < c {
        select_542([a, d, b, c, e, f, g, h, l, i])
    } else {
        select_543([a, d, c, b, e, f, g, i, j, k, h])
    }
}
/// n = 13, i = 4, b < g, c < g, c < j, c < m, d < e, e < f, e < h, e < j, h < i, h < m, j < k, j < l
fn select_545([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_541([b, c, e, f, h, a, i, g, j, k, l, m])
    } else {
        select_544([b, a, c, d, e, f, g, h, j, k, l, i])
    }
}
/// n = 10, i = 2, a < d, a < e, a < j, b < c, b < f, b < j, e < i, f < g, f < h
fn select_546([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_174([b, a, d, c, e, i, j])
    } else {
        select_179([a, b, c, d, f, g, h, j])
    }
}
/// n = 10, i = 3, a < e, a < f, b < d, b < g, c < e, c < f, c < g, f < j, g < h, g < i
fn select_547([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_268([b, a, c, d, e, f, j])
    } else {
        select_223([a, c, b, d, e, g, h, i])
    }
}
/// n = 11, i = 3, a < g, b < d, b < h, b < k, c < e, c < g, c < h, e < f, e < k, h < i, h < j
fn select_548([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_546([e, b, d, f, a, h, i, j, g, k])
    } else {
        select_547([a, b, c, d, g, e, h, i, j, f])
    }
}
/// n = 13, i = 4, a < d, a < i, a < j, b < c, b < e, c < i, d < g, d < h, d < m, e < f, e < j, e < m, j < k, j < l
fn select_549([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < g {
        select_548([c, e, a, f, d, h, i, j, k, l, m])
    } else {
        select_300([a, b, c, e, d, f, g, h, i, m])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, d < e, d < f, d < g
fn select_550([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_98([a, e, f, g])
    } else {
        select_42([c, b, d, h])
    }
}
/// n = 8, i = 2, b < d, b < h, c < e, c < h, e < f, e < g
fn select_551([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_238([a, c, b, d, h, e])
    } else {
        select_550([b, c, d, e, a, f, g, h])
    }
}
/// n = 9, i = 3, a < e, a < f, b < f, b < g, c < d, c < e, c < g, g < h, g < i
fn select_552([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_73([b, c, d, e, f, g])
    } else {
        select_179([a, c, d, f, g, h, i, e])
    }
}
/// n = 10, i = 3, a < f, b < f, b < g, b < j, c < d, c < e, c < g, e < j, g < h, g < i
fn select_553([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_551([d, e, b, a, g, h, i, j])
    } else {
        select_552([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 9, i = 4, a < e, a < i, b < e, b < h, b < i, c < f, c < i, d < f, d < i, f < g, f < h
fn select_554([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_29([a, g, b, h])
    } else {
        select_11([b, c, d, e, f, g, i])
    }
}
/// n = 8, i = 3, a < d, a < h, b < c, b < g, c < e, c < h, d < f, d < g
fn select_555([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_42([e, a, d, h])
    } else {
        select_42([f, b, c, g])
    }
}
/// n = 11, i = 5, a < f, a < g, a < k, b < g, b < h, c < g, c < h, d < e, d < h, e < f, g < j, h < i, h < k
fn select_556([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_554([e, a, b, c, f, h, i, k, g])
    } else {
        select_555([a, d, h, g, i, j, e, k])
    }
}
/// n = 13, i = 5, a < i, b < h, b < i, b < k, c < f, c < i, d < e, d < f, e < h, f < g, f < k, i < j, k < l, k < m
fn select_557([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_553([e, b, f, g, a, h, k, l, m, i])
    } else {
        select_556([b, a, c, d, e, h, i, f, g, j, k])
    }
}
/// n = 14, i = 5, b < f, b < j, b < k, c < f, c < g, d < e, d < g, e < j, f < i, f < n, g < h, g < k, g < n, k < l, k < m
fn select_558([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < a {
        select_549([b, d, e, f, g, h, a, i, j, k, l, m, n])
    } else {
        select_557([a, b, c, d, e, g, h, j, f, i, k, l, m])
    }
}
/// n = 15, i = 5, b < f, b < j, b < m, c < f, c < g, d < e, d < g, e < j, f < i, g < h, g < k, g < m, k < l, m < n, m < o
fn select_559([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < f {
        select_545([a, e, b, c, g, h, j, k, l, m, n, o, f])
    } else {
        select_558([a, b, c, d, e, f, g, h, i, j, m, n, o, k])
    }
}
/// n = 16, i = 6, a < d, a < g, a < l, b < e, b < i, c < f, c < i, e < k, f < h, g < k, g < n, i < j, i < l, i < n, l < m, n < o, n < p
fn select_560([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < g {
        select_538([f, a, b, d, h, e, i, j, g, l, m, n, k])
    } else {
        select_559([d, g, c, b, e, f, i, j, h, k, l, m, n, o, p])
    }
}
/// n = 7, i = 3, b < d, c < e, e < f, e < g
fn select_561([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_434([a, c, b, e, f, g, d])
    } else {
        select_34([b, c, a, d, e, f, g])
    }
}
/// n = 10, i = 3, b < d, c < e, e < f, e < g, e < h, h < i, h < j
fn select_562([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_511([a, b, e, f, g, h, i, j])
    } else {
        select_561([a, b, c, d, e, f, g])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, d < e, e < g
fn select_563([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < e {
        select_51([a, c, d, f, e, g])
    } else {
        select_162([b, c, a, g, f])
    }
}
/// n = 7, i = 4, a < g, b < g, c < f, d < e
fn select_564([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_563([c, a, b, d, e, g, f])
    } else {
        select_563([d, a, b, c, f, g, e])
    }
}
/// n = 11, i = 4, a < f, b < g, b < h, c < g, c < h, d < e, d < h, h < i, h < j, h < k
fn select_565([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_564([b, c, a, d, e, f, g])
    } else {
        select_227([a, b, c, d, e, g, h, i, j, k])
    }
}
/// n = 11, i = 4, a < f, b < g, b < k, c < g, c < h, d < e, d < h, h < i, h < j, h < k
fn select_566([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < b {
        select_562([e, a, c, f, h, i, j, b, g, k])
    } else {
        select_565([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 9, i = 2, b < d, b < i, c < e, c < i, e < f, e < g, e < h
fn select_567([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_238([a, c, b, d, i, e])
    } else {
        select_106([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 10, i = 3, b < d, b < g, c < e, c < g, e < f, g < h, g < i, g < j
fn select_568([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_125([a, d, c, g, h, i, j, e])
    } else {
        select_567([a, e, b, f, g, h, i, j, d])
    }
}
/// n = 12, i = 5, a < h, a < i, b < h, b < i, c < d, c < e, d < g, e < f, e < k, e < l, i < j, i < k, i < l
fn select_569([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < e {
        select_568([h, i, c, j, d, g, e, f, k, l])
    } else {
        select_565([d, a, b, e, f, g, h, i, j, k, l])
    }
}
/// n = 14, i = 5, a < h, a < i, b < h, b < i, b < k, c < d, c < e, d < g, e < f, e < k, e < n, i < j, i < n, k < l, k < m
fn select_570([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < i {
        select_566([d, a, b, e, f, g, h, k, l, m, i])
    } else {
        select_569([a, b, c, d, e, f, g, h, i, j, k, n])
    }
}
/// n = 8, i = 3, b < d, b < h, c < e, e < f, e < h, f < g
fn select_571([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_45([a, b, e, f, g, h])
    } else {
        select_52([a, c, b, d, e, h])
    }
}
/// n = 11, i = 3, b < d, c < e, e < f, e < g, f < k, g < h, g < i, g < j
fn select_572([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_311([a, f, g, b, h, i, j, k])
    } else {
        select_571([a, b, c, d, e, f, k, g])
    }
}
/// n = 11, i = 3, b < d, c < e, e < f, e < g, e < h, h < i, h < j, h < k
fn select_573([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_572([a, b, c, d, e, f, h, i, j, k, g])
    } else {
        select_572([a, b, c, d, e, g, h, i, j, k, f])
    }
}
/// n = 9, i = 1, a < i, b < h, c < d, c < e, c < f, c < g
fn select_574([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_310([a, c, d, e, f, g, b, i])
    } else {
        select_310([b, c, d, e, f, g, a, h])
    }
}
/// n = 9, i = 1, c < i, d < e, d < f, d < g, d < h
fn select_575([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_574([a, c, d, e, f, g, h, i, b])
    } else {
        select_574([b, c, d, e, f, g, h, i, a])
    }
}
/// n = 10, i = 4, a < e, a < j, b < f, c < d, c < g, c < j, f < g, f < j, g < h, g < i
fn select_576([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_179([a, c, d, e, g, h, i, j])
    } else {
        select_149([e, b, c, d, f, g, j])
    }
}
/// n = 9, i = 4, a < d, a < i, b < f, c < e, c < i, e < g, e < h, f < g, f < h, f < i
fn select_577([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_16([d, c, b, f, e, i])
    } else {
        select_22([a, e, b, f, d, g, h, i])
    }
}
/// n = 10, i = 5, a < d, a < j, b < e, c < f, e < h, e < j, f < g, f < i, f < j, h < i
fn select_578([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_577([a, c, e, d, h, f, g, i, j])
    } else {
        select_83([a, b, f, d, g, e, h, j])
    }
}
/// n = 11, i = 5, a < d, a < k, b < f, c < e, e < g, e < i, e < k, f < h, f < i, f < k, i < j
fn select_579([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < g {
        select_576([a, c, f, h, d, e, i, g, j, k])
    } else {
        select_578([a, c, b, d, e, f, h, g, i, k])
    }
}
/// n = 14, i = 5, a < d, b < f, c < e, e < g, e < i, e < j, f < h, f < i, f < j, i < n, j < k, j < l, j < m
fn select_580([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < a {
        select_575([g, h, i, j, a, k, l, m, n])
    } else {
        select_579([a, b, c, d, e, f, g, h, i, n, j])
    }
}
/// n = 14, i = 5, a < d, b < f, c < e, c < i, e < h, e < l, e < n, f < g, f < i, f < l, i < j, i < k, i < n, l < m
fn select_581([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < e {
        select_573([g, a, c, d, i, j, k, e, h, l, n])
    } else {
        select_580([a, b, c, d, e, f, h, g, l, i, j, k, n, m])
    }
}
/// n = 9, i = 2, a < c, b < d, b < e, d < i, e < f, e < g, e < h
fn select_582([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_100([d, e, a, f, g, h, i])
    } else {
        select_10([a, b, c, d, i, e])
    }
}
/// n = 9, i = 2, c < d, c < e, d < i, e < f, e < g, e < h
fn select_583([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_582([a, c, b, d, e, f, g, h, i])
    } else {
        select_582([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 11, i = 3, b < d, b < h, b < k, c < e, c < h, e < f, e < g, e < k, g < i, h < j
fn select_584([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_121([a, f, g, b, d, h, k, i])
    } else {
        select_583([a, d, c, h, e, f, g, k, j])
    }
}
/// n = 9, i = 3, b < d, b < i, c < e, e < f, e < g, e < i, g < h
fn select_585([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_179([b, e, f, d, g, a, h, i])
    } else {
        select_476([a, b, c, d, e, f, i, g])
    }
}
/// n = 10, i = 4, b < e, c < d, c < h, d < g, d < j, e < f, e < h, e < j, h < i
fn select_586([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_45([a, f, c, h, i, d])
    } else {
        select_478([a, d, b, g, e, h, i, f, j])
    }
}
/// n = 12, i = 5, a < d, a < j, b < f, c < e, c < j, e < h, e < i, f < g, f < i, f < j, i < k, j < l
fn select_587([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_585([h, a, b, d, f, g, i, k, j])
    } else {
        select_586([d, b, c, e, f, g, h, j, l, i])
    }
}
/// n = 14, i = 6, a < h, a < j, b < e, c < d, c < h, d < g, e < f, e < j, e < l, h < i, h < l, h < n, j < k, j < n, l < m
fn select_588([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < d {
        select_584([f, j, c, k, h, i, l, d, m, g, n])
    } else {
        select_587([d, b, a, g, h, e, f, i, l, j, m, k])
    }
}
/// n = 16, i = 6, a < h, a < j, b < e, b < l, c < d, c < h, d < g, e < f, e < j, e < o, h < i, h < l, h < o, j < k, l < m, l < n, o < p
fn select_589([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < j {
        select_581([d, a, b, g, e, h, i, f, l, m, n, o, p, j])
    } else {
        select_588([a, b, c, d, e, f, g, h, i, j, k, o, p, l])
    }
}
/// n = 16, i = 6, a < f, a < k, a < l, b < d, b < i, c < e, c < i, d < k, e < g, f < h, f < o, i < j, i < l, i < o, l < m, l < n, o < p
fn select_590([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if d < f {
        select_570([d, a, c, e, i, j, g, k, f, h, l, m, n, o])
    } else {
        select_589([b, a, c, e, f, h, g, i, j, d, k, l, m, n, o, p])
    }
}
/// n = 16, i = 6, a < e, a < l, b < k, b < m, c < f, c < i, d < g, d < i, f < k, g < h, i < j, i < l, i < m, l < p, m < n, m < o
fn select_591([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_560([a, c, d, e, f, g, b, h, i, j, k, l, p, m, n, o])
    } else {
        select_590([b, c, d, f, g, a, h, e, i, j, k, m, n, o, l, p])
    }
}
/// n = 16, i = 6, a < e, a < o, b < i, b < n, c < g, c < j, d < f, d < i, d < j, f < h, g < n, i < l, i < m, j < k, j < o, o < p
fn select_592([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_509([a, c, b, d, e, f, g, h, i, l, m, n, j, k, o])
    } else {
        select_591([a, b, c, d, e, g, f, h, j, k, n, o, i, l, m, p])
    }
}
/// n = 9, i = 2, a < e, a < i, b < f, b < h, c < d, c < h, c < i, f < g
fn select_593([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < b {
        select_179([a, c, d, e, b, f, h, i])
    } else {
        select_208([a, b, e, f, c, d, h, g])
    }
}
/// n = 9, i = 3, a < e, a < i, b < e, b < f, b < h, c < d, c < f, d < h, d < i, f < g
fn select_594([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_6([a, c, e, f, d, g])
    } else {
        select_22([a, b, c, d, e, f, h, i])
    }
}
/// n = 11, i = 3, a < g, a < j, a < k, b < d, b < h, c < e, c < g, c < h, d < k, e < f, e < j, h < i
fn select_595([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < d {
        select_593([a, b, e, f, g, h, i, d, j])
    } else {
        select_594([a, c, b, d, g, h, i, e, k])
    }
}
/// n = 11, i = 3, a < j, a < k, b < e, b < h, c < d, c < f, c < h, e < k, f < g, f < j, h < i
fn select_596([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_172([d, f, b, e, g, a, h, i])
    } else {
        select_595([a, b, c, e, f, g, d, h, i, j, k])
    }
}
/// n = 7, i = 2, b < e, b < f, b < g, c < d, c < f, c < g
fn select_597([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_148([a, b, c, d, f, g])
    } else {
        select_148([a, c, b, e, f, g])
    }
}
/// n = 10, i = 3, a < d, a < f, a < j, b < c, b < e, b < i, d < h, e < g, e < j, f < h, f < i
fn select_598([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_153([d, f, b, c, e, i, h])
    } else {
        select_597([c, a, e, g, d, f, j])
    }
}
/// n = 9, i = 2, a < d, a < i, b < c, b < e, b < i, d < h, e < f, e < g
fn select_599([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_91([c, e, a, f, g])
    } else {
        select_63([b, a, c, d, h, e, i])
    }
}
/// n = 11, i = 3, a < e, a < h, b < c, b < d, b < h, d < f, e < g, g < i, h < j, h < k
fn select_600([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_551([c, d, a, f, h, j, k, e])
    } else {
        select_599([e, b, c, g, h, j, k, i, d])
    }
}
/// n = 12, i = 3, a < e, a < k, b < h, b < l, c < d, c < f, c < h, e < l, f < g, f < k, h < i, h < j
fn select_601([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_598([a, c, d, e, f, b, g, l, h, k])
    } else {
        select_600([b, c, d, f, a, g, e, h, l, i, j])
    }
}
/// n = 13, i = 3, a < f, a < h, b < i, b < l, c < g, c < h, d < e, d < i, g < l, h < m, i < j, i < k
fn select_602([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < d {
        select_596([b, c, a, f, g, d, e, h, m, i, l])
    } else {
        select_601([c, b, d, e, g, a, f, i, j, k, h, l])
    }
}
/// n = 10, i = 2, a < e, a < j, b < c, b < d, b < j, d < f, d < g, e < h, e < i
fn select_603([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_91([c, d, a, f, g])
    } else {
        select_65([b, a, c, e, h, i, d, j])
    }
}
/// n = 10, i = 2, b < f, b < j, c < d, c < e, c < j, e < g, f < h, f < i
fn select_604([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_180([a, c, b, d, f, h, i, j, e])
    } else {
        select_603([b, c, d, e, f, a, g, h, i, j])
    }
}
/// n = 9, i = 3, a < e, a < i, b < f, b < i, c < d, c < f, c < i, f < g, f < h
fn select_605([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_187([b, a, c, d, e, i])
    } else {
        select_84([d, a, b, f, g, h, i])
    }
}
/// n = 10, i = 3, a < g, b < e, b < f, c < d, c < f, c < g, f < h, g < i, g < j
fn select_606([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_77([d, e, f, a, h])
    } else {
        select_605([b, a, c, d, e, g, i, j, f])
    }
}
/// n = 11, i = 3, a < e, b < h, b < k, c < d, c < f, c < h, f < g, f < k, h < i, h < j
fn select_607([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_604([d, b, f, g, a, h, e, i, j, k])
    } else {
        select_606([b, a, c, d, e, f, h, g, i, j])
    }
}
/// n = 7, i = 5, a < g, b < g, c < f, d < e
fn select_608([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_60([a, b, c, e, f, g])
    } else {
        select_60([a, b, d, f, e, g])
    }
}
/// n = 10, i = 5, a < g, b < g, b < h, c < f, d < e, d < h, f < h, h < i, h < j
fn select_609([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_423([a, e, b, f, h, i, j])
    } else {
        select_608([a, b, c, d, e, f, g])
    }
}
/// n = 11, i = 5, a < e, b < g, b < k, c < g, c < h, d < f, f < h, h < i, h < j, h < k
fn select_610([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_609([b, c, d, a, e, f, g, h, i, j])
    } else {
        select_407([b, h, i, j, g, a, e, k])
    }
}
/// n = 7, i = 4, a < f, b < f, b < g, c < f, c < g, d < e, e < g
fn select_611([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_40([a, e, b, c, g, f])
    } else {
        select_1([d])
    }
}
/// n = 8, i = 5, a < g, b < g, b < h, c < f, d < e, e < h, f < h
fn select_612([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_611([a, b, e, c, f, g, h])
    } else {
        select_15([d, g, c, f, e])
    }
}
/// n = 9, i = 3, a < d, b < h, b < i, c < e, e < f, e < h, e < i, f < g
fn select_613([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_64([b, f, a, g, h, i])
    } else {
        select_247([b, a, c, d, e, f, h, i])
    }
}
/// n = 11, i = 5, a < g, b < e, b < k, c < g, c < h, d < f, f < h, h < i, h < k, i < j
fn select_614([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_612([a, c, b, d, f, e, g, h])
    } else {
        select_613([a, b, f, g, h, i, j, e, k])
    }
}
/// n = 12, i = 5, a < g, a < l, b < e, b < k, c < g, c < h, c < i, d < f, f < h, f < i, h < k, i < j, i < l
fn select_615([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_610([b, a, c, d, e, f, g, i, h, j, l])
    } else {
        select_614([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 14, i = 5, a < e, a < n, b < g, b < j, c < g, c < k, d < f, f < h, f < j, h < i, h < k, j < n, k < l, k < m
fn select_616([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < b {
        select_607([a, c, h, i, e, b, j, k, l, m, g])
    } else {
        select_615([c, a, b, d, e, f, g, j, h, i, n, k])
    }
}
/// n = 15, i = 5, a < g, b < e, b < k, c < g, c < l, d < f, f < h, f < i, h < k, i < j, i < l, k < o, l < m, l < n
fn select_617([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < a {
        select_602([b, c, h, i, j, e, a, k, l, m, n, g, o])
    } else {
        select_616([b, a, c, d, e, f, g, i, j, h, l, m, n, k])
    }
}
/// n = 6, i = 1, d < e, d < f
fn select_618([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_94([c, a, d, e, f, b])
    } else {
        select_94([c, b, d, e, f, a])
    }
}
/// n = 6, i = 2, b < f, c < d, c < e, c < f
fn select_619([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_45([a, b, c, d, e, f])
    } else {
        select_45([a, b, c, e, d, f])
    }
}
/// n = 7, i = 2, c < d, c < e, c < f, f < g
fn select_620([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_618([b, d, e, f, a, g])
    } else {
        select_619([b, a, c, d, e, f])
    }
}
/// n = 8, i = 2, c < d, c < e, c < f, c < g, g < h
fn select_621([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_620([a, b, c, e, f, d, g])
    } else {
        select_511([a, b, c, e, f, g, d, h])
    }
}
/// n = 9, i = 3, b < e, c < d, c < f, f < g, f < h, f < i
fn select_622([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_621([a, d, f, g, h, i, b, e])
    } else {
        select_126([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 11, i = 3, a < e, a < g, b < d, b < f, c < f, c < g, f < h, f < i, g < j, g < k
fn select_623([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_185([a, c, b, d, e, f, h, i])
    } else {
        select_185([b, c, a, e, d, g, j, k])
    }
}
/// n = 13, i = 3, a < e, a < h, b < d, b < f, b < g, c < g, c < h, c < k, f < k, g < i, g < j, h < l, h < m
fn select_624([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < h {
        select_189([a, c, b, d, e, f, g, i, j, k])
    } else {
        select_623([a, b, c, d, e, g, h, i, j, l, m])
    }
}
/// n = 9, i = 4, a < e, a < i, b < f, c < d, c < g, c < i, f < g, f < i, g < h
fn select_625([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_83([a, b, c, e, d, f, g, i])
    } else {
        select_19([a, c, f, e, g, d, h, i])
    }
}
/// n = 12, i = 5, a < g, a < h, b < g, b < i, c < f, d < e, d < h, d < i, f < h, f < i, h < l, i < j, i < k
fn select_626([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_625([b, c, d, e, g, f, h, l, i])
    } else {
        select_576([a, c, d, e, g, f, i, j, k, h])
    }
}
/// n = 13, i = 5, a < g, a < h, b < e, b < j, c < g, c < j, c < m, d < f, f < h, f < j, h < i, h < m, j < k, j < l
fn select_627([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_174([c, h, i, g, b, e, m])
    } else {
        select_626([a, c, d, b, e, f, g, h, j, k, l, i])
    }
}
/// n = 15, i = 5, a < g, b < e, b < k, c < g, c < j, c < k, d < f, f < h, f < k, h < i, h < j, j < l, j < m, k < n, k < o
fn select_628([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < a {
        select_624([b, h, c, i, e, a, j, k, l, m, g, n, o])
    } else {
        select_627([a, b, c, d, e, f, g, h, i, k, n, o, j])
    }
}
/// n = 15, i = 5, a < g, b < g, b < m, c < e, c < j, d < f, f < h, f < j, h < i, h < m, j < k, j < l, m < n, m < o
fn select_629([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < b {
        select_622([a, h, c, e, i, j, b, k, l])
    } else {
        select_628([a, c, b, d, e, f, g, h, i, m, j, n, o, k, l])
    }
}
/// n = 15, i = 5, a < h, b < e, b < k, c < h, c < m, d < f, f < g, f < i, f < k, i < j, i < m, k < l, m < n, m < o
fn select_630([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < k {
        select_617([a, b, c, d, e, f, h, g, i, j, k, m, n, o, l])
    } else {
        select_629([a, c, b, d, e, f, h, i, j, k, g, l, m, n, o])
    }
}
/// n = 8, i = 2, a < g, a < h, b < e, b < f, c < d, c < f, c < g, c < h
fn select_631([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_73([c, b, a, e, d, f])
    } else {
        select_63([c, a, d, b, e, g, h])
    }
}
/// n = 8, i = 3, a < e, a < h, b < c, b < d, b < g, d < h, e < f, e < g
fn select_632([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_76([c, a, d, e, h])
    } else {
        select_62([b, e, f, c, d, g])
    }
}
/// n = 9, i = 3, a < c, a < d, a < h, a < i, b < e, b < f, e < i, f < g, f < h
fn select_633([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < c {
        select_631([e, f, a, d, g, h, c, i])
    } else {
        select_632([b, a, d, c, f, g, h, e])
    }
}
/// n = 9, i = 2, a < e, a < i, b < c, b < d, b < f, b < i, f < g, f < h
fn select_634([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_92([c, d, f, a, g, h])
    } else {
        select_78([a, b, c, d, e, f, i])
    }
}
/// n = 10, i = 3, a < e, a < g, b < c, b < d, b < g, b < j, e < f, e < j, g < h, g < i
fn select_635([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_42([d, a, e, g])
    } else {
        select_634([e, b, c, d, f, g, h, i, j])
    }
}
/// n = 10, i = 3, a < d, a < e, a < h, a < j, b < c, b < f, b < h, f < g, f < j, h < i
fn select_636([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < h {
        select_633([a, b, d, e, c, f, g, j, h])
    } else {
        select_635([b, a, d, e, f, g, h, c, i, j])
    }
}
/// n = 11, i = 3, a < g, a < i, b < f, b < h, c < d, c < e, c < h, c < i, h < j, i < k
fn select_637([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_636([c, a, g, d, e, b, f, i, k, h])
    } else {
        select_636([c, b, f, d, e, a, g, h, j, i])
    }
}
/// n = 8, i = 3, a < d, b < f, b < h, c < e, c < f, e < h, f < g
fn select_638([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_45([a, e, b, f, g, h])
    } else {
        select_51([b, a, c, d, e, h])
    }
}
/// n = 11, i = 3, a < d, b < e, b < f, c < e, c < f, e < k, f < g, f < h, f < i, f < j
fn select_639([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_310([e, f, g, h, i, j, a, k])
    } else {
        select_141([a, b, c, d, f, g, h, i, j, e])
    }
}
/// n = 11, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_640([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < g {
        select_638([a, b, c, e, d, f, k, g])
    } else {
        select_639([a, b, c, e, f, g, d, h, i, j, k])
    }
}
/// n = 11, i = 3, c < f, c < g, d < e, d < f, d < g, f < k, g < h, g < i, g < j
fn select_641([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_640([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_640([b, c, d, e, a, f, g, h, i, j, k])
    }
}
/// n = 12, i = 3, c < e, c < i, c < l, d < f, d < i, f < g, f < h, f < l, h < j, i < k
fn select_642([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < c {
        select_118([a, b, f, g, h, c, e, i, l, j])
    } else {
        select_641([a, b, d, c, e, i, f, g, h, l, k])
    }
}
/// n = 13, i = 4, a < e, a < g, b < c, b < d, b < f, b < k, e < j, f < h, f < m, g < i, g < k, g < m, k < l
fn select_643([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_637([e, g, b, c, d, i, j, k, f, l, h])
    } else {
        select_642([c, d, f, a, h, g, i, k, e, l, j, m])
    }
}
/// n = 7, i = 2, a < g, b < f, c < d, c < e, c < g
fn select_644([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_7([b, c, e, f, d, a])
    } else {
        select_416([b, a, c, e, f, d, g])
    }
}
/// n = 7, i = 2, c < g, d < e, d < f, d < g
fn select_645([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_644([c, a, d, e, f, b, g])
    } else {
        select_644([c, b, d, e, f, a, g])
    }
}
/// n = 9, i = 4, a < f, a < g, b < d, c < e, d < f, d < i, e < g, e < i, g < h
fn select_646([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < c {
        select_1([c])
    } else {
        select_242([a, e, b, d, g, h, i, f])
    }
}
/// n = 8, i = 4, a < d, b < f, b < g, c < e, d < f, e < g, g < h
fn select_647([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_15([b, d, c, e, f])
    } else {
        select_3([a, g, h])
    }
}
/// n = 10, i = 5, a < f, a < h, b < f, b < g, c < g, c < h, d < e, e < h, e < j, g < j, h < i
fn select_648([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_646([a, b, d, g, e, f, h, i, j])
    } else {
        select_647([b, c, d, f, e, g, h, i])
    }
}
/// n = 6, i = 2, a < e, b < c, b < d, e < f
fn select_649([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if f < b {
        select_1([f])
    } else {
        select_32([a, b, c, d, e])
    }
}
/// n = 9, i = 4, a < g, a < h, b < d, b < f, c < e, d < g, e < f, e < h, h < i
fn select_650([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_4([a, c, g, e, f])
    } else {
        select_649([a, b, d, f, h, i])
    }
}
/// n = 10, i = 5, a < g, b < g, b < i, c < f, d < e, d < i, e < h, f < h, f < i, i < j
fn select_651([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_648([b, a, d, c, f, g, e, i, j, h])
    } else {
        select_650([b, e, c, a, f, h, g, i, j])
    }
}
/// n = 9, i = 4, a < f, b < g, b < h, c < f, c < h, d < e, d < h, e < g, h < i
fn select_652([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_48([a, e, c, b, g, h, i])
    } else {
        select_563([b, a, c, d, e, f, g])
    }
}
/// n = 10, i = 5, a < h, b < h, b < i, c < e, c < i, d < f, f < g, f < i, i < j
fn select_653([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_651([a, b, d, c, e, f, h, g, i, j])
    } else {
        select_652([a, c, b, f, g, h, e, i, j])
    }
}
/// n = 11, i = 5, a < h, b < h, b < k, c < e, c < i, d < f, f < g, f < i, i < j, i < k
fn select_654([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < a {
        select_645([e, g, b, i, a, j, k])
    } else {
        select_653([b, a, c, d, e, f, g, h, i, j])
    }
}
/// n = 8, i = 2, b < d, c < e, e < f, e < g, e < h
fn select_655([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_104([a, e, b, f, g, h])
    } else {
        select_47([a, c, b, d, e])
    }
}
/// n = 9, i = 2, c < i, d < e, d < i, e < f, e < g, e < h
fn select_656([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < c {
        select_655([b, a, d, c, e, f, g, h])
    } else {
        select_567([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 8, i = 2, c < d, d < e, d < f, d < g, d < h
fn select_657([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_512([a, c, b, d, e, f, g, h])
    } else {
        select_512([b, c, a, d, e, f, g, h])
    }
}
/// n = 9, i = 4, b < e, c < d, d < f, d < h, d < i, e < g, e < h, e < i
fn select_658([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_149([a, b, d, f, e, h, i])
    } else {
        select_149([a, c, e, g, d, h, i])
    }
}
/// n = 12, i = 4, b < e, c < d, c < h, d < g, d < l, e < f, e < h, e < l, h < i, h < j, h < k
fn select_659([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < d {
        select_657([a, f, c, h, d, i, j, k])
    } else {
        select_658([a, b, c, d, e, g, f, h, l])
    }
}
/// n = 12, i = 4, b < d, b < i, c < e, d < h, e < f, e < g, e < i, i < j, i < k, i < l
fn select_660([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < d {
        select_656([a, g, f, b, i, j, k, l, d])
    } else {
        select_659([a, c, b, d, e, g, h, i, j, k, l, f])
    }
}
/// n = 9, i = 2, c < d, d < e, d < f, d < g, d < h, d < i
fn select_661([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_657([a, b, c, d, e, g, h, i])
    } else {
        select_657([a, b, c, d, f, g, h, i])
    }
}
/// n = 8, i = 4, b < e, c < d, d < f, d < h, e < g, e < h
fn select_662([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_52([a, b, d, f, e, h])
    } else {
        select_52([a, c, e, g, d, h])
    }
}
/// n = 8, i = 4, b < g, b < h, c < g, c < h, d < e, e < f, e < h
fn select_663([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_662([a, b, d, e, c, f, g, h])
    } else {
        select_662([a, c, d, e, b, f, g, h])
    }
}
/// n = 9, i = 5, a < h, a < i, b < h, b < i, c < e, c < i, d < f, f < g, f < i
fn select_664([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < c {
        select_481([b, c, d, e, h, f, g, i])
    } else {
        select_663([e, a, b, d, f, g, h, i])
    }
}
/// n = 13, i = 5, a < h, b < h, b < i, c < e, c < i, d < f, f < g, f < i, i < j, i < k, i < l, i < m
fn select_665([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < a {
        select_661([e, g, b, i, a, j, k, l, m])
    } else {
        select_664([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 14, i = 6, a < i, a < j, b < f, b < g, c < e, c < j, d < g, d < j, f < i, g < h, g < m, g < n, j < k, j < l, j < m, j < n
fn select_666([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < g {
        select_660([e, b, d, f, j, k, l, i, g, h, m, n])
    } else {
        select_665([f, a, c, d, e, g, h, i, j, k, l, m, n])
    }
}
/// n = 15, i = 6, a < i, a < j, b < f, b < g, c < e, c < j, c < m, d < g, d < j, f < i, g < h, g < m, g < o, j < k, j < l, j < o, m < n
fn select_667([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if m < j {
        select_654([f, a, c, d, e, g, h, i, m, n, j])
    } else {
        select_666([a, b, c, d, e, f, g, h, i, j, k, l, m, o])
    }
}
/// n = 16, i = 6, a < e, a < m, b < g, b < l, c < f, c < h, d < g, d < h, f < l, g < j, g < k, g < o, h < i, h < m, h < o, m < n, o < p
fn select_668([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < a {
        select_643([c, g, j, k, f, a, h, e, i, l, o, p, m])
    } else {
        select_667([b, c, a, d, e, f, h, i, l, g, j, k, m, n, o])
    }
}
/// n = 16, i = 6, a < e, a < o, b < g, b < l, c < f, c < h, d < g, d < h, f < l, g < j, g < k, h < i, h < m, h < o, m < n, o < p
fn select_669([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if m < g {
        select_630([f, a, b, d, e, h, i, l, m, n, o, p, g, j, k])
    } else {
        select_668([a, b, c, d, e, f, g, h, i, j, k, l, o, p, m, n])
    }
}
/// n = 8, i = 2, a < h, b < e, c < d, c < f, c < h, f < g
fn select_670([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_170([b, c, d, a, e, f, g])
    } else {
        select_455([a, c, d, f, b, g, e, h])
    }
}
/// n = 9, i = 3, a < e, a < g, a < i, b < c, b < d, b < g, d < i, e < f, g < h
fn select_671([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_174([e, b, c, f, g, h, d])
    } else {
        select_218([c, d, a, g, h, e, i])
    }
}
/// n = 9, i = 3, a < d, a < e, a < h, b < c, b < f, b < h, f < g, h < i
fn select_672([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_670([c, f, a, e, g, h, i, d])
    } else {
        select_671([b, a, e, d, f, g, h, i, c])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, b < g, c < d, c < f, c < g
fn select_673([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_147([d, a, b, f, g, e])
    } else {
        select_147([e, a, c, f, g, d])
    }
}
/// n = 7, i = 3, b < f, b < g, c < f, c < g, d < e, d < f, d < g
fn select_674([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_673([c, b, d, e, a, f, g])
    } else {
        select_54([c, a, d, e, b])
    }
}
/// n = 10, i = 4, a < g, a < j, b < e, b < f, b < i, c < d, c < g, c < j, f < j, g < h, g < i
fn select_675([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_674([e, a, f, c, d, g, j])
    } else {
        select_150([d, b, a, e, g, h, f, i])
    }
}
/// n = 11, i = 4, a < e, a < k, b < g, b < k, c < d, c < f, c < i, f < k, g < h, g < i, i < j
fn select_676([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_672([c, g, h, d, f, a, e, i, j])
    } else {
        select_675([b, c, a, e, d, f, g, h, i, k])
    }
}
/// n = 7, i = 3, a < g, b < f, c < f, c < g, d < e, d < g
fn select_677([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_486([a, b, d, c, e, f, g])
    } else {
        select_351([a, b, e, c])
    }
}
/// n = 9, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_678([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_98([a, g, h, i])
    } else {
        select_31([b, c, d, e, f])
    }
}
/// n = 9, i = 3, a < e, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_679([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_221([d, b, a, f, g, h, i])
    } else {
        select_678([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 9, i = 3, b < f, c < f, c < g, d < e, d < g, g < h, g < i
fn select_680([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_677([a, b, c, d, e, f, g])
    } else {
        select_679([b, c, d, e, f, g, a, h, i])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, c < d, c < f, c < g
fn select_681([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_73([c, b, a, e, d, f])
    } else {
        select_10([c, a, d, b, e, g])
    }
}
/// n = 8, i = 2, b < e, b < h, c < d, c < f, c < h, f < g
fn select_682([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_681([a, b, c, d, e, h, f])
    } else {
        select_179([b, c, d, e, f, a, g, h])
    }
}
/// n = 10, i = 3, b < d, b < g, b < j, c < e, c < g, e < f, e < j, g < h, g < i
fn select_683([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_465([e, b, d, a, f, g, h, i, j])
    } else {
        select_391([a, b, c, d, e, g, h, i, f])
    }
}
/// n = 12, i = 4, a < e, a < i, b < d, b < h, b < i, c < f, c < i, f < g, f < h, h < j, i < k, i < l
fn select_684([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_682([g, a, b, d, e, h, j, i])
    } else {
        select_683([e, b, c, d, f, g, i, k, l, h])
    }
}
/// n = 13, i = 5, a < d, a < h, b < f, b < i, c < e, c < h, e < i, e < k, f < g, f < k, f < m, h < j, h < m, k < l
fn select_685([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < f {
        select_680([d, e, b, h, j, i, f, g, m])
    } else {
        select_684([a, f, c, g, d, e, i, k, h, l, j, m])
    }
}
/// n = 13, i = 5, a < d, a < i, b < f, b < j, c < e, c < i, e < j, e < l, f < g, f < h, f < l, i < k, l < m
fn select_686([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < i {
        select_676([a, c, f, h, d, g, e, j, l, m, i])
    } else {
        select_685([a, b, c, d, e, f, h, i, j, k, l, m, g])
    }
}
/// n = 9, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, f < i, g < h
fn select_687([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_185([a, b, c, d, e, f, g, i])
    } else {
        select_185([a, b, c, d, e, g, f, h])
    }
}
/// n = 9, i = 3, c < f, c < g, d < e, d < f, d < g, f < i, g < h
fn select_688([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_687([a, c, d, e, b, f, g, h, i])
    } else {
        select_687([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 10, i = 3, c < f, c < g, d < e, d < f, d < g, f < j, g < h, g < i
fn select_689([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_688([a, b, c, d, e, f, g, h, j])
    } else {
        select_688([a, b, c, d, e, f, g, i, j])
    }
}
/// n = 10, i = 3, c < h, c < j, d < e, d < f, d < h, f < g, f < j, h < i
fn select_690([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < c {
        select_132([a, b, e, f, g, c, h, j])
    } else {
        select_689([a, b, c, d, e, h, f, g, j, i])
    }
}
/// n = 8, i = 3, a < c, a < d, b < e, b < h, d < h, e < f, e < g
fn select_691([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_32([a, e, f, g, c])
    } else {
        select_302([b, a, c, d, e, g, h])
    }
}
/// n = 11, i = 4, a < d, a < e, a < k, b < e, b < f, c < d, c < f, d < i, e < j, f < g, f < h, f < k
fn select_692([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < f {
        select_269([c, b, e, d, i, j, f])
    } else {
        select_36([a, c, e, f, g, h, j, d, k])
    }
}
/// n = 11, i = 4, a < f, a < k, b < f, b < g, c < d, c < g, d < e, f < j, g < h, g < i, g < k
fn select_693([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_691([d, b, e, a, g, h, i, f])
    } else {
        select_692([a, b, c, d, f, g, h, i, e, j, k])
    }
}
/// n = 11, i = 4, a < f, a < k, b < f, b < g, c < d, c < e, c < g, f < j, g < h, g < i, g < k
fn select_694([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_693([a, b, c, d, e, f, g, h, i, j, k])
    } else {
        select_693([a, b, c, e, d, f, g, h, i, j, k])
    }
}
/// n = 9, i = 3, a < c, a < i, b < d, c < f, d < e, d < g, d < i, g < h
fn select_695([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_43([e, a, g, h, i])
    } else {
        select_269([b, a, c, d, e, f, i])
    }
}
/// n = 10, i = 4, a < d, b < c, b < i, c < g, d < e, d < f, d < i, g < h, i < j
fn select_696([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < c {
        select_45([f, e, b, i, j, c])
    } else {
        select_695([c, a, g, d, f, h, i, j, e])
    }
}
/// n = 10, i = 5, a < g, a < h, b < g, b < h, c < d, d < e, d < f, d < j, g < j, h < i
fn select_697([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_476([g, h, c, i, d, f, e, j])
    } else {
        select_41([f, e, a, b, g, j, h])
    }
}
/// n = 11, i = 5, a < h, b < e, c < d, c < h, d < j, e < f, e < g, e < j, h < i, j < k
fn select_698([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_696([b, d, a, e, f, g, h, i, j, k])
    } else {
        select_697([a, c, b, e, f, g, d, h, i, j])
    }
}
/// n = 12, i = 5, a < h, a < l, b < e, c < d, c < h, c < j, e < f, e < g, e < j, h < i, j < k, j < l
fn select_699([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < d {
        select_694([a, c, e, f, g, h, j, d, k, i, l])
    } else {
        select_698([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 13, i = 5, a < d, a < h, b < h, b < l, c < e, e < f, e < g, e < i, h < k, i < j, i < l, l < m
fn select_700([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < a {
        select_690([f, g, b, i, j, a, d, l, m, h])
    } else {
        select_699([b, c, a, d, e, f, g, h, k, i, j, l])
    }
}
/// n = 13, i = 5, a < d, a < j, b < e, b < j, c < f, c < l, e < l, f < g, f < h, f < i, i < k, j < m
fn select_701([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < i {
        select_686([a, c, b, d, e, f, g, h, j, l, m, i, k])
    } else {
        select_700([a, b, c, d, f, g, h, j, i, k, m, e, l])
    }
}
/// n = 7, i = 2, b < e, c < d, c < f, f < g
fn select_702([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < b {
        select_77([a, d, f, b, g])
    } else {
        select_130([a, b, c, d, e, f])
    }
}
/// n = 7, i = 2, d < e, d < f, f < g
fn select_703([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_702([c, a, d, e, b, f, g])
    } else {
        select_702([c, b, d, e, a, f, g])
    }
}
/// n = 9, i = 3, b < d, c < e, e < f, e < g, g < h, g < i
fn select_704([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < b {
        select_92([a, f, g, b, h, i])
    } else {
        select_341([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 4, b < e, c < d, c < h, c < i, d < f, e < g, e < h, g < i
fn select_705([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_341([a, d, b, f, e, h, g])
    } else {
        select_148([a, g, c, h, d, i])
    }
}
/// n = 10, i = 4, b < e, c < d, c < h, c < i, d < g, e < f, e < h, e < i, i < j
fn select_706([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < f {
        select_704([a, d, b, g, e, h, i, f, j])
    } else {
        select_705([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 10, i = 4, b < d, b < i, c < e, d < h, e < f, e < g, e < i, i < j
fn select_707([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_297([a, g, f, b, d, i])
    } else {
        select_706([a, c, b, d, e, g, h, f, i, j])
    }
}
/// n = 10, i = 4, b < d, c < e, d < h, e < f, e < g, e < i, i < j
fn select_708([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < b {
        select_703([a, f, g, i, j, b, d])
    } else {
        select_707([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 11, i = 3, b < e, c < f, d < e, d < f, e < j, e < k, f < g, f < h, f < i
fn select_709([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_104([a, f, b, g, h, i])
    } else {
        select_413([a, c, b, d, e, j, k, f])
    }
}
/// n = 11, i = 3, b < h, c < h, c < i, d < e, e < f, e < g, e < i, h < j, h < k
fn select_710([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < c {
        select_620([a, b, e, f, g, c, i])
    } else {
        select_709([a, b, d, c, h, e, f, g, i, j, k])
    }
}
/// n = 7, i = 3, a < f, a < g, b < f, b < g, c < e, c < g, d < e, d < f
fn select_711([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_82([c, d, a, b, e, f, g])
    } else {
        select_82([c, d, b, a, e, f, g])
    }
}
/// n = 7, i = 4, a < g, b < g, c < f, d < f, d < g, e < f, e < g
fn select_712([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_40([b, c, d, e, f, g])
    } else {
        select_378([a, b, d, e, f])
    }
}
/// n = 9, i = 5, a < h, a < i, b < h, b < i, c < f, c < i, d < g, d < i, e < g, e < i, g < h
fn select_713([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_711([a, b, c, g, f, h, i])
    } else {
        select_712([a, b, f, d, e, g, i])
    }
}
/// n = 9, i = 4, a < i, b < i, c < f, d < f, d < i, e < f, e < i, f < g, f < h
fn select_714([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_357([a, c, d, e, b, f, g, h, i])
    } else {
        select_357([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 12, i = 5, a < g, a < l, b < g, b < l, c < h, c < k, c < l, d < h, d < k, d < l, e < f, e < l, f < h, g < k, h < i, h < j
fn select_715([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_713([c, d, e, a, b, f, g, k, l])
    } else {
        select_714([a, b, f, c, d, h, i, j, l])
    }
}
/// n = 7, i = 3, a < g, b < e, b < f, c < e, c < f, c < g, d < e, d < f, d < g
fn select_716([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_40([a, b, c, d, f, g])
    } else {
        select_2([e, f])
    }
}
/// n = 10, i = 5, a < i, a < j, b < g, b < j, c < g, c < i, c < j, d < f, d < h, d < j, e < f, e < h, e < j, f < i, g < h
fn select_717([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_716([a, g, d, e, f, h, j])
    } else {
        select_156([a, b, f, c, g, i, j])
    }
}
/// n = 8, i = 5, a < h, b < g, b < h, c < g, c < h, d < f, d < h, e < f, e < h
fn select_718([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_40([a, f, b, c, g, h])
    } else {
        select_40([a, g, d, e, f, h])
    }
}
/// n = 10, i = 5, b < g, b < h, c < g, c < h, d < f, d < h, e < f, e < h, h < i, h < j
fn select_719([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < h {
        select_718([a, b, c, d, e, f, g, h])
    } else {
        select_92([f, g, h, a, i, j])
    }
}
/// n = 13, i = 6, a < l, a < m, b < g, b < m, c < h, c < i, c < m, d < h, d < i, d < m, e < f, e < i, f < l, f < m, g < i, h < l, i < j, i < k
fn select_720([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_717([a, b, f, c, d, h, g, i, l, m])
    } else {
        select_719([a, c, d, e, g, f, h, i, j, k])
    }
}
/// n = 13, i = 6, a < i, a < m, b < h, b < j, b < m, c < h, c < j, c < m, d < g, d < m, e < f, e < j, f < i, f < m, g < j, j < k, j < l
fn select_721([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < h {
        select_715([a, f, b, c, d, g, i, j, k, l, h, m])
    } else {
        select_720([a, d, b, c, e, f, g, h, j, k, l, i, m])
    }
}
/// n = 15, i = 6, a < i, a < j, b < f, b < m, c < h, c < j, c < m, d < h, d < j, d < m, e < g, e < j, f < i, g < m, j < k, j < l, m < n, m < o
fn select_722([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < f {
        select_710([h, g, b, a, j, k, l, m, f, n, o])
    } else {
        select_721([a, c, d, e, b, f, g, h, i, m, n, o, j])
    }
}
/// n = 15, i = 6, a < i, a < j, b < h, b < m, c < f, c < m, d < h, d < j, d < m, e < g, e < j, f < i, g < m, j < k, j < l, m < n, m < o
fn select_723([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < b {
        select_708([g, c, d, f, j, k, l, i, b, h])
    } else {
        select_722([a, c, b, d, e, f, g, h, i, j, k, l, m, n, o])
    }
}
/// n = 10, i = 2, c < g, c < j, d < e, d < f, d < j, f < h, g < i
fn select_724([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_583([a, b, c, g, d, e, f, j, i])
    } else {
        select_124([a, b, d, e, f, c, g, j, h])
    }
}
/// n = 8, i = 3, b < f, c < e, c < g, d < e, d < f, d < g, g < h
fn select_725([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < a {
        select_115([b, c, e, f, g, a, h])
    } else {
        select_486([a, b, c, d, e, f, g])
    }
}
/// n = 8, i = 3, b < f, c < g, d < e, d < f, d < g, g < h
fn select_726([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < c {
        select_241([a, b, e, f, c, g])
    } else {
        select_725([a, b, c, d, e, f, g, h])
    }
}
/// n = 8, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, g < h
fn select_727([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < a {
        select_77([d, f, g, a, h])
    } else {
        select_467([b, a, c, d, e, f, g])
    }
}
/// n = 8, i = 3, c < f, c < g, d < e, d < f, d < g, g < h
fn select_728([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_727([a, c, d, e, b, f, g, h])
    } else {
        select_727([b, c, d, e, a, f, g, h])
    }
}
/// n = 10, i = 3, b < h, c < f, c < g, d < e, d < f, d < g, d < h, g < i, h < j
fn select_729([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_726([a, c, b, d, e, f, h, j])
    } else {
        select_728([a, b, c, d, e, f, g, i])
    }
}
/// n = 11, i = 3, b < k, c < g, c < h, c < j, d < e, d < f, d < g, d < h, f < j, f < k, h < i
fn select_730([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_724([a, e, f, c, g, h, b, i, k, j])
    } else {
        select_729([a, b, c, d, e, g, h, f, i, k])
    }
}
/// n = 8, i = 3, a < f, b < g, c < d, c < e, c < g, g < h
fn select_731([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < b {
        select_241([e, a, d, f, b, g])
    } else {
        select_727([a, b, c, e, f, d, g, h])
    }
}
/// n = 7, i = 2, a < d, a < e, a < f, a < g, b < c, b < e, b < f, b < g
fn select_732([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_62([a, b, c, d, e, g])
    } else {
        select_62([a, b, c, d, f, g])
    }
}
/// n = 7, i = 2, b < e, b < f, b < g, c < d, c < e, c < f, c < g
fn select_733([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_46([a, c, d, b])
    } else {
        select_732([b, c, d, a, e, f, g])
    }
}
/// n = 8, i = 3, a < e, a < h, b < f, b < h, c < d, c < f, c < h, f < g
fn select_734([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_187([b, a, c, d, e, h])
    } else {
        select_45([d, a, b, f, g, h])
    }
}
/// n = 12, i = 4, a < d, a < g, b < g, b < i, c < e, c < k, c < l, e < f, e < i, g < h, g < k, g < l, i < j
fn select_735([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_733([d, c, g, h, e, k, l])
    } else {
        select_734([a, b, e, f, d, i, j, g])
    }
}
/// n = 13, i = 4, a < d, a < h, b < h, b < j, c < e, c < l, c < m, e < f, e < g, e < j, h < i, h < l, h < m, j < k
fn select_736([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < h {
        select_731([a, b, e, f, g, d, j, k])
    } else {
        select_735([a, b, c, d, e, g, h, i, j, k, l, m])
    }
}
/// n = 10, i = 5, a < g, b < h, c < g, c < h, d < f, d < h, e < f, e < h, h < i, h < j
fn select_737([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < h {
        select_718([b, a, c, d, e, f, g, h])
    } else {
        select_91([f, h, a, i, j])
    }
}
/// n = 11, i = 5, a < g, b < h, c < g, c < h, d < f, d < h, e < f, e < h, h < i, h < j, h < k
fn select_738([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_718([b, a, c, d, e, f, g, h])
    } else {
        select_104([f, h, a, i, j, k])
    }
}
/// n = 11, i = 5, a < g, b < j, b < k, c < g, c < h, d < f, d < h, e < f, e < h, h < i, h < j, h < k
fn select_739([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < h {
        select_738([a, b, c, d, e, f, g, h, i, j, k])
    } else {
        select_131([f, a, h, i, g, b, j, k])
    }
}
/// n = 14, i = 5, a < g, a < j, b < j, b < k, b < l, c < f, c < h, d < g, d < h, d < j, e < f, e < h, e < j, h < i, h < k, h < l, j < m, j < n
fn select_740([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < h {
        select_737([c, b, e, a, d, g, f, j, m, n])
    } else {
        select_739([a, b, d, c, e, f, g, h, i, k, l])
    }
}
/// n = 15, i = 6, a < i, a < n, a < o, b < g, b < l, c < f, c < l, d < h, d < i, d < l, e < g, e < i, e < l, f < h, i < j, i < k, l < m, l < n, l < o
fn select_741([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_736([b, c, a, g, i, j, k, l, m, f, h, n, o])
    } else {
        select_740([f, a, b, d, e, g, h, l, m, i, n, o, j, k])
    }
}
/// n = 16, i = 6, a < h, a < i, b < i, b < n, b < o, c < g, c < j, d < f, d < j, e < g, e < i, e < j, f < h, i < l, i < m, j < k, j < n, j < o, o < p
fn select_742([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < a {
        select_730([g, f, b, j, k, a, n, o, p, i, h])
    } else {
        select_741([b, c, d, a, e, f, g, h, i, l, m, j, k, n, o])
    }
}
/// n = 16, i = 6, a < i, a < j, b < h, b < k, c < g, c < k, d < f, d < j, d < o, e < h, e < j, e < k, g < i, j < m, j < n, k < l, k < o, o < p
fn select_743([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < k {
        select_723([a, b, c, e, d, g, f, h, i, j, m, n, k, l, o])
    } else {
        select_742([a, d, b, c, e, g, h, i, j, k, l, m, n, f, o, p])
    }
}
/// n = 16, i = 6, a < f, a < o, b < i, b < l, c < h, c < m, d < g, d < m, e < h, e < i, e < m, g < l, i < j, i < k, m < n, m < o, o < p
fn select_744([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < a {
        select_701([c, d, b, h, g, i, j, k, a, m, f, l, n])
    } else {
        select_743([b, c, d, a, e, f, g, h, l, i, m, n, j, k, o, p])
    }
}
/// n = 16, i = 6, a < h, b < f, b < o, c < i, c < n, d < g, d < j, e < h, e < i, e < j, g < n, i < l, i < m, j < k, j < o, o < p
fn select_745([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < a {
        select_669([b, c, d, e, f, g, i, j, k, l, m, n, a, h, o, p])
    } else {
        select_744([b, c, a, d, e, f, g, h, i, l, m, n, j, k, o, p])
    }
}
/// n = 16, i = 6, b < g, b < o, c < i, c < n, d < h, d < j, e < f, e < i, e < j, h < n, i < l, i < m, j < k, j < o, o < p
fn select_746([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < a {
        select_592([b, c, d, e, g, f, h, a, i, j, k, l, m, n, o, p])
    } else {
        select_745([a, b, c, d, e, g, h, f, i, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 6, b < i, b < o, c < h, c < j, d < g, d < k, e < f, e < j, e < k, j < m, j < n, k < l, k < o, o < p
fn select_747([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < g {
        select_432([a, b, d, c, e, f, i, h, j, k, m, n, l, g, o, p])
    } else {
        select_746([a, b, c, d, e, f, i, g, j, k, l, m, n, h, o, p])
    }
}
/// n = 9, i = 2, a < d, a < f, a < i, b < c, b < e, b < i, e < g, f < h
fn select_748([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_174([a, b, c, d, e, g, i])
    } else {
        select_174([b, a, d, c, f, h, i])
    }
}
/// n = 9, i = 3, a < e, a < g, b < d, b < f, c < e, c < f, c < g, f < h, g < i
fn select_749([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_17([a, c, b, d, e, f, h])
    } else {
        select_268([b, a, c, d, e, g, i])
    }
}
/// n = 10, i = 3, a < d, b < g, b < h, b < j, c < e, c < g, c < h, e < f, e < j, h < i
fn select_750([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_748([b, e, f, g, a, h, d, i, j])
    } else {
        select_749([b, a, c, d, g, e, h, f, i])
    }
}
/// n = 10, i = 3, c < g, c < h, c < j, d < e, d < g, d < h, e < f, e < j, h < i
fn select_751([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_750([a, c, d, b, e, f, g, h, i, j])
    } else {
        select_750([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 11, i = 3, c < f, c < i, c < j, d < e, d < i, d < j, e < g, f < h, j < k
fn select_752([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_751([a, b, c, d, e, g, i, j, k, f])
    } else {
        select_751([a, b, d, c, f, h, i, j, k, e])
    }
}
/// n = 9, i = 3, a < d, b < g, b < h, b < i, c < e, c < f, e < i, f < g, f < h
fn select_753([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_183([e, a, f, d, b, g, h])
    } else {
        select_534([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 5, i = 2, c < d, d < e
fn select_754([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < d {
        select_18([b, a, c, d])
    } else {
        select_42([b, d, a, e])
    }
}
/// n = 8, i = 3, a < d, b < g, b < h, c < e, e < f, e < g, e < h
fn select_755([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_94([f, a, b, g, h, d])
    } else {
        select_754([b, d, c, e, f])
    }
}
/// n = 8, i = 3, a < d, b < f, b < g, b < h, c < e, e < f, e < g, e < h
fn select_756([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_351([b, d, c, e])
    } else {
        select_207([a, b, f, g, h, d])
    }
}
/// n = 10, i = 3, a < d, b < f, b < j, c < e, c < h, c < i, e < g, e < j, f < h, f < i
fn select_757([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_755([a, b, c, d, e, g, f, j])
    } else {
        select_756([a, c, b, d, f, e, h, i])
    }
}
/// n = 10, i = 3, a < e, b < f, b < i, b < j, c < d, c < g, f < h, g < i, g < j
fn select_758([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_753([a, b, c, e, d, g, i, j, f])
    } else {
        select_757([a, c, b, e, f, g, h, i, j, d])
    }
}
/// n = 9, i = 3, a < d, a < i, b < e, b < i, c < e, c < h, c < i, d < h, e < f, e < g
fn select_759([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_9([a, e, f, g, i])
    } else {
        select_21([b, d, c, h, i])
    }
}
/// n = 9, i = 4, a < e, a < f, b < g, c < f, c < g, d < e, d < g, g < h, g < i
fn select_760([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_181([a, b, c, e, g, h, i, f])
    } else {
        select_181([a, b, d, f, g, h, i, e])
    }
}
/// n = 10, i = 4, a < f, b < g, b < j, c < f, c < g, c < j, d < e, d < g, e < j, g < h, g < i
fn select_761([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_759([e, b, c, a, g, h, i, f, j])
    } else {
        select_760([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, a < f, b < h, b < i, b < j, c < f, c < g, c < j, d < e, d < g, e < j, g < h, g < i
fn select_762([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_183([e, a, g, f, b, h, i])
    } else {
        select_761([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 11, i = 4, a < f, b < g, b < j, b < k, c < e, c < h, d < f, d < g, d < h, g < i, h < j, h < k
fn select_763([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_757([a, c, b, f, g, h, i, j, k, e])
    } else {
        select_762([a, b, d, c, e, f, h, j, k, g])
    }
}
/// n = 11, i = 4, b < g, b < j, b < k, c < f, c < h, d < e, d < g, d < h, g < i, h < j, h < k
fn select_764([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_758([e, b, c, f, a, g, h, i, j, k])
    } else {
        select_763([a, b, c, d, f, e, g, h, i, j, k])
    }
}
/// n = 12, i = 4, b < f, c < h, c < j, c < k, d < e, d < g, d < h, g < j, g < k, h < i, k < l
fn select_765([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_752([a, e, c, g, b, h, f, i, j, k, l])
    } else {
        select_764([a, c, b, d, e, f, h, g, i, j, k])
    }
}
/// n = 10, i = 2, a < f, a < j, b < e, b < i, c < d, c < g, c < i, c < j, g < h
fn select_766([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_748([a, c, d, f, g, b, h, e, j])
    } else {
        select_748([b, c, d, e, g, a, h, f, i])
    }
}
/// n = 10, i = 3, a < g, b < d, b < h, b < j, c < e, c < g, c < h, e < f, e < j, h < i
fn select_767([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_748([b, e, f, d, a, h, g, i, j])
    } else {
        select_749([a, b, c, d, g, h, e, i, f])
    }
}
/// n = 11, i = 3, b < d, b < h, b < j, b < k, c < e, c < f, c < h, e < k, f < g, f < j, h < i
fn select_768([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_766([e, f, b, d, g, a, h, i, j, k])
    } else {
        select_767([a, b, c, d, f, g, e, h, i, j])
    }
}
/// n = 10, i = 3, a < c, a < d, b < e, b < j, d < j, e < f, e < g, f < i, g < h
fn select_769([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_119([f, g, a, c, d, h, i])
    } else {
        select_84([c, d, b, e, f, g, j])
    }
}
/// n = 9, i = 2, a < c, a < h, a < i, b < d, b < e, b < h, b < i, d < g, e < f
fn select_770([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_65([a, b, c, d, e, g, h, i])
    } else {
        select_65([a, b, c, e, d, f, h, i])
    }
}
/// n = 11, i = 3, a < c, a < f, a < k, b < d, b < f, c < e, d < g, d < h, d < k, g < j, h < i
fn select_771([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_31([b, c, e, f, d])
    } else {
        select_770([a, d, f, g, h, i, j, c, k])
    }
}
/// n = 11, i = 3, a < d, b < f, b < k, c < e, c < f, e < g, e < h, e < k, g < j, h < i
fn select_772([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_769([a, c, d, b, e, g, h, i, j, f])
    } else {
        select_771([b, c, a, e, d, f, g, h, i, j, k])
    }
}
/// n = 8, i = 3, a < c, a < d, b < e, b < h, d < h, e < f, f < g
fn select_773([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_76([c, b, d, e, h])
    } else {
        select_10([a, e, c, f, g, d])
    }
}
/// n = 8, i = 3, a < e, b < e, b < f, b < h, c < d, c < f, d < h, f < g
fn select_774([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_10([d, b, a, f, g, h])
    } else {
        select_282([a, b, c, d, e, f, g])
    }
}
/// n = 11, i = 4, a < e, a < g, b < e, b < g, b < h, b < j, c < d, c < k, d < f, d < h, f < j, g < k, h < i
fn select_775([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_773([a, c, e, g, d, h, i, k])
    } else {
        select_774([a, b, d, f, e, h, i, j])
    }
}
/// n = 12, i = 4, a < f, b < e, b < h, c < d, c < f, c < i, c < k, d < h, d < l, e < g, e < i, e < l, g < k, i < j
fn select_776([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < a {
        select_771([d, b, a, e, f, h, g, i, j, k, l])
    } else {
        select_775([a, c, b, e, f, g, d, i, j, k, h])
    }
}
/// n = 12, i = 4, b < f, b < h, c < d, c < e, c < i, c < k, e < h, e < l, f < g, f < i, f < l, g < k, i < j
fn select_777([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < a {
        select_772([d, e, b, a, f, h, g, i, j, k, l])
    } else {
        select_776([a, b, c, e, f, d, g, h, i, j, k, l])
    }
}
/// n = 13, i = 4, b < d, b < e, b < k, b < m, c < f, c < j, e < j, f < g, f < h, f < k, g < m, h < i, k < l
fn select_778([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_768([a, b, f, d, g, h, i, k, l, e, m])
    } else {
        select_777([a, c, b, d, e, f, g, j, k, l, m, h])
    }
}
/// n = 8, i = 3, a < h, b < d, b < g, b < h, c < e, c < h, e < f, e < g
fn select_779([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_73([b, e, a, f, d, g])
    } else {
        select_11([b, a, c, d, e, f, h])
    }
}
/// n = 8, i = 3, b < h, c < g, c < h, d < e, d < h, e < f, e < g
fn select_780([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_146([b, a, d, c, e, f])
    } else {
        select_779([b, c, d, a, e, f, g, h])
    }
}
/// n = 9, i = 4, b < e, b < i, c < g, c < i, d < f, d < g, f < i, g < h
fn select_781([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_139([a, c, b, d, f, e, g, i])
    } else {
        select_780([a, f, b, c, g, h, e, i])
    }
}
/// n = 11, i = 4, b < e, b < h, c < f, c < h, d < f, d < h, f < g, h < i, h < j, h < k
fn select_782([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_567([a, f, b, g, h, i, j, k, e])
    } else {
        select_135([a, e, c, d, h, i, j, k, f])
    }
}
/// n = 12, i = 5, b < f, b < g, c < g, c < i, d < e, d < f, e < i, e < k, e < l, g < h, g < k, g < l, i < j
fn select_783([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_126([a, d, g, h, f, e, i, k, l])
    } else {
        select_782([a, b, c, e, f, i, j, g, h, k, l])
    }
}
/// n = 12, i = 5, b < g, b < h, c < h, c < j, d < e, d < g, e < f, e < j, e < l, h < i, h < l, j < k
fn select_784([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_781([a, b, c, e, g, f, j, k, h])
    } else {
        select_783([a, b, c, d, e, g, h, i, j, k, f, l])
    }
}
/// n = 14, i = 5, b < h, c < e, c < j, d < f, d < h, e < g, e < l, e < n, f < i, f < j, f < l, i < n, j < k, l < m
fn select_785([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < b {
        select_778([a, e, d, g, b, f, i, j, k, h, l, m, n])
    } else {
        select_784([a, b, c, d, f, i, h, e, g, j, k, l])
    }
}
/// n = 14, i = 5, b < h, c < e, c < k, c < m, d < f, d < h, e < g, e < n, f < i, f < j, f < k, i < n, j < m, k < l
fn select_786([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < e {
        select_765([a, b, c, f, i, h, j, k, l, m, e, g])
    } else {
        select_785([a, b, c, d, e, f, g, h, i, k, l, j, m, n])
    }
}
/// n = 9, i = 2, a < d, a < e, b < c, b < f, e < i, f < g, f < h
fn select_787([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_91([c, f, a, g, h])
    } else {
        select_174([b, a, d, c, e, i, f])
    }
}
/// n = 10, i = 2, a < d, a < g, b < c, b < e, b < f, e < i, f < h, g < j
fn select_788([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_787([a, b, c, d, g, e, f, i, j])
    } else {
        select_787([a, b, c, d, g, f, e, h, j])
    }
}
/// n = 10, i = 3, a < d, a < g, b < e, b < f, c < e, c < f, c < g, e < i, f < h, g < j
fn select_789([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_259([b, a, c, d, g, e, j, f, i])
    } else {
        select_259([b, a, c, d, g, f, j, e, h])
    }
}
/// n = 11, i = 3, a < e, a < h, b < d, b < f, b < g, c < f, c < g, c < h, f < j, g < i, h < k
fn select_790([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < h {
        select_687([a, c, b, d, e, f, g, i, j])
    } else {
        select_789([a, b, c, e, f, g, h, i, j, k])
    }
}
/// n = 11, i = 3, a < e, b < d, b < h, b < i, c < f, c < h, c < i, f < g, h < k, i < j
fn select_791([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_788([f, b, d, g, h, i, a, j, k, e])
    } else {
        select_790([a, b, c, d, e, h, i, f, j, k, g])
    }
}
/// n = 11, i = 3, c < e, c < h, c < i, d < f, d < h, d < i, f < g, h < k, i < j
fn select_792([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_791([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_791([b, c, d, e, a, f, g, h, i, j, k])
    }
}
/// n = 8, i = 2, a < c, a < f, a < g, a < h, b < d, b < f, b < g, b < h, d < e
fn select_793([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_91([c, b, f, g, h])
    } else {
        select_207([d, a, f, g, h, e])
    }
}
/// n = 8, i = 2, a < e, b < f, b < g, b < h, c < d, c < f, c < g, c < h
fn select_794([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_32([c, a, b, e, d])
    } else {
        select_793([c, b, d, a, e, f, g, h])
    }
}
/// n = 9, i = 3, b < f, c < g, c < h, c < i, d < e, d < f, e < g, e < h, e < i
fn select_795([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_359([c, a, b, d, f, e])
    } else {
        select_794([b, c, e, a, f, g, h, i])
    }
}
/// n = 10, i = 4, b < e, b < i, b < j, c < g, c < h, d < f, d < g, f < h, g < i, g < j
fn select_796([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_528([a, c, b, d, f, e, h, g])
    } else {
        select_795([a, f, b, c, g, h, e, i, j])
    }
}
/// n = 10, i = 4, b < g, b < i, b < j, c < f, c < h, d < e, d < h, h < i, h < j
fn select_797([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_796([a, b, c, d, g, e, h, f, i, j])
    } else {
        select_796([a, b, d, c, g, f, h, e, i, j])
    }
}
/// n = 12, i = 4, b < g, c < f, c < i, c < j, d < e, d < h, h < i, h < j, i < l, j < k
fn select_798([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < b {
        select_792([a, e, c, h, f, b, g, i, j, k, l])
    } else {
        select_797([a, c, b, d, e, g, f, h, i, j])
    }
}
/// n = 10, i = 2, a < e, a < i, a < j, b < c, b < d, b < i, b < j, c < g, d < f, e < h
fn select_799([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_119([c, d, a, i, j, f, g])
    } else {
        select_281([a, b, c, e, g, h, i, j])
    }
}
/// n = 9, i = 3, a < d, a < i, b < e, b < f, b < i, c < e, c < f, c < i, e < h, f < g
fn select_800([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_19([a, b, c, d, e, f, h, i])
    } else {
        select_19([a, b, c, d, f, e, g, i])
    }
}
/// n = 9, i = 3, a < d, a < e, b < f, b < g, c < e, c < f, c < g, f < i, g < h
fn select_801([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_3([d, e, b])
    } else {
        select_800([a, b, c, d, f, g, h, i, e])
    }
}
/// n = 11, i = 3, a < d, b < f, b < g, b < j, b < k, c < e, c < f, c < g, e < j, e < k, f < i, g < h
fn select_802([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_799([e, b, f, g, a, h, i, d, j, k])
    } else {
        select_801([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 9, i = 3, a < d, b < e, b < f, c < e, c < f, f < g, f < h, f < i
fn select_803([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_104([e, f, a, g, h, i])
    } else {
        select_137([b, c, a, d, e, f])
    }
}
/// n = 11, i = 3, a < d, b < e, b < f, b < g, c < e, c < f, c < g, e < k, f < j, g < h, g < i
fn select_804([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_120([e, f, g, a, h, i, j, k])
    } else {
        select_800([a, b, c, d, e, f, j, k, g])
    }
}
/// n = 12, i = 4, a < d, b < f, b < g, c < e, c < h, c < i, e < g, e < l, f < h, f < i, f < l, h < k, i < j
fn select_805([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_803([a, b, e, d, g, f, h, i, l])
    } else {
        select_804([a, c, f, d, h, i, e, g, l, j, k])
    }
}
/// n = 13, i = 4, a < d, b < e, b < i, b < j, b < m, c < f, c < h, e < h, f < g, f < i, f < j, g < m, i < l, j < k
fn select_806([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_802([a, b, f, d, g, i, j, k, l, e, m])
    } else {
        select_805([a, c, b, d, e, f, h, i, j, k, l, g])
    }
}
/// n = 13, i = 4, c < e, c < i, c < j, c < m, d < f, d < h, e < h, f < g, f < i, f < j, g < m, i < l, j < k
fn select_807([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_806([a, c, d, b, e, f, g, h, i, j, k, l, m])
    } else {
        select_806([b, c, d, a, e, f, g, h, i, j, k, l, m])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < e, d < g
fn select_808([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_16([a, d, b, f, g, e])
    } else {
        select_360([b, e, a, c, g, f])
    }
}
/// n = 6, i = 3, a < d, b < e, b < f, c < e, c < f
fn select_809([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_27([a, b, c, d, e, f])
    } else {
        select_27([a, b, c, d, f, e])
    }
}
/// n = 9, i = 3, a < f, b < d, b < g, b < h, b < i, c < e, c < f, e < g, e < h, e < i
fn select_810([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_809([b, a, c, d, e, f])
    } else {
        select_793([b, e, d, a, f, g, h, i])
    }
}
/// n = 10, i = 4, b < g, c < f, c < g, d < e, d < g, e < h, e < i, e < j, f < h, f < i, f < j
fn select_811([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_808([b, a, d, c, f, e, g])
    } else {
        select_810([b, e, c, a, f, g, h, i, j])
    }
}
/// n = 11, i = 5, b < h, b < i, c < f, c < i, d < e, d < h, e < i, f < g, f < j, f < k, h < j, h < k
fn select_812([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_811([a, e, b, c, f, h, i, g, j, k])
    } else {
        select_528([a, b, f, d, e, g, i, h])
    }
}
/// n = 11, i = 5, b < h, b < i, c < e, c < i, d < f, d < h, f < g, f < j, f < k, i < j, i < k
fn select_813([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_812([a, b, d, c, e, f, g, i, h, j, k])
    } else {
        select_796([a, f, c, b, g, h, i, e, j, k])
    }
}
/// n = 14, i = 5, b < h, c < e, c < f, d < g, d < h, f < j, f < k, f < n, g < i, g < j, g < k, i < n, j < m, k < l
fn select_814([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < b {
        select_807([a, e, f, d, b, g, i, h, j, k, l, m, n])
    } else {
        select_813([a, b, c, d, e, g, i, h, f, j, k])
    }
}
/// n = 15, i = 5, b < h, c < e, c < f, c < k, d < g, d < h, f < l, f < o, g < i, g < j, g < l, i < o, j < k, k < m, l < n
fn select_815([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < f {
        select_798([a, b, c, g, i, e, h, j, f, k, m, l])
    } else {
        select_814([a, b, c, d, e, f, g, h, i, j, l, n, k, o])
    }
}
/// n = 15, i = 5, b < h, c < k, c < m, d < f, d < n, e < g, e < h, g < i, g < j, g < k, i < m, j < n, k < l, n < o
fn select_816([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if c < d {
        select_786([a, b, c, e, d, g, f, h, j, i, k, l, m, n])
    } else {
        select_815([a, b, d, e, f, c, g, h, i, j, n, k, o, l, m])
    }
}
/// n = 8, i = 3, a < e, a < h, b < d, b < h, c < e, c < g, c < h, d < g, e < f
fn select_817([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_3([a, c, g])
    } else {
        select_10([b, c, d, e, f, h])
    }
}
/// n = 8, i = 3, a < g, a < h, b < d, b < h, c < e, c < h, d < g, e < f
fn select_818([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_32([b, e, a, f, d])
    } else {
        select_817([c, b, a, d, e, f, g, h])
    }
}
/// n = 8, i = 3, a < e, a < h, b < d, b < h, c < f, c < h, f < g
fn select_819([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_818([a, b, c, d, f, g, e, h])
    } else {
        select_818([b, a, c, e, f, g, d, h])
    }
}
/// n = 11, i = 3, a < e, b < d, b < h, c < f, c < h, f < g, h < i, h < j, h < k
fn select_820([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_116([f, b, d, g, h, a, i, j, k])
    } else {
        select_819([a, b, c, d, e, f, g, h])
    }
}
/// n = 7, i = 2, a < g, b < f, c < d, c < f, c < g, d < e
fn select_821([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_10([a, c, b, d, e, g])
    } else {
        select_10([b, c, a, d, e, f])
    }
}
/// n = 9, i = 3, a < d, a < i, b < f, b < h, b < i, c < e, c < f, c < i, e < h, f < g
fn select_822([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_821([a, e, b, f, g, h, i])
    } else {
        select_263([b, a, c, d, e, h, i])
    }
}
/// n = 11, i = 3, a < d, b < f, b < g, b < k, c < e, c < f, c < g, e < k, f < j, g < h, g < i
fn select_823([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_120([e, f, g, a, h, i, j, k])
    } else {
        select_822([a, b, c, d, e, f, j, k, g])
    }
}
/// n = 10, i = 2, a < e, a < i, a < j, b < c, b < d, b < i, b < j, d < f, e < g, e < h
fn select_824([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_94([c, d, a, i, j, f])
    } else {
        select_65([b, a, c, e, g, h, i, j])
    }
}
/// n = 11, i = 2, a < c, a < e, a < k, b < d, b < f, b < k, d < i, e < j, f < g, f < h
fn select_825([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_64([d, f, g, h, a, i])
    } else {
        select_824([b, a, c, e, f, j, g, h, d, k])
    }
}
/// n = 12, i = 3, a < d, b < g, b < h, b < l, c < e, c < g, c < h, e < f, e < l, g < k, h < i, h < j
fn select_826([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_825([e, b, f, g, a, h, i, j, k, d, l])
    } else {
        select_210([a, b, c, d, g, e, h, i, j, k, f])
    }
}
/// n = 12, i = 3, a < e, b < d, b < h, b < i, c < f, c < h, c < i, f < g, h < l, i < j, i < k
fn select_827([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < f {
        select_823([a, c, b, e, d, h, i, j, k, l, f])
    } else {
        select_826([a, b, c, e, f, g, h, i, j, k, l, d])
    }
}
/// n = 13, i = 4, a < d, b < f, b < i, c < e, c < i, c < k, e < h, e < m, f < g, f < k, f < m, i < j, k < l
fn select_828([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_820([a, e, b, h, d, i, j, f, g, k, m])
    } else {
        select_827([a, f, c, g, d, i, j, k, e, h, m, l])
    }
}
/// n = 8, i = 3, a < e, b < d, b < h, c < f, f < g, f < h
fn select_829([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_51([c, b, a, d, e, f])
    } else {
        select_476([a, b, c, d, f, g, h, e])
    }
}
/// n = 9, i = 4, a < g, b < f, b < g, c < d, c < e, e < h, e < i, f < h, f < i
fn select_830([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < d {
        select_186([a, f, c, d, g, e, h, i])
    } else {
        select_47([e, a, b, f, g])
    }
}
/// n = 10, i = 4, a < g, b < d, b < e, c < f, c < g, e < j, f < h, f < j, h < i
fn select_831([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_829([a, b, f, d, g, h, i, e])
    } else {
        select_830([a, c, b, d, e, f, g, h, j])
    }
}
/// n = 9, i = 3, a < i, b < d, b < g, c < e, c < h, d < h, e < f, e < g, e < i
fn select_832([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_94([f, a, b, d, g, i])
    } else {
        select_84([a, d, c, e, f, g, h])
    }
}
/// n = 10, i = 4, a < e, b < f, b < i, c < g, c < j, d < f, d < g, f < j, g < h, g < i
fn select_833([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_531([c, b, a, d, e, f, j, g])
    } else {
        select_832([a, b, c, f, g, h, i, j, e])
    }
}
/// n = 9, i = 3, a < c, a < d, a < h, b < e, b < i, d < i, e < f, e < h, f < g
fn select_834([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_76([c, b, d, e, i])
    } else {
        select_63([a, e, c, f, g, d, h])
    }
}
/// n = 8, i = 4, a < f, b < f, b < h, c < d, d < e, e < h, f < g
fn select_835([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_21([a, e, b, h, f])
    } else {
        select_3([g, c, d])
    }
}
/// n = 10, i = 5, a < h, a < j, b < f, b < h, c < g, c < h, d < e, e < f, e < g, g < j, h < i
fn select_836([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_835([b, a, d, e, g, h, i, j])
    } else {
        select_51([a, b, c, f, g, j])
    }
}
/// n = 10, i = 5, a < h, a < j, b < f, b < h, c < g, c < h, d < e, d < f, e < g, g < j, h < i
fn select_837([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_836([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_835([a, c, d, f, e, h, i, g])
    }
}
/// n = 11, i = 5, a < f, b < g, b < j, c < g, c < i, d < e, d < f, e < i, e < k, g < h, g < k, i < j
fn select_838([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_834([g, d, h, a, e, i, j, k, f])
    } else {
        select_837([b, a, c, d, e, f, i, g, h, j])
    }
}
/// n = 12, i = 5, a < f, b < g, b < j, c < g, c < i, c < l, d < e, d < f, e < i, e < j, g < h, i < k, j < l
fn select_839([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_833([a, b, c, e, f, j, i, k, g, l])
    } else {
        select_838([a, c, b, d, e, f, g, h, j, l, i])
    }
}
/// n = 12, i = 5, a < h, b < j, c < e, c < i, c < l, d < f, d < h, e < g, f < i, f < j, i < k, j < l
fn select_840([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < b {
        select_831([a, e, d, g, b, f, h, i, k, j])
    } else {
        select_839([a, b, c, d, f, h, e, g, i, j, k, l])
    }
}
/// n = 14, i = 5, b < h, c < e, c < j, c < l, d < f, d < h, e < g, e < n, f < i, f < j, i < l, i < n, j < k, l < m
fn select_841([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_828([b, f, c, h, e, i, a, g, j, k, l, m, n])
    } else {
        select_840([b, a, c, d, e, f, g, h, j, i, k, l])
    }
}
/// n = 8, i = 3, a < d, a < f, b < e, b < g, c < e, c < f, c < g, g < h
fn select_842([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_71([b, c, a, d, f, e])
    } else {
        select_7([a, b, e, f, g, h])
    }
}
/// n = 9, i = 3, a < d, b < f, b < g, b < i, c < e, c < f, c < g, e < i, g < h
fn select_843([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_455([e, b, f, g, a, h, d, i])
    } else {
        select_842([a, b, c, d, f, e, g, h])
    }
}
/// n = 10, i = 4, a < e, b < g, b < j, c < f, c < h, c < i, d < f, d < g, f < j, g < h, g < i
fn select_844([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < c {
        select_359([e, b, c, d, f, g])
    } else {
        select_534([a, b, d, e, f, g, h, i, j])
    }
}
/// n = 11, i = 4, a < e, b < g, c < g, c < h, c < i, d < f, d < k, f < h, f < i, g < k, i < j
fn select_845([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_843([a, c, f, e, b, h, i, j, g])
    } else {
        select_844([a, d, c, b, e, g, f, h, i, k])
    }
}
/// n = 9, i = 3, a < i, b < f, c < f, c < g, d < e, e < g, e < i, f < h
fn select_846([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_122([a, c, e, b, g, i])
    } else {
        select_48([a, d, c, b, e, f, h])
    }
}
/// n = 9, i = 4, a < i, b < g, c < e, d < f, d < i, e < i, f < g, g < h
fn select_847([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_138([b, a, d, e, f, i])
    } else {
        select_48([a, c, b, f, e, g, h])
    }
}
/// n = 10, i = 4, a < e, b < g, c < h, c < j, d < f, d < g, f < h, g < j, h < i
fn select_848([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_846([a, f, c, b, g, h, j, i, e])
    } else {
        select_847([b, c, a, d, e, f, h, i, g])
    }
}
/// n = 11, i = 5, a < g, b < h, c < f, c < g, d < e, d < h, e < i, f < i, f < k, h < k, i < j
fn select_849([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < c {
        select_45([e, a, c, f, k, g])
    } else {
        select_848([a, b, f, d, g, e, h, i, j, k])
    }
}
/// n = 12, i = 5, a < g, b < h, c < e, c < h, c < j, d < f, d < g, f < i, f < l, h < l, i < j, j < k
fn select_850([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < e {
        select_845([a, b, c, f, g, i, h, e, j, k, l])
    } else {
        select_849([a, b, d, c, e, f, g, h, i, j, l])
    }
}
/// n = 8, i = 2, a < d, a < f, b < c, b < e, e < g, f < h
fn select_851([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_7([a, b, c, d, e, g])
    } else {
        select_7([b, a, d, c, f, h])
    }
}
/// n = 10, i = 3, a < h, b < d, b < g, c < e, c < g, c < h, e < f, g < i, h < j
fn select_852([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_851([b, e, f, d, a, g, h, i])
    } else {
        select_789([b, a, c, d, e, h, g, j, f, i])
    }
}
/// n = 9, i = 4, a < i, b < e, b < h, c < f, c < g, d < f, d < g, g < h, g < i
fn select_853([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < a {
        select_47([f, g, b, e, h])
    } else {
        select_198([a, b, c, d, e, f, g, i])
    }
}
/// n = 11, i = 4, a < g, b < i, c < e, c < h, d < f, d < g, f < h, f < i, h < j, i < k
fn select_854([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_852([b, c, f, e, a, g, h, i, j, k])
    } else {
        select_853([b, c, a, d, e, g, f, h, i])
    }
}
/// n = 8, i = 3, a < d, a < h, b < e, b < f, b < h, c < e, c < f, c < h, f < g
fn select_855([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_43([e, a, f, g, h])
    } else {
        select_137([b, c, a, d, e, h])
    }
}
/// n = 11, i = 3, a < d, b < e, b < f, b < g, c < e, c < f, c < g, f < k, g < h, g < i, g < j
fn select_856([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_311([e, f, g, a, h, i, j, k])
    } else {
        select_855([a, b, c, d, e, f, k, g])
    }
}
/// n = 13, i = 4, a < d, b < f, b < j, c < e, c < h, c < i, e < g, e < j, e < m, f < h, f < i, f < m, i < k, j < l
fn select_857([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_640([a, b, e, g, d, j, f, h, i, m, l])
    } else {
        select_856([a, c, f, d, h, i, e, g, j, m, k])
    }
}
/// n = 14, i = 5, a < h, b < f, b < h, c < d, c < e, c < j, e < g, e < k, e < n, f < i, f < k, i < j, i < n, j < l, k < m
fn select_858([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < i {
        select_854([a, d, e, b, g, f, h, k, i, m, j])
    } else {
        select_857([a, f, c, h, e, i, g, d, j, k, l, m, n])
    }
}
/// n = 14, i = 5, b < h, c < e, c < f, c < j, d < g, d < h, f < k, f < n, g < i, g < k, i < j, i < n, j < l, k < m
fn select_859([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < f {
        select_850([b, a, c, d, e, g, h, f, i, j, l, k])
    } else {
        select_858([b, d, c, e, f, g, a, h, i, j, k, l, m, n])
    }
}
/// n = 15, i = 5, b < h, c < f, c < m, d < j, d < l, e < g, e < h, g < i, g < j, i < l, i < m, j < k, l < n, m < o
fn select_860([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if d < c {
        select_841([a, b, d, e, c, g, f, h, i, j, k, l, n, m])
    } else {
        select_859([a, b, c, e, f, d, g, h, i, m, j, o, k, l])
    }
}
/// n = 9, i = 2, a < e, b < c, b < d, b < f, f < g, f < h, f < i
fn select_861([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_582([a, b, e, c, f, g, h, i, d])
    } else {
        select_582([a, b, e, d, f, g, h, i, c])
    }
}
/// n = 11, i = 3, a < d, a < g, a < h, a < i, b < c, b < g, b < h, b < i, c < e, d < f, i < j, i < k
fn select_862([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_634([c, a, g, h, e, i, j, k, d])
    } else {
        select_634([d, b, g, h, f, i, j, k, c])
    }
}
/// n = 11, i = 3, a < d, a < j, a < k, b < c, b < e, b < j, b < k, c < h, d < i, e < f, e < g
fn select_863([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_861([c, e, f, g, h, a, d, j, k])
    } else {
        select_862([a, b, c, d, h, i, j, k, e, f, g])
    }
}
/// n = 10, i = 3, a < h, b < d, b < i, c < e, c < h, e < f, e < g, e < i, h < j
fn select_864([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_407([b, e, f, g, d, a, h, i])
    } else {
        select_452([b, a, c, d, h, e, f, g, j])
    }
}
/// n = 9, i = 4, a < d, a < e, a < i, b < f, c < f, f < g, f < h, f < i
fn select_865([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_196([e, b, c, d, f])
    } else {
        select_78([a, f, g, h, e, d, i])
    }
}
/// n = 10, i = 5, a < e, a < h, b < f, c < f, d < e, d < i, e < g, f < h, f < i, i < j
fn select_866([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_10([a, d, h, i, j, e])
    } else {
        select_18([g, b, c, f])
    }
}
/// n = 11, i = 5, a < e, a < j, b < e, b < h, b < i, c < f, d < f, e < g, f < h, f < i, f < j, j < k
fn select_867([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_866([b, c, d, a, e, f, g, i, j, k])
    } else {
        select_649([a, b, h, i, j, k])
    }
}
/// n = 12, i = 5, a < j, b < e, b < h, b < i, c < g, d < g, e < f, e < k, g < h, g < i, g < j, g < k, j < l
fn select_868([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_865([e, c, d, a, f, g, h, i, k])
    } else {
        select_867([a, b, c, d, e, g, f, h, i, j, l])
    }
}
/// n = 14, i = 6, a < m, b < g, c < g, d < e, d < m, e < f, e < k, e < l, f < h, g < i, g < k, g < l, g < m, i < j, m < n
fn select_869([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < f {
        select_864([a, i, d, j, e, k, l, m, f, n])
    } else {
        select_868([a, e, b, c, f, h, g, k, l, m, i, n])
    }
}
/// n = 14, i = 6, b < g, c < g, d < e, d < k, e < f, e < m, e < n, f < h, g < i, g < k, g < m, g < n, i < j, k < l
fn select_870([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < a {
        select_863([e, g, i, f, k, a, l, j, h, m, n])
    } else {
        select_869([a, b, c, d, e, f, g, h, i, j, m, n, k, l])
    }
}
/// n = 10, i = 2, a < f, a < j, b < c, b < d, b < e, b < j, e < g, f < h, f < i
fn select_871([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_43([d, c, e, g, a])
    } else {
        select_824([a, b, d, e, f, g, h, i, c, j])
    }
}
/// n = 10, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, e < h, f < i, f < j
fn select_872([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_91([d, e, a, g, h])
    } else {
        select_19([b, a, c, d, f, i, j, e])
    }
}
/// n = 11, i = 3, a < d, b < h, b < k, c < e, c < h, e < f, e < g, e < k, h < i, h < j
fn select_873([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_871([b, e, f, g, a, h, d, i, j, k])
    } else {
        select_872([b, a, c, d, e, h, f, g, i, j])
    }
}
/// n = 8, i = 3, a < d, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_874([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_4([a, c, d, e, g])
    } else {
        select_4([b, c, d, f, h])
    }
}
/// n = 9, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < g, e < f, h < i
fn select_875([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_874([a, d, c, g, e, h, f, i])
    } else {
        select_283([a, e, b, f, d, h, i])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, c < d, c < h, d < f, e < g, e < h
fn select_876([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_10([a, c, g, d, f, h])
    } else {
        select_351([a, f, b, e])
    }
}
/// n = 9, i = 4, a < f, b < f, c < d, c < h, c < i, d < e, f < g, f < h, f < i
fn select_877([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_62([c, f, g, d, h, i])
    } else {
        select_14([a, b, e])
    }
}
/// n = 10, i = 4, a < g, a < j, b < d, b < g, c < e, c < h, c < i, c < j, d < j, e < f, g < h, g < i
fn select_878([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < g {
        select_876([a, b, c, e, d, f, g, j])
    } else {
        select_877([a, b, c, e, f, g, d, h, i])
    }
}
/// n = 11, i = 4, a < h, b < g, c < g, c < h, d < e, d < h, d < i, d < j, e < f, g < i, g < j, h < k
fn select_879([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_875([c, a, d, b, e, f, g, h, k])
    } else {
        select_878([c, b, d, a, e, f, g, i, j, h])
    }
}
/// n = 12, i = 4, b < h, b < l, c < g, c < h, d < e, d < g, d < j, d < k, e < f, g < i, g < l, h < j, h < k
fn select_880([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_873([e, b, c, f, g, a, i, h, j, k, l])
    } else {
        select_879([a, b, c, d, e, f, h, g, j, k, i])
    }
}
/// n = 9, i = 4, a < g, b < h, b < i, c < e, c < h, d < f, d < i, e < i, f < g, f < h
fn select_881([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_138([a, b, d, e, f, i])
    } else {
        select_486([b, a, c, f, e, g, h])
    }
}
/// n = 9, i = 4, b < h, b < i, c < e, c < h, d < f, d < i, e < i, f < g, f < h
fn select_882([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_818([b, c, f, e, g, a, i, h])
    } else {
        select_881([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 6, i = 4, b < f, c < f, d < f, e < f
fn select_883([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < a {
        select_196([a, c, d, e, f])
    } else {
        select_193([b, c, d, e])
    }
}
/// n = 9, i = 4, c < f, d < f, e < f, f < g, f < h, f < i
fn select_884([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_883([b, a, c, d, e, f])
    } else {
        select_104([b, f, a, g, h, i])
    }
}
/// n = 11, i = 5, b < k, c < g, d < g, e < f, e < k, f < h, f < i, f < j, g < h, g < i, g < j, g < k
fn select_885([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_135([a, b, e, g, f, h, i, j, k])
    } else {
        select_884([a, b, c, d, f, g, h, i, j])
    }
}
/// n = 13, i = 6, b < g, b < m, c < f, c < g, d < h, e < h, f < m, g < k, g < l, h < i, h < k, h < l, h < m, i < j
fn select_886([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_882([a, b, c, h, f, i, j, g, m])
    } else {
        select_885([a, f, d, e, b, g, h, i, k, l, m])
    }
}
/// n = 14, i = 6, b < f, b < g, c < g, c < k, d < h, e < h, g < m, g < n, h < i, h < k, h < m, h < n, i < j, k < l
fn select_887([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < f {
        select_880([a, b, c, h, i, j, k, g, l, m, n, f])
    } else {
        select_886([a, c, b, d, e, f, g, h, i, j, m, n, k])
    }
}
/// n = 14, i = 6, b < f, c < g, c < k, d < h, e < h, g < m, g < n, h < i, h < k, h < m, h < n, i < j, k < l
fn select_888([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_870([a, d, e, c, g, b, h, f, i, j, k, l, m, n])
    } else {
        select_887([a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 10, i = 5, a < j, b < h, c < f, d < g, d < j, e < g, e < j, f < j, g < h, h < i
fn select_889([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_847([a, b, c, e, f, g, h, i, j])
    } else {
        select_138([b, a, d, e, g, j])
    }
}
/// n = 11, i = 5, a < f, b < h, c < i, c < k, d < g, d < h, e < g, e < h, g < i, h < k, i < j
fn select_890([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_846([a, g, c, b, h, i, k, j, f])
    } else {
        select_889([b, c, a, d, e, f, g, i, j, h])
    }
}
/// n = 9, i = 3, a < f, b < h, c < e, c < g, d < g, d < h, h < i
fn select_891([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < b {
        select_345([a, d, c, e, f, b, g, h])
    } else {
        select_457([a, d, b, f, c, e, h, i, g])
    }
}
/// n = 11, i = 5, b < f, c < i, c < j, d < g, d < h, e < g, e < h, g < i, h < j, j < k
fn select_892([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_890([b, a, c, d, e, f, h, g, j, k, i])
    } else {
        select_891([b, h, g, c, a, f, i, j, k])
    }
}
/// n = 8, i = 5, a < h, b < f, c < g, d < g, e < f, e < h, g < h
fn select_893([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_40([a, b, e, g, f, h])
    } else {
        select_196([a, c, d, f, g])
    }
}
/// n = 8, i = 5, b < h, c < g, d < g, d < h, e < f, f < h
fn select_894([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_893([b, c, a, e, d, g, f, h])
    } else {
        select_808([b, c, d, f, a, g, h])
    }
}
/// n = 10, i = 6, b < i, b < j, c < g, d < f, e < h, f < i, g < j, h < i, h < j
fn select_895([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_894([a, b, d, g, e, h, f, j])
    } else {
        select_894([a, b, c, f, e, h, g, i])
    }
}
/// n = 8, i = 3, b < e, b < h, c < f, c < h, d < e, d < f, d < h, f < g
fn select_896([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_148([a, c, d, e, f, h])
    } else {
        select_218([a, b, d, f, g, e, h])
    }
}
/// n = 11, i = 3, b < e, c < f, c < g, d < e, d < f, d < g, f < k, g < h, g < i, g < j
fn select_897([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_311([a, f, g, b, h, i, j, k])
    } else {
        select_896([a, b, c, d, e, f, k, g])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < f, d < g, e < f, e < g
fn select_898([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_40([a, b, d, e, f, g])
    } else {
        select_40([a, b, c, e, f, g])
    }
}
/// n = 10, i = 4, b < j, c < f, c < j, d < f, d < j, e < f, e < j, f < g, f < h, f < i
fn select_899([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_100([b, f, a, g, h, i, j])
    } else {
        select_898([a, b, c, d, e, j, f])
    }
}
/// n = 10, i = 4, c < f, c < g, d < f, d < g, e < f, e < g, g < h, g < i, g < j
fn select_900([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_217([b, f, g, h, i, j, a])
    } else {
        select_899([b, a, c, d, e, g, h, i, j, f])
    }
}
/// n = 13, i = 5, b < i, c < f, c < j, d < g, d < h, e < g, e < h, f < h, f < l, f < m, g < i, g < j, g < l, g < m, j < k
fn select_901([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_897([a, b, c, g, i, j, f, h, l, m, k])
    } else {
        select_900([a, b, d, e, f, h, g, j, l, m])
    }
}
/// n = 14, i = 6, b < f, c < i, c < j, d < g, d < k, e < h, g < j, g < m, g < n, h < i, h < j, i < k, i < m, i < n, k < l
fn select_902([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < i {
        select_895([a, c, b, d, e, g, f, h, j, i])
    } else {
        select_901([a, b, d, c, h, g, i, j, f, k, l, m, n])
    }
}
/// n = 14, i = 6, b < f, c < g, c < k, d < i, d < j, e < h, g < m, g < n, h < i, h < j, j < k, j < m, j < n, k < l
fn select_903([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_892([a, b, c, d, h, f, i, j, g, k, l])
    } else {
        select_902([a, b, d, c, e, f, g, h, j, i, k, l, m, n])
    }
}
/// n = 14, i = 6, b < f, c < h, c < k, d < i, d < j, e < g, e < i, g < j, h < m, h < n, i < k, i < m, i < n, k < l
fn select_904([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_888([a, b, c, d, e, f, h, i, g, j, k, l, m, n])
    } else {
        select_903([a, b, c, d, e, f, h, g, j, i, k, l, m, n])
    }
}
/// n = 16, i = 6, b < f, b < o, c < h, c < l, d < i, d < j, e < g, e < i, g < j, h < n, i < k, i < l, i < n, k < o, l < m, o < p
fn select_905([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < h {
        select_860([a, g, b, c, d, f, i, j, k, l, m, h, o, n, p])
    } else {
        select_904([a, b, c, d, e, f, g, h, i, j, l, m, k, n])
    }
}
/// n = 16, i = 6, b < g, b < o, c < f, c < m, d < i, d < k, e < h, e < i, h < k, i < j, i < l, i < m, l < o, m < n, o < p
fn select_906([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < f {
        select_816([a, h, c, b, d, g, i, k, j, l, m, n, f, o, p])
    } else {
        select_905([a, b, c, d, e, g, h, f, i, k, l, m, n, j, o, p])
    }
}
/// n = 16, i = 6, b < i, b < o, c < h, c < m, d < g, d < j, e < f, e < j, j < k, j < l, j < m, l < o, m < n, o < p
fn select_907([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < g {
        select_906([a, b, c, d, e, h, i, f, j, k, g, l, m, n, o, p])
    } else {
        select_906([a, b, c, e, d, h, i, g, j, k, f, l, m, n, o, p])
    }
}
/// n = 9, i = 3, a < e, a < h, a < i, b < c, b < d, b < g, d < h, d < i, e < f, e < g
fn select_908([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < e {
        select_147([c, d, a, h, i, e])
    } else {
        select_65([e, b, f, d, h, i, c, g])
    }
}
/// n = 10, i = 3, a < c, a < d, b < e, b < i, b < j, d < i, d < j, e < f, e < g, g < h
fn select_909([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_42([f, g, a, h])
    } else {
        select_908([b, a, c, d, e, f, g, i, j])
    }
}
/// n = 10, i = 3, a < c, a < f, b < d, b < e, b < i, b < j, d < h, e < g, f < i, f < j
fn select_910([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_909([a, b, c, f, d, h, e, g, i, j])
    } else {
        select_909([a, b, c, f, e, g, d, h, i, j])
    }
}
/// n = 10, i = 3, a < e, a < g, b < c, b < d, b < f, b < i, b < j, f < h, g < i, g < j
fn select_911([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_910([a, b, e, c, f, g, h, d, i, j])
    } else {
        select_910([a, b, e, d, f, g, h, c, i, j])
    }
}
/// n = 7, i = 4, b < f, b < g, c < f, c < g, d < e, e < g
fn select_912([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_137([b, c, e, a, f, g])
    } else {
        select_60([b, c, a, d, e, f])
    }
}
/// n = 7, i = 4, b < g, c < g, d < f, d < g, e < f, e < g
fn select_913([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_912([a, d, e, b, c, f, g])
    } else {
        select_912([a, d, e, c, b, f, g])
    }
}
/// n = 10, i = 5, a < e, a < f, b < h, c < g, c < h, d < g, d < h, h < i, h < j
fn select_914([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_477([g, a, b, f, h, i, j, e])
    } else {
        select_913([f, b, e, c, d, g, h])
    }
}
/// n = 12, i = 5, a < g, b < e, b < f, c < h, d < g, d < h, f < k, f < l, h < i, h < j, h < k, h < l
fn select_915([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_911([b, h, i, j, e, a, f, g, k, l])
    } else {
        select_914([b, c, a, d, e, f, g, h, i, j])
    }
}
/// n = 9, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, g < h, g < i
fn select_916([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_92([d, f, g, a, h, i])
    } else {
        select_467([b, a, c, d, e, f, g])
    }
}
/// n = 9, i = 3, a < e, b < g, b < h, b < i, c < d, c < f, c < g, f < h, f < i
fn select_917([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_655([d, a, f, e, b, g, h, i])
    } else {
        select_916([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 11, i = 5, a < g, b < h, c < e, c < j, d < g, d < h, e < f, e < k, h < i, h < j, h < k
fn select_918([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_192([a, b, d, e, f, g, h, i, j, k])
    } else {
        select_916([a, c, h, i, g, j, e, f, k])
    }
}
/// n = 12, i = 5, a < g, b < e, b < k, b < l, c < h, d < g, d < h, e < f, h < i, h < j, h < k, j < l
fn select_919([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < e {
        select_917([a, b, h, i, g, j, k, e, l])
    } else {
        select_918([a, c, b, d, e, f, g, h, i, k, j])
    }
}
/// n = 10, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, g < h, g < i, g < j
fn select_920([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_92([d, f, g, h, i, j])
    } else {
        select_916([a, b, c, d, e, f, g, i, j])
    }
}
/// n = 13, i = 5, a < g, b < h, c < e, c < j, c < k, d < g, d < h, e < f, h < i, h < j, h < k, k < l, k < m
fn select_921([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if k < e {
        select_920([a, c, h, i, g, j, k, e, l, m])
    } else {
        select_918([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 13, i = 5, a < g, b < e, b < k, b < l, c < h, d < g, d < h, e < f, h < i, h < j, h < k, h < l, l < m
fn select_922([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < l {
        select_919([a, b, c, d, e, f, g, h, j, i, k, l])
    } else {
        select_921([a, c, b, d, e, f, g, h, j, k, l, i, m])
    }
}
/// n = 13, i = 5, a < g, b < f, c < k, c < l, d < h, e < g, e < h, h < i, h < j, h < k, h < l, l < m
fn select_923([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < c {
        select_915([a, b, d, e, f, c, g, h, i, j, k, l])
    } else {
        select_922([a, c, d, e, b, f, g, h, i, j, k, l, m])
    }
}
/// n = 10, i = 3, a < c, a < i, a < j, b < d, c < f, d < e, d < g, d < i, d < j, g < h
fn select_924([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < c {
        select_94([e, g, a, i, j, h])
    } else {
        select_35([a, b, c, d, f, g, h, i, j])
    }
}
/// n = 10, i = 3, a < d, b < i, b < j, c < e, e < f, e < g, e < i, e < j, g < h
fn select_925([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_292([a, c, b, d, e, f, g, h])
    } else {
        select_924([b, c, a, e, f, d, g, h, i, j])
    }
}
/// n = 8, i = 3, b < e, c < f, d < f, f < g, f < h
fn select_926([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_51([d, b, a, e, c, f])
    } else {
        select_185([b, d, c, a, e, f, g, h])
    }
}
/// n = 9, i = 4, b < f, c < g, d < g, e < f, e < g, g < h, g < i
fn select_927([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_926([a, b, c, d, f, g, h, i])
    } else {
        select_360([c, d, b, e, f, g])
    }
}
/// n = 11, i = 4, a < k, b < f, c < i, c < j, d < g, e < f, e < g, g < h, g < i, g < j, g < k
fn select_928([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_925([b, c, d, f, g, h, a, k, i, j])
    } else {
        select_927([c, b, a, d, e, f, g, h, k])
    }
}
/// n = 10, i = 6, a < i, a < j, b < g, c < h, c < j, d < h, d < j, e < f, e < j, f < i, g < j, h < i
fn select_929([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_718([a, c, d, e, g, f, h, j])
    } else {
        select_893([a, b, c, d, f, g, h, i])
    }
}
/// n = 10, i = 6, a < i, a < j, b < g, c < f, c < j, d < h, d < j, e < h, e < j, g < j, h < i
fn select_930([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < i {
        select_929([a, b, d, e, c, f, g, h, i, j])
    } else {
        select_531([c, h, b, a, g, i, f, j])
    }
}
/// n = 9, i = 6, a < i, b < h, c < g, d < g, e < f, f < h, g < i
fn select_931([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < h {
        select_191([b, a, g, e, f, i, h])
    } else {
        select_196([a, c, d, h, g])
    }
}
/// n = 9, i = 6, a < i, b < g, c < f, d < h, e < h, h < i
fn select_932([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_931([a, b, d, e, c, f, h, g, i])
    } else {
        select_931([a, c, d, e, b, g, h, f, i])
    }
}
/// n = 10, i = 6, b < j, c < i, c < j, d < g, e < h, e < j, f < h, f < j, g < j, h < i
fn select_933([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < a {
        select_930([c, d, b, e, f, a, g, h, i, j])
    } else {
        select_932([c, a, d, e, f, g, b, h, i])
    }
}
/// n = 13, i = 6, a < g, b < l, b < m, c < j, d < i, d < j, e < h, e < j, f < h, f < j, h < i, j < k, j < l, j < m
fn select_934([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < g {
        select_928([a, h, b, c, d, i, j, k, l, m, g])
    } else {
        select_933([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 9, i = 4, b < e, c < g, c < i, d < f, d < i, e < i, f < g, g < h
fn select_935([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_138([a, c, d, e, f, i])
    } else {
        select_48([a, b, c, f, e, g, h])
    }
}
/// n = 9, i = 5, b < f, c < h, d < g, e < g, g < h, h < i
fn select_936([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_48([a, b, c, g, f, h, i])
    } else {
        select_379([a, c, d, e, f, g])
    }
}
/// n = 10, i = 5, b < h, c < f, d < h, d < j, e < g, f < j, g < h, g < j, h < i
fn select_937([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_935([a, c, d, g, f, b, h, i, j])
    } else {
        select_936([a, c, d, b, e, f, g, h, i])
    }
}
/// n = 7, i = 4, a < e, b < f, c < f, d < f, f < g
fn select_938([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_3([a, f, g])
    } else {
        select_193([b, c, d, e])
    }
}
/// n = 7, i = 4, c < f, d < f, e < f, f < g
fn select_939([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_938([a, c, d, e, b, f, g])
    } else {
        select_938([b, c, d, e, a, f, g])
    }
}
/// n = 10, i = 5, b < i, c < f, d < h, d < i, e < g, e < h, f < h, g < i, i < j
fn select_940([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_937([a, b, c, d, e, f, g, i, j, h])
    } else {
        select_939([a, b, d, e, f, h, g])
    }
}
/// n = 8, i = 4, a < h, b < g, c < f, d < e, d < h, e < g, f < h
fn select_941([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_138([b, a, d, f, e, h])
    } else {
        select_359([a, b, c, e, f, g])
    }
}
/// n = 8, i = 4, a < h, b < h, c < h, d < g, e < f, e < h, f < g
fn select_942([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_941([c, d, a, e, f, b, g, h])
    } else {
        select_941([c, d, b, e, f, a, g, h])
    }
}
/// n = 8, i = 3, a < e, b < e, c < f, d < f, f < g, f < h
fn select_943([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_91([b, f, a, g, h])
    } else {
        select_360([c, d, b, a, e, f])
    }
}
/// n = 9, i = 4, b < g, c < h, d < h, e < f, e < h, f < g, h < i
fn select_944([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < h {
        select_942([a, c, d, b, e, f, g, h])
    } else {
        select_943([b, f, c, d, g, h, a, i])
    }
}
/// n = 10, i = 5, b < f, c < i, d < h, d < i, e < g, e < h, g < i, i < j
fn select_945([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_940([a, c, b, d, e, f, g, h, i, j])
    } else {
        select_944([a, b, c, g, d, h, f, i, j])
    }
}
/// n = 13, i = 6, a < g, b < l, b < m, c < j, d < j, e < i, e < j, f < h, f < j, h < i, j < k, j < l, j < m
fn select_946([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < h {
        select_934([a, b, d, e, c, f, g, h, i, j, k, l, m])
    } else {
        select_945([b, a, d, e, h, g, c, i, j, k])
    }
}
/// n = 14, i = 6, b < g, c < l, c < m, d < j, e < i, e < j, f < h, f < j, h < i, j < k, j < l, j < m, m < n
fn select_947([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < a {
        select_923([h, b, c, d, e, g, i, j, a, k, l, m, n])
    } else {
        select_946([b, c, a, d, e, f, g, h, i, j, k, l, m])
    }
}
/// n = 14, i = 6, b < i, c < l, c < m, d < j, e < h, e < j, f < g, f < j, j < k, j < l, j < m, m < n
fn select_948([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < h {
        select_947([a, b, c, d, e, f, i, g, h, j, k, l, m, n])
    } else {
        select_947([a, b, c, d, f, e, i, h, g, j, k, l, m, n])
    }
}
/// n = 9, i = 3, a < d, b < e, b < i, c < e, c < h, d < h, d < i, e < f, e < g
fn select_949([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_42([a, e, f, g])
    } else {
        select_21([b, c, d, h, i])
    }
}
/// n = 11, i = 5, a < e, b < h, b < k, c < g, c < h, d < f, e < g, e < k, f < h, f < k, h < i, h < j
fn select_950([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_51([b, c, d, g, f, k])
    } else {
        select_949([a, b, c, e, h, i, j, g, k])
    }
}
/// n = 12, i = 6, a < e, b < h, b < i, c < g, d < f, e < h, e < l, f < i, f < l, g < i, g < l, i < j, i < k
fn select_951([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < h {
        select_950([a, g, b, d, e, f, h, i, j, k, l])
    } else {
        select_46([c, d, f, g])
    }
}
/// n = 7, i = 4, b < f, c < f, c < g, d < e, e < g
fn select_952([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < e {
        select_52([a, d, c, f, e, g])
    } else {
        select_47([a, b, c, g, f])
    }
}
/// n = 8, i = 5, b < e, c < g, d < f, f < h, g < h
fn select_953([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_952([a, b, f, c, g, e, h])
    } else {
        select_382([a, d, e, c, g, f])
    }
}
/// n = 13, i = 7, a < k, b < f, c < i, c < k, d < h, e < g, f < i, f < j, g < j, g < k, h < j, h < k, k < l, k < m
fn select_954([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < f {
        select_951([b, c, d, e, f, g, h, i, k, l, m, j])
    } else {
        select_953([a, c, d, e, i, g, h, j])
    }
}
/// n = 8, i = 4, a < h, b < g, c < e, d < f, f < g, f < h
fn select_955([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_471([c, d, a, e, b, f, h, g])
    } else {
        select_471([c, d, b, e, a, f, g, h])
    }
}
/// n = 14, i = 7, a < k, a < l, b < i, b < l, c < f, d < g, e < h, f < l, g < i, g < j, h < j, h < l, j < k, l < m, l < n
fn select_956([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < j {
        select_954([a, d, b, c, e, g, h, f, i, j, l, m, n])
    } else {
        select_955([a, c, b, h, i, j, f, k])
    }
}
/// n = 8, i = 2, a < h, b < g, b < h, c < d, c < g, d < e, d < f
fn select_957([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < b {
        select_91([a, d, b, e, f])
    } else {
        select_147([a, c, b, d, g, h])
    }
}
/// n = 7, i = 3, a < f, b < d, b < g, c < e, c < f, e < g
fn select_958([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_10([b, e, d, a, f, g])
    } else {
        select_809([b, a, c, d, e, f])
    }
}
/// n = 10, i = 4, a < g, b < f, b < g, c < f, c < h, d < e, d < g, e < h, h < i, h < j
fn select_959([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_957([a, b, c, h, i, j, f, g])
    } else {
        select_958([a, c, d, f, e, g, h])
    }
}
/// n = 11, i = 5, a < k, b < f, b < k, c < f, c < h, d < g, e < g, g < h, g < k, h < i, h < j
fn select_960([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_957([a, b, c, h, i, j, f, k])
    } else {
        select_354([a, c, d, e, f, g, h, k])
    }
}
/// n = 11, i = 5, a < h, b < g, b < h, c < i, d < g, d < i, e < f, f < h, f < i, i < j, i < k
fn select_961([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < c {
        select_959([a, b, d, f, c, g, h, i, j, k])
    } else {
        select_960([a, b, d, c, e, g, f, i, j, k, h])
    }
}
/// n = 12, i = 5, a < h, b < g, b < h, c < i, c < j, d < g, d < j, e < f, f < j, h < i, j < k, j < l
fn select_962([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_955([c, e, d, a, g, h, f, i])
    } else {
        select_961([a, b, c, d, e, f, g, h, j, k, l])
    }
}
/// n = 14, i = 7, a < f, b < k, b < l, c < j, c < l, d < g, e < h, f < j, g < l, h < i, h < l, i < k, l < m, l < n
fn select_963([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < i {
        select_956([b, c, d, a, e, g, f, h, j, i, k, l, m, n])
    } else {
        select_962([a, i, c, b, d, g, k, f, j, l, m, n])
    }
}
/// n = 7, i = 4, a < g, b < e, c < f, d < f, f < g
fn select_964([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_21([a, b, f, e, g])
    } else {
        select_196([a, c, d, e, f])
    }
}
/// n = 12, i = 6, a < f, b < i, b < l, c < h, c < i, d < g, e < g, f < h, f < l, g < i, g < l, i < j, i < k
fn select_965([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_949([a, b, c, f, i, j, k, h, l])
    } else {
        select_964([b, c, d, e, h, g, l])
    }
}
/// n = 10, i = 4, a < e, b < g, b < j, c < g, c < j, d < f, d < g, e < f, e < j, g < h, g < i
fn select_966([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_162([b, c, d, f, j])
    } else {
        select_949([a, c, d, e, g, h, i, f, j])
    }
}
/// n = 12, i = 6, a < f, b < i, b < j, c < h, c < j, d < g, d < i, e < g, e < i, f < h, f < i, g < j, j < k, j < l
fn select_967([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_966([a, c, g, b, f, i, j, k, l, h])
    } else {
        select_611([b, d, e, c, h, i, g])
    }
}
/// n = 13, i = 7, a < f, b < j, b < k, c < i, c < k, d < h, e < g, e < j, f < i, f < j, g < k, h < j, h < k, k < l, k < m
fn select_968([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_965([a, b, c, d, g, f, h, i, k, l, m, j])
    } else {
        select_967([a, b, c, e, h, f, g, i, j, k, l, m])
    }
}
/// n = 14, i = 7, a < i, a < l, b < f, c < g, d < j, d < l, e < h, e < j, f < l, g < i, g < j, h < k, h < l, j < k, l < m, l < n
fn select_969([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < j {
        select_968([c, d, a, b, e, g, h, f, i, j, l, m, n])
    } else {
        select_955([b, h, a, d, i, j, k, f])
    }
}
/// n = 14, i = 7, a < f, b < j, b < l, c < g, d < i, d < l, e < h, e < i, f < j, g < l, h < k, h < l, i < k, l < m, l < n
fn select_970([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < i {
        select_969([b, c, a, d, e, g, f, h, j, i, k, l, m, n])
    } else {
        select_962([a, i, b, h, c, g, k, f, j, l, m, n])
    }
}
/// n = 14, i = 7, a < f, b < k, b < l, c < j, c < l, d < g, e < h, e < j, f < k, g < l, h < i, h < l, l < m, l < n
fn select_971([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < j {
        select_963([a, c, b, d, e, f, g, h, i, k, j, l, m, n])
    } else {
        select_970([a, b, d, c, e, f, g, h, j, k, i, l, m, n])
    }
}
/// n = 9, i = 5, a < e, b < g, c < g, d < f, d < h, f < i, g < h, g < i
fn select_972([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_381([e, b, c, d, f, g, h])
    } else {
        select_7([a, d, h, e, f, i])
    }
}
/// n = 10, i = 5, a < g, b < e, b < j, c < g, c < h, d < f, d < i, f < h, g < i, h < j
fn select_973([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_972([b, a, c, d, e, f, g, i, h])
    } else {
        select_354([a, b, c, f, e, h, j, g])
    }
}
/// n = 8, i = 4, a < h, b < g, c < e, c < h, d < f, f < g, f < h
fn select_974([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_486([a, b, c, f, e, g, h])
    } else {
        select_378([a, b, d, e, f])
    }
}
/// n = 10, i = 5, a < h, b < e, b < i, c < g, c < h, d < f, d < g, f < i, f < j, g < i, h < j
fn select_975([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_972([b, a, c, d, e, f, h, g, j])
    } else {
        select_974([f, a, b, c, e, g, h, i])
    }
}
/// n = 10, i = 5, a < i, a < j, b < g, c < h, d < f, d < j, e < g, e < h, g < j, h < i
fn select_976([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_973([c, d, e, a, f, b, h, g, i, j])
    } else {
        select_975([c, d, e, b, f, a, g, h, j, i])
    }
}
/// n = 9, i = 5, a < i, b < g, c < f, d < h, e < g, e < h, h < i
fn select_977([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_955([a, c, b, d, g, h, f, i])
    } else {
        select_894([a, d, b, e, c, f, g, h])
    }
}
/// n = 11, i = 6, a < j, b < i, b < k, c < g, d < f, d < k, e < h, g < i, h < j, j < k
fn select_978([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_976([b, a, c, d, h, f, j, g, i, k])
    } else {
        select_977([a, b, d, e, g, f, i, h, j])
    }
}
/// n = 11, i = 6, a < j, a < k, b < h, c < g, d < i, e < f, e < k, g < k, h < k, i < j
fn select_979([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_978([b, a, d, e, c, f, i, g, j, h, k])
    } else {
        select_978([c, a, d, e, b, f, i, h, j, g, k])
    }
}
/// n = 14, i = 7, a < g, b < l, c < k, c < l, d < h, e < f, e < i, g < k, h < l, i < j, i < l, l < m, l < n
fn select_980([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < f {
        select_971([a, c, b, d, e, g, h, i, j, f, k, l, m, n])
    } else {
        select_979([c, d, f, a, i, j, b, h, g, k, l])
    }
}
/// n = 8, i = 4, a < e, b < f, c < f, d < e, d < g, d < h, f < g, f < h
fn select_981([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_15([b, c, d, e, f])
    } else {
        select_42([a, d, g, h])
    }
}
/// n = 9, i = 5, a < g, b < g, b < h, b < i, c < f, d < e, e < h, e < i, f < h, f < i
fn select_982([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_981([a, c, e, b, g, f, h, i])
    } else {
        select_981([a, d, f, b, g, e, h, i])
    }
}
/// n = 11, i = 5, a < g, b < g, b < k, c < f, d < e, e < h, e < k, f < h, f < k, h < i, h < j
fn select_983([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < b {
        select_91([a, h, b, i, j])
    } else {
        select_982([a, b, c, d, e, f, g, h, k])
    }
}
/// n = 12, i = 6, a < h, b < h, b < i, c < j, d < g, e < f, f < i, f < j, g < i, g < j, j < k, j < l
fn select_984([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < b {
        select_983([a, b, d, e, f, g, h, j, k, l, i])
    } else {
        select_485([c, a, d, e, h, f, g, i, j])
    }
}
/// n = 13, i = 6, a < h, b < h, b < i, c < j, c < k, d < f, e < g, f < k, g < i, g < k, i < j, k < l, k < m
fn select_985([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_955([c, d, a, g, h, i, f, j])
    } else {
        select_984([a, b, c, d, e, g, f, h, i, k, l, m])
    }
}
/// n = 9, i = 4, b < f, c < g, d < f, d < g, e < f, e < g, g < h, g < i
fn select_986([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_221([b, c, f, g, a, h, i])
    } else {
        select_712([a, c, b, d, e, f, g])
    }
}
/// n = 11, i = 5, b < g, c < h, c < i, d < f, e < g, e < i, f < i, g < h, i < j, i < k
fn select_987([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_986([a, b, c, e, f, g, i, j, k])
    } else {
        select_359([a, c, d, g, f, h])
    }
}
/// n = 7, i = 3, a < g, b < f, c < e, d < e, d < f, d < g
fn select_988([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_359([b, a, c, d, e, g])
    } else {
        select_359([c, a, b, d, f, g])
    }
}
/// n = 12, i = 5, a < i, b < g, c < h, c < j, d < f, e < g, e < j, f < j, g < h, g < i, j < k, j < l
fn select_989([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_988([a, c, d, g, f, h, i])
    } else {
        select_986([a, b, c, e, f, g, j, k, l])
    }
}
/// n = 12, i = 5, a < i, b < g, b < h, c < g, c < j, d < f, e < h, e < j, f < j, h < i, j < k, j < l
fn select_990([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_987([a, b, e, d, c, f, g, h, j, k, l])
    } else {
        select_989([a, b, c, d, e, f, h, g, i, j, k, l])
    }
}
/// n = 14, i = 7, a < j, b < k, b < l, c < g, d < f, d < l, e < h, f < j, g < l, h < i, h < l, i < k, l < m, l < n
fn select_991([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < i {
        select_985([a, f, b, c, e, g, h, j, i, k, l, m, n])
    } else {
        select_990([a, i, b, c, d, g, k, f, j, l, m, n])
    }
}
/// n = 8, i = 3, a < h, b < h, c < e, d < e, d < h, e < f, e < g
fn select_992([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_91([b, e, a, f, g])
    } else {
        select_40([b, c, a, d, e, h])
    }
}
/// n = 10, i = 5, a < g, b < g, c < h, d < g, d < h, e < f, f < h, h < i, h < j
fn select_993([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_992([a, b, c, d, h, i, j, g])
    } else {
        select_191([c, b, d, e, f, g, h])
    }
}
/// n = 12, i = 6, a < h, b < h, c < i, c < j, d < g, e < f, e < i, f < j, g < j, h < i, j < k, j < l
fn select_994([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_993([a, b, c, g, e, f, h, j, k, l])
    } else {
        select_808([c, d, h, e, f, g, i])
    }
}
/// n = 13, i = 6, a < h, b < h, b < i, c < f, d < i, d < k, e < g, e < i, f < k, g < j, g < k, i < j, k < l, k < m
fn select_995([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < h {
        select_989([a, b, g, c, d, f, i, j, h, k, l, m])
    } else {
        select_994([a, b, d, c, e, g, f, h, i, k, l, m])
    }
}
/// n = 14, i = 7, a < j, b < g, c < i, c < l, d < f, d < l, e < h, e < i, f < j, g < l, h < k, h < l, i < k, l < m, l < n
fn select_996([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < i {
        select_995([a, f, b, c, e, g, h, j, i, k, l, m, n])
    } else {
        select_990([a, i, h, b, d, g, k, f, j, l, m, n])
    }
}
/// n = 14, i = 7, a < k, b < j, b < l, c < g, d < f, d < l, e < h, e < j, f < k, g < l, h < i, h < l, l < m, l < n
fn select_997([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < j {
        select_991([a, b, c, d, e, f, g, h, i, k, j, l, m, n])
    } else {
        select_996([a, c, b, d, e, f, g, h, j, k, i, l, m, n])
    }
}
/// n = 9, i = 4, a < h, b < g, b < i, c < e, d < f, d < i, e < i, f < g, f < h
fn select_998([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_138([a, b, d, e, f, i])
    } else {
        select_988([a, b, c, f, e, g, h])
    }
}
/// n = 9, i = 5, b < h, b < i, c < g, d < g, d < h, e < f, e < i, f < h, g < i
fn select_999([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < f {
        select_139([a, b, d, e, f, g, i, h])
    } else {
        select_359([a, b, c, d, g, h])
    }
}
/// n = 10, i = 5, a < i, b < h, c < f, c < j, d < g, d < j, e < g, e < h, g < i, h < j
fn select_1000([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_998([a, c, b, d, h, g, f, i, j])
    } else {
        select_999([a, d, b, e, c, f, h, g, j])
    }
}
/// n = 11, i = 6, a < i, b < j, c < f, c < k, d < h, e < g, e < k, g < i, h < j, j < k
fn select_1001([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_1000([a, b, c, e, h, f, g, j, i, k])
    } else {
        select_977([b, a, c, d, g, f, i, h, j])
    }
}
/// n = 11, i = 6, a < j, b < i, c < h, d < f, d < k, e < g, e < k, g < j, h < k, i < k
fn select_1002([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_1001([a, b, d, c, e, f, g, h, j, i, k])
    } else {
        select_1001([a, c, d, b, e, f, g, i, j, h, k])
    }
}
/// n = 14, i = 7, a < k, b < l, c < h, d < g, d < l, e < f, e < i, g < k, h < l, i < j, i < l, l < m, l < n
fn select_1003([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < f {
        select_997([a, b, c, d, e, g, h, i, j, f, k, l, m, n])
    } else {
        select_1002([a, c, f, i, d, j, g, b, h, k, l])
    }
}
/// n = 14, i = 7, a < h, b < l, c < i, d < g, d < l, e < f, e < j, i < l, j < k, j < l, l < m, l < n
fn select_1004([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_980([a, b, d, c, e, f, h, i, j, k, g, l, m, n])
    } else {
        select_1003([a, b, c, d, e, f, g, i, j, k, h, l, m, n])
    }
}
/// n = 15, i = 7, a < i, b < h, b < n, c < l, d < g, d < l, e < f, e < j, j < k, j < l, l < m, l < n, n < o
fn select_1005([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if l < h {
        select_948([f, a, b, c, d, j, k, g, i, l, m, h, n, o])
    } else {
        select_1004([a, c, b, d, e, f, g, i, h, j, k, l, m, n])
    }
}
/// n = 9, i = 4, b < e, b < i, c < f, d < f, f < g, f < i, g < h
fn select_1006([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_174([b, f, a, e, g, h, i])
    } else {
        select_938([b, a, c, d, e, f, g])
    }
}
/// n = 8, i = 5, a < f, b < g, c < g, d < g, e < f, e < h, g < h
fn select_1007([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_18([a, e, g, h])
    } else {
        select_193([b, c, d, f])
    }
}
/// n = 9, i = 5, b < f, c < g, d < g, e < f, e < h, g < h, h < i
fn select_1008([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_17([b, e, g, a, f, h, i])
    } else {
        select_1007([b, a, c, d, e, f, g, h])
    }
}
/// n = 10, i = 5, c < g, d < g, e < f, e < h, f < j, g < h, g < j, h < i
fn select_1009([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_1006([b, f, c, d, a, g, h, i, j])
    } else {
        select_1008([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 9, i = 5, c < g, d < g, e < f, e < h, f < i, g < h, g < i
fn select_1010([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_972([a, c, d, e, b, f, g, h, i])
    } else {
        select_972([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 10, i = 5, c < h, d < i, e < g, f < g, g < h, g < i, i < j
fn select_1011([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < c {
        select_1009([a, b, e, f, d, c, g, i, j, h])
    } else {
        select_1010([a, b, e, f, c, d, g, h, i])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < e, c < f, c < g
fn select_1012([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_218([a, b, c, d, e, f, g])
    } else {
        select_218([a, b, c, e, d, f, g])
    }
}
/// n = 10, i = 5, b < f, b < j, c < g, d < g, e < g, g < h, g < i, g < j
fn select_1013([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_1012([a, b, g, h, i, f, j])
    } else {
        select_883([a, c, d, e, f, g])
    }
}
/// n = 7, i = 1, d < e, d < f, d < g
fn select_1014([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_92([b, c, d, e, f, g])
    } else {
        select_618([a, b, c, d, f, g])
    }
}
/// n = 10, i = 4, a < e, b < f, c < f, d < f, f < g, f < h, f < i, f < j
fn select_1015([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_99([a, g, h, i, j])
    } else {
        select_196([e, b, c, d, f])
    }
}
/// n = 10, i = 4, c < f, d < f, e < f, f < g, f < h, f < i, f < j
fn select_1016([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1015([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_1015([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 11, i = 5, b < f, b < j, b < k, c < g, d < g, e < g, g < h, g < i, g < j, g < k
fn select_1017([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_1014([a, h, i, b, f, j, k])
    } else {
        select_1016([a, f, c, d, e, g, h, i, j, k])
    }
}
/// n = 12, i = 5, b < l, c < j, c < k, d < g, e < g, f < g, g < h, g < i, g < j, g < k, g < l
fn select_1018([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < c {
        select_1013([a, b, d, e, f, c, g, h, i, l])
    } else {
        select_1017([a, c, d, e, f, b, g, h, i, j, k])
    }
}
/// n = 13, i = 5, b < l, b < m, c < j, c < k, d < h, e < g, e < h, f < g, f < h, g < j, h < i, h < k, h < l, h < m
fn select_1019([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_1011([a, b, c, d, e, f, g, j, h, i])
    } else {
        select_1018([a, c, b, d, e, f, h, g, i, l, m, k])
    }
}
/// n = 9, i = 5, a < h, b < e, c < g, d < g, e < f, g < h, h < i
fn select_1020([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_236([a, b, g, e, h, i, f])
    } else {
        select_196([a, c, d, f, g])
    }
}
/// n = 10, i = 4, a < g, a < j, b < e, b < i, c < d, c < g, c < j, e < f, e < j, g < h, g < i
fn select_1021([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_148([d, b, g, h, e, i])
    } else {
        select_673([a, c, e, f, d, g, j])
    }
}
/// n = 11, i = 5, b < h, b < k, c < e, c < j, d < f, e < g, e < k, f < h, f < k, h < i, h < j
fn select_1022([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_1020([b, c, a, d, e, g, f, h, i])
    } else {
        select_1021([b, c, f, a, e, g, h, i, j, k])
    }
}
/// n = 8, i = 4, c < h, d < g, d < h, e < f, f < g, f < h
fn select_1023([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_487([c, a, d, e, b, f, g, h])
    } else {
        select_487([c, b, d, e, a, f, g, h])
    }
}
/// n = 12, i = 6, b < i, c < i, c < j, d < f, d < l, e < g, f < h, f < j, g < i, g < j, i < k, i < l
fn select_1024([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < f {
        select_1022([a, c, d, e, f, g, h, i, k, l, j])
    } else {
        select_1023([a, h, b, c, e, g, j, i])
    }
}
/// n = 7, i = 4, b < g, c < f, d < e, d < g, f < g
fn select_1025([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_187([b, d, f, a, e, g])
    } else {
        select_382([b, a, c, d, e, f])
    }
}
/// n = 8, i = 6, a < h, b < h, c < h, d < g, e < f
fn select_1026([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_608([b, c, d, e, f, g, h])
    } else {
        select_608([a, c, d, e, f, g, h])
    }
}
/// n = 8, i = 6, c < h, d < h, e < h, f < g
fn select_1027([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_1026([c, d, e, a, f, g, b, h])
    } else {
        select_1026([c, d, e, b, f, g, a, h])
    }
}
/// n = 9, i = 6, c < i, d < h, e < h, f < g, g < i, h < i
fn select_1028([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < a {
        select_1025([b, c, f, h, a, g, i])
    } else {
        select_1027([b, c, a, d, e, f, g, h])
    }
}
/// n = 9, i = 5, b < f, b < i, c < g, d < g, e < g, g < h, g < i
fn select_1029([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_148([a, b, g, h, f, i])
    } else {
        select_883([a, c, d, e, f, g])
    }
}
/// n = 5, i = 4
fn select_1030([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_193([b, c, d, e])
    } else {
        select_193([a, c, d, e])
    }
}
/// n = 8, i = 5, a < f, b < g, c < g, d < g, e < g, g < h
fn select_1031([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_3([a, g, h])
    } else {
        select_1030([b, c, d, e, f])
    }
}
/// n = 10, i = 5, b < f, b < i, b < j, c < g, d < g, e < g, g < h, g < i, g < j
fn select_1032([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_1031([b, a, c, d, e, f, g, h])
    } else {
        select_78([b, g, a, h, f, i, j])
    }
}
/// n = 11, i = 5, b < k, c < i, c < j, d < g, e < g, f < g, g < h, g < i, g < j, g < k
fn select_1033([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_1029([a, b, d, e, f, c, g, h, k])
    } else {
        select_1032([a, c, d, e, f, b, g, h, i, j])
    }
}
/// n = 14, i = 6, b < h, b < m, c < h, c < k, c < l, d < i, e < i, e < n, f < g, g < i, g < n, h < n, i < j, i < k, i < l, i < m
fn select_1034([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_1028([a, d, e, b, c, f, g, h, n])
    } else {
        select_1033([a, b, c, d, e, g, i, j, k, l, m])
    }
}
/// n = 14, i = 6, b < m, b < n, c < g, c < l, d < i, e < i, e < k, f < h, g < k, h < i, h < k, i < j, i < l, i < m, i < n
fn select_1035([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_1024([a, d, e, c, f, g, h, b, i, k, j, l])
    } else {
        select_1034([a, c, b, d, e, f, h, g, i, j, m, n, l, k])
    }
}
/// n = 14, i = 6, b < m, b < n, c < g, c < l, d < j, e < i, e < j, f < h, h < i, h < j, j < k, j < l, j < m, j < n
fn select_1036([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_1019([a, b, c, d, e, h, i, j, k, g, l, m, n])
    } else {
        select_1035([a, b, c, d, e, f, g, h, j, k, i, l, m, n])
    }
}
/// n = 8, i = 2, b < d, b < g, b < h, c < e, c < g, c < h, e < f
fn select_1037([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_148([a, c, b, d, g, h])
    } else {
        select_218([a, b, c, e, f, g, h])
    }
}
/// n = 9, i = 2, b < f, b < i, c < d, c < e, c < i, e < g, f < h
fn select_1038([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_77([a, d, e, b, g])
    } else {
        select_1037([a, c, b, d, f, h, e, i])
    }
}
/// n = 12, i = 5, b < f, b < k, b < l, c < g, d < g, e < g, g < h, g < i, g < k, g < l, i < j
fn select_1039([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_121([a, h, i, b, f, k, l, j])
    } else {
        select_1016([a, f, c, d, e, g, h, i, k, l])
    }
}
/// n = 15, i = 6, b < f, b < j, b < l, c < h, d < h, e < h, f < g, f < o, h < i, h < j, h < l, h < o, i < k, k < m, l < n
fn select_1040([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_1038([a, i, b, j, l, k, n, m, f])
    } else {
        select_1039([a, f, c, d, e, g, h, j, l, n, i, o])
    }
}
/// n = 9, i = 2, b < e, b < f, b < i, c < d, c < g, c < i, g < h
fn select_1041([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_1014([a, e, f, c, d, g, i])
    } else {
        select_121([a, d, g, b, e, f, i, h])
    }
}
/// n = 15, i = 6, b < f, b < l, b < m, c < h, d < h, e < h, f < g, f < o, h < i, h < l, h < m, h < o, i < j, i < k, m < n
fn select_1042([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_1041([a, i, b, l, j, k, m, n, f])
    } else {
        select_1039([a, f, c, d, e, g, h, l, m, n, i, o])
    }
}
/// n = 15, i = 6, b < f, b < l, b < m, c < h, d < h, e < h, f < g, f < o, h < i, h < j, h < l, h < m, h < o, j < k, m < n
fn select_1043([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < j {
        select_1040([a, b, c, d, e, f, g, h, i, l, j, m, k, n, o])
    } else {
        select_1042([a, b, c, d, e, f, g, h, j, i, k, l, m, n, o])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < h, c < d, c < f, c < h, f < g
fn select_1044([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_10([a, c, e, f, g, h])
    } else {
        select_45([d, a, b, f, g, h])
    }
}
/// n = 10, i = 3, a < e, b < e, b < f, b < g, c < d, c < f, c < g, f < j, g < h, g < i
fn select_1045([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_217([d, f, g, a, h, i, j])
    } else {
        select_1044([a, b, c, d, e, f, j, g])
    }
}
/// n = 10, i = 3, a < g, b < g, b < h, b < j, c < d, c < e, c < h, e < f, e < j, h < i
fn select_1046([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_95([a, d, e, f, b, h, j])
    } else {
        select_1045([a, b, c, d, g, h, e, f, j, i])
    }
}
/// n = 9, i = 2, a < d, a < g, a < h, a < i, b < c, b < e, b < g, b < h, b < i, e < f
fn select_1047([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_217([c, e, a, g, h, i, f])
    } else {
        select_732([a, b, c, d, g, h, i])
    }
}
/// n = 12, i = 3, a < e, a < h, a < l, b < c, b < d, b < h, b < j, b < k, d < f, d < l, e < g, e < j, e < k, h < i
fn select_1048([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_1037([c, d, a, f, h, i, e, l])
    } else {
        select_1047([e, b, c, g, h, i, d, j, k])
    }
}
/// n = 12, i = 3, b < f, b < h, b < l, c < d, c < e, c < h, c < j, c < k, e < g, e < l, f < j, f < k, h < i
fn select_1049([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < f {
        select_1046([a, b, c, d, e, g, f, h, i, l])
    } else {
        select_1048([b, c, d, e, f, g, a, h, i, j, k, l])
    }
}
/// n = 9, i = 1, b < i, c < g, c < h, d < e, d < f
fn select_1050([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_217([a, b, c, d, g, h, i])
    } else {
        select_217([a, b, d, c, e, f, i])
    }
}
/// n = 13, i = 5, a < l, a < m, b < f, b < k, c < g, d < g, e < g, g < h, g < i, g < k, g < l, g < m, i < j
fn select_1051([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < b {
        select_1050([h, i, a, b, f, k, l, m, j])
    } else {
        select_884([a, f, c, d, e, g, h, i, k])
    }
}
/// n = 9, i = 5, a < g, b < g, c < g, d < e, d < f, d < i, g < h, g < i
fn select_1052([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_62([d, g, h, f, e, i])
    } else {
        select_883([f, a, b, c, e, g])
    }
}
/// n = 13, i = 6, a < g, b < h, c < h, d < h, e < f, e < g, e < j, f < l, f < m, h < i, h < j, h < l, h < m, j < k
fn select_1053([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_219([i, a, e, j, f, l, m, k, g])
    } else {
        select_1052([b, c, d, e, f, g, h, i, j])
    }
}
/// n = 14, i = 6, b < h, c < h, d < h, e < f, e < g, e < j, f < n, g < l, g < m, h < i, h < j, h < l, h < m, h < n, j < k
fn select_1054([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < a {
        select_1051([g, f, b, c, d, a, h, i, j, k, n, l, m])
    } else {
        select_1053([a, b, c, d, e, g, f, h, i, j, k, l, m])
    }
}
/// n = 15, i = 6, b < f, b < g, b < l, c < h, d < h, e < h, g < n, g < o, h < i, h < j, h < l, h < n, h < o, j < k, l < m
fn select_1055([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < f {
        select_1049([a, b, h, i, j, g, k, l, m, n, o, f])
    } else {
        select_1054([a, c, d, e, b, f, g, h, i, l, m, n, o, j])
    }
}
/// n = 16, i = 6, b < l, b < n, c < g, c < m, d < h, e < h, f < h, h < i, h < j, h < l, h < m, h < n, j < k, m < o, n < p
fn select_1056([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if b < c {
        select_1043([a, b, d, e, f, c, g, h, i, j, k, l, n, p, m])
    } else {
        select_1055([a, c, d, e, f, g, b, h, i, j, k, m, o, l, n])
    }
}
/// n = 16, i = 6, b < l, b < n, c < g, c < m, d < j, e < i, e < j, f < h, f < j, h < i, j < k, j < l, j < m, j < n, m < o, n < p
fn select_1057([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < j {
        select_1036([a, b, c, d, e, f, g, h, i, j, k, m, l, n])
    } else {
        select_1056([a, b, c, d, e, f, g, j, k, h, i, l, m, n, o, p])
    }
}
/// n = 16, i = 6, b < l, b < n, c < i, c < m, d < j, e < h, e < j, f < g, f < j, j < k, j < l, j < m, j < n, m < o, n < p
fn select_1058([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < h {
        select_1057([a, b, c, d, e, f, i, g, h, j, k, l, m, n, o, p])
    } else {
        select_1057([a, b, c, d, f, e, i, h, g, j, k, l, m, n, o, p])
    }
}
/// n = 17, i = 7, a < i, a < o, b < h, b < n, c < l, d < g, d < l, e < f, e < j, j < k, j < l, l < m, l < n, l < o, n < p, o < q
fn select_1059([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if h < l {
        select_1004([a, c, b, d, e, f, g, i, h, j, k, l, m, n])
    } else {
        select_1058([f, b, a, c, d, j, k, g, i, l, m, h, o, n, q, p])
    }
}
/// n = 17, i = 7, a < i, a < p, b < h, b < l, c < g, c < m, d < l, d < m, e < f, e < j, j < k, j < l, j < m, l < o, m < n, m < p, p < q
fn select_1060([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if l < m {
        select_1005([a, c, d, b, e, f, h, g, i, j, k, l, o, m, n])
    } else {
        select_1059([a, b, d, c, e, f, g, h, i, j, k, m, n, l, p, o, q])
    }
}
/// n = 17, i = 7, a < n, b < i, b < p, c < h, c < n, d < g, d < l, e < f, e < j, j < k, j < l, j < n, l < m, l < p, n < o, p < q
fn select_1061([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if l < a {
        select_907([f, c, b, d, j, k, g, i, h, l, m, a, p, q, n, o])
    } else {
        select_1060([b, c, d, a, e, f, g, h, i, j, k, n, l, m, o, p, q])
    }
}
/// n = 17, i = 7, b < i, b < p, c < h, c < l, d < g, d < m, e < f, e < j, j < k, j < l, j < m, l < o, m < n, m < p, p < q
fn select_1062([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if l < a {
        select_747([f, b, c, d, j, k, g, h, i, l, m, n, a, o, p, q])
    } else {
        select_1061([a, b, c, d, e, f, g, h, i, j, k, m, n, l, o, p, q])
    }
}
/// n = 17, i = 7, b < i, b < o, c < h, c < n, d < g, d < j, e < f, e < k, j < m, j < o, k < l, k < n, n < p, o < q
fn select_1063([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < k {
        select_1062([a, c, b, e, d, g, f, i, h, j, m, o, k, l, q, n, p])
    } else {
        select_1062([a, b, c, d, e, f, g, h, i, k, l, n, j, m, p, o, q])
    }
}
/// n = 17, i = 7, b < i, b < p, c < h, c < k, d < g, d < j, e < f, e < l, j < n, k < o, l < m, l < p, p < q
fn select_1064([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < k {
        select_1063([a, b, c, e, d, g, f, h, i, l, j, n, m, k, p, o, q])
    } else {
        select_1063([a, b, d, e, c, h, f, g, i, l, k, o, m, j, p, n, q])
    }
}
/// n = 17, i = 7, b < k, c < j, d < i, d < p, e < h, e < l, f < g, f < m, l < o, m < n, m < p, p < q
fn select_1065([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if b < c {
        select_1064([a, d, b, e, f, g, h, k, i, l, c, m, n, o, j, p, q])
    } else {
        select_1064([a, d, c, e, f, g, h, j, i, l, b, m, n, o, k, p, q])
    }
}
/// n = 17, i = 7, b < m, c < l, d < k, e < j, f < i, f < p, g < h, g < n, n < o, n < p, p < q
fn select_1066([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if c < e {
        select_1065([a, b, d, f, c, g, h, l, i, k, m, e, n, o, j, p, q])
    } else {
        select_1065([a, b, d, f, e, g, h, j, i, k, m, c, n, o, l, p, q])
    }
}
/// n = 17, i = 7, d < m, e < l, f < k, g < j, g < p, h < i, h < n, n < o, n < p, p < q
fn select_1067([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1066([c, a, d, e, f, g, h, i, j, k, l, m, b, n, o, p, q])
    } else {
        select_1066([c, b, d, e, f, g, h, i, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 7, f < m, g < l, h < k, h < p, i < j, i < n, n < o, n < p, p < q
fn select_1068([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1067([c, d, e, a, f, g, h, i, j, k, l, m, b, n, o, p, q])
    } else {
        select_1067([c, d, e, b, f, g, h, i, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 7, h < m, i < l, i < p, j < k, j < n, n < o, n < p, p < q
fn select_1069([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1068([c, d, e, f, g, a, h, i, j, k, l, m, b, n, o, p, q])
    } else {
        select_1068([c, d, e, f, g, b, h, i, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 7, j < m, j < p, k < l, k < n, n < o, n < p, p < q
fn select_1070([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1069([c, d, e, f, g, h, i, a, j, k, l, m, b, n, o, p, q])
    } else {
        select_1069([c, d, e, f, g, h, i, b, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 7, j < m, j < o, k < l, k < n, n < p, o < q
fn select_1071([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if n < o {
        select_1070([a, b, c, d, e, f, g, h, i, j, k, l, m, n, p, o, q])
    } else {
        select_1070([a, b, c, d, e, f, g, h, i, k, j, m, l, o, q, n, p])
    }
}
/// n = 17, i = 7, j < o, k < n, l < m, l < p, p < q
fn select_1072([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < k {
        select_1071([a, b, c, d, e, f, g, h, i, j, l, m, o, p, k, q, n])
    } else {
        select_1071([a, b, c, d, e, f, g, h, i, k, l, m, n, p, j, q, o])
    }
}
/// n = 17, i = 7, l < o, m < n, m < p, p < q
fn select_1073([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1072([c, d, e, f, g, h, i, j, k, a, l, m, n, o, b, p, q])
    } else {
        select_1072([c, d, e, f, g, h, i, j, k, b, l, m, n, o, a, p, q])
    }
}
/// n = 17, i = 7, n < o, n < p, p < q
fn select_1074([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1073([c, d, e, f, g, h, i, j, k, l, m, a, n, o, b, p, q])
    } else {
        select_1073([c, d, e, f, g, h, i, j, k, l, m, b, n, o, a, p, q])
    }
}
/// n = 17, i = 7, n < q, o < p
fn select_1075([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if n < o {
        select_1074([a, b, c, d, e, f, g, h, i, j, k, l, m, n, q, o, p])
    } else {
        select_1074([a, b, c, d, e, f, g, h, i, j, k, l, m, o, p, n, q])
    }
}
/// n = 17, i = 7, p < q
fn select_1076([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1075([c, d, e, f, g, h, i, j, k, l, m, n, o, a, p, q, b])
    } else {
        select_1075([c, d, e, f, g, h, i, j, k, l, m, n, o, b, p, q, a])
    }
}
/// n = 17, i = 7
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1076([c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, a, b])
    } else {
        select_1076([c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, b, a])
    }
}
