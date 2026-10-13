#[allow(unused)]
pub const N: usize = 17;
#[allow(unused)]
pub const I: usize = 8;

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
/// n = 2, i = 1
fn select_5([a, b]: [usize; 2]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_1([a])
    }
}
/// n = 4, i = 2, b < d, c < d
fn select_6([a, b, c, d]: [usize; 4]) -> usize {
    if b < a {
        select_3([a, c, d])
    } else {
        select_5([b, c])
    }
}
/// n = 4, i = 2, c < d
fn select_7([a, b, c, d]: [usize; 4]) -> usize {
    if a < d {
        select_6([b, a, c, d])
    } else {
        select_3([b, d, a])
    }
}
/// n = 7, i = 4, a < g, b < d, c < e, d < g, e < f
fn select_8([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_4([a, c, g, e, f])
    } else {
        select_7([a, f, b, d])
    }
}
/// n = 4, i = 1, b < c, b < d
fn select_9([a, b, c, d]: [usize; 4]) -> usize {
    if c < a {
        select_2([c, d])
    } else {
        select_3([a, b, d])
    }
}
/// n = 8, i = 3, a < d, a < h, b < c, b < g, c < e, c < h, d < f, d < g
fn select_10([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_9([e, a, d, h])
    } else {
        select_9([f, b, c, g])
    }
}
/// n = 8, i = 4, a < f, b < f, c < d, c < h, d < e, f < g, f < h
fn select_11([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_6([e, a, b, f])
    } else {
        select_9([g, c, d, h])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, c < d, c < i, d < f, d < h, e < g, e < h, e < i
fn select_12([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_10([c, e, a, d, g, f, h, i])
    } else {
        select_11([a, b, c, d, f, e, g, i])
    }
}
/// n = 9, i = 4, a < d, a < i, b < f, c < e, c < i, e < g, e < h, f < h, f < i
fn select_13([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_8([b, a, c, d, e, g, f])
    } else {
        select_12([a, b, c, e, f, g, d, h, i])
    }
}
/// n = 3, i = 0
fn select_14([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_2([a, c])
    } else {
        select_2([b, c])
    }
}
/// n = 4, i = 0
fn select_15([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_14([a, c, d])
    } else {
        select_14([b, c, d])
    }
}
/// n = 5, i = 0
fn select_16([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_15([a, c, d, e])
    } else {
        select_15([b, c, d, e])
    }
}
/// n = 8, i = 2, a < c, b < d, d < e, d < f, d < g, d < h
fn select_17([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_16([a, e, f, g, h])
    } else {
        select_3([c, b, d])
    }
}
/// n = 5, i = 2, a < e, b < d, c < d, c < e
fn select_18([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_3([b, c, e])
    } else {
        select_3([a, c, d])
    }
}
/// n = 5, i = 1, a < e, b < c, b < d
fn select_19([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_2([b, e])
    } else {
        select_14([a, c, d])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < g, c < d, d < f, d < g, d < h
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_19([a, b, f, g, h])
    } else {
        select_4([b, c, e, d, h])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, c < d, c < i, d < g, d < h, e < f, e < h, e < i
fn select_21([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_18([a, f, c, d, i])
    } else {
        select_20([a, d, b, e, g, f, h, i])
    }
}
/// n = 12, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < g, d < i, e < f, e < i, h < j, h < k, h < l
fn select_22([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_17([d, c, g, h, e, j, k, l])
    } else {
        select_21([a, c, b, d, e, f, g, i, h])
    }
}
/// n = 14, i = 6, a < h, a < k, b < g, c < e, c < k, d < f, d < k, e < i, e < j, f < h, f < i, g < j, g < k, k < l, k < m, k < n
fn select_23([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < g {
        select_13([a, b, c, h, e, g, i, j, k])
    } else {
        select_22([a, d, c, f, e, j, h, k, i, l, m, n])
    }
}
/// n = 3, i = 2
fn select_24([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_5([b, c])
    } else {
        select_5([a, c])
    }
}
/// n = 6, i = 4, a < f, b < f, c < e, d < e
fn select_25([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_24([a, b, e])
    } else {
        select_24([c, d, f])
    }
}
/// n = 5, i = 3, a < e, b < e, c < d
fn select_26([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_24([a, b, d])
    } else {
        select_5([c, e])
    }
}
/// n = 6, i = 4, b < f, c < f, d < e
fn select_27([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_25([b, c, a, d, e, f])
    } else {
        select_26([b, c, e, a, f])
    }
}
/// n = 8, i = 5, a < h, b < g, b < h, c < e, d < f, e < h, f < g
fn select_28([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < h {
        select_27([a, b, f, c, e, g])
    } else {
        select_5([d, h])
    }
}
/// n = 8, i = 5, a < g, b < e, c < g, c < h, d < f, f < h
fn select_29([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_28([a, c, b, d, e, f, h, g])
    } else {
        select_8([b, a, d, g, f, h, e])
    }
}
/// n = 6, i = 3, a < e, b < e, b < f, c < d, d < f
fn select_30([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_6([a, b, d, f])
    } else {
        select_5([c, e])
    }
}
/// n = 7, i = 4, a < d, b < f, c < e, e < g, f < g
fn select_31([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_30([a, e, b, f, d, g])
    } else {
        select_26([c, d, b, f, e])
    }
}
/// n = 8, i = 3, a < d, b < e, c < e, e < f, e < g, e < h
fn select_32([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_15([a, f, g, h])
    } else {
        select_6([d, b, c, e])
    }
}
/// n = 12, i = 5, a < f, a < g, b < f, b < h, c < g, c < h, d < e, e < h, e < l, g < l, h < i, h < j, h < k
fn select_33([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_31([b, c, d, f, e, g, l])
    } else {
        select_32([a, c, e, f, h, i, j, k])
    }
}
/// n = 13, i = 6, a < e, a < h, b < h, b < j, c < f, d < g, e < i, f < j, g < i, g < j, j < k, j < l, j < m
fn select_34([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_29([c, b, e, d, h, g, f, i])
    } else {
        select_33([a, b, f, d, g, h, e, j, k, l, m, i])
    }
}
/// n = 15, i = 7, a < j, a < l, b < f, c < g, d < e, d < l, e < h, e < j, f < k, f < l, g < i, g < l, i < k, l < m, l < n, l < o
fn select_35([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_23([a, b, g, d, i, e, f, j, h, k, l, m, n, o])
    } else {
        select_34([e, a, b, c, h, f, g, j, i, l, m, n, o])
    }
}
/// n = 15, i = 7, a < k, a < l, b < g, c < f, d < e, d < l, e < h, e < k, f < i, f < l, g < j, g < l, l < m, l < n, l < o
fn select_36([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < j {
        select_35([a, b, c, d, e, g, f, h, i, k, j, l, m, n, o])
    } else {
        select_35([a, c, b, d, e, f, g, h, j, k, i, l, m, n, o])
    }
}
/// n = 6, i = 1, a < e, a < f, b < c, b < d
fn select_37([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_14([b, e, f])
    } else {
        select_14([a, c, d])
    }
}
/// n = 7, i = 1, a < g, b < f, c < d, c < e
fn select_38([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_37([a, c, d, e, b, g])
    } else {
        select_37([b, c, d, e, a, f])
    }
}
/// n = 7, i = 2, a < f, a < g, b < d, b < e, c < d, c < e, c < f, c < g
fn select_39([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_9([b, c, f, g])
    } else {
        select_9([a, c, d, e])
    }
}
/// n = 8, i = 2, a < h, b < f, b < g, c < d, c < f, c < g, c < h, d < e
fn select_40([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_37([b, d, a, e, f, g])
    } else {
        select_39([a, b, c, f, g, d, h])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < e, c < g, c < h, e < f
fn select_41([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_38([d, e, b, g, h, f, a])
    } else {
        select_40([a, b, c, e, f, g, h, d])
    }
}
/// n = 8, i = 4, a < e, a < f, b < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_42([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_4([a, d, f, g, h])
    } else {
        select_26([b, c, d, e, f])
    }
}
/// n = 6, i = 3, a < d, b < e, c < e, e < f
fn select_43([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < d {
        select_3([a, e, f])
    } else {
        select_24([b, c, d])
    }
}
/// n = 8, i = 4, a < f, b < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_44([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_42([a, b, c, d, e, f, g, h])
    } else {
        select_43([e, b, c, a, g, h])
    }
}
/// n = 6, i = 2, a < c, b < d, d < e, d < f
fn select_45([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_14([a, e, f])
    } else {
        select_3([c, b, d])
    }
}
/// n = 6, i = 2, a < d, b < c, b < e, e < f
fn select_46([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < e {
        select_4([a, b, d, c, e])
    } else {
        select_45([a, b, d, e, c, f])
    }
}
/// n = 8, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_47([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_44([a, b, c, d, e, f, g, h])
    } else {
        select_46([f, d, e, a, g, h])
    }
}
/// n = 10, i = 4, a < f, b < i, b < j, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_48([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_41([e, b, g, h, a, f, i, j])
    } else {
        select_47([b, a, c, d, e, f, g, h])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < f
fn select_49([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_18([a, d, b, e, f])
    } else {
        select_18([a, e, c, d, f])
    }
}
/// n = 6, i = 2, a < c, a < f, b < d, b < f, d < e
fn select_50([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_2([a, e])
    } else {
        select_9([c, b, d, f])
    }
}
/// n = 6, i = 2, a < e, a < f, b < c, b < d, b < f
fn select_51([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_50([a, b, e, c, d, f])
    } else {
        select_50([a, b, e, d, c, f])
    }
}
/// n = 8, i = 4, a < f, a < g, b < e, b < g, c < d, c < e, c < f, g < h
fn select_52([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_49([d, a, b, e, f, g])
    } else {
        select_51([g, c, e, f, h, d])
    }
}
/// n = 5, i = 2, a < d, a < e, b < c, b < e
fn select_53([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_3([c, a, e])
    } else {
        select_3([d, b, e])
    }
}
/// n = 6, i = 2, a < d, a < e, a < f, b < c, b < e, b < f
fn select_54([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_9([c, a, e, f])
    } else {
        select_9([d, b, e, f])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, c < d, c < e, c < f, c < g
fn select_55([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_53([a, c, d, b, g])
    } else {
        select_54([b, c, d, a, e, f])
    }
}
/// n = 7, i = 1, a < f, a < g, b < c, b < d, b < e
fn select_56([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_14([b, f, g])
    } else {
        select_15([a, c, d, e])
    }
}
/// n = 6, i = 2, a < f, b < d, b < e, c < d, c < e, c < f
fn select_57([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_3([b, c, f])
    } else {
        select_9([a, c, d, e])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < g, c < h, d < e, d < f
fn select_58([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_56([b, d, a, e, f, g, h])
    } else {
        select_57([a, b, c, g, h, d])
    }
}
/// n = 9, i = 3, a < d, a < f, a < i, b < c, b < e, b < g, b < h, e < i, f < g, f < h
fn select_59([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_55([d, f, b, c, g, h, e])
    } else {
        select_58([c, e, a, f, g, h, d, i])
    }
}
/// n = 10, i = 4, a < h, b < e, b < g, c < d, c < g, c < h, d < i, d < j, e < f, e < i, e < j
fn select_60([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_52([a, b, c, d, g, h, e, f])
    } else {
        select_59([c, e, f, g, a, d, i, j, h])
    }
}
/// n = 11, i = 5, a < g, a < h, b < e, b < h, c < d, c < f, c < g, f < j, f < k, h < i, h < j, h < k
fn select_61([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_48([d, f, b, a, g, e, h, i, j, k])
    } else {
        select_60([e, a, c, f, h, i, g, d, j, k])
    }
}
/// n = 6, i = 1, a < f, b < c, b < d, b < e
fn select_62([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_2([b, f])
    } else {
        select_15([a, c, d, e])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, d < e, d < f, d < g
fn select_63([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_15([a, e, f, g])
    } else {
        select_9([c, b, d, h])
    }
}
/// n = 8, i = 2, a < d, b < c, b < e, e < f, e < g, e < h
fn select_64([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < a {
        select_62([c, e, f, g, h, a])
    } else {
        select_63([a, b, d, e, f, g, h, c])
    }
}
/// n = 8, i = 2, c < d, c < e, e < f, e < g, e < h
fn select_65([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_64([a, c, d, b, e, f, g, h])
    } else {
        select_64([b, c, d, a, e, f, g, h])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, e < i, e < j, f < g, f < h
fn select_66([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_65([a, b, c, d, e, f, i, j])
    } else {
        select_65([a, b, c, d, f, e, g, h])
    }
}
/// n = 7, i = 2, a < c, b < d, d < e, d < f, d < g
fn select_67([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_15([a, e, f, g])
    } else {
        select_3([c, b, d])
    }
}
/// n = 8, i = 2, b < h, c < d, c < h, d < e, d < f, d < g
fn select_68([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_67([a, c, b, d, e, f, g])
    } else {
        select_63([b, c, a, d, e, f, g, h])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_69([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_68([a, d, b, f, g, h, i, e])
    } else {
        select_68([a, e, c, f, g, h, i, d])
    }
}
/// n = 13, i = 5, a < g, a < i, b < e, b < j, b < k, c < d, c < g, e < f, e < i, g < h, g < j, g < k, k < l, k < m
fn select_70([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_66([d, h, b, j, e, k, l, m, f, i])
    } else {
        select_69([f, a, c, d, i, g, h, j, k])
    }
}
/// n = 13, i = 5, a < f, a < g, b < j, b < k, c < g, c < h, d < e, d < h, h < i, h < j, h < k, k < l, k < m
fn select_71([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_61([c, d, a, f, e, b, g, h, i, j, k])
    } else {
        select_70([c, b, d, e, a, f, h, i, g, j, k, l, m])
    }
}
/// n = 15, i = 7, a < f, a < n, b < k, b < l, c < g, d < e, d < l, e < h, e < k, f < j, f < o, g < i, g < l, l < m, l < n, l < o
fn select_72([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < l {
        select_36([b, a, c, d, e, g, f, h, i, j, k, l, m, n, o])
    } else {
        select_71([e, a, b, g, i, h, k, l, m, n, f, j, o])
    }
}
/// n = 5, i = 2, a < e, b < c, b < d
fn select_73([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_4([a, b, e, c, d])
    } else {
        select_4([a, b, e, d, c])
    }
}
/// n = 7, i = 3, a < d, a < g, b < e, b < g, c < e, c < g, e < f
fn select_74([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if g < e {
        select_6([d, b, c, g])
    } else {
        select_43([a, b, c, d, e, f])
    }
}
/// n = 7, i = 3, a < g, b < d, b < g, c < e, c < g, e < f
fn select_75([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_73([b, e, a, f, d])
    } else {
        select_74([b, a, c, d, e, f, g])
    }
}
/// n = 9, i = 3, b < d, b < g, c < e, c < g, e < f, g < h, g < i
fn select_76([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_64([e, b, d, f, g, a, h, i])
    } else {
        select_75([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 3, b < f, b < g, c < d, c < e, c < g, g < h, g < i
fn select_77([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_76([a, b, c, f, d, e, g, h, i])
    } else {
        select_76([a, b, c, f, e, d, g, h, i])
    }
}
/// n = 11, i = 3, b < e, b < f, c < d, c < g, g < h, g < i, i < j, i < k
fn select_78([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_66([a, d, g, h, b, i, j, k, e, f])
    } else {
        select_77([a, c, b, e, f, d, g, h, i])
    }
}
/// n = 7, i = 2, c < d, d < e, d < f, d < g
fn select_79([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_67([a, c, b, d, e, f, g])
    } else {
        select_67([b, c, a, d, e, f, g])
    }
}
/// n = 6, i = 3, b < e, c < d, c < f, e < f
fn select_80([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_53([c, e, a, d, f])
    } else {
        select_26([a, b, c, d, e])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, c < d, c < g, c < h, e < f, e < h
fn select_81([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_80([d, b, a, g, e, h])
    } else {
        select_49([a, c, e, f, d, h])
    }
}
/// n = 10, i = 4, a < g, a < h, b < e, b < h, c < d, c < g, c < h, e < f, h < i, h < j
fn select_82([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_79([d, g, b, h, e, i, j])
    } else {
        select_81([a, b, c, d, e, f, g, h])
    }
}
/// n = 10, i = 4, a < g, a < h, b < f, b < g, b < h, c < d, c < e, c < h, h < i, h < j
fn select_83([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_82([a, c, b, f, d, e, g, h, i, j])
    } else {
        select_82([a, c, b, f, e, d, g, h, i, j])
    }
}
/// n = 12, i = 4, a < e, a < f, b < g, b < h, c < d, c < g, c < h, h < i, h < j, j < k, j < l
fn select_84([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_66([d, g, h, i, a, j, k, l, e, f])
    } else {
        select_83([b, c, a, e, f, d, g, h, i, j])
    }
}
/// n = 12, i = 4, a < e, a < f, b < i, b < j, c < d, c < g, c < i, g < h, g < j, j < k, j < l
fn select_85([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_78([d, a, g, h, e, f, b, i, j, k, l])
    } else {
        select_84([a, b, c, d, e, f, i, g, h, j, k, l])
    }
}
/// n = 7, i = 1, b < g, c < d, c < e, c < f
fn select_86([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_62([a, c, d, e, f, b])
    } else {
        select_56([b, c, d, e, f, a, g])
    }
}
/// n = 7, i = 1, d < e, d < f, d < g
fn select_87([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_86([c, a, d, e, f, g, b])
    } else {
        select_86([c, b, d, e, f, g, a])
    }
}
/// n = 9, i = 1, d < h, d < i, e < f, e < g
fn select_88([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_87([a, b, c, d, e, h, i])
    } else {
        select_87([a, b, c, e, d, f, g])
    }
}
/// n = 6, i = 3, a < e, a < f, b < e, b < f, c < d, c < f
fn select_89([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_6([d, a, b, f])
    } else {
        select_3([e, c, f])
    }
}
/// n = 7, i = 4, a < f, b < e, b < g, c < d, c < g, f < g
fn select_90([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_89([c, f, b, e, d, g])
    } else {
        select_26([a, d, b, e, f])
    }
}
/// n = 9, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_91([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_15([a, g, h, i])
    } else {
        select_53([b, c, d, e, f])
    }
}
/// n = 9, i = 3, a < e, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_92([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_67([d, b, a, f, g, h, i])
    } else {
        select_91([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, a < f, a < g, b < e, b < g, c < d, c < g, g < h, g < i, g < j
fn select_93([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_92([d, b, a, f, e, g, h, i, j])
    } else {
        select_92([e, c, a, f, d, g, h, i, j])
    }
}
/// n = 10, i = 4, b < g, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_94([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_90([a, c, d, e, f, b, g])
    } else {
        select_93([b, c, d, e, f, a, g, h, i, j])
    }
}
/// n = 13, i = 5, a < f, a < g, b < h, c < g, c < h, d < e, d < h, h < i, h < j, h < k, k < l, k < m
fn select_95([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_88([e, i, j, a, k, l, m, f, g])
    } else {
        select_94([f, b, c, d, e, g, h, i, j, k])
    }
}
/// n = 14, i = 6, a < e, a < h, b < i, c < f, c < k, d < h, d < i, f < g, f < l, i < j, i < k, i < l, l < m, l < n
fn select_96([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < f {
        select_84([a, c, i, j, e, h, k, f, g, l, m, n])
    } else {
        select_95([a, b, d, f, g, e, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 6, a < e, a < h, b < f, b < m, c < i, d < h, d < i, f < g, f < n, i < j, i < k, i < m, k < l, k < n
fn select_97([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < f {
        select_85([a, b, i, j, e, h, k, l, m, f, g, n])
    } else {
        select_96([a, c, b, d, e, f, g, h, i, j, m, k, l, n])
    }
}
/// n = 15, i = 7, a < g, a < n, b < k, b < l, c < f, c < l, d < e, d < l, e < h, e < k, f < i, f < o, g < j, g < o, l < m, l < n, l < o
fn select_98([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < l {
        select_72([a, b, c, d, e, g, f, h, i, j, k, l, m, n, o])
    } else {
        select_97([e, a, c, b, h, g, j, k, l, m, f, i, n, o])
    }
}
/// n = 4, i = 2, a < d, b < c
fn select_99([a, b, c, d]: [usize; 4]) -> usize {
    if c < d {
        select_5([a, c])
    } else {
        select_5([b, d])
    }
}
/// n = 7, i = 4, a < f, b < f, c < d, d < e, f < g
fn select_100([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_99([c, f, g, d])
    } else {
        select_24([a, b, e])
    }
}
/// n = 10, i = 5, a < f, a < g, b < g, b < i, c < g, c < i, d < e, e < f, e < i, g < h, i < j
fn select_101([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < i {
        select_100([b, c, d, e, f, g, h])
    } else {
        select_43([a, b, c, f, i, j])
    }
}
/// n = 11, i = 6, a < g, a < h, b < g, b < j, c < e, d < f, e < i, e < j, f < h, f < j, g < i, j < k
fn select_102([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_101([a, b, e, d, f, h, g, i, j, k])
    } else {
        select_100([c, g, d, f, h, e, i])
    }
}
/// n = 6, i = 3, a < d, b < c, c < e, d < f
fn select_103([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < c {
        select_3([f, b, c])
    } else {
        select_3([e, a, d])
    }
}
/// n = 9, i = 4, a < e, a < g, b < e, b < h, c < d, d < g, d < h, e < f, h < i
fn select_104([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < h {
        select_103([b, c, d, e, g, f])
    } else {
        select_4([a, b, g, h, i])
    }
}
/// n = 10, i = 5, a < d, a < g, b < e, c < f, d < h, e < h, e < i, f < g, f < i, i < j
fn select_105([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_104([a, e, c, f, d, h, g, i, j])
    } else {
        select_100([b, d, c, f, g, e, h])
    }
}
/// n = 11, i = 6, a < h, b < f, c < g, d < e, d < j, e < i, f < i, f < j, g < h, g < j, j < k
fn select_106([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < e {
        select_102([a, d, b, c, f, g, e, h, i, j, k])
    } else {
        select_105([e, c, b, a, g, f, i, h, j, k])
    }
}
/// n = 12, i = 7, a < j, b < e, c < g, d < f, e < j, e < k, f < h, f < k, g < i, g < k, k < l
fn select_107([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < i {
        select_106([a, c, b, f, h, g, e, j, i, k, l])
    } else {
        select_106([a, d, b, g, i, f, e, j, h, k, l])
    }
}
/// n = 6, i = 1, b < f, c < d, c < e
fn select_108([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_19([a, c, d, e, b])
    } else {
        select_37([b, c, d, e, a, f])
    }
}
/// n = 7, i = 2, a < c, a < g, b < d, b < g, d < e, d < f
fn select_109([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_14([a, e, f])
    } else {
        select_9([c, b, d, g])
    }
}
/// n = 8, i = 2, a < d, a < h, b < c, b < e, b < h, e < f, e < g
fn select_110([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_108([c, a, e, f, g, h])
    } else {
        select_109([a, b, d, e, f, g, h])
    }
}
/// n = 6, i = 2, c < d, d < e, d < f
fn select_111([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_45([a, c, b, d, e, f])
    } else {
        select_45([b, c, a, d, e, f])
    }
}
/// n = 8, i = 3, a < h, b < d, b < g, c < e, e < f, e < g, e < h
fn select_112([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_108([f, a, b, d, g, h])
    } else {
        select_111([a, d, c, e, f, g])
    }
}
/// n = 9, i = 3, b < d, b < i, c < e, e < f, e < g, e < i, g < h
fn select_113([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_110([b, e, f, d, g, a, h, i])
    } else {
        select_112([a, b, c, d, e, f, i, g])
    }
}
/// n = 13, i = 7, a < j, a < m, b < f, c < g, d < e, d < k, e < i, f < j, f < k, g < h, g < k, k < l, k < m
fn select_114([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < k {
        select_107([a, b, c, d, f, e, g, i, h, j, k, l])
    } else {
        select_113([h, a, d, j, k, l, e, i, m])
    }
}
/// n = 6, i = 3, c < e, d < e, e < f
fn select_115([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_43([a, c, d, b, e, f])
    } else {
        select_43([b, c, d, a, e, f])
    }
}
/// n = 8, i = 4, a < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_116([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_115([a, e, b, c, g, h])
    } else {
        select_27([b, a, c, d, e, f])
    }
}
/// n = 7, i = 4, a < e, b < f, c < f, d < e, d < g, f < g
fn select_117([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_6([a, d, f, g])
    } else {
        select_24([b, c, e])
    }
}
/// n = 8, i = 5, a < h, b < f, c < g, d < g, e < f, e < h, g < h
fn select_118([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_117([b, c, d, e, f, g, h])
    } else {
        select_27([a, c, d, e, f, g])
    }
}
/// n = 9, i = 5, b < g, c < h, d < g, d < h, e < f, f < h, h < i
fn select_119([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_116([b, c, d, f, a, g, h, i])
    } else {
        select_118([c, b, a, e, d, g, f, h])
    }
}
/// n = 6, i = 3, a < f, b < e, c < e, c < f, d < e, d < f
fn select_120([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_6([b, c, d, f])
    } else {
        select_6([a, c, d, e])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < e, e < g
fn select_121([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_120([a, b, c, e, f, g])
    } else {
        select_6([a, d, f, e])
    }
}
/// n = 6, i = 3, a < f, b < d, c < e, e < f
fn select_122([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_43([b, a, c, d, e, f])
    } else {
        select_4([b, e, d, a, f])
    }
}
/// n = 7, i = 4, b < g, c < f, d < e, d < g, f < g
fn select_123([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_121([b, a, d, c, f, e, g])
    } else {
        select_122([b, e, c, a, f, g])
    }
}
/// n = 10, i = 5, b < h, c < f, d < h, d < j, e < g, f < j, g < h, g < j, h < i
fn select_124([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < f {
        select_119([a, c, d, b, e, g, f, h, i])
    } else {
        select_123([a, d, e, f, b, g, j])
    }
}
/// n = 7, i = 2, a < d, b < c, b < e, e < f, e < g
fn select_125([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < a {
        select_19([c, e, f, g, a])
    } else {
        select_109([a, b, d, e, f, g, c])
    }
}
/// n = 8, i = 2, b < h, c < d, c < e, c < h, e < f, e < g
fn select_126([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_125([a, c, d, b, e, f, g])
    } else {
        select_110([b, c, d, a, e, f, g, h])
    }
}
/// n = 4, i = 3
fn select_127([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_24([b, c, d])
    } else {
        select_24([a, c, d])
    }
}
/// n = 7, i = 4, a < e, b < f, c < f, d < f, f < g
fn select_128([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_3([a, f, g])
    } else {
        select_127([b, c, d, e])
    }
}
/// n = 10, i = 4, a < e, b < f, b < g, c < f, c < g, d < f, d < g, f < j, g < h, g < i
fn select_129([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_108([a, f, g, h, i, j])
    } else {
        select_128([a, b, c, d, e, f, j])
    }
}
/// n = 10, i = 4, c < f, c < g, d < f, d < g, e < f, e < g, f < j, g < h, g < i
fn select_130([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_129([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_129([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 11, i = 4, b < k, c < h, c < j, d < f, d < h, e < f, e < h, f < g, f < j, f < k, h < i
fn select_131([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < c {
        select_126([a, b, f, g, c, h, j, k])
    } else {
        select_130([a, b, c, d, e, h, f, g, j, i])
    }
}
/// n = 12, i = 5, b < f, c < j, c < l, d < h, d < j, e < g, g < h, g < j, h < i, h < l, j < k
fn select_132([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_124([a, c, b, d, e, f, g, j, k, h])
    } else {
        select_131([a, b, c, d, g, h, i, j, k, l, f])
    }
}
/// n = 15, i = 8, a < l, a < n, b < f, c < g, d < e, d < k, e < i, f < j, f < n, g < h, g < k, j < l, j < m, k < m, k < n, n < o
fn select_133([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < k {
        select_114([a, f, c, d, e, j, g, h, i, l, k, m, n])
    } else {
        select_132([h, e, a, k, b, i, f, j, m, n, o, l])
    }
}
/// n = 5, i = 3, b < e, c < d
fn select_134([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_6([a, b, d, e])
    } else {
        select_6([a, c, e, d])
    }
}
/// n = 5, i = 3, d < e
fn select_135([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_134([c, a, d, e, b])
    } else {
        select_134([c, b, d, e, a])
    }
}
/// n = 5, i = 3, b < e, c < e, d < e
fn select_136([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < a {
        select_6([a, c, d, e])
    } else {
        select_24([b, c, d])
    }
}
/// n = 5, i = 1, b < c, b < d, b < e
fn select_137([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < a {
        select_14([c, d, e])
    } else {
        select_9([a, b, d, e])
    }
}
/// n = 7, i = 3, c < e, d < e, e < f, e < g
fn select_138([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_136([b, a, c, d, e])
    } else {
        select_137([b, e, a, f, g])
    }
}
/// n = 8, i = 3, b < e, c < f, d < f, f < g, f < h
fn select_139([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_135([a, c, d, b, e])
    } else {
        select_138([a, b, c, d, f, g, h])
    }
}
/// n = 12, i = 7, a < j, b < e, c < f, d < g, e < i, e < l, f < j, f < k, g < h, g < k, k < l
fn select_140([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < k {
        select_107([a, c, b, d, f, g, e, h, i, j, k, l])
    } else {
        select_139([h, a, b, k, j, e, i, l])
    }
}
/// n = 7, i = 3, a < e, b < e, b < f, b < g, c < d, d < f, d < g
fn select_141([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < d {
        select_45([a, c, e, d, f, g])
    } else {
        select_9([a, b, f, g])
    }
}
/// n = 6, i = 4, a < f, b < f, c < f, d < e
fn select_142([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_127([a, b, c, e])
    } else {
        select_5([d, f])
    }
}
/// n = 9, i = 5, a < e, b < g, c < g, d < f, f < h, f < i, g < h, g < i
fn select_143([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_141([a, g, d, f, e, h, i])
    } else {
        select_142([b, c, e, d, f, g])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < e, c < f
fn select_144([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_19([d, b, e, f, a])
    } else {
        select_57([a, b, c, e, f, d])
    }
}
/// n = 10, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < i, f < j, g < i, g < j
fn select_145([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_143([a, b, c, d, e, f, g, i, j])
    } else {
        select_144([a, g, f, h, i, j])
    }
}
/// n = 10, i = 5, a < e, b < f, c < g, c < i, d < g, d < i, f < i, f < j, g < h, g < j
fn select_146([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_145([a, c, d, b, e, f, g, i, h, j])
    } else {
        select_122([b, a, g, e, h, f])
    }
}
/// n = 8, i = 4, a < d, b < f, c < e, c < g, e < h, f < g, f < h
fn select_147([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_46([a, c, g, d, e, h])
    } else {
        select_80([d, b, c, e, f, g])
    }
}
/// n = 8, i = 4, a < e, b < g, b < h, c < g, c < h, d < f, d < g, f < h
fn select_148([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_147([a, b, d, e, f, c, g, h])
    } else {
        select_147([a, c, d, e, f, b, g, h])
    }
}
/// n = 10, i = 5, a < e, b < h, b < j, c < h, c < j, d < f, f < g, f < j, h < i
fn select_149([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_146([a, d, b, c, e, f, h, i, j, g])
    } else {
        select_148([a, b, c, f, e, g, j, h])
    }
}
/// n = 12, i = 7, a < k, b < f, c < e, c < l, d < g, e < i, f < h, f < l, g < j, g < k, j < l
fn select_150([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < j {
        select_140([a, b, d, c, f, g, e, i, h, k, j, l])
    } else {
        select_149([a, c, j, b, k, f, h, e, i, l])
    }
}
/// n = 15, i = 8, a < m, a < n, b < f, c < e, c < l, d < g, e < i, f < h, f < l, g < j, g < n, j < k, j < m, l < n, n < o
fn select_151([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if l < k {
        select_133([a, d, b, c, e, g, f, h, i, j, l, m, k, n, o])
    } else {
        select_150([a, b, c, g, e, f, j, h, i, k, m, l])
    }
}
/// n = 4, i = 1, a < d, b < c
fn select_152([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_2([b, d])
    } else {
        select_2([a, c])
    }
}
/// n = 7, i = 3, a < e, a < g, b < e, b < f, c < d, d < f, d < g
fn select_153([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_4([b, c, e, d, g])
    } else {
        select_4([a, c, e, d, f])
    }
}
/// n = 7, i = 3, a < f, b < f, b < g, c < d, d < e, d < g
fn select_154([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_152([b, e, a, g])
    } else {
        select_153([a, b, c, d, f, g, e])
    }
}
/// n = 10, i = 4, a < e, a < g, b < e, b < h, c < d, c < j, d < g, d < h, e < f, e < j, h < i
fn select_155([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_9([f, c, d, j])
    } else {
        select_50([a, b, g, h, i, e])
    }
}
/// n = 10, i = 4, a < e, a < j, b < d, b < f, c < f, c < h, e < h, f < g, f < j, h < i
fn select_156([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_154([d, a, c, f, g, e, j])
    } else {
        select_155([b, c, a, e, f, g, d, h, i, j])
    }
}
/// n = 8, i = 4, a < d, a < h, b < f, c < e, c < h, e < g, f < g, f < h
fn select_157([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_153([a, e, b, f, d, g, h])
    } else {
        select_30([d, c, b, f, e, h])
    }
}
/// n = 8, i = 4, a < e, b < f, c < d, c < g, f < g, g < h
fn select_158([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_46([a, c, d, e, g, h])
    } else {
        select_80([e, b, c, d, f, g])
    }
}
/// n = 9, i = 4, a < i, b < g, c < e, c < i, d < f, f < g, f < i, g < h
fn select_159([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_157([c, d, a, e, b, f, g, i])
    } else {
        select_158([c, d, b, a, e, f, g, h])
    }
}
/// n = 11, i = 5, a < e, a < h, b < d, b < f, c < g, f < j, g < h, g < j, h < i, j < k
fn select_160([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < d {
        select_156([b, a, g, e, f, h, i, j, k, d])
    } else {
        select_159([d, f, a, c, e, g, j, k, h])
    }
}
/// n = 8, i = 2, a < c, a < g, a < h, b < d, b < g, b < h, d < e, d < f
fn select_161([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_9([c, b, g, h])
    } else {
        select_37([a, d, e, f, g, h])
    }
}
/// n = 8, i = 3, a < d, a < h, b < e, b < h, c < e, c < h, e < f, e < g
fn select_162([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_6([d, b, c, h])
    } else {
        select_19([a, e, f, g, h])
    }
}
/// n = 11, i = 4, a < f, a < h, b < f, b < h, c < d, c < h, c < k, d < e, f < g, f < k, h < i, h < j
fn select_163([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_161([f, c, g, h, i, j, d, k])
    } else {
        select_162([d, a, b, e, h, i, j, f])
    }
}
/// n = 11, i = 4, a < g, a < k, b < f, b < g, c < d, c < f, c < j, d < e, f < h, f < k, g < i, g < j
fn select_164([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_125([d, f, h, e, a, g, k])
    } else {
        select_163([a, b, c, d, e, g, i, f, h, k, j])
    }
}
/// n = 7, i = 2, a < f, a < g, b < d, b < e, b < g, c < d, c < e, c < f
fn select_165([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_152([b, c, f, g])
    } else {
        select_19([a, c, d, e, g])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, b < g, c < d, c < e, c < f
fn select_166([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_19([d, b, e, f, a])
    } else {
        select_165([a, b, c, e, f, d, g])
    }
}
/// n = 11, i = 5, a < e, a < j, a < k, b < f, c < d, c < h, d < k, e < g, f < h, f < k, h < i, h < j
fn select_167([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_166([d, a, h, i, e, j, k])
    } else {
        select_157([e, b, c, g, d, f, k, h])
    }
}
/// n = 12, i = 5, a < d, a < i, b < e, b < h, b < l, c < f, e < g, f < h, f < i, h < j, i < k, i < l
fn select_168([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < d {
        select_164([a, f, b, e, g, h, i, j, k, l, d])
    } else {
        select_167([b, c, a, d, e, f, g, i, k, l, h])
    }
}
/// n = 12, i = 5, a < f, b < h, b < l, c < e, c < i, d < g, g < h, g < i, h < k, i < j, i < l
fn select_169([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_160([c, a, d, f, e, b, g, i, j, h, k])
    } else {
        select_168([c, b, d, e, a, g, f, h, i, k, j, l])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < e, c < f
fn select_170([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_53([a, c, d, b, f])
    } else {
        select_53([b, c, d, a, e])
    }
}
/// n = 9, i = 4, a < h, a < i, b < e, c < f, d < f, d < i, e < i, f < g, f < h
fn select_171([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_120([a, c, d, e, f, i])
    } else {
        select_170([a, b, f, g, e, h])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, c < f, d < g, d < h, e < f, e < i, f < h, g < i
fn select_172([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_120([a, c, e, g, f, i])
    } else {
        select_120([a, b, d, f, g, h])
    }
}
/// n = 10, i = 5, a < g, b < f, b < j, c < h, c < j, d < f, d < h, e < g, e < h, g < j, h < i
fn select_173([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_171([b, a, d, c, g, h, i, f, j])
    } else {
        select_172([c, a, d, e, b, f, g, h, j])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < g, c < e, c < f, d < e, d < f, d < g
fn select_174([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_18([b, c, d, f, g])
    } else {
        select_18([a, c, d, e, g])
    }
}
/// n = 8, i = 4, a < g, a < h, b < f, b < h, c < f, c < g, d < e, e < g, e < h
fn select_175([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_174([a, b, c, e, f, g, h])
    } else {
        select_6([a, d, f, e])
    }
}
/// n = 10, i = 5, a < g, a < j, b < f, c < h, c < j, d < h, d < j, e < g, e < h, f < j, h < i
fn select_176([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < f {
        select_173([b, a, d, e, c, g, f, h, i, j])
    } else {
        select_175([d, a, e, f, c, g, h, j])
    }
}
/// n = 9, i = 3, a < d, a < i, b < c, b < i, c < e, c < f, d < g, d < h
fn select_177([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_51([a, c, e, f, i, d])
    } else {
        select_51([b, d, g, h, i, c])
    }
}
/// n = 5, i = 1, b < e, c < d
fn select_178([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < c {
        select_9([a, b, c, e])
    } else {
        select_9([a, c, b, d])
    }
}
/// n = 6, i = 1, c < f, d < e
fn select_179([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < c {
        select_178([b, a, d, e, c])
    } else {
        select_108([b, d, c, a, f, e])
    }
}
/// n = 8, i = 2, a < c, b < d, c < h, d < e, d < f, d < g
fn select_180([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_15([a, e, f, g])
    } else {
        select_152([b, c, h, d])
    }
}
/// n = 8, i = 4, a < f, b < e, c < f, c < h, d < f, d < h, e < h, f < g
fn select_181([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_43([b, c, d, e, f, g])
    } else {
        select_6([a, c, d, h])
    }
}
/// n = 10, i = 4, a < f, b < e, b < g, c < f, c < g, d < f, d < g, f < j, g < h, g < i
fn select_182([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_180([a, b, f, g, e, h, i, j])
    } else {
        select_181([a, b, c, d, e, f, j, g])
    }
}
/// n = 12, i = 5, a < j, a < k, b < f, b < j, c < f, c < g, d < g, d < j, e < g, e < j, g < h, g < i, g < k, j < l
fn select_183([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_179([h, i, a, b, f, k])
    } else {
        select_182([a, c, d, e, f, j, g, h, i, l])
    }
}
/// n = 12, i = 5, a < l, b < f, b < g, c < f, c < h, d < g, d < h, e < g, e < h, g < k, h < i, h < j, h < l
fn select_184([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_177([b, c, h, g, i, j, a, k, f])
    } else {
        select_183([a, b, c, d, e, f, h, i, j, g, l, k])
    }
}
/// n = 12, i = 5, a < f, b < g, b < h, c < g, c < i, d < h, d < i, e < h, e < i, h < l, i < j, i < k
fn select_185([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_176([c, a, d, e, b, f, g, h, l, i])
    } else {
        select_184([a, b, c, d, e, g, h, i, j, k, l, f])
    }
}
/// n = 14, i = 7, a < f, b < k, b < l, c < i, c < l, d < i, d < l, e < g, g < h, g < l, h < k, h < m, i < j, i < m, l < n
fn select_186([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < h {
        select_169([a, b, i, e, j, f, g, l, h, m, n, k])
    } else {
        select_185([a, b, h, c, d, f, k, l, i, j, m, n])
    }
}
/// n = 5, i = 2, a < e, b < e, c < d
fn select_187([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_4([c, a, d, b, e])
    } else {
        select_4([c, b, d, a, e])
    }
}
/// n = 6, i = 3, a < f, b < e, c < d, c < f
fn select_188([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < d {
        select_30([a, c, b, e, f, d])
    } else {
        select_187([b, d, a, f, e])
    }
}
/// n = 6, i = 3, c < f, d < e, d < f
fn select_189([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_188([c, a, d, e, b, f])
    } else {
        select_188([c, b, d, e, a, f])
    }
}
/// n = 6, i = 3, a < f, b < f, c < e, d < e, d < f
fn select_190([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_30([c, d, a, b, e, f])
    } else {
        select_30([c, d, b, a, e, f])
    }
}
/// n = 5, i = 2, b < e, c < d, c < e
fn select_191([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_99([a, c, d, b])
    } else {
        select_53([b, c, d, a, e])
    }
}
/// n = 8, i = 4, a < h, b < f, c < f, d < e, e < h, f < g
fn select_192([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_190([b, c, a, e, h, f])
    } else {
        select_191([a, d, f, g, e])
    }
}
/// n = 9, i = 4, a < i, b < g, c < g, d < e, e < f, e < i, g < h
fn select_193([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_189([b, c, a, e, f, i])
    } else {
        select_192([a, b, c, d, e, g, h, i])
    }
}
/// n = 5, i = 2, b < d, b < e, c < d, c < e
fn select_194([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_5([b, c])
    } else {
        select_9([a, c, d, e])
    }
}
/// n = 8, i = 4, a < h, b < f, c < f, c < h, d < e, d < g, e < h, f < g
fn select_195([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_120([a, b, c, e, f, h])
    } else {
        select_194([a, d, f, e, g])
    }
}
/// n = 9, i = 4, a < i, b < g, c < g, d < f, e < f, e < g, f < i, g < h
fn select_196([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_116([d, c, e, b, a, f, g, h])
    } else {
        select_195([c, d, e, a, b, f, i, g])
    }
}
/// n = 10, i = 5, a < h, b < h, c < f, d < g, d < h, e < g, e < h, f < g, g < i, h < j
fn select_197([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_181([b, c, d, e, f, h, j, g])
    } else {
        select_115([a, b, d, e, g, i])
    }
}
/// n = 4, i = 1, c < d
fn select_198([a, b, c, d]: [usize; 4]) -> usize {
    if a < c {
        select_3([b, a, c])
    } else {
        select_9([b, c, a, d])
    }
}
/// n = 8, i = 3, a < c, b < d, c < g, d < e, d < f, g < h
fn select_199([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_152([b, g, h, d])
    } else {
        select_198([e, f, a, c])
    }
}
/// n = 10, i = 5, b < g, c < f, d < g, d < i, e < g, e < i, f < i, g < h, i < j
fn select_200([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_197([a, b, c, d, e, f, i, g, j, h])
    } else {
        select_199([c, b, f, g, a, h, i, j])
    }
}
/// n = 12, i = 5, a < l, b < h, b < j, c < h, c < j, d < g, d < h, e < f, f < g, f < j, g < l, h < i, j < k
fn select_201([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_196([a, b, c, d, f, g, j, k, l])
    } else {
        select_200([a, d, e, b, c, f, h, i, j, k])
    }
}
/// n = 12, i = 5, a < j, b < h, b < k, c < h, c < k, d < g, d < h, e < f, e < g, f < k, g < j, h < i, k < l
fn select_202([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_193([a, b, c, e, g, f, h, i, j])
    } else {
        select_201([a, b, c, d, e, f, g, h, i, k, l, j])
    }
}
/// n = 6, i = 2, a < d, b < c, c < e, d < f
fn select_203([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_3([a, c, e])
    } else {
        select_3([b, d, f])
    }
}
/// n = 9, i = 4, a < f, a < g, b < d, c < e, d < f, d < i, e < g, e < i, g < h
fn select_204([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_4([a, c, f, e, i])
    } else {
        select_203([a, b, d, g, i, h])
    }
}
/// n = 9, i = 4, a < g, a < h, b < d, c < e, d < h, e < f, e < g, h < i
fn select_205([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_204([a, c, b, e, d, g, h, i, f])
    } else {
        select_187([b, f, a, g, d])
    }
}
/// n = 9, i = 4, a < h, b < h, c < g, c < h, d < e, e < f, e < g, h < i
fn select_206([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_205([c, a, d, b, e, f, g, h, i])
    } else {
        select_205([c, b, d, a, e, f, g, h, i])
    }
}
/// n = 7, i = 3, a < e, a < g, b < e, b < f, b < g, c < d, c < g, d < f
fn select_207([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_3([a, b, f])
    } else {
        select_53([b, c, d, e, g])
    }
}
/// n = 7, i = 3, a < g, b < f, b < g, c < e, c < f, d < e, d < f, d < g
fn select_208([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_207([c, d, b, a, e, g, f])
    } else {
        select_30([c, d, a, b, e, g])
    }
}
/// n = 9, i = 4, a < h, a < i, b < f, c < f, c < i, d < e, e < h, e < i, f < g
fn select_209([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_208([b, c, a, e, h, i, f])
    } else {
        select_191([a, d, f, g, e])
    }
}
/// n = 10, i = 5, a < h, b < h, c < g, c < i, d < h, d < i, e < f, f < g, f < i, h < j
fn select_210([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_206([a, b, d, e, f, g, i, h, j])
    } else {
        select_209([c, b, d, e, f, h, j, g, i])
    }
}
/// n = 12, i = 5, a < h, b < g, b < l, c < g, c < j, d < g, d < j, e < f, f < h, f < j, g < i, h < l, j < k
fn select_211([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_196([b, c, d, a, f, h, j, k, l])
    } else {
        select_200([a, b, e, c, d, f, g, i, j, k])
    }
}
/// n = 12, i = 5, a < f, b < h, b < j, c < h, c < k, d < h, d < k, e < g, e < j, f < j, g < k, h < i, k < l
fn select_212([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_210([c, d, e, b, a, f, g, h, j, i])
    } else {
        select_211([a, b, c, d, e, g, h, f, i, k, l, j])
    }
}
/// n = 12, i = 5, a < f, b < h, b < i, c < i, c < k, d < i, d < k, e < g, e < h, g < k, i < j, k < l
fn select_213([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_202([a, c, d, b, e, g, h, i, j, f, k, l])
    } else {
        select_212([a, b, c, d, e, f, g, i, j, h, k, l])
    }
}
/// n = 14, i = 7, a < f, b < l, b < m, c < h, c < m, d < h, d < m, e < g, g < i, g < m, h < k, i < j, i < l, m < n
fn select_214([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_186([a, b, c, d, e, f, g, i, h, k, l, m, j, n])
    } else {
        select_213([a, b, c, d, i, f, j, l, m, n, h, k])
    }
}
/// n = 15, i = 8, a < m, a < n, b < f, b < n, c < e, c < g, d < h, e < i, f < j, g < n, h < k, h < n, k < l, k < m, n < o
fn select_215([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < g {
        select_151([a, b, c, d, e, f, h, j, i, k, l, g, m, n, o])
    } else {
        select_214([e, a, b, g, d, i, h, f, k, l, j, m, n, o])
    }
}
/// n = 16, i = 8, a < m, a < n, b < e, b < g, c < f, c < n, d < h, f < i, f < p, g < j, g < p, h < k, h < n, k < l, k < m, n < o, n < p
fn select_216([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if n < e {
        select_98([b, a, c, h, k, f, g, l, i, j, m, n, o, e, p])
    } else {
        select_215([a, c, b, d, g, f, e, h, j, i, k, l, m, n, o])
    }
}
/// n = 8, i = 2, a < c, b < d, b < e, d < h, e < f, e < g
fn select_217([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_37([d, e, f, g, a, h])
    } else {
        select_109([a, b, c, e, f, g, d])
    }
}
/// n = 8, i = 2, a < e, b < c, b < d, b < f, f < g, f < h
fn select_218([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_217([a, b, e, c, f, g, h, d])
    } else {
        select_217([a, b, e, d, f, g, h, c])
    }
}
/// n = 8, i = 2, c < d, c < e, c < f, f < g, f < h
fn select_219([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_218([a, c, d, e, b, f, g, h])
    } else {
        select_218([b, c, d, e, a, f, g, h])
    }
}
/// n = 8, i = 1, a < f, a < g, a < h, b < c, b < d, b < e
fn select_220([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_15([b, f, g, h])
    } else {
        select_15([a, c, d, e])
    }
}
/// n = 9, i = 1, a < i, b < g, b < h, c < d, c < e, c < f
fn select_221([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_56([a, c, d, e, f, b, i])
    } else {
        select_220([b, c, d, e, f, a, g, h])
    }
}
/// n = 9, i = 1, c < h, c < i, d < e, d < f, d < g
fn select_222([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_221([a, c, d, e, f, g, h, i, b])
    } else {
        select_221([b, c, d, e, f, g, h, i, a])
    }
}
/// n = 6, i = 1, c < d, c < e, c < f
fn select_223([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_62([a, c, d, e, f, b])
    } else {
        select_62([b, c, d, e, f, a])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, b < g, c < d, c < f, c < g
fn select_224([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_57([d, a, b, f, g, e])
    } else {
        select_57([e, a, c, f, g, d])
    }
}
/// n = 9, i = 3, a < i, b < e, b < f, b < i, c < d, c < f, c < i, f < g, f < h
fn select_225([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_223([d, e, f, a, g, h])
    } else {
        select_224([a, b, c, d, e, f, i])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < j, f < k, g < h, g < i
fn select_226([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_222([d, e, g, f, a, j, k, h, i])
    } else {
        select_225([a, b, c, d, e, g, h, i, f])
    }
}
/// n = 11, i = 3, b < e, b < i, c < d, c < f, c < i, f < g, f < h, i < j, i < k
fn select_227([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_219([a, d, f, g, h, b, e, i])
    } else {
        select_226([a, b, c, d, e, f, i, j, k, g, h])
    }
}
/// n = 8, i = 2, c < d, d < e, d < f, d < g, d < h
fn select_228([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_79([a, b, c, d, e, g, h])
    } else {
        select_79([a, b, c, d, f, g, h])
    }
}
/// n = 7, i = 4, a < f, a < g, b < f, b < g, c < e, c < g, d < e, d < g
fn select_229([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_6([e, a, b, g])
    } else {
        select_6([f, c, d, g])
    }
}
/// n = 8, i = 5, a < g, a < h, b < g, b < h, c < f, d < e, d < h, f < h
fn select_230([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_229([a, b, d, f, e, g, h])
    } else {
        select_25([a, b, c, e, f, g])
    }
}
/// n = 8, i = 5, a < h, b < h, c < g, c < h, d < g, d < h, e < f, e < h
fn select_231([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_230([c, d, a, e, f, b, g, h])
    } else {
        select_230([c, d, b, e, f, a, g, h])
    }
}
/// n = 11, i = 5, b < h, c < g, c < h, d < g, d < h, e < f, e < h, h < i, h < j, h < k
fn select_232([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_228([f, g, b, h, a, i, j, k])
    } else {
        select_231([a, b, c, d, e, f, g, h])
    }
}
/// n = 11, i = 5, b < f, b < k, c < h, d < g, d < h, e < g, e < h, h < i, h < j, h < k
fn select_233([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < b {
        select_219([a, g, h, i, j, b, f, k])
    } else {
        select_232([a, c, d, e, b, f, g, h, i, j, k])
    }
}
/// n = 13, i = 5, b < g, b < j, c < f, c < k, d < h, e < g, e < h, h < i, h < j, h < k, k < l, k < m
fn select_234([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_227([a, c, h, i, f, b, g, j, k, l, m])
    } else {
        select_233([a, c, d, b, e, f, g, h, i, j, k])
    }
}
/// n = 9, i = 4, a < e, b < h, b < i, c < f, d < f, f < g, f < h, f < i
fn select_235([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_108([g, a, b, h, i, e])
    } else {
        select_115([b, e, c, d, f, g])
    }
}
/// n = 9, i = 4, a < e, b < f, c < f, d < f, f < g, f < h, f < i
fn select_236([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_15([a, g, h, i])
    } else {
        select_136([e, b, c, d, f])
    }
}
/// n = 11, i = 4, a < e, b < h, b < k, c < f, c < h, d < f, d < h, f < g, f < k, h < i, h < j
fn select_237([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_235([a, b, c, d, e, f, g, h, k])
    } else {
        select_236([a, b, c, d, e, h, f, i, j])
    }
}
/// n = 11, i = 4, c < h, c < k, d < f, d < h, e < f, e < h, f < g, f < k, h < i, h < j
fn select_238([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_237([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_237([b, c, d, e, a, f, g, h, i, j, k])
    }
}
/// n = 9, i = 4, c < h, c < i, d < f, e < f, f < g, f < h, f < i
fn select_239([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_235([a, c, d, e, b, f, g, h, i])
    } else {
        select_235([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 7, i = 4, a < g, b < e, c < f, d < f, f < g
fn select_240([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_117([b, c, d, a, e, f, g])
    } else {
        select_26([c, d, e, a, f])
    }
}
/// n = 8, i = 5, b < h, c < g, d < g, e < f, e < h, g < h
fn select_241([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_118([b, a, c, d, e, f, g, h])
    } else {
        select_240([b, f, c, d, a, g, h])
    }
}
/// n = 10, i = 5, b < g, b < j, c < h, d < g, d < h, e < f, e < h, h < i, h < j
fn select_242([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_239([a, f, b, c, d, h, i, g, j])
    } else {
        select_241([a, c, b, d, e, f, g, h])
    }
}
/// n = 13, i = 6, b < i, b < k, c < h, d < g, d < k, e < f, e < h, g < i, g < j, h < j, h < k, k < l, k < m
fn select_243([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_238([a, f, b, d, h, g, j, k, l, m, i])
    } else {
        select_242([a, b, c, g, e, f, i, h, j, k])
    }
}
/// n = 8, i = 5, a < e, b < g, c < g, d < f, f < h, g < h
fn select_244([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_117([a, b, c, f, e, g, h])
    } else {
        select_25([b, c, d, e, f, g])
    }
}
/// n = 7, i = 2, b < g, c < d, c < g, d < e, d < f
fn select_245([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_45([a, c, b, d, e, f])
    } else {
        select_109([b, c, a, d, e, f, g])
    }
}
/// n = 11, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < k, g < k, h < i, h < j
fn select_246([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_244([a, b, c, d, e, f, g, k])
    } else {
        select_245([a, g, f, h, i, j, k])
    }
}
/// n = 11, i = 5, c < g, c < h, d < g, d < h, e < f, f < h, f < k, g < k, h < i, h < j
fn select_247([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_246([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_246([b, c, d, e, a, f, g, h, i, j, k])
    }
}
/// n = 7, i = 3, b < d, b < g, c < e, e < f, e < g
fn select_248([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_191([a, b, e, f, g])
    } else {
        select_80([a, c, b, d, e, g])
    }
}
/// n = 7, i = 4, b < f, c < f, c < g, d < e, e < g
fn select_249([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < e {
        select_80([a, d, c, f, e, g])
    } else {
        select_191([a, b, c, g, f])
    }
}
/// n = 8, i = 4, c < f, d < e, d < g, e < h, f < g, f < h
fn select_250([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_248([b, e, c, a, f, g, h])
    } else {
        select_249([b, a, d, c, f, e, g])
    }
}
/// n = 11, i = 5, c < i, d < g, e < f, e < i, f < h, g < h, g < i, i < j, i < k
fn select_251([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < f {
        select_247([a, b, c, e, d, g, f, i, j, k, h])
    } else {
        select_250([a, b, d, f, c, g, h, i])
    }
}
/// n = 14, i = 7, b < k, b < l, c < i, d < g, e < f, e < i, g < h, g < l, h < k, i < j, i < l, l < m, l < n
fn select_252([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_243([a, b, c, g, e, f, h, i, k, j, l, m, n])
    } else {
        select_251([a, f, b, d, i, j, g, h, l, m, n])
    }
}
/// n = 15, i = 7, b < f, b < i, c < k, c < l, d < i, d < l, e < g, g < h, g < l, h < k, h < n, i < j, i < o, l < m, l < n, l < o
fn select_253([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if l < i {
        select_234([a, h, b, d, c, f, k, l, m, n, i, j, o])
    } else {
        select_252([a, c, d, e, b, f, g, h, i, j, k, l, m, o])
    }
}
/// n = 7, i = 2, a < g, b < f, b < g, c < d, c < f, d < e
fn select_254([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < b {
        select_9([a, d, b, e])
    } else {
        select_57([a, c, b, d, f, g])
    }
}
/// n = 6, i = 2, b < f, c < d, c < f, d < e
fn select_255([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_19([b, d, a, e, f])
    } else {
        select_18([a, b, c, f, d])
    }
}
/// n = 8, i = 3, a < d, b < f, b < h, c < e, c < f, e < h, f < g
fn select_256([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_255([a, e, b, f, g, h])
    } else {
        select_122([b, a, c, d, e, h])
    }
}
/// n = 10, i = 4, a < f, b < f, b < h, c < g, c < h, d < e, d < h, e < g, g < i, h < j
fn select_257([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_254([a, b, c, g, i, h, f])
    } else {
        select_256([a, c, d, f, e, h, j, g])
    }
}
/// n = 9, i = 4, a < e, b < f, b < g, b < i, c < d, c < i, d < g, e < f, e < i, g < h
fn select_258([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_4([a, b, e, g, h])
    } else {
        select_53([b, c, d, f, i])
    }
}
/// n = 10, i = 5, a < e, b < f, c < d, c < g, d < i, e < g, e < h, f < h, f < i, i < j
fn select_259([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_258([a, f, c, d, e, h, i, j, g])
    } else {
        select_6([d, b, g, f])
    }
}
/// n = 7, i = 2, a < c, b < d, c < g, d < e, d < f
fn select_260([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_14([a, e, f])
    } else {
        select_152([b, c, g, d])
    }
}
/// n = 10, i = 4, a < f, a < h, b < d, b < g, c < e, d < f, e < g, e < h, g < i, h < j
fn select_261([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_260([a, b, h, g, d, i, j])
    } else {
        select_204([a, b, c, d, e, f, h, j, g])
    }
}
/// n = 12, i = 5, a < d, a < j, b < f, b < g, c < e, c < j, d < g, d < h, e < h, e < i, f < i, f < j, i < k, j < l
fn select_262([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < j {
        select_259([a, c, b, f, d, e, g, h, i, k])
    } else {
        select_261([e, a, b, d, f, h, j, i, l, k])
    }
}
/// n = 12, i = 5, a < e, a < j, b < f, b < j, c < d, c < g, e < h, f < h, f < i, g < i, g < j, i < k, j < l
fn select_263([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_257([d, a, g, b, f, e, i, j, k, l])
    } else {
        select_262([a, c, b, e, f, g, d, h, i, j, k, l])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < f, c < g, d < e
fn select_264([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_37([b, d, a, e, f, g])
    } else {
        select_57([a, b, c, f, g, d])
    }
}
/// n = 8, i = 3, a < d, a < g, a < h, b < c, b < e, d < f, e < g, e < h
fn select_265([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_57([c, e, a, g, h, d])
    } else {
        select_109([d, b, f, e, g, h, c])
    }
}
/// n = 10, i = 4, a < f, a < j, b < f, b < h, b < j, c < d, c < e, e < h, e < j, f < g, h < i
fn select_266([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_264([d, a, b, h, i, f, j])
    } else {
        select_265([b, c, d, f, e, g, h, j])
    }
}
/// n = 8, i = 5, a < g, b < g, c < g, d < e, e < f, g < h
fn select_267([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_99([d, g, h, e])
    } else {
        select_127([a, b, c, f])
    }
}
/// n = 8, i = 5, a < g, b < g, c < g, d < e, d < f, g < h
fn select_268([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_267([a, b, c, d, e, f, g, h])
    } else {
        select_267([a, b, c, d, f, e, g, h])
    }
}
/// n = 8, i = 5, a < g, a < h, b < g, b < h, c < g, c < h, d < e, d < f
fn select_269([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < h {
        select_268([a, b, c, d, e, f, g, h])
    } else {
        select_268([a, b, c, d, e, f, h, g])
    }
}
/// n = 12, i = 6, a < f, a < l, b < h, b < i, c < h, c < i, d < e, d < g, f < i, g < j, g < l, h < j, h < l, j < k
fn select_270([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_266([a, h, d, e, g, f, i, j, k, l])
    } else {
        select_269([b, c, f, d, e, g, h, i])
    }
}
/// n = 13, i = 6, a < i, a < k, b < f, b < j, c < h, c < i, d < e, d < g, f < i, g < j, g < k, h < j, h < k, j < l, k < m
fn select_271([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_263([b, h, d, e, f, a, g, i, k, j, m, l])
    } else {
        select_270([b, a, c, d, e, f, g, h, i, k, m, j])
    }
}
/// n = 10, i = 5, a < i, a < j, b < h, b < j, c < e, c < i, d < f, e < h, f < g, g < i, g < j
fn select_272([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_208([b, a, c, g, e, i, j])
    } else {
        select_249([a, b, e, d, f, h, g])
    }
}
/// n = 8, i = 3, a < d, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_273([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_4([a, c, d, e, g])
    } else {
        select_4([b, c, d, f, h])
    }
}
/// n = 8, i = 3, a < f, a < h, b < e, b < f, c < d, c < e, d < h, f < g
fn select_274([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_4([b, d, e, a, h])
    } else {
        select_273([b, c, a, e, d, f, h, g])
    }
}
/// n = 8, i = 3, a < e, a < h, b < d, b < h, c < e, c < g, c < h, d < g, e < f
fn select_275([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_152([b, e, f, h])
    } else {
        select_18([a, d, c, g, h])
    }
}
/// n = 9, i = 3, a < f, a < i, b < e, b < f, c < d, c < e, d < i, e < g, f < h
fn select_276([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_9([d, e, a, g])
    } else {
        select_275([b, c, a, d, f, h, i, e])
    }
}
/// n = 9, i = 3, a < e, b < g, c < e, c < f, d < f, d < g, f < h, g < i
fn select_277([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_274([d, c, a, b, e, f, h, g])
    } else {
        select_276([c, d, b, a, g, f, i, h, e])
    }
}
/// n = 5, i = 2, c < d, d < e
fn select_278([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < d {
        select_6([b, a, c, d])
    } else {
        select_9([b, d, a, e])
    }
}
/// n = 7, i = 3, a < g, b < d, c < e, e < f, e < g
fn select_279([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_178([f, a, b, d, g])
    } else {
        select_278([a, d, c, e, f])
    }
}
/// n = 10, i = 4, a < f, a < g, b < d, c < e, d < f, d < h, d < i, e < g, e < h, e < i, g < j
fn select_280([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_45([a, c, f, e, h, i])
    } else {
        select_260([a, b, g, d, h, i, j])
    }
}
/// n = 11, i = 4, a < f, a < h, b < d, b < g, c < e, d < f, d < j, e < g, e < h, e < j, g < i, h < k
fn select_281([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_260([a, b, h, g, d, i, k])
    } else {
        select_280([a, b, c, d, e, f, h, g, j, k])
    }
}
/// n = 12, i = 5, a < g, a < j, b < i, b < j, c < e, c < i, d < f, e < g, e < h, f < h, f < i, f < j, i < k, j < l
fn select_282([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < b {
        select_279([b, a, d, g, f, h, i])
    } else {
        select_281([a, c, d, e, f, g, i, j, k, h, l])
    }
}
/// n = 12, i = 5, a < h, a < j, b < i, b < j, c < e, c < i, d < f, e < h, f < g, f < i, f < j, i < k, j < l
fn select_283([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_277([g, a, c, b, e, i, j, k, l])
    } else {
        select_282([a, b, c, d, e, f, h, g, i, j, k, l])
    }
}
/// n = 12, i = 5, a < h, a < j, b < i, b < j, c < f, c < i, d < e, d < g, f < h, g < i, g < j, i < k, j < l
fn select_284([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < g {
        select_272([b, a, c, d, f, e, g, h, i, j])
    } else {
        select_283([a, b, c, d, f, g, e, h, i, j, k, l])
    }
}
/// n = 13, i = 6, a < i, a < k, b < f, b < j, c < h, c < i, d < e, d < g, g < j, g < k, h < j, h < k, j < l, k < m
fn select_285([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < i {
        select_271([a, b, c, d, e, f, g, h, i, j, k, l, m])
    } else {
        select_284([b, h, a, d, e, i, g, f, k, j, m, l])
    }
}
/// n = 10, i = 5, a < e, b < g, c < g, d < f, f < h, f < i, f < j, g < h, g < i, g < j
fn select_286([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_67([a, d, e, f, h, i, j])
    } else {
        select_27([e, b, c, d, f, g])
    }
}
/// n = 11, i = 6, a < f, b < g, c < h, d < h, e < f, e < g, g < j, g < k, h < i, h < j, h < k
fn select_287([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_286([a, c, d, b, f, g, h, i, j, k])
    } else {
        select_121([b, a, e, h, i, f, g])
    }
}
/// n = 8, i = 5, a < h, b < e, c < g, d < f, f < h, g < h
fn select_288([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_121([a, b, f, c, g, e, h])
    } else {
        select_27([a, d, e, c, g, f])
    }
}
/// n = 12, i = 7, a < g, b < j, c < h, d < g, d < h, e < f, f < j, h < i, h < l, j < k, j < l
fn select_289([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < i {
        select_287([a, c, b, f, d, g, h, j, k, i, l])
    } else {
        select_288([b, a, e, h, g, i, f, j])
    }
}
/// n = 8, i = 3, a < d, b < e, b < f, c < e, c < f, f < g, f < h
fn select_290([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_137([e, f, a, g, h])
    } else {
        select_89([b, c, a, d, e, f])
    }
}
/// n = 9, i = 4, a < e, b < f, c < d, c < g, c < h, f < g, f < h, h < i
fn select_291([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < d {
        select_290([a, c, f, e, g, h, d, i])
    } else {
        select_147([a, b, c, e, d, f, g, h])
    }
}
/// n = 9, i = 4, a < f, b < g, c < f, c < g, d < e, d < g, g < h, g < i
fn select_292([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_32([a, b, d, f, g, e, h, i])
    } else {
        select_121([b, a, c, d, e, f, g])
    }
}
/// n = 11, i = 6, a < f, b < h, c < f, c < h, d < e, e < g, e < k, g < i, h < j, h < k
fn select_293([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_291([a, d, h, j, f, e, k, g, i])
    } else {
        select_292([a, b, c, g, i, f, h, j, k])
    }
}
/// n = 12, i = 7, a < k, b < g, c < i, d < g, d < i, e < f, f < h, h < k, h < l, i < j, i < l
fn select_294([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < h {
        select_289([b, a, c, d, e, f, g, i, j, h, k, l])
    } else {
        select_293([b, c, d, f, h, g, a, i, k, j, l])
    }
}
/// n = 10, i = 5, a < e, b < f, c < g, d < g, f < i, f < j, g < h, g < i, g < j
fn select_295([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_286([a, c, d, b, e, f, g, h, i, j])
    } else {
        select_122([b, a, g, e, h, f])
    }
}
/// n = 11, i = 6, a < e, b < i, c < f, d < g, f < h, f < k, g < i, i < j, i < k
fn select_296([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_295([a, c, b, g, e, f, i, j, h, k])
    } else {
        select_288([b, a, d, f, e, h, g, i])
    }
}
/// n = 9, i = 5, a < d, b < f, c < e, e < g, e < i, f < h, f < i
fn select_297([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < h {
        select_147([a, b, e, d, g, f, i, h])
    } else {
        select_147([a, c, f, d, h, e, i, g])
    }
}
/// n = 8, i = 4, a < h, b < e, c < d, c < g, d < f, e < g, e < h
fn select_298([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_46([a, c, g, h, d, f])
    } else {
        select_278([a, f, b, e, g])
    }
}
/// n = 8, i = 4, a < d, b < f, c < e, c < h, e < g, f < h
fn select_299([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_298([a, b, c, e, f, g, h, d])
    } else {
        select_8([b, a, c, d, e, g, f])
    }
}
/// n = 10, i = 5, a < d, b < e, c < f, e < g, e < j, f < h, f < j, h < i
fn select_300([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < i {
        select_297([a, b, c, d, f, e, h, g, j])
    } else {
        select_299([a, b, f, d, h, e, i, j])
    }
}
/// n = 11, i = 6, a < j, b < e, c < f, d < g, f < h, f < k, g < i, i < j, i < k
fn select_301([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < i {
        select_296([b, a, c, d, e, f, g, h, i, j, k])
    } else {
        select_300([b, c, g, e, f, i, h, a, j, k])
    }
}
/// n = 12, i = 7, b < k, c < i, d < g, e < f, e < i, g < h, h < k, h < l, i < j, i < l
fn select_302([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < f {
        select_294([b, a, c, e, d, g, f, h, i, j, k, l])
    } else {
        select_301([b, f, c, d, a, i, g, j, h, k, l])
    }
}
/// n = 15, i = 8, a < m, a < n, b < f, b < j, c < h, d < e, d < g, g < j, g < n, h < i, i < k, i < m, j < l, k < n, n < o
fn select_303([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < j {
        select_285([a, b, i, d, e, f, g, k, m, j, n, l, o])
    } else {
        select_302([e, a, g, c, b, f, h, i, j, l, m, k])
    }
}
/// n = 16, i = 8, a < m, a < n, b < f, b < k, c < h, d < e, d < g, g < k, g < n, h < i, h < n, i < j, i < m, k < l, k < p, n < o, n < p
fn select_304([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if n < j {
        select_253([e, b, a, g, c, f, h, i, k, l, m, n, o, j, p])
    } else {
        select_303([a, b, c, d, e, f, g, h, i, k, j, l, m, n, o])
    }
}
/// n = 7, i = 2, a < c, a < f, a < g, b < d, b < f, b < g, d < e
fn select_305([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_9([c, b, f, g])
    } else {
        select_19([d, a, f, g, e])
    }
}
/// n = 9, i = 4, a < f, a < i, b < f, b < i, c < d, c < e, c < h, e < i, f < g, f < h
fn select_306([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_120([d, e, a, b, i, f])
    } else {
        select_305([f, c, g, e, i, d, h])
    }
}
/// n = 12, i = 4, a < e, a < l, b < h, b < k, c < f, c < h, d < f, d < h, f < g, f < k, f < l, h < i, h < j
fn select_307([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_306([c, d, a, e, b, f, g, l, h])
    } else {
        select_163([c, d, b, a, e, f, g, h, i, j, k])
    }
}
/// n = 12, i = 4, b < l, c < h, c < k, d < f, d < h, e < f, e < h, f < g, f < k, f < l, h < i, h < j
fn select_308([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_237([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_307([b, c, d, e, a, f, g, h, i, j, k, l])
    }
}
/// n = 8, i = 5, a < h, b < h, c < f, d < g, e < g, g < h
fn select_309([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_244([c, d, e, a, f, b, g, h])
    } else {
        select_244([c, d, e, b, f, a, g, h])
    }
}
/// n = 7, i = 3, a < g, b < f, c < f, c < g, d < e, d < f, d < g
fn select_310([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_49([c, a, d, e, b, g])
    } else {
        select_49([c, b, d, e, a, f])
    }
}
/// n = 8, i = 3, a < e, b < f, c < d, c < f, f < g, f < h
fn select_311([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_137([d, f, a, g, h])
    } else {
        select_49([b, a, c, d, e, f])
    }
}
/// n = 10, i = 4, a < f, b < g, c < g, c < j, d < e, d < g, d < j, f < j, g < h, g < i
fn select_312([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_310([b, f, c, d, e, j, g])
    } else {
        select_311([a, c, d, e, f, g, h, i])
    }
}
/// n = 11, i = 5, b < f, c < h, d < h, d < k, e < g, f < k, g < h, g < k, h < i, h < j
fn select_313([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_309([c, d, b, a, e, f, g, h])
    } else {
        select_312([b, c, d, g, a, f, h, i, j, k])
    }
}
/// n = 13, i = 5, b < f, c < j, c < m, d < h, d < j, e < g, g < h, g < j, h < i, h < m, j < k, j < l
fn select_314([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < f {
        select_308([a, b, c, d, g, h, i, j, k, l, m, f])
    } else {
        select_313([a, b, c, d, e, f, g, j, k, l, h])
    }
}
/// n = 11, i = 6, a < i, a < j, b < g, b < h, c < h, c < j, d < h, d < j, e < f, f < g, f < j, h < i, j < k
fn select_315([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_204([a, b, e, h, f, i, j, k, g])
    } else {
        select_181([a, b, c, d, g, j, k, h])
    }
}
/// n = 7, i = 4, a < g, b < e, c < d, c < g, e < f
fn select_316([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_134([a, c, e, f, d])
    } else {
        select_134([d, a, b, e, g])
    }
}
/// n = 11, i = 6, a < j, b < i, b < j, c < g, c < h, d < g, d < j, e < f, f < h, f < j, g < i, j < k
fn select_317([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_315([b, c, a, d, e, f, h, g, i, j, k])
    } else {
        select_316([b, e, g, a, f, h, i])
    }
}
/// n = 8, i = 4, a < d, b < f, b < g, c < e, d < f, e < g, g < h
fn select_318([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_4([a, b, d, g, h])
    } else {
        select_99([b, c, e, f])
    }
}
/// n = 10, i = 5, a < f, a < h, b < f, b < g, c < g, c < h, d < e, e < h, e < j, g < j, h < i
fn select_319([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_204([a, b, d, g, e, f, h, i, j])
    } else {
        select_318([b, c, d, f, e, g, h, i])
    }
}
/// n = 7, i = 4, a < e, b < f, c < d, c < g, f < g
fn select_320([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_80([e, b, c, d, f, g])
    } else {
        select_73([a, c, d, g, e])
    }
}
/// n = 10, i = 5, a < i, b < g, b < i, c < e, c < g, d < f, e < h, f < h, f < i, i < j
fn select_321([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_319([b, c, a, d, f, g, e, i, j, h])
    } else {
        select_320([b, d, e, a, g, f, h])
    }
}
/// n = 11, i = 6, a < i, b < j, c < h, c < j, d < g, e < f, e < j, f < h, g < i, g < j, j < k
fn select_322([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_317([b, c, a, e, d, g, f, i, h, j, k])
    } else {
        select_321([b, c, f, d, a, g, h, i, j, k])
    }
}
/// n = 7, i = 2, a < d, a < g, b < c, b < e, b < g, e < f
fn select_323([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_9([c, e, a, f])
    } else {
        select_54([a, b, c, d, e, g])
    }
}
/// n = 9, i = 4, a < f, a < h, b < e, b < f, c < d, c < e, c < i, d < h, f < g, f < i
fn select_324([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_89([a, d, b, e, h, f])
    } else {
        select_54([c, f, g, e, d, i])
    }
}
/// n = 10, i = 4, a < f, a < j, b < e, b < i, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_325([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_324([d, c, a, b, f, g, h, e, j])
    } else {
        select_324([c, d, b, a, e, g, h, f, i])
    }
}
/// n = 10, i = 4, a < f, b < e, b < j, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_326([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_323([b, g, i, e, h, a, j])
    } else {
        select_325([a, b, c, d, e, f, g, i, j, h])
    }
}
/// n = 12, i = 6, b < i, b < l, c < h, c < j, d < g, e < f, e < j, f < h, g < i, g < j, j < k, j < l
fn select_327([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < j {
        select_322([b, a, c, d, e, f, g, h, i, j, k])
    } else {
        select_326([f, b, c, g, i, h, j, a, k, l])
    }
}
/// n = 15, i = 7, b < k, b < m, c < i, c < j, d < g, e < f, e < i, f < j, g < h, g < m, h < k, h < l, i < l, i < m, m < n, m < o
fn select_328([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_314([a, f, b, i, d, j, g, h, l, m, n, o, k])
    } else {
        select_327([a, b, c, g, e, f, h, j, k, i, l, m])
    }
}
/// n = 15, i = 7, b < k, b < m, c < g, c < j, d < f, d < j, e < h, h < i, h < m, i < k, i < l, j < l, j < m, m < n, m < o
fn select_329([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < g {
        select_328([a, b, c, e, d, f, h, i, j, g, k, l, m, n, o])
    } else {
        select_328([a, b, d, e, c, g, h, i, j, f, k, l, m, n, o])
    }
}
/// n = 5, i = 3, c < e, d < e
fn select_330([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_26([c, d, a, b, e])
    } else {
        select_26([c, d, b, a, e])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < e, d < g
fn select_331([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_27([a, b, c, d, e, f])
    } else {
        select_330([b, e, a, c, g])
    }
}
/// n = 8, i = 5, b < h, c < g, d < g, d < h, e < f, f < h
fn select_332([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_118([b, c, a, e, d, g, f, h])
    } else {
        select_331([b, c, d, f, a, g, h])
    }
}
/// n = 10, i = 6, a < j, b < i, c < h, d < f, e < g, f < i, f < j, g < h, g < j
fn select_333([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_332([b, a, c, g, d, f, h, j])
    } else {
        select_332([c, a, b, f, e, g, i, j])
    }
}
/// n = 8, i = 4, a < d, b < f, c < e, e < g, e < h, f < g, f < h
fn select_334([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_141([a, e, b, f, d, g, h])
    } else {
        select_26([c, d, b, f, e])
    }
}
/// n = 8, i = 4, a < h, b < e, c < g, c < h, d < f, f < g, f < h
fn select_335([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_334([b, a, d, e, f, c, g, h])
    } else {
        select_147([b, d, c, e, a, f, g, h])
    }
}
/// n = 11, i = 6, a < i, b < j, c < h, c < k, d < f, d < k, e < g, f < h, g < i, g < j, j < k
fn select_336([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < j {
        select_333([b, a, c, e, d, g, f, h, i, j])
    } else {
        select_335([c, a, d, b, i, j, f, k])
    }
}
/// n = 9, i = 5, a < g, a < i, b < f, b < h, c < e, c < g, d < e, d < f, f < i, g < h
fn select_337([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_89([a, f, c, e, i, g])
    } else {
        select_89([b, g, d, e, h, f])
    }
}
/// n = 8, i = 4, a < f, a < h, b < f, b < h, c < d, d < e, d < h, f < g
fn select_338([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_4([f, c, g, d, h])
    } else {
        select_6([e, a, b, h])
    }
}
/// n = 8, i = 5, a < g, a < h, b < f, c < f, d < e, e < g, f < h
fn select_339([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_26([a, f, d, e, h])
    } else {
        select_24([b, c, g])
    }
}
/// n = 9, i = 5, a < g, a < i, b < g, b < i, c < f, c < h, d < e, d < f, f < i, g < h
fn select_340([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_338([a, b, d, f, e, g, h, i])
    } else {
        select_339([c, a, b, d, e, g, f, h])
    }
}
/// n = 11, i = 6, a < h, a < j, b < g, b < h, c < f, c < k, d < e, d < g, e < j, e < k, f < i, f < j, h < i, h < k
fn select_341([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_337([a, f, b, d, g, e, h, i, j])
    } else {
        select_340([c, e, a, b, g, h, f, j, k])
    }
}
/// n = 9, i = 4, a < f, a < h, b < f, b < h, c < d, c < h, c < i, d < e, f < g, f < i
fn select_342([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_89([a, b, d, e, h, f])
    } else {
        select_54([c, f, g, h, d, i])
    }
}
/// n = 10, i = 5, a < e, a < h, b < f, b < j, c < d, c < g, c < j, d < h, e < i, e < j, f < g, f < i
fn select_343([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_324([a, f, c, d, g, e, i, h, j])
    } else {
        select_342([b, e, c, d, h, f, i, j, g])
    }
}
/// n = 11, i = 6, a < h, b < g, b < k, c < f, c < j, d < e, d < h, e < j, e < k, f < i, f < k, g < i, g < j
fn select_344([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_341([b, a, c, d, e, f, h, g, i, k, j])
    } else {
        select_343([d, c, g, a, e, f, i, h, k, j])
    }
}
/// n = 8, i = 4, a < e, b < g, b < h, c < f, c < g, d < f, d < g, f < h
fn select_345([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_240([b, a, c, d, e, f, h])
    } else {
        select_191([a, f, b, g, h])
    }
}
/// n = 7, i = 3, a < d, b < f, b < g, c < e, c < f, e < g
fn select_346([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_191([d, b, c, e, f])
    } else {
        select_46([a, c, f, d, e, g])
    }
}
/// n = 8, i = 4, a < h, b < e, c < g, c < h, d < f, d < h, f < g
fn select_347([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_345([b, c, a, d, e, f, h, g])
    } else {
        select_346([b, c, f, e, a, g, h])
    }
}
/// n = 11, i = 6, a < h, b < e, b < k, c < g, c < i, d < f, d < h, e < j, f < i, f < k, g < j, g < k
fn select_348([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < i {
        select_344([a, b, c, d, f, g, e, h, j, i, k])
    } else {
        select_347([g, a, b, f, h, i, e, k])
    }
}
/// n = 12, i = 7, a < j, b < f, b < l, c < h, d < e, d < g, f < k, g < k, g < l, h < i, h < j, i < l
fn select_349([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_336([a, e, g, b, c, f, h, k, j, i, l])
    } else {
        select_348([a, b, d, h, f, i, g, j, e, k, l])
    }
}
/// n = 8, i = 4, a < h, b < e, b < g, c < f, d < f, f < g, f < h
fn select_350([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_57([a, b, f, e, g, h])
    } else {
        select_136([a, c, d, e, f])
    }
}
/// n = 9, i = 5, a < g, b < e, b < i, c < g, c < h, d < f, f < h, h < i
fn select_351([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_350([a, b, c, f, e, h, i, g])
    } else {
        select_244([b, a, c, d, e, f, g, h])
    }
}
/// n = 9, i = 5, b < i, c < g, d < g, d < h, e < f, f < h, h < i
fn select_352([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_351([c, b, d, e, a, f, g, h, i])
    } else {
        select_29([c, a, d, e, b, f, g, h])
    }
}
/// n = 7, i = 3, a < g, b < d, b < f, c < e, e < f, e < g
fn select_353([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_6([a, c, d, e])
    } else {
        select_57([a, b, e, d, f, g])
    }
}
/// n = 8, i = 5, a < g, b < g, b < h, c < f, d < e, e < h, f < h
fn select_354([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_117([a, c, e, b, g, f, h])
    } else {
        select_117([a, d, f, b, g, e, h])
    }
}
/// n = 9, i = 5, a < e, b < g, b < i, c < g, c < h, d < f, f < h, h < i
fn select_355([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < e {
        select_353([a, b, f, g, h, i, e])
    } else {
        select_354([b, c, a, d, f, e, g, h])
    }
}
/// n = 9, i = 5, c < g, c < i, d < g, d < h, e < f, f < h, h < i
fn select_356([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_355([a, c, d, e, b, f, g, h, i])
    } else {
        select_355([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 11, i = 6, b < i, c < h, c < k, d < f, e < g, f < i, f < j, g < h, g < j, j < k
fn select_357([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_352([a, c, b, f, e, g, i, j, k])
    } else {
        select_356([a, b, c, g, d, f, h, j, k])
    }
}
/// n = 8, i = 4, b < f, b < h, c < f, c < h, d < e, d < g, e < h, f < g
fn select_358([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_74([e, b, c, a, f, g, h])
    } else {
        select_117([a, b, c, d, e, f, g])
    }
}
/// n = 6, i = 3, b < f, c < e, c < f, d < e, d < f
fn select_359([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < a {
        select_89([c, d, b, a, e, f])
    } else {
        select_26([c, d, a, b, e])
    }
}
/// n = 8, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, d < h, f < h
fn select_360([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_359([a, e, b, c, g, f])
    } else {
        select_144([a, f, d, g, e, h])
    }
}
/// n = 9, i = 4, b < i, c < g, c < h, d < f, d < g, e < f, e < g, f < h, f < i
fn select_361([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_358([a, d, e, b, c, f, i, g])
    } else {
        select_360([a, d, e, c, b, f, g, h])
    }
}
/// n = 6, i = 4, d < f, e < f
fn select_362([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_27([c, d, e, a, b, f])
    } else {
        select_27([c, d, e, b, a, f])
    }
}
/// n = 6, i = 4, c < f, d < f, e < f
fn select_363([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_142([c, d, e, a, b, f])
    } else {
        select_142([c, d, e, b, a, f])
    }
}
/// n = 7, i = 4, b < g, c < g, d < f, e < f, e < g
fn select_364([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_362([a, b, c, d, e, f])
    } else {
        select_363([a, d, b, c, e, g])
    }
}
/// n = 6, i = 3, b < f, c < f, d < e, d < f
fn select_365([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < e {
        select_359([a, c, b, d, e, f])
    } else {
        select_7([a, c, e, b])
    }
}
/// n = 8, i = 4, b < g, c < g, c < h, d < g, d < h, e < f, f < h
fn select_366([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < f {
        select_364([a, c, d, e, b, f, g])
    } else {
        select_365([a, c, d, f, b, h])
    }
}
/// n = 9, i = 4, b < f, c < h, c < i, d < g, d < h, e < g, e < h, g < i
fn select_367([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_361([a, b, c, d, e, g, h, i, f])
    } else {
        select_366([a, c, d, e, b, f, h, g])
    }
}
/// n = 11, i = 6, b < j, c < i, c < k, d < f, d < k, e < g, f < i, g < h, g < j, h < k
fn select_368([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_357([a, b, c, e, d, g, f, i, j, h, k])
    } else {
        select_367([a, b, c, d, h, j, f, k, i])
    }
}
/// n = 12, i = 7, a < k, b < f, b < l, c < h, d < e, d < g, g < i, g < l, h < j, h < k, j < l
fn select_369([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_349([a, b, c, d, e, f, g, h, j, k, i, l])
    } else {
        select_368([e, a, b, g, c, i, h, j, f, k, l])
    }
}
/// n = 16, i = 8, a < m, a < n, b < f, b < l, c < h, d < e, d < g, g < i, g < l, h < j, h < n, j < k, j < m, l < n, n < o, n < p
fn select_370([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < k {
        select_329([e, a, b, g, c, i, f, h, j, l, m, k, n, o, p])
    } else {
        select_369([a, b, h, d, e, f, g, j, i, k, m, l])
    }
}
/// n = 16, i = 8, a < m, a < n, b < f, b < g, c < e, c < h, d < i, g < j, g < p, h < n, i < k, i < n, k < l, k < m, n < o, n < p
fn select_371([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < g {
        select_304([a, b, d, c, e, f, h, i, k, l, g, j, m, n, o, p])
    } else {
        select_370([a, c, d, b, f, e, g, i, j, k, l, h, m, n, o, p])
    }
}
/// n = 16, i = 8, a < g, a < p, b < n, c < m, c < n, d < f, d < h, e < i, h < j, h < p, i < k, i < n, k < l, k < m, n < o, n < p
fn select_372([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if b < a {
        select_216([c, d, b, e, f, a, h, i, g, j, k, l, m, n, o, p])
    } else {
        select_371([c, d, a, e, g, f, h, b, i, j, k, l, m, n, o, p])
    }
}
/// n = 7, i = 4, b < f, c < f, d < e, d < g, f < g
fn select_373([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_117([a, b, c, d, e, f, g])
    } else {
        select_43([e, b, c, a, f, g])
    }
}
/// n = 11, i = 5, a < e, b < g, c < g, d < f, d < h, f < i, f < j, f < k, g < h, g < i, g < j, g < k
fn select_374([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_64([a, d, h, e, f, i, j, k])
    } else {
        select_373([e, b, c, d, f, g, h])
    }
}
/// n = 7, i = 3, a < e, a < g, b < c, b < d, d < g, e < f
fn select_375([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_18([c, d, a, g, e])
    } else {
        select_50([e, b, f, d, g, c])
    }
}
/// n = 8, i = 4, a < g, a < h, b < d, b < g, c < e, c < h, d < h, e < f
fn select_376([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_375([c, b, g, d, e, f, h])
    } else {
        select_191([f, a, b, d, g])
    }
}
/// n = 12, i = 6, a < g, a < j, b < h, c < h, d < e, d < g, e < f, g < k, g < l, h < i, h < j, h < k, h < l
fn select_377([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_374([e, b, c, a, f, g, h, j, i, k, l])
    } else {
        select_376([a, h, d, i, e, f, j, g])
    }
}
/// n = 11, i = 5, a < e, b < f, b < i, c < g, d < g, f < j, f < k, g < h, g < i, g < j, g < k
fn select_378([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_346([a, b, g, e, h, i, f])
    } else {
        select_374([a, c, d, b, e, f, g, i, h, j, k])
    }
}
/// n = 12, i = 6, a < j, a < l, b < h, c < h, d < e, d < f, e < g, f < k, f < l, h < i, h < j, h < k, h < l
fn select_379([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < f {
        select_377([a, b, c, d, e, g, f, h, i, j, k, l])
    } else {
        select_378([e, f, b, c, g, a, h, i, k, j, l])
    }
}
/// n = 7, i = 3, a < f, a < g, b < d, b < g, c < e, c < g, e < f
fn select_380([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_99([c, d, a, e])
    } else {
        select_207([b, a, c, e, d, f, g])
    }
}
/// n = 8, i = 4, a < e, b < f, c < f, d < e, d < g, d < h, f < g, f < h
fn select_381([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_26([b, c, d, e, f])
    } else {
        select_9([a, d, g, h])
    }
}
/// n = 8, i = 4, a < f, b < g, c < e, c < g, d < e, d < f, d < h, g < h
fn select_382([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_381([a, b, c, d, f, g, e, h])
    } else {
        select_122([b, a, c, f, e, g])
    }
}
/// n = 9, i = 4, a < f, a < i, b < f, b < h, c < g, c < i, d < e, d < g, e < i, g < h
fn select_383([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_380([a, c, e, g, b, f, i])
    } else {
        select_382([a, c, d, b, e, f, g, h])
    }
}
/// n = 10, i = 4, a < f, b < f, c < d, c < h, c < i, c < j, d < e, f < g, f < h, f < i, f < j
fn select_384([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_137([g, c, h, i, j])
    } else {
        select_32([d, a, b, e, f, h, i, j])
    }
}
/// n = 8, i = 4, a < e, a < h, b < f, b < h, c < f, c < h, d < e, d < g, d < h, f < g
fn select_385([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_18([a, f, d, g, h])
    } else {
        select_6([e, b, c, h])
    }
}
/// n = 10, i = 6, a < f, a < h, b < h, b < i, c < g, d < g, e < f, e < h, e < j, g < i, i < j
fn select_386([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_385([a, b, g, e, f, i, j, h])
    } else {
        select_136([f, c, d, h, g])
    }
}
/// n = 13, i = 6, a < f, a < m, b < f, b < g, c < h, d < h, e < g, e < i, g < j, g < k, g < l, h < i, h < j, h < k, h < l, i < m
fn select_387([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_384([c, d, g, a, f, h, i, j, k, l])
    } else {
        select_386([b, e, c, d, a, f, h, g, i, m])
    }
}
/// n = 13, i = 6, a < f, a < m, b < f, b < g, c < g, c < j, d < h, e < h, g < k, g < l, h < i, h < j, h < k, h < l, j < m
fn select_388([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_383([b, a, c, h, i, f, j, m, g])
    } else {
        select_387([a, b, d, e, c, f, g, h, j, i, k, l, m])
    }
}
/// n = 13, i = 6, a < g, a < m, b < j, b < l, c < f, c < g, d < h, e < h, f < k, f < l, h < i, h < j, h < k, h < l, j < m
fn select_389([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < b {
        select_378([a, f, d, e, g, b, h, i, k, j, l])
    } else {
        select_388([a, c, b, d, e, g, f, h, i, j, k, l, m])
    }
}
/// n = 13, i = 6, a < m, b < j, b < l, c < f, c < g, d < h, e < h, g < k, g < l, h < i, h < j, h < k, h < l, j < m
fn select_390([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_379([b, d, e, c, f, g, a, h, i, j, k, l])
    } else {
        select_389([a, b, c, d, e, g, f, h, i, j, k, l, m])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < g, c < d, d < g, d < h, e < f
fn select_391([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_3([c, e, f])
    } else {
        select_18([a, b, d, g, h])
    }
}
/// n = 7, i = 3, a < e, b < d, c < e, c < g, d < g, e < f
fn select_392([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_6([a, c, d, g])
    } else {
        select_3([b, e, f])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, b < g, c < e, c < f, d < f, d < g, f < i, g < h
fn select_393([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_391([a, b, c, f, g, h, e, i])
    } else {
        select_392([a, c, d, e, g, h, f])
    }
}
/// n = 8, i = 5, a < h, b < g, c < g, c < h, d < g, d < h, e < f, f < h
fn select_394([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < b {
        select_120([a, b, c, d, g, h])
    } else {
        select_27([a, c, d, e, f, g])
    }
}
/// n = 11, i = 6, a < i, a < k, b < f, b < g, c < h, c < k, d < h, d < k, e < g, e < i, g < k, h < i, i < j
fn select_395([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_393([a, h, b, e, f, g, i, j, k])
    } else {
        select_394([a, f, c, d, e, g, h, k])
    }
}
/// n = 7, i = 4, a < g, b < f, b < g, c < f, c < g, d < e, d < g
fn select_396([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_120([a, e, b, c, f, g])
    } else {
        select_18([a, f, d, e, g])
    }
}
/// n = 8, i = 6, a < h, b < h, c < h, d < g, e < g, f < g
fn select_397([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < h {
        select_127([a, b, c, g])
    } else {
        select_127([d, e, f, h])
    }
}
/// n = 8, i = 6, b < h, c < h, d < h, e < g, f < g
fn select_398([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < g {
        select_397([b, c, d, a, e, f, g, h])
    } else {
        select_142([b, c, d, g, a, h])
    }
}
/// n = 9, i = 6, b < i, c < h, d < h, e < g, e < i, f < g, f < i, h < i
fn select_399([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < a {
        select_396([b, e, f, h, a, g, i])
    } else {
        select_398([b, a, c, d, e, f, g, h])
    }
}
/// n = 12, i = 7, a < j, a < l, b < f, b < g, c < i, c < l, d < i, d < l, e < h, g < l, h < j, h < l, i < j, j < k
fn select_400([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_395([a, b, c, d, h, f, g, i, j, k, l])
    } else {
        select_399([f, a, e, g, c, d, i, h, l])
    }
}
/// n = 9, i = 4, a < e, b < f, b < g, c < f, c < g, d < f, d < g, f < i, g < h
fn select_401([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_178([a, f, g, h, i])
    } else {
        select_128([a, b, c, d, e, g, h])
    }
}
/// n = 10, i = 4, a < h, b < e, b < i, c < f, c < h, d < f, d < h, f < g, f < i, h < j
fn select_402([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_323([b, f, g, e, a, h, i])
    } else {
        select_401([b, a, c, d, e, f, h, j, g])
    }
}
/// n = 13, i = 7, a < f, a < g, b < j, b < l, c < i, c < j, d < i, d < j, e < h, h < j, h < l, i < l, j < k, l < m
fn select_403([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < j {
        select_400([b, a, c, d, e, g, f, h, i, l, m, j])
    } else {
        select_402([i, a, b, h, g, j, k, l, f, m])
    }
}
/// n = 8, i = 4, a < g, b < e, b < f, c < f, c < g, d < e, d < g, g < h
fn select_404([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_392([a, b, c, e, g, h, f])
    } else {
        select_392([a, b, d, f, g, h, e])
    }
}
/// n = 9, i = 5, a < h, b < e, b < g, c < g, c < h, d < f, f < h, h < i
fn select_405([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_404([a, b, c, f, e, g, h, i])
    } else {
        select_27([a, d, e, c, g, f])
    }
}
/// n = 9, i = 4, a < e, b < g, b < i, c < f, c < g, d < f, d < g, f < i, g < h
fn select_406([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_255([a, f, b, g, h, i])
    } else {
        select_240([b, a, c, d, e, f, i])
    }
}
/// n = 11, i = 5, a < e, a < g, b < h, b < i, c < g, c < i, c < k, d < f, f < h, f < i, h < k, i < j
fn select_407([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_405([b, a, c, d, e, f, g, i, j])
    } else {
        select_406([a, c, b, f, e, h, i, j, k])
    }
}
/// n = 12, i = 6, a < e, a < f, b < i, b < j, c < h, d < g, d < i, g < j, g < l, h < i, h < j, i < l, j < k
fn select_408([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_407([a, b, g, c, f, h, e, i, j, k, l])
    } else {
        select_332([f, b, e, d, c, h, g, i])
    }
}
/// n = 13, i = 7, a < f, a < g, b < k, b < m, c < j, c < k, d < i, e < h, e < k, h < j, i < j, i < k, j < m, k < l
fn select_409([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < h {
        select_403([a, c, b, e, d, f, g, i, h, k, l, j, m])
    } else {
        select_408([a, c, d, h, f, g, b, i, j, k, l, m])
    }
}
/// n = 15, i = 7, a < i, a < o, b < f, b < h, c < j, c < n, d < j, d < l, e < g, e < j, g < l, h < m, h < o, i < l, j < k, j < m, j < o, l < n
fn select_410([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < i {
        select_390([g, a, b, c, d, f, h, j, k, i, m, o, l])
    } else {
        select_409([b, c, d, a, e, f, h, g, i, l, j, k, n])
    }
}
/// n = 9, i = 5, a < f, a < g, b < g, b < h, c < f, c < h, d < e, e < h, h < i
fn select_411([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_318([a, b, d, f, e, g, h, i])
    } else {
        select_318([a, c, d, g, e, f, h, i])
    }
}
/// n = 10, i = 5, a < h, b < e, b < g, c < g, c < h, c < i, d < f, e < i, f < h, f < i, h < j
fn select_412([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_411([b, a, c, d, f, g, e, h, j])
    } else {
        select_90([d, c, e, a, g, f, i])
    }
}
/// n = 7, i = 5, a < g, b < g, c < g, d < f, e < f
fn select_413([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_127([a, b, c, f])
    } else {
        select_24([d, e, g])
    }
}
/// n = 8, i = 6, a < h, b < h, c < h, d < g, e < f
fn select_414([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_413([a, b, c, d, f, g, h])
    } else {
        select_413([a, b, c, e, g, f, h])
    }
}
/// n = 8, i = 6, c < h, d < h, e < h, f < g
fn select_415([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_414([c, d, e, a, f, g, b, h])
    } else {
        select_414([c, d, e, b, f, g, a, h])
    }
}
/// n = 12, i = 7, a < j, b < f, b < g, c < i, d < i, e < h, g < k, h < j, h < k, i < j, i < k, j < l
fn select_416([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_412([a, b, i, e, g, h, f, j, k, l])
    } else {
        select_415([a, g, c, d, f, e, h, i])
    }
}
/// n = 7, i = 4, b < g, c < g, d < f, d < g, e < f, e < g
fn select_417([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_396([c, d, e, b, a, f, g])
    } else {
        select_27([c, d, e, a, b, f])
    }
}
/// n = 7, i = 5, b < g, c < g, d < g, e < f
fn select_418([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_413([b, c, d, a, e, f, g])
    } else {
        select_142([b, c, d, f, a, g])
    }
}
/// n = 7, i = 5, d < g, e < g, f < g
fn select_419([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_418([c, d, e, f, a, b, g])
    } else {
        select_418([c, d, e, f, b, a, g])
    }
}
/// n = 9, i = 6, b < i, c < i, d < h, e < h, f < g, f < i, h < i
fn select_420([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_417([a, b, c, f, h, g, i])
    } else {
        select_419([a, b, c, d, e, g, h])
    }
}
/// n = 13, i = 8, a < j, b < j, c < f, c < g, d < i, e < h, e < j, g < k, h < l, i < k, i < l, j < l, l < m
fn select_421([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < k {
        select_416([h, c, a, b, d, f, g, i, j, l, k, m])
    } else {
        select_420([f, a, b, g, i, e, h, k, j])
    }
}
/// n = 8, i = 4, a < e, b < f, b < g, c < d, c < f, d < h, e < g, f < h
fn select_422([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_89([b, e, c, d, g, f])
    } else {
        select_18([a, d, f, h, e])
    }
}
/// n = 7, i = 4, a < f, a < g, b < e, c < d, d < f, e < g
fn select_423([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_99([a, c, d, g])
    } else {
        select_99([a, b, e, f])
    }
}
/// n = 8, i = 5, a < e, b < d, c < f, d < h, e < h, f < g
fn select_424([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_423([d, a, c, f, e, g, h])
    } else {
        select_26([b, g, a, e, d])
    }
}
/// n = 11, i = 6, a < g, a < h, b < g, b < i, c < f, d < e, d < i, e < h, e < k, f < i, f < j, g < j, i < k
fn select_425([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_422([a, f, d, e, g, i, j, k])
    } else {
        select_424([b, c, d, f, g, e, h, j])
    }
}
/// n = 7, i = 3, a < d, b < c, b < g, c < f, d < e, d < g
fn select_426([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_3([f, a, d])
    } else {
        select_9([e, b, c, g])
    }
}
/// n = 8, i = 4, a < g, b < d, c < e, d < g, d < h, e < f, e < h
fn select_427([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_426([c, d, a, e, f, g, h])
    } else {
        select_100([a, b, c, e, f, d, g])
    }
}
/// n = 10, i = 6, a < h, b < f, c < g, d < e, d < j, e < i, f < i, f < j, g < h, g < j
fn select_428([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_427([a, c, b, g, f, i, h, j])
    } else {
        select_147([a, b, d, h, e, f, j, i])
    }
}
/// n = 12, i = 7, a < e, a < i, b < i, b < k, c < f, d < g, f < j, f < k, g < h, g < k, h < l, i < j, k < l
fn select_429([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_425([a, b, c, g, h, f, i, e, k, j, l])
    } else {
        select_428([e, c, d, b, i, f, g, h, j, k])
    }
}
/// n = 9, i = 5, b < g, c < f, c < i, d < e, d < h, e < i, f < h, g < h, g < i
fn select_430([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_358([a, d, g, c, f, e, i, h])
    } else {
        select_373([a, b, e, c, f, g, i])
    }
}
/// n = 8, i = 4, b < h, c < f, d < e, d < h, e < g, f < g, f < h
fn select_431([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_353([b, e, c, a, f, g, h])
    } else {
        select_121([b, a, d, c, f, e, h])
    }
}
/// n = 10, i = 6, b < f, c < g, d < e, d < i, e < j, f < i, f < j, g < h, g < j
fn select_432([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_430([a, b, d, g, h, e, f, j, i])
    } else {
        select_431([a, e, c, f, i, g, h, j])
    }
}
/// n = 12, i = 7, a < k, b < e, b < f, c < g, d < h, f < j, g < j, g < k, h < i, h < k, i < l, k < l
fn select_433([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < f {
        select_429([b, a, c, d, e, g, h, i, f, j, k, l])
    } else {
        select_432([e, c, d, f, a, g, h, i, j, k])
    }
}
/// n = 13, i = 8, a < m, b < j, c < f, c < g, d < i, e < h, e < j, g < k, h < l, i < k, i < l, j < l, l < m
fn select_434([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < j {
        select_421([a, b, c, d, e, f, g, h, i, j, k, l, m])
    } else {
        select_433([h, c, d, b, f, g, i, j, a, k, l, m])
    }
}
/// n = 10, i = 5, a < h, a < i, b < f, b < g, c < f, c < i, d < e, e < g, e < i, f < h, i < j
fn select_435([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_204([a, b, d, f, e, h, i, j, g])
    } else {
        select_392([a, b, c, g, i, j, f])
    }
}
/// n = 9, i = 4, a < f, a < g, b < d, b < f, c < e, d < i, e < g, e < i, g < h
fn select_436([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_43([a, c, d, f, e, i])
    } else {
        select_273([a, b, e, f, d, g, i, h])
    }
}
/// n = 10, i = 5, a < h, b < g, b < i, c < f, d < e, d < i, e < g, f < h, f < i, i < j
fn select_437([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_435([b, a, d, c, f, e, h, g, i, j])
    } else {
        select_436([b, e, c, a, f, g, i, j, h])
    }
}
/// n = 11, i = 6, a < i, b < e, b < j, c < f, d < g, f < i, f < j, g < h, g < j, j < k
fn select_438([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_106([a, d, c, b, e, g, f, i, h, j, k])
    } else {
        select_437([a, b, c, g, h, f, e, i, j, k])
    }
}
/// n = 8, i = 4, a < h, b < f, b < g, c < f, c < g, c < h, d < e, e < g, e < h
fn select_439([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_80([a, d, c, f, e, g])
    } else {
        select_30([b, c, d, e, f, h])
    }
}
/// n = 10, i = 6, a < j, b < f, b < i, c < h, d < h, e < g, g < i, g < j, h < i, h < j
fn select_440([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_439([a, b, h, e, g, f, i, j])
    } else {
        select_418([a, c, d, f, e, g, h])
    }
}
/// n = 7, i = 4, a < f, b < f, b < g, c < f, c < g, d < e, e < g
fn select_441([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_26([b, c, d, e, f])
    } else {
        select_6([a, b, c, g])
    }
}
/// n = 7, i = 4, a < g, b < g, c < f, d < f, d < g, e < f, e < g
fn select_442([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_441([c, d, e, a, b, f, g])
    } else {
        select_441([c, d, e, b, a, f, g])
    }
}
/// n = 9, i = 6, a < i, b < i, c < g, d < h, e < h, f < g, f < i, h < i
fn select_443([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_442([a, b, c, f, h, g, i])
    } else {
        select_363([a, b, d, e, g, h])
    }
}
/// n = 11, i = 7, a < i, b < j, c < j, d < h, d < k, e < h, e < j, f < g, g < i, g < k, j < k
fn select_444([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if j < i {
        select_440([a, d, b, c, f, h, g, j, k, i])
    } else {
        select_443([b, c, d, a, g, e, h, i, j])
    }
}
/// n = 12, i = 7, b < j, c < i, d < h, d < k, e < h, e < i, f < g, g < j, g < k, i < k, k < l
fn select_445([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < a {
        select_438([b, d, f, c, h, g, i, a, j, k, l])
    } else {
        select_444([b, a, c, d, e, f, g, h, j, i, k])
    }
}
/// n = 10, i = 6, a < i, b < h, c < h, d < f, d < j, e < g, g < i, g < j, h < j
fn select_446([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_440([a, d, b, c, e, f, g, h, j, i])
    } else {
        select_309([b, c, d, a, g, f, i, h])
    }
}
/// n = 11, i = 6, b < i, c < g, d < f, d < j, e < h, g < j, h < i, h < j, j < k
fn select_447([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_438([b, d, e, c, f, h, g, a, i, j, k])
    } else {
        select_446([b, a, c, d, e, f, h, g, i, j])
    }
}
/// n = 12, i = 7, b < j, c < k, d < i, d < k, e < h, f < g, f < i, g < k, h < j, h < k, k < l
fn select_448([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < g {
        select_445([a, b, c, d, f, e, h, i, g, j, k, l])
    } else {
        select_447([a, b, g, d, e, i, c, h, j, k, l])
    }
}
/// n = 15, i = 8, a < m, a < o, b < g, c < k, c < m, d < f, d < h, e < i, e < k, g < l, h < j, i < l, i < m, j < m, k < l, l < o, m < n
fn select_449([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < j {
        select_434([a, c, d, b, e, f, h, i, g, k, j, l, o])
    } else {
        select_448([f, b, a, c, h, e, i, j, k, g, m, n])
    }
}
/// n = 16, i = 8, a < l, a < o, b < h, b < p, c < f, c < g, d < k, d < l, e < i, e < k, g < j, g < p, h < n, i < l, i < n, k < n, l < m, l < p, n < o
fn select_450([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < j {
        select_410([b, c, a, i, d, f, k, g, h, l, m, n, j, o, p])
    } else {
        select_449([a, b, d, c, e, f, h, g, i, j, k, n, l, m, o])
    }
}
/// n = 8, i = 3, a < d, a < h, b < f, b < g, b < h, c < e, c < f, c < h, e < g
fn select_451([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_144([d, b, c, e, f, h])
    } else {
        select_323([a, c, f, d, e, g, h])
    }
}
/// n = 10, i = 4, a < h, b < e, b < i, c < g, c < h, c < i, d < f, d < h, f < g, f < i, h < j
fn select_452([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_451([b, c, f, e, a, g, h, i])
    } else {
        select_406([b, c, a, d, e, f, h, j, g])
    }
}
/// n = 9, i = 3, a < e, a < i, b < e, b < f, b < g, c < d, d < f, d < g, d < i, e < h
fn select_453([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_19([a, b, f, g, i])
    } else {
        select_203([b, c, d, e, i, h])
    }
}
/// n = 10, i = 4, a < g, a < j, b < e, b < g, b < h, c < e, c < f, d < f, d < g, f < h, f < j, g < i
fn select_454([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_392([a, c, d, e, g, i, f])
    } else {
        select_453([a, b, c, f, g, e, h, i, j])
    }
}
/// n = 12, i = 6, a < i, a < l, b < f, b < g, c < h, c < l, d < h, d < l, e < g, e < i, g < j, g < l, h < i, h < j, i < k
fn select_455([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_454([a, h, b, e, f, g, i, j, k, l])
    } else {
        select_394([a, f, c, d, e, g, h, l])
    }
}
/// n = 13, i = 6, a < f, a < g, b < i, b < k, c < g, c < j, c < k, d < h, d < i, e < h, e < i, g < m, h < k, h < m, i < j, k < l
fn select_456([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_452([h, a, c, b, f, i, j, k, g, l])
    } else {
        select_455([b, a, d, e, c, f, g, h, k, m, l, i])
    }
}
/// n = 7, i = 3, a < d, b < f, b < g, c < e, e < f, e < g
fn select_457([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_7([b, d, c, e])
    } else {
        select_19([a, b, f, g, d])
    }
}
/// n = 9, i = 5, a < g, b < h, c < f, c < i, d < g, d < h, e < g, e < h, h < i
fn select_458([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_457([a, c, b, g, h, f, i])
    } else {
        select_394([b, a, d, e, c, f, g, h])
    }
}
/// n = 7, i = 3, a < f, b < e, b < g, c < d, c < g
fn select_459([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_346([a, b, c, f, d, g, e])
    } else {
        select_346([a, c, b, f, e, g, d])
    }
}
/// n = 9, i = 5, b < h, c < f, c < i, d < g, d < h, e < g, e < h, g < i
fn select_460([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_458([b, a, c, d, e, f, h, g, i])
    } else {
        select_459([b, c, g, a, f, h, i])
    }
}
/// n = 13, i = 6, a < k, a < l, b < f, b < g, c < i, c < l, d < h, d < i, e < h, e < i, g < j, h < j, h < l, i < k, l < m
fn select_461([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < g {
        select_456([b, c, a, d, e, f, g, h, i, k, l, m, j])
    } else {
        select_460([f, c, g, d, e, a, h, i, j])
    }
}
/// n = 9, i = 6, c < i, d < h, e < h, f < g, g < i, h < i
fn select_462([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < a {
        select_123([b, c, f, h, a, g, i])
    } else {
        select_415([b, c, a, d, e, f, g, h])
    }
}
/// n = 8, i = 5, a < h, b < g, c < g, d < e, e < f, g < h
fn select_463([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_30([a, g, d, e, h, f])
    } else {
        select_136([a, b, c, f, g])
    }
}
/// n = 8, i = 5, a < h, b < g, c < g, d < e, d < f, g < h
fn select_464([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_463([a, b, c, d, e, f, g, h])
    } else {
        select_463([a, b, c, d, f, e, g, h])
    }
}
/// n = 9, i = 6, a < i, b < h, c < h, d < h, e < f, e < g, h < i
fn select_465([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_464([a, c, d, e, f, g, h, i])
    } else {
        select_363([a, g, b, c, d, h])
    }
}
/// n = 12, i = 7, a < j, b < j, b < l, c < i, c < k, d < h, e < f, e < g, g < i, h < j, h < l, i < l, j < k
fn select_466([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < j {
        select_462([a, f, b, c, g, d, h, i, l])
    } else {
        select_465([c, a, b, h, e, f, g, j, k])
    }
}
/// n = 9, i = 6, a < h, b < h, c < h, d < g, d < i, e < f, f < g, h < i
fn select_467([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_26([d, h, e, f, i])
    } else {
        select_127([a, b, c, g])
    }
}
/// n = 9, i = 6, a < i, b < h, c < h, d < h, e < f, f < g, h < i
fn select_468([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_467([b, c, d, a, e, f, g, h, i])
    } else {
        select_142([b, c, d, g, a, h])
    }
}
/// n = 10, i = 6, b < i, b < j, c < h, d < h, e < f, f < g, f < j, h < i, h < j
fn select_469([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_81([b, e, h, a, f, g, i, j])
    } else {
        select_468([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 8, i = 4, a < f, a < g, b < e, c < d, d < f, d < h, e < g, e < h
fn select_470([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_4([a, c, g, d, h])
    } else {
        select_4([a, b, f, e, h])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, c < g, c < h, d < e, e < f, e < h, g < i
fn select_471([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_470([a, b, d, e, g, h, i, f])
    } else {
        select_190([b, f, a, c, h, g])
    }
}
/// n = 7, i = 4, a < f, a < g, b < e, c < d, c < f, e < g
fn select_472([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_89([a, e, c, d, g, f])
    } else {
        select_6([d, b, f, e])
    }
}
/// n = 8, i = 4, a < g, a < h, b < d, b < g, c < e, e < f, e < h
fn select_473([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_472([a, c, b, d, e, g, h])
    } else {
        select_188([a, b, e, f, d, h])
    }
}
/// n = 9, i = 5, b < h, b < i, c < f, d < e, d < i, e < h, f < g, f < i
fn select_474([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_471([b, a, d, c, f, g, e, i, h])
    } else {
        select_473([b, e, c, a, f, g, h, i])
    }
}
/// n = 10, i = 6, b < i, c < i, c < j, d < f, e < g, f < i, f < j, g < h, g < j
fn select_475([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < f {
        select_469([a, c, b, d, e, g, h, f, i, j])
    } else {
        select_474([a, c, e, f, b, g, h, i, j])
    }
}
/// n = 12, i = 7, a < l, b < j, c < j, c < k, d < h, e < f, e < g, g < i, h < j, h < k, i < k, j < l
fn select_476([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < i {
        select_466([b, c, a, d, e, f, g, h, i, j, l, k])
    } else {
        select_475([f, b, c, d, g, h, i, a, j, k])
    }
}
/// n = 14, i = 7, a < l, a < m, b < f, b < g, c < k, c < m, d < j, d < k, e < h, g < i, h < j, h < k, j < m, k < l, m < n
fn select_477([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < i {
        select_461([a, b, c, d, h, f, g, j, k, i, l, m, n])
    } else {
        select_476([a, c, d, e, b, f, g, h, i, k, j, l])
    }
}
/// n = 8, i = 4, a < h, b < f, b < h, c < f, c < h, d < e, d < g, d < h, f < g
fn select_478([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_57([a, f, d, e, g, h])
    } else {
        select_120([a, e, b, c, f, h])
    }
}
/// n = 10, i = 5, a < h, b < g, c < g, c < h, d < g, d < h, e < f, e < h, e < j, g < j, h < i
fn select_479([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_44([b, c, d, e, f, g, h, i])
    } else {
        select_478([a, c, d, e, f, g, j, h])
    }
}
/// n = 9, i = 5, a < i, b < g, c < g, c < i, d < g, d < i, e < f, e < h, f < i, g < h
fn select_480([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_373([a, c, d, e, f, g, h])
    } else {
        select_120([a, b, c, d, g, i])
    }
}
/// n = 11, i = 5, a < k, b < h, c < h, c < i, d < g, e < g, e < h, f < g, f < h, g < i, g < k, h < j
fn select_481([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < a {
        select_479([b, d, e, f, c, a, g, h, j, i])
    } else {
        select_480([b, d, e, f, a, c, g, k, h])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, b < g, c < f, c < g, d < e, d < f, f < h
fn select_482([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_153([a, b, d, f, g, e, h])
    } else {
        select_30([a, c, d, e, g, f])
    }
}
/// n = 9, i = 4, a < i, b < e, b < g, c < f, c < g, d < e, d < f, d < h, f < i, g < h
fn select_483([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_482([a, b, c, d, e, f, g, i])
    } else {
        select_51([g, d, e, f, a, h])
    }
}
/// n = 7, i = 3, b < e, b < f, c < d, c < f, c < g, e < g
fn select_484([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_144([a, e, c, f, d, g])
    } else {
        select_191([a, d, b, f, e])
    }
}
/// n = 9, i = 4, a < i, b < g, c < e, c < f, d < f, d < g, d < h, e < h, g < i
fn select_485([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < e {
        select_483([a, c, b, d, f, g, e, h, i])
    } else {
        select_484([a, e, d, f, b, h, g])
    }
}
/// n = 13, i = 6, a < k, a < m, b < i, b < k, c < h, d < f, d < g, e < h, e < k, f < j, g < k, h < i, h < j, i < m, k < l
fn select_486([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_481([f, a, b, c, e, g, h, k, i, l, j])
    } else {
        select_485([a, b, d, h, f, g, i, j, m])
    }
}
/// n = 7, i = 2, a < g, b < e, c < d, c < g, e < f
fn select_487([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_178([a, c, e, f, d])
    } else {
        select_178([d, a, b, e, g])
    }
}
/// n = 7, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g
fn select_488([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_4([a, b, d, f, g])
    } else {
        select_4([a, c, e, f, g])
    }
}
/// n = 8, i = 3, a < f, b < d, b < e, c < e, c < f, c < g, d < g, f < h
fn select_489([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_53([c, d, a, e, g])
    } else {
        select_488([b, a, c, e, d, f, h])
    }
}
/// n = 8, i = 3, a < g, b < e, b < f, c < d, c < f, c < g, g < h
fn select_490([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_487([d, a, b, f, g, h, e])
    } else {
        select_489([a, b, c, e, f, g, d, h])
    }
}
/// n = 9, i = 4, a < g, b < g, b < h, c < e, d < f, d < g, d < h, e < f, e < h, g < i
fn select_491([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_258([c, d, b, a, e, f, g, i, h])
    } else {
        select_318([c, d, a, e, b, f, g, i])
    }
}
/// n = 10, i = 4, a < e, b < h, b < j, c < f, c < h, d < f, d < g, d < h, e < g, f < j, h < i
fn select_492([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_254([a, f, b, h, i, j, e])
    } else {
        select_491([b, c, a, d, e, g, h, f, i])
    }
}
/// n = 12, i = 5, a < e, a < k, b < g, b < l, c < f, d < g, d < i, e < j, f < i, f < j, g < h, g < k, i < l
fn select_493([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_490([c, a, g, h, e, k, f, i])
    } else {
        select_492([a, b, d, f, e, i, j, g, h, l])
    }
}
/// n = 13, i = 6, a < i, a < m, b < f, b < g, c < i, c < k, d < h, e < h, e < i, g < l, h < k, h < l, i < j, k < m
fn select_494([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < i {
        select_486([a, c, d, b, e, g, f, h, k, l, i, j, m])
    } else {
        select_493([b, a, d, c, g, h, i, j, k, l, f, m])
    }
}
/// n = 8, i = 4, a < h, b < g, c < f, d < e, d < g, d < h, f < g, f < h
fn select_495([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_90([c, a, d, e, b, f, h])
    } else {
        select_90([c, b, d, e, a, f, g])
    }
}
/// n = 10, i = 6, b < j, c < i, d < h, e < h, f < g, g < i, g < j, h < i, h < j
fn select_496([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_495([b, c, f, h, a, g, i, j])
    } else {
        select_415([b, c, a, d, e, f, g, h])
    }
}
/// n = 7, i = 4, c < f, d < f, e < f, f < g
fn select_497([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_128([a, c, d, e, b, f, g])
    } else {
        select_128([b, c, d, e, a, f, g])
    }
}
/// n = 8, i = 5, c < g, d < g, e < g, f < g, g < h
fn select_498([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_497([a, b, d, e, f, g, h])
    } else {
        select_497([a, b, c, e, f, g, h])
    }
}
/// n = 10, i = 6, b < h, c < h, d < h, e < f, e < g, g < j, h < i, h < j
fn select_499([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_484([a, e, h, i, g, f, j])
    } else {
        select_498([a, g, b, c, d, f, h, i])
    }
}
/// n = 12, i = 7, a < l, b < j, c < i, c < j, d < h, e < f, e < g, g < i, h < j, h < l, i < l, j < k
fn select_500([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < j {
        select_496([f, a, b, c, g, d, h, i, j, l])
    } else {
        select_499([a, b, c, h, e, f, g, j, k, i])
    }
}
/// n = 7, i = 5, a < g, b < g, c < f, d < e
fn select_501([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_25([a, b, c, e, f, g])
    } else {
        select_25([a, b, d, f, e, g])
    }
}
/// n = 8, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h
fn select_502([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < h {
        select_501([b, c, a, d, f, e, g])
    } else {
        select_7([a, g, f, h])
    }
}
/// n = 9, i = 5, a < h, b < e, b < i, c < g, d < f, d < h, f < i, g < h, g < i
fn select_503([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_157([b, c, f, e, a, g, h, i])
    } else {
        select_502([b, a, d, c, e, g, f, h])
    }
}
/// n = 8, i = 5, a < h, b < f, c < g, d < e, d < h, g < h
fn select_504([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_502([b, a, d, c, f, g, e, h])
    } else {
        select_31([b, c, e, f, a, g, h])
    }
}
/// n = 9, i = 5, b < i, c < h, d < g, e < f, e < h, f < i, g < h, g < i
fn select_505([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_503([c, b, d, e, a, f, g, h, i])
    } else {
        select_504([c, a, d, e, f, b, g, h])
    }
}
/// n = 12, i = 7, a < l, b < j, c < j, d < h, e < f, e < g, g < i, h < j, h < l, i < l, j < k
fn select_506([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < i {
        select_500([a, c, b, d, e, f, g, h, i, j, k, l])
    } else {
        select_505([f, c, a, d, i, b, h, l, j])
    }
}
/// n = 14, i = 7, a < k, a < n, b < j, c < f, c < g, d < k, d < m, e < h, g < i, h < j, h < k, j < m, k < l, m < n
fn select_507([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < i {
        select_494([a, c, d, b, h, f, g, j, k, l, m, i, n])
    } else {
        select_506([b, a, d, e, c, f, g, h, i, k, l, j])
    }
}
/// n = 14, i = 7, a < l, a < n, b < i, c < f, c < g, d < k, d < l, e < h, e < k, g < j, h < l, i < k, k < n, l < m
fn select_508([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < h {
        select_477([a, c, d, e, b, f, g, i, j, h, k, n, l, m])
    } else {
        select_507([a, b, c, d, e, f, g, h, j, i, l, m, k, n])
    }
}
/// n = 16, i = 8, a < l, b < l, b < o, c < i, c < p, d < f, d < h, e < g, e < j, g < n, h < k, h < p, i < n, j < l, j < n, l < m, l < p, n < o
fn select_509([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < g {
        select_450([b, c, d, a, e, f, h, i, j, k, g, l, m, n, o, p])
    } else {
        select_508([b, c, d, j, g, f, h, a, i, k, n, l, m, o])
    }
}
/// n = 9, i = 5, b < i, c < h, d < h, d < i, e < g, f < g, g < h, g < i
fn select_510([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_241([a, d, e, f, b, c, g, i])
    } else {
        select_241([a, d, e, f, c, b, g, h])
    }
}
/// n = 5, i = 4
fn select_511([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_127([b, c, d, e])
    } else {
        select_127([a, c, d, e])
    }
}
/// n = 9, i = 5, a < f, b < g, c < g, d < g, e < g, g < h, g < i
fn select_512([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_9([a, g, h, i])
    } else {
        select_511([b, c, d, e, f])
    }
}
/// n = 9, i = 5, a < i, b < f, c < g, d < g, e < g, g < h, g < i
fn select_513([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_512([b, a, c, d, e, f, g, h, i])
    } else {
        select_46([b, g, h, f, a, i])
    }
}
/// n = 9, i = 5, c < i, d < g, e < g, f < g, g < h, g < i
fn select_514([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_513([c, a, d, e, f, b, g, h, i])
    } else {
        select_513([c, b, d, e, f, a, g, h, i])
    }
}
/// n = 10, i = 5, b < j, c < h, d < g, e < h, e < j, f < g, f < h, g < j, h < i
fn select_515([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_510([a, b, c, e, d, f, g, h, j])
    } else {
        select_514([a, b, d, c, e, f, h, i, g])
    }
}
/// n = 9, i = 5, b < i, c < h, d < h, d < i, e < g, e < h, f < g, f < h, g < i
fn select_516([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < h {
        select_510([a, b, c, d, e, f, g, h, i])
    } else {
        select_498([a, b, c, d, e, f, h, g])
    }
}
/// n = 11, i = 6, b < k, c < i, d < g, e < i, e < k, f < h, f < k, g < k, h < i, i < j
fn select_517([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_515([a, b, c, d, e, h, g, i, j, k])
    } else {
        select_516([a, c, b, e, f, g, h, k, i])
    }
}
/// n = 8, i = 3, b < e, b < f, b < h, c < d, c < f, c < g, d < h, e < g
fn select_518([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_144([a, d, b, f, e, h])
    } else {
        select_144([a, e, c, f, d, g])
    }
}
/// n = 8, i = 3, b < g, c < f, c < h, d < e, d < f, d < g, e < h
fn select_519([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_484([a, b, d, e, c, g, f])
    } else {
        select_518([a, c, d, e, b, f, g, h])
    }
}
/// n = 9, i = 3, a < h, b < g, c < f, c < i, d < e, d < f, d < g, d < h, e < i
fn select_520([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < h {
        select_519([a, b, c, d, e, f, g, i])
    } else {
        select_519([b, a, c, d, e, f, h, i])
    }
}
/// n = 11, i = 5, a < k, b < j, c < h, c < i, d < g, e < g, f < g, g < h, g < i, g < j, g < k
fn select_521([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_40([b, c, g, a, k, h, i, j])
    } else {
        select_498([b, c, a, d, e, f, g, k])
    }
}
/// n = 12, i = 5, a < l, b < k, c < g, c < j, d < h, e < g, e < h, f < g, f < h, g < i, h < j, h < k, h < l
fn select_522([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_514([a, b, d, c, e, f, g, i, h])
    } else {
        select_521([a, b, c, d, e, f, h, g, j, k, l])
    }
}
/// n = 12, i = 5, a < l, b < k, c < h, d < h, d < i, e < g, f < g, f < h, g < i, g < k, g < l, h < j
fn select_523([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < c {
        select_520([a, b, d, g, c, i, k, l, h])
    } else {
        select_522([a, b, d, e, c, f, h, g, j, i, k, l])
    }
}
/// n = 13, i = 6, a < m, b < g, c < j, c < l, d < i, e < i, e < j, f < h, f < i, h < j, i < l, i < m, j < k
fn select_524([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < i {
        select_517([a, d, c, b, e, f, g, h, j, k, i])
    } else {
        select_523([a, b, h, c, d, e, i, j, l, k, g, m])
    }
}
/// n = 9, i = 5, b < g, c < g, c < h, d < g, d < h, e < f, e < i, f < h, g < i
fn select_525([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_358([a, c, d, e, f, g, i, h])
    } else {
        select_359([a, b, c, d, h, g])
    }
}
/// n = 8, i = 3, a < h, b < e, b < f, b < h, c < d, c < f, c < g, c < h, e < g
fn select_526([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_170([a, d, b, f, e, h])
    } else {
        select_55([a, e, c, f, d, g, h])
    }
}
/// n = 11, i = 5, a < k, b < i, c < g, d < g, d < i, e < f, e < h, e < k, f < i, f < j, g < h, g < j, g < k
fn select_527([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < g {
        select_525([a, c, b, d, e, f, g, i, h])
    } else {
        select_526([a, e, g, b, f, h, j, k])
    }
}
/// n = 7, i = 3, b < e, c < d, c < f, c < g, e < f, e < g
fn select_528([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_54([c, e, a, d, f, g])
    } else {
        select_26([a, b, c, d, e])
    }
}
/// n = 9, i = 4, a < d, a < i, b < f, c < e, c < g, c < i, e < h, f < g, f < h, f < i
fn select_529([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_528([d, b, c, e, f, g, i])
    } else {
        select_323([a, c, g, d, e, h, i])
    }
}
/// n = 9, i = 4, b < i, c < f, d < e, d < g, d < i, e < h, f < g, f < h, f < i
fn select_530([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_147([a, c, d, b, e, f, g, h])
    } else {
        select_529([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 12, i = 6, a < l, b < k, c < h, d < h, d < k, e < f, f < g, f < j, f < l, g < k, h < i, h < j, h < l
fn select_531([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_527([a, b, c, d, f, g, h, j, k, i, l])
    } else {
        select_530([b, a, e, h, i, f, j, g, l])
    }
}
/// n = 8, i = 3, b < e, b < h, c < d, c < f, c < g, d < h, e < f, e < g
fn select_532([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_109([d, b, a, e, f, g, h])
    } else {
        select_141([a, c, b, e, d, f, g])
    }
}
/// n = 8, i = 5, a < f, b < g, c < g, d < g, e < f, e < h, g < h
fn select_533([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_6([a, e, g, h])
    } else {
        select_127([b, c, d, f])
    }
}
/// n = 8, i = 5, b < g, c < g, d < g, e < f, e < h, g < h
fn select_534([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_533([a, b, c, d, e, f, g, h])
    } else {
        select_128([f, b, c, d, a, g, h])
    }
}
/// n = 10, i = 5, b < j, c < g, d < g, e < f, e < j, f < h, f < i, g < h, g < i, g < j
fn select_535([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_532([a, e, g, b, f, h, i, j])
    } else {
        select_534([a, b, c, d, e, f, g, j])
    }
}
/// n = 11, i = 6, b < k, c < h, d < h, e < f, f < g, f < k, g < j, h < i, h < j, h < k
fn select_536([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_535([a, b, c, d, f, g, h, i, j, k])
    } else {
        select_431([a, b, e, h, i, f, g, k])
    }
}
/// n = 9, i = 4, b < i, c < g, c < h, d < f, e < f, f < g, f < h, f < i
fn select_537([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_363([b, c, a, d, e, f])
    } else {
        select_55([b, c, f, a, g, h, i])
    }
}
/// n = 13, i = 6, a < m, b < j, c < h, c < l, d < h, d < j, e < f, f < g, f < j, g < k, h < i, h < k, j < l, j < m
fn select_538([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < j {
        select_536([a, b, c, d, e, f, g, h, i, k, j])
    } else {
        select_537([g, a, c, b, d, j, h, l, m])
    }
}
/// n = 14, i = 6, a < m, a < n, b < j, c < h, c < l, d < h, d < j, e < f, e < g, f < k, f < m, g < j, h < i, h < k, h < m, j < l, j < n
fn select_539([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < g {
        select_531([a, b, c, d, e, f, g, h, i, k, j, m])
    } else {
        select_538([a, b, c, d, e, g, f, h, i, j, k, l, n])
    }
}
/// n = 7, i = 3, b < g, c < e, c < g, d < e, d < g, e < f
fn select_540([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_19([b, e, a, f, g])
    } else {
        select_120([a, b, c, d, g, e])
    }
}
/// n = 8, i = 4, b < f, c < g, d < f, d < g, e < f, e < g, g < h
fn select_541([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < b {
        select_540([a, b, d, e, g, h, f])
    } else {
        select_359([a, c, d, e, f, g])
    }
}
/// n = 6, i = 2, b < d, c < e, e < f
fn select_542([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_187([a, c, b, d, e])
    } else {
        select_45([b, c, d, e, a, f])
    }
}
/// n = 8, i = 3, a < h, b < e, b < h, c < f, c < h, d < e, d < f, f < g
fn select_543([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < c {
        select_542([a, b, c, e, f, g])
    } else {
        select_254([a, b, d, f, g, e, h])
    }
}
/// n = 10, i = 5, a < j, b < h, b < j, c < f, c < h, d < g, d < h, e < g, e < h, g < j, h < i
fn select_544([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_541([a, f, b, d, e, g, h, i])
    } else {
        select_543([a, g, b, c, f, h, i, j])
    }
}
/// n = 9, i = 5, a < i, b < h, b < i, c < f, d < g, d < h, e < g, e < h, f < h, g < i
fn select_545([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_118([b, c, d, e, a, f, g, i])
    } else {
        select_396([b, d, e, f, a, g, h])
    }
}
/// n = 10, i = 5, b < j, c < h, d < h, d < j, e < g, e < h, f < g, f < h, g < j, h < i
fn select_546([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_544([b, d, c, e, f, a, g, h, i, j])
    } else {
        select_545([b, d, a, e, f, c, g, h, j])
    }
}
/// n = 9, i = 5, a < f, a < i, b < f, b < h, c < g, d < g, e < g, g < h, g < i
fn select_547([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_18([a, b, g, h, i])
    } else {
        select_127([c, d, e, f])
    }
}
/// n = 9, i = 5, a < i, b < f, b < h, c < g, d < g, e < g, g < h, g < i
fn select_548([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_547([a, b, c, d, e, f, g, h, i])
    } else {
        select_128([f, c, d, e, a, g, h])
    }
}
/// n = 10, i = 5, a < j, b < i, c < h, d < g, e < g, f < g, g < h, g < i, g < j
fn select_549([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_548([c, a, d, e, f, b, g, j, h])
    } else {
        select_548([c, b, d, e, f, a, g, i, h])
    }
}
/// n = 11, i = 5, a < k, b < i, b < j, c < g, d < h, e < h, e < j, f < g, f < h, g < j, h < i, h < k
fn select_550([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_549([a, b, c, d, e, f, h, g, i, k])
    } else {
        select_510([a, b, d, e, c, f, g, h, j])
    }
}
/// n = 13, i = 6, a < m, b < j, b < k, c < i, d < g, e < i, e < j, f < h, f < j, g < j, h < i, i < k, i < m, j < l
fn select_551([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_546([a, c, b, e, f, g, h, j, l, i])
    } else {
        select_550([a, b, d, c, e, h, g, i, k, j, m])
    }
}
/// n = 15, i = 7, a < n, a < o, b < l, c < j, c < m, d < j, d < l, e < i, e < j, f < g, f < i, g < h, g < n, i < l, j < k, j < n, l < m, l < o
fn select_552([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < h {
        select_539([a, b, c, d, f, g, i, j, k, l, h, m, n, o])
    } else {
        select_551([a, c, b, g, d, e, h, i, l, j, m, k, o])
    }
}
/// n = 15, i = 7, a < n, a < o, b < l, c < j, d < j, d < m, e < j, e < l, f < g, f < h, g < i, g < o, h < l, j < k, j < o, l < m, l < n
fn select_553([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < c {
        select_524([a, g, d, b, e, h, i, c, l, j, k, m, n])
    } else {
        select_552([a, b, d, e, c, f, g, i, h, j, k, l, m, o, n])
    }
}
/// n = 8, i = 3, c < e, d < e, e < f, e < g, e < h
fn select_554([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_32([a, c, d, b, e, f, g, h])
    } else {
        select_32([b, c, d, a, e, f, g, h])
    }
}
/// n = 11, i = 6, b < g, c < g, d < f, e < f, f < i, f < j, f < k, g < h, g < j, g < k
fn select_555([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_554([a, i, b, c, g, h, j, k])
    } else {
        select_554([a, h, d, e, f, i, j, k])
    }
}
/// n = 12, i = 7, b < h, c < h, c < j, d < h, d < i, e < g, f < g, g < i, g < j, g < l, h < k, h < l
fn select_556([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < j {
        select_555([a, b, c, e, f, g, h, k, i, j, l])
    } else {
        select_123([a, b, c, d, i, j, h])
    }
}
/// n = 10, i = 5, b < g, c < g, d < e, d < f, d < i, f < j, g < h, g < i, g < j
fn select_557([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_239([a, e, f, b, c, g, i, h, j])
    } else {
        select_484([a, g, d, e, h, i, f])
    }
}
/// n = 12, i = 7, b < k, c < g, d < h, e < h, f < g, f < j, g < i, g < l, h < j, h < k, h < l
fn select_558([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < g {
        select_556([a, c, b, f, d, e, h, g, j, k, i, l])
    } else {
        select_557([a, d, e, g, i, b, h, j, l, k])
    }
}
/// n = 9, i = 4, a < i, b < e, b < g, b < h, c < f, d < f, f < g, f < h, f < i
fn select_559([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_305([b, f, e, a, i, g, h])
    } else {
        select_128([b, a, c, d, e, f, i])
    }
}
/// n = 7, i = 4, a < g, b < g, c < e, d < f, f < g
fn select_560([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_31([c, a, d, e, f, b, g])
    } else {
        select_31([c, b, d, e, f, a, g])
    }
}
/// n = 9, i = 4, a < f, b < g, c < g, d < e, d < h, d < i, g < h, g < i
fn select_561([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_559([a, d, b, c, e, g, h, i, f])
    } else {
        select_560([b, c, d, a, e, f, g])
    }
}
/// n = 9, i = 3, a < d, b < f, b < h, b < i, c < e, c < f, e < h, e < i, f < g
fn select_562([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_191([d, b, c, e, f])
    } else {
        select_217([a, c, d, f, e, h, i, g])
    }
}
/// n = 7, i = 2, a < c, b < d, b < e, d < g, e < f
fn select_563([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_19([e, d, a, g, f])
    } else {
        select_50([a, b, c, e, f, d])
    }
}
/// n = 9, i = 4, a < d, b < f, c < e, c < g, e < i, f < g, f < i, g < h
fn select_564([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_80([d, b, c, e, f, g])
    } else {
        select_563([a, c, d, e, g, h, i])
    }
}
/// n = 10, i = 4, a < e, b < d, b < h, b < j, c < f, f < g, f < h, g < j, h < i
fn select_565([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_562([a, b, f, e, g, h, i, d, j])
    } else {
        select_564([a, c, b, e, d, f, h, i, g])
    }
}
/// n = 10, i = 4, a < j, b < f, c < g, d < e, d < h, d < j, g < h, g < j, h < i
fn select_566([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_561([b, a, c, d, e, f, g, h, j])
    } else {
        select_565([b, d, c, e, f, g, a, h, i, j])
    }
}
/// n = 13, i = 7, a < m, b < k, c < g, c < l, d < g, d < j, d < m, e < h, f < h, g < i, h < j, h < k, h < l, l < m
fn select_567([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < l {
        select_558([a, b, c, e, f, d, g, h, i, j, k, l])
    } else {
        select_566([a, b, c, d, j, k, l, g, i, m])
    }
}
/// n = 8, i = 4, b < g, b < h, c < f, c < h, d < e, d < g, e < h, f < g
fn select_568([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_359([a, b, c, e, f, h])
    } else {
        select_359([a, b, d, f, e, g])
    }
}
/// n = 11, i = 6, b < g, b < k, c < h, d < h, e < f, e < g, f < k, g < j, h < i, h < j, h < k
fn select_569([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_535([a, f, c, d, b, g, h, i, j, k])
    } else {
        select_568([a, b, e, h, i, f, k, g])
    }
}
/// n = 7, i = 4, a < g, b < g, c < f, c < g, d < e, e < f
fn select_570([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_30([b, c, d, e, g, f])
    } else {
        select_7([a, b, c, f])
    }
}
/// n = 8, i = 4, b < f, c < f, c < g, d < e, d < h, e < g, f < h
fn select_571([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_426([b, d, e, f, a, g, h])
    } else {
        select_570([a, b, c, d, e, g, f])
    }
}
/// n = 7, i = 2, a < e, a < g, b < c, b < d, b < g, e < f
fn select_572([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_198([c, d, a, g])
    } else {
        select_50([b, a, d, e, f, g])
    }
}
/// n = 8, i = 3, b < d, b < g, c < e, c < h, d < h, e < f, e < g
fn select_573([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_572([b, e, a, f, d, h, g])
    } else {
        select_392([a, b, c, d, e, f, h])
    }
}
/// n = 9, i = 4, b < g, b < i, c < f, c < g, d < e, d < f, d < h, f < i, g < h
fn select_574([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_571([a, b, c, d, e, g, f, h])
    } else {
        select_573([a, b, d, g, f, e, i, h])
    }
}
/// n = 11, i = 6, b < g, b < j, c < f, c < g, d < h, e < h, g < k, h < i, h < j, h < k
fn select_575([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < j {
        select_569([a, b, d, e, c, f, g, h, i, k, j])
    } else {
        select_574([a, c, b, h, i, j, g, k, f])
    }
}
/// n = 10, i = 5, a < e, b < g, c < g, d < f, d < h, f < i, f < j, g < h, g < i, g < j
fn select_576([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_125([a, d, h, e, f, i, j])
    } else {
        select_373([e, b, c, d, f, g, h])
    }
}
/// n = 10, i = 5, a < e, b < f, b < i, c < g, d < g, f < j, g < h, g < i, g < j
fn select_577([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_576([a, c, d, b, e, f, g, i, h, j])
    } else {
        select_346([a, b, g, e, h, i, f])
    }
}
/// n = 10, i = 5, c < f, c < i, d < g, e < g, f < j, g < h, g < i, g < j
fn select_578([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_577([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_577([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 11, i = 6, b < j, c < f, c < g, d < h, e < h, g < k, h < i, h < j, h < k
fn select_579([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < g {
        select_575([a, b, c, d, e, f, g, h, i, j, k])
    } else {
        select_578([a, f, g, d, e, b, h, i, k, j])
    }
}
/// n = 13, i = 7, a < m, b < k, c < j, c < m, d < g, d < l, e < i, f < i, g < h, i < j, i < k, i < l, l < m
fn select_580([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < g {
        select_567([a, b, d, c, e, f, g, i, h, j, k, l, m])
    } else {
        select_579([a, b, g, e, f, h, c, i, l, k, j])
    }
}
/// n = 9, i = 5, a < i, b < g, c < g, d < e, d < i, e < f, g < h, g < i
fn select_581([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_115([a, f, b, c, g, h])
    } else {
        select_255([h, a, d, e, f, i])
    }
}
/// n = 6, i = 4, c < f, d < e
fn select_582([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_27([b, a, d, c, f, e])
    } else {
        select_134([b, c, e, a, f])
    }
}
/// n = 11, i = 6, a < h, a < k, b < i, c < i, d < g, d < i, e < f, e < k, f < g, f < h, i < j, i < k
fn select_583([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_581([a, b, c, e, f, g, i, j, k])
    } else {
        select_582([b, c, a, d, g, h])
    }
}
/// n = 7, i = 3, a < g, b < g, c < e, d < e, d < g, e < f
fn select_584([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_392([c, a, d, b, e, f, g])
    } else {
        select_392([c, b, d, a, e, f, g])
    }
}
/// n = 9, i = 3, b < e, c < f, d < e, d < f, e < i, f < g, f < h
fn select_585([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_180([b, c, e, f, a, g, h, i])
    } else {
        select_584([a, c, b, d, e, i, f])
    }
}
/// n = 14, i = 7, a < g, b < k, b < l, c < h, d < h, e < h, e < j, f < g, f < l, g < j, g < k, h < i, h < l, l < m, l < n
fn select_586([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < h {
        select_583([b, c, d, e, f, g, j, k, h, i, l])
    } else {
        select_585([i, a, b, f, g, l, m, n, j])
    }
}
/// n = 7, i = 3, a < f, b < e, b < f, b < g, c < d, c < e, d < g
fn select_587([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_207([a, b, c, d, f, g, e])
    } else {
        select_152([d, e, a, g])
    }
}
/// n = 8, i = 4, a < g, b < e, c < d, c < g, c < h, e < f, e < h
fn select_588([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_279([d, a, b, g, e, h, f])
    } else {
        select_587([a, c, e, f, h, g, d])
    }
}
/// n = 8, i = 3, b < h, c < e, c < h, d < e, d < h, e < f, e < g
fn select_589([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_62([b, e, a, f, g, h])
    } else {
        select_120([a, b, c, d, h, e])
    }
}
/// n = 10, i = 4, b < f, c < g, d < f, d < g, e < f, e < g, f < j, g < h, g < i
fn select_590([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_540([a, c, d, e, f, j, g])
    } else {
        select_589([a, b, d, e, g, h, i, f])
    }
}
/// n = 13, i = 6, a < g, b < j, b < k, c < g, c < k, d < e, e < f, e < h, g < i, g < j, h < k, k < l, k < m
fn select_591([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_588([b, d, g, i, e, f, j, h])
    } else {
        select_590([f, a, b, c, h, g, k, l, m, i])
    }
}
/// n = 14, i = 7, a < l, b < g, c < k, c < l, d < h, e < h, e < j, f < g, f < l, g < j, g < k, h < i, h < l, l < m, l < n
fn select_592([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < h {
        select_586([b, c, a, d, e, f, g, h, i, j, k, l, m, n])
    } else {
        select_591([b, c, f, d, h, i, g, a, j, k, l, m, n])
    }
}
/// n = 8, i = 4, a < e, b < f, c < f, d < f, f < g, f < h
fn select_593([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_9([a, f, g, h])
    } else {
        select_127([b, c, d, e])
    }
}
/// n = 8, i = 3, a < e, b < e, b < f, b < h, c < d, c < f, d < h, f < g
fn select_594([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_50([d, b, a, f, g, h])
    } else {
        select_488([a, b, c, d, e, f, g])
    }
}
/// n = 10, i = 5, a < f, b < g, c < g, d < e, d < f, d < i, e < j, g < h, g < j, h < i
fn select_595([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_593([a, b, c, e, f, g, h, j])
    } else {
        select_594([a, d, g, h, f, e, j, i])
    }
}
/// n = 10, i = 6, a < i, b < f, c < f, d < g, e < g, f < i, f < j, g < h, g < j
fn select_596([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_381([a, d, e, f, i, g, h, j])
    } else {
        select_136([a, b, c, h, f])
    }
}
/// n = 13, i = 6, a < f, b < i, b < j, c < g, d < g, e < f, e < j, f < i, f < m, g < h, g < j, g < m, j < k, j < l
fn select_597([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < f {
        select_138([a, h, b, e, j, k, l])
    } else {
        select_596([b, a, e, c, d, f, g, h, i, m])
    }
}
/// n = 14, i = 6, a < j, a < n, b < h, b < j, c < f, d < g, e < g, f < h, f < k, f < n, g < i, g < j, g < n, i < k, j < l, j < m
fn select_598([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < a {
        select_595([b, d, e, f, a, h, g, i, k, n])
    } else {
        select_597([c, b, d, e, a, f, g, i, h, j, l, m, n])
    }
}
/// n = 7, i = 3, a < e, b < f, c < d, c < f, f < g
fn select_599([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_278([a, d, b, f, g])
    } else {
        select_134([b, a, c, d, e])
    }
}
/// n = 8, i = 4, b < e, c < g, d < f, f < g, g < h
fn select_600([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_599([b, c, f, a, e, g, h])
    } else {
        select_240([c, b, a, d, e, f, g])
    }
}
/// n = 7, i = 2, a < d, a < g, b < c, b < g, c < e, d < f
fn select_601([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_152([a, c, e, g])
    } else {
        select_152([b, d, f, g])
    }
}
/// n = 8, i = 2, a < g, a < h, b < f, b < h, c < d, c < f, c < g, d < e
fn select_602([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_601([a, c, d, b, e, h, g])
    } else {
        select_601([b, c, d, a, e, h, f])
    }
}
/// n = 11, i = 3, a < f, a < i, b < f, b < h, c < d, d < e, d < h, d < i, e < g, f < j, f < k
fn select_603([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < c {
        select_14([c, j, k])
    } else {
        select_602([a, b, d, e, g, h, i, f])
    }
}
/// n = 9, i = 3, a < e, b < d, c < e, c < h, c < i, d < h, d < i, e < f, e < g
fn select_604([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_9([a, c, h, i])
    } else {
        select_45([b, c, d, e, f, g])
    }
}
/// n = 11, i = 5, a < h, b < e, c < g, d < g, e < f, e < k, g < h, g < k, h < i, h < j
fn select_605([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_136([a, c, d, f, g])
    } else {
        select_604([a, b, g, e, h, i, j, f, k])
    }
}
/// n = 14, i = 6, a < j, a < l, b < h, b < j, c < f, d < g, e < g, f < h, f < i, f < l, g < i, g < j, g < l, i < k, j < m, j < n
fn select_606([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_603([a, b, c, f, i, j, k, h, l, m, n])
    } else {
        select_605([a, c, d, e, f, h, g, j, m, n, i])
    }
}
/// n = 14, i = 6, a < j, a < l, b < h, b < j, c < f, d < g, e < g, e < i, f < h, f < i, f < l, g < j, g < k, g < l, i < k, j < m, j < n
fn select_607([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_600([a, b, d, e, h, i, g, k])
    } else {
        select_606([a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 6, a < k, a < n, b < j, b < k, c < f, d < g, e < g, e < i, f < i, f < j, f < n, g < h, g < k, g < n, k < l, k < m
fn select_608([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_598([a, b, c, d, e, f, g, j, h, k, i, l, m, n])
    } else {
        select_607([a, b, c, d, e, f, g, j, i, k, h, n, l, m])
    }
}
/// n = 15, i = 7, a < j, b < h, c < g, c < n, d < g, d < m, e < j, e < l, f < h, f < j, g < i, g < o, h < l, h < m, h < n, j < k, j < n, j < o
fn select_609([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < j {
        select_592([a, b, e, c, d, f, h, g, i, m, l, j, k, o])
    } else {
        select_608([c, d, b, a, e, h, j, k, l, m, g, i, o, n])
    }
}
/// n = 10, i = 5, a < h, a < j, b < g, b < j, c < g, c < j, d < e, d < f, f < i, f < j, g < h, g < i
fn select_610([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_340([b, c, a, d, e, f, g, h, j])
    } else {
        select_358([e, b, c, f, a, g, i, j])
    }
}
/// n = 13, i = 6, a < h, b < i, b < j, c < h, c < j, d < h, d < j, e < f, e < g, g < j, g < m, h < i, h < m, j < k, j < l
fn select_611([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < g {
        select_610([b, c, d, e, f, g, h, i, m, j])
    } else {
        select_590([f, a, b, c, d, h, j, k, l, m])
    }
}
/// n = 7, i = 3, a < g, b < f, c < e, d < e, d < f, d < g
fn select_612([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_587([c, d, a, b, g, e, f])
    } else {
        select_587([c, d, b, a, f, e, g])
    }
}
/// n = 7, i = 3, a < e, b < e, b < f, c < d, c < f, f < g
fn select_613([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_89([a, b, c, d, e, f])
    } else {
        select_9([d, f, a, g])
    }
}
/// n = 7, i = 3, a < g, b < d, c < e, c < g, e < f
fn select_614([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_46([a, e, f, g, b, d])
    } else {
        select_613([a, c, b, d, g, e, f])
    }
}
/// n = 8, i = 4, a < g, b < e, b < g, b < h, c < d, c < f, f < h
fn select_615([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_612([a, d, f, b, h, e, g])
    } else {
        select_614([e, a, c, g, f, h, d])
    }
}
/// n = 13, i = 6, a < k, b < h, c < i, c < k, d < h, d < k, e < f, e < g, g < j, g < k, h < i, h < j, k < l, k < m
fn select_616([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < h {
        select_611([b, c, a, d, e, f, g, h, i, k, l, m, j])
    } else {
        select_615([c, h, e, f, a, g, i, j])
    }
}
/// n = 10, i = 5, a < h, b < e, b < j, c < f, c < h, d < f, d < h, e < g, h < i, h < j
fn select_617([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_396([a, c, d, e, g, f, h])
    } else {
        select_144([f, b, h, i, e, j])
    }
}
/// n = 7, i = 2, b < g, c < d, c < e, c < g, e < f
fn select_618([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_46([a, c, d, b, e, f])
    } else {
        select_323([b, c, d, a, e, f, g])
    }
}
/// n = 9, i = 3, a < e, a < i, b < c, b < d, b < h, d < f, d < i, e < g, e < h
fn select_619([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_54([b, e, g, c, d, h])
    } else {
        select_144([c, a, d, f, e, i])
    }
}
/// n = 11, i = 4, a < g, b < e, b < k, c < d, c < g, c < i, e < f, e < i, g < h, g < k, i < j
fn select_620([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_618([f, a, c, d, i, j, g])
    } else {
        select_619([b, c, d, g, e, h, f, i, k])
    }
}
/// n = 13, i = 6, a < e, b < h, c < f, c < m, d < h, d < j, e < j, e < m, f < g, f < k, h < i, h < k, h < m, k < l
fn select_621([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < h {
        select_617([b, c, d, e, f, j, g, h, i, m])
    } else {
        select_620([a, c, h, i, f, g, e, j, k, l, m])
    }
}
/// n = 15, i = 7, a < j, b < i, c < j, c < l, d < f, d < o, e < i, e < j, f < g, f < h, h < m, h < n, i < l, i < m, i < o, j < k, j < n, j < o
fn select_622([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < j {
        select_616([a, b, c, e, f, g, h, i, l, m, j, k, n])
    } else {
        select_621([b, a, d, c, i, f, g, j, k, l, h, n, o])
    }
}
/// n = 15, i = 7, a < m, a < o, b < j, c < i, d < g, d < n, e < j, e < l, f < i, f < j, g < h, g < o, i < l, i < m, i < n, j < k, j < n, j < o
fn select_623([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < g {
        select_609([b, c, d, a, e, f, g, i, h, j, k, l, m, n, o])
    } else {
        select_622([b, c, e, d, f, g, h, a, i, j, k, l, m, o, n])
    }
}
/// n = 15, i = 7, a < n, a < o, b < j, c < j, c < l, d < i, e < g, e < k, f < i, f < j, g < h, g < o, i < k, i < l, i < n, j < m, j < o
fn select_624([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < j {
        select_580([b, a, c, e, d, f, g, h, i, l, n, k, j])
    } else {
        select_623([a, b, d, e, c, f, g, h, i, j, m, l, n, k, o])
    }
}
/// n = 15, i = 7, a < n, a < o, b < k, c < j, d < k, d < m, e < g, e < h, f < j, f < k, h < i, h < o, j < m, j < n, k < l, k < o
fn select_625([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < j {
        select_553([a, c, b, d, f, e, h, g, i, k, l, j, m, n, o])
    } else {
        select_624([a, b, d, c, e, f, h, i, j, k, g, m, l, n, o])
    }
}
/// n = 16, i = 8, a < g, a < p, b < m, c < m, c < n, d < f, d < i, e < h, e < j, h < l, i < k, i < p, j < l, j < m, l < n, m < o, m < p
fn select_626([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < l {
        select_509([b, c, a, d, e, f, h, i, g, j, k, m, o, l, n, p])
    } else {
        select_625([a, b, h, c, d, j, f, i, k, l, m, o, n, g, p])
    }
}
/// n = 16, i = 8, a < h, a < p, b < n, c < m, c < n, d < g, d < i, e < f, e < j, i < k, i < p, j < l, j < n, l < m, n < o, n < p
fn select_627([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < f {
        select_372([a, b, c, d, e, g, h, i, j, k, l, f, m, n, o, p])
    } else {
        select_626([a, b, c, d, e, g, h, f, i, j, k, l, n, m, o, p])
    }
}
/// n = 9, i = 3, a < e, a < f, b < d, b < f, c < d, c < e, d < g, e < h, f < i
fn select_628([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_203([a, b, d, e, g, h])
    } else {
        select_203([b, c, e, f, h, i])
    }
}
/// n = 10, i = 3, a < g, b < f, b < g, b < i, c < d, c < f, d < e, d < i, f < h, g < j
fn select_629([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_572([b, d, a, e, f, h, i])
    } else {
        select_628([a, b, c, f, d, g, h, e, j])
    }
}
/// n = 10, i = 3, b < g, c < e, c < f, d < f, d < g, d < i, e < i, f < h, g < j
fn select_630([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_277([a, b, c, d, e, f, g, h, j])
    } else {
        select_629([b, d, c, e, a, f, g, h, i, j])
    }
}
/// n = 8, i = 3, b < e, b < f, c < d, c < f, c < h, e < h, f < g
fn select_631([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_323([e, c, d, a, f, g, h])
    } else {
        select_613([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 3, b < e, b < f, b < i, c < d, c < f, c < h, d < i, e < h, f < g
fn select_632([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_245([a, e, c, f, d, g, h])
    } else {
        select_532([a, c, b, e, d, f, i, h])
    }
}
/// n = 10, i = 3, b < g, c < f, c < i, d < e, d < f, d < g, e < i, f < h, g < j
fn select_633([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_631([a, b, d, e, c, g, j, f])
    } else {
        select_632([a, c, d, e, b, f, h, g, i])
    }
}
/// n = 10, i = 3, b < h, c < f, c < g, d < e, d < g, d < h, g < i, h < j
fn select_634([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_630([a, b, c, d, f, g, h, i, e, j])
    } else {
        select_633([a, b, c, d, e, g, h, i, f, j])
    }
}
/// n = 8, i = 1, a < h, b < g, c < d, c < e, c < f
fn select_635([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_56([a, c, d, e, f, b, h])
    } else {
        select_56([b, c, d, e, f, a, g])
    }
}
/// n = 8, i = 1, c < h, d < e, d < f, d < g
fn select_636([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_635([a, c, d, e, f, g, h, b])
    } else {
        select_635([b, c, d, e, f, g, h, a])
    }
}
/// n = 7, i = 2, a < g, b < f, c < d, c < f, c < g, d < e
fn select_637([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_19([b, d, a, e, f])
    } else {
        select_57([b, a, c, d, g, f])
    }
}
/// n = 8, i = 3, a < h, b < e, b < f, b < h, c < d, c < f, c < h, f < g
fn select_638([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_637([a, d, b, f, g, e, h])
    } else {
        select_637([a, e, c, f, g, d, h])
    }
}
/// n = 10, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < j, g < h, g < i
fn select_639([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_636([d, e, f, g, a, h, i, j])
    } else {
        select_638([a, b, c, d, e, f, j, g])
    }
}
/// n = 10, i = 3, b < e, b < i, c < d, c < f, c < i, f < g, f < h, i < j
fn select_640([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_219([a, d, f, g, h, b, e, i])
    } else {
        select_639([a, b, c, d, e, i, f, g, h, j])
    }
}
/// n = 10, i = 3, c < f, c < i, d < e, d < g, d < i, g < h, i < j
fn select_641([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_634([b, a, c, d, e, f, i, g, j, h])
    } else {
        select_640([b, c, d, e, f, g, a, h, i, j])
    }
}
/// n = 9, i = 4, b < e, c < g, c < h, d < f, f < g, f < h, h < i
fn select_642([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < a {
        select_290([b, c, f, e, g, h, a, i])
    } else {
        select_335([a, b, c, d, e, f, g, h])
    }
}
/// n = 8, i = 4, b < e, b < h, c < f, d < f, f < g, f < h
fn select_643([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_128([b, a, c, d, e, f, g])
    } else {
        select_51([b, f, a, g, e, h])
    }
}
/// n = 10, i = 4, b < e, b < j, c < f, d < f, f < g, f < h, f < j, h < i
fn select_644([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_618([a, b, f, g, h, i, j])
    } else {
        select_643([a, b, c, d, e, f, g, j])
    }
}
/// n = 10, i = 4, b < e, b < j, c < g, c < i, d < f, d < g, f < i, g < h, g < j
fn select_645([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_642([a, b, c, d, e, f, i, g, h])
    } else {
        select_644([a, b, c, d, e, g, h, f, i, j])
    }
}
/// n = 10, i = 4, b < g, b < j, c < f, c < h, d < e, d < h, h < i, h < j
fn select_646([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_645([a, b, c, d, g, e, h, i, f, j])
    } else {
        select_645([a, b, d, c, g, f, h, i, e, j])
    }
}
/// n = 11, i = 4, b < g, c < f, c < j, d < e, d < h, h < i, h < j, j < k
fn select_647([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < b {
        select_641([a, e, c, h, i, f, b, g, j, k])
    } else {
        select_646([a, c, b, d, e, g, f, h, i, j])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < e, b < h, d < g, e < f
fn select_648([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_109([a, b, c, d, e, g, h])
    } else {
        select_109([a, b, c, e, d, f, h])
    }
}
/// n = 8, i = 2, a < e, a < h, b < c, b < d, b < f, b < h, f < g
fn select_649([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_648([a, b, e, c, f, g, d, h])
    } else {
        select_648([a, b, e, d, f, g, c, h])
    }
}
/// n = 9, i = 4, a < e, b < g, c < f, c < g, d < f, d < g, g < h, g < i
fn select_650([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_111([a, f, b, g, h, i])
    } else {
        select_27([b, c, d, a, e, f])
    }
}
/// n = 10, i = 4, a < f, b < e, b < j, c < g, d < f, d < g, g < h, g < i, g < j
fn select_651([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_649([b, g, h, i, e, a, f, j])
    } else {
        select_650([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 8, i = 4, a < e, b < g, c < f, c < h, d < f, d < g, g < h
fn select_652([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_353([a, c, b, f, g, h, e])
    } else {
        select_121([b, c, d, a, e, f, g])
    }
}
/// n = 10, i = 5, a < e, b < h, b < i, c < g, d < f, d < i, f < h, g < i, i < j
fn select_653([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_406([a, b, d, g, e, f, i, j, h])
    } else {
        select_652([a, c, b, f, e, h, g, i])
    }
}
/// n = 11, i = 5, a < f, a < k, b < h, b < i, c < e, c < i, d < g, d < i, g < h, i < j, i < k
fn select_654([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < e {
        select_651([g, a, c, b, f, h, i, e, j, k])
    } else {
        select_653([a, b, c, d, f, g, e, h, i, j])
    }
}
/// n = 11, i = 5, a < h, a < k, b < g, b < i, c < f, c < i, d < e, d < i, i < j, i < k
fn select_655([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_654([a, b, c, d, f, h, e, g, i, j, k])
    } else {
        select_654([a, d, c, b, f, h, g, e, i, j, k])
    }
}
/// n = 12, i = 5, a < h, b < g, b < k, c < f, c < i, d < e, d < i, i < j, i < k, k < l
fn select_656([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < a {
        select_641([e, f, b, i, j, g, a, h, k, l])
    } else {
        select_655([b, a, c, d, e, f, h, g, i, j, k])
    }
}
/// n = 12, i = 5, a < h, b < g, b < k, c < f, c < k, d < e, d < i, i < j, i < k, k < l
fn select_657([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < b {
        select_647([e, a, c, i, j, f, h, b, g, k, l])
    } else {
        select_656([a, c, b, d, e, g, f, h, i, j, k, l])
    }
}
/// n = 8, i = 3, a < e, b < e, b < f, c < d, c < f, f < g, f < h
fn select_658([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_137([d, f, a, g, h])
    } else {
        select_89([a, b, c, d, e, f])
    }
}
/// n = 10, i = 5, a < f, b < g, c < g, c < h, d < e, d < f, e < h, e < j, g < i, g < j
fn select_659([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_650([a, b, c, e, f, h, g, i, j])
    } else {
        select_658([a, d, g, i, f, e, h, j])
    }
}
/// n = 11, i = 6, a < h, b < g, c < f, d < e, d < h, e < i, e < j, f < i, f < k, g < j, g < k
fn select_660([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_659([a, c, g, d, e, h, f, j, k, i])
    } else {
        select_659([a, b, f, d, e, h, g, i, k, j])
    }
}
/// n = 10, i = 5, a < e, a < j, b < g, c < g, c < h, d < f, d < j, f < h, f < i, g < i, g < j
fn select_661([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_12([a, b, d, f, g, h, e, i, j])
    } else {
        select_28([b, c, a, d, e, f, h, g])
    }
}
/// n = 10, i = 5, a < e, a < j, b < g, c < f, c < j, d < g, d < i, f < i, g < h, g < j
fn select_662([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_661([a, b, d, c, e, f, g, i, h, j])
    } else {
        select_380([c, a, g, e, h, f, j])
    }
}
/// n = 12, i = 6, a < h, a < l, b < f, b < l, c < e, c < i, d < g, d < h, e < k, f < j, f < k, g < i, g < j, i < l
fn select_663([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_660([a, b, c, d, g, e, f, h, i, j, k])
    } else {
        select_662([a, c, b, g, h, f, i, e, j, l])
    }
}
/// n = 6, i = 3, a < e, b < c, b < d, e < f
fn select_664([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < c {
        select_53([b, e, f, d, c])
    } else {
        select_6([d, a, c, e])
    }
}
/// n = 8, i = 4, a < h, b < e, c < d, c < h, d < g, e < f, e < h
fn select_665([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < d {
        select_338([a, c, b, e, f, d, g, h])
    } else {
        select_664([b, d, a, g, e, f])
    }
}
/// n = 9, i = 5, a < f, b < g, c < g, c < h, d < e, d < f, e < h, g < i
fn select_666([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_665([a, b, d, e, g, i, h, f])
    } else {
        select_121([b, e, c, a, f, h, g])
    }
}
/// n = 11, i = 6, a < h, b < g, c < e, c < i, d < f, d < h, e < k, f < j, f < k, g < i, g < j
fn select_667([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < i {
        select_660([a, b, c, d, f, e, g, h, k, j, i])
    } else {
        select_666([a, c, g, d, f, h, i, j, e])
    }
}
/// n = 12, i = 6, a < i, b < j, c < h, c < l, d < f, d < l, e < g, e < h, f < i, f < k, g < j, g < k, j < l
fn select_668([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < a {
        select_663([c, d, b, e, a, f, g, h, j, k, i, l])
    } else {
        select_667([c, d, a, e, b, g, f, h, i, k, j])
    }
}
/// n = 7, i = 3, b < g, c < f, d < e, e < f, e < g
fn select_669([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_122([c, a, d, b, e, f])
    } else {
        select_353([c, b, d, a, e, g, f])
    }
}
/// n = 9, i = 4, a < i, b < g, c < f, d < e, d < g, e < h, f < h, g < i
fn select_670([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_431([a, b, c, d, e, f, h, g])
    } else {
        select_669([e, a, c, b, g, f, i])
    }
}
/// n = 9, i = 5, b < g, c < g, c < h, d < f, d < i, e < f, e < i, f < h, g < i
fn select_671([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_358([a, d, e, c, g, f, h, i])
    } else {
        select_191([a, b, c, h, g])
    }
}
/// n = 7, i = 4, b < f, b < g, c < f, c < g, d < e, d < g
fn select_672([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_373([a, b, c, d, e, f, g])
    } else {
        select_115([a, e, b, c, g, f])
    }
}
/// n = 9, i = 5, b < i, c < f, d < g, d < h, e < g, e < h, f < h, g < i
fn select_673([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_671([a, c, b, d, e, g, f, i, h])
    } else {
        select_672([a, d, e, f, b, g, h])
    }
}
/// n = 10, i = 5, a < j, b < i, c < h, d < f, e < g, e < i, f < i, g < h, h < j
fn select_674([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_670([a, c, d, g, b, f, h, i, j])
    } else {
        select_673([a, c, d, b, e, f, g, i, h])
    }
}
/// n = 7, i = 4, c < g, d < f, e < f, f < g
fn select_675([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_240([c, a, d, e, b, f, g])
    } else {
        select_240([c, b, d, e, a, f, g])
    }
}
/// n = 9, i = 4, a < i, b < g, c < g, d < f, e < f, e < h, f < i, g < h
fn select_676([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_675([b, c, a, d, e, f, i])
    } else {
        select_675([a, d, e, b, c, g, h])
    }
}
/// n = 8, i = 4, a < g, b < e, c < d, c < g, d < h, e < f, e < h
fn select_677([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_122([d, a, b, g, e, h])
    } else {
        select_80([a, c, e, f, d, h])
    }
}
/// n = 8, i = 3, a < f, a < h, b < f, b < g, c < d, d < e, d < g, d < h
fn select_678([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_426([c, a, b, d, e, f, h])
    } else {
        select_426([c, b, a, d, e, f, g])
    }
}
/// n = 9, i = 4, a < h, a < i, b < e, c < d, c < g, c < i, d < h, e < f, e < g, e < i
fn select_679([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_170([a, f, c, g, d, i])
    } else {
        select_678([a, d, b, e, g, h, f, i])
    }
}
/// n = 9, i = 4, a < g, b < h, b < i, c < g, c < i, d < e, e < f, e < h, e < i
fn select_680([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_677([c, d, a, b, e, f, g, h])
    } else {
        select_679([c, d, b, a, e, f, h, g, i])
    }
}
/// n = 10, i = 4, a < j, b < h, b < i, c < g, d < g, d < i, e < f, f < h, f < i, g < j
fn select_681([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_676([b, c, d, e, a, f, g, j, h])
    } else {
        select_680([c, b, d, e, f, a, g, h, i])
    }
}
/// n = 11, i = 5, a < k, b < h, c < f, c < j, d < i, d < j, e < g, e < h, g < i, h < j, i < k
fn select_682([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_674([a, b, d, c, e, f, g, i, h, k])
    } else {
        select_681([a, c, g, d, b, h, i, f, j, k])
    }
}
/// n = 12, i = 6, a < j, b < k, c < i, c < l, d < f, d < l, e < g, e < i, f < j, g < h, g < k, k < l
fn select_683([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_668([a, b, c, d, e, f, g, i, j, k, h, l])
    } else {
        select_682([a, b, c, d, g, i, h, k, f, l, j])
    }
}
/// n = 8, i = 4, a < d, b < f, c < f, d < e, f < g, g < h
fn select_684([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_6([e, b, c, f])
    } else {
        select_152([a, g, h, d])
    }
}
/// n = 11, i = 6, a < e, b < g, c < g, d < f, e < h, e < i, f < h, f < j, g < i, g < j, j < k
fn select_685([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_684([a, b, c, e, h, g, j, k])
    } else {
        select_100([b, c, d, f, h, g, i])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, c < d, c < h, d < f, e < g, e < h
fn select_686([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_50([a, c, g, d, f, h])
    } else {
        select_7([a, f, b, e])
    }
}
/// n = 12, i = 6, a < g, a < l, b < f, c < g, c < h, d < e, d < l, e < i, e < j, f < h, f < i, g < j, h < l, j < k
fn select_687([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_685([b, a, c, d, f, e, g, i, h, j, k])
    } else {
        select_686([a, c, d, e, h, i, g, l])
    }
}
/// n = 8, i = 4, a < f, a < g, a < h, b < e, c < d, c < h, d < g, e < f, e < h
fn select_688([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_53([a, c, d, f, h])
    } else {
        select_99([a, b, e, g])
    }
}
/// n = 10, i = 5, a < e, b < f, c < d, c < g, d < i, d < j, e < h, e < i, f < g, f < h, g < j
fn select_689([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_688([e, b, c, d, f, h, i, g])
    } else {
        select_18([a, d, g, j, e])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, c < d, c < f, d < h, e < f, e < g
fn select_690([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_470([a, b, c, d, e, h, g, f])
    } else {
        select_4([a, c, g, f, d])
    }
}
/// n = 10, i = 5, a < e, b < d, b < h, c < f, d < j, e < g, e < i, f < h, f < i, h < j
fn select_691([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < g {
        select_689([a, c, b, d, e, f, h, i, g, j])
    } else {
        select_690([b, c, e, g, f, i, h, d])
    }
}
/// n = 12, i = 6, a < k, a < l, b < g, c < f, c < l, d < e, d < h, e < j, f < i, f < j, g < h, g < i, h < l, j < k
fn select_692([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < e {
        select_687([a, b, d, c, f, g, e, h, i, j, k, l])
    } else {
        select_691([b, e, c, a, g, f, h, j, i, k])
    }
}
/// n = 8, i = 3, a < f, a < g, b < d, c < e, c < h, d < h, e < f, e < g
fn select_693([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_194([a, c, d, e, h])
    } else {
        select_57([b, a, e, f, g, d])
    }
}
/// n = 8, i = 4, a < g, a < h, b < f, c < e, d < f, d < h, e < h, f < g
fn select_694([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_120([a, b, d, e, f, h])
    } else {
        select_18([a, c, f, e, g])
    }
}
/// n = 9, i = 4, a < g, b < e, b < h, c < f, c < i, d < f, d < g, f < h, g < i
fn select_695([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_693([b, a, c, g, f, e, h, i])
    } else {
        select_694([c, a, b, d, e, g, i, f])
    }
}
/// n = 8, i = 4, a < f, b < e, c < f, c < g, d < e, d < h, e < g, f < h
fn select_696([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_6([b, d, f, h])
    } else {
        select_6([a, c, e, g])
    }
}
/// n = 9, i = 4, a < h, a < i, b < f, b < g, c < e, d < f, d < g, d < i, e < g, e < i, f < h
fn select_697([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_696([b, c, d, a, e, f, i, h])
    } else {
        select_207([b, d, e, a, f, i, g])
    }
}
/// n = 7, i = 3, a < g, b < e, c < d, c < f, d < g, e < f
fn select_698([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_30([b, a, c, d, e, g])
    } else {
        select_53([c, e, a, d, f])
    }
}
/// n = 9, i = 4, a < f, a < h, b < e, c < g, c < i, d < f, d < g, d < i, e < i, g < h
fn select_699([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_697([a, c, b, d, e, g, i, h, f])
    } else {
        select_698([b, c, a, f, g, h, e])
    }
}
/// n = 11, i = 5, a < h, a < i, b < g, c < e, c < j, d < f, d < k, e < k, f < i, g < h, g < k, i < j
fn select_700([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_695([b, c, a, f, e, i, g, j, h])
    } else {
        select_699([a, c, d, g, e, h, f, i, k])
    }
}
/// n = 12, i = 6, a < h, a < l, b < f, c < e, c < i, d < g, d < h, e < k, f < i, f < j, g < j, g < k, k < l
fn select_701([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_692([a, b, d, c, e, g, f, i, j, k, l, h])
    } else {
        select_700([g, b, a, c, h, e, f, j, k, l, i])
    }
}
/// n = 8, i = 4, a < h, b < g, b < h, c < e, c < g, d < f, f < g, f < h
fn select_702([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < e {
        select_439([a, c, b, d, f, e, g, h])
    } else {
        select_134([a, d, e, b, f])
    }
}
/// n = 10, i = 5, a < f, b < g, c < g, d < e, d < f, e < i, e < j, g < h, g < i, g < j
fn select_703([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < h {
        select_235([a, e, b, c, f, g, h, i, j])
    } else {
        select_3([a, d, e])
    }
}
/// n = 11, i = 5, a < f, a < k, b < h, c < h, c < k, d < e, d < f, e < g, e < j, g < k, h < i, h < j
fn select_704([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_702([b, c, a, e, f, g, k, h])
    } else {
        select_703([a, b, c, d, e, f, h, i, g, j])
    }
}
/// n = 9, i = 4, a < i, b < d, b < i, c < e, c < h, d < g, e < f, e < i, g < h
fn select_705([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_255([a, g, c, e, f, h])
    } else {
        select_255([f, a, b, d, g, i])
    }
}
/// n = 10, i = 5, a < j, b < f, b < i, c < f, c < h, d < e, d < j, e < h, e < i, f < g, f < j
fn select_706([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_705([a, d, b, e, f, g, h, i, j])
    } else {
        select_705([a, d, c, e, f, g, i, h, j])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, b < h, c < e, d < f, e < h, e < i, f < g, f < i
fn select_707([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_153([a, b, d, f, h, g, i])
    } else {
        select_122([a, b, c, g, e, i])
    }
}
/// n = 9, i = 5, a < g, a < i, b < g, b < h, c < e, d < f, e < i, f < h, f < i
fn select_708([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_707([a, b, c, d, e, f, h, g, i])
    } else {
        select_8([c, a, d, g, f, h, e])
    }
}
/// n = 11, i = 6, a < g, b < h, b < i, c < f, d < e, d < g, e < h, e < j, f < i, f < j, i < k
fn select_709([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < g {
        select_706([a, b, f, d, e, i, k, j, h, g])
    } else {
        select_708([b, e, a, c, g, f, h, j, i])
    }
}
/// n = 12, i = 6, a < h, a < l, b < f, c < e, c < i, d < g, d < h, e < k, e < l, f < j, f < k, g < i, g < j, i < l
fn select_710([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_704([a, b, e, d, g, h, i, f, k, j, l])
    } else {
        select_709([a, c, b, d, g, f, h, i, e, j, k])
    }
}
/// n = 12, i = 6, a < j, b < i, b < l, c < h, c < l, d < f, e < g, e < h, f < i, f < k, g < j, g < k, j < l
fn select_711([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < a {
        select_701([c, d, b, e, a, f, g, h, i, k, j, l])
    } else {
        select_710([c, d, a, e, b, f, g, h, j, k, i, l])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < g, c < d, c < f, c < g
fn select_712([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_191([d, b, a, f, g])
    } else {
        select_49([a, b, c, d, e, g])
    }
}
/// n = 7, i = 3, a < f, b < d, b < g, c < e, c < f, e < g
fn select_713([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_73([b, c, e, f, d])
    } else {
        select_255([d, a, c, e, g, f])
    }
}
/// n = 8, i = 3, a < h, b < g, c < f, c < h, d < e, d < f, d < h, e < g
fn select_714([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_712([c, a, d, e, b, f, h])
    } else {
        select_713([c, b, d, a, e, f, g])
    }
}
/// n = 8, i = 4, b < h, c < g, d < f, e < f, f < g, f < h
fn select_715([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_373([a, d, e, b, c, f, h])
    } else {
        select_373([a, d, e, c, b, f, g])
    }
}
/// n = 9, i = 4, a < i, b < h, c < f, d < g, e < f, e < g, f < i, g < h
fn select_716([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_715([b, a, d, c, e, f, g, i])
    } else {
        select_715([a, b, c, d, e, g, f, h])
    }
}
/// n = 10, i = 4, a < i, a < j, b < h, b < i, c < g, d < g, e < f, f < h, f < i, g < j
fn select_717([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < c {
        select_714([a, d, b, f, c, h, g, i])
    } else {
        select_716([a, b, d, e, c, g, f, h, j])
    }
}
/// n = 11, i = 5, a < j, a < k, b < i, c < h, d < f, d < k, e < g, e < h, g < i, h < k, i < j
fn select_718([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_717([a, d, b, g, c, h, i, f, k, j])
    } else {
        select_674([a, c, b, d, e, f, g, i, h, j])
    }
}
/// n = 12, i = 6, a < k, b < j, b < l, c < i, c < l, d < f, e < g, e < i, f < j, g < h, g < k, k < l
fn select_719([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_711([a, b, c, d, e, f, g, i, j, k, h, l])
    } else {
        select_718([b, d, a, c, g, i, h, k, f, j, l])
    }
}
/// n = 12, i = 6, a < g, b < k, c < j, c < l, d < f, d < l, e < h, e < j, h < i, h < k, k < l
fn select_720([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_683([a, b, c, d, e, f, h, i, j, g, k, l])
    } else {
        select_719([b, d, c, a, e, g, h, i, j, f, k, l])
    }
}
/// n = 13, i = 6, b < g, c < j, c < l, d < f, d < l, e < h, e < j, h < i, h < k, k < l, l < m
fn select_721([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if k < a {
        select_657([b, c, d, h, i, f, j, g, k, a, l, m])
    } else {
        select_720([b, a, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, b < h, c < f, c < h, d < f, d < h, f < g
fn select_722([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_385([b, c, d, a, e, f, g, h])
    } else {
        select_26([c, d, e, a, f])
    }
}
/// n = 8, i = 4, a < h, b < g, b < h, c < e, c < h, d < f, d < h, f < g
fn select_723([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_722([b, c, a, d, e, f, g, h])
    } else {
        select_188([b, c, f, a, e, g])
    }
}
/// n = 11, i = 4, a < e, b < h, c < g, c < h, d < f, d < h, f < g, h < i, h < j, h < k
fn select_724([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_17([f, b, g, h, a, i, j, k])
    } else {
        select_723([b, c, a, d, e, f, g, h])
    }
}
/// n = 8, i = 3, a < f, a < h, b < e, b < f, b < g, c < d, c < e, d < g, d < h
fn select_725([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_50([b, d, e, a, h, g])
    } else {
        select_488([b, a, c, e, f, d, h])
    }
}
/// n = 9, i = 4, a < g, a < i, b < f, b < i, c < d, c < e, e < h, e < i, f < g, f < h
fn select_726([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_190([a, e, d, b, f, i])
    } else {
        select_725([a, f, c, e, d, g, h, i])
    }
}
/// n = 10, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g, f < h, f < i, f < j
fn select_727([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_16([a, g, h, i, j])
    } else {
        select_53([b, c, d, e, f])
    }
}
/// n = 9, i = 4, a < h, a < i, b < d, b < i, c < e, c < g, d < g, d < h, e < f, e < i
fn select_728([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_255([f, a, b, d, g, i])
    } else {
        select_46([a, c, g, h, e, f])
    }
}
/// n = 12, i = 4, a < h, a < i, b < e, b < i, c < d, c < g, c < i, d < f, e < g, e < h, i < j, i < k, i < l
fn select_729([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < d {
        select_727([e, a, c, g, h, i, d, j, k, l])
    } else {
        select_728([a, b, c, e, d, f, g, h, i])
    }
}
/// n = 12, i = 4, a < e, b < h, b < i, c < g, c < i, d < f, d < i, f < g, f < h, i < j, i < k, i < l
fn select_730([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_726([c, d, a, e, b, f, g, h, i])
    } else {
        select_729([c, d, b, a, f, e, h, g, i, j, k, l])
    }
}
/// n = 12, i = 4, a < f, b < h, b < i, c < e, c < i, d < g, d < i, g < h, i < j, i < k, i < l
fn select_731([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < g {
        select_724([a, b, d, c, f, e, g, i, j, k, l])
    } else {
        select_730([a, b, c, d, f, g, e, h, i, j, k, l])
    }
}
/// n = 9, i = 4, a < e, a < i, b < e, b < g, b < h, c < f, c < g, d < f, d < g, f < h, f < i
fn select_732([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_50([b, f, g, a, i, h])
    } else {
        select_42([b, c, d, a, e, g, f, i])
    }
}
/// n = 10, i = 5, a < h, a < j, b < f, b < g, c < f, c < g, d < e, d < j, e < h, e < i, g < i, g < j
fn select_733([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_732([a, e, b, c, h, g, f, i, j])
    } else {
        select_190([a, g, f, d, e, j])
    }
}
/// n = 13, i = 5, a < g, b < h, b < j, c < f, c < g, d < e, d < j, e < h, e < i, f < i, f < j, j < k, j < l, j < m
fn select_734([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < f {
        select_733([b, a, c, d, e, g, f, h, i, j])
    } else {
        select_729([b, d, f, a, e, g, i, h, j, k, l, m])
    }
}
/// n = 13, i = 5, a < h, b < i, b < j, c < e, c < j, d < f, d < h, e < i, f < g, f < j, j < k, j < l, j < m
fn select_735([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_724([a, b, c, f, h, g, e, j, k, l, m])
    } else {
        select_734([a, b, d, c, e, f, h, i, g, j, k, l, m])
    }
}
/// n = 13, i = 5, b < i, b < j, c < f, c < j, d < e, d < g, f < i, g < h, g < j, j < k, j < l, j < m
fn select_736([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < a {
        select_731([e, b, g, c, h, a, f, i, j, k, l, m])
    } else {
        select_735([a, b, c, d, f, g, h, e, i, j, k, l, m])
    }
}
/// n = 11, i = 4, c < h, d < g, d < h, e < f, e < h, f < g, h < i, h < j, h < k
fn select_737([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_724([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_724([b, c, d, e, a, f, g, h, i, j, k])
    }
}
/// n = 10, i = 5, a < h, a < j, b < g, b < h, c < f, c < j, d < e, d < g, f < h, f < i, g < i, g < j
fn select_738([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_44([a, b, f, d, e, h, g, i])
    } else {
        select_540([e, a, c, g, f, i, j])
    }
}
/// n = 13, i = 5, a < e, b < i, b < j, c < g, c < i, d < f, d < j, f < h, f < i, g < h, g < j, j < k, j < l, j < m
fn select_739([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < g {
        select_738([b, c, d, a, e, f, g, i, h, j])
    } else {
        select_729([b, d, g, a, f, e, h, i, j, k, l, m])
    }
}
/// n = 7, i = 3, a < d, b < e, c < e, e < f, e < g
fn select_740([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_14([a, f, g])
    } else {
        select_6([d, b, c, e])
    }
}
/// n = 8, i = 5, a < g, b < g, b < h, c < g, c < h, d < f, e < f, f < h
fn select_741([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_25([b, c, d, e, f, g])
    } else {
        select_6([a, b, c, h])
    }
}
/// n = 9, i = 5, a < g, b < g, b < h, c < g, c < h, d < f, d < h, e < f, e < h, h < i
fn select_742([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_740([a, d, e, g, h, f, i])
    } else {
        select_741([a, b, c, d, e, f, g, h])
    }
}
/// n = 11, i = 6, a < i, a < k, b < h, b < i, c < g, c < h, d < g, d < h, e < f, e < k, f < i, f < j, h < j, h < k
fn select_743([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_742([a, b, f, c, d, g, i, h, j])
    } else {
        select_540([g, a, e, h, f, j, k])
    }
}
/// n = 14, i = 6, a < g, b < i, b < k, c < h, c < i, d < g, d < h, e < f, e < k, f < i, f < j, h < j, h < k, k < l, k < m, k < n
fn select_744([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < h {
        select_743([b, c, a, d, e, f, g, h, i, j, k])
    } else {
        select_729([b, e, h, a, f, g, j, i, k, l, m, n])
    }
}
/// n = 14, i = 6, b < i, b < k, c < h, c < i, d < g, d < k, e < f, e < h, g < i, g < j, h < j, h < k, k < l, k < m, k < n
fn select_745([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < a {
        select_739([f, b, c, d, a, g, h, j, i, k, l, m, n])
    } else {
        select_744([a, b, c, e, d, g, f, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 6, b < j, b < k, c < g, c < k, d < h, d < j, e < f, e < h, g < j, h < i, h < k, k < l, k < m, k < n
fn select_746([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_737([a, f, b, c, h, i, g, k, l, m, n])
    } else {
        select_745([a, b, d, c, e, f, g, h, j, i, k, l, m, n])
    }
}
/// n = 14, i = 6, b < h, b < k, c < g, c < k, d < h, d < i, e < f, e < i, i < j, i < k, k < l, k < m, k < n
fn select_747([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_736([a, c, b, e, f, h, i, j, g, k, l, m, n])
    } else {
        select_746([a, b, c, d, e, f, g, i, j, h, k, l, m, n])
    }
}
/// n = 14, i = 6, b < h, b < j, c < h, c < i, d < g, d < j, e < f, e < i, i < k, i < m, i < n, j < l, j < m, j < n
fn select_748([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < j {
        select_747([a, b, d, c, e, f, g, h, i, k, j, l, m, n])
    } else {
        select_747([a, c, e, b, d, g, f, h, j, l, i, k, m, n])
    }
}
/// n = 8, i = 3, a < g, a < h, b < d, b < h, c < e, c < h, d < g, e < f
fn select_749([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_73([b, e, a, f, d])
    } else {
        select_275([c, b, a, d, e, f, g, h])
    }
}
/// n = 8, i = 3, a < e, a < h, b < d, b < h, c < f, c < h, f < g
fn select_750([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_749([a, b, c, d, f, g, e, h])
    } else {
        select_749([b, a, c, e, f, g, d, h])
    }
}
/// n = 8, i = 4, a < f, b < e, c < g, d < f, d < g, g < h
fn select_751([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_279([b, a, c, f, g, h, e])
    } else {
        select_121([c, a, d, b, e, f, g])
    }
}
/// n = 9, i = 4, a < h, b < g, b < i, c < e, c < i, d < f, d < g, f < h, f < i
fn select_752([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_750([b, c, f, e, g, a, h, i])
    } else {
        select_751([b, c, a, d, e, g, f, h])
    }
}
/// n = 8, i = 4, a < g, b < e, c < g, c < h, d < f, f < h
fn select_753([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_652([b, d, c, a, e, g, f, h])
    } else {
        select_346([b, c, f, e, a, h, g])
    }
}
/// n = 9, i = 4, a < f, b < h, b < i, c < e, c < i, d < g, d < h, g < i
fn select_754([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_752([a, b, c, d, e, g, h, f, i])
    } else {
        select_753([b, c, d, a, e, f, h, g])
    }
}
/// n = 8, i = 3, a < f, b < f, b < g, b < h, c < d, d < e, d < g, d < h
fn select_755([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_19([e, b, g, h, a])
    } else {
        select_20([a, b, c, d, f, g, h, e])
    }
}
/// n = 11, i = 5, a < h, b < h, b < i, c < f, c < i, c < k, d < e, d < h, e < i, f < g, h < j, h < k
fn select_756([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_723([a, b, f, d, g, e, i, h])
    } else {
        select_755([e, c, a, h, j, i, f, k])
    }
}
/// n = 9, i = 4, a < e, a < h, b < e, b < g, c < d, c < i, d < g, d < h, e < f, e < i
fn select_757([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_9([f, c, d, i])
    } else {
        select_53([a, b, g, h, e])
    }
}
/// n = 11, i = 4, a < e, a < h, b < e, b < f, b < i, c < d, c < k, d < f, d < h, d < i, e < g, e < k, i < j
fn select_758([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < e {
        select_46([a, b, f, h, i, j])
    } else {
        select_757([a, b, c, d, e, g, f, h, k])
    }
}
/// n = 11, i = 4, a < f, a < i, b < f, b < h, c < d, c < k, d < e, d < h, d < i, e < g, f < j, f < k
fn select_759([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_46([a, e, g, i, b, h])
    } else {
        select_758([a, b, c, d, f, h, j, i, e, g, k])
    }
}
/// n = 9, i = 5, a < g, b < g, c < g, d < e, d < i, e < f, g < h, g < i
fn select_760([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_53([d, g, h, e, i])
    } else {
        select_127([a, b, c, f])
    }
}
/// n = 11, i = 5, a < g, b < g, b < i, c < g, c < i, d < e, d < k, e < f, e < i, g < h, g < k, i < j
fn select_761([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < g {
        select_115([a, f, b, c, i, j])
    } else {
        select_760([a, b, c, d, e, f, g, h, k])
    }
}
/// n = 12, i = 5, a < h, a < k, b < h, b < j, c < g, c < h, d < e, d < l, e < f, e < g, g < j, g < k, h < i, h < l
fn select_762([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_255([f, b, g, a, k, j])
    } else {
        select_761([b, a, c, d, e, f, h, i, g, k, l])
    }
}
/// n = 12, i = 5, a < h, a < j, b < h, b < i, c < g, c < h, d < e, d < g, d < l, e < f, g < i, g < j, h < k, h < l
fn select_763([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_759([a, b, d, g, e, h, f, i, j, k, l])
    } else {
        select_762([a, b, c, d, e, f, g, h, k, i, j, l])
    }
}
/// n = 12, i = 5, a < i, a < j, b < e, b < i, c < h, c < i, d < f, d < h, d < l, f < g, h < j, i < k, i < l
fn select_764([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < h {
        select_756([a, c, d, b, e, f, g, i, h, k, l])
    } else {
        select_763([a, b, c, d, f, g, h, i, e, j, k, l])
    }
}
/// n = 12, i = 5, a < j, b < i, b < j, c < e, c < j, d < f, d < g, d < l, f < i, g < h, j < k, j < l
fn select_765([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_754([g, b, c, f, e, h, a, i, j])
    } else {
        select_764([b, c, a, d, e, g, h, f, j, i, k, l])
    }
}
/// n = 9, i = 2, a < c, b < d, c < i, d < e, d < f, d < g, d < h
fn select_766([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_16([a, e, f, g, h])
    } else {
        select_152([b, c, i, d])
    }
}
/// n = 9, i = 2, a < c, a < d, b < e, e < f, e < g, e < h, e < i
fn select_767([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_16([a, f, g, h, i])
    } else {
        select_198([c, d, b, e])
    }
}
/// n = 9, i = 2, b < d, c < e, e < f, e < g, e < h, e < i
fn select_768([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_766([a, c, b, e, f, g, h, i, d])
    } else {
        select_767([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 11, i = 4, a < d, b < c, b < j, c < e, c < h, c < k, d < f, d < g, d < j, d < k, h < i
fn select_769([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < c {
        select_65([f, g, b, j, c, e, h, k])
    } else {
        select_768([e, h, a, i, d, f, g, j, k])
    }
}
/// n = 9, i = 4, a < f, b < f, b < h, c < d, d < e, d < h, e < g, f < i
fn select_770([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_73([b, e, a, g, h])
    } else {
        select_104([b, a, c, d, f, i, h, e, g])
    }
}
/// n = 9, i = 4, a < g, b < g, b < h, c < d, d < e, d < f, d < h, g < i
fn select_771([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_770([a, b, c, d, e, g, f, h, i])
    } else {
        select_770([a, b, c, d, f, g, e, h, i])
    }
}
/// n = 6, i = 2, a < f, b < e, b < f, c < d, c < e
fn select_772([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_4([c, a, d, b, f])
    } else {
        select_50([c, b, d, a, f, e])
    }
}
/// n = 8, i = 3, a < f, a < h, b < d, b < e, b < h, c < e, c < f, c < h, f < g
fn select_773([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_601([a, b, e, f, d, g, h])
    } else {
        select_275([a, b, c, d, f, g, e, h])
    }
}
/// n = 10, i = 4, a < f, a < g, b < d, b < j, c < f, c < g, c < i, d < e, d < i, f < j, g < h
fn select_774([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_772([f, b, g, h, d, j])
    } else {
        select_773([a, d, c, e, i, f, j, g])
    }
}
/// n = 11, i = 4, a < g, a < h, b < g, b < h, b < i, c < d, c < k, d < e, d < f, d < i, g < k, h < j
fn select_775([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_771([a, b, c, d, e, f, h, i, j])
    } else {
        select_774([a, c, b, d, f, g, h, j, i, k])
    }
}
/// n = 12, i = 4, a < i, b < e, b < l, c < d, c < i, c < j, d < h, d < k, e < f, e < g, e < j, e < k, i < l
fn select_776([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < a {
        select_769([b, c, d, e, h, f, g, a, i, j, k])
    } else {
        select_775([a, c, b, e, f, g, i, d, j, h, l])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < e, b < i, d < h, e < f, e < g
fn select_777([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_62([d, e, a, f, g, h])
    } else {
        select_305([a, b, c, d, h, e, i])
    }
}
/// n = 11, i = 3, a < d, a < g, a < k, b < c, b < g, b < j, c < e, c < k, d < f, d < j, g < h, g < i
fn select_778([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_161([c, a, e, g, h, i, d, k])
    } else {
        select_161([d, b, f, g, h, i, c, j])
    }
}
/// n = 11, i = 3, a < d, a < g, a < h, b < c, b < g, b < h, b < k, c < f, d < e, d < k, h < i, h < j
fn select_779([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < c {
        select_777([d, b, e, g, h, i, j, c, k])
    } else {
        select_778([a, b, c, d, f, e, h, i, j, k, g])
    }
}
/// n = 11, i = 3, a < e, a < g, a < h, b < c, b < d, b < g, b < h, b < k, e < f, e < k, h < i, h < j
fn select_780([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_779([a, b, c, e, f, d, g, h, i, j, k])
    } else {
        select_779([a, b, d, e, f, c, g, h, i, j, k])
    }
}
/// n = 11, i = 4, a < e, b < g, b < k, c < f, c < g, d < f, d < g, f < k, g < h, g < i, g < j
fn select_781([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_240([b, a, c, d, e, f, k])
    } else {
        select_68([a, f, b, g, h, i, j, k])
    }
}
/// n = 11, i = 4, a < e, b < g, b < h, c < f, c < h, d < f, d < g, d < h, h < i, h < j, h < k
fn select_782([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_781([a, b, c, d, e, f, h, i, j, k, g])
    } else {
        select_781([a, c, b, d, e, g, h, i, j, k, f])
    }
}
/// n = 13, i = 5, a < g, a < i, b < g, b < h, c < d, c < e, c < l, d < h, d < i, e < f, e < m, g < j, g < k, g < l, g < m
fn select_783([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < d {
        select_780([c, g, j, k, e, f, l, d, h, i, m])
    } else {
        select_782([e, a, b, d, f, h, i, g, j, k, l])
    }
}
/// n = 13, i = 5, a < h, a < k, b < e, b < f, b < l, c < d, c < h, e < k, f < g, f < m, h < i, h < j, h < l, h < m
fn select_784([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < e {
        select_776([d, a, b, f, h, i, j, g, e, l, m, k])
    } else {
        select_783([a, c, b, e, f, g, h, d, k, i, j, l, m])
    }
}
/// n = 13, i = 5, b < i, b < j, c < e, c < i, d < f, d < g, d < l, f < j, g < h, g < m, i < k, i < l, i < m
fn select_785([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < i {
        select_765([a, b, c, d, e, f, g, h, j, i, k, l])
    } else {
        select_784([b, d, c, e, f, g, h, i, a, k, j, l, m])
    }
}
/// n = 9, i = 2, a < d, b < c, b < e, e < f, e < g, e < h, e < i
fn select_786([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_64([a, b, c, d, e, f, h, i])
    } else {
        select_64([a, b, c, d, e, g, h, i])
    }
}
/// n = 8, i = 3, a < g, a < h, b < f, b < g, b < h, c < d, c < f, c < h, d < e
fn select_787([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < b {
        select_198([a, e, b, f])
    } else {
        select_323([a, c, f, g, d, e, h])
    }
}
/// n = 11, i = 3, a < g, b < f, b < g, b < h, c < d, c < f, c < h, d < e, h < i, h < j, h < k
fn select_788([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_786([d, b, f, e, h, a, i, j, k])
    } else {
        select_787([a, b, c, d, e, f, g, h])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, c < d, c < e, d < g
fn select_789([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_4([b, d, e, a, g])
    } else {
        select_488([b, a, c, e, f, d, g])
    }
}
/// n = 11, i = 3, a < d, a < f, b < e, b < g, b < k, c < e, c < f, c < g, d < k, g < h, g < i, g < j
fn select_790([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_64([a, b, e, f, g, h, i, j])
    } else {
        select_789([b, c, a, d, f, e, k])
    }
}
/// n = 11, i = 3, a < g, b < f, c < e, c < g, c < h, d < e, d < f, d < h, h < i, h < j, h < k
fn select_791([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_790([a, d, c, b, e, g, h, i, j, k, f])
    } else {
        select_790([b, c, d, a, e, f, h, i, j, k, g])
    }
}
/// n = 11, i = 3, b < g, c < f, c < g, c < h, d < e, d < f, d < h, h < i, h < j, h < k
fn select_792([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_788([b, c, d, e, a, f, g, h, i, j, k])
    } else {
        select_791([a, b, d, c, f, g, e, h, i, j, k])
    }
}
/// n = 8, i = 3, a < e, b < f, b < h, c < d, c < f, e < h, f < g
fn select_793([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_278([a, d, b, f, g])
    } else {
        select_122([b, c, a, d, e, h])
    }
}
/// n = 11, i = 3, a < d, a < e, b < f, c < e, c < f, e < k, f < g, f < h, f < i, f < j
fn select_794([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_16([a, g, h, i, j])
    } else {
        select_255([d, b, c, e, k, f])
    }
}
/// n = 11, i = 3, a < e, a < f, b < d, b < g, c < f, c < g, f < k, g < h, g < i, g < j
fn select_795([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < g {
        select_793([b, c, a, e, d, f, k, g])
    } else {
        select_794([a, b, c, e, f, g, d, h, i, j, k])
    }
}
/// n = 12, i = 4, a < d, a < i, b < f, b < i, c < e, c < i, e < g, f < h, i < j, i < k, i < l
fn select_796([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_795([e, a, b, d, g, f, i, j, k, l, h])
    } else {
        select_795([f, a, c, d, h, e, i, j, k, l, g])
    }
}
/// n = 12, i = 4, a < f, a < i, b < g, b < i, c < d, c < e, c < i, g < h, i < j, i < k, i < l
fn select_797([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_796([a, b, c, f, d, g, e, h, i, j, k, l])
    } else {
        select_796([a, b, c, f, e, g, d, h, i, j, k, l])
    }
}
/// n = 13, i = 5, a < d, a < i, b < f, b < j, b < l, c < e, c < i, e < j, f < g, f < h, f < m, i < k, i < l, i < m
fn select_798([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_792([d, e, b, i, k, l, j, f, g, h, m])
    } else {
        select_797([a, c, f, g, h, d, e, j, i, k, l, m])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < f
fn select_799([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < a {
        select_73([c, b, a, e, d])
    } else {
        select_50([c, a, d, b, e, f])
    }
}
/// n = 8, i = 3, a < g, a < h, b < e, b < f, c < d, c < f, c < g, e < h
fn select_800([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_799([d, a, b, f, g, e])
    } else {
        select_725([a, c, b, e, f, g, d, h])
    }
}
/// n = 8, i = 3, a < e, a < f, b < f, b < g, b < h, c < d, c < e, d < g, d < h
fn select_801([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_19([a, b, g, h, e])
    } else {
        select_73([b, c, d, e, f])
    }
}
/// n = 8, i = 3, a < f, a < g, b < e, b < g, c < d, c < f, c < h, e < h
fn select_802([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_789([b, a, c, d, f, g, e])
    } else {
        select_801([a, c, b, e, g, f, d, h])
    }
}
/// n = 8, i = 3, a < h, b < g, c < f, c < h, d < e, d < f, d < g
fn select_803([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_800([c, b, d, e, a, g, f, h])
    } else {
        select_802([c, a, d, e, b, f, h, g])
    }
}
/// n = 8, i = 3, c < e, c < f, d < e, d < f, f < g, f < h
fn select_804([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_223([b, e, f, a, g, h])
    } else {
        select_359([b, a, c, d, e, f])
    }
}
/// n = 9, i = 3, a < f, b < e, b < f, b < g, c < d, c < e, c < g, g < h, g < i
fn select_805([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_613([a, b, c, d, f, e, g])
    } else {
        select_92([a, b, c, d, f, g, e, h, i])
    }
}
/// n = 10, i = 4, a < i, b < f, b < h, c < d, c < e, c < g, e < h, e < j, f < g, f < i, f < j
fn select_806([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_804([a, d, b, e, h, f, g, j])
    } else {
        select_805([a, f, c, d, g, i, e, h, j])
    }
}
/// n = 10, i = 4, a < i, b < e, b < f, b < h, c < d, c < g, f < j, g < h, g < i, g < j
fn select_807([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_803([a, d, g, b, e, h, f, i])
    } else {
        select_806([a, c, b, e, f, g, h, d, i, j])
    }
}
/// n = 9, i = 4, a < f, b < f, b < g, c < d, c < i, d < e, d < g, f < h, f < i
fn select_808([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_426([a, c, d, f, h, g, i])
    } else {
        select_7([a, e, b, g])
    }
}
/// n = 11, i = 4, a < h, a < i, a < k, b < e, b < h, c < f, c < h, d < e, d < f, e < k, f < g, f < i, h < j
fn select_809([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_178([g, a, b, e, i])
    } else {
        select_276([a, c, d, e, f, h, g, j, k])
    }
}
/// n = 12, i = 5, a < f, a < k, a < l, b < h, b < i, c < f, c < i, d < e, d < i, e < h, f < g, h < k, i < j, i < l
fn select_810([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_808([b, d, a, f, g, i, e, j, l])
    } else {
        select_809([a, e, c, b, h, i, j, f, l, g, k])
    }
}
/// n = 10, i = 5, a < i, a < j, b < f, c < f, c < g, d < e, d < j, e < g, e < i, f < h, f < j
fn select_811([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_599([a, b, c, g, i, f, h])
    } else {
        select_255([h, a, d, e, g, j])
    }
}
/// n = 10, i = 4, a < e, a < f, b < g, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_812([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_134([b, c, d, e, f])
    } else {
        select_32([a, b, d, f, g, h, i, j])
    }
}
/// n = 11, i = 5, a < h, a < i, b < f, c < f, c < i, d < e, d < i, e < h, e < j, e < k, f < g, f < j, f < k
fn select_813([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_658([a, d, f, g, i, e, j, k])
    } else {
        select_812([a, b, c, e, h, i, f, g, j, k])
    }
}
/// n = 13, i = 6, a < h, a < k, b < g, b < h, c < f, c < k, d < e, d < h, e < i, e < k, f < g, f < i, f < m, g < j, h < l, h < m
fn select_814([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_811([a, b, f, d, e, g, i, j, k, h])
    } else {
        select_813([e, b, a, c, f, h, l, i, k, g, m])
    }
}
/// n = 13, i = 6, a < g, a < i, b < h, b < i, c < e, c < i, d < f, d < g, e < k, f < h, f < k, f < m, h < j, i < l, i < m
fn select_815([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_810([f, c, b, a, g, h, j, e, i, l, k, m])
    } else {
        select_814([a, b, d, c, e, f, h, i, k, j, g, l, m])
    }
}
/// n = 13, i = 6, a < j, b < g, b < h, c < e, c < h, d < f, d < g, e < k, f < j, f < k, f < m, h < i, h < m, j < l
fn select_816([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_807([e, h, d, g, i, a, f, m, k, j])
    } else {
        select_815([b, a, c, d, e, f, g, j, h, l, k, i, m])
    }
}
/// n = 14, i = 6, b < g, b < i, c < e, c < i, d < f, d < g, e < l, f < h, f < l, f < m, h < j, h < n, i < k, i < m, i < n
fn select_817([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < a {
        select_798([b, f, c, g, e, h, a, j, i, l, k, m, n])
    } else {
        select_816([a, b, c, d, e, f, g, i, k, h, l, j, m])
    }
}
/// n = 14, i = 6, b < h, b < j, c < e, c < j, d < f, d < h, f < g, f < i, f < m, i < k, i < n, j < l, j < m, j < n
fn select_818([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < e {
        select_785([a, c, b, f, h, g, i, k, j, e, l, m, n])
    } else {
        select_817([a, b, c, d, e, f, h, i, j, k, l, g, m, n])
    }
}
/// n = 14, i = 6, b < g, b < n, c < j, c < k, d < h, d < j, e < f, e < k, h < i, h < m, h < n, k < l, k < m, k < n
fn select_819([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < h {
        select_748([a, c, d, e, b, g, f, j, h, k, i, l, m, n])
    } else {
        select_818([a, c, e, d, f, h, i, j, b, k, g, l, m, n])
    }
}
/// n = 14, i = 6, b < g, b < n, c < k, c < l, d < f, d < l, e < h, e < k, h < i, h < j, h < n, l < m, l < n
fn select_820([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < l {
        select_721([a, b, c, d, e, f, g, h, j, k, i, l, m])
    } else {
        select_819([a, b, c, e, d, f, g, h, j, k, l, m, i, n])
    }
}
/// n = 7, i = 2, c < d, c < e, e < f, e < g
fn select_821([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_125([a, c, d, b, e, f, g])
    } else {
        select_125([b, c, d, a, e, f, g])
    }
}
/// n = 8, i = 3, b < e, b < f, c < d, c < f, f < g, f < h
fn select_822([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_245([a, d, b, f, g, h, e])
    } else {
        select_245([a, e, c, f, g, h, d])
    }
}
/// n = 8, i = 3, b < e, b < h, c < d, c < f, f < g, f < h
fn select_823([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < b {
        select_821([a, d, f, g, b, e, h])
    } else {
        select_822([a, b, c, d, e, f, g, h])
    }
}
/// n = 8, i = 3, b < g, b < h, c < f, c < h, d < e, d < h
fn select_824([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < d {
        select_823([a, c, b, g, f, d, e, h])
    } else {
        select_823([a, c, d, e, f, b, g, h])
    }
}
/// n = 10, i = 4, a < e, a < f, a < j, b < f, b < g, c < d, c < g, g < h, g < i, g < j
fn select_825([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_87([d, h, i, a, e, f, j])
    } else {
        select_69([e, b, c, d, f, g, h, i, j])
    }
}
/// n = 11, i = 4, a < h, a < i, b < e, b < i, c < d, c < f, c < h, f < g, i < j, i < k
fn select_826([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < i {
        select_824([d, a, b, f, g, e, h, i])
    } else {
        select_825([c, a, b, e, d, h, i, j, k, f])
    }
}
/// n = 8, i = 3, a < e, a < h, b < c, b < d, b < g, d < h, e < f, e < g
fn select_827([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_194([c, a, d, e, h])
    } else {
        select_54([b, e, f, c, d, g])
    }
}
/// n = 9, i = 3, a < e, a < g, b < c, b < d, b < g, b < i, e < f, e < i, g < h
fn select_828([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_110([e, b, d, f, g, c, h, i])
    } else {
        select_827([a, b, d, c, e, f, i, g])
    }
}
/// n = 9, i = 3, a < c, a < d, a < i, b < e, b < f, e < h, f < g, f < i
fn select_829([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_125([f, e, h, g, a, c, d])
    } else {
        select_828([b, a, c, d, f, g, e, h, i])
    }
}
/// n = 9, i = 3, a < e, a < f, a < i, b < c, b < d, b < g, g < h, g < i
fn select_830([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_829([a, b, e, f, c, g, h, d, i])
    } else {
        select_829([a, b, e, f, d, g, h, c, i])
    }
}
/// n = 9, i = 2, b < i, c < d, c < i, d < e, d < f, d < g, d < h
fn select_831([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_68([a, b, c, d, e, g, h, i])
    } else {
        select_68([a, b, c, d, f, g, h, i])
    }
}
/// n = 10, i = 3, b < e, b < f, c < d, c < f, f < g, f < h, f < i, f < j
fn select_832([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_831([a, d, b, f, g, h, i, j, e])
    } else {
        select_831([a, e, c, f, g, h, i, j, d])
    }
}
/// n = 9, i = 2, a < i, b < h, c < d, c < h, c < i, d < e, d < f, d < g
fn select_833([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_63([a, c, b, d, e, f, g, i])
    } else {
        select_63([b, c, a, d, e, f, g, h])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < i, e < f, e < g, e < h
fn select_834([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_635([b, d, e, f, g, h, a, i])
    } else {
        select_833([a, b, c, e, f, g, h, i, d])
    }
}
/// n = 10, i = 3, a < e, a < f, a < g, b < c, b < d, b < g, g < h, g < i, g < j
fn select_835([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < e {
        select_834([d, c, a, f, g, h, i, j, e])
    } else {
        select_834([f, e, b, d, g, h, i, j, c])
    }
}
/// n = 11, i = 4, a < e, a < i, b < c, b < d, b < i, d < f, d < j, d < k, e < g, e < h, e < j, e < k
fn select_836([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_832([c, a, d, f, i, e, g, h, j, k])
    } else {
        select_835([b, e, g, h, c, i, d, f, j, k])
    }
}
/// n = 12, i = 4, a < c, a < d, a < i, b < e, b < i, d < h, d < l, e < f, e < g, e < j, e < l, j < k
fn select_837([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < d {
        select_830([a, e, f, g, c, i, j, k, d])
    } else {
        select_836([b, a, c, d, e, h, f, g, i, j, l])
    }
}
/// n = 12, i = 4, a < e, b < g, b < k, c < d, c < f, c < k, f < h, f < l, g < i, g < j, g < l
fn select_838([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < g {
        select_826([b, a, c, d, e, f, h, k, g, i, j])
    } else {
        select_837([c, b, d, f, g, i, j, h, k, a, e, l])
    }
}
/// n = 11, i = 5, a < g, a < j, b < f, b < g, c < d, c < f, d < e, d < j, d < k, g < h, g < i, g < k
fn select_839([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_65([h, i, c, f, d, e, j, k])
    } else {
        select_69([e, a, b, f, j, g, h, i, k])
    }
}
/// n = 12, i = 5, a < h, a < j, b < f, b < h, c < d, c < f, d < e, d < g, d < j, g < i, h < k, h < l
fn select_840([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_824([e, a, b, g, i, f, j, h])
    } else {
        select_839([a, b, c, d, e, f, h, k, l, j, g])
    }
}
/// n = 10, i = 4, a < e, a < i, b < c, b < d, c < h, d < i, d < j, e < f, e < g, e < j
fn select_841([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < e {
        select_658([d, a, c, h, i, e, f, g])
    } else {
        select_828([b, e, f, g, d, i, c, h, j])
    }
}
/// n = 10, i = 4, a < f, a < i, b < c, b < d, b < e, e < i, e < j, f < g, f < h, f < j
fn select_842([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_841([a, b, c, e, f, g, h, d, i, j])
    } else {
        select_841([a, b, d, e, f, g, h, c, i, j])
    }
}
/// n = 13, i = 5, a < c, a < l, b < d, b < k, c < g, c < i, c < k, d < e, d < f, d < h, d < m, h < l, i < j, i < m
fn select_843([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_842([a, d, e, f, h, c, g, k, l, i])
    } else {
        select_836([b, c, g, i, d, j, e, f, k, h, m])
    }
}
/// n = 13, i = 5, a < i, b < d, b < l, c < e, c < i, d < g, d < h, d < m, e < f, e < j, e < l, j < k, j < m
fn select_844([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < d {
        select_840([b, a, c, e, f, i, j, d, k, l, g, h])
    } else {
        select_843([c, b, e, d, g, h, f, a, j, k, l, i, m])
    }
}
/// n = 13, i = 5, b < e, b < l, c < d, c < f, e < h, e < i, e < m, f < g, f < j, f < l, j < k, j < m
fn select_845([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < a {
        select_838([d, b, f, g, a, j, e, k, h, i, l, m])
    } else {
        select_844([a, b, c, e, f, g, h, i, d, j, k, l, m])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < h, c < d, c < f, c < h, f < g
fn select_846([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_50([a, c, e, f, g, h])
    } else {
        select_255([d, a, b, f, g, h])
    }
}
/// n = 8, i = 3, a < g, a < h, b < d, b < h, c < e, c < g, c < h, e < f
fn select_847([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_46([a, e, f, g, b, d])
    } else {
        select_846([a, c, b, d, g, e, f, h])
    }
}
/// n = 11, i = 3, a < d, b < g, b < h, c < e, c < g, c < h, e < f, h < i, h < j, h < k
fn select_848([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_786([e, b, g, f, h, a, i, j, k])
    } else {
        select_847([b, a, c, d, e, f, g, h])
    }
}
/// n = 11, i = 3, a < f, b < g, b < h, c < d, c < e, c < g, c < h, h < i, h < j, h < k
fn select_849([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_848([a, b, c, f, d, e, g, h, i, j, k])
    } else {
        select_848([a, b, c, f, e, d, g, h, i, j, k])
    }
}
/// n = 11, i = 3, c < g, c < h, d < e, d < f, d < g, d < h, h < i, h < j, h < k
fn select_850([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_849([a, c, d, e, f, b, g, h, i, j, k])
    } else {
        select_849([b, c, d, e, f, a, g, h, i, j, k])
    }
}
/// n = 8, i = 1, b < g, b < h, c < d, c < e, c < f
fn select_851([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_62([a, c, d, e, f, b])
    } else {
        select_220([b, c, d, e, f, a, g, h])
    }
}
/// n = 9, i = 1, b < h, b < i, c < d, c < e, c < f, c < g
fn select_852([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_851([a, b, c, d, f, g, h, i])
    } else {
        select_851([a, b, c, e, f, g, h, i])
    }
}
/// n = 10, i = 2, c < d, c < e, d < j, e < f, e < g, e < h, e < i
fn select_853([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_852([b, d, e, f, g, h, i, a, j])
    } else {
        select_831([b, a, c, e, f, g, h, i, d])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, c < d, c < h, d < g, e < f, e < h
fn select_854([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_153([a, d, b, e, g, f, h])
    } else {
        select_18([a, f, c, d, h])
    }
}
/// n = 10, i = 4, a < g, a < h, a < j, b < e, b < h, c < d, c < h, c < j, d < g, e < f, e < j, h < i
fn select_855([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_594([d, a, b, e, g, h, i, j])
    } else {
        select_854([a, b, c, d, e, f, g, j])
    }
}
/// n = 9, i = 3, a < d, a < g, b < c, b < g, b < i, c < f, d < e, d < i, g < h
fn select_856([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_109([d, b, e, g, c, h, i])
    } else {
        select_10([a, b, c, d, f, e, i, g])
    }
}
/// n = 11, i = 5, a < f, b < e, b < i, c < d, c < i, c < k, d < h, e < g, e < k, f < h, f < i, f < k, i < j
fn select_857([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_528([g, a, c, d, f, i, k])
    } else {
        select_856([b, c, d, e, g, h, i, j, k])
    }
}
/// n = 11, i = 5, a < d, a < i, a < k, b < f, c < e, c < i, e < h, e < k, f < g, f < i, f < k, i < j
fn select_858([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_855([a, c, f, g, e, h, d, i, j, k])
    } else {
        select_857([b, c, a, d, e, f, h, g, i, j, k])
    }
}
/// n = 14, i = 5, a < d, a < i, a < j, b < f, c < e, c < i, c < j, e < h, f < g, f < i, f < j, i < n, j < k, j < l, j < m
fn select_859([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < e {
        select_853([d, g, c, i, j, e, k, l, m, n])
    } else {
        select_858([a, b, c, d, e, f, g, h, i, n, j])
    }
}
/// n = 14, i = 5, a < f, a < i, a < j, b < g, c < d, c < e, c < i, c < j, g < h, g < i, g < j, i < n, j < k, j < l, j < m
fn select_860([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < e {
        select_859([a, b, c, f, d, g, h, e, i, j, k, l, m, n])
    } else {
        select_859([a, b, c, f, e, g, h, d, i, j, k, l, m, n])
    }
}
/// n = 15, i = 6, a < e, a < m, b < d, b < j, b < n, c < f, e < h, e < i, e < n, f < g, f < j, f < n, j < k, j < l, j < m, n < o
fn select_861([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < e {
        select_850([d, g, a, j, k, l, m, e, h, i, n])
    } else {
        select_860([b, c, e, h, i, d, f, g, n, j, k, l, m, o])
    }
}
/// n = 9, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, d < i, f < i, g < h
fn select_862([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_44([a, b, c, d, e, f, g, h])
    } else {
        select_323([f, d, e, a, g, h, i])
    }
}
/// n = 9, i = 4, b < h, c < f, c < h, d < e, d < g, d < h, f < g, h < i
fn select_863([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_862([a, b, c, d, e, f, h, i, g])
    } else {
        select_484([a, f, d, e, b, g, h])
    }
}
/// n = 8, i = 3, a < h, b < e, c < f, c < h, d < e, d < f, d < h, f < g
fn select_864([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_191([a, c, d, e, f])
    } else {
        select_637([a, b, d, f, g, e, h])
    }
}
/// n = 11, i = 4, a < h, b < f, c < f, c < i, d < e, d < k, e < h, e < i, f < g, f < k, i < j
fn select_865([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_144([a, d, f, g, e, k])
    } else {
        select_864([b, a, c, e, h, i, j, f])
    }
}
/// n = 13, i = 5, a < h, b < h, b < k, c < f, c < m, d < e, d < h, d < k, e < i, f < g, f < i, f < k, h < j, h < m, k < l
fn select_866([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_863([a, b, d, f, g, e, i, k, l])
    } else {
        select_865([e, a, b, c, f, h, j, i, k, l, m])
    }
}
/// n = 8, i = 4, b < f, c < g, d < g, e < f, e < g, g < h
fn select_867([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_497([a, b, c, d, e, g, h])
    } else {
        select_362([a, c, d, b, e, f])
    }
}
/// n = 6, i = 1, d < e, d < f
fn select_868([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_108([c, a, d, e, f, b])
    } else {
        select_108([c, b, d, e, f, a])
    }
}
/// n = 8, i = 3, b < d, b < h, c < e, e < f, e < g, e < h
fn select_869([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_868([a, f, g, b, d, h])
    } else {
        select_79([a, d, c, e, f, g, h])
    }
}
/// n = 9, i = 4, c < f, c < i, d < g, e < f, e < g, g < h, g < i
fn select_870([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_867([b, c, a, d, e, f, g, h])
    } else {
        select_869([b, c, d, f, g, a, h, i])
    }
}
/// n = 8, i = 2, b < e, b < h, c < d, c < h, d < f, e < g
fn select_871([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_108([a, e, c, d, h, g])
    } else {
        select_108([a, d, b, e, h, f])
    }
}
/// n = 8, i = 2, b < f, b < h, c < d, c < e, c < h, f < g
fn select_872([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_871([a, b, c, d, f, e, g, h])
    } else {
        select_871([a, b, c, e, f, d, g, h])
    }
}
/// n = 8, i = 3, b < f, c < e, d < e, d < f, e < g, f < h
fn select_873([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_255([a, c, d, f, h, e])
    } else {
        select_255([a, b, d, e, g, f])
    }
}
/// n = 13, i = 5, a < g, a < j, b < g, b < i, c < g, c < i, c < j, d < e, d < m, e < f, e < i, e < j, g < h, g < m, i < k, j < l
fn select_874([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_873([f, a, b, c, i, j, k, l])
    } else {
        select_761([b, a, c, d, e, f, g, h, j, l, m])
    }
}
/// n = 14, i = 5, a < h, b < h, b < k, b < m, c < g, c < h, c < k, d < e, d < n, e < f, e < g, e < k, g < i, g < m, h < j, h < n, k < l
fn select_875([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < a {
        select_872([f, b, g, a, i, k, l, m])
    } else {
        select_874([a, b, c, d, e, f, h, j, k, g, l, i, n])
    }
}
/// n = 15, i = 6, a < j, b < i, b < j, b < m, c < f, d < e, d < o, e < g, e < h, e < m, f < j, f < k, f < m, h < i, h < k, j < l, j < o, m < n
fn select_876([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < f {
        select_870([a, g, b, c, h, i, f, k, m])
    } else {
        select_875([a, b, f, d, e, g, h, j, k, l, m, n, i, o])
    }
}
/// n = 15, i = 6, a < k, b < j, b < k, b < m, c < f, d < e, d < o, e < h, e < i, e < m, f < g, f < k, f < m, i < j, k < l, k < o, m < n
fn select_877([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < i {
        select_866([a, b, d, f, g, e, h, k, i, l, m, n, o])
    } else {
        select_876([a, b, c, d, e, f, h, i, j, k, g, l, m, n, o])
    }
}
/// n = 9, i = 3, a < d, b < c, b < h, b < i, c < f, d < e, d < h, d < i, f < g
fn select_878([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < c {
        select_9([e, b, h, i])
    } else {
        select_260([c, a, f, d, h, i, g])
    }
}
/// n = 10, i = 4, a < g, b < d, c < e, d < f, d < h, d < i, e < g, e < h, e < i, g < j
fn select_879([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_684([b, a, c, d, f, e, g, j])
    } else {
        select_878([b, e, a, d, f, g, j, h, i])
    }
}
/// n = 11, i = 5, a < i, b < g, c < e, d < g, d < h, e < f, e < h, e < j, g < i, g < j, i < k
fn select_880([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_879([a, c, b, e, g, h, i, f, j, k])
    } else {
        select_365([a, b, f, d, h, g])
    }
}
/// n = 11, i = 3, a < j, a < k, b < d, c < e, c < i, d < f, d < i, e < g, e < j, e < k, g < h
fn select_881([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_144([a, c, d, f, e, i])
    } else {
        select_40([b, a, e, g, h, j, k, d])
    }
}
/// n = 12, i = 5, a < g, a < i, b < f, b < g, c < g, c < i, c < k, d < e, d < l, e < f, e < i, f < k, g < h, g < l, i < j
fn select_882([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_9([h, d, e, l])
    } else {
        select_275([a, b, c, f, i, j, k, g])
    }
}
/// n = 13, i = 5, a < g, b < g, b < j, b < l, c < g, c < h, c < j, c < l, d < e, d < m, e < f, e < j, f < h, f < l, g < i, g < m, j < k
fn select_883([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_773([b, f, c, a, h, j, k, l])
    } else {
        select_882([b, a, c, d, e, f, g, i, j, k, h, m])
    }
}
/// n = 14, i = 5, a < h, b < g, b < h, b < k, c < e, c < n, d < g, d < h, d < i, d < k, e < f, e < k, e < m, f < i, g < m, h < j, h < n, k < l
fn select_884([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < f {
        select_881([g, a, c, h, e, j, k, l, n, f, m])
    } else {
        select_883([a, b, d, c, e, f, h, i, j, k, l, g, n])
    }
}
/// n = 15, i = 6, a < j, b < g, c < f, c < o, d < e, d < j, d < m, e < i, f < h, f < i, f < m, g < j, g < k, g < m, h < k, j < l, j < o, m < n
fn select_885([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < g {
        select_880([a, b, c, e, f, h, g, i, j, m, l])
    } else {
        select_884([a, d, c, g, f, h, e, j, k, l, m, n, i, o])
    }
}
/// n = 8, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g, f < h
fn select_886([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_14([a, g, h])
    } else {
        select_53([b, c, d, e, f])
    }
}
/// n = 9, i = 3, a < e, a < f, b < f, b < g, b < i, c < d, c < e, c < g, d < i, g < h
fn select_887([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_801([a, b, c, d, e, f, g, i])
    } else {
        select_886([a, b, c, e, f, g, d, h])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < g, b < h, c < d, c < h, d < f, d < g
fn select_888([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_9([a, b, f, g])
    } else {
        select_53([b, c, d, e, h])
    }
}
/// n = 9, i = 4, a < f, a < g, b < e, c < d, c < f, d < h, d < i, e < g, e < h, e < i
fn select_889([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_6([d, b, f, e])
    } else {
        select_888([a, e, c, d, g, h, i, f])
    }
}
/// n = 10, i = 4, a < f, a < g, b < e, c < d, c < f, c < h, d < j, e < g, e < h, e < j, h < i
fn select_890([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_886([a, c, e, g, f, h, d, i])
    } else {
        select_889([a, b, c, d, e, f, g, h, j])
    }
}
/// n = 12, i = 5, a < e, a < j, a < l, b < f, c < d, c < h, c < i, c < j, d < l, e < h, f < g, f < j, f < l, g < i, j < k
fn select_891([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_887([g, a, c, d, i, e, j, k, l])
    } else {
        select_890([e, b, c, d, f, h, g, j, k, l])
    }
}
/// n = 10, i = 4, a < d, a < i, a < j, b < f, b < g, b < i, b < j, c < e, d < f, e < g, e < i, e < j, g < h
fn select_892([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_19([g, a, i, j, h])
    } else {
        select_141([d, b, c, e, f, i, j])
    }
}
/// n = 8, i = 5, a < f, b < g, c < g, d < g, e < g, g < h
fn select_893([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_3([a, g, h])
    } else {
        select_511([b, c, d, e, f])
    }
}
/// n = 13, i = 7, a < f, a < l, a < m, b < i, c < i, d < i, e < g, f < j, g < h, g < l, g < m, h < k, i < j, i < k, i < l, i < m
fn select_894([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < h {
        select_892([a, i, e, f, g, j, h, k, l, m])
    } else {
        select_893([a, b, c, d, h, f, i, k])
    }
}
/// n = 15, i = 7, a < i, a < n, b < f, b < j, b < o, c < i, c < j, d < i, d < j, e < g, f < l, g < h, g < j, g < o, h < m, i < l, i < m, i < o, j < k, j < n
fn select_895([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < i {
        select_239([f, h, a, c, d, j, k, i, n])
    } else {
        select_894([b, a, c, d, e, f, g, h, i, l, m, j, o])
    }
}
/// n = 16, i = 7, a < k, b < i, b < p, c < f, c < k, c < n, d < i, d < k, e < g, f < j, g < h, g < k, g < n, h < l, i < j, i < l, i < n, k < m, k < p, n < o
fn select_896([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < a {
        select_891([c, e, i, a, f, g, h, j, l, n, o, k])
    } else {
        select_895([b, c, a, d, e, f, g, h, i, k, m, j, l, p, n])
    }
}
/// n = 16, i = 7, a < l, b < i, b < p, c < g, d < f, d < l, d < n, e < i, e < l, f < k, g < h, g < l, g < n, i < j, i < k, i < n, l < m, l < p, n < o
fn select_897([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < h {
        select_885([a, c, b, d, f, i, g, j, k, l, h, m, n, o, p])
    } else {
        select_896([a, b, d, e, c, f, g, h, i, k, l, j, m, n, o, p])
    }
}
/// n = 16, i = 7, a < l, b < i, b < p, c < f, c < l, c < n, d < g, e < i, e < l, g < h, g < l, g < n, i < j, i < k, i < n, l < m, l < p, n < o
fn select_898([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < f {
        select_877([a, c, d, b, i, g, h, k, j, f, l, m, n, o, p])
    } else {
        select_897([a, b, d, c, e, f, g, h, i, k, j, l, m, n, o, p])
    }
}
/// n = 16, i = 7, b < i, b < n, c < f, c < l, c < o, d < g, e < i, e < l, g < h, g < l, g < o, i < j, i < k, i < o, l < m, l < n, o < p
fn select_899([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < a {
        select_861([b, c, d, f, i, g, h, j, k, l, a, m, n, o, p])
    } else {
        select_898([a, b, c, d, e, f, g, h, i, j, k, l, m, o, p, n])
    }
}
/// n = 16, i = 7, b < f, b < o, c < i, c < n, d < g, e < i, e < l, g < h, g < l, g < o, i < j, i < k, i < o, l < m, l < n, o < p
fn select_900([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < b {
        select_845([a, c, g, h, i, l, m, j, k, b, f, n, o])
    } else {
        select_899([a, c, b, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }
}
/// n = 9, i = 2, c < d, c < e, c < f, e < i, f < g, f < h
fn select_901([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_636([b, d, e, f, a, g, h, i])
    } else {
        select_618([b, a, c, d, e, i, f])
    }
}
/// n = 11, i = 3, b < e, b < i, c < d, c < f, c < i, f < g, f < h, h < j, i < k
fn select_902([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_901([a, d, f, g, h, b, e, i, j])
    } else {
        select_639([a, b, c, d, e, i, f, g, h, k])
    }
}
/// n = 10, i = 4, a < f, b < h, c < f, c < g, d < e, e < g, e < h, g < i, h < j
fn select_903([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_436([c, a, d, b, e, f, g, i, h])
    } else {
        select_261([c, b, d, a, e, f, h, g, j, i])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, c < g, d < e, d < f
fn select_904([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_14([a, e, f])
    } else {
        select_19([c, b, d, h, g])
    }
}
/// n = 9, i = 2, a < d, b < c, b < e, c < h, d < i, e < f, e < g
fn select_905([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < a {
        select_37([c, e, f, g, a, h])
    } else {
        select_904([a, b, d, e, f, g, i, c])
    }
}
/// n = 11, i = 4, a < h, b < d, b < g, c < e, d < f, d < j, e < g, e < h, e < j, g < i, h < k
fn select_906([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_111([a, f, c, e, g, j])
    } else {
        select_905([a, b, g, h, d, f, j, i, k])
    }
}
/// n = 11, i = 4, b < h, c < e, c < g, d < f, e < j, f < g, f < h, f < j, g < i, h < k
fn select_907([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < e {
        select_903([a, b, c, d, f, e, g, h, i, k])
    } else {
        select_906([b, c, d, e, f, a, g, h, i, j, k])
    }
}
/// n = 11, i = 4, b < i, c < e, c < h, d < f, f < g, f < h, f < i, h < j, i < k
fn select_908([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_633([a, b, c, f, g, h, i, j, e, k])
    } else {
        select_907([a, b, c, d, e, f, h, i, j, g, k])
    }
}
/// n = 12, i = 4, c < e, c < j, d < f, f < g, f < h, f < j, h < i, i < k, j < l
fn select_909([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_902([b, c, f, g, e, h, a, i, j, k, l])
    } else {
        select_908([b, a, c, d, e, f, g, j, h, l, i])
    }
}
/// n = 10, i = 5, a < h, b < g, b < i, c < g, c < i, d < e, d < f, f < i, g < h, i < j
fn select_910([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_340([b, c, a, d, e, f, g, h, i])
    } else {
        select_47([e, b, c, f, a, g, i, j])
    }
}
/// n = 7, i = 4, a < g, b < f, b < g, c < e, c < g, d < e, d < f
fn select_911([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_30([a, b, d, e, g, f])
    } else {
        select_30([a, c, d, f, g, e])
    }
}
/// n = 7, i = 4, a < g, b < g, c < f, c < g, d < e, d < f
fn select_912([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_911([b, a, c, d, f, e, g])
    } else {
        select_134([b, c, e, a, f])
    }
}
/// n = 10, i = 5, a < f, b < h, b < i, c < h, c < i, d < e, d < g, f < h, g < i, i < j
fn select_913([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_867([e, a, b, c, g, f, i, j])
    } else {
        select_912([b, c, f, d, e, g, h])
    }
}
/// n = 10, i = 5, a < f, b < h, b < i, c < h, c < i, d < e, d < g, g < i, i < j
fn select_914([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_910([a, b, c, d, e, g, h, f, i, j])
    } else {
        select_913([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < e, c < f, c < g
fn select_915([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_108([e, d, b, f, g, a])
    } else {
        select_55([a, b, c, e, f, g, d])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < f, c < i, f < g, f < h
fn select_916([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_87([a, d, e, f, b, g, h])
    } else {
        select_915([a, b, c, d, e, f, i])
    }
}
/// n = 11, i = 5, a < g, a < h, b < g, b < h, c < f, c < h, d < e, d < h, h < i, h < j, h < k
fn select_917([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_228([f, g, d, h, e, i, j, k])
    } else {
        select_230([a, b, d, c, f, e, g, h])
    }
}
/// n = 12, i = 5, a < f, a < l, b < g, b < h, c < g, c < h, d < e, d < h, g < k, h < i, h < j, h < k, h < l
fn select_918([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_916([e, g, h, i, j, a, f, l, k])
    } else {
        select_917([b, c, a, d, e, f, g, h, i, j, l])
    }
}
/// n = 12, i = 5, a < g, a < l, b < h, b < i, c < h, c < i, d < e, d < f, d < i, h < k, i < j, i < k, i < l
fn select_919([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_914([a, b, c, d, f, g, e, h, i, j])
    } else {
        select_918([a, b, c, d, f, g, h, i, e, j, k, l])
    }
}
/// n = 13, i = 5, a < h, b < g, b < k, c < h, c < i, d < e, d < f, d < i, h < m, i < j, i < k, i < m, k < l
fn select_920([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < a {
        select_909([e, f, b, c, g, i, j, a, h, k, m, l])
    } else {
        select_919([b, a, c, d, e, f, g, h, i, j, m, k])
    }
}
/// n = 10, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, g < h, g < i, g < j
fn select_921([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_834([a, d, b, f, g, h, i, j, e])
    } else {
        select_834([a, e, c, f, g, h, i, j, d])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < j, d < i, e < f, e < g, e < h
fn select_922([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_221([b, d, e, f, g, h, a, i, j])
    } else {
        select_833([a, b, c, e, f, g, h, j, d])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_923([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_922([a, d, b, f, g, h, i, j, k, e])
    } else {
        select_922([a, e, c, f, g, h, i, j, k, d])
    }
}
/// n = 12, i = 4, a < d, a < f, a < i, b < c, b < e, b < j, e < h, e < i, e < l, f < g, f < j, f < l, j < k
fn select_924([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_921([c, a, e, h, d, i, f, g, j, l])
    } else {
        select_923([d, b, f, g, c, j, e, h, i, l, k])
    }
}
/// n = 12, i = 4, a < e, a < f, a < j, b < c, b < d, b < g, f < i, f < l, g < h, g < j, g < l, j < k
fn select_925([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < f {
        select_634([d, c, g, a, e, h, j, f, k, i])
    } else {
        select_924([b, a, e, d, f, g, h, i, c, j, k, l])
    }
}
/// n = 11, i = 4, b < h, c < f, c < h, d < e, d < g, d < h, e < j, f < g, f < j, g < i, h < k
fn select_926([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_632([a, d, f, b, e, g, i, j, h])
    } else {
        select_862([a, b, c, d, e, f, h, k, g])
    }
}
/// n = 11, i = 4, b < i, c < e, c < h, c < i, d < f, d < i, f < g, f < h, h < j, i < k
fn select_927([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_630([a, b, f, c, g, h, i, j, e, k])
    } else {
        select_926([a, b, d, c, e, f, h, i, j, g, k])
    }
}
/// n = 13, i = 5, a < f, a < g, a < k, b < g, b < h, c < d, c < e, c < h, g < j, g < m, h < i, h < k, h < m, k < l
fn select_928([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < g {
        select_927([e, d, a, b, f, h, i, k, g, l, j])
    } else {
        select_924([c, a, f, e, g, h, i, j, d, k, l, m])
    }
}
/// n = 14, i = 5, a < f, a < l, b < g, b < i, c < d, c < e, c < i, g < h, g < k, i < j, i < k, i < l, k < m, l < n
fn select_929([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < a {
        select_925([g, c, d, e, h, a, i, j, f, k, m, l])
    } else {
        select_928([a, b, c, d, e, f, g, i, j, h, l, n, k])
    }
}
/// n = 14, i = 5, b < g, b < l, c < h, c < i, d < e, d < f, d < i, h < k, i < j, i < k, i < l, k < m, l < n
fn select_930([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < h {
        select_920([a, b, c, d, e, f, g, h, i, j, l, n, k])
    } else {
        select_929([b, c, d, e, f, g, h, a, i, j, k, l, m, n])
    }
}
/// n = 8, i = 2, a < d, b < g, b < h, c < e, c < g, c < h, e < f
fn select_931([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_198([b, d, c, e])
    } else {
        select_38([a, e, b, g, h, f, d])
    }
}
/// n = 8, i = 2, a < h, b < e, c < d, c < f, c < h, f < g
fn select_932([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_563([b, c, e, d, f, g, a])
    } else {
        select_931([b, a, c, e, f, g, d, h])
    }
}
/// n = 8, i = 2, c < h, d < e, d < f, d < h, f < g
fn select_933([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_932([c, a, d, e, b, f, g, h])
    } else {
        select_932([c, b, d, e, a, f, g, h])
    }
}
/// n = 9, i = 1, b < i, c < h, d < e, d < f, d < g
fn select_934([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_635([a, c, d, e, f, g, h, b])
    } else {
        select_221([c, b, d, e, f, g, a, i, h])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, e < j, f < g, f < h, f < i
fn select_935([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_934([b, d, e, f, g, h, i, j, a])
    } else {
        select_922([b, a, c, e, f, g, h, i, j, d])
    }
}
/// n = 10, i = 2, d < e, d < f, d < g, f < j, g < h, g < i
fn select_936([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_933([b, c, a, d, e, f, j, g])
    } else {
        select_935([b, c, d, e, f, g, a, h, i, j])
    }
}
/// n = 9, i = 3, a < f, b < d, b < g, b < i, c < e, c < f, c < g, e < i, g < h
fn select_937([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_931([a, e, b, f, g, h, d, i])
    } else {
        select_594([a, c, b, d, f, g, h, e])
    }
}
/// n = 9, i = 3, b < e, b < g, b < i, c < d, c < f, c < g, f < i, g < h
fn select_938([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_932([f, d, b, e, a, g, h, i])
    } else {
        select_937([a, b, c, e, f, d, g, h, i])
    }
}
/// n = 11, i = 3, b < f, b < g, b < h, c < d, c < e, c < g, c < h, g < k, h < i, h < j
fn select_939([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < h {
        select_938([a, b, c, e, f, d, g, k, h])
    } else {
        select_923([a, b, c, e, f, g, h, d, i, j, k])
    }
}
/// n = 12, i = 3, b < f, b < j, c < d, c < e, c < g, c < j, g < h, g < i, i < k, j < l
fn select_940([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_936([a, d, e, g, h, i, b, f, j, k])
    } else {
        select_939([a, b, c, d, e, f, j, g, h, i, l])
    }
}
/// n = 9, i = 2, a < e, b < c, b < d, b < f, e < i, f < g, f < h
fn select_941([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_223([c, d, f, a, g, h])
    } else {
        select_572([a, b, c, d, e, i, f])
    }
}
/// n = 9, i = 3, a < f, a < i, b < d, b < e, b < i, c < e, c < f, c < i, e < g, f < h
fn select_942([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_904([a, b, f, e, d, g, h, i])
    } else {
        select_275([a, b, c, d, f, h, e, i])
    }
}
/// n = 11, i = 3, a < f, b < d, b < e, b < g, c < e, c < f, c < g, e < j, f < k, g < h, g < i
fn select_943([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_86([d, e, g, a, h, i, j])
    } else {
        select_942([a, b, c, d, e, f, j, k, g])
    }
}
/// n = 11, i = 3, a < i, b < d, b < h, c < e, c < h, c < i, e < f, e < g, h < j, i < k
fn select_944([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_941([a, e, f, g, i, b, d, h, k])
    } else {
        select_943([a, b, c, d, h, i, e, f, g, j, k])
    }
}
/// n = 10, i = 5, a < g, a < h, b < e, c < f, d < f, e < g, e < j, f < h, f < j, h < i
fn select_945([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_43([a, c, d, g, f, j])
    } else {
        select_203([a, b, e, h, j, i])
    }
}
/// n = 11, i = 5, a < g, a < i, b < e, b < h, c < f, d < f, e < g, f < h, f < i, h < j, i < k
fn select_946([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_260([a, b, i, h, e, j, k])
    } else {
        select_945([a, b, c, d, e, f, g, i, k, h])
    }
}
/// n = 8, i = 2, a < e, b < c, b < d, c < g, d < f, e < h
fn select_947([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < a {
        select_19([d, c, a, g, f])
    } else {
        select_601([a, b, d, e, f, h, c])
    }
}
/// n = 12, i = 5, a < g, a < i, b < e, b < h, c < f, d < f, e < g, e < k, f < h, f < i, f < k, h < j, i < l
fn select_948([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_947([a, b, e, h, i, j, k, l])
    } else {
        select_740([a, c, d, g, f, h, k])
    }
}
/// n = 12, i = 5, a < e, a < i, b < f, b < h, c < g, d < g, f < k, g < h, g < i, g < k, h < j, i < l
fn select_949([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_946([b, a, c, d, e, g, f, i, h, l, j])
    } else {
        select_948([a, b, c, d, f, g, e, h, i, j, k, l])
    }
}
/// n = 13, i = 5, a < j, b < i, c < f, c < h, d < g, e < g, g < h, g < i, g < j, h < k, i < l, j < m
fn select_950([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_949([c, a, d, e, f, b, g, j, h, m, i, k])
    } else {
        select_949([c, b, d, e, f, a, g, i, h, l, j, k])
    }
}
/// n = 13, i = 5, b < k, c < f, c < j, d < g, e < g, g < h, g < j, g < k, h < i, j < l, k < m
fn select_951([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_944([b, c, g, f, h, a, i, j, k, l, m])
    } else {
        select_950([a, b, c, d, e, f, g, j, k, h, l, m, i])
    }
}
/// n = 13, i = 5, b < k, c < f, c < j, d < g, e < g, g < h, g < i, g < j, g < k, j < l, k < m
fn select_952([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_951([a, b, c, d, e, f, g, h, i, j, k, l, m])
    } else {
        select_951([a, b, c, d, e, f, g, i, h, j, k, l, m])
    }
}
/// n = 14, i = 5, c < f, c < l, d < g, e < g, g < h, g < i, g < j, g < l, j < k, k < m, l < n
fn select_953([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < a {
        select_940([b, c, g, h, i, f, j, a, k, l, m, n])
    } else {
        select_952([b, a, c, d, e, f, g, h, i, l, j, n, k])
    }
}
/// n = 10, i = 6, a < j, b < g, c < g, c < i, d < h, e < h, f < h, h < i, h < j
fn select_954([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < h {
        select_548([a, c, d, e, f, g, h, i, j])
    } else {
        select_799([b, a, c, i, j, g])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < f, d < g, e < f, e < g
fn select_955([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_136([b, c, d, e, g])
    } else {
        select_136([a, c, d, e, f])
    }
}
/// n = 10, i = 6, a < i, a < j, b < h, b < j, c < h, c < j, d < h, d < j, e < f, f < g, f < j, h < i
fn select_956([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_153([a, h, e, f, i, g, j])
    } else {
        select_955([a, g, b, c, d, h, j])
    }
}
/// n = 11, i = 6, a < j, b < i, b < k, c < g, d < h, d < k, e < h, e < k, f < h, f < k, g < k, h < i, h < j
fn select_957([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_954([b, c, a, d, e, f, g, h, j, i])
    } else {
        select_956([b, d, e, f, c, g, a, h, i, k])
    }
}
/// n = 8, i = 5, a < h, b < h, c < h, d < g, e < g, e < h, f < g, f < h
fn select_958([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_394([c, d, e, f, a, b, g, h])
    } else {
        select_394([c, d, e, f, b, a, g, h])
    }
}
/// n = 10, i = 6, a < j, b < i, b < j, c < i, c < j, d < h, e < g, f < h, f < j, g < j, h < i
fn select_959([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_958([a, b, c, d, f, g, h, j])
    } else {
        select_364([a, b, c, e, h, g, i])
    }
}
/// n = 11, i = 6, a < h, b < j, b < k, c < g, d < i, d < k, e < i, e < k, f < h, f < i, f < k, g < k, i < j
fn select_960([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_957([a, b, c, d, e, f, g, i, j, h, k])
    } else {
        select_959([b, d, e, a, c, f, g, h, i, k])
    }
}
/// n = 9, i = 4, a < e, b < f, c < f, c < i, d < f, d < i, e < i, f < g, f < h
fn select_961([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_9([a, f, g, h])
    } else {
        select_136([b, c, d, e, i])
    }
}
/// n = 8, i = 3, a < d, b < e, b < f, c < e, c < f, c < h, d < h, f < g
fn select_962([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_198([a, e, f, g])
    } else {
        select_30([b, c, a, d, e, h])
    }
}
/// n = 10, i = 5, a < f, b < g, b < j, c < g, c < j, d < e, d < h, d < j, e < i, f < j, g < h, g < i
fn select_963([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_961([a, e, b, c, f, g, h, i, j])
    } else {
        select_962([a, g, d, f, h, e, i, j])
    }
}
/// n = 9, i = 5, a < e, b < g, c < g, d < f, d < h, f < i, g < h, g < i
fn select_964([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_373([e, b, c, d, f, g, h])
    } else {
        select_46([a, d, h, e, f, i])
    }
}
/// n = 10, i = 5, a < i, b < h, b < j, c < f, d < g, d < j, e < g, e < j, f < j, g < h, g < i
fn select_965([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < a {
        select_963([c, d, e, b, a, f, g, h, i, j])
    } else {
        select_964([c, d, e, a, f, b, g, i, h])
    }
}
/// n = 9, i = 5, a < g, b < e, c < g, c < i, d < f, e < i, f < h, f < i, g < h
fn select_966([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_696([a, b, c, f, e, g, i, h])
    } else {
        select_117([a, d, e, c, g, f, i])
    }
}
/// n = 9, i = 5, a < h, a < i, b < h, b < i, c < g, d < f, e < g, e < i, f < i, g < h
fn select_967([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_966([c, d, e, a, f, b, g, h, i])
    } else {
        select_966([c, d, e, b, f, a, g, h, i])
    }
}
/// n = 7, i = 4, a < g, b < g, c < f, d < e, d < g
fn select_968([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_320([c, a, d, e, f, b, g])
    } else {
        select_320([c, b, d, e, f, a, g])
    }
}
/// n = 9, i = 5, a < i, b < h, b < i, c < h, c < i, d < f, e < g, f < i, g < h
fn select_969([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_967([b, c, e, d, a, f, g, h, i])
    } else {
        select_968([b, c, d, g, a, f, h])
    }
}
/// n = 10, i = 5, a < f, b < i, b < j, c < g, d < h, d < j, e < h, e < j, g < j, h < i
fn select_970([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_965([a, b, c, d, e, g, h, i, f, j])
    } else {
        select_969([b, d, e, c, a, g, f, h, j])
    }
}
/// n = 11, i = 6, b < j, b < k, c < h, d < i, d < k, e < i, e < k, f < g, f < i, f < k, h < k, i < j
fn select_971([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_960([a, b, c, d, e, f, h, g, i, j, k])
    } else {
        select_970([g, b, c, d, e, a, h, i, j, k])
    }
}
/// n = 8, i = 2, a < f, b < c, b < d, b < e, e < g, f < h
fn select_972([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_947([a, b, c, e, f, g, d, h])
    } else {
        select_947([a, b, d, e, f, g, c, h])
    }
}
/// n = 8, i = 2, a < e, a < f, b < c, b < d, b < g, g < h
fn select_973([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_972([a, b, c, d, g, e, h, f])
    } else {
        select_972([a, b, c, d, g, f, h, e])
    }
}
/// n = 8, i = 2, a < d, a < g, a < h, b < c, b < g, b < h, c < e, d < f
fn select_974([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_19([c, a, g, h, e])
    } else {
        select_19([d, b, g, h, f])
    }
}
/// n = 10, i = 2, a < e, a < i, a < j, b < c, b < d, b < i, b < j, c < g, d < f, e < h
fn select_975([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < e {
        select_38([c, d, a, i, j, f, g])
    } else {
        select_974([a, b, d, e, f, h, i, j])
    }
}
/// n = 10, i = 2, a < f, a < i, a < j, b < c, b < d, b < e, b < i, b < j, e < g, f < h
fn select_976([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_975([a, b, c, e, f, g, d, h, i, j])
    } else {
        select_975([a, b, d, e, f, g, c, h, i, j])
    }
}
/// n = 10, i = 2, a < f, b < i, b < j, c < d, c < e, c < g, c < i, c < j, g < h
fn select_977([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_973([a, c, d, e, b, f, g, h])
    } else {
        select_976([b, c, d, e, g, a, h, f, i, j])
    }
}
/// n = 8, i = 5, a < h, b < f, c < g, d < g, e < g, g < h
fn select_978([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_533([b, c, d, e, a, f, g, h])
    } else {
        select_142([c, d, e, f, a, g])
    }
}
/// n = 8, i = 4, c < f, d < f, e < f, f < g, f < h
fn select_979([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_593([a, c, d, e, b, f, g, h])
    } else {
        select_593([b, c, d, e, a, f, g, h])
    }
}
/// n = 11, i = 5, a < g, b < h, b < k, c < f, c < h, d < g, d < h, e < g, e < h, g < k, h < i, h < j
fn select_980([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_978([b, c, a, d, e, f, g, k])
    } else {
        select_979([a, f, b, d, e, h, i, j])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, c < d, c < f, e < g
fn select_981([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_153([a, c, b, e, f, d, g])
    } else {
        select_187([b, d, a, f, e])
    }
}
/// n = 9, i = 3, a < d, a < e, b < f, c < e, c < f, f < g, f < h, f < i
fn select_982([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_7([b, d, c, e])
    } else {
        select_67([a, b, e, f, g, h, i])
    }
}
/// n = 9, i = 3, a < e, a < f, b < f, b < g, c < d, c < g, g < h, g < i
fn select_983([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_981([b, c, a, e, d, f, g])
    } else {
        select_982([a, c, b, e, f, g, d, h, i])
    }
}
/// n = 11, i = 5, b < h, b < i, c < f, c < i, d < g, d < i, e < g, e < i, g < h, i < j, i < k
fn select_984([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_980([a, b, c, d, e, f, g, i, j, k, h])
    } else {
        select_983([g, b, c, f, a, h, i, j, k])
    }
}
/// n = 13, i = 5, a < f, b < l, b < m, c < h, c < k, d < g, d < h, e < g, e < h, g < k, h < i, h < j, h < l, h < m
fn select_985([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_977([g, b, h, i, j, k, a, f, l, m])
    } else {
        select_984([b, c, a, d, e, f, g, k, h, i, j])
    }
}
/// n = 13, i = 5, c < l, c < m, d < h, d < k, e < g, e < h, f < g, f < h, g < k, h < i, h < j, h < l, h < m
fn select_986([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_985([a, c, d, e, f, b, g, h, i, j, k, l, m])
    } else {
        select_985([b, c, d, e, f, a, g, h, i, j, k, l, m])
    }
}
/// n = 14, i = 6, b < h, b < n, c < j, c < m, d < i, d < j, e < i, e < j, f < g, f < i, f < j, i < m, j < k, j < l, j < n
fn select_987([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_971([a, c, b, d, e, f, g, h, i, m, j])
    } else {
        select_986([a, g, b, c, d, e, i, j, k, l, m, h, n])
    }
}
/// n = 15, i = 6, b < i, c < h, c < n, d < j, d < m, e < i, e < j, f < g, f < i, f < j, i < m, j < k, j < l, j < n, n < o
fn select_988([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < b {
        select_953([a, g, c, d, e, h, j, k, l, b, i, n, m, o])
    } else {
        select_987([a, c, d, b, e, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 16, i = 7, b < f, b < o, c < j, c < n, d < j, d < m, e < g, e < h, g < m, h < i, h < m, h < o, j < k, j < l, j < o, m < n, o < p
fn select_989([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < h {
        select_930([a, b, e, j, k, l, f, g, h, i, m, o, n, p])
    } else {
        select_988([a, g, b, c, d, h, i, f, m, j, k, l, n, o, p])
    }
}
/// n = 16, i = 7, b < g, b < o, c < j, c < n, d < j, d < m, e < f, e < h, h < i, h < m, h < o, j < k, j < l, j < o, m < n, o < p
fn select_990([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if m < f {
        select_900([a, b, c, e, d, g, h, i, j, k, l, m, f, n, o, p])
    } else {
        select_989([a, b, c, d, e, g, f, h, i, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 7, b < g, b < o, c < f, c < i, d < k, d < n, e < h, e < k, h < n, i < j, i < o, k < l, k < m, k < o, o < p
fn select_991([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < i {
        select_820([a, b, h, c, d, f, g, k, l, m, n, i, j, o])
    } else {
        select_990([a, b, d, e, c, f, g, i, j, k, l, m, h, n, o, p])
    }
}
/// n = 17, i = 8, b < h, b < p, c < m, c < o, d < g, d < i, e < f, e < j, i < k, i < p, j < l, j < m, l < o, m < n, m < p, p < q
fn select_992([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < m {
        select_627([b, a, c, d, e, f, g, h, i, j, k, l, o, m, n, p])
    } else {
        select_991([f, b, d, c, j, g, h, l, i, k, m, a, n, o, p, q])
    }
}
/// n = 9, i = 2, a < c, b < d, b < e, d < i, e < f, e < g, e < h
fn select_993([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_56([d, e, f, g, h, a, i])
    } else {
        select_63([a, b, c, e, f, g, h, d])
    }
}
/// n = 10, i = 2, a < j, b < d, c < e, c < f, c < j, e < i, f < g, f < h
fn select_994([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_931([b, a, c, d, e, i, f, j])
    } else {
        select_993([b, c, d, e, f, a, g, h, i])
    }
}
/// n = 10, i = 2, c < j, d < e, d < f, d < j, e < i, f < g, f < h
fn select_995([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_994([c, a, d, b, e, f, g, h, i, j])
    } else {
        select_994([c, b, d, a, e, f, g, h, i, j])
    }
}
/// n = 10, i = 1, a < i, a < j, b < g, b < h, c < d, c < e, c < f
fn select_996([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_220([a, c, d, e, f, b, i, j])
    } else {
        select_220([b, c, d, e, f, a, g, h])
    }
}
/// n = 10, i = 1, b < j, c < h, c < i, d < e, d < f, d < g
fn select_997([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_221([a, c, d, e, f, g, h, i, b])
    } else {
        select_996([b, c, d, e, f, g, h, i, a, j])
    }
}
/// n = 8, i = 2, a < d, a < g, a < h, b < c, b < e, b < g, b < h, e < f
fn select_998([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_108([c, e, a, g, h, f])
    } else {
        select_54([a, b, c, d, g, h])
    }
}
/// n = 10, i = 3, a < j, b < e, b < f, b < j, c < d, c < f, c < h, c < i, c < j, e < h, e < i, f < g
fn select_999([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_998([e, c, d, a, f, g, h, i])
    } else {
        select_846([a, b, c, d, e, f, g, j])
    }
}
/// n = 12, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, c < k, c < l, e < k, e < l, f < j, g < h, g < i
fn select_1000([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_997([d, f, e, g, a, h, i, k, l, j])
    } else {
        select_999([a, b, c, d, e, f, j, k, l, g])
    }
}
/// n = 12, i = 3, b < f, b < g, b < h, c < d, c < e, c < g, c < h, c < l, f < l, g < k, h < i, h < j
fn select_1001([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < f {
        select_995([a, e, d, b, g, h, i, j, k, f])
    } else {
        select_1000([a, b, c, e, f, g, h, i, j, k, d, l])
    }
}
/// n = 8, i = 4, b < h, c < g, c < h, d < f, d < h, e < f, e < h, f < g
fn select_1002([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_722([c, b, d, e, a, f, g, h])
    } else {
        select_240([c, a, d, e, b, f, g])
    }
}
/// n = 11, i = 4, c < g, c < k, d < f, d < g, e < f, e < g, f < k, g < h, g < i, g < j
fn select_1003([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_831([b, f, c, g, a, h, i, j, k])
    } else {
        select_1002([b, a, c, d, e, f, k, g])
    }
}
/// n = 8, i = 5, b < g, c < g, d < f, d < h, e < f, e < h, g < h
fn select_1004([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < a {
        select_89([d, e, g, a, f, h])
    } else {
        select_413([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 5, b < i, c < g, c < i, d < g, d < i, e < f, f < h, f < i, g < h
fn select_1005([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_358([a, c, d, f, b, g, h, i])
    } else {
        select_1004([a, b, e, c, d, g, f, i])
    }
}
/// n = 12, i = 5, c < g, c < h, d < g, d < h, e < f, f < h, f < l, g < l, h < i, h < j, h < k
fn select_1006([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_831([b, g, f, h, a, i, j, k, l])
    } else {
        select_1005([b, a, c, d, e, f, g, l, h])
    }
}
/// n = 12, i = 5, c < g, c < i, d < g, d < h, d < i, e < f, f < h, f < i, i < j, i < k, i < l
fn select_1007([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_1003([a, b, c, d, f, h, i, j, k, l, g])
    } else {
        select_1006([a, b, c, d, e, f, g, i, j, k, l, h])
    }
}
/// n = 15, i = 6, b < h, b < i, c < e, c < f, c < m, d < g, e < o, f < h, f < j, g < i, g < j, i < k, i < l, i < m, i < o, m < n
fn select_1008([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_1001([a, c, i, k, l, e, m, f, h, j, n, o])
    } else {
        select_1007([a, e, b, f, d, g, h, j, i, k, l, m])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < i, d < h, e < f, e < g
fn select_1009([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_86([a, d, e, b, f, g, h])
    } else {
        select_264([a, b, c, d, h, e, i])
    }
}
/// n = 10, i = 3, a < j, b < e, b < i, c < d, c < f, f < g, f < i, f < j, g < h
fn select_1010([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_1009([d, a, f, g, b, e, i, h, j])
    } else {
        select_822([a, b, c, d, e, f, g, i])
    }
}
/// n = 13, i = 5, b < f, b < h, c < f, c < g, d < e, d < j, d < l, e < g, e < h, e < m, f < i, f < j, f < m, i < l, j < k
fn select_1011([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < e {
        select_922([a, i, d, j, e, g, h, m, k, l])
    } else {
        select_69([a, b, c, g, h, f, i, j, m])
    }
}
/// n = 15, i = 6, a < g, a < k, b < g, b < j, c < d, c < e, e < f, e < m, e < o, f < j, f < k, g < h, g < i, g < m, h < o, i < l, m < n
fn select_1012([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_1010([h, i, c, d, l, e, m, n, f, o])
    } else {
        select_1011([d, a, b, e, f, g, j, k, h, m, n, o, i])
    }
}
/// n = 15, i = 6, a < g, a < l, b < g, b < k, c < d, c < e, e < f, e < m, e < o, f < k, f < l, g < h, g < i, g < j, g < m, j < o, m < n
fn select_1013([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_1012([a, b, c, d, e, f, g, j, h, k, l, i, m, n, o])
    } else {
        select_1012([a, b, c, d, e, f, g, j, i, k, l, h, m, n, o])
    }
}
/// n = 16, i = 7, a < i, a < j, b < f, c < d, c < e, e < g, e < h, e < o, f < i, f < n, h < j, h < n, i < k, i < l, i < m, i < o, o < p
fn select_1014([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < k {
        select_1008([d, a, e, b, g, h, f, j, i, n, l, m, o, p, k])
    } else {
        select_1013([a, f, c, d, e, h, i, l, m, k, n, j, o, p, g])
    }
}
/// n = 10, i = 4, a < e, a < h, b < e, b < g, c < d, c < i, c < j, d < g, d < h, e < f, e < i, e < j
fn select_1015([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_137([f, c, d, i, j])
    } else {
        select_53([a, b, g, h, e])
    }
}
/// n = 12, i = 5, a < f, a < j, b < f, b < i, c < d, d < e, d < k, d < l, e < i, e < j, f < g, f < h, f < k, f < l
fn select_1016([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_111([g, h, c, d, k, l])
    } else {
        select_1015([a, b, d, e, f, h, i, j, k, l])
    }
}
/// n = 10, i = 5, a < i, a < j, b < g, b < j, c < g, c < j, d < e, d < h, e < f, e < j, g < h, g < i
fn select_1017([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_725([a, g, d, e, h, i, f, j])
    } else {
        select_120([a, f, b, c, g, j])
    }
}
/// n = 15, i = 7, a < j, a < n, b < i, b < j, c < i, c < j, d < g, d < j, d < m, e < f, f < g, f < i, f < o, g < h, i < m, i < n, j < k, j < l, j < o
fn select_1018([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < g {
        select_1016([a, d, e, f, i, j, k, l, m, n, g, o])
    } else {
        select_1017([a, b, c, d, g, h, i, m, n, j])
    }
}
/// n = 9, i = 4, a < f, a < h, a < i, b < e, b < f, b < i, c < d, c < e, d < h, d < i, f < g
fn select_1019([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_207([d, a, b, e, h, f, i])
    } else {
        select_10([b, c, d, f, h, g, e, i])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, b < g, c < d, c < h, c < i, d < f, e < h, e < i
fn select_1020([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_375([b, d, f, a, e, h, g])
    } else {
        select_1019([c, a, b, e, g, d, f, h, i])
    }
}
/// n = 9, i = 4, a < h, a < i, b < d, b < g, b < h, c < e, c < g, c < i, d < i, e < f
fn select_1021([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_614([a, e, b, f, g, d, h])
    } else {
        select_1020([a, b, c, e, d, f, h, g, i])
    }
}
/// n = 15, i = 7, a < j, b < j, b < l, c < h, c < j, d < g, d < j, d < k, e < f, f < g, f < h, f < o, g < i, h < k, h < l, j < m, j < n, j < o
fn select_1022([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < h {
        select_1018([b, a, c, d, e, f, g, i, h, j, m, n, k, l, o])
    } else {
        select_1021([b, h, d, a, g, i, k, l, j])
    }
}
/// n = 9, i = 5, b < i, c < g, d < g, e < f, e < i, f < h, g < h, g < i
fn select_1023([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_540([a, b, e, g, f, h, i])
    } else {
        select_497([a, b, c, d, f, g, h])
    }
}
/// n = 7, i = 5, b < g, c < g, d < f, e < f
fn select_1024([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < f {
        select_27([a, d, e, c, g, f])
    } else {
        select_6([a, b, c, g])
    }
}
/// n = 7, i = 5, c < g, d < g, e < f
fn select_1025([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_1024([b, c, d, a, e, f, g])
    } else {
        select_27([b, c, d, f, a, g])
    }
}
/// n = 11, i = 6, a < h, a < k, b < i, c < i, d < g, d < k, e < f, e < i, g < h, g < j, i < j, i < k
fn select_1026([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_1023([f, a, b, c, d, g, i, j, k])
    } else {
        select_1025([b, c, a, g, e, f, h])
    }
}
/// n = 9, i = 5, a < e, b < f, c < d, c < g, d < i, e < h, e < i, f < g, f < h
fn select_1027([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_688([e, b, c, d, f, h, i, g])
    } else {
        select_6([d, a, g, e])
    }
}
/// n = 9, i = 5, a < h, b < g, c < e, d < f, e < h, e < i, f < g, f < i
fn select_1028([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1027([d, c, a, b, f, e, h, i, g])
    } else {
        select_1027([c, d, b, a, e, f, g, i, h])
    }
}
/// n = 10, i = 5, b < h, b < j, c < f, d < e, d < j, e < h, e < i, f < g, f < i, g < j
fn select_1029([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_728([b, d, f, e, g, a, i, h, j])
    } else {
        select_1028([a, b, c, d, f, e, h, g, i])
    }
}
/// n = 11, i = 6, a < k, b < i, b < k, c < h, d < g, d < k, e < f, e < h, g < i, g < j, h < j, h < k
fn select_1030([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_1026([b, a, c, d, e, f, g, i, h, j, k])
    } else {
        select_1029([f, b, c, d, g, h, a, i, j, k])
    }
}
/// n = 16, i = 8, a < l, b < k, b < l, c < f, d < j, d < l, e < g, f < l, f < m, g < h, g < j, g < p, h < i, j < k, j < m, l < n, l < o, l < p
fn select_1031([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < h {
        select_1022([a, b, d, f, e, g, h, j, i, l, m, k, n, o, p])
    } else {
        select_1030([a, b, c, d, h, i, j, f, k, m, l])
    }
}
/// n = 11, i = 5, a < f, a < k, b < f, b < i, b < j, c < g, d < g, e < g, g < h, g < i, g < j, g < k
fn select_1032([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_108([h, a, b, i, j, k])
    } else {
        select_593([b, c, d, e, f, g, h, k])
    }
}
/// n = 12, i = 6, a < h, b < h, c < h, d < e, d < f, e < g, f < g, f < k, f < l, h < i, h < j, h < k, h < l
fn select_1033([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_1032([e, f, a, b, c, g, h, j, k, l, i])
    } else {
        select_245([j, i, d, f, k, l, e])
    }
}
/// n = 10, i = 5, a < i, a < j, b < g, c < g, d < e, d < h, d < j, e < f, g < h, g < i, g < j
fn select_1034([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_323([a, d, h, i, e, f, j])
    } else {
        select_115([a, f, b, c, g, h])
    }
}
/// n = 15, i = 7, a < j, a < n, b < i, b < j, c < i, c < j, d < g, d < j, d < m, e < f, e < g, f < h, f < i, f < o, g < h, i < m, i < n, j < k, j < l, j < o
fn select_1035([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < i {
        select_1033([a, b, c, e, g, f, h, j, k, l, i, o])
    } else {
        select_1034([a, b, c, d, g, h, i, m, n, j])
    }
}
/// n = 15, i = 7, a < j, b < j, b < l, c < h, c < j, d < g, d < j, d < k, e < f, e < g, f < h, f < i, f < o, g < i, h < k, h < l, j < m, j < n, j < o
fn select_1036([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < h {
        select_1035([b, a, c, d, e, f, g, i, h, j, m, n, k, l, o])
    } else {
        select_1021([b, h, d, a, g, i, k, l, j])
    }
}
/// n = 9, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h, f < i
fn select_1037([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_278([a, d, c, e, g])
    } else {
        select_260([b, a, e, f, h, i, g])
    }
}
/// n = 14, i = 7, a < i, a < k, b < g, b < j, c < h, d < h, e < h, f < g, f < k, g < i, g < n, h < j, h < k, h < n, k < l, k < m
fn select_1038([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_1037([a, b, f, j, g, k, n, l, m])
    } else {
        select_593([a, c, d, e, i, h, j, n])
    }
}
/// n = 9, i = 6, a < h, b < f, c < e, d < g, e < i, f < i, g < h
fn select_1039([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < i {
        select_501([a, g, b, c, e, f, h])
    } else {
        select_27([a, e, f, d, g, i])
    }
}
/// n = 9, i = 6, a < i, b < i, c < h, d < f, e < g, f < i, g < h
fn select_1040([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1039([c, a, d, e, f, b, g, h, i])
    } else {
        select_1039([c, b, d, e, f, a, g, h, i])
    }
}
/// n = 14, i = 7, a < j, a < k, b < g, b < h, c < i, d < i, e < h, e < k, f < g, f < i, h < j, h < l, i < k, i < l, k < m, k < n
fn select_1041([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_1038([a, b, c, d, f, e, h, i, j, g, k, m, n, l])
    } else {
        select_1040([c, d, a, f, e, g, h, j, i])
    }
}
/// n = 12, i = 5, a < i, a < j, b < f, b < g, c < f, c < j, d < e, d < h, e < g, e < j, f < h, f < i, j < k, j < l
fn select_1042([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_46([a, d, h, i, e, g])
    } else {
        select_1037([a, b, c, g, f, j, h, k, l])
    }
}
/// n = 13, i = 6, a < i, a < k, b < e, b < g, c < g, c < k, d < f, f < h, f < j, g < i, g < j, h < k, k < l, k < m
fn select_1043([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_1042([a, b, c, f, h, g, e, j, i, k, l, m])
    } else {
        select_1028([a, e, c, d, g, f, h, i, j])
    }
}
/// n = 14, i = 7, a < l, b < j, b < l, c < g, c < h, d < i, e < h, e < l, f < g, f < i, h < j, h < k, i < k, i < l, l < m, l < n
fn select_1044([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < i {
        select_1041([b, c, a, d, e, f, g, h, i, j, l, k, m, n])
    } else {
        select_1043([b, c, e, d, g, i, h, a, j, k, l, m, n])
    }
}
/// n = 16, i = 8, a < l, b < k, b < l, c < h, d < j, d < l, e < f, e < g, f < i, g < i, g < j, g < p, h < l, h < m, j < k, j < m, l < n, l < o, l < p
fn select_1045([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < f {
        select_1036([a, b, d, h, e, g, f, j, i, l, m, k, n, o, p])
    } else {
        select_1044([a, b, g, c, d, f, i, j, h, k, m, l, n, o])
    }
}
/// n = 16, i = 8, a < l, b < k, b < l, c < h, d < j, d < l, e < f, e < g, g < i, g < j, g < p, h < l, h < m, j < k, j < m, l < n, l < o, l < p
fn select_1046([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < f {
        select_1031([a, b, c, d, e, h, g, i, f, j, k, l, m, n, o, p])
    } else {
        select_1045([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }
}
/// n = 17, i = 8, b < k, b < l, c < h, d < j, d < k, e < f, e < g, g < i, g < j, g < p, h < k, h < o, j < l, j < o, k < m, k < n, k < p, p < q
fn select_1047([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if k < a {
        select_1014([b, c, e, f, g, h, i, j, k, l, a, m, n, o, p, q])
    } else {
        select_1046([a, b, c, d, e, f, g, h, i, j, l, k, o, m, n, p])
    }
}
/// n = 5, i = 1, c < d, c < e
fn select_1048([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_19([a, c, d, e, b])
    } else {
        select_19([b, c, d, e, a])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < f
fn select_1049([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < c {
        select_1048([a, e, c, d, f])
    } else {
        select_1048([a, d, b, e, f])
    }
}
/// n = 7, i = 3, b < e, b < g, c < d, c < f, f < g
fn select_1050([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_1049([d, b, f, a, e, g])
    } else {
        select_188([a, b, c, d, e, f])
    }
}
/// n = 10, i = 3, b < f, b < g, c < d, c < e, c < g, g < h, g < i, g < j
fn select_1051([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < g {
        select_1050([a, b, c, e, f, d, g])
    } else {
        select_832([a, b, c, e, f, g, d, h, i, j])
    }
}
/// n = 9, i = 6, a < f, b < h, c < h, d < h, e < g, g < i, h < i
fn select_1052([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_533([a, b, c, d, g, f, h, i])
    } else {
        select_413([b, c, d, e, f, g, h])
    }
}
/// n = 13, i = 6, a < f, b < h, b < i, c < h, c < i, d < h, d < i, e < g, g < i, g < m, h < m, i < j, i < k, i < l
fn select_1053([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < i {
        select_1052([a, b, c, d, e, f, g, h, m])
    } else {
        select_68([a, h, g, i, j, k, l, m])
    }
}
/// n = 13, i = 6, c < h, c < i, d < h, d < i, e < h, e < i, f < g, g < i, g < m, h < m, i < j, i < k, i < l
fn select_1054([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_1053([a, c, d, e, f, b, g, h, i, j, k, l, m])
    } else {
        select_1053([b, c, d, e, f, a, g, h, i, j, k, l, m])
    }
}
/// n = 14, i = 6, c < h, c < i, d < h, d < i, e < h, e < i, f < g, g < i, g < n, h < n, i < j, i < k, i < l, i < m
fn select_1055([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < a {
        select_831([b, h, g, i, j, k, l, m, n])
    } else {
        select_1054([a, b, c, d, e, f, g, h, i, k, l, m, n])
    }
}
/// n = 15, i = 7, b < f, b < g, c < i, c < j, d < i, d < j, e < h, g < i, g < m, g < n, h < j, h < o, i < o, j < k, j < l, j < m, j < n
fn select_1056([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < g {
        select_1051([a, b, j, k, l, f, g, i, m, n])
    } else {
        select_1055([a, f, c, d, g, e, h, i, j, k, l, m, n, o])
    }
}
/// n = 8, i = 3, a < h, b < d, b < f, c < e, c < g, d < g, e < f, e < h
fn select_1057([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_50([d, c, a, e, f, g])
    } else {
        select_153([a, b, c, e, d, f, h])
    }
}
/// n = 10, i = 5, a < g, b < h, b < j, c < f, d < e, d < i, e < g, e < j, f < h, f < j, h < i
fn select_1058([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_1057([a, b, d, h, e, j, i, g])
    } else {
        select_8([b, c, d, f, e, g, h])
    }
}
/// n = 11, i = 7, a < i, b < f, c < i, c < j, d < g, e < h, f < j, g < j, g < k, h < i, i < k
fn select_1059([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_966([a, b, c, d, f, g, i, k, j])
    } else {
        select_28([a, c, e, d, h, g, j, i])
    }
}
/// n = 12, i = 7, a < f, b < j, b < l, c < i, c < j, d < g, e < h, g < j, g < l, h < i, h < k, i < l, j < k
fn select_1060([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_1058([a, b, d, h, i, g, f, j, k, l])
    } else {
        select_1059([b, a, c, e, d, f, h, g, j, i, k])
    }
}
/// n = 12, i = 7, c < j, c < l, d < i, d < j, e < g, f < h, g < j, g < l, h < i, h < k, i < l, j < k
fn select_1061([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_1060([a, c, d, e, f, b, g, h, i, j, k, l])
    } else {
        select_1060([b, c, d, e, f, a, g, h, i, j, k, l])
    }
}
/// n = 8, i = 4, b < g, b < h, c < f, d < e, d < g, d < h, f < g, f < h
fn select_1062([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_224([b, d, f, a, e, g, h])
    } else {
        select_27([b, a, c, d, e, f])
    }
}
/// n = 10, i = 6, c < i, c < j, d < h, e < h, f < g, g < i, g < j, h < i, h < j
fn select_1063([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_1062([b, c, f, h, a, g, i, j])
    } else {
        select_415([b, c, a, d, e, f, g, h])
    }
}
/// n = 8, i = 5, c < h, d < h, e < g, f < g, g < h
fn select_1064([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_309([c, d, a, e, f, b, g, h])
    } else {
        select_309([c, d, b, e, f, a, g, h])
    }
}
/// n = 10, i = 6, c < i, c < j, d < h, e < h, f < g, g < i, g < j, h < j
fn select_1065([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_1063([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_1064([a, b, d, e, c, g, i, h])
    }
}
/// n = 12, i = 7, c < j, c < l, d < h, e < i, f < g, f < j, g < l, h < k, h < l, i < j, i < l, j < k
fn select_1066([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_1061([a, b, c, f, e, d, i, h, g, j, k, l])
    } else {
        select_1065([a, b, c, d, g, e, i, h, j, l])
    }
}
/// n = 16, i = 8, b < k, b < l, c < h, d < f, d < g, e < i, g < j, g < k, g < o, h < l, h < p, i < k, i < l, k < p, l < m, l < n, l < o
fn select_1067([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < j {
        select_1056([a, d, b, i, c, f, g, h, k, l, m, n, j, o, p])
    } else {
        select_1066([a, f, b, c, e, g, j, h, i, k, p, l])
    }
}
/// n = 8, i = 2, b < e, c < d, c < f, f < g, f < h
fn select_1068([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_799([a, b, c, d, e, f])
    } else {
        select_64([b, c, d, e, f, a, g, h])
    }
}
/// n = 7, i = 3, a < g, b < f, c < e, c < g, d < e, d < f, d < g
fn select_1069([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < b {
        select_170([a, b, d, e, f, g])
    } else {
        select_191([a, c, d, f, e])
    }
}
/// n = 9, i = 3, b < f, c < e, c < g, d < e, d < f, d < g, g < h, g < i
fn select_1070([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_64([b, c, e, f, g, a, h, i])
    } else {
        select_1069([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 3, c < f, c < g, d < e, d < f, d < g, g < h, g < i
fn select_1071([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_1068([b, e, c, f, a, g, h, i])
    } else {
        select_1070([b, a, c, d, f, e, g, h, i])
    }
}
/// n = 9, i = 2, c < d, c < e, d < i, e < f, e < g, e < h
fn select_1072([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_851([b, d, e, f, g, h, a, i])
    } else {
        select_68([b, a, c, e, f, g, h, d])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, c < h, c < i, e < h, e < i, f < g
fn select_1073([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_58([a, e, c, f, d, g, h, i])
    } else {
        select_532([a, b, c, d, e, h, i, f])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, c < k, e < k, f < j, g < h, g < i
fn select_1074([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_1072([a, d, b, f, g, e, h, i, j])
    } else {
        select_1073([a, b, c, d, e, f, j, g, k])
    }
}
/// n = 12, i = 4, b < f, b < i, c < d, c < e, c < j, d < l, e < g, e < i, f < h, f < j, f < l, j < k
fn select_1075([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_1071([a, d, b, e, g, i, f, h, j])
    } else {
        select_1074([a, c, f, h, d, j, e, g, i, k, l])
    }
}
/// n = 9, i = 2, a < c, a < h, a < i, b < d, b < e, b < h, b < i, d < g, e < f
fn select_1076([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_161([a, b, c, d, e, g, h, i])
    } else {
        select_161([a, b, c, e, d, f, h, i])
    }
}
/// n = 10, i = 3, b < e, b < f, b < j, c < d, c < f, c < h, c < i, d < j, e < h, e < i, f < g
fn select_1077([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_1076([e, c, a, d, f, g, j, h, i])
    } else {
        select_594([a, b, c, d, e, f, g, j])
    }
}
/// n = 11, i = 3, b < g, c < f, c < i, c < j, d < e, d < f, d < g, e < i, e < j, f < h, g < k
fn select_1078([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_631([a, b, d, e, c, g, k, f])
    } else {
        select_1077([a, d, c, b, e, f, h, i, j, g])
    }
}
/// n = 6, i = 2, a < f, b < f, c < d, d < e
fn select_1079([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_9([b, d, a, e])
    } else {
        select_18([b, c, a, d, f])
    }
}
/// n = 8, i = 2, a < h, b < e, c < d, c < h, d < f, e < g
fn select_1080([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_1079([a, c, b, e, g, h])
    } else {
        select_255([b, a, c, d, f, h])
    }
}
/// n = 10, i = 3, a < f, b < d, b < e, c < e, c < f, c < h, c < i, d < h, d < i, e < g, f < j
fn select_1081([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_305([d, c, a, e, g, h, i])
    } else {
        select_273([a, b, c, d, e, f, g, j])
    }
}
/// n = 10, i = 3, a < g, b < e, b < f, c < d, c < f, c < g, c < i, e < i, f < h, g < j
fn select_1082([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_1080([d, a, b, f, g, h, j, e])
    } else {
        select_1081([a, b, c, e, f, g, h, d, i, j])
    }
}
/// n = 10, i = 6, a < j, b < g, b < i, c < h, d < h, e < h, f < h, h < i, h < j
fn select_1083([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_548([a, b, d, e, f, g, h, i, j])
    } else {
        select_893([b, c, d, e, f, g, h, j])
    }
}
/// n = 13, i = 6, b < j, c < g, c < i, d < h, e < h, f < h, g < l, h < i, h < j, h < l, i < k, j < m
fn select_1084([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_1082([b, c, h, a, g, i, j, k, l, m])
    } else {
        select_1083([b, c, a, d, e, f, g, h, i, j])
    }
}
/// n = 14, i = 6, b < k, c < g, c < j, c < m, d < h, e < h, f < h, h < i, h < j, h < k, i < m, j < l, k < n
fn select_1085([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_1078([a, b, c, h, i, j, k, l, g, m, n])
    } else {
        select_1084([a, b, c, d, e, f, g, h, j, k, l, i, n])
    }
}
/// n = 15, i = 7, b < f, b < g, c < i, d < i, e < i, g < h, g < m, g < o, i < j, i < k, i < m, j < o, k < l, m < n
fn select_1086([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < f {
        select_1075([a, b, i, j, k, g, l, h, f, m, n, o])
    } else {
        select_1085([a, f, g, c, d, e, h, i, j, m, k, n, o, l])
    }
}
/// n = 15, i = 7, b < f, b < g, c < i, d < i, e < i, g < h, g < m, g < o, i < j, i < k, i < l, i < m, l < o, m < n
fn select_1087([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < k {
        select_1086([a, b, c, d, e, f, g, h, i, l, j, k, m, n, o])
    } else {
        select_1086([a, b, c, d, e, f, g, h, i, l, k, j, m, n, o])
    }
}
/// n = 17, i = 8, b < k, b < n, c < i, d < f, d < h, e < g, e < k, g < n, h < j, h < n, h < o, i < k, i < q, k < l, k < m, k < o, n < q, o < p
fn select_1088([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if g < k {
        select_1067([a, b, c, d, e, f, h, i, g, j, n, k, l, m, o, q])
    } else {
        select_1087([a, d, b, e, i, f, h, j, k, l, m, g, o, p, n])
    }
}
/// n = 17, i = 8, b < g, b < l, c < k, c < l, d < i, e < f, e < h, h < j, h < k, h < p, i < l, i < o, k < o, l < m, l < n, l < p, p < q
fn select_1089([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if k < g {
        select_1047([a, b, d, c, e, f, h, i, j, k, l, g, m, n, o, p, q])
    } else {
        select_1088([a, c, d, e, b, f, g, h, i, j, l, m, n, k, p, q, o])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, c < d, c < f, d < h, f < g
fn select_1090([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_45([a, c, e, f, d, g])
    } else {
        select_153([a, b, c, d, e, f, h])
    }
}
/// n = 9, i = 3, a < f, a < i, b < e, b < f, b < g, c < d, c < e, d < g, d < i, f < h
fn select_1091([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_50([b, d, e, a, i, g])
    } else {
        select_273([b, c, a, e, d, f, i, h])
    }
}
/// n = 9, i = 3, a < e, a < f, b < d, b < g, c < f, c < g, c < i, e < i, g < h
fn select_1092([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_1090([a, c, b, d, f, g, h, e])
    } else {
        select_1091([b, c, a, e, f, g, i, h, d])
    }
}
/// n = 10, i = 4, a < e, a < f, b < f, b < h, b < j, c < d, c < e, d < h, e < j, f < g, h < i
fn select_1093([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_50([f, c, g, e, j, d])
    } else {
        select_275([d, a, b, e, h, i, j, f])
    }
}
/// n = 11, i = 4, a < f, a < g, b < g, b < i, b < k, c < d, c < f, d < e, d < i, f < k, g < h, i < j
fn select_1094([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_793([a, b, d, e, f, i, j, k])
    } else {
        select_1093([a, b, c, d, f, g, h, i, j, k])
    }
}
/// n = 11, i = 4, a < g, b < g, b < h, b < j, c < d, c < e, d < h, e < f, e < j, g < i, j < k
fn select_1095([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_1092([d, e, b, f, a, h, j, k, g])
    } else {
        select_1094([a, b, c, e, f, d, g, i, j, k, h])
    }
}
/// n = 10, i = 3, a < d, a < g, b < c, b < g, c < e, d < f, g < h, g < i, g < j
fn select_1096([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_63([c, a, e, g, h, i, j, d])
    } else {
        select_63([d, b, f, g, h, i, j, c])
    }
}
/// n = 10, i = 3, a < e, a < g, b < c, b < d, b < g, e < f, g < h, g < i, g < j
fn select_1097([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_1096([a, b, c, e, d, f, g, h, i, j])
    } else {
        select_1096([a, b, d, e, c, f, g, h, i, j])
    }
}
/// n = 10, i = 2, a < c, b < d, b < e, d < i, d < j, e < f, e < g, e < h
fn select_1098([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_220([d, e, f, g, h, a, i, j])
    } else {
        select_63([a, b, c, e, f, g, h, d])
    }
}
/// n = 10, i = 3, a < e, b < e, b < f, b < j, c < d, c < f, d < j, f < g, f < h, f < i
fn select_1099([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_63([d, b, a, f, g, h, i, j])
    } else {
        select_91([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 11, i = 3, a < e, b < e, b < f, b < g, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_1100([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_1098([a, c, e, f, g, h, i, j, d, k])
    } else {
        select_1099([a, b, c, d, e, g, h, i, j, f])
    }
}
/// n = 12, i = 4, a < d, a < i, a < j, b < c, b < e, c < i, d < g, d < h, d < l, e < f, e < j, e < l, j < k
fn select_1101([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_1097([b, d, g, h, c, i, e, f, j, l])
    } else {
        select_1100([c, a, e, f, i, j, d, g, h, l, k])
    }
}
/// n = 12, i = 4, b < e, b < i, b < j, c < d, c < f, d < i, e < h, e < l, f < g, f < j, f < l, j < k
fn select_1102([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < e {
        select_1095([a, b, c, d, f, g, e, i, h, j, k])
    } else {
        select_1101([b, c, d, e, f, g, a, h, i, j, k, l])
    }
}
/// n = 7, i = 3, a < g, b < f, c < f, c < g, d < e, d < g
fn select_1103([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_380([c, d, a, e, b, f, g])
    } else {
        select_188([c, d, b, a, e, f])
    }
}
/// n = 10, i = 3, a < e, b < e, b < f, c < d, c < f, f < g, f < h, f < i, f < j
fn select_1104([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_17([a, c, e, f, g, h, i, j])
    } else {
        select_92([a, b, c, d, e, f, h, i, j])
    }
}
/// n = 10, i = 3, b < f, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_1105([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_1103([a, b, c, d, e, f, g])
    } else {
        select_1104([b, c, d, e, f, g, a, h, i, j])
    }
}
/// n = 10, i = 2, a < d, b < c, b < e, b < f, e < j, f < g, f < h, f < i
fn select_1106([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < e {
        select_993([a, b, d, c, f, g, h, i, e])
    } else {
        select_1098([a, b, d, e, f, g, h, i, c, j])
    }
}
/// n = 7, i = 2, a < d, a < e, b < c, b < f, f < g
fn select_1107([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_73([b, a, d, e, c])
    } else {
        select_46([a, b, c, e, f, g])
    }
}
/// n = 9, i = 3, a < i, b < d, b < g, b < i, c < e, c < g, c < i, e < f, g < h
fn select_1108([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_1107([e, b, d, a, f, g, h])
    } else {
        select_942([a, b, c, d, g, e, h, f, i])
    }
}
/// n = 11, i = 3, b < d, b < g, b < h, c < e, c < g, c < h, e < f, g < k, h < i, h < j
fn select_1109([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_1106([e, b, d, f, g, h, a, i, j, k])
    } else {
        select_1108([a, b, c, d, e, f, g, k, h])
    }
}
/// n = 12, i = 4, b < f, b < i, c < d, c < e, c < j, d < i, e < g, e < l, f < h, f < j, f < l, j < k
fn select_1110([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_1105([a, d, b, e, g, i, f, h, j, l])
    } else {
        select_1109([a, f, c, h, d, i, j, e, g, l, k])
    }
}
/// n = 12, i = 4, b < e, b < g, c < d, c < f, c < j, f < i, f < l, g < h, g < j, g < l, j < k
fn select_1111([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < d {
        select_1102([a, c, b, e, f, g, h, i, d, j, k, l])
    } else {
        select_1110([a, b, c, d, f, g, i, h, e, j, k, l])
    }
}
/// n = 13, i = 4, b < g, b < k, c < f, c < j, d < e, d < h, h < i, h < j, h < k, j < l, k < m
fn select_1112([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < c {
        select_1111([a, d, b, g, e, c, h, i, f, k, m, j])
    } else {
        select_1111([a, d, c, f, e, b, h, i, g, j, l, k])
    }
}
/// n = 9, i = 4, a < f, a < g, b < e, b < g, b < i, c < d, c < f, c < h, f < i, g < h
fn select_1113([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_594([d, b, a, f, e, g, h, i])
    } else {
        select_594([e, c, a, g, d, f, i, h])
    }
}
/// n = 11, i = 4, a < g, b < e, b < g, b < h, c < d, c < f, c < k, e < j, f < h, f < j, g < k, h < i
fn select_1114([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_632([d, b, f, a, e, h, i, j, g])
    } else {
        select_1113([a, b, c, d, e, f, g, k, h])
    }
}
/// n = 11, i = 4, a < h, b < e, b < h, b < i, c < d, c < f, c < k, f < g, f < i, h < k, i < j
fn select_1115([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_630([d, a, f, b, g, i, h, j, e, k])
    } else {
        select_1114([a, b, c, d, e, f, h, i, j, g, k])
    }
}
/// n = 12, i = 4, a < d, a < f, a < j, b < c, b < e, b < i, e < g, e < j, f < h, f < i, i < k, j < l
fn select_1116([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_639([c, a, e, g, d, j, f, h, i, l])
    } else {
        select_639([d, b, f, h, c, i, e, g, j, k])
    }
}
/// n = 12, i = 4, b < e, b < f, b < i, c < d, c < g, c < j, f < j, g < h, g < i, i < k, j < l
fn select_1117([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < f {
        select_1115([a, b, c, d, e, g, h, f, i, k, j])
    } else {
        select_1116([b, c, d, e, g, f, h, a, j, i, l, k])
    }
}
/// n = 9, i = 4, a < e, b < h, c < g, c < h, d < f, d < h, f < g, h < i
fn select_1118([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_406([a, c, b, d, e, f, h, i, g])
    } else {
        select_346([a, c, f, e, b, g, h])
    }
}
/// n = 9, i = 4, a < g, b < h, c < f, c < h, d < e, d < h, h < i
fn select_1119([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_1118([a, b, c, d, g, e, f, h, i])
    } else {
        select_1118([a, b, d, c, g, f, e, h, i])
    }
}
/// n = 8, i = 3, a < f, b < e, b < f, b < g, b < h, c < d, c < e, d < g, d < h
fn select_1120([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_19([e, d, g, h, a])
    } else {
        select_888([a, b, c, d, f, g, h, e])
    }
}
/// n = 8, i = 3, b < e, b < f, c < d, c < f, c < g, c < h, e < g, e < h
fn select_1121([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_125([d, b, f, a, e, g, h])
    } else {
        select_1120([a, c, b, e, f, d, g, h])
    }
}
/// n = 10, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, c < j, e < j, g < h, g < i
fn select_1122([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_65([a, d, b, f, g, e, h, i])
    } else {
        select_1121([a, b, c, d, e, f, g, j])
    }
}
/// n = 12, i = 5, a < g, a < h, b < e, b < g, b < j, c < d, c < h, e < f, e < l, g < k, h < i, h < j, h < k, h < l
fn select_1123([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_1122([d, b, h, i, g, j, e, f, l, k])
    } else {
        select_93([a, c, e, f, d, g, h, i, j, l])
    }
}
/// n = 12, i = 5, a < h, a < i, b < e, b < i, c < d, c < f, c < h, f < g, f < l, h < k, i < j, i < k, i < l
fn select_1124([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < i {
        select_1119([f, d, a, b, e, h, g, i, j])
    } else {
        select_1123([a, c, b, e, f, g, h, i, j, d, k, l])
    }
}
/// n = 13, i = 5, a < j, b < e, b < g, c < d, c < f, c < j, f < h, f < k, g < i, g < k, g < m, j < m, k < l
fn select_1125([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_1117([e, g, c, d, i, a, f, h, k, j, l, m])
    } else {
        select_1124([a, b, c, d, e, f, h, j, g, i, m, k])
    }
}
/// n = 14, i = 5, b < e, b < h, c < d, c < f, c < g, f < l, g < i, g < k, h < j, h < k, h < l, k < m, l < n
fn select_1126([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < a {
        select_1112([d, f, g, b, e, i, a, h, j, k, l, m, n])
    } else {
        select_1125([a, b, c, d, e, g, h, i, j, f, k, m, l])
    }
}
/// n = 9, i = 3, a < g, b < f, b < g, b < h, c < d, c < f, d < e, d < h, g < i
fn select_1127([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_51([b, d, a, e, f, h])
    } else {
        select_273([b, c, a, f, d, g, e, i])
    }
}
/// n = 9, i = 4, a < f, a < g, b < e, b < i, c < d, c < f, d < h, e < g, e < h, f < i
fn select_1128([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_392([d, a, b, f, e, h, i])
    } else {
        select_273([a, e, c, g, d, f, h, i])
    }
}
/// n = 9, i = 4, a < h, b < f, c < f, c < g, d < e, d < i, e < g, e < h, f < i
fn select_1129([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_690([c, d, a, b, e, h, g, f])
    } else {
        select_1128([c, d, b, a, e, f, g, h, i])
    }
}
/// n = 10, i = 4, b < g, c < g, c < h, c < i, d < e, d < j, e < f, e < h, f < i, g < j
fn select_1130([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_1127([b, c, e, f, a, h, g, i, j])
    } else {
        select_1129([a, b, c, d, e, g, h, f, j])
    }
}
/// n = 12, i = 4, b < f, c < f, c < g, c < h, d < e, d < l, e < g, e < h, f < l, h < i, h < j, h < k
fn select_1131([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < b {
        select_834([a, b, c, g, h, i, j, k, f])
    } else {
        select_532([a, d, c, f, e, g, h, l])
    }
}
/// n = 12, i = 4, b < g, c < g, c < h, c < i, d < e, d < l, e < f, e < h, e < i, g < l, i < j, i < k
fn select_1132([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_1130([a, b, c, d, e, f, g, h, i, l])
    } else {
        select_1131([a, b, c, d, e, g, h, i, f, j, k, l])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < i, c < h, d < e, d < f, d < g
fn select_1133([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_15([a, e, f, g])
    } else {
        select_19([c, b, d, i, h])
    }
}
/// n = 10, i = 2, a < d, b < c, b < e, c < i, d < j, e < f, e < g, e < h
fn select_1134([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_56([c, e, f, g, h, a, i])
    } else {
        select_1133([a, b, d, e, f, g, h, j, c])
    }
}
/// n = 11, i = 2, a < c, b < d, b < e, c < k, d < i, d < j, e < f, e < g, e < h
fn select_1135([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_220([d, e, f, g, h, a, i, j])
    } else {
        select_1133([a, b, c, e, f, g, h, k, d])
    }
}
/// n = 11, i = 2, a < e, b < c, b < d, b < f, d < j, e < k, f < g, f < h, f < i
fn select_1136([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_1134([a, b, c, e, f, g, h, i, d, k])
    } else {
        select_1135([a, b, e, d, f, g, h, i, c, j, k])
    }
}
/// n = 8, i = 2, a < d, a < f, b < c, b < e, e < g, f < h
fn select_1137([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_46([a, b, c, d, e, g])
    } else {
        select_46([b, a, d, c, f, h])
    }
}
/// n = 10, i = 3, a < j, b < d, b < g, b < j, c < e, c < g, c < j, e < f, f < h, g < i
fn select_1138([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_1137([b, e, a, d, f, g, h, i])
    } else {
        select_942([a, b, c, d, g, e, i, f, j])
    }
}
/// n = 12, i = 3, b < d, b < g, b < h, c < e, c < g, c < h, e < f, f < i, g < l, h < j, h < k
fn select_1139([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_1136([e, b, d, g, f, h, a, j, k, l, i])
    } else {
        select_1138([a, b, c, d, e, f, g, i, l, h])
    }
}
/// n = 9, i = 3, a < i, b < d, b < g, b < h, c < e, e < f, e < g, e < h, e < i
fn select_1140([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_86([f, a, b, d, g, h, i])
    } else {
        select_79([a, d, c, e, f, g, h])
    }
}
/// n = 10, i = 4, a < e, a < i, a < j, b < f, b < g, c < d, c < g, f < h, g < h, g < i, g < j
fn select_1141([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < g {
        select_320([a, c, b, f, e, d, g])
    } else {
        select_1140([f, a, c, e, g, d, i, j, h])
    }
}
/// n = 13, i = 4, a < f, b < e, b < h, b < i, c < d, c < g, f < m, g < h, g < i, g < m, h < l, i < j, i < k
fn select_1142([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_1139([d, b, g, e, a, f, h, i, m, j, k, l])
    } else {
        select_1141([b, a, c, d, e, f, g, m, h, i])
    }
}
/// n = 14, i = 5, a < h, b < e, b < h, b < k, c < d, c < f, e < g, e < l, f < i, f < l, f < n, h < n, i < j, i < k, l < m
fn select_1143([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < e {
        select_1132([d, a, b, f, i, j, h, k, e, g, l, n])
    } else {
        select_1142([a, e, c, d, g, h, f, l, i, j, k, m, n])
    }
}
/// n = 9, i = 3, b < e, b < h, b < i, c < d, c < f, f < g, f < h, f < i
fn select_1144([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_65([a, d, f, g, b, e, h, i])
    } else {
        select_69([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, b < e, b < i, c < d, c < h, d < f, d < i, d < j, e < g, e < h, e < j
fn select_1145([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_65([a, f, b, i, e, g, h, j])
    } else {
        select_65([a, g, c, h, d, f, i, j])
    }
}
/// n = 13, i = 5, a < e, a < k, b < c, b < d, d < g, d < l, d < m, e < f, e < h, e < l, g < i, g < k, h < j, h < m
fn select_1146([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_1144([f, h, b, c, j, d, l, g, m])
    } else {
        select_1145([c, a, d, g, e, i, f, l, k, h])
    }
}
/// n = 10, i = 3, b < e, b < i, b < j, c < d, c < f, f < g, f < i, f < j, g < h
fn select_1147([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_1072([a, d, f, g, b, e, i, j, h])
    } else {
        select_69([a, b, c, d, e, f, g, i, j])
    }
}
/// n = 11, i = 4, b < e, b < h, c < d, c < i, d < g, d < h, d < k, e < f, e < i, e < k, i < j
fn select_1148([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_65([a, g, b, h, e, f, i, k])
    } else {
        select_1072([a, f, c, i, d, g, h, k, j])
    }
}
/// n = 14, i = 5, a < e, a < k, b < c, b < d, d < g, d < l, d < n, e < f, e < h, e < l, g < i, g < k, h < j, h < n, l < m
fn select_1149([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_1147([f, h, b, c, j, d, l, m, g, n])
    } else {
        select_1148([c, a, d, g, e, f, i, k, l, m, h])
    }
}
/// n = 14, i = 5, a < d, a < e, a < k, b < c, b < f, d < h, d < l, e < g, e < m, f < i, f < l, f < m, i < j, i < k, m < n
fn select_1150([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < e {
        select_1146([a, b, c, f, d, h, i, e, j, g, k, l, m])
    } else {
        select_1149([a, b, c, f, e, g, i, d, j, h, k, m, n, l])
    }
}
/// n = 14, i = 5, b < e, b < f, b < k, c < d, c < g, e < l, f < h, f < m, g < i, g < l, g < m, i < j, i < k, m < n
fn select_1151([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < e {
        select_1143([a, b, c, d, f, g, h, e, i, j, k, m, n, l])
    } else {
        select_1150([b, c, d, e, f, g, h, a, i, j, k, l, m, n])
    }
}
/// n = 8, i = 4, a < f, a < g, b < e, b < h, c < d, c < f, e < g, f < h
fn select_1152([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_488([e, a, c, d, g, f, h])
    } else {
        select_30([d, b, a, f, e, h])
    }
}
/// n = 8, i = 4, a < f, a < g, b < e, b < g, c < d, c < f, c < h, g < h
fn select_1153([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_1152([a, c, b, e, d, g, f, h])
    } else {
        select_375([c, b, e, g, f, d, h])
    }
}
/// n = 9, i = 3, a < d, a < g, a < h, a < i, b < c, b < e, b < g, d < f, e < h, e < i
fn select_1154([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_55([c, e, a, g, h, i, d])
    } else {
        select_110([d, b, g, f, e, h, i, c])
    }
}
/// n = 10, i = 4, a < d, b < f, b < g, c < e, c < g, c < h, e < i, e < j, f < h, f < i, f < j
fn select_1155([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_1153([b, a, c, e, d, g, f, h])
    } else {
        select_1154([f, c, g, a, e, d, h, i, j])
    }
}
/// n = 10, i = 4, c < f, c < g, d < e, d < g, d < h, e < i, e < j, f < h, f < i, f < j
fn select_1156([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1155([a, c, d, b, e, f, g, h, i, j])
    } else {
        select_1155([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 9, i = 3, c < e, c < f, d < e, d < f, f < g, f < h, f < i
fn select_1157([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < c {
        select_194([b, c, d, e, f])
    } else {
        select_65([a, b, d, e, f, g, h, i])
    }
}
/// n = 7, i = 3, a < g, b < f, c < e, c < f, c < g, d < e, d < f, d < g
fn select_1158([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_89([c, d, a, b, e, g])
    } else {
        select_89([c, d, b, a, e, f])
    }
}
/// n = 9, i = 3, b < i, c < e, c < f, c < i, d < e, d < f, d < i, f < g, f < h
fn select_1159([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_86([e, b, f, a, g, h, i])
    } else {
        select_1158([a, b, c, d, e, i, f])
    }
}
/// n = 11, i = 3, c < e, c < f, c < g, d < e, d < f, d < g, f < j, f < k, g < h, g < i
fn select_1160([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_222([b, e, g, f, a, j, k, h, i])
    } else {
        select_1159([b, a, c, d, e, g, h, i, f])
    }
}
/// n = 12, i = 4, c < f, c < h, d < e, d < g, d < i, e < h, e < l, f < g, f < i, f < l, i < j, i < k
fn select_1161([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_1157([a, b, c, e, h, f, g, i, l])
    } else {
        select_1160([a, b, d, f, g, e, i, j, k, h, l])
    }
}
/// n = 12, i = 4, c < g, c < i, d < h, d < j, e < f, e < g, f < h, f < i, f < j, j < k, j < l
fn select_1162([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < d {
        select_1156([a, b, e, c, d, f, g, i, h, j])
    } else {
        select_1161([a, b, e, d, c, f, h, g, j, k, l, i])
    }
}
/// n = 7, i = 1, a < g, b < c, b < d, b < e, b < f
fn select_1163([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_2([b, g])
    } else {
        select_16([a, c, d, e, f])
    }
}
/// n = 10, i = 3, a < d, a < j, b < e, b < j, c < e, c < j, e < f, e < g, e < h, e < i
fn select_1164([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_16([a, f, g, h, i])
    } else {
        select_194([d, b, c, e, j])
    }
}
/// n = 10, i = 3, a < d, b < e, b < f, c < e, c < f, f < g, f < h, f < i, f < j
fn select_1165([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_1163([e, f, g, h, i, j, a])
    } else {
        select_1164([a, b, c, d, f, g, h, i, j, e])
    }
}
/// n = 10, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, g < h, g < i, g < j
fn select_1166([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < g {
        select_346([a, b, c, e, d, f, g])
    } else {
        select_1165([a, b, c, e, f, g, d, h, i, j])
    }
}
/// n = 9, i = 2, a < i, b < h, b < i, c < d, c < e, c < h, d < g, e < f
fn select_1167([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_38([b, e, d, a, g, f, h])
    } else {
        select_602([a, b, c, e, f, h, d, i])
    }
}
/// n = 9, i = 3, a < d, a < i, b < e, b < i, c < e, c < h, c < i, d < h, e < f, e < g
fn select_1168([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_19([a, e, f, g, i])
    } else {
        select_18([b, d, c, h, i])
    }
}
/// n = 10, i = 3, a < d, a < j, b < e, b < f, b < j, c < e, c < f, c < i, c < j, d < i, e < h, f < g
fn select_1169([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_1168([a, b, c, d, e, f, h, i, j])
    } else {
        select_1168([a, b, c, d, f, e, g, i, j])
    }
}
/// n = 10, i = 3, a < e, a < j, b < f, b < g, b < j, c < d, c < f, c < g, c < j, f < i, g < h
fn select_1170([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_1167([d, a, b, f, g, h, i, j, e])
    } else {
        select_1169([a, b, c, e, f, g, h, i, d, j])
    }
}
/// n = 12, i = 3, a < e, b < f, b < g, b < h, c < d, c < f, c < g, c < h, f < l, g < k, h < i, h < j
fn select_1171([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_934([d, f, g, h, a, i, j, k, l])
    } else {
        select_1170([a, b, c, d, e, f, g, k, l, h])
    }
}
/// n = 13, i = 4, a < d, b < f, b < i, c < e, c < j, c < k, e < h, e < i, f < g, f < j, f < k, j < m, k < l
fn select_1172([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_1166([a, b, e, h, d, i, f, g, j, k])
    } else {
        select_1171([a, c, f, g, d, j, k, e, h, i, l, m])
    }
}
/// n = 13, i = 4, c < f, c < i, d < e, d < j, d < k, e < h, e < i, f < g, f < j, f < k, j < m, k < l
fn select_1173([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_1172([a, c, d, b, e, f, g, h, i, j, k, l, m])
    } else {
        select_1172([b, c, d, a, e, f, g, h, i, j, k, l, m])
    }
}
/// n = 14, i = 5, c < e, c < f, c < m, d < g, d < j, e < l, f < j, f < k, g < h, g < k, g < l, h < i, h < m, m < n
fn select_1174([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < h {
        select_1162([a, b, f, e, d, g, j, l, k, h, i, m])
    } else {
        select_1173([a, b, g, c, f, h, i, j, k, e, m, n, l])
    }
}
/// n = 15, i = 6, b < f, b < l, c < d, c < e, e < g, e < i, e < n, f < h, f < j, f < m, i < l, i < m, j < k, j < n, n < o
fn select_1175([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < g {
        select_1151([a, f, c, d, h, j, e, k, i, l, m, g, n, o])
    } else {
        select_1174([a, d, e, b, g, i, f, j, k, l, m, h, n, o])
    }
}
/// n = 15, i = 6, b < d, b < e, c < f, c < m, e < i, e < j, e < n, f < g, f < h, f < k, j < m, k < l, k < n, n < o
fn select_1176([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < j {
        select_1126([a, b, f, h, d, g, k, e, l, i, n, j, o, m])
    } else {
        select_1175([a, c, b, d, e, f, i, h, j, k, l, m, g, n, o])
    }
}
/// n = 17, i = 8, b < g, b < p, c < k, c < l, d < i, e < f, e < h, h < j, h < k, h < p, i < l, i < o, k < o, l < m, l < n, l < p, p < q
fn select_1177([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if b < l {
        select_1089([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q])
    } else {
        select_1176([a, e, i, f, h, l, m, n, j, k, b, g, o, p, q])
    }
}
/// n = 10, i = 4, a < e, b < f, c < f, d < f, f < g, f < h, f < i, f < j
fn select_1178([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_16([a, g, h, i, j])
    } else {
        select_136([e, b, c, d, f])
    }
}
/// n = 10, i = 4, c < f, d < f, e < f, f < g, f < h, f < i, f < j
fn select_1179([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1178([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_1178([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 12, i = 4, c < h, c < l, d < f, d < h, e < f, e < h, f < g, f < l, h < i, h < j, h < k
fn select_1180([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_239([a, b, c, d, e, f, g, h, l])
    } else {
        select_1179([a, b, c, d, e, h, f, i, j, k])
    }
}
/// n = 13, i = 5, c < h, c < j, d < g, e < f, e < j, f < h, f < i, g < i, g < j, j < k, j < l, j < m
fn select_1181([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_870([a, b, c, d, f, h, g, i, j])
    } else {
        select_1180([a, b, c, e, g, f, i, j, k, l, m, h])
    }
}
/// n = 9, i = 5, a < e, b < g, c < g, c < h, d < f, f < h, f < i, g < i
fn select_1182([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < h {
        select_966([b, a, c, d, e, f, g, i, h])
    } else {
        select_698([a, b, f, h, g, i, e])
    }
}
/// n = 9, i = 5, c < g, d < g, d < h, e < f, f < h, f < i, g < i
fn select_1183([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1182([a, c, d, e, b, f, g, h, i])
    } else {
        select_1182([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 13, i = 6, c < g, d < h, e < f, e < j, f < i, g < j, h < i, h < j, j < k, j < l, j < m
fn select_1184([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_1006([a, b, e, g, d, h, f, j, k, l, m, i])
    } else {
        select_1183([a, b, c, f, d, h, g, i, j])
    }
}
/// n = 14, i = 6, c < f, c < k, d < g, e < h, g < j, g < k, h < i, h < k, i < j, k < l, k < m, k < n
fn select_1185([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < f {
        select_1181([a, b, c, d, h, i, g, f, j, k, l, m, n])
    } else {
        select_1184([a, b, d, e, c, f, g, h, i, k, l, m, n])
    }
}
/// n = 11, i = 4, a < e, a < g, b < d, b < h, c < f, f < g, f < h, h < i, h < j, h < k
fn select_1186([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_320([b, c, a, e, d, f, g])
    } else {
        select_92([a, f, b, d, g, h, i, j, k])
    }
}
/// n = 12, i = 5, a < g, b < e, b < h, c < g, c < h, d < f, f < h, f < l, g < l, h < i, h < j, h < k
fn select_1187([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_244([b, a, c, d, e, f, g, l])
    } else {
        select_554([a, e, c, f, h, i, j, k])
    }
}
/// n = 12, i = 5, b < e, b < i, c < g, d < f, d < i, f < h, g < h, g < i, i < j, i < k, i < l
fn select_1188([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_1186([f, b, c, e, a, g, h, i, j, k, l])
    } else {
        select_1187([a, b, d, c, e, g, f, i, j, k, l, h])
    }
}
/// n = 10, i = 6, a < i, b < h, b < i, c < h, c < i, d < g, d < j, e < f, e < i, g < h, i < j
fn select_1189([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < h {
        select_534([f, a, b, c, d, g, i, j])
    } else {
        select_418([a, b, c, g, e, f, h])
    }
}
/// n = 11, i = 7, a < i, b < i, b < j, c < h, d < g, d < k, e < f, e < i, g < j, h < j, h < k, i < k
fn select_1190([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_1189([a, b, h, d, e, f, g, j, i, k])
    } else {
        select_373([f, c, i, d, g, h, k])
    }
}
/// n = 14, i = 7, b < i, b < j, c < h, d < g, d < k, e < f, e < i, g < j, h < j, h < k, i < k, k < l, k < m, k < n
fn select_1191([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_1188([f, i, c, d, a, g, h, j, k, l, m, n])
    } else {
        select_1190([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 13, i = 5, b < h, b < j, c < f, c < j, d < e, d < g, f < h, f < i, g < i, g < j, j < k, j < l, j < m
fn select_1192([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < a {
        select_730([e, b, g, c, a, f, i, h, j, k, l, m])
    } else {
        select_734([a, b, d, c, f, g, e, h, i, j, k, l, m])
    }
}
/// n = 15, i = 7, b < g, b < l, c < i, c < j, d < h, e < f, e < i, h < j, h < l, i < k, i < l, j < k, l < m, l < n, l < o
fn select_1193([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < j {
        select_1191([a, c, d, b, e, f, g, h, i, j, l, m, n, o])
    } else {
        select_1192([a, b, h, e, f, j, i, g, k, l, m, n, o])
    }
}
/// n = 15, i = 7, b < g, b < l, c < h, d < i, d < k, e < f, e < i, h < k, h < l, i < j, i < l, l < m, l < n, l < o
fn select_1194([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < k {
        select_1185([a, f, b, c, d, g, h, i, j, k, l, m, n, o])
    } else {
        select_1193([a, b, d, c, e, f, g, h, i, k, j, l, m, n, o])
    }
}
/// n = 15, i = 7, b < i, b < j, c < g, c < l, d < f, d < i, e < h, h < j, h < l, i < k, i < n, i < o, l < m, l < n, l < o
fn select_1195([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if l < i {
        select_747([a, b, d, h, c, g, f, j, l, m, i, k, n, o])
    } else {
        select_1194([a, c, e, b, d, f, g, h, i, k, j, l, m, n, o])
    }
}
/// n = 15, i = 7, b < g, b < o, c < j, c < l, d < f, d < j, e < h, h < i, h < l, i < k, i < n, i < o, j < m, j < n, j < o
fn select_1196([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < b {
        select_818([a, c, d, h, f, i, k, l, b, j, g, m, n, o])
    } else {
        select_1195([a, c, b, d, e, f, g, h, j, l, m, i, k, n, o])
    }
}
/// n = 15, i = 7, b < g, b < o, c < i, c < m, d < f, d < i, e < h, h < j, h < m, i < n, i < o, j < k, j < l, j < o
fn select_1197([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < i {
        select_721([a, b, c, d, h, f, g, j, l, m, k, i, n])
    } else {
        select_1196([a, b, c, d, e, f, g, h, j, i, l, m, n, k, o])
    }
}
/// n = 17, i = 8, b < g, b < p, c < f, c < i, d < j, e < h, e < l, h < o, i < k, i < p, j < l, j < o, l < m, l < n, l < p, p < q
fn select_1198([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if i < h {
        select_1177([a, b, e, d, c, f, g, i, j, k, h, l, m, n, o, p, q])
    } else {
        select_1197([a, b, h, c, d, f, g, j, i, l, m, n, o, k, p])
    }
}
/// n = 8, i = 3, a < f, a < h, b < e, b < h, c < d, c < f, c < g, c < h, e < g
fn select_1199([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_51([b, c, d, f, e, h])
    } else {
        select_55([e, a, c, d, f, h, g])
    }
}
/// n = 10, i = 5, b < h, b < j, c < f, c < j, d < g, e < g, f < i, g < h, g < i, g < j
fn select_1200([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_1199([b, c, g, a, f, h, i, j])
    } else {
        select_978([b, c, a, d, e, f, g, h])
    }
}
/// n = 12, i = 5, b < h, b < j, c < g, c < l, d < f, d < h, e < g, e < h, f < k, g < j, g < k, h < i, h < l
fn select_1201([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_239([a, f, c, b, e, h, i, g, l])
    } else {
        select_1200([a, b, d, c, e, f, g, j, k, h])
    }
}
/// n = 10, i = 4, a < g, a < h, a < j, b < d, b < j, c < e, c < f, d < f, d < g, e < h, e < j, h < i
fn select_1202([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_73([a, c, e, f, g])
    } else {
        select_601([a, b, d, h, f, i, j])
    }
}
/// n = 8, i = 4, a < f, a < h, b < e, b < f, c < d, c < e, d < h, f < g
fn select_1203([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_426([b, c, d, f, g, h, e])
    } else {
        select_30([d, a, b, e, h, f])
    }
}
/// n = 10, i = 5, a < f, b < e, b < h, c < d, c < g, d < i, d < j, e < g, e < i, f < h, f < j
fn select_1204([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_808([a, d, b, e, g, f, i, j, h])
    } else {
        select_1203([c, f, b, e, h, d, j, g])
    }
}
/// n = 11, i = 5, a < e, a < k, b < d, b < i, b < k, c < f, c < h, d < h, e < i, e < j, f < g, f < j, g < k
fn select_1205([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_1202([b, a, f, e, g, j, i, d, h, k])
    } else {
        select_1204([a, b, c, f, d, e, h, i, g, j])
    }
}
/// n = 8, i = 5, b < g, b < h, c < f, d < e, e < g, f < h
fn select_1206([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < h {
        select_27([a, b, e, c, f, g])
    } else {
        select_27([a, b, f, d, e, h])
    }
}
/// n = 9, i = 5, a < i, b < g, b < h, c < f, c < i, d < e, d < g, f < h, g < i
fn select_1207([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_665([a, d, c, f, g, e, h, i])
    } else {
        select_1206([a, b, c, d, e, f, g, h])
    }
}
/// n = 10, i = 5, a < i, a < j, b < f, b < j, c < g, c < h, c < i, d < e, d < g, e < i, f < h, g < j
fn select_1208([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < i {
        select_1207([a, c, b, d, e, f, g, h, j])
    } else {
        select_190([b, g, e, a, i, j])
    }
}
/// n = 12, i = 6, a < h, a < l, b < e, b < k, b < l, c < h, c < i, c < j, d < f, d < j, e < j, f < g, f < i, h < k, i < l
fn select_1209([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_1205([a, b, d, e, h, f, i, j, k, g, l])
    } else {
        select_1208([a, b, c, f, g, e, i, j, h, l])
    }
}
/// n = 15, i = 7, a < j, a < m, b < f, b < o, c < e, c < l, c < m, d < g, d < k, e < k, f < j, f < k, g < h, g < i, i < m, j < l, m < n, m < o
fn select_1210([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_1201([h, a, b, c, i, e, f, m, n, j, k, o])
    } else {
        select_1209([a, c, f, d, e, g, h, j, i, k, l, m])
    }
}
/// n = 10, i = 4, a < g, a < h, b < e, c < d, c < f, d < h, d < i, e < f, e < g, e < i, h < j
fn select_1211([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_46([a, c, f, g, d, i])
    } else {
        select_260([a, b, h, e, f, i, j])
    }
}
/// n = 11, i = 5, a < i, a < j, b < g, c < g, c < j, d < e, d < h, e < f, e < j, g < h, g < i, j < k
fn select_1212([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_1211([a, b, d, e, g, h, i, j, f, k])
    } else {
        select_584([b, f, a, c, j, k, g])
    }
}
/// n = 10, i = 6, a < g, b < g, c < f, d < e, e < h, e < i, f < h, f < j, g < i, g < j
fn select_1213([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_423([g, c, d, e, f, i, j])
    } else {
        select_24([a, b, h])
    }
}
/// n = 11, i = 6, a < g, b < g, b < h, c < f, d < e, d < j, e < i, e < k, f < h, f < i, g < k, h < j
fn select_1214([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_1213([a, b, c, d, e, f, g, i, k, h])
    } else {
        select_298([a, b, d, e, h, i, j, g])
    }
}
/// n = 14, i = 7, a < i, a < m, b < e, c < g, d < f, d < k, e < i, e < k, f < j, f < l, f < m, g < h, g < j, h < k, h < m, i < l, m < n
fn select_1215([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < e {
        select_1212([a, b, h, d, f, j, e, k, i, m, n])
    } else {
        select_1214([a, e, c, d, f, g, i, h, j, k, l])
    }
}
/// n = 10, i = 5, a < f, b < g, b < h, b < i, c < f, c < h, d < e, d < h, e < g, e < j, f < i, h < j
fn select_1216([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_422([a, b, d, e, f, h, i, j])
    } else {
        select_80([a, c, e, g, h, j])
    }
}
/// n = 11, i = 6, a < h, b < g, b < h, c < f, c < j, d < e, d < g, e < i, e < j, f < i, f < k, g < j, h < k
fn select_1217([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1216([a, f, b, d, e, h, i, g, k, j])
    } else {
        select_345([a, e, c, g, h, f, j, i])
    }
}
/// n = 15, i = 8, a < i, a < n, b < f, c < h, c < k, d < h, d < j, e < g, e < k, f < i, f < k, g < j, g < l, h < l, h < m, h < n, i < m, j < n, n < o
fn select_1218([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < k {
        select_1215([a, b, e, c, f, h, g, j, i, l, k, m, n, o])
    } else {
        select_1217([a, f, d, e, g, h, k, i, l, j, m])
    }
}
/// n = 9, i = 5, a < i, b < i, c < f, d < g, e < g, f < h, g < h, g < i
fn select_1219([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_118([b, c, d, e, a, f, g, i])
    } else {
        select_373([b, d, e, f, a, g, h])
    }
}
/// n = 10, i = 6, a < j, b < i, c < g, d < h, d < i, e < f, e < i, f < j, g < h, i < j
fn select_1220([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < h {
        select_1219([a, f, c, b, d, g, i, h, j])
    } else {
        select_241([a, b, d, g, e, f, h, i])
    }
}
/// n = 9, i = 6, a < i, b < h, c < g, d < h, d < i, e < f, e < g
fn select_1221([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < i {
        select_332([f, a, b, d, c, g, h, i])
    } else {
        select_504([c, b, a, e, f, h, i, g])
    }
}
/// n = 12, i = 7, a < j, a < l, b < i, b < k, c < h, c < k, d < g, e < f, e < h, f < k, g < i, g < j, h < l, k < l
fn select_1222([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if k < j {
        select_1220([a, f, d, b, c, h, g, i, k, l])
    } else {
        select_1221([a, b, c, g, e, f, h, i, j])
    }
}
/// n = 15, i = 8, a < j, a < n, b < f, c < i, c < l, d < i, d < k, e < g, e < l, f < j, f < l, g < h, g < k, i < m, i < n, j < m, k < n, n < o
fn select_1223([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_1218([a, b, c, d, e, f, g, i, j, k, l, h, m, n, o])
    } else {
        select_1222([a, c, d, b, g, h, f, k, l, j, i, n])
    }
}
/// n = 16, i = 8, a < l, b < j, b < n, c < f, c < p, d < g, d < k, e < h, e < l, f < j, f < l, g < m, g < n, h < i, h < k, j < m, k < n, n < o, n < p
fn select_1224([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < a {
        select_1210([b, c, g, e, a, f, h, i, k, j, l, m, n, o, p])
    } else {
        select_1223([b, c, a, d, e, f, h, i, g, j, k, l, m, n, o])
    }
}
/// n = 8, i = 3, a < h, b < d, b < f, b < g, c < e, e < f, e < g, e < h
fn select_1225([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_45([d, c, a, e, f, g])
    } else {
        select_20([a, b, c, e, d, f, g, h])
    }
}
/// n = 9, i = 4, b < i, c < f, d < e, d < i, e < g, e < h, f < g, f < h, f < i
fn select_1226([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_121([b, a, d, c, f, e, i])
    } else {
        select_1225([b, e, c, a, f, g, h, i])
    }
}
/// n = 8, i = 5, b < h, c < g, c < h, d < g, d < h, e < f, f < h
fn select_1227([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_230([c, d, e, b, a, f, g, h])
    } else {
        select_501([c, d, a, e, f, b, g])
    }
}
/// n = 11, i = 6, a < k, b < i, c < h, c < i, d < h, d < i, e < f, e < k, f < g, f < j, i < j, i < k
fn select_1228([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < g {
        select_1226([h, a, b, e, f, i, g, j, k])
    } else {
        select_1227([a, b, c, d, f, g, h, i])
    }
}
/// n = 8, i = 3, a < d, a < g, b < c, b < g, b < h, c < f, d < e, d < h
fn select_1229([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_53([a, c, f, g, d])
    } else {
        select_54([b, d, e, g, c, h])
    }
}
/// n = 8, i = 4, a < f, a < g, b < f, b < g, c < d, c < h, d < e, g < h
fn select_1230([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_89([a, b, d, e, f, g])
    } else {
        select_194([f, c, g, d, h])
    }
}
/// n = 9, i = 4, a < g, b < d, b < g, c < e, c < h, d < h, d < i, e < f, e < i
fn select_1231([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_1229([c, d, a, e, f, g, h, i])
    } else {
        select_1230([a, b, c, e, f, g, d, h])
    }
}
/// n = 8, i = 4, a < f, b < g, b < h, c < f, c < g, c < h, d < e, d < g, e < h
fn select_1232([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_121([b, a, c, d, e, f, h])
    } else {
        select_359([a, e, b, c, g, h])
    }
}
/// n = 10, i = 5, a < i, b < g, b < j, c < g, c < i, c < j, d < e, d < i, e < f, e < j, g < h
fn select_1233([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_1231([a, d, b, e, g, h, i, j, f])
    } else {
        select_1232([a, b, c, e, f, i, j, g])
    }
}
/// n = 12, i = 7, a < k, b < i, c < f, c < l, d < i, d < j, e < g, e < k, f < j, g < h, g < l, i < k, i < l
fn select_1234([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_1228([a, b, d, f, e, g, h, j, i, l, k])
    } else {
        select_1233([a, c, i, e, g, h, f, j, k, l])
    }
}
/// n = 16, i = 8, a < m, b < j, b < l, c < g, c < l, d < h, d < m, e < f, e < n, f < j, f < k, g < k, g < n, h < i, h < l, j < m, l < n, n < o, n < p
fn select_1235([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < j {
        select_243([a, g, b, e, h, i, f, l, k, j, n, o, p])
    } else {
        select_1234([a, b, c, f, d, g, h, i, j, k, m, l])
    }
}
/// n = 8, i = 2, c < d, c < e, c < f, e < h, f < g
fn select_1236([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_821([a, b, c, d, e, f, h])
    } else {
        select_821([a, b, c, d, f, e, g])
    }
}
/// n = 9, i = 4, c < g, d < f, d < g, e < f, e < g, g < h, g < i
fn select_1237([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_650([a, c, d, e, b, f, g, h, i])
    } else {
        select_650([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 10, i = 4, c < f, d < g, e < f, e < g, g < h, g < i, i < j
fn select_1238([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < c {
        select_1236([a, b, g, h, c, i, j, f])
    } else {
        select_1237([a, b, d, c, e, f, g, h, i])
    }
}
/// n = 12, i = 5, c < h, c < j, d < g, e < f, e < j, f < h, f < i, g < i, g < j, j < k, j < l
fn select_1239([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_870([a, b, c, d, f, h, g, i, j])
    } else {
        select_238([a, b, c, e, g, f, i, j, k, l, h])
    }
}
/// n = 12, i = 5, c < h, c < i, d < g, d < i, e < f, e < i, f < h, f < j, g < j, g < l, i < k, i < l
fn select_1240([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_1238([a, b, f, d, c, h, i, k, g, l])
    } else {
        select_1239([a, b, c, d, e, f, g, h, j, i, k, l])
    }
}
/// n = 16, i = 8, a < m, b < j, b < p, c < g, c < k, d < h, d < m, e < f, e < n, f < j, f < l, g < l, g < n, h < i, h < k, j < m, k < n, n < o, n < p
fn select_1241([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if b < k {
        select_1235([a, b, c, d, e, f, g, h, i, j, l, k, m, n, o, p])
    } else {
        select_1240([a, i, g, k, e, f, b, l, n, j, o, p])
    }
}
/// n = 16, i = 8, a < m, b < g, b < p, c < f, c < n, d < h, d < k, e < i, e < m, f < l, g < m, h < l, h < n, i < j, i < k, k < n, n < o, n < p
fn select_1242([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < f {
        select_1224([a, c, b, d, e, g, h, i, j, f, k, m, l, n, o, p])
    } else {
        select_1241([a, b, d, e, c, f, h, i, j, g, k, l, m, n, o, p])
    }
}
/// n = 9, i = 4, a < d, a < f, b < f, b < g, b < i, c < e, c < i, d < i, e < g, g < h
fn select_1243([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_89([c, d, b, f, e, i])
    } else {
        select_488([a, b, e, d, f, g, h])
    }
}
/// n = 9, i = 4, a < d, a < g, b < f, b < g, b < h, c < e, c < f, e < h, h < i
fn select_1244([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_1243([a, b, c, d, e, g, h, i, f])
    } else {
        select_487([f, e, a, g, h, i, d])
    }
}
/// n = 9, i = 4, a < f, a < g, b < e, b < h, b < i, c < d, c < f, e < g, f < h, f < i
fn select_1245([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_141([d, b, a, f, e, h, i])
    } else {
        select_886([e, a, c, d, g, f, h, i])
    }
}
/// n = 9, i = 4, a < g, a < h, a < i, b < e, b < h, b < i, c < d, c < f, e < g, f < h, f < i
fn select_1246([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_528([d, f, b, e, a, h, i])
    } else {
        select_1245([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 9, i = 4, a < d, a < i, b < f, b < h, c < e, c < g, c < i, e < h, f < g, f < i
fn select_1247([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_1245([b, c, a, d, e, f, h, g, i])
    } else {
        select_1229([f, c, e, a, d, h, g, i])
    }
}
/// n = 11, i = 5, a < e, a < h, b < g, b < j, b < k, c < f, c < h, c < i, d < f, d < g, d < k, f < j, g < i, h < k
fn select_1248([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_1246([b, d, a, e, f, h, j, g, k])
    } else {
        select_1247([a, b, c, e, f, g, i, j, h])
    }
}
/// n = 11, i = 5, a < h, a < k, b < e, b < g, c < f, c < j, c < k, d < g, d < h, d < i, f < i, g < k, h < j
fn select_1249([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_1244([b, d, f, e, a, i, g, h, j])
    } else {
        select_1248([b, c, d, a, e, h, f, g, i, j, k])
    }
}
/// n = 9, i = 4, b < i, c < h, d < f, e < f, f < g, f < h, f < i
fn select_1250([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_643([a, b, d, e, c, f, g, i])
    } else {
        select_643([a, c, d, e, b, f, g, h])
    }
}
/// n = 12, i = 5, b < g, c < h, c < j, d < f, d < h, e < g, e < h, f < k, f < l, g < j, g < l, h < i, h < k
fn select_1251([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_1250([a, b, f, c, e, h, i, k, g])
    } else {
        select_1200([a, c, d, b, e, f, g, j, l, h])
    }
}
/// n = 14, i = 6, a < h, b < i, b < k, c < e, c < g, d < f, d < k, d < m, f < j, f < n, g < k, h < i, h < j, i < m, k < l, k < n
fn select_1252([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_1249([b, c, d, h, e, f, g, i, j, m, k])
    } else {
        select_1251([e, a, b, d, g, f, h, k, l, i, n, j])
    }
}
/// n = 9, i = 4, a < g, b < e, b < h, c < f, c < g, d < e, d < f, d < i, f < h, g < i
fn select_1253([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_381([b, a, c, d, e, g, f, i])
    } else {
        select_353([a, b, c, e, f, h, g])
    }
}
/// n = 10, i = 5, a < h, a < i, b < g, c < f, c < i, d < e, d < g, e < i, f < h, f < j, g < j
fn select_1254([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_1253([b, a, d, f, h, e, g, i, j])
    } else {
        select_345([b, a, c, e, g, f, i, h])
    }
}
/// n = 8, i = 3, a < d, b < e, c < e, c < h, d < h, e < f, e < g
fn select_1255([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_6([b, c, d, h])
    } else {
        select_9([a, e, f, g])
    }
}
/// n = 9, i = 4, a < f, b < e, b < g, c < f, c < g, d < e, d < g, d < i, f < i, g < h
fn select_1256([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_1255([a, b, d, f, g, e, h, i])
    } else {
        select_696([a, b, c, d, e, f, g, i])
    }
}
/// n = 7, i = 3, a < d, b < e, b < f, c < e, c < f, f < g
fn select_1257([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_26([b, c, a, d, e])
    } else {
        select_198([a, e, f, g])
    }
}
/// n = 10, i = 5, a < g, b < f, b < h, c < f, c < g, d < e, d < i, e < h, e < j, f < i, g < j
fn select_1258([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_1256([a, b, c, e, h, g, f, i, j])
    } else {
        select_1257([a, d, f, g, i, e, h])
    }
}
/// n = 11, i = 4, a < f, b < f, b < h, c < d, c < k, d < e, d < h, f < g, f < k, h < i, h < j
fn select_1259([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_111([a, e, b, h, i, j])
    } else {
        select_11([a, b, c, d, e, f, g, k])
    }
}
/// n = 12, i = 5, a < g, b < i, b < j, c < g, c < j, d < e, d < h, e < f, e < j, g < h, g < i, j < k, j < l
fn select_1260([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_728([b, c, d, g, e, f, h, i, j])
    } else {
        select_1259([a, c, d, e, f, g, i, j, k, l, h])
    }
}
/// n = 14, i = 6, a < e, b < i, b < k, c < f, c < h, d < g, d < j, e < i, e < j, f < j, f < k, g < h, g < k, g < n, i < n, k < l, k < m
fn select_1261([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < f {
        select_1258([b, c, e, d, g, f, i, h, j, n])
    } else {
        select_1260([a, b, f, d, g, h, e, j, i, k, l, m])
    }
}
/// n = 15, i = 7, a < f, b < j, b < l, c < g, c < k, d < h, d < i, e < g, e < h, f < i, f < j, g < l, h < k, h < l, h < o, j < o, l < m, l < n
fn select_1262([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < g {
        select_1254([c, b, e, f, i, h, j, k, g, o])
    } else {
        select_1261([a, b, c, d, f, g, h, k, j, i, l, m, n, o])
    }
}
/// n = 15, i = 7, a < g, b < k, b < l, c < f, c < h, d < i, d < j, e < h, e < i, g < j, g < k, h < l, i < l, i < o, k < o, l < m, l < n
fn select_1263([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_1262([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o])
    } else {
        select_1222([b, d, e, a, c, f, g, h, j, k, i, l])
    }
}
/// n = 15, i = 7, a < j, a < o, b < g, c < k, c < l, d < f, d < i, e < h, e < i, g < j, g < k, h < l, h < n, i < l, k < n, l < m, l < o
fn select_1264([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < a {
        select_1252([b, c, d, h, f, a, i, g, k, j, l, m, n, o])
    } else {
        select_1263([b, c, d, a, e, f, g, i, h, j, k, l, m, o, n])
    }
}
/// n = 10, i = 4, c < h, c < j, d < f, d < h, e < f, e < h, f < g, f < j, h < i
fn select_1265([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < c {
        select_821([a, b, f, g, c, h, j])
    } else {
        select_130([a, b, c, d, e, h, f, g, j, i])
    }
}
/// n = 12, i = 6, b < i, b < k, c < h, d < g, d < k, e < f, e < h, g < i, g < j, h < j, h < k, k < l
fn select_1266([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_242([a, b, c, g, e, f, i, h, j, k])
    } else {
        select_1265([a, f, b, d, h, g, j, k, l, i])
    }
}
/// n = 8, i = 3, b < h, c < f, c < g, d < e, e < f, e < g, e < h
fn select_1267([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_122([b, a, d, c, e, h])
    } else {
        select_1225([b, c, d, a, e, f, g, h])
    }
}
/// n = 10, i = 5, a < j, b < f, b < i, c < h, d < g, d < h, e < g, e < h, h < i, h < j
fn select_1268([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_1267([g, a, b, c, h, f, i, j])
    } else {
        select_1227([a, c, d, e, b, f, g, h])
    }
}
/// n = 8, i = 3, a < d, b < g, b < h, c < e, c < g, e < f, e < h
fn select_1269([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_323([b, e, f, g, a, d, h])
    } else {
        select_613([b, c, a, d, g, e, f])
    }
}
/// n = 9, i = 4, a < f, b < e, b < i, c < g, c < i, d < f, d < g, d < i, g < h
fn select_1270([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_1232([a, c, d, b, e, f, i, g])
    } else {
        select_1269([a, b, c, f, g, h, i, e])
    }
}
/// n = 11, i = 6, a < j, b < f, b < k, c < h, d < g, d < k, e < h, e < i, g < i, h < j, h < k
fn select_1271([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_1268([a, b, c, e, g, f, i, h, k, j])
    } else {
        select_1270([a, b, d, h, f, j, g, i, k])
    }
}
/// n = 14, i = 7, a < l, b < f, b < j, c < i, c < j, d < h, d < j, e < g, e < m, g < i, g < k, h < k, h < m, i < l, j < m, m < n
fn select_1272([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < i {
        select_1266([a, h, c, e, b, f, g, j, k, i, m, n])
    } else {
        select_1271([a, b, c, d, g, f, h, i, k, l, j])
    }
}
/// n = 8, i = 4, a < e, b < e, b < h, c < f, d < f, f < g, f < h
fn select_1273([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_43([b, c, d, e, f, g])
    } else {
        select_198([a, g, b, h])
    }
}
/// n = 11, i = 5, a < h, a < j, b < f, b < g, c < f, c < h, d < e, d < h, e < g, e < k, f < j, f < k, h < i
fn select_1274([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_12([a, c, d, e, f, g, j, k, h])
    } else {
        select_1273([e, b, a, c, g, h, i, f])
    }
}
/// n = 10, i = 5, a < h, a < j, b < f, b < h, c < g, c < h, d < e, e < f, e < g, g < j, h < i
fn select_1275([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_100([a, c, d, e, f, h, i])
    } else {
        select_122([a, b, c, f, g, j])
    }
}
/// n = 9, i = 4, a < f, a < i, b < f, b < g, c < d, d < e, d < g, e < i, f < h
fn select_1276([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_187([a, e, b, g, i])
    } else {
        select_103([a, c, d, f, g, h])
    }
}
/// n = 10, i = 5, a < h, a < i, b < g, b < h, c < f, c < h, d < e, d < f, e < g, f < i, h < j
fn select_1277([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_1275([a, b, c, d, e, g, f, h, j, i])
    } else {
        select_1276([b, a, d, f, e, h, i, j, g])
    }
}
/// n = 11, i = 5, a < h, a < i, b < f, b < h, c < e, c < g, d < g, d < h, f < k, g < i, g < k, h < j
fn select_1278([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_1274([a, c, d, b, f, g, e, h, j, i, k])
    } else {
        select_1277([a, b, d, c, e, g, f, h, i, j])
    }
}
/// n = 7, i = 3, a < g, b < f, b < g, c < d, c < f, d < e
fn select_1279([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_73([b, d, a, e, f])
    } else {
        select_488([b, a, c, f, g, d, e])
    }
}
/// n = 9, i = 3, a < g, a < h, b < d, b < f, c < f, c < g, c < h, d < e, h < i
fn select_1280([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < d {
        select_46([b, a, g, f, h, i])
    } else {
        select_1279([a, c, b, d, e, f, g])
    }
}
/// n = 9, i = 4, a < f, a < i, b < f, b < h, b < i, c < d, d < e, d < i, e < h, f < g
fn select_1281([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_18([a, e, b, h, i])
    } else {
        select_4([f, c, g, d, i])
    }
}
/// n = 10, i = 4, a < f, a < j, b < f, b < h, c < d, d < e, d < h, e < j, f < g, h < i
fn select_1282([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_9([e, h, a, i])
    } else {
        select_1281([b, a, c, d, e, f, g, j, h])
    }
}
/// n = 10, i = 4, a < f, a < h, b < e, b < f, c < d, c < e, d < h, d < j, e < j, f < g, h < i
fn select_1283([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_53([c, f, g, e, d])
    } else {
        select_275([a, b, d, e, h, i, j, f])
    }
}
/// n = 10, i = 4, a < f, a < g, b < g, b < i, c < d, c < f, d < e, d < i, g < h, i < j
fn select_1284([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_1282([a, b, c, d, e, g, h, i, j, f])
    } else {
        select_1283([b, a, c, d, f, g, h, i, j, e])
    }
}
/// n = 10, i = 4, a < g, a < h, b < d, b < h, c < e, c < g, c < i, d < i, e < f, h < j
fn select_1285([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_1280([d, a, c, h, j, g, i, e, f])
    } else {
        select_1284([a, b, c, e, f, g, h, j, d, i])
    }
}
/// n = 11, i = 5, a < e, b < h, b < i, c < f, c < i, d < g, d < i, f < j, g < h, g < j, i < k
fn select_1286([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_1278([b, c, a, d, e, f, g, i, h, k, j])
    } else {
        select_1285([b, c, g, f, a, e, h, i, j, k])
    }
}
/// n = 11, i = 5, a < h, a < j, b < f, b < h, c < g, c < h, d < e, d < k, e < f, e < g, g < j, h < i, h < k
fn select_1287([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_11([a, c, d, e, f, h, i, k])
    } else {
        select_122([a, b, c, f, g, j])
    }
}
/// n = 10, i = 4, a < f, a < i, b < f, b < g, c < d, c < j, d < e, d < g, e < i, f < h, f < j
fn select_1288([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_187([a, e, b, g, i])
    } else {
        select_426([a, c, d, f, h, g, j])
    }
}
/// n = 11, i = 5, a < h, a < i, b < g, b < h, c < f, c < h, d < e, d < f, d < k, e < g, f < i, h < j, h < k
fn select_1289([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_1287([a, b, c, d, e, g, f, h, j, i, k])
    } else {
        select_1288([b, a, d, f, e, h, i, j, g, k])
    }
}
/// n = 9, i = 4, a < e, a < i, b < e, b < g, b < h, c < f, d < f, f < g, f < h, f < i
fn select_1290([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_19([a, b, g, h, i])
    } else {
        select_43([b, c, d, e, f, i])
    }
}
/// n = 9, i = 4, a < e, b < e, b < h, b < i, c < f, d < f, f < g, f < h, f < i
fn select_1291([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_19([g, b, h, i, a])
    } else {
        select_1290([a, b, c, d, e, f, h, i, g])
    }
}
/// n = 12, i = 5, a < h, a < j, b < f, b < g, b < l, c < f, c < h, d < e, d < h, e < g, e < k, f < j, f < k, h < i, h < l
fn select_1292([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_12([a, c, d, e, f, g, j, k, h])
    } else {
        select_1291([e, b, a, c, g, h, i, f, l])
    }
}
/// n = 12, i = 5, a < h, a < i, b < f, b < h, c < e, c < g, c < l, d < g, d < h, f < k, g < i, g < k, h < j, h < l
fn select_1293([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_1289([a, b, d, c, e, g, f, h, i, j, l])
    } else {
        select_1292([a, c, d, b, f, g, e, h, j, i, k, l])
    }
}
/// n = 12, i = 5, a < e, a < l, b < h, b < i, c < f, c < i, d < g, d < i, f < j, g < h, g < j, i < k, i < l
fn select_1294([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_1285([b, c, g, f, a, e, h, i, j, k])
    } else {
        select_1293([b, c, a, d, e, f, g, i, h, k, j, l])
    }
}
/// n = 12, i = 5, b < l, c < h, c < i, d < f, d < i, e < g, e < i, f < j, g < h, g < j, i < k, i < l
fn select_1295([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_1286([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_1294([b, c, d, e, a, f, g, h, i, j, k, l])
    }
}
/// n = 15, i = 7, a < l, a < o, b < j, c < f, c < i, d < h, d < i, e < g, e < m, g < j, g < k, h < k, h < m, i < m, j < l, m < n, m < o
fn select_1296([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if b < i {
        select_1272([a, c, b, d, e, f, g, h, j, i, k, l, m, n])
    } else {
        select_1295([f, a, h, i, e, b, g, k, m, j, n, o])
    }
}
/// n = 15, i = 7, a < k, a < o, b < h, c < g, c < m, d < f, d < j, e < i, e < j, g < l, h < k, i < l, i < m, j < m, m < n, m < o
fn select_1297([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < g {
        select_1264([a, b, c, d, e, f, h, i, j, k, g, m, n, l, o])
    } else {
        select_1296([a, b, d, e, c, f, g, i, j, h, l, k, m, n, o])
    }
}
/// n = 16, i = 8, a < k, b < f, b < p, c < g, c < n, d < h, d < l, e < i, e < k, g < m, h < m, h < n, i < j, i < l, l < n, n < o, n < p
fn select_1298([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < k {
        select_1242([a, b, c, d, e, g, f, h, i, j, l, m, k, n, o, p])
    } else {
        select_1297([b, a, c, i, d, j, g, k, h, l, f, m, n, o, p])
    }
}
/// n = 14, i = 6, a < i, a < k, b < h, b < n, c < e, c < g, d < f, d < k, d < m, f < j, g < k, h < i, h < j, i < m, k < l, k < n
fn select_1299([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_1249([a, c, d, h, e, f, g, i, j, m, k])
    } else {
        select_1201([e, a, b, d, g, f, h, k, l, i, j, n])
    }
}
/// n = 9, i = 4, a < e, b < d, b < g, c < f, c < h, d < h, e < i, f < g, f < i
fn select_1300([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_1257([a, c, d, e, h, f, g])
    } else {
        select_962([a, b, f, e, g, d, h, i])
    }
}
/// n = 8, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_1301([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_260([a, b, f, e, d, g, h])
    } else {
        select_392([a, b, c, d, f, h, e])
    }
}
/// n = 11, i = 4, a < g, a < h, b < f, c < d, c < e, e < g, e < j, f < h, f < j, g < i, h < k
fn select_1302([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_1301([a, c, f, d, e, h, j, k])
    } else {
        select_873([d, a, b, e, f, g, j, i])
    }
}
/// n = 13, i = 6, a < i, a < k, b < f, c < e, c < h, d < g, d < j, e < j, e < k, f < i, f < j, g < h, g < k, g < m, i < m, k < l
fn select_1303([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_1300([a, c, d, e, i, g, h, j, m])
    } else {
        select_1302([a, b, c, h, e, f, k, i, l, j, m])
    }
}
/// n = 14, i = 7, a < j, a < l, b < g, b < k, c < f, d < h, d < i, e < g, e < h, f < i, f < j, g < l, h < k, h < l, h < n, j < n, l < m
fn select_1304([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < i {
        select_1303([a, c, b, d, g, f, h, k, j, i, l, m, n])
    } else {
        select_1254([b, a, e, f, i, h, j, k, g, n])
    }
}
/// n = 14, i = 7, a < k, a < l, b < g, c < f, c < h, d < i, d < j, e < h, e < i, g < j, g < k, h < l, i < l, i < n, k < n, l < m
fn select_1305([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < f {
        select_1304([a, c, b, d, e, g, h, i, j, k, f, l, m, n])
    } else {
        select_1222([a, d, e, b, c, f, g, h, j, k, i, l])
    }
}
/// n = 15, i = 7, a < j, b < k, b < l, c < g, c < o, d < f, d < i, e < h, e < i, g < j, g < k, h < l, h < n, i < l, k < n, l < m, l < o
fn select_1306([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < a {
        select_1299([b, c, d, h, f, a, i, g, k, j, l, m, n, o])
    } else {
        select_1305([b, c, d, a, e, f, g, i, h, j, k, l, m, n])
    }
}
/// n = 15, i = 7, a < l, b < f, b < j, c < i, c < j, d < h, d < j, e < g, e < m, g < i, g < k, h < k, h < m, i < l, j < m, m < n, m < o
fn select_1307([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < i {
        select_243([a, h, c, e, b, f, g, j, k, i, m, n, o])
    } else {
        select_1271([a, b, c, d, g, f, h, i, k, l, j])
    }
}
/// n = 15, i = 7, a < l, b < j, b < o, c < f, c < i, d < h, d < i, e < g, e < m, g < j, g < k, h < k, h < m, i < m, j < l, m < n, m < o
fn select_1308([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if b < i {
        select_1307([a, c, b, d, e, f, g, h, j, i, k, l, m, n, o])
    } else {
        select_1240([a, f, h, i, e, g, b, k, m, j, n, o])
    }
}
/// n = 15, i = 7, a < k, b < h, b < o, c < g, c < m, d < f, d < j, e < i, e < j, g < l, h < k, i < l, i < m, j < m, m < n, m < o
fn select_1309([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < g {
        select_1306([a, c, b, d, e, f, h, i, j, k, g, m, n, l, o])
    } else {
        select_1308([a, b, d, e, c, f, g, i, j, h, l, k, m, n, o])
    }
}
/// n = 15, i = 7, a < h, b < g, b < o, c < i, c < m, d < f, d < k, e < j, e < k, i < l, j < l, j < m, k < m, m < n, m < o
fn select_1310([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < g {
        select_1297([b, a, c, d, e, f, i, h, j, k, g, l, m, n, o])
    } else {
        select_1309([a, b, c, d, e, f, i, g, j, k, h, l, m, n, o])
    }
}
/// n = 16, i = 8, b < g, b < p, c < h, c < n, d < i, d < l, e < f, e < j, h < m, i < m, i < n, j < k, j < l, l < n, n < o, n < p
fn select_1311([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < f {
        select_1298([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o, p])
    } else {
        select_1310([f, b, c, j, d, k, g, a, h, i, l, m, n, o, p])
    }
}
/// n = 6, i = 3, c < f, d < e, e < f
fn select_1312([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_122([c, a, d, b, e, f])
    } else {
        select_122([c, b, d, a, e, f])
    }
}
/// n = 10, i = 5, a < i, a < j, b < h, b < j, c < e, c < j, d < f, e < g, f < h, f < j, h < i
fn select_1313([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_749([a, b, c, h, e, g, i, j])
    } else {
        select_1312([a, g, b, d, f, h])
    }
}
/// n = 10, i = 6, a < j, b < f, c < g, d < g, e < f, e < i, f < h, g < i, g < j
fn select_1314([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_599([a, b, e, i, j, f, h])
    } else {
        select_115([a, h, c, d, g, i])
    }
}
/// n = 11, i = 6, a < j, a < k, b < g, b < k, c < h, c < k, d < f, d < g, e < f, e < h, f < k, g < j, h < i
fn select_1315([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1313([a, b, c, d, h, f, i, g, j, k])
    } else {
        select_1314([a, c, b, d, e, h, g, i, f, j])
    }
}
/// n = 12, i = 4, a < d, a < l, b < h, b < l, c < e, d < g, e < f, e < h, e < l, h < i, h < j, h < k
fn select_1316([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < d {
        select_68([f, a, b, h, i, j, k, l])
    } else {
        select_665([b, c, a, d, e, f, g, l])
    }
}
/// n = 9, i = 3, a < d, b < e, c < e, c < i, d < i, e < f, e < g, e < h
fn select_1317([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_15([a, f, g, h])
    } else {
        select_18([b, d, c, i, e])
    }
}
/// n = 9, i = 3, a < i, b < i, c < e, d < e, d < i, e < f, e < g, e < h
fn select_1318([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1317([a, c, d, b, e, f, g, h, i])
    } else {
        select_1317([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 12, i = 5, a < e, b < g, b < h, c < f, d < f, e < g, e < l, f < h, f < l, h < i, h < j, h < k
fn select_1319([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_137([a, h, i, j, k])
    } else {
        select_117([b, c, d, e, g, f, l])
    }
}
/// n = 13, i = 6, a < f, b < i, b < j, c < g, d < g, e < f, e < j, f < i, g < h, g < j, j < k, j < l, j < m
fn select_1320([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < f {
        select_1318([a, h, b, e, j, k, l, m, f])
    } else {
        select_1319([a, b, c, d, f, g, i, j, k, l, m, h])
    }
}
/// n = 14, i = 6, a < f, a < n, b < g, b < n, c < i, c < k, d < g, d < i, e < f, e < i, f < k, g < h, g < l, g < m, i < j, i < l, i < m, i < n
fn select_1321([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_1316([a, b, d, f, i, j, k, g, h, l, m, n])
    } else {
        select_1320([a, c, b, d, e, f, g, h, k, i, j, l, m])
    }
}
/// n = 14, i = 6, a < f, a < g, b < j, b < l, c < f, c < h, d < h, d < j, e < g, e < j, g < l, h < i, h < m, h < n, j < k, j < m, j < n
fn select_1322([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < j {
        select_1315([b, e, d, a, c, f, g, h, i, l, j])
    } else {
        select_1321([a, c, b, d, e, g, h, i, j, k, l, m, n, f])
    }
}
/// n = 15, i = 7, a < i, a < k, b < h, b < j, c < j, c < k, d < f, d < k, e < g, e < j, f < i, g < h, j < l, j < n, j < o, k < m, k < n, k < o
fn select_1323([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < g {
        select_1322([f, b, a, c, e, i, g, k, m, j, l, h, n, o])
    } else {
        select_1322([g, a, b, c, d, h, f, j, l, k, m, i, n, o])
    }
}
/// n = 15, i = 7, a < i, a < j, b < g, b < k, c < f, c < k, d < j, d < k, e < h, e < j, h < i, j < l, j < n, j < o, k < m, k < n, k < o
fn select_1324([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < g {
        select_1323([a, b, d, e, c, h, f, g, i, k, j, m, l, n, o])
    } else {
        select_1323([a, c, d, e, b, h, g, f, i, k, j, m, l, n, o])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, c < d, c < f, c < g
fn select_1325([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_1048([a, e, c, d, f])
    } else {
        select_108([d, a, b, e, f, g])
    }
}
/// n = 9, i = 3, a < i, b < e, b < g, b < i, c < d, c < g, c < h, c < i, d < f, e < h
fn select_1326([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_1325([a, d, b, g, f, e, i])
    } else {
        select_55([a, e, c, g, d, h, i])
    }
}
/// n = 11, i = 3, b < e, b < g, b < h, c < d, c < g, c < h, c < k, d < f, e < k, h < i, h < j
fn select_1327([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_1106([d, b, g, f, e, h, a, i, j, k])
    } else {
        select_1326([a, b, c, d, e, f, g, k, h])
    }
}
/// n = 9, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, g < h, g < i
fn select_1328([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_396([a, b, c, d, e, f, g])
    } else {
        select_223([e, f, g, a, h, i])
    }
}
/// n = 9, i = 4, b < e, b < h, b < i, c < f, c < g, d < f, d < g, g < h, g < i
fn select_1329([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < g {
        select_1328([a, c, d, b, e, f, g, h, i])
    } else {
        select_79([a, f, g, b, e, h, i])
    }
}
/// n = 12, i = 4, b < g, c < e, c < h, c < i, d < f, d < g, e < l, f < h, f < i, f < l, i < j, i < k
fn select_1330([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_1327([a, c, f, b, e, g, h, i, j, k, l])
    } else {
        select_1329([a, c, b, d, e, g, f, h, i])
    }
}
/// n = 8, i = 3, b < d, b < g, b < h, c < e, e < f, e < g, e < h
fn select_1331([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_144([a, b, e, f, g, h])
    } else {
        select_528([a, c, b, d, e, g, h])
    }
}
/// n = 9, i = 4, a < f, a < h, a < i, b < e, b < g, c < d, c < g, g < h, g < i
fn select_1332([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_1331([e, a, c, f, g, d, h, i])
    } else {
        select_320([a, c, b, e, f, d, g])
    }
}
/// n = 12, i = 4, a < e, b < f, b < h, b < i, c < d, c < g, f < l, g < h, g < i, g < l, i < j, i < k
fn select_1333([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_1327([d, b, g, a, f, e, h, i, j, k, l])
    } else {
        select_1332([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 13, i = 5, a < i, b < f, b < i, c < d, c < e, e < g, e < j, e < k, f < h, f < j, f < k, k < l, k < m
fn select_1334([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_1330([d, a, e, b, g, f, i, j, k, l, m, h])
    } else {
        select_1333([a, f, c, d, i, h, e, j, k, l, m, g])
    }
}
/// n = 14, i = 6, a < i, a < l, b < e, b < i, c < d, c < f, e < l, f < g, f < h, f < m, h < j, h < n, i < k, i < m, i < n
fn select_1335([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_736([d, a, b, f, g, e, h, j, l, i, k, m, n])
    } else {
        select_1334([e, a, c, d, f, i, g, k, l, m, h, j, n])
    }
}
/// n = 15, i = 7, a < g, a < o, b < k, b < l, c < f, c < i, d < i, d < l, e < h, e < l, h < k, i < j, i < n, i < o, l < m, l < n, l < o
fn select_1336([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < i {
        select_1324([b, a, c, d, e, f, g, h, k, l, i, m, j, n, o])
    } else {
        select_1335([b, e, c, f, h, i, j, a, l, g, m, k, n, o])
    }
}
/// n = 9, i = 4, a < f, b < f, b < g, c < e, c < g, d < e, d < g, g < h, g < i
fn select_1337([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_229([a, b, c, d, e, f, g])
    } else {
        select_137([e, g, a, h, i])
    }
}
/// n = 11, i = 6, a < g, b < h, c < h, d < g, d < i, e < f, e < i, h < i, i < j, i < k
fn select_1338([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_1337([a, d, e, h, f, g, i, j, k])
    } else {
        select_533([a, b, c, f, d, g, h, i])
    }
}
/// n = 12, i = 6, a < f, b < g, b < l, c < h, d < h, e < g, e < i, h < i, i < j, i < k, i < l
fn select_1339([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < i {
        select_1338([b, c, d, e, a, f, g, h, i, j, k])
    } else {
        select_649([b, i, j, k, g, a, f, l])
    }
}
/// n = 10, i = 5, a < h, b < g, c < e, d < f, e < h, e < i, f < g, f < i, i < j
fn select_1340([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_105([a, d, c, b, f, e, h, g, i, j])
    } else {
        select_105([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 10, i = 7, a < i, b < h, c < h, d < f, e < g, f < j, g < i, h < j
fn select_1341([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_28([a, h, e, d, g, f, j, i])
    } else {
        select_142([b, c, i, d, f, h])
    }
}
/// n = 12, i = 7, a < i, b < f, c < h, d < h, e < g, g < i, g < k, h < j, j < k, k < l
fn select_1342([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < f {
        select_1340([a, b, e, h, g, j, f, i, k, l])
    } else {
        select_1341([a, c, d, b, e, f, g, h, i, j])
    }
}
/// n = 13, i = 7, a < f, b < j, b < m, c < h, d < h, e < g, g < j, g < k, h < i, h < k, k < l, k < m
fn select_1343([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if k < i {
        select_1339([a, b, c, d, g, f, j, h, k, i, l, m])
    } else {
        select_1342([b, a, c, d, e, f, g, h, j, i, k, l])
    }
}
/// n = 9, i = 4, b < g, c < f, d < g, d < i, e < g, e < i, f < i, g < h
fn select_1344([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_867([a, c, d, e, b, f, g, h])
    } else {
        select_365([a, d, e, f, b, i])
    }
}
/// n = 11, i = 4, b < f, c < i, c < k, d < g, d < i, e < g, e < i, g < h, g < k, i < j
fn select_1345([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1344([a, c, b, d, e, f, i, j, g])
    } else {
        select_131([a, b, c, d, e, g, h, i, j, k, f])
    }
}
/// n = 14, i = 7, a < f, b < j, b < m, c < g, c < m, d < h, e < h, g < j, g < l, h < i, h < k, k < l, k < m, m < n
fn select_1346([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < k {
        select_1343([a, b, d, e, c, f, g, h, i, j, k, l, m])
    } else {
        select_1345([i, a, b, c, k, f, g, l, m, n, j])
    }
}
/// n = 9, i = 4, b < e, c < h, c < i, d < f, f < g, f < h, g < i
fn select_1347([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_115([a, c, d, e, f, g])
    } else {
        select_519([a, b, c, f, g, h, e, i])
    }
}
/// n = 9, i = 4, a < h, a < i, b < e, b < g, c < f, d < f, f < g, f < h, f < i
fn select_1348([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_136([a, c, d, e, f])
    } else {
        select_39([a, b, f, e, g, h, i])
    }
}
/// n = 10, i = 4, a < e, a < j, b < g, b < i, c < f, c < g, d < f, d < g, f < i, f < j, g < h
fn select_1349([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_593([a, b, c, d, e, g, f, h])
    } else {
        select_1348([b, a, c, d, e, f, j, g, i])
    }
}
/// n = 10, i = 4, b < j, c < g, c < i, d < f, d < g, e < f, e < g, f < i, f < j, g < h
fn select_1350([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_406([a, c, d, e, b, f, g, h, i])
    } else {
        select_1349([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 10, i = 4, b < f, c < h, c < j, d < g, d < h, e < g, e < h, g < j, h < i
fn select_1351([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_1344([a, c, b, d, e, f, h, i, g])
    } else {
        select_1350([a, b, c, d, e, g, h, i, j, f])
    }
}
/// n = 10, i = 4, b < f, c < i, d < h, d < i, e < g, e < i, g < h, i < j
fn select_1352([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < c {
        select_1347([a, b, d, e, f, g, c, h, i])
    } else {
        select_1351([a, b, d, c, e, f, g, i, j, h])
    }
}
/// n = 14, i = 7, a < f, b < j, b < m, c < g, c < m, d < h, e < h, g < j, h < i, h < k, k < l, k < m, m < n
fn select_1353([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < l {
        select_1346([a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    } else {
        select_1352([i, a, b, c, k, f, l, g, m, n])
    }
}
/// n = 10, i = 5, a < j, b < g, c < h, c < j, d < f, e < g, e < h, f < h, g < j, h < i
fn select_1354([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_172([c, b, d, e, a, f, g, h, j])
    } else {
        select_116([b, c, e, f, a, g, h, i])
    }
}
/// n = 13, i = 7, a < k, a < m, b < i, b < m, c < h, d < g, e < f, e < h, f < j, g < i, h < j, h < l, i < k, i < l, j < m
fn select_1355([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_1354([a, f, b, d, h, g, j, i, l, m])
    } else {
        select_116([a, c, i, e, f, k, h, l])
    }
}
/// n = 10, i = 5, a < j, b < i, b < j, c < g, d < f, e < g, e < h, f < h, g < j, h < i
fn select_1356([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_1219([a, b, d, c, e, f, g, h, j])
    } else {
        select_715([a, b, c, e, f, h, g, i])
    }
}
/// n = 13, i = 7, a < l, a < m, b < i, b < m, c < h, d < g, e < f, e < h, f < j, g < i, g < k, h < j, h < k, i < l, j < m
fn select_1357([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < k {
        select_1355([a, b, c, d, e, f, g, h, i, j, l, k, m])
    } else {
        select_1356([a, b, f, d, h, g, j, k, i, m])
    }
}
/// n = 9, i = 4, a < e, b < f, b < i, c < d, c < g, c < i, d < h, e < i, f < g, f < h
fn select_1358([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_688([c, a, b, f, e, d, g, i])
    } else {
        select_154([a, d, b, f, g, e, h])
    }
}
/// n = 10, i = 5, a < f, b < g, b < h, c < f, c < g, c < i, d < e, e < h, e < j, g < j, h < i
fn select_1359([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_334([a, b, d, f, e, g, h, j])
    } else {
        select_381([a, b, e, c, f, h, g, i])
    }
}
/// n = 11, i = 5, a < h, b < g, c < f, c < k, d < e, d < g, d < j, e < i, e < k, f < h, f < i, g < k, h < j
fn select_1360([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_1358([b, c, e, a, g, f, i, h, k])
    } else {
        select_1359([b, a, d, c, f, g, e, h, j, i])
    }
}
/// n = 10, i = 5, a < g, b < f, c < f, d < e, d < i, e < g, e < j, f < h, f < j, h < i
fn select_1361([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_381([a, b, c, e, g, f, h, j])
    } else {
        select_194([a, d, h, e, i])
    }
}
/// n = 8, i = 4, a < h, b < e, c < f, d < f, f < g, f < h
fn select_1362([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_170([a, b, f, g, e, h])
    } else {
        select_136([a, c, d, e, f])
    }
}
/// n = 11, i = 6, a < h, b < i, c < g, d < g, e < f, e < k, f < h, f < j, g < i, g < j, i < k
fn select_1363([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < f {
        select_1361([a, c, d, e, f, g, h, i, k, j])
    } else {
        select_1362([b, a, c, d, h, g, j, i])
    }
}
/// n = 12, i = 7, a < i, b < j, c < h, c < j, d < f, d < j, e < g, e < l, f < k, g < h, g < i, j < k, k < l
fn select_1364([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < h {
        select_1363([a, f, b, c, e, g, j, i, k, h, l])
    } else {
        select_504([b, a, c, d, f, i, h, j])
    }
}
/// n = 13, i = 7, a < j, b < i, b < m, c < h, d < g, d < m, e < f, e < h, f < k, g < i, g < j, h < k, h < l, j < l, k < m
fn select_1365([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_1360([a, f, d, h, b, g, k, j, i, l, m])
    } else {
        select_1364([a, c, b, e, d, f, g, i, j, h, k, m])
    }
}
/// n = 13, i = 7, a < j, a < m, b < g, c < f, c < m, d < i, e < h, e < i, f < j, g < l, h < k, i < k, i < l, k < m
fn select_1366([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_1357([a, c, d, b, e, h, g, i, f, k, l, j, m])
    } else {
        select_1365([b, a, d, c, e, h, f, i, j, g, k, l, m])
    }
}
/// n = 11, i = 6, a < h, b < i, b < j, b < k, c < g, d < g, d < k, e < f, f < h, f < j, g < i, g < j, h < k
fn select_1367([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_439([a, b, g, e, f, i, j, h])
    } else {
        select_120([b, c, d, h, g, k])
    }
}
/// n = 9, i = 4, a < h, a < i, b < g, b < h, c < e, d < f, d < i, e < i, f < g, f < h
fn select_1368([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_359([b, a, d, e, f, i])
    } else {
        select_1069([a, c, b, f, g, e, h])
    }
}
/// n = 12, i = 6, a < i, b < g, b < l, c < g, c < k, d < h, d < j, d < l, e < f, f < h, f < i, g < j, h < k, i < l
fn select_1369([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_1367([a, d, c, b, e, f, g, i, j, h, l])
    } else {
        select_1368([b, c, a, d, i, h, k, g, l])
    }
}
/// n = 9, i = 4, a < g, b < f, b < i, c < e, c < f, d < e, d < g, d < h, f < h, g < i
fn select_1370([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_122([b, c, a, e, g, i])
    } else {
        select_80([a, b, d, e, f, h])
    }
}
/// n = 10, i = 5, a < h, b < g, b < j, c < e, c < g, d < f, f < h, f < i, g < i, h < j
fn select_1371([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_1370([a, b, c, f, e, g, h, i, j])
    } else {
        select_240([a, b, d, e, g, f, h])
    }
}
/// n = 9, i = 4, a < f, b < d, b < i, c < e, c < i, d < h, e < g, e < h, f < g, f < i
fn select_1372([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_207([b, f, c, e, d, g, i])
    } else {
        select_30([a, d, c, e, f, h])
    }
}
/// n = 10, i = 5, a < h, b < g, b < j, c < g, c < h, d < e, d < j, e < f, e < i, g < i, h < j
fn select_1373([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_1372([a, b, d, g, e, h, f, i, j])
    } else {
        select_195([a, b, c, e, f, g, i, h])
    }
}
/// n = 11, i = 6, a < j, b < h, b < i, b < k, c < f, d < e, d < k, e < g, e < i, f < h, f < j, j < k
fn select_1374([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_1371([a, b, e, c, g, f, i, j, h, k])
    } else {
        select_1373([a, b, f, d, e, g, h, j, i, k])
    }
}
/// n = 12, i = 6, a < k, b < j, c < f, c < l, d < h, d < i, d < l, e < g, f < h, g < i, g < j, i < k, j < l
fn select_1375([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < f {
        select_1369([b, c, a, d, e, g, f, i, j, h, k, l])
    } else {
        select_1374([b, d, e, c, f, g, a, i, h, j, l])
    }
}
/// n = 7, i = 2, a < e, b < c, b < d, d < f, e < g
fn select_1376([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_198([a, c, d, f])
    } else {
        select_4([b, a, c, e, g])
    }
}
/// n = 8, i = 3, a < d, b < f, c < e, c < h, e < g, f < h
fn select_1377([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_1376([b, e, g, a, f, d, h])
    } else {
        select_793([b, c, a, d, f, e, g, h])
    }
}
/// n = 9, i = 4, a < h, b < g, c < e, d < f, d < g, e < i, f < h, f < i
fn select_1378([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_1377([b, c, f, g, a, e, h, i])
    } else {
        select_751([b, c, a, d, e, g, f, h])
    }
}
/// n = 9, i = 4, a < h, b < e, c < g, d < f, d < h, f < i, g < i
fn select_1379([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_1378([b, a, c, d, g, f, h, e, i])
    } else {
        select_753([a, c, d, b, g, e, h, f])
    }
}
/// n = 12, i = 6, a < k, b < j, c < i, c < l, d < f, d < l, e < g, f < i, g < h, g < j, h < k, j < l
fn select_1380([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < h {
        select_1375([a, b, d, c, e, f, g, i, h, j, k, l])
    } else {
        select_1379([a, b, d, h, j, c, f, k, i])
    }
}
/// n = 13, i = 7, a < f, b < k, b < m, c < g, c < m, d < i, e < h, e < i, g < k, h < l, i < j, i < l, l < m
fn select_1381([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < j {
        select_1366([b, a, c, d, e, g, f, h, i, k, l, j, m])
    } else {
        select_1380([a, h, b, c, d, g, i, j, k, l, f, m])
    }
}
/// n = 14, i = 7, a < g, b < k, b < m, c < h, c < m, d < i, e < f, e < i, h < k, i < j, i < l, l < m, m < n
fn select_1382([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if l < f {
        select_1353([a, b, c, d, e, g, h, i, j, k, l, f, m, n])
    } else {
        select_1381([a, b, c, d, e, g, h, f, i, j, k, l, m])
    }
}
/// n = 15, i = 7, a < g, a < o, b < l, b < m, c < h, c < m, d < f, d < i, e < i, e < m, h < l, i < j, i < k, i < o, m < n, m < o
fn select_1383([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if m < j {
        select_1336([a, b, d, e, c, f, g, h, i, k, l, m, n, j, o])
    } else {
        select_1382([a, b, c, e, d, f, g, h, i, k, l, j, m, n])
    }
}
/// n = 8, i = 4, a < f, b < f, b < g, c < d, d < e, d < g, f < h
fn select_1384([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_103([a, c, d, f, g, h])
    } else {
        select_7([a, e, b, g])
    }
}
/// n = 8, i = 4, a < f, b < f, c < e, d < e, d < h, e < g, f < h
fn select_1385([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_7([a, b, e, g])
    } else {
        select_6([c, d, f, h])
    }
}
/// n = 8, i = 4, b < f, c < e, d < f, d < h, e < h, f < g
fn select_1386([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_1384([b, d, c, e, a, f, h, g])
    } else {
        select_1385([a, c, b, d, f, e, g, h])
    }
}
/// n = 8, i = 4, a < h, b < g, b < h, c < f, c < h, d < e, d < g, d < h, f < g
fn select_1387([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_396([b, a, c, d, e, f, h])
    } else {
        select_49([b, d, f, a, e, g])
    }
}
/// n = 11, i = 6, a < k, b < i, b < k, c < g, d < h, d < i, d < k, e < f, e < k, f < i, g < h, g < k, i < j
fn select_1388([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1386([a, b, c, d, g, i, j, h])
    } else {
        select_1387([a, b, e, d, h, f, i, k])
    }
}
/// n = 8, i = 3, a < h, b < e, b < h, c < d, c < f, c < g, c < h, e < f, e < g
fn select_1389([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_54([c, e, a, d, f, g])
    } else {
        select_89([a, b, c, d, e, h])
    }
}
/// n = 10, i = 4, a < g, b < e, c < d, c < g, d < i, d < j, e < f, e < i, e < j, g < h
fn select_1390([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_191([d, b, g, h, e])
    } else {
        select_1389([a, c, e, f, d, i, j, g])
    }
}
/// n = 10, i = 4, a < h, b < e, c < d, c < g, d < h, d < i, e < f, e < g, e < i, h < j
fn select_1391([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_46([f, c, g, a, d, i])
    } else {
        select_1211([a, b, c, d, e, g, f, h, i, j])
    }
}
/// n = 10, i = 4, a < h, a < j, b < g, c < g, c < j, d < e, e < f, e < h, e < j, g < i
fn select_1392([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_1390([b, d, c, a, e, f, g, i, h, j])
    } else {
        select_1391([b, d, a, c, e, f, h, g, j, i])
    }
}
/// n = 14, i = 7, a < j, b < h, b < n, c < h, c < l, d < j, d < l, e < i, e < j, e < l, f < g, f < j, g < l, h < i, j < k, j < n, l < m
fn select_1393([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_1388([a, d, b, e, f, g, h, i, l, m, j])
    } else {
        select_1392([b, g, c, a, j, k, l, n, m, h])
    }
}
/// n = 10, i = 5, a < g, a < i, b < g, b < i, c < g, c < i, d < e, d < i, e < f, g < h, i < j
fn select_1394([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_128([e, a, b, c, f, i, j])
    } else {
        select_46([g, d, e, h, i, j])
    }
}
/// n = 11, i = 5, a < i, b < g, b < i, c < g, c < i, d < e, d < i, e < f, e < j, g < h, g < j, i < k
fn select_1395([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_1394([a, b, c, d, e, f, g, h, i, k])
    } else {
        select_827([d, g, h, a, e, f, j, i])
    }
}
/// n = 13, i = 7, a < j, a < l, b < i, b < j, b < l, c < h, c < j, d < h, d < j, e < f, e < l, f < g, f < i, h < l, j < k, l < m
fn select_1396([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < i {
        select_1395([h, a, b, e, f, g, j, k, l, i, m])
    } else {
        select_395([a, f, c, d, b, g, i, h, l, m, j])
    }
}
/// n = 9, i = 5, a < h, b < g, b < h, c < f, c < h, d < e, e < f, e < g, h < i
fn select_1397([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_100([a, c, d, e, g, h, i])
    } else {
        select_134([a, b, c, f, g])
    }
}
/// n = 11, i = 6, a < i, a < k, b < h, b < i, b < k, c < f, d < e, d < k, e < i, f < g, f < h, g < k, i < j
fn select_1398([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_1397([a, b, e, c, f, g, h, i, j])
    } else {
        select_396([a, d, g, b, h, e, k])
    }
}
/// n = 10, i = 4, a < f, b < f, b < g, c < d, c < g, d < e, d < j, f < i, g < h, g < j
fn select_1399([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_11([a, b, c, d, e, f, i, g])
    } else {
        select_112([a, d, b, e, g, h, j, f])
    }
}
/// n = 12, i = 6, a < i, a < j, b < h, b < i, b < j, c < e, c < j, d < f, d < j, e < i, f < g, f < h, i < k, j < l
fn select_1400([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < j {
        select_1398([a, b, d, c, e, f, g, h, i, k, j])
    } else {
        select_1399([e, a, d, f, h, i, j, l, k, g])
    }
}
/// n = 13, i = 7, a < j, b < j, b < l, c < i, c < j, c < l, d < f, d < j, e < g, e < l, f < l, g < h, g < i, j < k, l < m
fn select_1401([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < f {
        select_1396([b, c, a, d, e, g, h, f, i, j, k, l, m])
    } else {
        select_1400([b, c, f, e, a, g, h, i, j, l, k, m])
    }
}
/// n = 14, i = 7, a < n, b < j, c < g, c < l, d < j, d < l, e < i, e < j, e < l, f < h, f < j, g < i, h < l, j < k, j < n, l < m
fn select_1402([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < g {
        select_1393([b, a, c, d, e, f, h, g, i, j, k, l, m, n])
    } else {
        select_1401([b, d, e, f, c, h, g, a, i, j, k, l, m])
    }
}
/// n = 8, i = 3, b < e, c < g, c < h, d < f, f < g, f < h
fn select_1403([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_1312([a, c, d, b, e, f])
    } else {
        select_1267([a, b, c, d, f, g, h, e])
    }
}
/// n = 9, i = 3, b < e, c < g, c < i, d < f, d < h, f < i, g < h
fn select_1404([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_1403([a, b, c, d, e, f, g, i])
    } else {
        select_1403([a, b, d, c, e, g, f, h])
    }
}
/// n = 9, i = 4, a < i, b < f, b < h, c < g, d < e, d < g, g < h, g < i
fn select_1405([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_1267([e, a, b, c, g, f, h, i])
    } else {
        select_123([a, c, b, d, e, f, g])
    }
}
/// n = 8, i = 4, a < h, b < f, c < g, d < e, d < g, g < h
fn select_1406([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_669([e, a, b, c, g, f, h])
    } else {
        select_123([a, c, b, d, e, f, g])
    }
}
/// n = 9, i = 4, a < g, b < h, c < f, c < i, d < e, d < g, d < h, h < i
fn select_1407([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_1405([a, c, b, d, e, f, h, i, g])
    } else {
        select_1406([b, c, a, d, e, f, g, h])
    }
}
/// n = 10, i = 4, a < j, b < h, c < f, c < j, d < e, d < g, d < h, f < i, g < i, g < j
fn select_1408([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_1404([e, b, c, g, h, a, f, i, j])
    } else {
        select_1407([b, a, c, d, e, f, h, g, j])
    }
}
/// n = 14, i = 7, a < l, b < j, c < i, c < m, d < g, d < m, e < j, e < m, f < h, f < j, g < i, h < m, j < k, j < l, m < n
fn select_1409([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if c < j {
        select_1402([a, b, d, e, c, f, g, h, i, j, k, m, n, l])
    } else {
        select_1408([h, a, d, j, k, g, c, l, i, m])
    }
}
/// n = 8, i = 3, a < h, b < g, b < h, c < e, c < g, d < e, d < g, d < h, e < f
fn select_1410([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_392([c, a, d, b, e, f, h])
    } else {
        select_275([c, b, d, a, e, f, h, g])
    }
}
/// n = 11, i = 5, a < j, a < k, b < g, b < k, c < g, c < h, d < f, e < g, e < h, e < k, f < h, f < k, g < j, h < i
fn select_1411([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < f {
        select_697([a, c, d, e, f, g, h, j, k])
    } else {
        select_1410([a, b, c, e, h, i, g, k])
    }
}
/// n = 13, i = 7, a < j, a < m, b < h, b < m, c < f, d < h, d < k, d < m, e < g, f < k, f < m, g < i, g < m, h < j, i < k, k < l
fn select_1412([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < h {
        select_1411([a, b, i, c, d, f, h, k, l, j, m])
    } else {
        select_708([a, h, c, e, f, g, j, i, m])
    }
}
/// n = 9, i = 4, a < i, b < f, c < f, c < g, d < e, d < g, g < h, g < i
fn select_1413([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < b {
        select_618([e, a, g, h, b, f, i])
    } else {
        select_47([a, b, c, d, e, f, g, h])
    }
}
/// n = 14, i = 7, a < j, a < k, b < f, c < h, c < k, d < h, d < k, d < m, e < g, f < m, g < i, g < k, h < j, i < m, k < l, m < n
fn select_1414([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < k {
        select_1412([a, c, b, d, e, f, g, h, i, j, m, n, k])
    } else {
        select_1413([b, h, a, g, i, j, k, l, f])
    }
}
/// n = 10, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < j, g < j, h < i
fn select_1415([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_244([a, b, c, d, e, f, g, j])
    } else {
        select_255([a, g, f, h, i, j])
    }
}
/// n = 10, i = 5, a < e, b < i, c < g, d < f, d < i, f < h, g < h, g < i, i < j
fn select_1416([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < f {
        select_1415([a, b, d, c, e, g, f, i, j, h])
    } else {
        select_147([a, c, f, e, b, g, h, i])
    }
}
/// n = 10, i = 5, a < e, b < h, b < i, c < g, c < h, d < f, d < i, f < h, g < i, i < j
fn select_1417([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_406([a, b, d, g, e, f, i, j, h])
    } else {
        select_345([a, b, c, f, e, g, h, i])
    }
}
/// n = 11, i = 6, a < e, b < i, b < j, c < g, d < f, d < i, f < j, g < h, g < j, j < k
fn select_1418([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_1416([a, f, c, b, e, i, g, h, j, k])
    } else {
        select_1417([a, b, d, g, e, h, f, i, j, k])
    }
}
/// n = 14, i = 7, a < j, a < k, b < g, c < k, c < m, d < f, d < k, e < h, f < j, g < m, h < i, h < k, i < m, k < l, m < n
fn select_1419([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if c < f {
        select_1414([a, b, d, c, e, g, h, f, i, j, k, l, m, n])
    } else {
        select_1418([b, a, e, f, g, c, h, i, j, k, l])
    }
}
/// n = 9, i = 5, a < g, b < f, c < g, c < i, d < g, d < i, e < g, e < i, f < i, g < h
fn select_1420([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_128([b, c, d, e, f, g, h])
    } else {
        select_136([a, c, d, e, i])
    }
}
/// n = 7, i = 3, a < e, b < f, b < g, c < f, c < g, d < e, d < f, d < g
fn select_1421([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_57([a, c, d, f, g, e])
    } else {
        select_7([b, c, d, e])
    }
}
/// n = 13, i = 7, a < j, a < k, b < h, b < l, c < h, c < l, d < g, e < i, e < k, f < i, f < k, g < i, h < k, i < j, i < l, k < m
fn select_1422([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_1420([a, d, e, f, h, g, k, m, i])
    } else {
        select_1421([a, b, c, i, j, h, l])
    }
}
/// n = 6, i = 3, a < f, b < f, c < d, d < e
fn select_1423([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_103([a, c, d, b, e, f])
    } else {
        select_103([b, c, d, a, e, f])
    }
}
/// n = 6, i = 3, a < f, b < f, c < d, c < e
fn select_1424([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_1423([a, b, c, d, e, f])
    } else {
        select_1423([a, b, c, e, d, f])
    }
}
/// n = 8, i = 4, a < g, b < f, b < h, c < f, c < h, d < e, d < g, e < h
fn select_1425([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_713([a, c, d, f, e, g, h])
    } else {
        select_1424([b, c, d, e, g, f])
    }
}
/// n = 13, i = 7, a < k, a < l, b < j, b < k, c < h, c < l, d < h, d < l, e < g, f < i, f < k, g < i, h < k, i < j, i < l, k < m
fn select_1426([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < i {
        select_1422([b, c, d, e, a, f, g, h, i, j, k, l, m])
    } else {
        select_1425([b, c, d, i, a, h, j, l])
    }
}
/// n = 6, i = 2, b < f, c < d, c < e, c < f
fn select_1427([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < b {
        select_198([a, e, d, b])
    } else {
        select_144([a, b, c, e, d, f])
    }
}
/// n = 11, i = 5, a < i, b < i, c < i, c < j, d < f, e < g, e < i, f < g, g < h, g < j, i < k
fn select_1428([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_197([b, c, d, a, e, f, g, i, h, k])
    } else {
        select_1427([b, c, g, a, h, j])
    }
}
/// n = 14, i = 7, a < j, a < m, b < k, b < m, c < i, c < m, d < h, d < k, e < h, e < k, f < g, g < i, g < k, h < m, i < j, k < l, m < n
fn select_1429([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < k {
        select_1426([b, a, d, e, f, c, g, h, i, j, m, k, n])
    } else {
        select_1428([a, h, c, f, b, g, k, l, m, i, n])
    }
}
/// n = 8, i = 3, a < h, b < f, c < e, c < h, d < e, d < f, d < h, f < g
fn select_1430([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < b {
        select_170([a, b, d, e, f, h])
    } else {
        select_255([a, c, d, f, g, e])
    }
}
/// n = 9, i = 4, a < g, b < h, c < f, c < g, d < f, d < h, e < f, e < g, e < h, h < i
fn select_1431([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_1430([a, b, c, e, f, h, i, g])
    } else {
        select_208([b, d, c, e, g, f, h])
    }
}
/// n = 7, i = 4, b < g, c < f, c < g, d < f, d < g, e < f, e < g
fn select_1432([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_955([a, b, c, d, e, g, f])
    } else {
        select_152([b, f, a, g])
    }
}
/// n = 12, i = 6, a < g, b < j, b < l, c < i, c < j, d < g, d < i, e < h, e < l, f < h, f < l, g < l, h < i, h < j, j < k
fn select_1433([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_1431([a, b, d, c, h, i, g, j, k])
    } else {
        select_1432([c, b, e, f, g, h, l])
    }
}
/// n = 12, i = 6, a < h, b < j, b < l, c < g, c < j, d < g, d < h, e < i, e < l, f < i, f < l, h < l, i < j, j < k
fn select_1434([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_1433([a, b, c, d, e, f, h, i, g, j, k, l])
    } else {
        select_969([b, e, f, a, c, h, g, i, l])
    }
}
/// n = 14, i = 7, a < j, a < m, b < h, c < k, c < m, d < g, d < m, e < i, e < k, f < i, f < k, g < j, h < k, i < m, k < l, m < n
fn select_1435([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_1429([a, c, d, e, f, b, h, i, g, j, k, l, m, n])
    } else {
        select_1434([b, c, a, g, e, f, j, h, i, m, n, k])
    }
}
/// n = 14, i = 7, a < l, b < j, b < k, c < i, d < k, d < l, e < g, e < k, f < h, f < l, g < j, h < k, i < l, k < m, l < n
fn select_1436([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < a {
        select_1419([b, c, d, e, f, g, i, h, a, j, k, m, l, n])
    } else {
        select_1435([b, c, d, e, a, f, g, i, h, j, l, n, k, m])
    }
}
/// n = 14, i = 7, a < g, b < k, c < j, c < m, d < k, d < m, e < h, e < m, f < i, f < k, h < j, i < m, k < l, m < n
fn select_1437([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < g {
        select_1409([a, b, c, e, d, f, h, i, j, k, l, g, m, n])
    } else {
        select_1436([b, c, a, d, e, f, h, i, g, j, m, k, n, l])
    }
}
/// n = 10, i = 4, a < e, b < g, b < j, c < f, c < g, d < f, d < g, f < j, g < h, g < i
fn select_1438([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_245([a, f, b, g, h, i, j])
    } else {
        select_240([b, a, c, d, e, f, j])
    }
}
/// n = 10, i = 4, a < e, b < h, c < g, c < h, d < f, d < h, f < g, h < i, h < j
fn select_1439([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_346([a, c, f, e, b, g, h])
    } else {
        select_1438([a, c, b, d, e, f, h, i, j, g])
    }
}
/// n = 11, i = 4, a < f, b < h, b < k, c < g, c < h, d < e, d < g, d < h, g < k, h < i, h < j
fn select_1440([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_1439([a, b, c, d, f, e, g, h, i, j])
    } else {
        select_237([a, b, c, d, f, g, e, h, i, j, k])
    }
}
/// n = 9, i = 4, a < i, b < e, b < h, c < f, d < f, f < g, f < h, f < i
fn select_1441([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_323([b, f, g, e, a, i, h])
    } else {
        select_593([b, a, c, d, e, f, g, i])
    }
}
/// n = 9, i = 6, a < i, b < h, c < h, c < i, d < g, e < f, f < i, g < i
fn select_1442([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_118([a, b, d, f, c, h, g, i])
    } else {
        select_118([a, b, e, g, c, h, f, i])
    }
}
/// n = 11, i = 6, a < f, b < h, b < k, c < i, d < h, d < i, e < g, g < i, i < j, i < k
fn select_1443([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < f {
        select_1441([a, b, c, g, h, i, j, k, f])
    } else {
        select_1442([c, b, d, a, e, g, f, h, i])
    }
}
/// n = 11, i = 6, a < g, b < h, b < k, c < i, d < i, e < i, f < h, f < i, i < j, i < k
fn select_1444([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_1443([a, b, e, f, c, g, d, h, i, j, k])
    } else {
        select_1443([a, b, e, f, d, g, c, h, i, j, k])
    }
}
/// n = 14, i = 7, a < g, b < i, b < l, c < j, d < j, e < j, f < h, f < l, h < i, j < k, j < l, l < m, l < n
fn select_1445([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < h {
        select_1440([a, b, f, j, k, g, h, l, m, n, i])
    } else {
        select_1444([a, b, c, d, e, h, g, i, j, k, l])
    }
}
/// n = 8, i = 2, a < h, b < f, b < g, c < d, c < e, c < f, c < g, c < h
fn select_1446([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_40([a, b, c, d, e, f, g, h])
    } else {
        select_40([a, b, c, e, d, f, g, h])
    }
}
/// n = 10, i = 4, b < j, c < h, c < i, d < f, e < f, f < g, f < h, f < i, f < j
fn select_1447([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_497([b, c, a, d, e, f, g])
    } else {
        select_1446([b, c, f, a, g, h, i, j])
    }
}
/// n = 8, i = 6, c < h, d < h, e < g, f < g
fn select_1448([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < h {
        select_363([a, b, c, d, g, h])
    } else {
        select_363([a, b, e, f, h, g])
    }
}
/// n = 9, i = 6, b < i, c < i, d < h, d < i, e < h, e < i, f < g, g < i
fn select_1449([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_231([b, c, d, e, g, a, h, i])
    } else {
        select_1448([b, c, d, e, a, f, g, h])
    }
}
/// n = 12, i = 6, a < g, b < k, b < l, c < i, d < i, e < h, e < i, f < h, f < i, i < j, i < k, i < l
fn select_1450([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_1447([h, a, b, c, d, i, j, k, l, g])
    } else {
        select_1449([b, c, d, e, f, a, g, h, i])
    }
}
/// n = 7, i = 1, b < f, b < g, c < d, c < e
fn select_1451([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < b {
        select_9([a, c, d, e])
    } else {
        select_108([a, c, b, f, g, e])
    }
}
/// n = 7, i = 1, c < g, d < e, d < f
fn select_1452([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_108([b, a, d, e, f, c])
    } else {
        select_1451([b, c, d, e, f, a, g])
    }
}
/// n = 9, i = 2, b < e, b < g, c < d, c < f, f < h, g < i
fn select_1453([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_1452([a, e, g, c, d, f, i])
    } else {
        select_1452([a, d, f, b, e, g, h])
    }
}
/// n = 10, i = 3, a < f, a < h, b < e, b < g, c < d, c < f, c < g, c < h, g < i, h < j
fn select_1454([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_649([b, c, d, f, e, h, j, g])
    } else {
        select_41([e, a, c, d, g, i, f, h])
    }
}
/// n = 10, i = 3, a < e, b < h, b < i, c < d, c < f, c < h, c < i, f < g, i < j
fn select_1455([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_1453([d, b, f, g, h, a, i, e, j])
    } else {
        select_1454([b, a, c, d, e, h, f, i, g, j])
    }
}
/// n = 13, i = 6, a < h, b < g, c < k, c < l, d < i, e < i, f < h, f < i, i < j, i < k, i < l, l < m
fn select_1456([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < i {
        select_1450([b, c, d, e, a, f, g, h, i, j, k, l])
    } else {
        select_1455([b, c, i, j, g, a, h, k, l, m])
    }
}
/// n = 14, i = 7, a < g, b < j, b < n, c < i, c < l, d < j, d < l, e < j, e < l, f < h, f < l, h < i, j < k, l < m, l < n
fn select_1457([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < l {
        select_1445([a, c, b, d, e, f, g, h, i, j, k, l, m, n])
    } else {
        select_1456([h, a, b, d, e, c, g, i, l, m, n, j, k])
    }
}
/// n = 8, i = 3, a < f, a < h, b < d, b < g, c < e, c < f, e < g, e < h
fn select_1458([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_109([a, e, f, b, d, g, h])
    } else {
        select_613([a, c, b, d, f, e, g])
    }
}
/// n = 10, i = 4, a < g, a < j, b < d, b < i, c < e, c < g, d < f, e < h, e < i, h < j
fn select_1459([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_1458([b, a, e, g, h, i, j, d])
    } else {
        select_1231([a, c, b, e, d, f, g, i, h])
    }
}
/// n = 8, i = 3, a < g, a < h, b < d, b < f, c < e, e < f, e < g, e < h
fn select_1460([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_6([a, c, d, e])
    } else {
        select_39([a, b, e, d, f, g, h])
    }
}
/// n = 9, i = 3, a < h, a < i, b < d, b < f, c < e, c < f, e < h, e < i, f < g
fn select_1461([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_111([a, d, c, f, e, g])
    } else {
        select_1460([a, b, c, d, e, f, h, i])
    }
}
/// n = 10, i = 4, a < g, a < h, b < d, c < e, c < g, d < f, d < j, e < h, e < j, h < i
fn select_1462([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_613([a, c, d, f, g, e, j])
    } else {
        select_1037([b, a, e, g, h, d, i, f, j])
    }
}
/// n = 11, i = 4, a < g, a < i, b < d, b < j, c < e, c < g, d < f, e < h, e < i, h < j, i < k
fn select_1463([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < d {
        select_1461([b, a, e, g, h, i, k, d, j])
    } else {
        select_1462([a, b, c, d, e, f, g, i, k, h])
    }
}
/// n = 11, i = 4, a < h, a < j, b < e, b < i, c < d, c < f, c < h, d < i, e < g, f < j, j < k
fn select_1464([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < f {
        select_1459([a, b, c, e, d, g, h, f, i, j])
    } else {
        select_1463([a, b, c, e, f, g, h, d, j, i, k])
    }
}
/// n = 10, i = 3, a < d, a < g, b < e, b < f, c < e, c < f, c < g, e < i, f < h, g < j
fn select_1465([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_264([d, b, c, g, j, e, f])
    } else {
        select_648([a, c, d, e, f, h, i, g])
    }
}
/// n = 10, i = 3, a < d, a < h, b < g, b < h, c < e, c < g, e < f, g < i, h < j
fn select_1466([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_46([a, e, f, d, b, g])
    } else {
        select_1465([a, c, b, d, e, g, h, i, f, j])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < e
fn select_1467([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_30([a, b, c, d, f, e])
    } else {
        select_4([a, c, f, e, d])
    }
}
/// n = 9, i = 4, a < d, a < h, b < e, b < g, c < f, e < i, f < g, f < h, f < i
fn select_1468([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < e {
        select_1467([b, f, a, d, h, g])
    } else {
        select_147([a, c, b, d, e, f, g, i])
    }
}
/// n = 9, i = 4, a < i, b < h, c < e, c < g, d < f, f < g, f < h, f < i
fn select_1469([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1468([c, a, d, e, b, f, i, g, h])
    } else {
        select_1468([c, b, d, e, a, f, h, g, i])
    }
}
/// n = 12, i = 5, a < i, a < k, b < d, b < e, c < f, d < j, e < g, f < h, f < i, f < j, h < k, k < l
fn select_1470([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_1466([a, h, b, i, d, j, e, k, g, l])
    } else {
        select_1469([a, d, e, c, g, f, h, j, i])
    }
}
/// n = 12, i = 5, a < j, a < k, b < d, b < e, c < f, e < h, f < g, f < i, f < j, i < k, k < l
fn select_1471([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < d {
        select_1464([a, b, f, g, e, i, h, j, d, k, l])
    } else {
        select_1470([a, b, c, d, e, f, h, i, j, g, k, l])
    }
}
/// n = 12, i = 5, a < f, b < e, c < j, c < k, d < g, g < h, g < i, g < j, i < k, k < l
fn select_1472([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_1471([c, a, d, f, b, g, h, e, i, j, k, l])
    } else {
        select_1471([c, b, d, e, a, g, h, f, i, j, k, l])
    }
}
/// n = 14, i = 7, a < g, b < j, c < j, c < n, d < i, d < k, e < j, e < k, f < h, f < k, h < i, j < m, k < l, k < n
fn select_1473([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < k {
        select_1457([a, c, d, b, e, f, g, h, i, j, m, k, l, n])
    } else {
        select_1472([a, h, c, e, i, g, k, l, b, n, j, m])
    }
}
/// n = 14, i = 7, a < h, b < l, c < j, c < k, d < g, d < l, e < k, e < l, f < i, f < k, i < j, k < m, l < n
fn select_1474([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < k {
        select_1437([a, b, c, e, f, d, h, i, g, j, l, n, k, m])
    } else {
        select_1473([a, b, d, c, e, f, h, i, j, l, k, m, n, g])
    }
}
/// n = 15, i = 7, b < h, b < o, c < l, c < m, d < g, d < j, e < j, e < m, f < i, f < m, i < l, j < k, j < o, m < n, m < o
fn select_1475([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < a {
        select_1383([b, c, f, d, e, g, h, i, j, a, k, l, m, n, o])
    } else {
        select_1474([b, a, c, d, e, f, g, h, i, l, m, j, n, k])
    }
}
/// n = 16, i = 8, b < g, b < p, c < f, c < i, d < h, d < n, e < j, e < k, h < m, i < l, i < p, j < n, k < m, k < n, n < o, n < p
fn select_1476([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_1311([a, b, d, e, c, f, g, h, k, i, l, j, m, n, o, p])
    } else {
        select_1475([a, b, k, c, j, d, f, g, h, i, l, m, n, o, p])
    }
}
/// n = 17, i = 8, b < h, b < p, c < g, c < j, d < i, d < m, e < f, e < k, i < o, j < l, j < p, k < m, k < o, m < n, m < p, p < q
fn select_1477([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if m < f {
        select_1198([a, b, c, e, d, g, h, i, j, k, l, m, f, n, o, p, q])
    } else {
        select_1476([a, b, c, d, e, g, h, i, j, f, k, l, o, m, n, p])
    }
}
/// n = 17, i = 8, b < i, b < p, c < h, c < n, d < g, d < j, e < f, e < k, j < m, j < p, k < l, k < n, n < o, n < p, p < q
fn select_1478([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if l < h {
        select_992([a, b, c, d, e, f, g, i, j, k, m, l, n, o, h, p, q])
    } else {
        select_1477([a, b, d, c, e, f, g, i, h, j, k, m, n, o, l, p, q])
    }
}
/// n = 17, i = 8, b < i, b < o, c < h, c < n, d < g, d < j, e < f, e < k, j < m, j < o, k < l, k < n, n < p, o < q
fn select_1479([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if n < o {
        select_1478([a, b, c, d, e, f, g, h, i, j, k, l, m, n, p, o, q])
    } else {
        select_1478([a, c, b, e, d, g, f, i, h, k, j, m, l, o, q, n, p])
    }
}
/// n = 17, i = 8, b < i, b < p, c < h, c < k, d < g, d < j, e < f, e < l, j < n, k < o, l < m, l < p, p < q
fn select_1480([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < k {
        select_1479([a, b, c, e, d, g, f, h, i, l, j, n, m, k, p, o, q])
    } else {
        select_1479([a, b, d, e, c, h, f, g, i, l, k, o, m, j, p, n, q])
    }
}
/// n = 17, i = 8, b < k, c < j, d < i, d < p, e < h, e < l, f < g, f < m, l < o, m < n, m < p, p < q
fn select_1481([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if b < c {
        select_1480([a, d, b, e, f, g, h, k, i, l, c, m, n, o, j, p, q])
    } else {
        select_1480([a, d, c, e, f, g, h, j, i, l, b, m, n, o, k, p, q])
    }
}
/// n = 17, i = 8, b < m, c < l, d < k, e < j, f < i, f < p, g < h, g < n, n < o, n < p, p < q
fn select_1482([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if b < d {
        select_1481([a, c, e, f, b, g, h, m, i, j, l, d, n, o, k, p, q])
    } else {
        select_1481([a, c, e, f, d, g, h, k, i, j, l, b, n, o, m, p, q])
    }
}
/// n = 17, i = 8, d < m, e < l, f < k, g < j, g < p, h < i, h < n, n < o, n < p, p < q
fn select_1483([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1482([c, a, d, e, f, g, h, i, j, k, l, m, b, n, o, p, q])
    } else {
        select_1482([c, b, d, e, f, g, h, i, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 8, f < m, g < l, h < k, h < p, i < j, i < n, n < o, n < p, p < q
fn select_1484([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1483([c, d, e, a, f, g, h, i, j, k, l, m, b, n, o, p, q])
    } else {
        select_1483([c, d, e, b, f, g, h, i, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 8, h < m, i < l, i < p, j < k, j < n, n < o, n < p, p < q
fn select_1485([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1484([c, d, e, f, g, a, h, i, j, k, l, m, b, n, o, p, q])
    } else {
        select_1484([c, d, e, f, g, b, h, i, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 8, j < m, j < p, k < l, k < n, n < o, n < p, p < q
fn select_1486([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1485([c, d, e, f, g, h, i, a, j, k, l, m, b, n, o, p, q])
    } else {
        select_1485([c, d, e, f, g, h, i, b, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 8, j < m, j < o, k < l, k < n, n < p, o < q
fn select_1487([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if n < o {
        select_1486([a, b, c, d, e, f, g, h, i, j, k, l, m, n, p, o, q])
    } else {
        select_1486([a, b, c, d, e, f, g, h, i, k, j, m, l, o, q, n, p])
    }
}
/// n = 17, i = 8, j < o, k < n, l < m, l < p, p < q
fn select_1488([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < k {
        select_1487([a, b, c, d, e, f, g, h, i, j, l, m, o, p, k, q, n])
    } else {
        select_1487([a, b, c, d, e, f, g, h, i, k, l, m, n, p, j, q, o])
    }
}
/// n = 17, i = 8, l < o, m < n, m < p, p < q
fn select_1489([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1488([c, d, e, f, g, h, i, j, k, a, l, m, n, o, b, p, q])
    } else {
        select_1488([c, d, e, f, g, h, i, j, k, b, l, m, n, o, a, p, q])
    }
}
/// n = 17, i = 8, n < o, n < p, p < q
fn select_1490([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1489([c, d, e, f, g, h, i, j, k, l, m, a, n, o, b, p, q])
    } else {
        select_1489([c, d, e, f, g, h, i, j, k, l, m, b, n, o, a, p, q])
    }
}
/// n = 17, i = 8, n < q, o < p
fn select_1491([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if n < o {
        select_1490([a, b, c, d, e, f, g, h, i, j, k, l, m, n, q, o, p])
    } else {
        select_1490([a, b, c, d, e, f, g, h, i, j, k, l, m, o, p, n, q])
    }
}
/// n = 17, i = 8, p < q
fn select_1492([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1491([c, d, e, f, g, h, i, j, k, l, m, n, o, a, p, q, b])
    } else {
        select_1491([c, d, e, f, g, h, i, j, k, l, m, n, o, b, p, q, a])
    }
}
/// n = 17, i = 8
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_1492([c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, a, b])
    } else {
        select_1492([c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, b, a])
    }
}
