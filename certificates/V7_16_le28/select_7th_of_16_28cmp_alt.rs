#[allow(unused)]
pub const N: usize = 16;
#[allow(unused)]
pub const I: usize = 6;

#[allow(unused)]
pub fn run(input: [usize; 16]) -> usize {
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
/// n = 4, i = 1, b < c, b < d
fn select_4([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_3([a, c, d])
    }
}
/// n = 5, i = 1, a < e, b < c, b < d
fn select_5([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_2([b, e])
    } else {
        select_3([a, c, d])
    }
}
/// n = 3, i = 1, b < c
fn select_6([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_2([a, c])
    }
}
/// n = 5, i = 2, a < e, b < d, c < d, c < e
fn select_7([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_6([b, c, e])
    } else {
        select_6([a, c, d])
    }
}
/// n = 6, i = 2, b < f, c < d, c < f, d < e
fn select_8([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_5([b, d, a, e, f])
    } else {
        select_7([a, b, c, f, d])
    }
}
/// n = 7, i = 2, b < e, c < d, d < f, e < g
fn select_9([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < b {
        select_4([a, d, b, f])
    } else {
        select_8([a, c, b, e, g, d])
    }
}
/// n = 5, i = 1, c < d, c < e
fn select_10([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_5([a, c, d, e, b])
    } else {
        select_5([b, c, d, e, a])
    }
}
/// n = 6, i = 2, a < f, b < d, b < e, c < d, c < e, c < f
fn select_11([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_6([b, c, f])
    } else {
        select_4([a, c, d, e])
    }
}
/// n = 6, i = 2, a < f, b < e, b < f, c < d, c < e
fn select_12([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < b {
        select_6([a, d, b])
    } else {
        select_11([a, c, b, d, e, f])
    }
}
/// n = 7, i = 2, a < c, a < g, b < d, b < g, d < e, d < f
fn select_13([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_3([a, e, f])
    } else {
        select_4([c, b, d, g])
    }
}
/// n = 7, i = 2, b < d, b < g, c < e, c < g, e < f
fn select_14([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_12([a, c, b, d, g, e])
    } else {
        select_13([b, c, d, e, a, f, g])
    }
}
/// n = 8, i = 2, b < f, c < d, c < e, e < g, f < h
fn select_15([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_10([a, d, e, b, g])
    } else {
        select_14([a, c, b, d, f, h, e])
    }
}
/// n = 8, i = 2, c < f, d < e, e < g, f < h
fn select_16([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_9([b, a, d, e, c, g, f])
    } else {
        select_15([b, d, c, a, f, e, h, g])
    }
}
/// n = 2, i = 1
fn select_17([a, b]: [usize; 2]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_1([a])
    }
}
/// n = 3, i = 2
fn select_18([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_17([b, c])
    } else {
        select_17([a, c])
    }
}
/// n = 4, i = 2, b < d, c < d
fn select_19([a, b, c, d]: [usize; 4]) -> usize {
    if a < d {
        select_18([a, b, c])
    } else {
        select_1([d])
    }
}
/// n = 6, i = 3, a < f, b < d, c < e, e < f
fn select_20([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < d {
        select_7([a, b, e, d, f])
    } else {
        select_19([a, c, d, e])
    }
}
/// n = 4, i = 2, a < d, b < c
fn select_21([a, b, c, d]: [usize; 4]) -> usize {
    if c < d {
        select_17([a, c])
    } else {
        select_17([b, d])
    }
}
/// n = 4, i = 2, c < d
fn select_22([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_21([a, c, d, b])
    } else {
        select_21([b, c, d, a])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, c < d, c < f, e < g
fn select_23([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_20([a, c, b, d, e, g])
    } else {
        select_22([b, d, a, f])
    }
}
/// n = 5, i = 2, a < e, b < e, c < d
fn select_24([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < d {
        select_7([b, c, a, d, e])
    } else {
        select_6([b, d, a])
    }
}
/// n = 5, i = 2, a < c, b < d, d < e
fn select_25([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < a {
        select_2([a, e])
    } else {
        select_6([c, b, d])
    }
}
/// n = 8, i = 3, a < f, a < h, b < e, b < f, c < d, c < e, d < h, f < g
fn select_26([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_24([a, d, b, e, h])
    } else {
        select_25([c, a, e, f, g])
    }
}
/// n = 4, i = 1, a < d, b < c
fn select_27([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_2([b, d])
    } else {
        select_2([a, c])
    }
}
/// n = 5, i = 2, a < d, a < e, b < c, b < e
fn select_28([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_6([c, a, e])
    } else {
        select_6([d, b, e])
    }
}
/// n = 6, i = 2, a < d, b < c, b < e, e < f
fn select_29([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_4([c, e, a, f])
    } else {
        select_28([a, b, c, d, e])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, c < d, c < f, d < h, f < g
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < b {
        select_27([a, b, f, h])
    } else {
        select_29([a, c, d, e, f, g])
    }
}
/// n = 8, i = 3, a < e, a < f, b < f, b < g, c < d, c < g, g < h
fn select_31([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_26([c, b, a, e, f, g, h, d])
    } else {
        select_30([a, b, c, d, f, g, h, e])
    }
}
/// n = 8, i = 3, b < g, c < e, c < f, d < f, d < g, g < h
fn select_32([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_23([d, a, c, e, b, f, g])
    } else {
        select_31([c, d, b, a, e, f, g, h])
    }
}
/// n = 5, i = 1, b < c, b < d, b < e
fn select_33([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < a {
        select_3([c, d, e])
    } else {
        select_4([a, b, d, e])
    }
}
/// n = 6, i = 1, c < d, c < e, c < f
fn select_34([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_33([b, c, d, e, f])
    } else {
        select_10([a, b, c, e, f])
    }
}
/// n = 6, i = 1, a < e, a < f, b < c, b < d
fn select_35([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_3([b, e, f])
    } else {
        select_3([a, c, d])
    }
}
/// n = 7, i = 1, a < g, b < f, c < d, c < e
fn select_36([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_35([a, c, d, e, b, g])
    } else {
        select_35([b, c, d, e, a, f])
    }
}
/// n = 7, i = 1, c < g, d < e, d < f
fn select_37([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_36([a, c, d, e, f, g, b])
    } else {
        select_36([b, c, d, e, f, g, a])
    }
}
/// n = 8, i = 1, c < h, d < e, d < f, d < g
fn select_38([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < c {
        select_34([a, b, d, e, f, g])
    } else {
        select_37([a, b, c, d, f, g, h])
    }
}
/// n = 4, i = 0
fn select_39([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_3([a, c, d])
    } else {
        select_3([b, c, d])
    }
}
/// n = 6, i = 1, a < f, b < c, b < d, b < e
fn select_40([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_2([b, f])
    } else {
        select_39([a, c, d, e])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, d < e, d < f, d < g
fn select_41([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_39([a, e, f, g])
    } else {
        select_4([c, b, d, h])
    }
}
/// n = 8, i = 2, a < d, b < c, b < e, e < f, e < g, e < h
fn select_42([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < a {
        select_40([c, e, f, g, h, a])
    } else {
        select_41([a, b, d, e, f, g, h, c])
    }
}
/// n = 8, i = 2, c < d, c < e, e < f, e < g, e < h
fn select_43([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_42([a, c, d, b, e, f, g, h])
    } else {
        select_42([b, c, d, a, e, f, g, h])
    }
}
/// n = 10, i = 3, b < d, b < g, b < j, c < e, c < g, e < f, e < h, e < j, h < i
fn select_44([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_38([a, f, h, b, d, g, j, i])
    } else {
        select_43([a, d, c, g, e, f, h, j])
    }
}
/// n = 10, i = 3, c < e, c < g, c < j, d < f, d < g, f < h, f < j, h < i
fn select_45([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_32([b, a, c, d, e, g, f, h])
    } else {
        select_44([b, c, d, e, f, a, g, h, i, j])
    }
}
/// n = 10, i = 3, c < f, c < j, d < e, d < g, g < h, g < j, h < i
fn select_46([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < c {
        select_16([a, b, e, g, h, c, i, f])
    } else {
        select_45([a, b, c, d, f, g, e, h, i, j])
    }
}
/// n = 6, i = 3, a < f, b < e, c < e, c < f, d < e, d < f
fn select_47([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_19([b, c, d, f])
    } else {
        select_19([a, c, d, e])
    }
}
/// n = 5, i = 2, b < e, c < d, c < e
fn select_48([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_21([a, c, d, b])
    } else {
        select_28([b, c, d, a, e])
    }
}
/// n = 8, i = 4, a < h, b < f, c < e, d < f, d < h, e < h, f < g
fn select_49([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_47([a, b, d, e, f, h])
    } else {
        select_48([a, c, f, g, e])
    }
}
/// n = 7, i = 3, a < g, b < d, c < e, d < g, e < f
fn select_50([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_7([a, c, d, e, g])
    } else {
        select_48([a, b, e, f, d])
    }
}
/// n = 8, i = 4, a < f, b < e, c < g, d < f, d < g, g < h
fn select_51([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_49([a, c, b, d, e, g, h, f])
    } else {
        select_50([b, a, c, f, g, h, e])
    }
}
/// n = 9, i = 3, a < d, b < e, b < i, c < e, c < h, d < h, d < i, e < f, e < g
fn select_52([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_4([a, e, f, g])
    } else {
        select_7([b, c, d, h, i])
    }
}
/// n = 7, i = 3, a < d, b < e, c < e, e < f, e < g
fn select_53([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_3([a, f, g])
    } else {
        select_19([d, b, c, e])
    }
}
/// n = 10, i = 4, a < e, a < f, b < g, b < j, c < e, c < g, d < f, d < g, f < j, g < h, g < i
fn select_54([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_20([b, c, d, e, f, j])
    } else {
        select_53([a, b, d, e, g, h, i])
    }
}
/// n = 10, i = 4, a < f, a < j, b < g, b < j, c < f, c < g, d < e, d < g, e < j, g < h, g < i
fn select_55([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_52([e, b, c, a, g, h, i, f, j])
    } else {
        select_54([a, b, c, d, f, e, g, h, i, j])
    }
}
/// n = 4, i = 1, c < d
fn select_56([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_27([a, c, d, b])
    } else {
        select_27([b, c, d, a])
    }
}
/// n = 7, i = 2, a < c, b < d, c < g, d < e, d < f
fn select_57([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_3([a, e, f])
    } else {
        select_27([b, c, g, d])
    }
}
/// n = 9, i = 3, a < f, b < d, c < f, c < i, d < e, d < i, f < g, f < h
fn select_58([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_56([a, e, c, i])
    } else {
        select_57([b, c, d, f, g, h, e])
    }
}
/// n = 11, i = 4, a < h, a < k, b < e, b < h, c < e, c < f, d < f, d < h, f < g, f < k, h < i, h < j
fn select_59([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_58([b, c, a, f, g, h, i, j, k])
    } else {
        select_54([c, a, b, d, e, f, h, i, j, k])
    }
}
/// n = 11, i = 4, a < f, a < g, b < f, b < i, c < e, c < i, d < g, d < i, g < h, i < j, i < k
fn select_60([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_55([a, d, b, c, e, f, i, j, k, g])
    } else {
        select_59([c, b, a, d, f, g, h, i, j, k, e])
    }
}
/// n = 12, i = 5, a < f, a < h, b < e, b < j, c < h, c < j, d < g, d < h, g < j, h < i, j < k, j < l
fn select_61([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_51([f, b, c, d, e, g, h, i])
    } else {
        select_60([a, g, b, c, e, f, h, i, j, k, l])
    }
}
/// n = 6, i = 1, b < f, c < d, c < e
fn select_62([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_5([a, c, d, e, b])
    } else {
        select_35([b, c, d, e, a, f])
    }
}
/// n = 6, i = 1, d < e, d < f
fn select_63([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_62([c, a, d, e, f, b])
    } else {
        select_62([c, b, d, e, f, a])
    }
}
/// n = 7, i = 1, d < e, d < f, d < g
fn select_64([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_34([b, c, d, e, f, g])
    } else {
        select_63([a, b, c, d, f, g])
    }
}
/// n = 8, i = 1, d < h, e < f, e < g
fn select_65([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_63([a, b, c, d, e, h])
    } else {
        select_64([a, b, c, e, d, f, g])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < e, c < f
fn select_66([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_5([d, b, e, f, a])
    } else {
        select_11([a, b, c, e, f, d])
    }
}
/// n = 5, i = 3, b < e, c < e, d < e
fn select_67([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < a {
        select_19([a, c, d, e])
    } else {
        select_18([b, c, d])
    }
}
/// n = 8, i = 4, b < e, b < h, c < f, d < f, f < g, f < h
fn select_68([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_66([a, b, f, g, e, h])
    } else {
        select_67([a, c, d, e, f])
    }
}
/// n = 5, i = 0
fn select_69([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_39([a, c, d, e])
    } else {
        select_39([b, c, d, e])
    }
}
/// n = 9, i = 3, a < d, b < e, c < e, e < f, e < g, e < h, e < i
fn select_70([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_69([a, f, g, h, i])
    } else {
        select_19([d, b, c, e])
    }
}
/// n = 9, i = 3, c < e, d < e, e < f, e < g, e < h, e < i
fn select_71([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_70([a, c, d, b, e, f, g, h, i])
    } else {
        select_70([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 11, i = 4, b < e, b < h, c < f, c < h, d < f, d < h, f < g, h < i, h < j, h < k
fn select_72([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_68([a, b, c, d, e, f, g, h])
    } else {
        select_71([a, e, c, d, h, f, i, j, k])
    }
}
/// n = 13, i = 5, a < f, a < g, b < e, b < h, c < g, c < h, d < g, d < h, g < k, h < i, h < j, h < l, l < m
fn select_73([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_65([e, i, j, l, a, f, g, m])
    } else {
        select_72([f, b, c, d, e, g, k, h, i, j, l])
    }
}
/// n = 13, i = 5, a < g, a < h, b < f, b < i, c < h, c < i, d < e, d < h, d < i, h < k, i < j, i < l, l < m
fn select_74([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < i {
        select_61([a, b, c, d, f, g, e, h, k, i, j, l])
    } else {
        select_73([a, b, c, d, f, g, h, i, e, j, k, l, m])
    }
}
/// n = 13, i = 5, a < g, a < j, b < j, b < l, c < f, c < h, d < e, d < h, d < j, h < i, h < l, j < k, l < m
fn select_75([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_46([e, f, a, h, i, g, b, l, m, j])
    } else {
        select_74([a, c, b, d, e, f, g, j, h, i, k, l, m])
    }
}
/// n = 5, i = 2, a < e, b < c, b < d
fn select_76([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < a {
        select_6([d, c, a])
    } else {
        select_28([a, b, d, e, c])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, c < d, c < e, d < g
fn select_77([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < b {
        select_27([a, b, e, g])
    } else {
        select_76([a, c, d, e, f])
    }
}
/// n = 8, i = 3, a < f, b < g, c < e, c < f, d < e, d < g, g < h
fn select_78([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_77([d, c, a, b, f, e, g])
    } else {
        select_30([c, d, b, a, e, g, h, f])
    }
}
/// n = 5, i = 3, a < e, b < e, c < d
fn select_79([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_18([a, b, d])
    } else {
        select_17([c, e])
    }
}
/// n = 6, i = 3, b < e, c < d, c < f, e < f
fn select_80([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_28([c, e, a, d, f])
    } else {
        select_79([a, b, c, d, e])
    }
}
/// n = 9, i = 4, a < g, b < d, c < e, d < f, d < h, e < g, e < h, g < i
fn select_81([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_6([b, g, i])
    } else {
        select_80([a, c, d, f, e, h])
    }
}
/// n = 8, i = 3, a < f, b < f, b < g, c < d, d < e, d < g, f < h
fn select_82([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_6([c, f, h])
    } else {
        select_48([a, b, d, e, g])
    }
}
/// n = 10, i = 5, a < i, b < g, b < i, c < e, d < f, e < g, e < h, f < h, f < i, i < j
fn select_83([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < f {
        select_81([a, c, d, e, f, g, i, h, j])
    } else {
        select_82([a, b, c, e, h, i, g, j])
    }
}
/// n = 11, i = 5, a < e, b < i, b < j, c < h, c < j, d < f, e < h, f < g, f < j, g < i, j < k
fn select_84([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_78([a, c, g, b, i, e, j, k])
    } else {
        select_83([b, c, a, d, e, f, h, g, j, k])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < g, c < d, d < g, d < h, e < f
fn select_85([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_6([c, e, f])
    } else {
        select_7([a, b, d, g, h])
    }
}
/// n = 10, i = 5, a < h, a < j, b < g, b < h, c < e, d < f, e < g, e < j, f < h, f < j, h < i
fn select_86([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_85([a, b, c, e, h, i, g, j])
    } else {
        select_20([a, b, d, g, f, j])
    }
}
/// n = 10, i = 5, a < h, a < i, b < g, b < i, c < e, d < f, e < i, f < g, f < h, i < j
fn select_87([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_86([a, b, d, c, f, e, g, i, j, h])
    } else {
        select_20([c, b, a, g, h, e])
    }
}
/// n = 6, i = 2, a < e, a < f, b < d, b < f, c < d, c < e
fn select_88([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_27([b, c, e, f])
    } else {
        select_27([a, c, d, f])
    }
}
/// n = 6, i = 2, a < c, a < f, b < d, b < f, d < e
fn select_89([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_6([c, b, f])
    } else {
        select_27([a, d, e, f])
    }
}
/// n = 8, i = 3, a < e, a < g, a < h, b < e, b < f, b < h, c < d, c < h, d < f, d < g
fn select_90([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_27([a, b, f, g])
    } else {
        select_89([b, c, e, d, g, h])
    }
}
/// n = 10, i = 4, a < f, a < g, a < j, b < d, b < j, c < e, c < f, d < g, d < i, e < h, e < i, e < j, f < h
fn select_91([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_88([b, e, f, h, d, j])
    } else {
        select_90([a, d, c, e, g, i, j, f])
    }
}
/// n = 7, i = 2, a < d, a < g, b < c, b < g, c < e, d < f
fn select_92([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_27([a, c, e, g])
    } else {
        select_27([b, d, f, g])
    }
}
/// n = 11, i = 4, a < f, a < h, a < k, b < e, b < k, c < d, c < g, d < h, d < k, e < f, e < g, g < i, h < j
fn select_93([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < d {
        select_29([a, c, d, f, g, i])
    } else {
        select_92([a, b, e, h, g, j, k])
    }
}
/// n = 11, i = 4, a < f, a < k, b < h, b < j, b < k, c < f, c < g, c < k, d < e, d < k, e < g, e < h, f < j, h < i
fn select_94([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_91([c, d, a, e, b, f, g, j, h, k])
    } else {
        select_93([c, d, b, a, e, g, h, f, i, j, k])
    }
}
/// n = 12, i = 5, a < e, a < l, b < h, b < j, b < l, c < g, c < l, d < f, d < j, e < h, f < i, f < l, g < i, g < j, j < k
fn select_95([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_87([b, d, c, a, g, e, f, h, j, k])
    } else {
        select_94([a, b, f, c, g, e, i, j, k, h, l])
    }
}
/// n = 12, i = 5, a < f, a < l, b < i, b < j, b < l, c < e, c < j, d < g, d < l, e < l, f < i, g < h, g < j, j < k
fn select_96([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_84([a, c, b, d, f, g, h, i, e, j, k])
    } else {
        select_95([a, b, d, c, f, e, g, i, h, j, k, l])
    }
}
/// n = 7, i = 3, a < d, a < g, b < e, b < g, c < e, c < g, e < f
fn select_97([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_27([a, e, f, g])
    } else {
        select_19([d, b, c, g])
    }
}
/// n = 6, i = 3, a < e, a < f, b < e, b < f, c < d, c < f
fn select_98([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_19([d, a, b, f])
    } else {
        select_6([e, c, f])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < e
fn select_99([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_98([a, b, c, d, f, e])
    } else {
        select_6([d, e, a])
    }
}
/// n = 8, i = 4, a < g, a < h, b < d, b < g, c < e, c < h, d < h, e < f
fn select_100([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_97([a, c, d, g, e, f, h])
    } else {
        select_99([a, b, e, f, d, g])
    }
}
/// n = 6, i = 2, a < d, a < e, a < f, b < c, b < e, b < f
fn select_101([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_4([c, a, e, f])
    } else {
        select_4([d, b, e, f])
    }
}
/// n = 8, i = 3, a < d, a < g, b < c, b < g, b < h, c < f, d < e, d < h
fn select_102([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_28([a, c, f, g, d])
    } else {
        select_101([b, d, e, g, c, h])
    }
}
/// n = 5, i = 2, b < d, b < e, c < d, c < e
fn select_103([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_17([b, c])
    } else {
        select_4([a, c, d, e])
    }
}
/// n = 8, i = 3, a < e, a < h, b < c, b < d, b < g, d < h, e < f, e < g
fn select_104([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_103([c, a, d, e, h])
    } else {
        select_101([b, e, f, c, d, g])
    }
}
/// n = 9, i = 3, a < g, a < i, b < d, b < h, c < e, c < g, e < f, e < h, e < i
fn select_105([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_102([c, a, b, e, f, d, g, i])
    } else {
        select_104([c, b, d, a, e, f, h, g])
    }
}
/// n = 10, i = 4, a < d, a < g, b < g, b < h, c < e, c < h, e < f, e < j, h < i, h < j
fn select_106([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < h {
        select_100([b, a, c, d, e, f, g, h])
    } else {
        select_105([a, e, b, f, h, i, g, j, d])
    }
}
/// n = 8, i = 3, a < g, a < h, b < g, b < h, c < e, c < h, d < e, d < g, e < f
fn select_107([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_85([c, d, a, b, e, f, g, h])
    } else {
        select_85([c, d, b, a, e, f, g, h])
    }
}
/// n = 6, i = 3, a < e, b < e, b < f, c < d, d < f
fn select_108([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_21([b, c, d, e])
    } else {
        select_6([a, b, f])
    }
}
/// n = 9, i = 4, a < f, a < g, b < d, c < e, d < f, d < i, e < g, e < i, g < h
fn select_109([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_6([b, g, h])
    } else {
        select_108([a, d, c, e, f, i])
    }
}
/// n = 11, i = 5, a < g, a < k, b < h, b < i, b < k, c < g, c < i, d < e, e < f, e < i, f < k, g < h, i < j
fn select_110([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_107([a, f, b, c, i, j, g, k])
    } else {
        select_109([b, a, d, g, e, h, i, j, f])
    }
}
/// n = 6, i = 2, a < c, b < d, d < e, d < f
fn select_111([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_3([a, e, f])
    } else {
        select_6([c, b, d])
    }
}
/// n = 8, i = 3, a < d, b < c, b < g, b < h, c < f, d < e, d < g, d < h
fn select_112([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < c {
        select_4([e, b, g, h])
    } else {
        select_111([c, a, f, d, g, h])
    }
}
/// n = 10, i = 3, a < c, a < j, b < d, c < f, d < e, d < g, d < j, g < h, g < i
fn select_113([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_33([e, g, a, h, i])
    } else {
        select_112([b, a, c, d, e, f, g, j])
    }
}
/// n = 8, i = 2, a < c, b < d, c < h, d < e, d < f, d < g
fn select_114([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_39([a, e, f, g])
    } else {
        select_27([b, c, h, d])
    }
}
/// n = 8, i = 4, a < d, b < f, b < g, c < e, d < f, e < g, g < h
fn select_115([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_25([a, b, d, g, h])
    } else {
        select_21([b, c, e, f])
    }
}
/// n = 13, i = 5, a < f, a < h, b < f, b < i, c < g, c < h, c < i, d < e, e < h, e < i, f < g, h < m, i < j, i < k, i < l
fn select_116([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_114([a, b, h, i, j, k, l, m])
    } else {
        select_115([b, c, d, f, e, g, h, m])
    }
}
/// n = 14, i = 5, a < f, a < j, a < n, b < f, b < g, c < g, c < i, c < j, d < e, d < g, e < j, e < k, e < l, f < i, g < h, g < k, g < l, g < n, j < m
fn select_117([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < e {
        select_113([a, d, f, g, h, i, e, k, l, n])
    } else {
        select_116([a, b, c, d, e, f, i, j, g, h, k, l, m])
    }
}
/// n = 14, i = 5, a < g, a < k, a < n, b < g, b < h, c < h, c < j, c < k, d < e, d < h, e < f, e < k, e < l, g < j, h < i, h < l, h < n, k < m
fn select_118([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < h {
        select_110([b, c, a, d, e, f, g, j, k, m, h])
    } else {
        select_117([a, b, c, d, e, g, h, i, j, k, f, l, m, n])
    }
}
/// n = 14, i = 5, a < k, a < n, b < e, b < i, c < h, c < i, c < k, d < f, d < i, e < h, f < g, f < k, f < l, i < j, i < l, i < n, k < m
fn select_119([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < a {
        select_106([e, c, d, a, f, g, h, i, j, l])
    } else {
        select_118([a, b, c, d, f, g, e, i, j, h, k, l, m, n])
    }
}
/// n = 14, i = 5, a < e, a < l, b < f, b < j, c < i, c < j, c < l, d < g, d < j, f < i, g < h, g < l, g < m, j < k, j < m, l < n
fn select_120([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < j {
        select_96([b, c, a, d, e, f, g, h, i, l, n, j])
    } else {
        select_119([a, b, c, d, f, g, h, i, j, k, l, m, n, e])
    }
}
/// n = 8, i = 3, a < e, a < h, b < d, b < h, c < e, c < g, c < h, d < g, e < f
fn select_121([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_27([b, e, f, h])
    } else {
        select_7([a, d, c, g, h])
    }
}
/// n = 10, i = 3, a < e, a < j, b < e, b < f, c < d, c < f, d < j, e < i, f < g, f < h
fn select_122([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_33([d, f, a, g, h])
    } else {
        select_121([b, c, a, d, e, i, j, f])
    }
}
/// n = 10, i = 3, a < e, b < g, c < e, c < f, d < f, d < g, f < h, g < i, g < j
fn select_123([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_26([d, c, a, b, e, f, h, g])
    } else {
        select_122([c, d, b, a, f, g, i, j, h, e])
    }
}
/// n = 8, i = 3, a < d, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_124([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_25([a, c, d, e, g])
    } else {
        select_25([b, c, d, f, h])
    }
}
/// n = 8, i = 3, a < f, a < g, b < d, b < e, b < f, c < d, c < e, c < g, g < h
fn select_125([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_124([a, b, c, f, d, g, e, h])
    } else {
        select_124([a, b, c, f, e, g, d, h])
    }
}
/// n = 10, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, d < i, e < j, f < g, f < h
fn select_126([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_3([a, g, h])
    } else {
        select_92([b, c, d, e, i, j, f])
    }
}
/// n = 12, i = 4, a < d, a < h, b < g, b < h, b < i, c < e, c < f, c < h, d < f, d < g, e < i, h < l, i < j, i < k
fn select_127([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_125([b, d, c, e, f, g, h, l])
    } else {
        select_126([a, b, e, d, h, i, j, k, f, l])
    }
}
/// n = 12, i = 4, a < e, a < h, b < g, b < h, b < i, c < d, c < f, c < h, e < g, f < i, h < l, i < j, i < k
fn select_128([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_123([d, f, a, b, e, h, i, l, j, k])
    } else {
        select_127([a, b, c, e, f, d, g, h, i, j, k, l])
    }
}
/// n = 7, i = 1, a < f, a < g, b < c, b < d, b < e
fn select_129([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < a {
        select_3([c, d, e])
    } else {
        select_35([a, b, d, e, f, g])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < g, c < h, d < e, d < f
fn select_130([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_129([b, d, a, e, f, g, h])
    } else {
        select_11([a, b, c, g, h, d])
    }
}
/// n = 10, i = 3, b < e, b < f, b < j, c < d, c < f, c < i, d < j, e < i, f < g, f < h
fn select_131([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_130([a, d, b, f, g, h, e, j])
    } else {
        select_130([a, e, c, f, g, h, d, i])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < f, c < g, d < e
fn select_132([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_35([b, d, a, e, f, g])
    } else {
        select_11([a, b, c, f, g, d])
    }
}
/// n = 7, i = 3, a < e, a < g, b < e, b < f, b < g, c < d, c < g, d < f
fn select_133([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_6([a, b, f])
    } else {
        select_28([b, c, d, e, g])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < h, c < d, c < f, c < h, f < g
fn select_134([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_13([a, c, e, f, d, g, h])
    } else {
        select_133([a, b, c, d, e, f, h])
    }
}
/// n = 10, i = 4, a < f, a < g, a < h, b < e, b < h, b < j, c < d, c < f, c < h, e < g, f < j, h < i
fn select_135([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_132([d, f, b, h, i, e, j])
    } else {
        select_134([e, a, c, d, g, h, i, f])
    }
}
/// n = 13, i = 4, a < g, a < h, b < e, b < h, b < i, c < d, c < f, c < h, e < g, e < l, f < i, f < l, h < m, i < j, i < k
fn select_136([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_131([d, b, f, a, e, i, j, k, l, h])
    } else {
        select_135([a, b, c, d, e, f, g, h, m, i])
    }
}
/// n = 13, i = 4, a < e, a < h, b < f, b < h, b < i, c < d, c < g, c < h, f < l, g < i, g < l, h < m, i < j, i < k
fn select_137([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_128([a, b, c, d, e, g, f, h, i, j, k, m])
    } else {
        select_136([a, b, c, d, f, g, e, h, i, j, k, l, m])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < e, b < h, d < g, e < f
fn select_138([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_13([a, b, c, d, e, g, h])
    } else {
        select_13([a, b, c, e, d, f, h])
    }
}
/// n = 9, i = 3, a < d, a < i, b < f, b < h, b < i, c < e, c < f, c < i, e < h, f < g
fn select_139([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_66([d, b, c, e, f, i])
    } else {
        select_138([a, c, d, e, f, g, h, i])
    }
}
/// n = 7, i = 2, a < c, a < f, a < g, b < d, b < f, b < g, d < e
fn select_140([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_4([c, b, f, g])
    } else {
        select_5([d, a, f, g, e])
    }
}
/// n = 9, i = 2, a < e, a < h, a < i, b < c, b < d, b < h, b < i, d < f, e < g
fn select_141([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_62([c, d, a, h, i, f])
    } else {
        select_140([b, a, c, e, g, h, i])
    }
}
/// n = 9, i = 3, a < f, a < i, b < d, b < e, b < i, c < e, c < f, c < i, e < g, f < h
fn select_142([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_8([d, a, c, e, g, i])
    } else {
        select_97([b, a, c, d, f, h, i])
    }
}
/// n = 11, i = 3, a < d, a < k, b < g, b < i, b < j, b < k, c < e, c < g, c < k, e < f, e < i, e < j, g < h
fn select_143([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_141([b, e, f, a, g, d, h, i, j])
    } else {
        select_142([b, a, c, d, e, g, f, h, k])
    }
}
/// n = 11, i = 3, a < e, a < k, b < d, b < h, b < j, b < k, c < f, c < h, c < k, f < g, f < j, h < i
fn select_144([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < f {
        select_139([a, c, b, e, d, h, i, f, k])
    } else {
        select_143([a, b, c, e, f, g, h, i, d, j, k])
    }
}
/// n = 7, i = 2, a < g, b < f, b < g, c < d, c < f, d < e
fn select_145([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < b {
        select_4([a, d, b, e])
    } else {
        select_11([a, c, b, d, f, g])
    }
}
/// n = 6, i = 3, a < d, b < e, c < e, e < f
fn select_146([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < d {
        select_6([a, e, f])
    } else {
        select_18([b, c, d])
    }
}
/// n = 8, i = 4, a < f, b < e, c < f, c < h, d < f, d < h, e < h, f < g
fn select_147([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_146([b, c, d, e, f, g])
    } else {
        select_19([a, c, d, h])
    }
}
/// n = 9, i = 4, a < e, b < g, b < i, c < f, c < g, d < f, d < g, f < i, g < h
fn select_148([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_145([a, f, b, g, h, i, e])
    } else {
        select_147([b, a, c, d, e, g, h, f])
    }
}
/// n = 7, i = 2, a < d, b < c, b < e, e < f, e < g
fn select_149([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_33([c, e, a, f, g])
    } else {
        select_28([a, b, c, d, e])
    }
}
/// n = 4, i = 3
fn select_150([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_18([b, c, d])
    } else {
        select_18([a, c, d])
    }
}
/// n = 7, i = 4, a < e, b < f, c < f, d < f, f < g
fn select_151([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_6([a, f, g])
    } else {
        select_150([b, c, d, e])
    }
}
/// n = 10, i = 4, a < e, b < f, b < g, c < f, c < g, d < f, d < g, f < j, g < h, g < i
fn select_152([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_62([a, f, g, h, i, j])
    } else {
        select_151([a, b, c, d, e, f, j])
    }
}
/// n = 10, i = 4, a < e, b < h, b < j, c < f, c < h, d < f, d < h, f < g, f < j, h < i
fn select_153([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_149([a, f, g, e, b, h, j])
    } else {
        select_152([a, b, c, d, e, h, f, g, j, i])
    }
}
/// n = 10, i = 4, a < e, b < f, b < i, c < g, c < i, d < f, d < g, d < i, g < h, i < j
fn select_154([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_148([a, c, b, d, e, f, i, j, g])
    } else {
        select_153([a, b, c, d, e, g, h, i, j, f])
    }
}
/// n = 13, i = 5, a < e, a < m, b < g, b < j, c < f, c < j, c < m, d < g, d < i, d < j, f < i, g < h, g < l, g < m, j < k, j < l
fn select_155([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_144([a, g, c, h, e, j, k, f, i, l, m])
    } else {
        select_154([a, f, b, d, e, i, j, k, g, h])
    }
}
/// n = 14, i = 5, a < e, a < l, b < i, b < j, c < g, c < j, d < f, d < j, d < l, f < i, g < h, g < l, g < m, j < k, j < m, l < n
fn select_156([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_137([a, d, g, h, e, f, b, l, j, k, m, i, n])
    } else {
        select_155([a, c, d, b, e, f, g, h, i, j, k, m, l])
    }
}
/// n = 14, i = 5, a < g, a < l, b < f, b < j, c < e, c < j, c < l, d < h, d < j, h < i, h < l, h < m, j < k, j < m, l < n
fn select_157([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < e {
        select_120([a, b, c, d, g, f, h, i, e, j, k, l, m, n])
    } else {
        select_156([a, b, d, c, g, e, h, i, f, j, k, l, m, n])
    }
}
/// n = 14, i = 5, a < g, a < j, b < f, b < l, c < h, c < l, d < e, d < j, d < l, h < i, h < n, j < k, l < m, l < n
fn select_158([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < h {
        select_75([b, c, a, d, e, g, f, j, k, l, m, h, i])
    } else {
        select_157([a, b, d, c, e, f, g, h, i, l, m, j, n, k])
    }
}
/// n = 6, i = 2, b < f, c < d, c < e, c < f
fn select_159([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < b {
        select_56([a, e, d, b])
    } else {
        select_66([a, b, c, e, d, f])
    }
}
/// n = 8, i = 2, c < d, c < e, c < f, f < g, f < h
fn select_160([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_64([b, d, e, f, a, g, h])
    } else {
        select_159([b, a, c, d, e, f])
    }
}
/// n = 8, i = 2, b < h, c < d, c < e, c < h, e < f, e < g
fn select_161([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_34([a, d, e, b, f, g])
    } else {
        select_66([a, b, c, d, e, h])
    }
}
/// n = 9, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, g < h, g < i
fn select_162([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_161([a, d, b, f, g, h, i, e])
    } else {
        select_161([a, e, c, f, g, h, i, d])
    }
}
/// n = 9, i = 3, b < e, b < i, c < d, c < f, c < i, f < g, f < h
fn select_163([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_160([a, d, f, g, h, b, e, i])
    } else {
        select_162([a, b, c, d, e, i, f, g, h])
    }
}
/// n = 8, i = 3, b < e, b < f, b < h, c < d, c < f, c < g, d < h, e < g
fn select_164([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_66([a, d, b, f, e, h])
    } else {
        select_66([a, e, c, f, d, g])
    }
}
/// n = 7, i = 3, a < g, b < e, b < g, c < d, c < f, c < g, e < f
fn select_165([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_28([c, e, a, d, f])
    } else {
        select_98([a, b, c, d, e, g])
    }
}
/// n = 8, i = 3, b < e, b < f, c < d, c < f, c < h, e < h, f < g
fn select_166([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_62([d, e, f, a, g, h])
    } else {
        select_165([a, b, c, d, e, h, f])
    }
}
/// n = 9, i = 3, b < g, c < f, c < i, d < e, d < f, d < g, e < i, g < h
fn select_167([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < b {
        select_164([a, c, d, e, b, f, g, i])
    } else {
        select_166([a, b, d, e, c, g, h, f])
    }
}
/// n = 8, i = 3, a < f, b < d, b < e, c < e, c < f, c < g, d < g, f < h
fn select_168([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_48([a, d, c, e, g])
    } else {
        select_25([b, a, e, f, h])
    }
}
/// n = 9, i = 3, a < g, b < f, b < g, b < h, c < d, c < f, d < e, d < h, g < i
fn select_169([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_82([a, b, c, d, e, g, h, i])
    } else {
        select_168([a, c, b, d, f, g, h, i])
    }
}
/// n = 9, i = 3, b < g, c < e, c < f, d < f, d < g, d < h, e < h, g < i
fn select_170([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_78([a, b, c, d, f, e, g, i])
    } else {
        select_169([b, d, c, e, a, f, g, h, i])
    }
}
/// n = 9, i = 3, b < h, c < f, c < g, d < e, d < g, d < h, h < i
fn select_171([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_167([a, b, c, d, e, g, h, i, f])
    } else {
        select_170([a, b, c, d, f, g, h, e, i])
    }
}
/// n = 9, i = 3, c < f, c < i, d < e, d < g, d < i, g < h
fn select_172([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_163([b, c, d, e, f, g, a, h, i])
    } else {
        select_171([b, a, c, d, e, f, i, g, h])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < e, c < g, c < h, e < f
fn select_173([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_4([b, c, d, e])
    } else {
        select_37([a, d, e, b, g, h, f])
    }
}
/// n = 7, i = 4, a < e, b < f, c < f, d < e, d < g, f < g
fn select_174([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_19([a, d, f, g])
    } else {
        select_18([b, c, e])
    }
}
/// n = 8, i = 4, a < f, b < f, b < g, c < e, c < g, d < e, d < g, g < h
fn select_175([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_53([a, c, d, f, g, e, h])
    } else {
        select_174([a, c, d, b, f, e, g])
    }
}
/// n = 7, i = 3, a < d, b < e, b < f, c < e, c < f, f < g
fn select_176([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_98([b, c, a, d, e, f])
    } else {
        select_4([e, f, a, g])
    }
}
/// n = 8, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_177([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_175([a, d, b, c, f, e, g, h])
    } else {
        select_176([e, b, c, a, f, g, h])
    }
}
/// n = 10, i = 4, a < f, b < i, b < j, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_178([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_173([e, b, g, h, a, f, i, j])
    } else {
        select_177([b, a, c, d, e, f, g, h])
    }
}
/// n = 11, i = 5, a < j, a < k, b < g, b < h, c < f, c < h, d < e, d < h, h < i, h < j, h < k
fn select_179([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_178([e, a, b, c, f, g, h, i, j, k])
    } else {
        select_178([g, a, d, c, f, e, h, i, j, k])
    }
}
/// n = 7, i = 3, a < g, b < d, b < f, c < e, e < f, e < g
fn select_180([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_19([a, c, d, e])
    } else {
        select_11([a, b, e, d, f, g])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < e, e < g
fn select_181([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_47([a, b, c, e, f, g])
    } else {
        select_19([a, d, f, e])
    }
}
/// n = 8, i = 4, a < e, b < g, c < f, c < h, d < f, d < g, g < h
fn select_182([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_180([a, c, b, f, g, h, e])
    } else {
        select_181([b, c, d, a, e, f, g])
    }
}
/// n = 8, i = 2, a < h, b < g, c < d, c < g, c < h, d < e, d < f
fn select_183([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_40([b, d, a, e, f, g])
    } else {
        select_11([b, a, c, d, h, g])
    }
}
/// n = 8, i = 4, a < e, a < h, b < f, b < h, c < f, c < h, d < e, d < g, d < h, f < g
fn select_184([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_7([a, f, d, g, h])
    } else {
        select_19([e, b, c, h])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, b < h, c < f, c < h, d < f, d < h, f < g
fn select_185([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_184([b, c, d, a, e, f, g, h])
    } else {
        select_79([c, d, e, a, f])
    }
}
/// n = 11, i = 4, a < e, a < k, b < g, b < j, b < k, c < f, c < g, c < k, d < f, d < g, d < k, f < j, g < h, g < i
fn select_186([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_183([a, f, b, g, h, i, j, k])
    } else {
        select_185([b, a, c, d, e, f, j, k])
    }
}
/// n = 12, i = 5, a < e, a < l, b < h, b < i, b < l, c < g, d < f, d < i, d < l, f < h, g < i, g < l, i < j, i < k
fn select_187([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_182([a, c, b, f, e, h, g, i])
    } else {
        select_186([a, b, d, g, e, f, i, j, k, h, l])
    }
}
/// n = 12, i = 5, a < g, a < l, b < h, c < f, c < i, c < l, d < e, d < i, d < l, h < i, h < l, i < j, i < k
fn select_188([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_187([a, c, b, d, g, e, h, f, i, j, k, l])
    } else {
        select_187([a, d, b, c, g, f, h, e, i, j, k, l])
    }
}
/// n = 14, i = 5, a < g, a < i, b < h, b < n, c < f, c < i, c < k, d < e, d < i, d < k, h < k, i < j, i < n, k < l, k < m
fn select_189([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < h {
        select_179([b, a, c, d, e, f, g, i, j, h, n])
    } else {
        select_188([a, b, c, d, e, f, g, h, k, l, m, i])
    }
}
/// n = 14, i = 5, a < g, a < l, b < f, b < i, c < h, c < k, d < e, d < i, d < l, h < l, i < j, i < k, l < m, l < n
fn select_190([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_172([e, f, c, i, j, h, a, g, k])
    } else {
        select_189([b, c, a, d, e, g, f, h, i, j, l, m, n, k])
    }
}
/// n = 9, i = 3, a < d, a < e, b < f, b < i, c < e, c < f, d < i, f < g, f < h
fn select_191([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_24([b, d, c, e, i])
    } else {
        select_111([a, b, e, f, g, h])
    }
}
/// n = 10, i = 4, a < f, b < g, b < h, c < f, c < h, d < e, d < h, e < g, h < i, h < j
fn select_192([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_191([e, c, b, a, g, h, i, j, f])
    } else {
        select_54([a, b, c, d, f, e, h, i, j, g])
    }
}
/// n = 9, i = 3, a < d, a < e, b < d, b < f, c < e, c < f, e < i, f < g, f < h
fn select_193([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_3([a, g, h])
    } else {
        select_89([b, c, d, e, i, f])
    }
}
/// n = 10, i = 3, a < f, a < j, b < d, b < e, c < e, c < f, c < i, d < i, d < j, f < g, f < h
fn select_194([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_89([c, d, e, a, j, i])
    } else {
        select_193([b, c, a, e, d, f, g, h, j])
    }
}
/// n = 8, i = 3, a < d, a < e, b < f, c < e, c < f, f < g, f < h
fn select_195([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_22([b, d, c, e])
    } else {
        select_111([a, b, e, f, g, h])
    }
}
/// n = 11, i = 4, a < e, a < g, b < e, b < f, c < g, c < j, c < k, d < f, d < g, f < j, f < k, g < h, g < i
fn select_196([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_5([a, c, j, k, e])
    } else {
        select_195([b, c, d, e, f, g, h, i])
    }
}
/// n = 11, i = 4, a < f, a < k, b < f, b < h, c < g, c < h, c < k, d < e, d < h, e < g, e < k, h < i, h < j
fn select_197([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_194([b, e, c, a, g, h, i, j, k, f])
    } else {
        select_196([b, a, c, d, f, e, h, i, j, g, k])
    }
}
/// n = 11, i = 4, a < g, a < k, b < g, b < h, c < f, c < h, d < e, d < h, d < k, f < k, h < i, h < j
fn select_198([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_192([a, c, b, d, e, g, f, h, i, j])
    } else {
        select_197([a, b, d, c, f, g, e, h, i, j, k])
    }
}
/// n = 7, i = 2, a < c, b < d, d < e, d < f, d < g
fn select_199([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_39([a, e, f, g])
    } else {
        select_6([c, b, d])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, c < d, c < h, d < g, e < f, e < h
fn select_200([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_25([a, b, g, e, f])
    } else {
        select_48([f, a, c, d, h])
    }
}
/// n = 10, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < g, e < f, h < i, h < j
fn select_201([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_199([d, c, g, h, e, i, j])
    } else {
        select_200([a, c, b, d, e, f, g, h])
    }
}
/// n = 8, i = 3, a < d, b < e, c < e, e < f, e < g, e < h
fn select_202([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_39([a, f, g, h])
    } else {
        select_19([d, b, c, e])
    }
}
/// n = 8, i = 3, a < d, a < h, b < c, b < g, c < e, c < h, d < f, d < g
fn select_203([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_4([e, a, d, h])
    } else {
        select_4([f, b, c, g])
    }
}
/// n = 8, i = 4, a < f, b < f, c < d, c < h, d < e, f < g, f < h
fn select_204([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_19([e, a, b, f])
    } else {
        select_4([g, c, d, h])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, c < d, c < i, d < f, d < h, e < g, e < h, e < i
fn select_205([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_203([c, e, a, d, g, f, h, i])
    } else {
        select_204([a, b, c, d, f, e, g, i])
    }
}
/// n = 11, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < g, d < i, e < f, e < i, h < j, h < k
fn select_206([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < d {
        select_202([e, a, b, f, h, d, j, k])
    } else {
        select_205([a, b, c, e, d, f, g, i, h])
    }
}
/// n = 11, i = 4, a < d, a < h, b < e, b < h, c < f, c < h, e < i, f < g, f < i, h < j, h < k
fn select_207([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_201([b, a, c, d, f, g, e, h, j, k])
    } else {
        select_206([a, b, c, e, f, g, d, h, i, j, k])
    }
}
/// n = 12, i = 4, a < l, b < f, b < h, c < e, c < h, c < l, d < g, d < h, f < i, g < i, g < l, h < j, h < k
fn select_208([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < f {
        select_198([a, b, d, c, e, g, f, h, j, k, l])
    } else {
        select_207([c, d, b, e, g, f, a, h, i, j, k])
    }
}
/// n = 7, i = 4, a < g, b < d, c < e, d < g, e < f
fn select_209([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_25([a, c, g, e, f])
    } else {
        select_22([a, f, b, d])
    }
}
/// n = 11, i = 6, a < i, a < j, b < e, c < f, d < g, e < h, e < i, f < i, f < j, g < h, g < j, j < k
fn select_210([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_109([a, b, d, e, g, i, j, k, h])
    } else {
        select_209([a, c, d, f, g, h, i])
    }
}
/// n = 11, i = 6, a < h, a < j, b < e, c < f, d < g, e < j, f < h, f < i, g < i, g < j, j < k
fn select_211([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_210([a, c, b, d, f, e, g, i, h, j, k])
    } else {
        select_209([b, a, d, h, g, i, e])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, c < d, d < f, e < g
fn select_212([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_7([a, c, e, d, g])
    } else {
        select_7([a, b, d, e, f])
    }
}
/// n = 9, i = 4, a < g, a < h, a < i, b < f, b < g, c < e, c < g, d < e, d < f, e < h, f < i
fn select_213([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_212([a, b, c, e, f, h, i])
    } else {
        select_108([b, a, d, e, g, h])
    }
}
/// n = 10, i = 4, a < f, b < f, b < g, c < g, c < h, c < j, d < e, d < g, e < h, f < j, h < i
fn select_214([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_30([b, c, e, a, g, h, i, f])
    } else {
        select_213([c, b, d, a, e, f, g, h, j])
    }
}
/// n = 12, i = 6, a < e, b < i, b < j, b < k, c < f, c < j, d < g, e < i, f < k, g < h, g < k, h < j, k < l
fn select_215([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < h {
        select_211([b, c, a, d, f, e, g, i, h, k, l])
    } else {
        select_214([a, h, b, c, f, e, j, k, l, i])
    }
}
/// n = 7, i = 4, a < f, b < f, b < g, c < e, c < g, d < e, d < g
fn select_216([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_79([c, d, a, f, g])
    } else {
        select_22([a, e, b, g])
    }
}
/// n = 10, i = 5, a < f, b < e, c < g, c < h, c < i, d < f, d < h, e < g, e < h, f < i, i < j
fn select_217([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < i {
        select_216([a, d, c, e, g, f, h])
    } else {
        select_6([b, i, j])
    }
}
/// n = 11, i = 6, a < g, b < h, b < i, b < j, c < f, c < i, d < e, d < i, e < j, f < j, g < h, g < i, j < k
fn select_218([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_217([e, a, b, c, g, f, h, i, j, k])
    } else {
        select_217([f, a, b, d, g, e, h, i, j, k])
    }
}
/// n = 7, i = 2, a < f, a < g, b < d, b < e, b < g, c < d, c < e, c < f
fn select_219([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_27([b, c, f, g])
    } else {
        select_5([a, c, d, e, g])
    }
}
/// n = 8, i = 2, a < h, b < f, b < g, b < h, c < d, c < f, c < g, d < e
fn select_220([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_35([b, d, a, e, f, g])
    } else {
        select_219([a, b, c, f, g, d, h])
    }
}
/// n = 8, i = 2, a < g, a < h, b < e, c < d, c < g, c < h, e < f
fn select_221([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_29([c, b, a, d, e, f])
    } else {
        select_140([c, a, d, b, e, g, h])
    }
}
/// n = 10, i = 3, a < g, b < f, b < i, c < e, c < g, c < i, d < e, d < f, d < i, f < h, g < j
fn select_222([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_220([a, c, d, f, h, e, i, g])
    } else {
        select_221([b, a, d, e, g, j, f, i])
    }
}
/// n = 13, i = 6, a < e, a < m, b < h, b < i, b < k, c < g, c < i, d < f, d < i, e < h, f < k, g < j, g < k, g < m, i < j, i < m, k < l
fn select_223([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < i {
        select_218([a, b, c, d, f, g, e, h, i, k, l])
    } else {
        select_222([f, a, g, i, j, e, k, h, m, l])
    }
}
/// n = 13, i = 6, a < e, a < m, b < i, b < j, b < k, c < f, c < j, d < g, d < j, e < i, f < k, g < h, g < k, g < m, j < m, k < l
fn select_224([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < j {
        select_215([a, b, c, d, e, f, g, h, i, j, k, l])
    } else {
        select_223([a, b, d, c, e, f, g, i, j, h, k, l, m])
    }
}
/// n = 15, i = 6, a < j, a < k, b < e, b < l, c < f, c < i, d < g, d < i, e < j, f < k, g < h, g < k, g < l, i < l, k < o, l < m, l < n
fn select_225([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < a {
        select_208([f, b, g, i, h, e, a, l, j, m, n, k])
    } else {
        select_224([b, a, c, d, e, f, g, h, j, i, k, o, l])
    }
}
/// n = 7, i = 3, a < f, b < d, b < e, c < e, c < f, f < g
fn select_226([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_25([b, a, e, f, g])
    } else {
        select_22([a, d, c, e])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, b < g, c < e, c < f, d < f, d < g, f < i, g < h
fn select_227([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_226([a, c, d, e, f, g, h])
    } else {
        select_27([a, b, e, i])
    }
}
/// n = 9, i = 4, a < f, b < g, b < h, c < f, c < h, d < e, d < h, e < g, h < i
fn select_228([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_227([b, c, a, d, f, e, h, i, g])
    } else {
        select_26([c, b, e, a, g, h, i, f])
    }
}
/// n = 9, i = 3, a < f, a < i, b < e, b < f, b < g, c < d, c < e, d < g, d < i, f < h
fn select_229([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_89([b, d, e, a, i, g])
    } else {
        select_124([b, c, a, e, d, f, i, h])
    }
}
/// n = 10, i = 4, a < e, a < g, b < g, b < h, b < i, c < e, c < f, d < f, d < g, f < h, f < i, g < j
fn select_230([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_5([a, b, h, i, e])
    } else {
        select_226([b, c, d, e, f, g, j])
    }
}
/// n = 10, i = 4, a < f, a < j, b < f, b < h, c < g, c < h, c < j, d < e, d < h, e < g, e < j, h < i
fn select_231([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_229([b, c, e, a, g, h, j, i, f])
    } else {
        select_230([b, c, a, d, f, e, h, g, j, i])
    }
}
/// n = 10, i = 4, a < g, a < j, b < g, b < h, c < f, c < h, d < e, d < h, d < j, f < j, h < i
fn select_232([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_228([a, c, b, d, e, g, f, h, i])
    } else {
        select_231([a, b, d, c, f, g, e, h, i, j])
    }
}
/// n = 7, i = 2, a < d, a < e, b < c, b < f, f < g
fn select_233([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_76([b, a, d, e, c])
    } else {
        select_29([a, b, c, e, f, g])
    }
}
/// n = 8, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_234([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_4([d, e, a, g])
    } else {
        select_97([b, a, c, d, f, h, e])
    }
}
/// n = 8, i = 3, b < d, b < g, c < e, c < g, e < f, g < h
fn select_235([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_233([e, b, d, a, f, g, h])
    } else {
        select_234([a, b, c, d, g, e, h, f])
    }
}
/// n = 7, i = 3, a < e, a < g, b < c, b < d, d < g, e < f
fn select_236([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_28([b, e, f, c, d])
    } else {
        select_103([c, a, d, e, g])
    }
}
/// n = 8, i = 3, a < d, a < h, b < c, b < e, d < g, e < f, e < h
fn select_237([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_48([c, a, e, f, h])
    } else {
        select_236([a, b, c, e, d, g, h])
    }
}
/// n = 11, i = 5, a < d, a < j, b < f, b < h, c < e, c < j, d < i, e < h, e < i, f < g, f < j, j < k
fn select_238([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_235([g, a, c, d, e, h, j, k])
    } else {
        select_237([a, b, h, d, f, g, i, j])
    }
}
/// n = 12, i = 5, a < e, a < j, b < f, b < j, c < d, c < g, d < k, e < i, f < i, f < k, g < h, g < j, g < k, j < l
fn select_239([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_232([d, a, b, g, h, f, e, j, l, k])
    } else {
        select_238([b, c, a, f, e, g, h, d, i, j, l])
    }
}
/// n = 9, i = 3, a < e, a < i, b < e, b < f, c < d, c < f, d < i, f < g, f < h
fn select_240([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_33([d, f, a, g, h])
    } else {
        select_133([b, a, c, d, e, i, f])
    }
}
/// n = 11, i = 4, a < f, b < f, b < g, c < g, c < h, c < k, d < e, d < g, e < h, f < k, h < i, h < j
fn select_241([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_240([b, c, e, a, g, h, i, j, f])
    } else {
        select_213([c, b, d, a, e, f, g, h, k])
    }
}
/// n = 11, i = 5, a < e, b < h, b < k, c < g, c < h, d < f, e < g, e < k, f < h, f < k, h < i, h < j
fn select_242([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_20([b, c, d, g, f, k])
    } else {
        select_52([a, b, c, e, h, i, j, g, k])
    }
}
/// n = 11, i = 5, a < e, b < h, b < i, c < g, c < i, d < f, e < g, e < h, f < i, i < j, i < k
fn select_243([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_242([a, c, b, d, e, f, h, i, j, k, g])
    } else {
        select_20([d, b, c, h, g, f])
    }
}
/// n = 12, i = 5, a < f, b < h, b < i, b < j, c < g, c < i, d < e, d < j, e < i, f < h, g < j, j < k, j < l
fn select_244([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_241([a, e, b, c, g, f, i, j, k, l, h])
    } else {
        select_243([a, b, d, c, f, g, e, h, j, k, l])
    }
}
/// n = 10, i = 4, a < f, a < h, a < j, b < e, b < f, b < j, c < d, c < e, d < h, d < i, d < j, e < i, f < g
fn select_245([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_203([b, c, d, f, h, g, e, j])
    } else {
        select_90([a, d, b, e, h, i, f, j])
    }
}
/// n = 12, i = 4, a < f, a < l, b < f, b < g, b < l, c < d, c < e, d < i, e < g, e < i, e < l, f < h, i < j, i < k
fn select_246([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < a {
        select_240([b, e, d, a, g, i, j, k, f])
    } else {
        select_245([b, a, c, e, d, f, h, g, i, l])
    }
}
/// n = 8, i = 3, a < d, b < e, c < e, c < h, d < h, e < f, e < g
fn select_247([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_19([b, c, d, h])
    } else {
        select_4([a, e, f, g])
    }
}
/// n = 10, i = 4, a < g, b < e, c < g, c < j, d < f, d < g, d < j, e < f, e < j, g < h, g < i
fn select_248([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_48([a, c, d, f, j])
    } else {
        select_247([b, c, d, e, g, h, i, f])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < f
fn select_249([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_98([a, c, b, e, d, f])
    } else {
        select_21([b, d, a, e])
    }
}
/// n = 12, i = 6, a < e, b < h, b < i, b < j, c < g, c < i, d < f, d < g, e < h, e < i, f < j, g < j, j < k, j < l
fn select_250([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_248([f, a, g, b, e, i, j, k, l, h])
    } else {
        select_249([h, c, d, f, i, g])
    }
}
/// n = 14, i = 6, a < e, a < n, b < g, b < i, b < k, c < g, c < h, d < f, d < h, e < i, f < j, f < k, f < n, g < j, g < n, h < k, k < l, k < m
fn select_251([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < e {
        select_246([a, g, d, h, f, e, j, i, k, l, m, n])
    } else {
        select_250([a, b, c, d, e, f, h, i, g, k, l, m])
    }
}
/// n = 14, i = 6, a < e, a < n, b < h, b < j, b < k, c < h, c < i, d < f, d < i, e < j, f < g, f < k, f < n, h < n, i < k, k < l, k < m
fn select_252([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < h {
        select_244([a, b, c, f, g, e, i, j, h, k, l, m])
    } else {
        select_251([a, b, c, d, e, f, h, i, j, g, k, l, m, n])
    }
}
/// n = 15, i = 6, a < j, a < l, b < e, b < k, c < f, c < i, d < g, d < i, e < j, f < k, g < h, g < k, g < l, i < l, k < m, l < n, l < o
fn select_253([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < a {
        select_239([b, f, d, i, e, a, g, h, j, k, l, m])
    } else {
        select_252([b, a, c, d, e, g, h, f, i, j, l, n, o, k])
    }
}
/// n = 15, i = 6, a < f, a < k, b < e, b < l, c < g, c < j, d < h, d < j, g < k, h < i, h < k, h < l, j < l, k < o, l < m, l < n
fn select_254([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < f {
        select_225([a, b, c, d, e, g, h, i, j, f, k, l, m, n, o])
    } else {
        select_253([b, a, c, d, f, g, h, i, j, e, k, l, o, m, n])
    }
}
/// n = 15, i = 6, a < g, a < k, b < f, b < m, c < e, c < j, d < h, d < j, h < i, h < k, h < m, j < m, k < l, m < n, m < o
fn select_255([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < e {
        select_190([b, a, c, h, i, g, f, j, k, l, e, m, n, o])
    } else {
        select_254([a, b, c, d, f, g, e, h, i, j, k, m, n, o, l])
    }
}
/// n = 15, i = 6, a < g, a < o, b < f, b < k, c < e, c < m, d < h, d < i, h < m, i < j, i < k, i < m, k < l, m < n, m < o
fn select_256([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < a {
        select_158([b, c, h, i, j, e, f, a, g, k, l, m, n, o])
    } else {
        select_255([b, c, a, d, g, e, f, i, j, h, k, l, m, n, o])
    }
}
/// n = 7, i = 2, a < f, a < g, b < e, b < g, c < d, c < f
fn select_257([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_29([c, b, e, d, a, g])
    } else {
        select_13([c, a, d, b, e, g, f])
    }
}
/// n = 8, i = 2, a < e, b < c, b < d, d < f, e < g, e < h
fn select_258([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_4([c, d, a, f])
    } else {
        select_13([b, a, c, e, g, h, d])
    }
}
/// n = 9, i = 3, a < g, a < i, b < e, c < d, c < g, e < f, e < i, g < h
fn select_259([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_257([a, c, e, f, d, i, g])
    } else {
        select_258([b, c, d, g, e, h, f, i])
    }
}
/// n = 8, i = 2, a < g, a < h, b < f, b < g, b < h, c < d, c < f, d < e
fn select_260([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_5([b, d, a, e, f])
    } else {
        select_219([c, a, b, g, h, f, d])
    }
}
/// n = 9, i = 3, a < h, a < i, b < d, b < f, c < e, c < f, e < h, e < i, f < g
fn select_261([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_8([a, d, c, f, g, e])
    } else {
        select_260([a, e, b, f, g, d, h, i])
    }
}
/// n = 10, i = 3, a < f, a < j, b < d, b < h, c < e, c < h, e < j, f < g, h < i
fn select_262([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_259([c, a, b, d, f, g, h, i, e])
    } else {
        select_261([a, b, c, d, e, h, i, f, j])
    }
}
/// n = 8, i = 2, a < h, b < g, b < h, c < d, c < g, d < e, d < f
fn select_263([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < b {
        select_33([a, d, b, e, f])
    } else {
        select_11([a, c, b, d, g, h])
    }
}
/// n = 10, i = 3, a < d, a < j, b < f, b < i, c < e, c < f, e < i, e < j, f < g, f < h
fn select_264([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_263([a, e, b, f, g, h, i, j])
    } else {
        select_180([b, a, c, d, e, j, i])
    }
}
/// n = 9, i = 2, a < e, a < i, b < c, b < d, b < i, d < f, e < g, e < h
fn select_265([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_3([b, g, h])
    } else {
        select_62([c, d, a, e, i, f])
    }
}
/// n = 9, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h, f < i
fn select_266([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_3([b, h, i])
    } else {
        select_8([d, a, c, e, g, f])
    }
}
/// n = 10, i = 3, a < d, b < g, b < j, c < e, c < g, e < f, e < j, g < h, g < i
fn select_267([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_265([b, e, f, a, g, d, h, i, j])
    } else {
        select_266([b, a, c, d, e, g, f, h, i])
    }
}
/// n = 11, i = 3, a < d, a < k, b < e, b < h, c < f, c < h, e < k, f < g, h < i, h < j
fn select_268([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_264([a, c, b, d, e, h, i, j, f, k])
    } else {
        select_267([a, b, c, d, f, g, h, i, j, e])
    }
}
/// n = 12, i = 3, a < h, b < i, c < e, c < h, d < f, d < i, f < g, h < l, i < j, i < k
fn select_269([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_262([d, c, a, e, b, f, g, h, l, i])
    } else {
        select_268([c, b, d, e, a, f, g, i, j, k, h])
    }
}
/// n = 12, i = 3, a < h, b < i, c < g, c < h, d < e, d < f, d < i, h < l, i < j, i < k
fn select_270([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_269([a, b, c, d, g, e, f, h, i, j, k, l])
    } else {
        select_269([a, b, c, d, g, f, e, h, i, j, k, l])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, b < g, c < d, c < f, c < g
fn select_271([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_11([d, a, b, f, g, e])
    } else {
        select_11([e, a, c, f, g, d])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < g, c < d, c < f, c < g
fn select_272([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < b {
        select_6([d, f, b])
    } else {
        select_271([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, g < h, g < i
fn select_273([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_34([d, f, g, a, h, i])
    } else {
        select_272([b, a, c, d, e, f, g])
    }
}
/// n = 9, i = 3, c < f, c < g, d < e, d < f, d < g, g < h, g < i
fn select_274([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_273([a, c, d, e, b, f, g, h, i])
    } else {
        select_273([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 10, i = 3, a < d, a < g, b < c, b < g, b < j, c < f, d < e, d < j, g < h, g < i
fn select_275([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_199([c, a, f, g, d, h, i])
    } else {
        select_112([a, b, c, d, e, f, g, j])
    }
}
/// n = 8, i = 2, a < d, a < h, b < c, b < e, b < h, e < f, e < g
fn select_276([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_33([c, e, a, f, g])
    } else {
        select_101([a, b, c, d, e, h])
    }
}
/// n = 10, i = 3, a < e, a < g, a < j, b < c, b < d, b < g, d < j, e < f, g < h, g < i
fn select_277([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_276([e, b, c, f, g, h, i, d])
    } else {
        select_130([c, d, a, g, h, i, e, j])
    }
}
/// n = 12, i = 3, a < h, b < d, b < g, c < e, c < g, c < h, e < f, g < i, g < j, h < k, h < l
fn select_278([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_275([a, c, e, b, d, f, h, k, l, g])
    } else {
        select_277([c, b, d, a, e, f, g, i, j, h])
    }
}
/// n = 12, i = 3, a < h, b < f, b < g, c < d, c < e, c < g, c < h, g < i, g < j, h < k, h < l
fn select_279([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_278([a, b, c, f, d, e, g, h, i, j, k, l])
    } else {
        select_278([a, b, c, f, e, d, g, h, i, j, k, l])
    }
}
/// n = 13, i = 4, a < k, b < d, b < e, c < f, c < j, e < i, e < j, f < g, f < h, f < k, k < l, k < m
fn select_280([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_274([a, d, c, e, i, j, f, g, h])
    } else {
        select_279([a, b, f, g, h, d, e, k, i, j, l, m])
    }
}
/// n = 13, i = 4, a < k, b < e, b < f, c < d, c < g, f < j, g < h, g < i, g < k, k < l, k < m
fn select_281([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < f {
        select_270([d, a, b, g, h, i, e, f, k, l, m, j])
    } else {
        select_280([a, b, c, e, f, g, h, i, j, d, k, l, m])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < f, c < i, e < h, f < g
fn select_282([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_37([a, d, f, e, b, h, g])
    } else {
        select_173([a, b, c, d, f, g, e, i])
    }
}
/// n = 8, i = 2, b < e, b < h, c < d, c < f, c < h, f < g
fn select_283([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_34([a, e, c, d, f, h])
    } else {
        select_37([a, d, f, b, e, h, g])
    }
}
/// n = 9, i = 2, b < i, c < h, d < e, d < f, d < h, d < i, f < g
fn select_284([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_283([a, b, d, e, c, f, g, i])
    } else {
        select_283([a, c, d, e, b, f, g, h])
    }
}
/// n = 9, i = 2, c < i, d < e, d < f, d < g, d < i, g < h
fn select_285([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_282([b, c, d, f, e, g, h, a, i])
    } else {
        select_284([b, a, c, d, f, g, h, i, e])
    }
}
/// n = 8, i = 1, a < f, a < g, a < h, b < c, b < d, b < e
fn select_286([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_39([b, f, g, h])
    } else {
        select_39([a, c, d, e])
    }
}
/// n = 9, i = 1, a < i, b < g, b < h, c < d, c < e, c < f
fn select_287([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_129([a, c, d, e, f, b, i])
    } else {
        select_286([b, c, d, e, f, a, g, h])
    }
}
/// n = 10, i = 1, a < i, a < j, b < g, b < h, c < d, c < e, c < f
fn select_288([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_286([a, c, d, e, f, b, i, j])
    } else {
        select_286([b, c, d, e, f, a, g, h])
    }
}
/// n = 10, i = 1, b < j, c < h, c < i, d < e, d < f, d < g
fn select_289([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_287([a, c, d, e, f, g, h, i, b])
    } else {
        select_288([b, c, d, e, f, g, h, i, a, j])
    }
}
/// n = 10, i = 1, d < i, d < j, e < f, e < g, e < h
fn select_290([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_289([c, a, d, e, f, g, h, i, j, b])
    } else {
        select_289([c, b, d, e, f, g, h, i, j, a])
    }
}
/// n = 9, i = 2, a < i, b < h, c < d, c < h, c < i, d < e, d < f, d < g
fn select_291([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_41([a, c, b, d, e, f, g, i])
    } else {
        select_41([b, c, a, d, e, f, g, h])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < j, d < i, e < f, e < g, e < h
fn select_292([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_287([b, d, e, f, g, h, a, i, j])
    } else {
        select_291([a, b, c, e, f, g, h, j, d])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < f, c < j, f < g, f < h, f < i
fn select_293([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_292([a, b, c, d, f, g, h, i, e, j])
    } else {
        select_292([a, b, c, e, f, g, h, i, d, j])
    }
}
/// n = 11, i = 2, c < d, c < e, c < f, c < g, f < k, g < h, g < i, g < j
fn select_294([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_290([b, d, e, f, g, h, i, j, a, k])
    } else {
        select_293([b, a, c, d, e, g, h, i, j, f])
    }
}
/// n = 11, i = 2, d < e, d < f, d < g, d < h, g < k, h < i, h < j
fn select_295([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_285([b, c, a, d, e, f, g, k, h])
    } else {
        select_294([b, c, d, e, f, g, h, a, i, j, k])
    }
}
/// n = 7, i = 2, c < d, d < e, d < f, d < g
fn select_296([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_199([a, c, b, d, e, f, g])
    } else {
        select_199([b, c, a, d, e, f, g])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_297([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_292([a, d, b, f, g, h, i, j, k, e])
    } else {
        select_292([a, e, c, f, g, h, i, j, k, d])
    }
}
/// n = 11, i = 3, b < e, b < g, b < k, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_298([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_296([a, d, f, b, e, g, k])
    } else {
        select_297([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 6, i = 1, b < c, b < d, b < e, b < f
fn select_299([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_39([c, d, e, f])
    } else {
        select_33([a, b, d, e, f])
    }
}
/// n = 8, i = 2, a < d, a < g, a < h, b < c, b < e, b < g, b < h, e < f
fn select_300([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_62([c, e, a, g, h, f])
    } else {
        select_101([a, b, c, d, g, h])
    }
}
/// n = 11, i = 2, a < d, a < e, a < k, b < c, b < f, b < k, e < j, f < g, f < h, f < i
fn select_301([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_299([c, f, a, g, h, i])
    } else {
        select_300([b, a, d, c, e, j, f, k])
    }
}
/// n = 8, i = 3, a < d, a < h, b < e, b < h, c < e, c < h, e < f, e < g
fn select_302([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_19([d, b, c, h])
    } else {
        select_5([a, e, f, g, h])
    }
}
/// n = 8, i = 3, a < d, a < h, b < e, b < f, b < h, c < e, c < f, c < h, f < g
fn select_303([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_97([a, b, c, d, e, f, h])
    } else {
        select_302([a, b, c, d, f, e, g, h])
    }
}
/// n = 11, i = 3, a < e, a < f, b < d, b < g, c < e, c < f, c < g, f < k, g < h, g < i, g < j
fn select_304([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_299([d, g, a, h, i, j])
    } else {
        select_303([b, a, c, d, e, f, k, g])
    }
}
/// n = 12, i = 3, a < g, b < d, b < h, b < l, c < e, c < g, c < h, e < f, e < l, h < i, h < j, h < k
fn select_305([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_301([e, b, d, f, a, h, i, j, k, g, l])
    } else {
        select_304([a, b, c, d, g, e, h, i, j, k, f])
    }
}
/// n = 7, i = 1, a < g, b < c, b < d, b < e, b < f
fn select_306([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_2([b, g])
    } else {
        select_69([a, c, d, e, f])
    }
}
/// n = 10, i = 2, a < c, a < i, a < j, b < d, b < i, b < j, d < e, d < f, d < g, d < h
fn select_307([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_69([a, e, f, g, h])
    } else {
        select_33([c, b, d, i, j])
    }
}
/// n = 10, i = 2, a < d, a < j, b < c, b < e, b < j, e < f, e < g, e < h, e < i
fn select_308([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_306([c, e, f, g, h, i, a])
    } else {
        select_307([a, b, d, e, f, g, h, i, c, j])
    }
}
/// n = 12, i = 3, a < g, b < d, b < l, c < e, c < g, e < f, e < h, e < l, h < i, h < j, h < k
fn select_309([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_308([b, e, f, d, h, a, i, j, k, l])
    } else {
        select_105([a, b, c, d, e, f, g, l, h])
    }
}
/// n = 13, i = 4, a < d, a < f, b < c, b < e, b < i, e < h, e < m, f < g, f < i, f < m, i < j, i < k, i < l
fn select_310([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < c {
        select_305([d, f, b, g, e, h, c, i, j, k, l, m])
    } else {
        select_309([c, e, a, h, f, g, d, i, j, k, l, m])
    }
}
/// n = 13, i = 4, a < d, a < e, a < j, b < c, b < f, e < i, f < g, f < h, f < j, j < k, j < l, j < m
fn select_311([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_298([c, a, f, h, d, g, j, k, l, m, e])
    } else {
        select_310([b, a, d, c, e, f, h, i, j, k, l, m, g])
    }
}
/// n = 14, i = 4, a < d, a < e, b < c, b < f, e < i, f < g, f < h, f < j, j < k, j < l, j < m, m < n
fn select_312([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < a {
        select_295([c, g, h, j, k, l, m, a, d, e, n])
    } else {
        select_311([a, b, c, d, e, f, g, h, i, j, k, l, m])
    }
}
/// n = 14, i = 4, b < e, b < f, c < d, c < g, f < j, g < h, g < i, g < k, k < l, k < m, m < n
fn select_313([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < k {
        select_281([a, b, c, d, e, f, g, h, i, j, k, l, m])
    } else {
        select_312([b, c, d, e, f, g, h, i, j, k, a, l, m, n])
    }
}
/// n = 7, i = 4, a < f, a < g, b < f, b < g, c < e, c < g, d < e, d < g
fn select_314([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_19([e, a, b, g])
    } else {
        select_19([f, c, d, g])
    }
}
/// n = 6, i = 4, a < f, b < f, c < e, d < e
fn select_315([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_18([a, b, e])
    } else {
        select_18([c, d, f])
    }
}
/// n = 8, i = 5, a < g, a < h, b < g, b < h, c < f, d < e, d < h, f < h
fn select_316([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_314([a, b, d, f, e, g, h])
    } else {
        select_315([a, b, c, e, f, g])
    }
}
/// n = 11, i = 5, a < g, b < g, b < h, c < f, d < e, d < h, f < h, h < i, h < j, h < k
fn select_317([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_316([a, b, c, d, e, f, g, h])
    } else {
        select_299([e, h, a, i, j, k])
    }
}
/// n = 8, i = 2, a < c, b < d, b < e, d < h, e < f, e < g
fn select_318([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_35([d, e, f, g, a, h])
    } else {
        select_13([a, b, c, e, f, g, d])
    }
}
/// n = 8, i = 2, a < e, b < c, b < d, b < f, f < g, f < h
fn select_319([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_318([a, b, e, c, f, g, h, d])
    } else {
        select_318([a, b, e, d, f, g, h, c])
    }
}
/// n = 11, i = 5, a < g, b < e, b < k, c < g, c < h, d < f, f < h, h < i, h < j, h < k
fn select_320([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < h {
        select_317([a, c, d, b, e, f, g, h, i, j, k])
    } else {
        select_319([a, h, i, j, g, b, e, k])
    }
}
/// n = 8, i = 5, a < g, b < g, b < h, c < f, d < e, e < h, f < h
fn select_321([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_174([a, c, e, b, g, f, h])
    } else {
        select_174([a, d, f, b, g, e, h])
    }
}
/// n = 9, i = 5, a < e, b < g, b < i, c < g, c < h, d < f, f < h, h < i
fn select_322([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < e {
        select_180([a, b, f, g, h, i, e])
    } else {
        select_321([b, c, a, d, f, e, g, h])
    }
}
/// n = 11, i = 6, a < e, b < i, b < j, c < g, d < f, d < j, f < i, g < h, h < j, j < k
fn select_323([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_148([a, b, d, h, e, f, j, k, i])
    } else {
        select_322([a, b, f, c, e, g, i, h, j])
    }
}
/// n = 12, i = 6, a < e, a < l, b < i, b < j, c < g, d < f, d < j, f < i, g < h, g < j, j < k, j < l
fn select_324([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < h {
        select_320([f, a, b, c, e, g, i, j, h, k, l])
    } else {
        select_323([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 12, i = 6, a < g, a < l, b < f, b < j, c < e, c < j, d < h, h < i, h < j, j < k, j < l
fn select_325([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_324([a, b, d, c, g, e, h, i, f, j, k, l])
    } else {
        select_324([a, c, d, b, g, f, h, i, e, j, k, l])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < f
fn select_326([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < b {
        select_6([a, d, b])
    } else {
        select_66([a, c, b, e, d, f])
    }
}
/// n = 6, i = 3, a < f, b < e, c < d, c < f
fn select_327([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_99([b, a, c, d, f, e])
    } else {
        select_21([c, e, a, d])
    }
}
/// n = 7, i = 3, b < e, b < g, c < d, c < f, f < g
fn select_328([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_326([d, b, f, a, e, g])
    } else {
        select_327([a, b, c, d, e, f])
    }
}
/// n = 8, i = 3, b < f, c < g, d < e, d < g, g < h
fn select_329([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_328([a, d, b, f, e, c, g])
    } else {
        select_235([a, d, c, e, b, f, g, h])
    }
}
/// n = 8, i = 2, b < h, c < d, c < h, d < e, d < f, d < g
fn select_330([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < b {
        select_299([a, d, b, e, f, g])
    } else {
        select_103([a, b, c, d, h])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_331([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_330([a, d, b, f, g, h, i, e])
    } else {
        select_330([a, e, c, f, g, h, i, d])
    }
}
/// n = 9, i = 3, b < e, b < i, c < d, c < f, f < g, f < h, f < i
fn select_332([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_160([a, d, f, g, h, b, e, i])
    } else {
        select_331([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 9, i = 3, c < f, c < i, d < e, d < g, g < h, g < i
fn select_333([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_329([b, c, a, d, e, f, g, h])
    } else {
        select_332([b, c, d, e, f, g, a, h, i])
    }
}
/// n = 12, i = 6, a < g, a < l, b < f, b < l, c < e, c < j, d < h, h < i, h < j, j < k, j < l
fn select_334([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < j {
        select_325([a, b, c, d, e, f, g, h, i, j, k, l])
    } else {
        select_333([e, i, a, j, k, g, b, f, l])
    }
}
/// n = 7, i = 2, a < e, a < f, a < g, b < c, b < d, b < f, b < g
fn select_335([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_140([a, b, e, c, d, f, g])
    } else {
        select_140([a, b, e, d, c, f, g])
    }
}
/// n = 9, i = 2, a < e, a < i, b < c, b < d, b < f, b < i, f < g, f < h
fn select_336([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_34([c, d, f, a, g, h])
    } else {
        select_335([a, b, c, d, e, f, i])
    }
}
/// n = 10, i = 2, a < e, a < j, b < c, b < d, b < f, b < g, b < j, f < i, g < h
fn select_337([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_336([a, b, c, d, e, f, g, i, j])
    } else {
        select_336([a, b, c, d, e, g, f, h, j])
    }
}
/// n = 11, i = 4, a < e, b < g, b < k, c < f, c < g, d < f, d < g, f < k, g < h, g < i, g < j
fn select_338([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_306([f, g, a, h, i, j, k])
    } else {
        select_185([b, a, c, d, e, f, k, g])
    }
}
/// n = 13, i = 4, a < f, b < e, b < l, c < g, c < m, d < f, d < g, f < m, g < h, g < i, g < j, g < l, j < k
fn select_339([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_337([b, g, h, i, e, a, j, k, f, l])
    } else {
        select_338([b, c, a, d, e, f, g, h, i, j, m])
    }
}
/// n = 9, i = 2, a < e, a < h, a < i, b < c, b < d, b < f, b < h, b < i, f < g
fn select_340([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_37([c, d, f, a, h, i, g])
    } else {
        select_335([a, b, c, d, e, h, i])
    }
}
/// n = 11, i = 2, a < e, a < k, b < c, b < d, b < f, b < g, b < k, f < j, g < h, g < i
fn select_341([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_38([c, d, f, g, a, h, i, j])
    } else {
        select_340([a, b, c, d, e, f, j, g, k])
    }
}
/// n = 9, i = 5, a < e, a < i, b < g, b < i, c < g, c < i, d < f, f < h, f < i, g < h
fn select_342([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_184([a, b, c, f, e, g, h, i])
    } else {
        select_315([b, c, d, e, f, g])
    }
}
/// n = 12, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < l, g < l, h < i, h < j, h < k
fn select_343([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_306([g, h, a, i, j, k, l])
    } else {
        select_342([a, b, c, d, e, f, g, l, h])
    }
}
/// n = 12, i = 5, a < e, b < g, b < i, c < g, c < h, c < i, d < f, f < h, f < i, i < j, i < k, i < l
fn select_344([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_338([a, b, c, f, e, h, i, j, k, l, g])
    } else {
        select_343([a, b, c, d, e, f, g, i, j, k, l, h])
    }
}
/// n = 14, i = 5, a < g, a < h, b < e, b < n, c < g, c < i, d < f, f < h, f < i, i < j, i < k, i < l, i < n, l < m
fn select_345([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_341([b, i, j, k, e, l, a, g, h, m, n])
    } else {
        select_344([b, c, a, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 15, i = 6, a < e, a < o, b < i, b < j, c < g, d < f, d < j, f < i, g < h, g < j, j < k, j < l, j < m, j < o, m < n
fn select_346([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < f {
        select_339([h, a, b, d, e, f, j, k, l, m, n, o, i])
    } else {
        select_345([f, a, b, c, e, g, i, h, j, k, l, m, n, o])
    }
}
/// n = 15, i = 6, a < g, a < o, b < f, b < j, c < e, c < j, d < h, h < i, h < j, j < k, j < l, j < m, j < o, m < n
fn select_347([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < f {
        select_346([a, b, d, c, g, e, h, i, f, j, k, l, m, n, o])
    } else {
        select_346([a, c, d, b, g, f, h, i, e, j, k, l, m, n, o])
    }
}
/// n = 16, i = 6, a < g, a < j, b < f, b < l, c < e, c < l, d < h, h < i, h < j, h < l, j < k, l < m, l < n, l < o, o < p
fn select_348([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < l {
        select_334([b, c, a, d, g, e, f, h, i, j, k, l])
    } else {
        select_347([a, b, c, d, e, f, g, h, i, l, m, n, o, p, j])
    }
}
/// n = 16, i = 6, a < g, a < o, b < f, b < j, c < e, c < k, d < h, h < i, h < j, h < k, j < n, k < l, k < m, k < o, o < p
fn select_349([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < a {
        select_313([i, b, c, e, f, j, k, l, m, n, a, g, o, p])
    } else {
        select_348([b, a, c, d, e, g, f, h, i, j, n, k, l, m, o, p])
    }
}
/// n = 16, i = 6, a < h, a < o, b < g, b < k, c < f, c < l, d < e, d < i, i < j, i < k, i < l, k < n, l < m, l < o, o < p
fn select_350([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if e < l {
        select_256([a, b, c, d, f, g, h, e, i, j, k, n, l, m, o])
    } else {
        select_349([a, b, c, d, f, g, h, i, j, k, l, e, m, n, o, p])
    }
}
/// n = 16, i = 6, a < h, a < n, b < g, b < m, c < f, c < i, d < e, d < j, i < l, i < n, j < k, j < m, m < o, n < p
fn select_351([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_350([b, a, d, c, f, e, h, g, i, l, n, j, k, p, m, o])
    } else {
        select_350([a, b, c, d, e, f, g, h, j, k, m, i, l, o, n, p])
    }
}
/// n = 16, i = 6, a < h, a < o, b < g, b < j, c < f, c < i, d < e, d < k, i < m, j < n, k < l, k < o, o < p
fn select_352([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_351([a, b, d, c, f, e, g, h, k, i, m, l, j, o, n, p])
    } else {
        select_351([a, c, d, b, g, e, f, h, k, j, n, l, i, o, m, p])
    }
}
/// n = 16, i = 6, a < j, b < i, c < h, c < o, d < g, d < k, e < f, e < l, k < n, l < m, l < o, o < p
fn select_353([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_352([c, a, d, e, f, g, j, h, k, b, l, m, n, i, o, p])
    } else {
        select_352([c, b, d, e, f, g, i, h, k, a, l, m, n, j, o, p])
    }
}
/// n = 16, i = 6, c < j, d < i, d < o, e < h, e < k, f < g, f < l, k < n, l < m, l < o, o < p
fn select_354([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_353([a, c, d, e, f, g, h, i, j, b, k, l, m, n, o, p])
    } else {
        select_353([b, c, d, e, f, g, h, i, j, a, k, l, m, n, o, p])
    }
}
/// n = 16, i = 6, e < j, e < o, f < i, f < k, g < h, g < l, k < n, l < m, l < o, o < p
fn select_355([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_354([c, d, a, e, f, g, h, i, j, b, k, l, m, n, o, p])
    } else {
        select_354([c, d, b, e, f, g, h, i, j, a, k, l, m, n, o, p])
    }
}
/// n = 16, i = 6, e < j, e < m, f < i, f < l, g < h, g < k, k < n, l < o, m < p
fn select_356([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < m {
        select_355([a, b, c, d, e, g, f, i, h, j, k, l, o, n, m, p])
    } else {
        select_355([a, b, c, d, f, g, e, j, h, i, k, m, p, n, l, o])
    }
}
/// n = 16, i = 6, e < l, f < k, g < j, g < n, h < i, h < m, m < o, n < p
fn select_357([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if e < f {
        select_356([a, b, c, d, e, g, h, i, j, l, m, n, f, o, p, k])
    } else {
        select_356([a, b, c, d, f, g, h, i, j, k, m, n, e, o, p, l])
    }
}
/// n = 16, i = 6, g < l, h < k, h < n, i < j, i < m, m < o, n < p
fn select_358([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_357([c, d, e, f, a, g, h, i, j, k, l, b, m, n, o, p])
    } else {
        select_357([c, d, e, f, b, g, h, i, j, k, l, a, m, n, o, p])
    }
}
/// n = 16, i = 6, i < l, i < n, j < k, j < m, m < o, n < p
fn select_359([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_358([c, d, e, f, g, h, a, i, j, k, l, b, m, n, o, p])
    } else {
        select_358([c, d, e, f, g, h, b, i, j, k, l, a, m, n, o, p])
    }
}
/// n = 16, i = 6, i < n, j < m, k < l, k < o, o < p
fn select_360([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_359([a, b, c, d, e, f, g, h, i, k, l, n, o, j, p, m])
    } else {
        select_359([a, b, c, d, e, f, g, h, j, k, l, m, o, i, p, n])
    }
}
/// n = 16, i = 6, i < p, j < o, k < n, l < m
fn select_361([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < k {
        select_360([a, b, c, d, e, f, g, h, i, l, j, o, m, p, k, n])
    } else {
        select_360([a, b, c, d, e, f, g, h, i, l, k, n, m, p, j, o])
    }
}
/// n = 16, i = 6, k < p, l < o, m < n
fn select_362([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_361([c, d, e, f, g, h, i, j, a, k, l, m, n, o, p, b])
    } else {
        select_361([c, d, e, f, g, h, i, j, b, k, l, m, n, o, p, a])
    }
}
/// n = 16, i = 6, m < p, n < o
fn select_363([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_362([c, d, e, f, g, h, i, j, k, l, a, m, n, o, p, b])
    } else {
        select_362([c, d, e, f, g, h, i, j, k, l, b, m, n, o, p, a])
    }
}
/// n = 16, i = 6, o < p
fn select_364([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_363([c, d, e, f, g, h, i, j, k, l, m, n, a, o, p, b])
    } else {
        select_363([c, d, e, f, g, h, i, j, k, l, m, n, b, o, p, a])
    }
}
/// n = 16, i = 6
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_364([c, d, e, f, g, h, i, j, k, l, m, n, o, p, a, b])
    } else {
        select_364([c, d, e, f, g, h, i, j, k, l, m, n, o, p, b, a])
    }
}
