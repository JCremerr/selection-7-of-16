#[allow(unused)]
pub const N: usize = 17;
#[allow(unused)]
pub const I: usize = 5;

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
/// n = 7, i = 1, a < f, a < g, b < c, b < d, b < e
fn select_5([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_3([b, f, g])
    } else {
        select_4([a, c, d, e])
    }
}
/// n = 8, i = 1, a < h, b < g, c < d, c < e, c < f
fn select_6([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_5([a, c, d, e, f, b, h])
    } else {
        select_5([b, c, d, e, f, a, g])
    }
}
/// n = 8, i = 1, c < h, d < e, d < f, d < g
fn select_7([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_6([a, c, d, e, f, g, h, b])
    } else {
        select_6([b, c, d, e, f, g, h, a])
    }
}
/// n = 3, i = 1, b < c
fn select_8([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_2([a, c])
    }
}
/// n = 5, i = 2, a < c, b < d, d < e
fn select_9([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < a {
        select_2([a, e])
    } else {
        select_8([c, b, d])
    }
}
/// n = 5, i = 2, a < e, b < c, b < d
fn select_10([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_9([a, b, e, c, d])
    } else {
        select_9([a, b, e, d, c])
    }
}
/// n = 6, i = 2, a < d, b < c, b < e, e < f
fn select_11([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if f < a {
        select_8([c, e, f])
    } else {
        select_10([a, b, c, e, d])
    }
}
/// n = 10, i = 2, a < d, b < c, b < e, b < f, e < j, f < g, f < h, f < i
fn select_12([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_7([a, c, e, f, g, h, i, j])
    } else {
        select_11([a, b, c, d, e, j])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, e < j, f < g, f < h, f < i
fn select_13([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_12([a, c, d, b, e, f, g, h, i, j])
    } else {
        select_12([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 5, i = 0
fn select_14([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_4([a, c, d, e])
    } else {
        select_4([b, c, d, e])
    }
}
/// n = 7, i = 1, a < g, b < c, b < d, b < e, b < f
fn select_15([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_2([b, g])
    } else {
        select_14([a, c, d, e, f])
    }
}
/// n = 8, i = 1, a < g, a < h, b < c, b < d, b < e, b < f
fn select_16([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_3([b, g, h])
    } else {
        select_14([a, c, d, e, f])
    }
}
/// n = 8, i = 1, b < h, c < d, c < e, c < f, c < g
fn select_17([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_15([a, c, d, e, f, g, b])
    } else {
        select_16([b, c, d, e, f, g, a, h])
    }
}
/// n = 4, i = 1, b < c, b < d
fn select_18([a, b, c, d]: [usize; 4]) -> usize {
    if c < a {
        select_2([c, d])
    } else {
        select_8([a, b, d])
    }
}
/// n = 5, i = 1, a < e, b < c, b < d
fn select_19([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_2([b, e])
    } else {
        select_3([a, c, d])
    }
}
/// n = 7, i = 2, a < c, a < f, a < g, b < d, b < f, b < g, d < e
fn select_20([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_18([c, b, f, g])
    } else {
        select_19([d, a, f, g, e])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < f, c < g, d < e
fn select_21([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_9([a, c, b, d, e])
    } else {
        select_20([b, c, a, d, e, f, g])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < j, d < i, e < f, e < g, e < h
fn select_22([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_17([a, d, e, b, f, g, h, i])
    } else {
        select_21([a, b, c, d, i, e, j])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_23([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_22([a, d, b, f, g, h, i, j, k, e])
    } else {
        select_22([a, e, c, f, g, h, i, j, k, d])
    }
}
/// n = 12, i = 3, b < e, b < i, b < l, c < d, c < f, c < i, f < g, f < h, f < l, h < j, i < k
fn select_24([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_13([a, d, f, g, h, b, e, i, l, j])
    } else {
        select_23([a, b, c, d, e, i, f, g, h, l, k])
    }
}
/// n = 8, i = 2, a < d, a < f, b < c, b < e, e < g, f < h
fn select_25([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_11([a, b, c, d, e, g])
    } else {
        select_11([b, a, d, c, f, h])
    }
}
/// n = 8, i = 2, a < e, a < h, b < c, b < d, b < h, d < f, e < g
fn select_26([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_18([c, d, a, f])
    } else {
        select_20([b, a, c, e, g, d, h])
    }
}
/// n = 9, i = 2, a < d, a < f, a < i, b < c, b < e, b < i, e < g, f < h
fn select_27([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_26([a, b, c, e, f, g, h, i])
    } else {
        select_26([b, a, d, f, e, h, g, i])
    }
}
/// n = 9, i = 2, a < f, b < e, b < i, c < d, c < g, c < i, g < h
fn select_28([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_25([a, c, d, f, g, b, h, e])
    } else {
        select_27([b, c, d, e, g, a, h, f, i])
    }
}
/// n = 4, i = 1, a < d, b < c
fn select_29([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_2([b, d])
    } else {
        select_2([a, c])
    }
}
/// n = 6, i = 2, a < c, a < f, b < d, b < f, d < e
fn select_30([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_8([c, b, f])
    } else {
        select_29([a, d, e, f])
    }
}
/// n = 7, i = 2, a < e, b < c, b < d, d < f, e < g
fn select_31([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_18([c, d, a, f])
    } else {
        select_30([b, a, c, e, g, d])
    }
}
/// n = 5, i = 2, a < e, b < d, c < d, c < e
fn select_32([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_8([b, c, e])
    } else {
        select_8([a, c, d])
    }
}
/// n = 6, i = 2, b < f, c < d, c < f, d < e
fn select_33([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_19([b, d, a, e, f])
    } else {
        select_32([a, b, c, f, d])
    }
}
/// n = 7, i = 3, a < e, b < e, b < f, c < d, c < f, f < g
fn select_34([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_33([d, a, b, f, g, e])
    } else {
        select_1([c])
    }
}
/// n = 9, i = 3, a < e, a < g, b < d, b < f, c < e, c < f, c < g, f < h, g < i
fn select_35([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_31([b, a, e, g, f, i, h])
    } else {
        select_34([a, c, b, d, e, f, h])
    }
}
/// n = 10, i = 3, a < g, b < d, b < h, b < j, c < e, c < g, c < h, e < f, e < j, h < i
fn select_36([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_27([b, e, f, d, a, h, g, i, j])
    } else {
        select_35([a, b, c, d, g, h, e, i, f])
    }
}
/// n = 10, i = 3, b < e, b < h, b < j, c < d, c < f, c < h, f < g, f < j, h < i
fn select_37([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_28([d, f, b, e, g, a, h, i, j])
    } else {
        select_36([a, b, c, e, f, g, d, h, i, j])
    }
}
/// n = 6, i = 1, a < f, b < c, b < d, b < e
fn select_38([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_2([b, f])
    } else {
        select_4([a, c, d, e])
    }
}
/// n = 7, i = 1, b < g, c < d, c < e, c < f
fn select_39([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_38([a, c, d, e, f, b])
    } else {
        select_5([b, c, d, e, f, a, g])
    }
}
/// n = 7, i = 2, a < c, b < d, d < e, d < f, d < g
fn select_40([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_4([a, e, f, g])
    } else {
        select_8([c, b, d])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, d < e, d < f, d < g
fn select_41([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_4([a, e, f, g])
    } else {
        select_18([c, b, d, h])
    }
}
/// n = 8, i = 2, b < h, c < d, c < h, d < e, d < f, d < g
fn select_42([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_40([a, c, b, d, e, f, g])
    } else {
        select_41([b, c, a, d, e, f, g, h])
    }
}
/// n = 8, i = 2, c < d, c < e, e < f, e < g, e < h
fn select_43([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_39([b, d, e, f, g, h, a])
    } else {
        select_42([b, a, c, e, f, g, h, d])
    }
}
/// n = 7, i = 2, a < g, b < f, c < d, c < f, c < g, d < e
fn select_44([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_30([a, c, b, d, e, g])
    } else {
        select_30([b, c, a, d, e, f])
    }
}
/// n = 2, i = 1
fn select_45([a, b]: [usize; 2]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_1([a])
    }
}
/// n = 4, i = 2, b < d, c < d
fn select_46([a, b, c, d]: [usize; 4]) -> usize {
    if b < a {
        select_8([a, c, d])
    } else {
        select_45([b, c])
    }
}
/// n = 7, i = 3, a < d, a < g, b < e, b < g, c < e, c < g, e < f
fn select_47([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_29([a, e, f, g])
    } else {
        select_46([d, b, c, g])
    }
}
/// n = 8, i = 3, a < h, b < g, c < e, c < g, c < h, d < e, d < g, d < h, e < f
fn select_48([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_47([a, c, d, b, e, f, h])
    } else {
        select_47([b, c, d, a, e, f, g])
    }
}
/// n = 9, i = 4, a < i, b < f, b < g, b < i, c < f, c < g, c < i, d < e, d < g, d < i, g < h
fn select_49([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_44([a, f, d, g, h, e, i])
    } else {
        select_48([a, e, b, c, g, h, f, i])
    }
}
/// n = 11, i = 4, b < f, b < g, b < h, c < f, c < g, c < h, d < e, d < g, d < h, g < k, h < i, h < j
fn select_50([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_7([e, f, g, h, a, i, j, k])
    } else {
        select_49([a, b, c, d, e, f, g, k, h])
    }
}
/// n = 11, i = 4, b < f, b < i, b < k, c < f, c < g, c < i, d < e, d < g, d < i, g < h, g < k, i < j
fn select_51([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_43([a, e, g, h, b, f, i, k])
    } else {
        select_50([a, b, c, d, e, f, i, g, h, k, j])
    }
}
/// n = 12, i = 4, b < e, b < j, c < h, c < i, c < j, d < f, d < h, d < j, f < g, f < i, i < k, j < l
fn select_52([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_37([a, c, f, g, h, b, e, i, k, j])
    } else {
        select_51([a, c, d, b, e, h, f, g, j, l, i])
    }
}
/// n = 13, i = 4, b < e, c < h, c < i, c < k, d < f, d < h, d < i, f < g, f < k, f < m, i < j, i < m, k < l
fn select_53([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < b {
        select_24([a, f, c, h, g, i, j, b, k, e, l, m])
    } else {
        select_52([a, b, c, d, e, f, g, h, k, i, l, j])
    }
}
/// n = 7, i = 2, a < d, a < g, b < c, b < e, b < g, e < f
fn select_54([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < a {
        select_29([c, e, f, a])
    } else {
        select_20([a, b, d, e, f, c, g])
    }
}
/// n = 8, i = 2, a < d, a < h, b < c, b < e, b < h, e < f, e < g
fn select_55([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_30([a, b, d, c, e, h])
    } else {
        select_41([a, b, d, e, c, f, g, h])
    }
}
/// n = 10, i = 2, a < d, a < e, a < j, b < c, b < f, b < j, e < i, f < g, f < h
fn select_56([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_54([b, a, d, c, e, i, j])
    } else {
        select_55([a, b, c, d, f, g, h, j])
    }
}
/// n = 10, i = 3, a < d, a < f, b < e, b < g, c < e, c < f, c < g, f < j, g < h, g < i
fn select_57([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_21([d, b, c, f, j, e, g])
    } else {
        select_55([a, c, e, d, g, h, i, f])
    }
}
/// n = 11, i = 3, a < d, b < g, b < h, b < k, c < e, c < g, c < h, e < f, e < k, h < i, h < j
fn select_58([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_56([e, b, g, f, a, h, i, j, d, k])
    } else {
        select_57([a, b, c, d, g, e, h, i, j, f])
    }
}
/// n = 11, i = 3, c < g, c < h, c < k, d < e, d < g, d < h, e < f, e < k, h < i, h < j
fn select_59([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_58([a, c, d, b, e, f, g, h, i, j, k])
    } else {
        select_58([b, c, d, a, e, f, g, h, i, j, k])
    }
}
/// n = 12, i = 3, c < f, c < i, c < j, d < e, d < i, d < j, e < g, f < h, j < k, j < l
fn select_60([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_59([a, b, c, d, e, g, i, j, k, l, f])
    } else {
        select_59([a, b, d, c, f, h, i, j, k, l, e])
    }
}
/// n = 6, i = 3, a < e, b < c, b < d, e < f
fn select_61([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < c {
        select_10([e, b, c, d, f])
    } else {
        select_8([d, a, e])
    }
}
/// n = 7, i = 3, a < d, b < c, b < g, c < f, d < e, d < g
fn select_62([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_8([f, a, d])
    } else {
        select_18([e, b, c, g])
    }
}
/// n = 7, i = 3, a < g, b < d, c < e, e < f, e < g
fn select_63([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_61([c, b, a, d, e, f])
    } else {
        select_62([c, a, b, e, f, d, g])
    }
}
/// n = 8, i = 3, a < d, b < c, b < g, b < h, c < f, d < e, d < g, d < h
fn select_64([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_1([a])
    } else {
        select_20([d, b, e, c, f, g, h])
    }
}
/// n = 8, i = 3, a < d, b < g, b < h, c < e, e < f, e < g, e < h
fn select_65([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_61([c, a, b, d, e, f])
    } else {
        select_64([c, b, a, e, f, d, g, h])
    }
}
/// n = 9, i = 3, a < d, b < g, b < h, b < i, c < e, c < f, e < i, f < g, f < h
fn select_66([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_63([b, a, c, d, e, f, i])
    } else {
        select_65([a, b, c, d, f, e, g, h])
    }
}
/// n = 5, i = 1, b < c, b < d, b < e
fn select_67([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < a {
        select_3([c, d, e])
    } else {
        select_18([a, b, d, e])
    }
}
/// n = 7, i = 2, a < g, b < d, b < e, b < f, c < d, c < e, c < f, c < g
fn select_68([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_8([b, c, g])
    } else {
        select_67([a, c, d, e, f])
    }
}
/// n = 8, i = 3, a < d, b < f, b < g, b < h, c < e, e < f, e < g, e < h
fn select_69([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_46([b, c, d, e])
    } else {
        select_68([a, b, e, f, g, h, d])
    }
}
/// n = 10, i = 3, a < d, b < f, b < j, c < e, c < h, c < i, e < g, e < j, f < h, f < i
fn select_70([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_65([a, b, c, d, e, g, f, j])
    } else {
        select_69([a, c, b, d, f, e, h, i])
    }
}
/// n = 10, i = 3, a < e, b < f, b < i, b < j, c < d, c < g, f < h, g < i, g < j
fn select_71([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_66([a, b, c, e, d, g, i, j, f])
    } else {
        select_70([a, c, b, e, f, g, h, i, j, d])
    }
}
/// n = 5, i = 2, a < d, a < e, b < c, b < e
fn select_72([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_8([c, a, e])
    } else {
        select_8([d, b, e])
    }
}
/// n = 3, i = 2
fn select_73([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_45([b, c])
    } else {
        select_45([a, c])
    }
}
/// n = 5, i = 3, a < e, b < e, c < d
fn select_74([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_73([a, b, d])
    } else {
        select_45([c, e])
    }
}
/// n = 6, i = 3, b < e, c < d, c < f, e < f
fn select_75([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_72([c, e, a, d, f])
    } else {
        select_74([a, b, c, d, e])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < e, c < f
fn select_76([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_72([a, c, d, b, f])
    } else {
        select_72([b, c, d, a, e])
    }
}
/// n = 8, i = 4, a < h, b < f, c < f, c < g, c < h, d < e, e < g, e < h
fn select_77([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < e {
        select_75([a, d, c, f, e, g])
    } else {
        select_76([a, b, c, g, f, h])
    }
}
/// n = 10, i = 4, a < f, b < h, b < i, b < j, c < f, c < g, c < j, d < e, d < g, e < j, g < h, g < i
fn select_78([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_77([b, a, c, d, e, f, g, j])
    } else {
        select_65([a, b, d, f, g, e, h, i])
    }
}
/// n = 11, i = 4, a < f, b < g, b < j, b < k, c < e, c < h, d < f, d < g, d < h, g < i, h < j, h < k
fn select_79([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_70([a, c, b, f, g, h, i, j, k, e])
    } else {
        select_78([a, b, d, c, e, f, h, j, k, g])
    }
}
/// n = 11, i = 4, b < g, b < j, b < k, c < f, c < h, d < e, d < g, d < h, g < i, h < j, h < k
fn select_80([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_71([e, b, c, f, a, g, h, i, j, k])
    } else {
        select_79([a, b, c, d, f, e, g, h, i, j, k])
    }
}
/// n = 13, i = 4, b < f, c < h, c < j, c < k, d < e, d < g, d < h, g < j, g < k, h < i, k < l, k < m
fn select_81([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < b {
        select_60([a, e, c, g, b, h, f, i, j, k, l, m])
    } else {
        select_80([a, c, b, d, e, f, h, g, i, j, k])
    }
}
/// n = 13, i = 4, b < g, c < h, c < j, d < f, d < i, d < m, e < h, e < i, e < j, i < k, j < l, j < m
fn select_82([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < d {
        select_53([a, b, e, c, g, d, f, h, j, l, i, k, m])
    } else {
        select_81([a, b, e, d, f, g, c, i, k, h, j, l, m])
    }
}
/// n = 10, i = 1, c < j, d < h, d < i, e < f, e < g
fn select_83([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_7([a, b, c, d, e, h, i, j])
    } else {
        select_7([a, b, c, e, d, f, g, j])
    }
}
/// n = 7, i = 1, b < f, b < g, c < d, c < e
fn select_84([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_19([a, c, d, e, b])
    } else {
        select_5([c, b, a, f, g, d, e])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, c < d, c < e, c < f, c < g
fn select_85([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_18([d, b, e, f])
    } else {
        select_76([a, b, c, d, f, g])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < e, c < g, c < h, e < f
fn select_86([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_84([d, b, e, a, f, g, h])
    } else {
        select_85([a, b, c, d, g, h, e])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < f, c < j, e < i, f < g, f < h
fn select_87([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_7([a, d, e, f, b, g, h, i])
    } else {
        select_86([a, b, c, d, e, i, f, j])
    }
}
/// n = 11, i = 2, c < d, c < e, c < f, c < g, e < k, f < j, g < h, g < i
fn select_88([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_83([b, d, f, e, g, h, i, a, k, j])
    } else {
        select_87([b, a, c, d, f, g, h, i, j, e])
    }
}
/// n = 12, i = 2, c < f, c < i, d < e, d < g, d < h, g < k, h < j, i < l
fn select_89([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < d {
        select_13([a, b, c, f, i, d, e, g, h, l])
    } else {
        select_88([a, b, d, e, g, h, c, f, i, j, k])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < f
fn select_90([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_32([a, d, b, e, f])
    } else {
        select_32([a, e, c, d, f])
    }
}
/// n = 8, i = 3, a < e, a < h, b < f, b < h, c < d, c < f, c < h, f < g
fn select_91([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_90([b, a, c, d, e, h])
    } else {
        select_33([d, a, b, f, g, h])
    }
}
/// n = 11, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_92([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_17([d, f, g, a, h, i, j, k])
    } else {
        select_91([a, b, c, d, e, f, k, g])
    }
}
/// n = 9, i = 4, a < f, a < i, b < g, b < i, c < f, c < g, c < i, d < e, d < g, d < i, g < h
fn select_93([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < c {
        select_32([a, b, c, g, f])
    } else {
        select_91([a, b, d, e, f, g, h, i])
    }
}
/// n = 12, i = 4, a < f, b < g, b < h, c < f, c < g, c < h, d < e, d < g, d < h, g < l, h < i, h < j, h < k
fn select_94([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_17([e, g, h, a, i, j, k, l])
    } else {
        select_93([a, b, c, d, e, f, g, l, h])
    }
}
/// n = 12, i = 4, b < g, b < h, c < f, c < g, c < h, d < e, d < g, d < h, g < l, h < i, h < j, h < k
fn select_95([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_92([f, b, d, e, a, g, h, i, j, k, l])
    } else {
        select_94([a, b, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 14, i = 4, b < f, b < k, c < g, c < k, d < e, d < g, d < k, g < h, g < i, g < j, i < m, j < l, k < n
fn select_96([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_88([a, e, g, h, i, j, b, f, k, l, m])
    } else {
        select_95([a, c, b, d, e, f, k, g, h, i, j, n])
    }
}
/// n = 14, i = 4, b < f, c < g, c < h, d < e, d < g, d < h, g < j, h < i, h < k, h < l, k < n, l < m
fn select_97([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_89([a, e, g, h, i, j, k, l, b, m, n, f])
    } else {
        select_96([a, b, c, d, e, f, h, i, k, l, g, m, n, j])
    }
}
/// n = 15, i = 5, b < e, c < g, c < k, d < f, d < j, f < h, f < k, f < n, g < i, g < j, g < n, j < l, k < m, n < o
fn select_98([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < g {
        select_97([a, b, c, f, h, e, k, g, i, m, j, n, o, l])
    } else {
        select_97([a, b, d, g, i, e, j, f, h, l, k, n, o, m])
    }
}
/// n = 15, i = 5, b < e, c < g, c < k, c < l, d < f, d < h, f < k, g < j, g < n, h < i, h < l, h < n, l < m, n < o
fn select_99([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < g {
        select_82([a, b, f, h, c, i, e, k, l, g, m, j, n])
    } else {
        select_98([a, b, c, d, e, h, g, i, j, f, l, k, m, n, o])
    }
}
/// n = 4, i = 2, a < d, b < c
fn select_100([a, b, c, d]: [usize; 4]) -> usize {
    if c < d {
        select_45([a, c])
    } else {
        select_45([b, d])
    }
}
/// n = 7, i = 3, a < e, a < g, b < e, b < f, b < g, c < d, c < g, d < f
fn select_101([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_8([a, b, f])
    } else {
        select_72([b, c, d, e, g])
    }
}
/// n = 7, i = 3, a < f, a < g, b < d, b < g, c < e, c < g, e < f
fn select_102([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_100([c, d, a, e])
    } else {
        select_101([b, a, c, e, d, f, g])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < e
fn select_103([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < d {
        select_10([a, c, d, e, f])
    } else {
        select_8([a, b, e])
    }
}
/// n = 7, i = 4, a < g, b < f, c < e, c < g, d < e, d < f
fn select_104([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_103([b, d, c, g, e, f])
    } else {
        select_103([a, c, d, f, e, g])
    }
}
/// n = 8, i = 4, a < g, b < g, b < h, c < f, c < h, d < e, d < f, e < h
fn select_105([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_102([b, c, e, f, a, g, h])
    } else {
        select_104([b, c, a, d, e, f, g])
    }
}
/// n = 7, i = 4, b < f, c < f, d < e, d < g, f < g
fn select_106([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < e {
        select_75([a, c, d, e, f, g])
    } else {
        select_46([a, b, c, f])
    }
}
/// n = 5, i = 3, c < e, d < e
fn select_107([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_74([c, d, a, b, e])
    } else {
        select_74([c, d, b, a, e])
    }
}
/// n = 8, i = 4, a < g, b < f, c < f, d < e, d < h, e < g, f < h
fn select_108([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_106([a, b, c, d, e, f, h])
    } else {
        select_107([b, c, a, e, g])
    }
}
/// n = 5, i = 2, a < e, b < e, c < d
fn select_109([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_9([c, a, d, b, e])
    } else {
        select_9([c, b, d, a, e])
    }
}
/// n = 6, i = 3, a < f, b < f, c < e, d < e, d < f
fn select_110([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_32([b, c, d, e, f])
    } else {
        select_109([a, b, c, e, f])
    }
}
/// n = 8, i = 2, a < c, b < d, d < e, d < f, d < g, d < h
fn select_111([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_14([a, e, f, g, h])
    } else {
        select_8([c, b, d])
    }
}
/// n = 9, i = 3, b < e, c < f, d < e, d < f, f < g, f < h, f < i
fn select_112([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_110([a, c, b, d, e, f])
    } else {
        select_111([b, c, e, f, a, g, h, i])
    }
}
/// n = 12, i = 4, a < f, b < g, b < h, c < f, c < h, d < e, d < h, d < l, e < g, f < l, h < i, h < j, h < k
fn select_113([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_108([b, a, c, d, e, f, g, l])
    } else {
        select_112([a, e, c, b, g, h, i, j, k])
    }
}
/// n = 12, i = 4, a < f, b < g, c < h, c < l, d < f, d < h, e < g, e < h, g < l, h < i, h < j, h < k
fn select_114([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < c {
        select_105([b, e, d, a, c, f, g, h])
    } else {
        select_113([b, d, e, c, a, g, f, h, i, j, k, l])
    }
}
/// n = 5, i = 2, b < e, c < d
fn select_115([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < a {
        select_10([c, b, a, e, d])
    } else {
        select_9([c, a, d, b, e])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < f
fn select_116([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < c {
        select_8([a, e, c])
    } else {
        select_76([a, b, c, d, e, f])
    }
}
/// n = 9, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_117([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_4([a, g, h, i])
    } else {
        select_72([b, c, d, e, f])
    }
}
/// n = 11, i = 3, a < d, a < e, a < f, b < f, b < g, c < e, c < g, c < k, d < k, g < h, g < i, g < j
fn select_118([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < g {
        select_116([d, b, c, e, f, k])
    } else {
        select_117([a, b, c, e, f, g, h, i, j])
    }
}
/// n = 7, i = 3, a < g, b < d, b < f, c < e, e < f, e < g
fn select_119([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if g < b {
        select_1([g])
    } else {
        select_75([a, c, b, d, e, f])
    }
}
/// n = 8, i = 3, a < d, b < e, c < e, e < f, e < g, e < h
fn select_120([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_4([a, f, g, h])
    } else {
        select_46([d, b, c, e])
    }
}
/// n = 12, i = 4, a < e, a < f, b < g, b < l, c < e, c < g, c < k, d < f, d < g, f < k, f < l, g < h, g < i, g < j
fn select_121([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_119([b, c, d, e, f, k, l])
    } else {
        select_120([a, b, d, e, g, h, i, j])
    }
}
/// n = 12, i = 4, a < f, b < h, b < i, c < f, c < g, c < i, d < e, d < i, e < g, e < h, i < j, i < k, i < l
fn select_122([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_118([e, b, c, a, g, h, i, j, k, l, f])
    } else {
        select_121([a, b, c, d, f, e, i, j, k, l, g, h])
    }
}
/// n = 12, i = 4, a < g, b < h, b < i, c < g, c < i, d < e, d < i, e < f, e < h, i < j, i < k, i < l
fn select_123([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < c {
        select_115([a, b, f, c, h])
    } else {
        select_122([a, b, c, d, e, g, f, h, i, j, k, l])
    }
}
/// n = 12, i = 4, b < g, c < h, c < i, d < g, d < i, e < f, e < i, f < h, i < j, i < k, i < l
fn select_124([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < f {
        select_114([b, a, c, d, e, g, f, i, j, k, l, h])
    } else {
        select_123([b, c, d, e, f, a, g, h, i, j, k, l])
    }
}
/// n = 12, i = 4, b < h, c < h, c < i, d < g, d < i, e < f, e < i, i < j, i < k, i < l
fn select_125([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_124([a, b, d, c, e, f, h, g, i, j, k, l])
    } else {
        select_124([a, b, e, c, d, g, h, f, i, j, k, l])
    }
}
/// n = 5, i = 1, c < d, c < e
fn select_126([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_19([a, c, d, e, b])
    } else {
        select_19([b, c, d, e, a])
    }
}
/// n = 6, i = 1, c < d, c < e, c < f
fn select_127([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_38([a, c, d, e, f, b])
    } else {
        select_38([b, c, d, e, f, a])
    }
}
/// n = 7, i = 1, c < g, d < e, d < f
fn select_128([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_126([a, b, c, d, g])
    } else {
        select_127([a, b, d, c, e, f])
    }
}
/// n = 9, i = 1, c < i, d < h, e < f, e < g
fn select_129([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < e {
        select_128([a, b, d, c, e, i, h])
    } else {
        select_7([a, b, d, e, c, f, g, h])
    }
}
/// n = 6, i = 1, b < c, b < d, b < e, b < f
fn select_130([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_4([c, d, e, f])
    } else {
        select_67([a, b, d, e, f])
    }
}
/// n = 10, i = 2, a < d, a < j, b < c, b < e, b < j, d < i, e < f, e < g, e < h
fn select_131([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_130([c, e, a, f, g, h])
    } else {
        select_20([b, a, c, d, i, e, j])
    }
}
/// n = 8, i = 3, a < h, b < d, b < f, c < e, c < g, d < g, e < f, e < h
fn select_132([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_9([b, c, d, e, h])
    } else {
        select_33([a, d, c, e, f, g])
    }
}
/// n = 9, i = 3, a < i, b < d, b < g, c < e, c < h, d < h, e < f, e < g, e < i
fn select_133([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_76([a, b, e, f, g, i])
    } else {
        select_132([a, b, c, d, e, g, h, i])
    }
}
/// n = 11, i = 3, b < d, b < j, c < e, c < k, d < k, e < f, e < g, e < j, g < h, g < i
fn select_134([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_131([b, e, f, d, g, a, h, i, k, j])
    } else {
        select_133([a, b, c, d, e, f, j, k, g])
    }
}
/// n = 12, i = 3, b < d, b < k, c < e, c < l, d < l, e < f, e < g, e < h, e < k, g < j, h < i
fn select_135([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_134([a, b, c, d, e, f, g, h, j, k, l])
    } else {
        select_134([a, b, c, d, e, f, h, g, i, k, l])
    }
}
/// n = 13, i = 3, b < d, c < e, c < m, d < m, e < f, e < g, e < h, e < i, g < l, h < k, i < j
fn select_136([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < b {
        select_129([a, f, g, h, i, b, j, k, l])
    } else {
        select_135([a, b, c, d, e, f, g, h, k, l, i, m])
    }
}
/// n = 11, i = 1, c < k, d < j, e < i, f < g, f < h
fn select_137([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_83([a, b, e, c, f, g, h, d, k, i])
    } else {
        select_83([a, b, e, d, f, g, h, c, j, i])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < i, d < e, d < f, d < g, d < h
fn select_138([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_14([a, e, f, g, h])
    } else {
        select_18([c, b, d, i])
    }
}
/// n = 9, i = 2, b < i, c < d, c < i, d < e, d < f, d < g, d < h
fn select_139([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_111([a, c, b, d, e, f, g, h])
    } else {
        select_138([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 10, i = 3, b < e, b < f, c < d, c < f, f < g, f < h, f < i, f < j
fn select_140([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_139([a, d, b, f, g, h, i, j, e])
    } else {
        select_139([a, e, c, f, g, h, i, j, d])
    }
}
/// n = 14, i = 4, b < e, b < f, c < f, c < g, d < e, d < g, g < h, g < i, g < j, g < k, i < n, j < m, k < l
fn select_141([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_137([a, h, i, j, k, b, e, f, l, m, n])
    } else {
        select_140([a, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 14, i = 4, b < f, c < f, c < g, d < e, d < g, g < h, g < i, g < j, g < k, i < n, j < m, k < l
fn select_142([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < b {
        select_136([a, e, c, b, g, h, i, j, k, l, m, n, f])
    } else {
        select_141([a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 4, b < h, c < g, d < h, d < i, e < f, e < i, i < j, i < k, i < l, k < n, l < m
fn select_143([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if c < i {
        select_125([a, b, d, c, e, f, g, h, i, j, k, l])
    } else {
        select_142([a, b, d, e, f, h, i, j, c, k, l, m, n, g])
    }
}
/// n = 7, i = 1, d < e, d < f, d < g
fn select_144([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_39([c, a, d, e, f, g, b])
    } else {
        select_39([c, b, d, e, f, g, a])
    }
}
/// n = 9, i = 1, d < h, d < i, e < f, e < g
fn select_145([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_144([a, b, c, d, e, h, i])
    } else {
        select_144([a, b, c, e, d, f, g])
    }
}
/// n = 7, i = 1, c < d, c < e, c < f, c < g
fn select_146([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_15([a, c, d, e, f, g, b])
    } else {
        select_15([b, c, d, e, f, g, a])
    }
}
/// n = 5, i = 2, b < e, c < d, c < e
fn select_147([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_100([a, c, d, b])
    } else {
        select_72([b, c, d, a, e])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < f
fn select_148([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_147([a, b, c, d, f])
    } else {
        select_147([a, c, b, e, f])
    }
}
/// n = 9, i = 2, b < e, c < d, c < f, f < g, f < h, f < i
fn select_149([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_146([a, d, f, b, g, h, i])
    } else {
        select_148([a, b, c, d, e, f])
    }
}
/// n = 12, i = 3, a < d, a < e, b < c, b < f, e < i, f < g, f < h, f < j, j < k, j < l
fn select_150([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_145([c, g, h, a, j, k, l, d, e])
    } else {
        select_149([d, e, b, c, i, f, g, h, j])
    }
}
/// n = 6, i = 2, a < f, b < e, b < f, c < d, c < e
fn select_151([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_8([d, b, e])
    } else {
        select_109([a, b, c, d, f])
    }
}
/// n = 9, i = 2, b < d, b < i, c < e, c < i, e < f, e < g, e < h
fn select_152([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_151([a, c, b, d, i, e])
    } else {
        select_138([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 10, i = 3, b < d, b < g, c < e, c < g, e < f, g < h, g < i, g < j
fn select_153([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_42([a, d, c, g, h, i, j, e])
    } else {
        select_152([a, e, b, f, g, h, i, j, d])
    }
}
/// n = 13, i = 4, a < e, a < f, b < d, b < g, c < f, c < g, f < j, g < h, g < i, g < k, k < l, k < m
fn select_154([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_145([d, h, i, a, k, l, m, e, f])
    } else {
        select_153([e, b, c, d, f, j, g, h, i, k])
    }
}
/// n = 13, i = 4, a < e, b < d, b < h, c < f, c < h, f < g, h < i, h < j, h < k, k < l, k < m
fn select_155([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_150([f, b, d, g, a, h, i, j, e, k, l, m])
    } else {
        select_154([a, b, c, d, e, f, h, i, j, g, k, l, m])
    }
}
/// n = 13, i = 3, a < d, a < e, b < c, b < f, e < h, f < g, f < i, f < j, i < m, j < k, j < l
fn select_156([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_83([c, g, i, a, j, k, l, d, e, m])
    } else {
        select_149([d, e, b, c, h, f, g, i, j])
    }
}
/// n = 14, i = 4, a < e, a < f, b < d, b < g, c < f, c < g, f < i, g < h, g < j, g < k, j < n, k < l, k < m
fn select_157([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < a {
        select_83([d, h, j, a, k, l, m, e, f, n])
    } else {
        select_153([e, b, c, d, f, i, g, h, j, k])
    }
}
/// n = 14, i = 4, a < e, b < d, b < h, c < f, c < h, f < g, h < i, h < j, h < k, j < n, k < l, k < m
fn select_158([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < a {
        select_156([f, b, d, g, a, h, i, e, j, k, l, m, n])
    } else {
        select_157([a, b, c, d, e, f, h, i, g, j, k, l, m, n])
    }
}
/// n = 14, i = 4, a < e, b < d, b < h, b < i, c < f, c < h, c < i, f < g, h < k, h < l, i < j, i < m, m < n
fn select_159([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_155([a, b, c, d, e, f, g, h, k, l, i, j, m])
    } else {
        select_158([a, b, c, d, e, f, g, i, j, m, h, k, l, n])
    }
}
/// n = 12, i = 3, a < d, a < e, b < c, b < f, e < h, f < g, f < i, f < j, i < l, j < k
fn select_160([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_129([c, g, i, j, a, d, e, k, l])
    } else {
        select_149([d, e, b, c, h, f, g, i, j])
    }
}
/// n = 13, i = 4, a < e, a < f, b < d, b < g, c < f, c < g, f < i, g < h, g < j, g < k, j < m, k < l
fn select_161([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_129([d, h, j, k, a, e, f, l, m])
    } else {
        select_153([e, b, c, d, f, i, g, h, j, k])
    }
}
/// n = 13, i = 4, a < e, b < d, b < h, c < f, c < h, f < g, h < i, h < j, h < k, j < m, k < l
fn select_162([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_160([f, b, d, g, a, h, i, e, j, k, l, m])
    } else {
        select_161([a, b, c, d, e, f, h, i, g, j, k, l, m])
    }
}
/// n = 6, i = 2, c < d, c < e, e < f
fn select_163([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_126([b, d, e, a, f])
    } else {
        select_147([b, a, c, d, e])
    }
}
/// n = 6, i = 2, c < d, d < e, d < f
fn select_164([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < c {
        select_8([b, c, d])
    } else {
        select_126([a, b, d, e, f])
    }
}
/// n = 8, i = 4, b < e, c < d, c < h, d < g, e < f, e < h
fn select_165([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_163([a, f, c, h, d, g])
    } else {
        select_164([a, g, b, e, f, h])
    }
}
/// n = 7, i = 3, b < d, b < g, c < e, e < f, e < g
fn select_166([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_147([a, b, e, f, g])
    } else {
        select_75([a, c, b, d, e, g])
    }
}
/// n = 9, i = 5, a < d, a < i, b < f, c < e, c < h, e < g, f < h, f < i
fn select_167([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_165([d, b, c, e, f, i, g, h])
    } else {
        select_166([g, a, b, d, f, h, i])
    }
}
/// n = 7, i = 3, b < e, c < d, c < f, c < g, e < f, e < g
fn select_168([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_75([a, b, c, d, e, g])
    } else {
        select_18([d, e, f, g])
    }
}
/// n = 6, i = 3, a < e, b < e, b < f, c < d, d < f
fn select_169([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_100([b, c, d, e])
    } else {
        select_8([a, b, f])
    }
}
/// n = 8, i = 3, b < e, b < h, c < d, c < f, c < g, d < h, e < f, e < g
fn select_170([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_169([a, b, c, d, e, h])
    } else {
        select_20([e, c, a, d, h, f, g])
    }
}
/// n = 10, i = 4, b < e, c < d, c < h, d < f, d < i, d < j, e < g, e < i, e < j, g < h
fn select_171([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_168([a, b, d, f, e, i, j])
    } else {
        select_170([a, c, e, g, d, i, j, h])
    }
}
/// n = 10, i = 4, b < g, b < i, b < j, c < g, c < i, c < j, d < e, d < h, e < f, e < i, e < j, g < h
fn select_172([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_171([a, b, d, e, c, f, g, h, i, j])
    } else {
        select_171([a, c, d, e, b, f, g, h, i, j])
    }
}
/// n = 11, i = 5, a < e, a < j, a < k, b < d, b < f, c < g, c < i, e < i, f < j, f < k, g < h, g < j, g < k
fn select_173([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_167([b, a, c, d, g, e, h, i, f])
    } else {
        select_172([d, a, f, c, g, h, e, i, j, k])
    }
}
/// n = 10, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, f < j, g < h, g < i
fn select_174([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_39([d, f, g, a, h, i, j])
    } else {
        select_91([a, b, c, d, e, f, j, g])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < e, b < i, d < h, e < f, e < g
fn select_175([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_38([d, e, a, f, g, h])
    } else {
        select_20([a, b, c, d, h, e, i])
    }
}
/// n = 12, i = 4, a < d, a < f, b < f, b < g, b < h, c < e, c < l, e < g, e < h, f < l, g < k, h < i, h < j
fn select_176([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_175([a, b, d, g, h, i, j, k, f])
    } else {
        select_170([d, c, b, f, e, g, h, l])
    }
}
/// n = 13, i = 4, a < d, a < g, b < g, b < h, b < i, c < e, c < m, e < f, e < h, e < i, g < m, h < l, i < j, i < k
fn select_177([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_174([a, b, e, f, d, h, i, j, k, l])
    } else {
        select_176([a, b, c, d, e, g, h, i, j, k, l, m])
    }
}
/// n = 6, i = 2, a < f, b < d, b < e, c < d, c < e, c < f
fn select_178([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_8([b, c, f])
    } else {
        select_18([a, c, d, e])
    }
}
/// n = 7, i = 2, a < c, a < g, b < d, b < g, d < e, d < f
fn select_179([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_3([a, e, f])
    } else {
        select_18([c, b, d, g])
    }
}
/// n = 8, i = 3, a < d, a < g, a < h, b < c, b < e, d < f, e < g, e < h
fn select_180([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_178([c, e, a, g, h, d])
    } else {
        select_179([d, b, f, e, g, h, c])
    }
}
/// n = 12, i = 4, a < e, a < f, b < f, b < h, b < i, c < d, c < e, d < h, d < i, f < g, h < l, i < j, i < k
fn select_181([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < a {
        select_175([a, b, e, h, i, j, k, l, f])
    } else {
        select_180([b, c, e, f, d, g, h, i])
    }
}
/// n = 13, i = 4, a < f, a < g, b < g, b < i, b < j, c < d, c < f, d < e, d < i, d < j, g < h, i < m, j < k, j < l
fn select_182([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < g {
        select_174([a, b, d, e, f, i, j, k, l, m])
    } else {
        select_181([a, b, c, d, f, g, h, i, j, k, l, m])
    }
}
/// n = 14, i = 5, a < d, a < j, a < k, b < e, b < j, b < k, c < f, c < i, d < i, e < h, f < g, f < j, f < k, j < n, k < l, k < m
fn select_183([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < d {
        select_177([e, a, c, h, f, g, d, j, k, l, m, n, i])
    } else {
        select_182([d, b, c, f, g, i, e, h, j, k, l, m, n])
    }
}
/// n = 14, i = 5, a < e, b < j, b < k, c < f, c < j, c < k, d < g, d < i, f < i, g < h, g < j, g < k, j < n, k < l, k < m
fn select_184([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_173([c, a, d, e, f, b, g, h, i, j, k])
    } else {
        select_183([c, b, d, f, a, g, h, e, i, j, k, l, m, n])
    }
}
/// n = 15, i = 5, a < e, b < m, c < f, c < j, c < m, d < g, d < i, f < i, g < h, g < j, g < m, j < k, j < l, l < n, m < o
fn select_185([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < b {
        select_162([a, g, c, h, e, f, i, j, k, b, l, n, m])
    } else {
        select_184([a, b, c, d, e, f, g, h, i, m, j, k, l, o])
    }
}
/// n = 15, i = 5, b < e, c < f, c < j, c < k, d < g, d < i, f < i, g < h, g < j, g < k, j < m, k < l, k < n, n < o
fn select_186([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < a {
        select_159([b, g, c, h, e, f, i, j, k, l, a, m, n, o])
    } else {
        select_185([b, a, c, d, e, f, g, h, i, k, l, n, j, o, m])
    }
}
/// n = 15, i = 5, b < e, c < h, c < k, d < f, d < g, d < l, f < k, g < i, g < n, h < j, h < l, h < n, l < m, n < o
fn select_187([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < h {
        select_143([a, f, b, c, g, i, e, k, h, j, l, n, o, m])
    } else {
        select_186([a, b, d, c, e, f, h, j, k, l, g, i, m, n, o])
    }
}
/// n = 15, i = 5, b < g, c < f, c < i, d < e, d < h, d < l, h < k, h < n, i < j, i < l, i < n, l < m, n < o
fn select_188([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < e {
        select_99([a, b, d, c, g, f, h, i, j, k, e, l, m, n, o])
    } else {
        select_187([a, b, c, d, g, e, h, i, k, j, f, l, m, n, o])
    }
}
/// n = 8, i = 2, b < d, c < e, e < f, e < g, e < h
fn select_189([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_109([a, c, b, d, e])
    } else {
        select_111([b, c, d, e, a, f, g, h])
    }
}
/// n = 7, i = 2, a < f, a < g, b < d, b < e, b < g, c < d, c < e, c < f
fn select_190([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_29([b, c, f, g])
    } else {
        select_19([a, c, d, e, g])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, b < g, c < d, c < e, c < f
fn select_191([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_19([d, b, e, f, a])
    } else {
        select_190([a, b, c, e, f, d, g])
    }
}
/// n = 10, i = 2, a < c, a < i, a < j, b < d, b < i, b < j, d < e, d < f, d < g, d < h
fn select_192([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_14([a, e, f, g, h])
    } else {
        select_67([c, b, d, i, j])
    }
}
/// n = 10, i = 2, b < d, b < i, b < j, c < e, c < i, c < j, e < f, e < g, e < h
fn select_193([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_191([a, c, b, d, i, j, e])
    } else {
        select_192([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 10, i = 2, c < i, c < j, d < e, d < i, d < j, e < f, e < g, e < h
fn select_194([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < c {
        select_189([b, a, d, c, e, f, g, h])
    } else {
        select_193([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 11, i = 3, b < d, b < e, b < i, c < f, c < j, c < k, e < j, e < k, f < g, f < h, f < i
fn select_195([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_144([a, g, h, b, d, e, i])
    } else {
        select_194([a, d, e, c, f, g, h, i, j, k])
    }
}
/// n = 7, i = 2, a < e, a < f, a < g, b < c, b < d, b < f, b < g
fn select_196([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_20([a, b, e, c, d, f, g])
    } else {
        select_20([a, b, e, d, c, f, g])
    }
}
/// n = 7, i = 2, b < e, b < f, b < g, c < d, c < f, c < g
fn select_197([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_9([c, a, d, b, e])
    } else {
        select_196([c, b, a, e, d, f, g])
    }
}
/// n = 7, i = 2, c < f, c < g, d < e, d < f, d < g
fn select_198([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_115([b, a, d, e, c])
    } else {
        select_197([b, c, d, e, a, f, g])
    }
}
/// n = 7, i = 3, b < e, c < f, d < e, d < f, f < g
fn select_199([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < b {
        select_33([a, b, d, f, g, e])
    } else {
        select_147([a, c, d, e, f])
    }
}
/// n = 6, i = 3, b < f, c < e, d < e, d < f
fn select_200([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < c {
        select_147([a, c, d, f, e])
    } else {
        select_147([a, b, d, e, f])
    }
}
/// n = 7, i = 4, b < g, c < f, c < g, d < e, d < g
fn select_201([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_200([a, b, e, c, f, g])
    } else {
        select_200([a, b, f, d, e, g])
    }
}
/// n = 9, i = 4, a < h, b < f, b < g, c < g, c < h, d < e, d < g, d < h, h < i
fn select_202([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_199([e, b, a, c, g, h, i])
    } else {
        select_201([a, c, b, d, e, f, g])
    }
}
/// n = 11, i = 4, a < h, b < h, b < i, b < j, c < f, c < g, d < e, d < g, d < h, g < i, g < j, h < k
fn select_203([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_198([e, f, b, g, a, i, j])
    } else {
        select_202([b, c, a, d, e, f, g, h, k])
    }
}
/// n = 12, i = 4, b < h, b < k, b < l, c < f, c < g, c < j, d < e, d < g, d < h, g < k, g < l, h < i, h < j
fn select_204([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_195([e, c, b, f, g, h, a, i, j, k, l])
    } else {
        select_203([a, b, c, d, e, f, g, h, k, l, i])
    }
}
/// n = 14, i = 4, b < f, b < n, c < h, c < j, c < k, d < e, d < g, d < h, g < j, g < k, h < i, h < n, k < l, k < m
fn select_205([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_60([a, e, c, g, b, h, f, i, j, k, l, m])
    } else {
        select_204([a, c, b, d, e, f, g, h, i, n, j, k])
    }
}
/// n = 14, i = 4, b < g, b < n, c < h, c < j, d < f, d < i, d < m, e < h, e < i, e < j, i < k, i < n, j < l, j < m
fn select_206([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if c < d {
        select_53([a, b, e, c, g, d, f, h, j, l, i, k, m])
    } else {
        select_205([a, b, e, d, f, g, c, i, k, h, j, l, m, n])
    }
}
/// n = 9, i = 1, a < g, a < h, a < i, b < c, b < d, b < e, b < f
fn select_207([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_4([b, g, h, i])
    } else {
        select_14([a, c, d, e, f])
    }
}
/// n = 9, i = 1, b < h, b < i, c < d, c < e, c < f, c < g
fn select_208([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_15([a, c, d, e, f, g, b])
    } else {
        select_207([b, c, d, e, f, g, a, h, i])
    }
}
/// n = 11, i = 2, a < d, b < c, b < e, b < f, e < j, e < k, f < g, f < h, f < i
fn select_209([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_208([c, e, f, a, g, h, i, j, k])
    } else {
        select_55([a, b, c, d, e, j, k, f])
    }
}
/// n = 11, i = 2, c < d, c < e, c < f, e < j, e < k, f < g, f < h, f < i
fn select_210([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_209([a, c, d, b, e, f, g, h, i, j, k])
    } else {
        select_209([b, c, d, a, e, f, g, h, i, j, k])
    }
}
/// n = 13, i = 2, c < f, c < i, d < e, d < g, d < h, g < k, h < j, i < l, i < m
fn select_211([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < d {
        select_210([a, b, c, f, i, d, e, g, h, l, m])
    } else {
        select_88([a, b, d, e, g, h, c, f, i, j, k])
    }
}
/// n = 9, i = 3, a < d, b < e, c < e, e < f, e < g, e < h, e < i
fn select_212([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_14([a, f, g, h, i])
    } else {
        select_46([d, b, c, e])
    }
}
/// n = 9, i = 3, c < e, d < e, e < f, e < g, e < h, e < i
fn select_213([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_212([a, c, d, b, e, f, g, h, i])
    } else {
        select_212([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 12, i = 3, c < e, c < f, d < e, d < f, e < j, e < k, e < l, f < g, f < h, f < i
fn select_214([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_213([a, b, c, d, e, f, j, k, l])
    } else {
        select_213([a, b, c, d, f, e, g, h, i])
    }
}
/// n = 7, i = 2, a < c, b < d, c < g, d < e, d < f
fn select_215([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_8([b, c, g])
    } else {
        select_18([a, d, e, f])
    }
}
/// n = 8, i = 2, a < c, b < d, c < h, d < e, d < f, d < g
fn select_216([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_3([e, f, g])
    } else {
        select_215([a, b, c, d, f, g, h])
    }
}
/// n = 10, i = 3, a < d, a < e, a < j, b < f, b < j, c < e, c < f, c < j, f < g, f < h, f < i
fn select_217([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_147([d, b, c, e, j])
    } else {
        select_41([a, b, e, f, g, h, i, j])
    }
}
/// n = 10, i = 3, a < e, a < j, b < f, b < j, c < d, c < f, c < j, f < g, f < h, f < i
fn select_218([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_216([d, b, a, f, g, h, i, e])
    } else {
        select_217([a, b, c, e, d, f, g, h, i, j])
    }
}
/// n = 9, i = 3, a < d, b < e, c < e, c < i, d < i, e < f, e < g, e < h
fn select_219([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_4([a, f, g, h])
    } else {
        select_32([b, d, c, i, e])
    }
}
/// n = 9, i = 3, a < i, b < i, c < e, d < e, d < i, e < f, e < g, e < h
fn select_220([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_219([a, c, d, b, e, f, g, h, i])
    } else {
        select_219([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 10, i = 4, a < f, b < g, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_221([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_220([a, e, b, c, g, h, i, j, f])
    } else {
        select_219([a, b, d, f, g, h, i, j, e])
    }
}
/// n = 11, i = 4, b < g, b < k, c < f, c < g, d < e, d < g, d < k, f < k, g < h, g < i, g < j
fn select_222([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_218([f, b, d, e, a, g, h, i, j, k])
    } else {
        select_221([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 13, i = 4, b < g, b < h, c < f, c < g, c < h, d < e, d < g, d < h, g < l, g < m, h < i, h < j, h < k
fn select_223([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_214([a, f, b, d, g, h, i, j, k, e, l, m])
    } else {
        select_222([a, b, d, c, f, e, h, i, j, k, g])
    }
}
/// n = 15, i = 4, b < f, b < k, c < g, c < k, d < e, d < g, d < k, g < h, g < i, g < j, i < m, j < l, k < n, k < o
fn select_224([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < b {
        select_88([a, e, g, h, i, j, b, f, k, l, m])
    } else {
        select_223([a, c, b, d, e, f, k, g, h, i, j, n, o])
    }
}
/// n = 15, i = 4, b < f, b < o, c < g, c < h, d < e, d < g, d < h, g < j, g < o, h < i, h < k, h < l, k < n, l < m
fn select_225([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < b {
        select_211([a, e, g, h, i, j, k, l, b, m, n, f, o])
    } else {
        select_224([a, b, c, d, e, f, h, i, k, l, g, m, n, j, o])
    }
}
/// n = 16, i = 5, b < e, b < p, c < g, c < j, d < f, d < k, f < i, f < j, f < n, g < h, g < k, g < n, j < m, k < l, k < p, n < o
fn select_226([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < g {
        select_97([a, b, c, f, i, e, j, g, h, m, k, n, o, l])
    } else {
        select_225([a, b, d, g, h, e, k, f, i, l, j, n, o, m, p])
    }
}
/// n = 16, i = 5, b < e, b < p, c < g, c < k, c < l, d < f, d < h, f < k, g < j, g < n, h < i, h < l, h < n, l < m, l < p, n < o
fn select_227([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < g {
        select_206([a, b, f, h, c, i, e, k, l, g, m, j, n, p])
    } else {
        select_226([a, b, d, c, e, g, h, i, j, f, l, m, k, n, o, p])
    }
}
/// n = 7, i = 2, c < d, c < e, e < f, e < g
fn select_228([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_126([b, d, e, f, g])
    } else {
        select_163([a, b, c, d, e, g])
    }
}
/// n = 7, i = 3, a < g, b < e, b < g, c < d, c < f, c < g, e < f
fn select_229([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_8([d, e, f])
    } else {
        select_90([a, b, c, d, e, g])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, c < i, e < i, f < g, f < h
fn select_230([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_39([d, e, f, a, g, h, i])
    } else {
        select_229([a, b, c, d, e, i, f])
    }
}
/// n = 9, i = 3, b < d, b < h, b < i, c < e, c < f, e < i, f < g, f < h
fn select_231([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_228([a, e, f, g, b, d, h])
    } else {
        select_230([a, c, b, d, e, f, g, h, i])
    }
}
/// n = 8, i = 2, c < d, d < e, d < f, d < g, d < h
fn select_232([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_111([a, c, b, d, e, f, g, h])
    } else {
        select_111([b, c, a, d, e, f, g, h])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < e, c < f
fn select_233([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_18([d, b, e, f])
    } else {
        select_147([a, b, c, d, f])
    }
}
/// n = 8, i = 3, b < d, b < g, b < h, c < e, e < f, e < g, e < h
fn select_234([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < d {
        select_233([a, b, e, f, g, h])
    } else {
        select_166([a, b, c, d, e, f, h])
    }
}
/// n = 11, i = 3, b < d, b < g, b < k, c < e, c < g, e < f, e < k, g < h, g < i, g < j
fn select_235([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_232([a, d, c, g, e, h, i, j])
    } else {
        select_234([a, b, c, d, e, f, g, k])
    }
}
/// n = 11, i = 3, b < e, b < h, b < k, c < d, c < f, c < h, f < g, f < k, h < i, h < j
fn select_236([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < h {
        select_231([a, b, c, e, d, f, g, k, h])
    } else {
        select_235([a, b, c, e, f, g, h, d, i, j, k])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_237([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_42([a, d, b, f, g, h, i, e])
    } else {
        select_42([a, e, c, f, g, h, i, d])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < j, f < k, g < h, g < i
fn select_238([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_237([a, b, c, d, e, f, g, j, k])
    } else {
        select_237([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 12, i = 4, b < e, b < h, c < d, c < h, c < i, d < g, d < l, e < f, e < i, e < l, i < j, i < k
fn select_239([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_237([a, b, d, g, h, e, f, i, l])
    } else {
        select_238([a, c, e, f, h, d, i, j, k, g, l])
    }
}
/// n = 13, i = 4, b < d, b < h, b < k, c < e, c < h, d < g, e < f, e < i, e < k, i < j, k < l, k < m
fn select_240([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < d {
        select_236([a, b, e, f, h, i, j, k, l, m, d])
    } else {
        select_239([a, c, b, d, e, f, g, h, k, l, m, i])
    }
}
/// n = 10, i = 4, b < g, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_241([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_112([a, e, b, c, f, g, h, i, j])
    } else {
        select_112([a, f, b, d, e, g, h, i, j])
    }
}
/// n = 10, i = 4, b < f, b < i, b < j, c < g, d < e, d < g, g < h, g < i, g < j
fn select_242([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_43([a, e, g, h, b, f, i, j])
    } else {
        select_241([a, c, b, d, e, f, g, h, i, j])
    }
}
/// n = 6, i = 3, b < f, c < e, c < f, d < e, d < f
fn select_243([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_147([a, b, d, e, f])
    } else {
        select_46([b, c, d, e])
    }
}
/// n = 9, i = 3, c < e, c < f, d < e, d < f, f < g, f < h, f < i
fn select_244([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_146([b, e, f, a, g, h, i])
    } else {
        select_243([b, a, c, d, e, f])
    }
}
/// n = 11, i = 5, b < h, c < f, c < i, d < e, d < h, f < g, f < j, f < k, h < i, h < j, h < k
fn select_245([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_244([a, e, c, h, i, f, g, j, k])
    } else {
        select_241([a, b, d, f, g, e, h, i, j, k])
    }
}
/// n = 13, i = 5, b < e, b < h, b < l, c < h, c < i, d < f, d < k, f < g, f < i, f < m, h < k, h < m, i < j, i < l
fn select_246([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < h {
        select_242([a, b, c, f, g, e, i, j, h, l])
    } else {
        select_245([a, c, d, b, e, f, g, h, k, i, m])
    }
}
/// n = 15, i = 5, b < e, b < o, c < f, c < j, d < g, d < i, f < i, f < l, g < h, g < j, g < l, j < k, j < o, l < m, l < n
fn select_247([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < b {
        select_240([a, f, d, b, g, h, e, i, j, k, l, m, n])
    } else {
        select_246([a, b, c, d, e, g, h, f, j, k, i, o, l])
    }
}
/// n = 16, i = 5, b < e, b < p, c < f, c < j, c < k, d < g, d < i, f < i, g < h, g < j, g < k, j < m, j < p, k < l, k < n, n < o
fn select_248([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < f {
        select_225([a, b, c, g, h, e, j, k, l, m, f, n, o, i, p])
    } else {
        select_247([a, b, c, d, e, f, g, h, i, j, m, k, l, n, p])
    }
}
/// n = 16, i = 5, b < e, b < p, c < h, c < k, d < f, d < g, d < l, f < k, g < i, g < n, h < j, h < l, h < n, l < m, l < p, n < o
fn select_249([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < h {
        select_143([a, f, b, c, g, i, e, k, h, j, l, n, o, m])
    } else {
        select_248([a, b, d, c, e, f, h, j, k, l, g, i, m, n, o, p])
    }
}
/// n = 16, i = 5, b < g, b < p, c < f, c < i, d < e, d < h, d < l, h < k, h < n, i < j, i < l, i < n, l < m, l < p, n < o
fn select_250([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < e {
        select_227([a, b, d, c, g, f, h, i, j, k, e, l, m, n, o, p])
    } else {
        select_249([a, b, c, d, g, e, h, i, k, j, f, l, m, n, o, p])
    }
}
/// n = 16, i = 5, b < i, b < p, c < h, c < l, d < g, d < m, e < f, e < j, j < k, j < l, j < m, l < o, m < n, m < p
fn select_251([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if c < d {
        select_188([a, b, e, c, h, f, i, d, j, k, g, l, o, m, n])
    } else {
        select_250([a, b, e, d, g, f, i, c, j, k, h, m, n, l, o, p])
    }
}
/// n = 17, i = 5, b < i, b < o, c < h, c < n, d < g, d < j, e < f, e < k, j < m, j < o, k < l, k < n, n < p, o < q
fn select_252([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < k {
        select_251([a, c, b, e, d, g, f, i, h, j, m, o, k, l, q, n])
    } else {
        select_251([a, b, c, d, e, f, g, h, i, k, l, n, j, m, p, o])
    }
}
/// n = 17, i = 5, b < i, b < p, c < h, c < k, d < g, d < j, e < f, e < l, j < n, k < o, l < m, l < p, p < q
fn select_253([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < k {
        select_252([a, b, c, e, d, g, f, h, i, l, j, n, m, k, p, o, q])
    } else {
        select_252([a, b, d, e, c, h, f, g, i, l, k, o, m, j, p, n, q])
    }
}
/// n = 17, i = 5, b < k, c < j, d < i, d < p, e < h, e < l, f < g, f < m, l < o, m < n, m < p, p < q
fn select_254([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if b < c {
        select_253([a, d, b, e, f, g, h, k, i, l, c, m, n, o, j, p, q])
    } else {
        select_253([a, d, c, e, f, g, h, j, i, l, b, m, n, o, k, p, q])
    }
}
/// n = 17, i = 5, d < k, e < j, e < p, f < i, f < l, g < h, g < m, l < o, m < n, m < p, p < q
fn select_255([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_254([c, a, d, e, f, g, h, i, j, k, b, l, m, n, o, p, q])
    } else {
        select_254([c, b, d, e, f, g, h, i, j, k, a, l, m, n, o, p, q])
    }
}
/// n = 17, i = 5, f < k, f < p, g < j, g < l, h < i, h < m, l < o, m < n, m < p, p < q
fn select_256([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_255([c, d, e, a, f, g, h, i, j, k, b, l, m, n, o, p, q])
    } else {
        select_255([c, d, e, b, f, g, h, i, j, k, a, l, m, n, o, p, q])
    }
}
/// n = 17, i = 5, f < m, g < l, h < k, h < p, i < j, i < n, n < o, n < p, p < q
fn select_257([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if f < g {
        select_256([a, b, c, d, e, h, f, i, j, m, k, g, n, o, l, p, q])
    } else {
        select_256([a, b, c, d, e, h, g, i, j, l, k, f, n, o, m, p, q])
    }
}
/// n = 17, i = 5, f < m, g < l, h < k, h < o, i < j, i < n, n < p, o < q
fn select_258([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if n < o {
        select_257([a, b, c, d, e, f, g, h, i, j, k, l, m, n, p, o, q])
    } else {
        select_257([a, b, c, d, e, f, g, i, h, k, j, l, m, o, q, n, p])
    }
}
/// n = 17, i = 5, h < m, i < l, i < o, j < k, j < n, n < p, o < q
fn select_259([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_258([c, d, e, f, g, a, h, i, j, k, l, m, b, n, o, p, q])
    } else {
        select_258([c, d, e, f, g, b, h, i, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 5, j < m, j < o, k < l, k < n, n < p, o < q
fn select_260([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_259([c, d, e, f, g, h, i, a, j, k, l, m, b, n, o, p, q])
    } else {
        select_259([c, d, e, f, g, h, i, b, j, k, l, m, a, n, o, p, q])
    }
}
/// n = 17, i = 5, j < o, k < n, l < m, l < p, p < q
fn select_261([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if j < k {
        select_260([a, b, c, d, e, f, g, h, i, j, l, m, o, p, k, q, n])
    } else {
        select_260([a, b, c, d, e, f, g, h, i, k, l, m, n, p, j, q, o])
    }
}
/// n = 17, i = 5, l < o, m < n, m < p, p < q
fn select_262([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_261([c, d, e, f, g, h, i, j, k, a, l, m, n, o, b, p, q])
    } else {
        select_261([c, d, e, f, g, h, i, j, k, b, l, m, n, o, a, p, q])
    }
}
/// n = 17, i = 5, n < o, n < p, p < q
fn select_263([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_262([c, d, e, f, g, h, i, j, k, l, m, a, n, o, b, p, q])
    } else {
        select_262([c, d, e, f, g, h, i, j, k, l, m, b, n, o, a, p, q])
    }
}
/// n = 17, i = 5, n < q, o < p
fn select_264([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if n < o {
        select_263([a, b, c, d, e, f, g, h, i, j, k, l, m, n, q, o, p])
    } else {
        select_263([a, b, c, d, e, f, g, h, i, j, k, l, m, o, p, n, q])
    }
}
/// n = 17, i = 5, p < q
fn select_265([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_264([c, d, e, f, g, h, i, j, k, l, m, n, o, a, p, q, b])
    } else {
        select_264([c, d, e, f, g, h, i, j, k, l, m, n, o, b, p, q, a])
    }
}
/// n = 17, i = 5
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q]: [usize; 17]) -> usize {
    if a < b {
        select_265([c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, a, b])
    } else {
        select_265([c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, b, a])
    }
}
