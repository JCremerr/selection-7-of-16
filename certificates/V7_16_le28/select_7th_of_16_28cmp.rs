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
/// n = 5, i = 1, a < e, b < c, b < d
fn select_4([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_2([b, e])
    } else {
        select_3([a, c, d])
    }
}
/// n = 6, i = 1, a < e, a < f, b < c, b < d
fn select_5([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_3([b, e, f])
    } else {
        select_3([a, c, d])
    }
}
/// n = 6, i = 1, b < f, c < d, c < e
fn select_6([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_4([a, c, d, e, b])
    } else {
        select_5([b, c, d, e, a, f])
    }
}
/// n = 6, i = 1, d < e, d < f
fn select_7([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_6([c, a, d, e, f, b])
    } else {
        select_6([c, b, d, e, f, a])
    }
}
/// n = 7, i = 1, a < g, b < f, c < d, c < e
fn select_8([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_5([a, c, d, e, b, g])
    } else {
        select_5([b, c, d, e, a, f])
    }
}
/// n = 7, i = 1, c < g, d < e, d < f
fn select_9([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_8([a, c, d, e, f, g, b])
    } else {
        select_8([b, c, d, e, f, g, a])
    }
}
/// n = 8, i = 2, b < e, b < f, c < d, c < g, g < h
fn select_10([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_7([a, e, f, c, d, g])
    } else {
        select_9([a, d, g, b, e, f, h])
    }
}
/// n = 9, i = 3, a < d, a < i, b < c, b < e, b < i, d < h, e < f, e < g
fn select_11([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < b {
        select_1([b])
    } else {
        select_10([c, e, a, i, f, g, d, h])
    }
}
/// n = 3, i = 1, b < c
fn select_12([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_2([a, c])
    }
}
/// n = 4, i = 1, b < c, b < d
fn select_13([a, b, c, d]: [usize; 4]) -> usize {
    if c < a {
        select_2([c, d])
    } else {
        select_12([a, b, d])
    }
}
/// n = 9, i = 3, a < d, a < e, b < c, b < f, b < i, e < i, f < g, f < h
fn select_14([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < b {
        select_13([e, b, c, f])
    } else {
        select_10([c, f, a, d, g, h, e, i])
    }
}
/// n = 9, i = 3, a < i, b < e, c < d, c < f, c < i, f < g, f < h
fn select_15([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_11([a, c, d, b, f, g, h, e, i])
    } else {
        select_14([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 9, i = 3, c < i, d < e, d < f, d < i, f < g, f < h
fn select_16([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_15([c, a, d, e, b, f, g, h, i])
    } else {
        select_15([c, b, d, e, a, f, g, h, i])
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
/// n = 5, i = 3, a < e, b < e, c < d
fn select_19([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_18([a, b, d])
    } else {
        select_17([c, e])
    }
}
/// n = 6, i = 3, a < e, b < e, b < f, c < d, d < f
fn select_20([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < f {
        select_19([a, b, c, d, e])
    } else {
        select_1([f])
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
/// n = 6, i = 3, a < f, b < d, c < e, e < f
fn select_22([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_20([b, a, c, e, d, f])
    } else {
        select_21([c, d, a, e])
    }
}
/// n = 4, i = 2, c < d
fn select_23([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_21([a, c, d, b])
    } else {
        select_21([b, c, d, a])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, c < d, c < f, e < g
fn select_24([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_22([a, c, b, d, e, g])
    } else {
        select_23([b, d, a, f])
    }
}
/// n = 5, i = 2, a < c, b < d, d < e
fn select_25([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < a {
        select_2([a, e])
    } else {
        select_12([c, b, d])
    }
}
/// n = 8, i = 3, a < d, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_26([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_25([a, c, d, e, g])
    } else {
        select_25([b, c, d, f, h])
    }
}
/// n = 8, i = 3, a < f, a < h, b < e, b < f, c < d, c < e, d < h, f < g
fn select_27([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_25([b, d, e, a, h])
    } else {
        select_26([b, c, a, e, d, f, h, g])
    }
}
/// n = 5, i = 2, a < e, b < d, c < d, c < e
fn select_28([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_12([b, c, e])
    } else {
        select_12([a, c, d])
    }
}
/// n = 6, i = 2, a < f, b < f, c < d, d < e
fn select_29([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_13([b, d, a, e])
    } else {
        select_28([b, c, a, d, f])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, c < d, c < f, d < h, f < g
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_20([b, a, c, d, e, h])
    } else {
        select_29([a, d, b, f, g, h])
    }
}
/// n = 8, i = 3, a < e, a < f, b < f, b < g, c < d, c < g, g < h
fn select_31([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_27([c, b, a, e, f, g, h, d])
    } else {
        select_30([a, b, c, d, f, g, h, e])
    }
}
/// n = 8, i = 3, b < g, c < e, c < f, d < f, d < g, g < h
fn select_32([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_24([d, a, c, e, b, f, g])
    } else {
        select_31([c, d, b, a, e, f, g, h])
    }
}
/// n = 8, i = 1, a < h, b < g, c < d, c < e, c < f
fn select_33([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_8([a, b, c, d, f, g, h])
    } else {
        select_8([a, b, c, e, f, g, h])
    }
}
/// n = 8, i = 1, c < h, d < e, d < f, d < g
fn select_34([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_33([a, c, d, e, f, g, h, b])
    } else {
        select_33([b, c, d, e, f, g, h, a])
    }
}
/// n = 7, i = 1, b < g, c < d, c < e, c < f
fn select_35([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_6([a, b, c, d, f, g])
    } else {
        select_6([a, b, c, e, f, g])
    }
}
/// n = 4, i = 0
fn select_36([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_3([a, c, d])
    } else {
        select_3([b, c, d])
    }
}
/// n = 7, i = 2, a < c, b < d, d < e, d < f, d < g
fn select_37([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_36([a, e, f, g])
    } else {
        select_12([c, b, d])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, d < e, d < f, d < g
fn select_38([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_36([a, e, f, g])
    } else {
        select_13([c, b, d, h])
    }
}
/// n = 8, i = 2, b < h, c < d, c < h, d < e, d < f, d < g
fn select_39([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_37([a, c, b, d, e, f, g])
    } else {
        select_38([b, c, a, d, e, f, g, h])
    }
}
/// n = 8, i = 2, c < d, c < e, e < f, e < g, e < h
fn select_40([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_35([b, d, e, f, g, h, a])
    } else {
        select_39([b, a, c, e, f, g, h, d])
    }
}
/// n = 10, i = 3, b < d, b < g, b < j, c < e, c < g, e < f, e < h, e < j, h < i
fn select_41([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_34([a, f, h, b, d, g, j, i])
    } else {
        select_40([a, d, c, g, e, f, h, j])
    }
}
/// n = 10, i = 3, c < e, c < g, c < j, d < f, d < g, f < h, f < j, h < i
fn select_42([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_32([b, a, c, d, e, g, f, h])
    } else {
        select_41([b, c, d, e, f, a, g, h, i, j])
    }
}
/// n = 4, i = 1, a < d, b < c
fn select_43([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_2([b, d])
    } else {
        select_2([a, c])
    }
}
/// n = 6, i = 2, a < e, a < f, b < d, b < f, c < d, c < e
fn select_44([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_43([b, c, e, f])
    } else {
        select_43([a, c, d, f])
    }
}
/// n = 7, i = 3, a < f, a < g, b < d, b < g, c < e, c < g, e < f
fn select_45([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_28([a, d, c, e, g])
    } else {
        select_44([a, b, e, d, f, g])
    }
}
/// n = 8, i = 3, a < e, b < f, b < h, c < d, c < f, e < h, f < g
fn select_46([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_13([d, f, a, g])
    } else {
        select_45([b, c, a, d, e, h, f])
    }
}
/// n = 4, i = 2, b < d, c < d
fn select_47([a, b, c, d]: [usize; 4]) -> usize {
    if b < a {
        select_12([a, c, d])
    } else {
        select_17([b, c])
    }
}
/// n = 8, i = 3, a < d, a < h, b < e, b < h, c < e, c < h, e < f, e < g
fn select_48([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_47([d, b, c, h])
    } else {
        select_4([a, e, f, g, h])
    }
}
/// n = 9, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h, f < i
fn select_49([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_13([d, e, a, g])
    } else {
        select_48([b, a, c, d, f, h, i, e])
    }
}
/// n = 9, i = 3, a < e, a < g, b < d, b < f, c < f, c < g, f < h, g < i
fn select_50([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_46([b, c, a, e, d, g, i, f])
    } else {
        select_49([b, a, c, e, g, f, i, d, h])
    }
}
/// n = 9, i = 3, b < g, c < e, c < f, d < f, d < g, f < h, g < i
fn select_51([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_46([a, d, c, e, b, f, h, g])
    } else {
        select_50([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 6, i = 1, a < f, b < c, b < d, b < e
fn select_52([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_2([b, f])
    } else {
        select_36([a, c, d, e])
    }
}
/// n = 8, i = 1, a < f, a < g, a < h, b < c, b < d, b < e
fn select_53([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_36([b, f, g, h])
    } else {
        select_36([a, c, d, e])
    }
}
/// n = 8, i = 1, b < g, b < h, c < d, c < e, c < f
fn select_54([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_52([a, c, d, e, f, b])
    } else {
        select_53([b, c, d, e, f, a, g, h])
    }
}
/// n = 9, i = 2, c < d, c < e, d < i, e < f, e < g, e < h
fn select_55([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_54([b, d, e, f, g, h, a, i])
    } else {
        select_39([b, a, c, e, f, g, h, d])
    }
}
/// n = 11, i = 3, b < d, b < h, b < k, c < e, c < h, e < f, e < g, e < k, g < i, h < j
fn select_56([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_34([a, f, g, b, d, h, k, i])
    } else {
        select_55([a, d, c, h, e, f, g, k, j])
    }
}
/// n = 11, i = 3, c < e, c < h, c < k, d < f, d < h, f < g, f < k, g < i, h < j
fn select_57([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_51([b, a, c, d, e, h, f, j, g])
    } else {
        select_56([b, c, d, e, f, a, g, h, i, j, k])
    }
}
/// n = 11, i = 3, c < g, c < i, d < h, d < i, e < f, e < g, e < h, h < j, i < k
fn select_58([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_42([a, b, e, c, f, d, g, i, k, h])
    } else {
        select_57([a, b, e, d, f, c, i, h, k, j, g])
    }
}
/// n = 11, i = 3, c < j, d < i, d < j, e < f, e < g, e < i, g < h, j < k
fn select_59([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < c {
        select_16([a, b, d, e, f, g, c, h, i])
    } else {
        select_58([a, b, d, c, e, f, i, g, j, h, k])
    }
}
/// n = 5, i = 2, a < d, a < e, b < c, b < e
fn select_60([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_12([c, a, e])
    } else {
        select_12([d, b, e])
    }
}
/// n = 7, i = 3, a < e, a < g, b < e, b < f, b < g, c < d, c < g, d < f
fn select_61([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_12([a, b, f])
    } else {
        select_60([b, c, d, e, g])
    }
}
/// n = 9, i = 4, a < f, a < h, b < e, b < f, c < d, c < e, d < h, d < i, e < i, f < g
fn select_62([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_60([c, f, g, e, d])
    } else {
        select_61([a, d, b, e, h, i, f])
    }
}
/// n = 6, i = 2, a < f, b < e, b < f, c < d, c < e
fn select_63([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_43([b, d, a, e])
    } else {
        select_44([a, b, c, e, d, f])
    }
}
/// n = 9, i = 4, a < f, a < i, b < f, b < g, c < d, d < e, d < g, e < i, f < h
fn select_64([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_12([h, c, d])
    } else {
        select_63([e, a, b, g, f, i])
    }
}
/// n = 9, i = 4, a < g, a < h, b < f, b < g, c < d, c < f, d < e, d < h, g < i
fn select_65([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_62([a, b, c, d, f, g, i, h, e])
    } else {
        select_64([b, a, c, d, e, g, h, i, f])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, c < d, d < f, e < g
fn select_66([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_28([a, c, e, d, g])
    } else {
        select_28([a, b, d, e, f])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < e, c < f
fn select_67([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_60([a, c, d, b, f])
    } else {
        select_60([b, c, d, a, e])
    }
}
/// n = 10, i = 4, a < e, a < i, b < e, b < h, c < f, d < f, d < j, e < j, f < g, f < h, f < i
fn select_68([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_47([c, d, e, j])
    } else {
        select_67([a, b, f, g, h, i])
    }
}
/// n = 10, i = 4, a < f, a < i, b < e, b < f, c < g, c < j, d < e, d < g, f < j, g < h, g < i
fn select_69([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_66([c, a, d, e, f, g, j])
    } else {
        select_68([a, b, d, c, f, g, h, e, i, j])
    }
}
/// n = 10, i = 4, a < j, b < e, b < f, c < g, c < i, d < f, d < g, e < i, g < h, g < j
fn select_70([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_65([c, d, b, e, a, f, g, i, h])
    } else {
        select_69([a, b, c, d, f, e, g, h, j, i])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < g, c < e, c < f, d < e, d < f, d < g
fn select_71([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_28([b, c, d, f, g])
    } else {
        select_28([a, c, d, e, g])
    }
}
/// n = 9, i = 4, a < g, a < h, a < i, b < f, b < i, c < f, c < h, d < e, e < h, e < i, f < g
fn select_72([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_71([a, b, c, e, f, h, i])
    } else {
        select_28([a, d, f, e, g])
    }
}
/// n = 12, i = 5, a < g, a < j, a < l, b < f, b < g, c < h, c < k, c < l, d < g, d < h, e < f, e < h, f < l, g < k, h < i, h < j
fn select_73([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_72([c, a, d, e, f, g, k, h, l])
    } else {
        select_68([a, b, e, c, g, h, i, f, j, k])
    }
}
/// n = 12, i = 5, a < k, a < l, b < f, b < g, c < h, c < j, c < l, d < f, d < h, e < g, e < h, f < l, g < j, h < i, h < k
fn select_74([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_65([c, d, b, g, a, f, h, j, i])
    } else {
        select_73([a, b, c, e, d, f, g, h, i, k, j, l])
    }
}
/// n = 12, i = 5, a < k, a < l, b < g, c < h, c < i, c < l, d < g, d < i, e < f, e < i, f < h, g < l, i < j, i < k
fn select_75([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_70([a, f, d, c, b, h, i, j, g, k])
    } else {
        select_74([a, b, c, d, e, g, f, i, j, h, k, l])
    }
}
/// n = 6, i = 3, a < d, b < e, c < e, e < f
fn select_76([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < f {
        select_19([b, c, a, d, e])
    } else {
        select_1([f])
    }
}
/// n = 9, i = 4, a < f, a < g, b < d, b < f, c < e, d < i, e < g, e < i, g < h
fn select_77([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_76([a, c, d, f, e, i])
    } else {
        select_26([a, b, e, f, d, g, i, h])
    }
}
/// n = 9, i = 4, a < d, a < g, b < g, b < h, c < e, e < f, e < h, h < i
fn select_78([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_30([a, b, e, f, g, h, i, d])
    } else {
        select_77([b, a, c, d, e, g, h, i, f])
    }
}
/// n = 6, i = 3, a < e, b < c, b < d, e < f
fn select_79([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < c {
        select_60([b, e, f, d, c])
    } else {
        select_47([d, a, c, e])
    }
}
/// n = 6, i = 2, a < c, a < f, b < d, b < f, d < e
fn select_80([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < b {
        select_1([b])
    } else {
        select_4([d, a, c, f, e])
    }
}
/// n = 6, i = 2, a < d, b < c, b < e, e < f
fn select_81([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_43([c, e, f, a])
    } else {
        select_80([a, b, d, e, f, c])
    }
}
/// n = 8, i = 3, a < c, a < d, b < e, e < f, e < g, g < h
fn select_82([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < g {
        select_79([b, a, c, d, e, f])
    } else {
        select_81([a, e, f, d, g, h])
    }
}
/// n = 9, i = 4, a < d, a < g, b < g, b < h, c < e, c < h, e < f, h < i
fn select_83([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < h {
        select_78([a, b, c, d, e, f, g, h, i])
    } else {
        select_82([a, c, d, g, h, i, e, f])
    }
}
/// n = 6, i = 3, a < f, b < e, c < e, c < f, d < e, d < f
fn select_84([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_28([a, b, d, e, f])
    } else {
        select_28([a, b, c, e, f])
    }
}
/// n = 7, i = 3, a < g, b < g, c < e, d < e, d < g, e < f
fn select_85([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_13([b, e, a, f])
    } else {
        select_84([b, c, a, d, e, g])
    }
}
/// n = 8, i = 2, a < c, b < d, c < h, d < e, d < f, d < g
fn select_86([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_36([a, e, f, g])
    } else {
        select_43([b, c, h, d])
    }
}
/// n = 10, i = 3, a < f, a < i, b < f, b < g, b < h, c < d, d < e, d < g, d < h, d < i, f < j
fn select_87([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < b {
        select_6([e, a, b, g, h, i])
    } else {
        select_86([a, c, f, d, e, g, h, j])
    }
}
/// n = 11, i = 4, a < e, a < h, b < h, b < i, b < j, c < f, d < f, d < h, f < g, f < i, f < j, h < k
fn select_88([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_85([c, e, b, d, h, k, f])
    } else {
        select_87([a, b, c, f, g, h, i, j, e, k])
    }
}
/// n = 10, i = 4, a < f, a < g, a < j, b < e, c < d, c < j, d < g, d < i, e < f, e < i, e < j, g < h
fn select_89([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_12([b, g, h])
    } else {
        select_61([a, e, c, d, f, i, j])
    }
}
/// n = 11, i = 5, a < g, b < h, b < i, b < k, c < g, c < i, d < e, d < k, e < f, e < i, g < h, g < k, i < j
fn select_90([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_85([a, f, b, c, i, j, g])
    } else {
        select_89([b, a, d, e, g, h, i, j, f, k])
    }
}
/// n = 13, i = 5, a < g, a < k, a < m, b < g, b < h, c < h, c < j, c < k, d < e, d < h, e < f, e < k, g < j, h < i, h < m, k < l
fn select_91([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_88([e, a, b, c, f, h, i, k, g, m, l])
    } else {
        select_90([b, c, a, d, e, f, g, j, k, l, h])
    }
}
/// n = 13, i = 5, a < k, a < m, b < e, b < i, c < h, c < i, c < k, d < f, d < i, e < h, f < g, f < k, i < j, i < m, k < l
fn select_92([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < a {
        select_83([e, c, d, a, f, g, h, i, j])
    } else {
        select_91([a, b, c, d, f, g, e, i, j, h, k, l, m])
    }
}
/// n = 13, i = 5, b < k, b < m, c < f, c < i, d < h, d < i, d < k, e < g, e < i, f < h, g < k, i < j, i < m, k < l
fn select_93([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < g {
        select_75([b, a, d, e, c, f, g, h, i, j, m, k])
    } else {
        select_92([b, c, d, e, f, g, a, h, i, j, k, l, m])
    }
}
/// n = 8, i = 4, a < f, b < e, c < f, c < g, d < e, d < h, e < g, f < h
fn select_94([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_47([b, d, f, h])
    } else {
        select_47([a, c, e, g])
    }
}
/// n = 9, i = 4, a < h, a < i, b < f, b < g, c < e, d < f, d < g, d < i, e < g, e < i, f < h
fn select_95([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_94([b, c, d, a, e, f, i, h])
    } else {
        select_61([b, d, e, a, f, i, g])
    }
}
/// n = 12, i = 5, a < h, a < l, b < g, b < j, b < k, c < f, c < g, d < f, d < h, e < g, e < h, e < k, f < k, g < l, h < i, h < j
fn select_96([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_95([a, b, d, e, f, g, k, l, h])
    } else {
        select_68([b, c, d, a, g, h, i, f, j, l])
    }
}
/// n = 12, i = 5, a < k, a < l, b < h, b < j, c < f, c < g, d < g, d < h, e < f, e < h, e < k, f < j, g < k, h < i, h < l
fn select_97([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_65([b, d, c, f, a, g, h, j, i])
    } else {
        select_96([b, a, c, d, e, g, f, h, i, l, k, j])
    }
}
/// n = 12, i = 5, a < k, a < l, b < g, c < h, c < i, d < g, d < i, e < f, e < i, e < l, f < h, g < l, i < j, i < k
fn select_98([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_70([a, f, d, c, b, h, i, j, g, k])
    } else {
        select_97([a, c, b, d, e, f, g, i, j, h, l, k])
    }
}
/// n = 6, i = 2, a < d, a < e, a < f, b < c, b < e, b < f
fn select_99([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_60([a, b, c, d, e])
    } else {
        select_60([a, b, c, d, f])
    }
}
/// n = 7, i = 2, a < d, a < g, b < c, b < e, b < g, e < f
fn select_100([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_13([c, e, a, f])
    } else {
        select_99([a, b, c, d, e, g])
    }
}
/// n = 6, i = 2, a < c, b < d, d < e, d < f
fn select_101([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < b {
        select_1([b])
    } else {
        select_4([a, d, e, f, c])
    }
}
/// n = 7, i = 3, a < e, b < e, b < f, c < d, c < f, f < g
fn select_102([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_101([a, c, e, f, d, g])
    } else {
        select_20([a, b, c, d, e, f])
    }
}
/// n = 8, i = 3, a < d, b < g, b < h, c < e, c < g, e < f, e < h
fn select_103([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_100([b, e, f, g, a, d, h])
    } else {
        select_102([b, c, a, d, g, e, f])
    }
}
/// n = 9, i = 4, a < g, a < i, b < d, b < g, c < e, c < i, d < h, d < i, e < f, e < h
fn select_104([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_102([a, b, e, f, g, d, h])
    } else {
        select_48([a, c, d, g, e, f, h, i])
    }
}
/// n = 10, i = 4, a < g, a < h, b < d, b < g, c < e, c < h, d < j, e < f, e < j, h < i
fn select_105([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_103([e, b, a, f, h, i, g, d])
    } else {
        select_104([a, b, c, d, e, f, g, j, h])
    }
}
/// n = 7, i = 3, b < g, c < e, c < g, d < e, d < g, e < f
fn select_106([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_4([b, e, a, f, g])
    } else {
        select_84([a, b, c, d, g, e])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < g, b < h, c < d, c < h, d < f, d < g
fn select_107([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_13([a, b, f, g])
    } else {
        select_60([b, c, d, e, h])
    }
}
/// n = 9, i = 3, a < e, a < i, b < e, b < h, b < i, c < d, c < i, d < f, d < h, f < g
fn select_108([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_13([a, f, b, g])
    } else {
        select_107([a, b, c, d, e, f, h, i])
    }
}
/// n = 11, i = 5, a < h, a < k, b < g, b < i, c < g, c < i, d < e, d < k, e < f, e < i, g < h, g < k, i < j
fn select_109([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_106([a, f, b, c, i, j, g])
    } else {
        select_108([a, g, d, e, h, i, j, f, k])
    }
}
/// n = 13, i = 5, a < h, a < j, b < g, b < k, b < m, c < g, c < h, c < k, d < e, d < h, e < f, e < k, g < j, h < i, h < m, k < l
fn select_110([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_88([e, b, a, c, f, h, i, k, g, m, l])
    } else {
        select_109([a, b, c, d, e, f, g, j, k, l, h])
    }
}
/// n = 13, i = 5, a < k, a < m, b < h, b < i, c < f, c < i, d < e, d < i, d < k, e < h, f < g, f < k, i < j, i < m, k < l
fn select_111([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < a {
        select_105([b, e, c, a, f, g, h, i, j, k])
    } else {
        select_110([b, a, d, c, f, g, e, i, j, h, k, l, m])
    }
}
/// n = 13, i = 5, b < k, b < m, c < h, c < i, d < g, d < i, e < f, e < i, e < k, f < h, g < k, i < j, i < m, k < l
fn select_112([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < g {
        select_98([b, a, c, d, e, f, g, h, i, j, m, k])
    } else {
        select_111([b, c, d, e, f, g, a, h, i, j, k, l, m])
    }
}
/// n = 13, i = 5, b < k, b < m, c < g, c < i, d < h, d < i, e < f, e < i, e < k, h < k, i < j, i < m, k < l
fn select_113([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_93([a, b, c, e, d, g, h, f, i, j, k, l, m])
    } else {
        select_112([a, b, c, d, e, f, h, g, i, j, k, l, m])
    }
}
/// n = 13, i = 5, b < g, c < k, c < l, d < h, d < i, e < f, e < i, e < l, h < l, i < j, i < k, l < m
fn select_114([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < b {
        select_59([a, f, h, c, i, j, b, g, k, l, m])
    } else {
        select_113([a, c, b, d, e, f, g, h, i, j, l, m, k])
    }
}
/// n = 9, i = 2, b < e, b < g, c < d, c < f, f < h, g < i
fn select_115([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_9([a, e, g, c, d, f, i])
    } else {
        select_9([a, d, f, b, e, g, h])
    }
}
/// n = 10, i = 2, b < e, b < f, c < d, c < g, f < j, g < h, g < i
fn select_116([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_115([a, b, c, d, e, g, f, h, j])
    } else {
        select_115([a, b, c, d, e, g, f, i, j])
    }
}
/// n = 8, i = 2, a < d, a < g, a < h, b < c, b < e, b < g, b < h, e < f
fn select_117([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_6([c, e, a, g, h, f])
    } else {
        select_99([a, b, c, d, g, h])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < e, c < g, c < h, e < f
fn select_118([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_81([a, c, d, b, e, f])
    } else {
        select_117([b, c, d, a, e, f, g, h])
    }
}
/// n = 6, i = 2, a < e, a < f, b < c, b < d, b < f
fn select_119([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_80([a, b, e, c, d, f])
    } else {
        select_80([a, b, e, d, c, f])
    }
}
/// n = 9, i = 2, a < e, a < i, b < c, b < d, b < f, b < i, f < g, f < h
fn select_120([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_9([c, d, a, f, g, h, i])
    } else {
        select_119([a, b, c, d, e, i])
    }
}
/// n = 11, i = 3, a < e, a < g, b < f, b < h, c < d, c < f, c < g, c < h, g < k, h < i, h < j
fn select_121([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_118([e, b, c, d, g, k, f, h])
    } else {
        select_120([a, c, d, f, e, h, i, j, g])
    }
}
/// n = 11, i = 3, a < e, b < h, b < i, c < d, c < f, c < h, c < i, f < g, i < j, i < k
fn select_122([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_116([d, f, b, h, g, a, i, j, k, e])
    } else {
        select_121([a, b, c, d, e, h, f, i, j, k, g])
    }
}
/// n = 11, i = 3, c < h, c < i, d < e, d < f, d < h, d < i, f < g, i < j, i < k
fn select_123([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_122([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_122([b, c, d, e, a, f, g, h, i, j, k])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < f
fn select_124([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_28([a, d, b, e, f])
    } else {
        select_28([a, e, c, d, f])
    }
}
/// n = 6, i = 4, a < f, b < f, c < e, d < e
fn select_125([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_18([a, b, e])
    } else {
        select_18([c, d, f])
    }
}
/// n = 6, i = 4, b < f, c < f, d < e
fn select_126([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_125([b, c, a, d, e, f])
    } else {
        select_19([b, c, e, a, f])
    }
}
/// n = 7, i = 4, b < g, c < f, d < e, d < g, f < g
fn select_127([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_124([b, d, f, a, e, g])
    } else {
        select_126([b, a, c, d, e, f])
    }
}
/// n = 8, i = 4, b < e, c < g, d < f, f < g, g < h
fn select_128([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < g {
        select_127([a, c, d, b, e, f, g])
    } else {
        select_13([a, g, b, h])
    }
}
/// n = 6, i = 2, c < d, d < e, d < f
fn select_129([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_101([a, c, b, d, e, f])
    } else {
        select_101([b, c, a, d, e, f])
    }
}
/// n = 7, i = 3, c < e, d < e, e < f, e < g
fn select_130([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_47([b, c, d, e])
    } else {
        select_129([a, b, d, e, f, g])
    }
}
/// n = 10, i = 4, a < e, b < i, b < j, c < f, d < f, f < g, f < h, f < i, f < j
fn select_131([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_9([g, h, a, b, i, j, e])
    } else {
        select_130([b, e, c, d, f, g, h])
    }
}
/// n = 10, i = 4, a < f, b < i, b < j, c < g, d < e, d < g, g < h, g < i, g < j
fn select_132([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_128([b, a, c, d, f, e, g, h])
    } else {
        select_131([a, b, c, d, f, g, e, h, i, j])
    }
}
/// n = 6, i = 3, a < f, b < f, c < e, d < e, d < f
fn select_133([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_20([c, d, a, b, e, f])
    } else {
        select_20([c, d, b, a, e, f])
    }
}
/// n = 7, i = 3, b < e, c < f, d < e, d < f, f < g
fn select_134([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_101([b, c, e, f, a, g])
    } else {
        select_133([a, c, b, d, e, f])
    }
}
/// n = 9, i = 5, b < g, c < h, d < g, d < h, e < f, f < h, h < i
fn select_135([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_134([a, b, c, d, g, h, i])
    } else {
        select_127([a, c, e, d, g, f, h])
    }
}
/// n = 11, i = 5, a < g, b < j, b < k, c < h, d < g, d < h, e < f, e < h, h < i, h < j, h < k
fn select_136([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_131([a, b, c, e, g, h, f, i, j, k])
    } else {
        select_135([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 11, i = 5, b < j, b < k, c < h, d < g, d < h, e < f, e < h, h < i, h < j, h < k
fn select_137([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_132([f, b, c, d, g, a, h, i, j, k])
    } else {
        select_136([a, b, c, e, d, g, f, h, i, j, k])
    }
}
/// n = 13, i = 5, b < g, c < j, c < k, d < h, e < f, e < h, h < i, h < j, h < k, k < l, k < m
fn select_138([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_123([a, f, c, h, i, b, g, j, k, l, m])
    } else {
        select_137([a, c, d, b, e, f, g, h, i, j, k])
    }
}
/// n = 7, i = 2, a < c, a < f, a < g, b < d, b < f, b < g, d < e
fn select_139([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_13([c, b, f, g])
    } else {
        select_4([d, a, f, g, e])
    }
}
/// n = 7, i = 2, a < e, a < f, a < g, b < c, b < d, b < f, b < g
fn select_140([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_139([a, b, e, c, d, f, g])
    } else {
        select_139([a, b, e, d, c, f, g])
    }
}
/// n = 8, i = 3, b < d, b < g, b < h, c < e, e < f, e < g, e < h
fn select_141([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_76([b, a, c, d, e, f])
    } else {
        select_140([b, e, a, f, d, g, h])
    }
}
/// n = 7, i = 4, a < g, b < g, c < f, c < g, d < e, d < g
fn select_142([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_133([a, b, e, c, f, g])
    } else {
        select_133([a, b, f, d, e, g])
    }
}
/// n = 10, i = 5, a < h, b < f, b < g, b < j, c < g, c < h, d < e, d < h, h < i, h < j
fn select_143([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_141([e, b, a, g, h, i, f, j])
    } else {
        select_142([a, f, c, d, e, g, h])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < f
fn select_144([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_63([a, c, b, e, f, d])
    } else {
        select_80([b, c, e, d, a, f])
    }
}
/// n = 7, i = 2, b < g, c < d, c < e, c < g, e < f
fn select_145([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_6([d, b, e, a, f, g])
    } else {
        select_67([a, b, c, d, g, e])
    }
}
/// n = 8, i = 2, b < e, b < h, c < d, c < f, c < h, f < g
fn select_146([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_144([a, b, c, d, e, h])
    } else {
        select_145([a, b, c, d, f, g, h])
    }
}
/// n = 8, i = 3, a < f, b < e, b < g, c < d, c < f, c < g, g < h
fn select_147([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_119([b, c, d, f, e, g])
    } else {
        select_145([e, a, c, d, g, h, f])
    }
}
/// n = 9, i = 3, b < e, b < g, b < i, c < d, c < f, c < g, f < i, g < h
fn select_148([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_146([d, f, b, e, a, g, h, i])
    } else {
        select_147([a, b, c, d, e, f, g, h])
    }
}
/// n = 11, i = 5, a < g, b < h, c < f, c < g, c < j, d < e, d < h, h < i, h < j, j < k
fn select_149([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_143([b, c, a, d, e, f, g, h, i, j])
    } else {
        select_148([e, c, h, i, f, a, j, k, g])
    }
}
/// n = 7, i = 1, a < f, a < g, b < c, b < d, b < e
fn select_150([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_3([b, f, g])
    } else {
        select_36([a, c, d, e])
    }
}
/// n = 9, i = 1, a < i, b < g, b < h, c < d, c < e, c < f
fn select_151([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_150([a, c, d, e, f, b, i])
    } else {
        select_53([b, c, d, e, f, a, g, h])
    }
}
/// n = 9, i = 1, c < h, c < i, d < e, d < f, d < g
fn select_152([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_151([a, c, d, e, f, g, h, i, b])
    } else {
        select_151([b, c, d, e, f, g, h, i, a])
    }
}
/// n = 9, i = 2, a < i, b < h, c < d, c < h, c < i, d < e, d < f, d < g
fn select_153([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_38([a, c, b, d, e, f, g, i])
    } else {
        select_38([b, c, a, d, e, f, g, h])
    }
}
/// n = 10, i = 3, a < j, b < e, b < f, b < j, c < d, c < f, c < j, f < g, f < h, f < i
fn select_154([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_153([a, d, b, f, g, h, i, e, j])
    } else {
        select_153([a, e, c, f, g, h, i, d, j])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_155([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_152([d, e, f, g, h, i, j, a, k])
    } else {
        select_154([a, b, c, d, e, g, h, i, j, f])
    }
}
/// n = 5, i = 0
fn select_156([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_36([a, c, d, e])
    } else {
        select_36([b, c, d, e])
    }
}
/// n = 10, i = 3, a < d, b < e, c < e, c < j, d < j, e < f, e < g, e < h, e < i
fn select_157([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_156([a, f, g, h, i])
    } else {
        select_28([b, d, c, j, e])
    }
}
/// n = 5, i = 2, b < e, c < d, c < e
fn select_158([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_21([a, c, d, b])
    } else {
        select_60([b, c, d, a, e])
    }
}
/// n = 10, i = 3, a < d, a < e, b < f, c < e, c < f, f < g, f < h, f < i, f < j
fn select_159([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_156([a, g, h, i, j])
    } else {
        select_158([d, b, c, e, f])
    }
}
/// n = 10, i = 3, b < e, c < f, d < e, d < f, f < g, f < h, f < i, f < j
fn select_160([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_157([a, c, d, b, f, g, h, i, j, e])
    } else {
        select_159([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 11, i = 4, b < g, c < f, c < g, d < e, d < g, g < h, g < i, g < j, g < k
fn select_161([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_160([a, e, b, c, f, g, h, i, j, k])
    } else {
        select_160([a, f, b, d, e, g, h, i, j, k])
    }
}
/// n = 13, i = 5, a < h, b < e, b < h, c < d, c < f, c < j, f < g, f < l, f < m, h < i, h < j, h < l, h < m, j < k
fn select_162([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < f {
        select_155([e, c, h, i, d, j, f, g, l, m, k])
    } else {
        select_161([d, a, b, f, g, e, h, i, j, l, m])
    }
}
/// n = 13, i = 5, b < h, c < f, c < g, c < j, d < e, d < h, g < l, g < m, h < i, h < j, h < l, h < m, j < k
fn select_163([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < g {
        select_149([a, b, c, d, e, f, g, h, i, j, k])
    } else {
        select_162([b, d, c, f, e, g, a, h, i, j, k, l, m])
    }
}
/// n = 9, i = 2, a < c, b < d, c < i, d < e, d < f, d < g, d < h
fn select_164([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_156([a, e, f, g, h])
    } else {
        select_43([b, c, i, d])
    }
}
/// n = 4, i = 1, c < d
fn select_165([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_43([a, c, d, b])
    } else {
        select_43([b, c, d, a])
    }
}
/// n = 9, i = 2, a < c, a < d, b < e, e < f, e < g, e < h, e < i
fn select_166([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_156([a, f, g, h, i])
    } else {
        select_165([c, d, b, e])
    }
}
/// n = 9, i = 2, b < d, c < e, e < f, e < g, e < h, e < i
fn select_167([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_164([a, c, b, e, f, g, h, i, d])
    } else {
        select_166([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 12, i = 3, a < c, a < d, a < l, b < e, d < h, e < f, e < g, e < i, e < l, i < j, i < k
fn select_168([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_152([f, g, i, a, c, d, l, j, k])
    } else {
        select_167([c, d, b, h, e, f, g, i, l])
    }
}
/// n = 6, i = 1, c < d, c < e, c < f
fn select_169([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_52([a, c, d, e, f, b])
    } else {
        select_52([b, c, d, e, f, a])
    }
}
/// n = 7, i = 3, a < d, b < c, b < g, c < f, d < e, d < g
fn select_170([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_12([f, a, d])
    } else {
        select_13([e, b, c, g])
    }
}
/// n = 10, i = 4, a < f, a < i, a < j, b < f, b < g, b < j, c < d, d < e, d < g, d < j, e < i, f < h
fn select_171([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_63([e, a, b, g, j, i])
    } else {
        select_170([c, a, f, d, g, h, j])
    }
}
/// n = 12, i = 4, a < f, a < l, b < f, b < g, b < i, c < d, d < e, d < g, d < i, e < l, f < h, i < j, i < k
fn select_172([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < a {
        select_169([e, g, i, a, j, k])
    } else {
        select_171([a, b, c, d, e, f, g, h, l, i])
    }
}
/// n = 10, i = 3, a < d, a < j, b < e, b < f, b < j, c < e, c < f, c < i, c < j, d < i, f < g, f < h
fn select_173([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_6([e, a, f, g, h, j])
    } else {
        select_61([b, c, a, d, e, i, j])
    }
}
/// n = 12, i = 4, a < d, a < f, b < f, b < g, b < i, c < e, d < l, e < g, e < i, e < l, f < h, i < j, i < k
fn select_174([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_158([d, c, f, h, e])
    } else {
        select_173([a, b, e, d, g, i, j, k, l, f])
    }
}
/// n = 12, i = 4, a < d, a < g, b < g, b < h, b < j, c < e, e < f, e < h, e < j, g < i, j < k, j < l
fn select_175([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < d {
        select_172([a, b, c, e, f, g, h, i, j, k, l, d])
    } else {
        select_174([a, b, c, d, e, g, h, i, j, k, l, f])
    }
}
/// n = 13, i = 4, a < d, b < f, c < e, c < i, c < j, e < h, e < m, f < g, f < i, f < j, f < m, j < k, j < l
fn select_176([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < a {
        select_168([e, b, h, a, f, g, i, d, j, k, l, m])
    } else {
        select_175([a, c, b, d, f, g, e, i, h, j, k, l])
    }
}
/// n = 13, i = 4, a < d, a < e, a < m, b < f, c < f, c < i, e < i, f < g, f < h, f < j, f < m, j < k, j < l
fn select_177([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_152([g, h, j, a, d, e, m, k, l])
    } else {
        select_160([d, e, b, c, i, f, g, h, j, m])
    }
}
/// n = 13, i = 5, a < e, a < f, b < g, c < f, c < i, c < j, d < e, d < g, e < m, f < h, g < i, g < j, g < m, j < k, j < l
fn select_178([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_124([b, d, f, h, e, g])
    } else {
        select_173([a, c, g, e, i, j, k, l, m, f])
    }
}
/// n = 8, i = 4, a < f, b < f, c < d, c < h, d < e, f < g, f < h
fn select_179([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_47([e, a, b, f])
    } else {
        select_13([g, c, d, h])
    }
}
/// n = 11, i = 5, a < e, a < j, a < k, b < e, b < i, b < k, c < f, d < f, e < g, f < h, f < i, f < k, h < j
fn select_180([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_63([h, a, b, i, k, j])
    } else {
        select_179([c, d, a, e, g, f, i, k])
    }
}
/// n = 13, i = 5, a < e, a < m, b < e, b < i, b < j, c < f, d < f, e < g, f < h, f < i, f < j, h < m, j < k, j < l
fn select_181([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < a {
        select_169([h, i, j, a, k, l])
    } else {
        select_180([a, b, c, d, e, f, g, h, i, m, j])
    }
}
/// n = 13, i = 5, a < e, a < f, b < g, c < f, c < j, c < k, d < e, d < g, f < i, g < h, g < j, g < k, k < l, k < m
fn select_182([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < h {
        select_178([a, b, c, d, e, f, g, i, j, k, l, m, h])
    } else {
        select_181([a, c, b, d, f, g, i, h, j, k, l, m, e])
    }
}
/// n = 14, i = 5, a < g, b < h, c < e, c < j, c < k, d < g, d < h, e < f, e < n, h < i, h < j, h < k, h < n, k < l, k < m
fn select_183([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < a {
        select_177([e, b, d, f, a, h, i, j, g, k, l, m, n])
    } else {
        select_182([a, b, c, d, g, e, h, i, f, j, k, l, m])
    }
}
/// n = 14, i = 5, b < h, c < f, c < j, c < k, d < e, d < h, f < g, f < n, h < i, h < j, h < k, h < n, k < l, k < m
fn select_184([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < a {
        select_176([e, b, c, a, f, h, i, g, j, k, l, m, n])
    } else {
        select_183([a, b, c, d, f, g, e, h, i, j, k, l, m, n])
    }
}
/// n = 15, i = 5, b < g, b < k, c < j, c < l, d < h, e < f, e < h, h < i, h < j, h < k, h < l, k < o, l < m, l < n
fn select_185([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if b < c {
        select_163([a, d, b, e, f, g, c, h, i, k, o, j, l])
    } else {
        select_184([a, d, c, e, f, b, g, h, i, j, l, m, n, k])
    }
}
/// n = 15, i = 5, b < g, b < n, c < h, c < l, d < i, d < m, e < f, e < h, e < i, h < k, h < m, i < j, i < l, i < n, n < o
fn select_186([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_138([a, b, d, c, e, f, g, h, k, m, i, j, l])
    } else {
        select_185([a, b, c, d, e, f, g, i, j, l, n, h, k, m, o])
    }
}
/// n = 15, i = 5, b < h, b < n, c < g, c < i, d < j, d < m, e < f, e < i, e < j, i < l, i < m, j < k, j < n, n < o
fn select_187([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < j {
        select_114([a, b, d, c, e, f, h, g, i, l, m, j, k])
    } else {
        select_186([a, b, c, d, e, f, h, i, j, k, l, g, m, n, o])
    }
}
/// n = 6, i = 3, c < e, d < e, e < f
fn select_188([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_76([a, c, d, b, e, f])
    } else {
        select_76([b, c, d, a, e, f])
    }
}
/// n = 8, i = 4, a < h, b < e, b < g, c < f, d < f, f < g, f < h
fn select_189([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < f {
        select_188([a, e, c, d, f, g])
    } else {
        select_4([a, b, e, g, h])
    }
}
/// n = 9, i = 4, a < e, b < h, b < i, c < f, d < f, f < g, f < h, f < i
fn select_190([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_6([g, a, b, h, i, e])
    } else {
        select_188([b, e, c, d, f, g])
    }
}
/// n = 10, i = 4, a < e, a < j, b < g, c < f, c < i, d < f, d < g, f < j, g < h, g < i
fn select_191([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_189([b, a, c, d, e, f, j, g])
    } else {
        select_190([a, c, b, d, e, g, h, f, i])
    }
}
/// n = 11, i = 4, a < e, a < k, b < f, b < h, c < f, c < g, d < g, d < h, f < k, g < i, h < j
fn select_192([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_191([a, d, b, c, e, f, g, i, h, k])
    } else {
        select_191([a, d, c, b, e, f, h, j, g, k])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < e, e < g
fn select_193([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_84([a, b, c, e, f, g])
    } else {
        select_47([a, d, f, e])
    }
}
/// n = 8, i = 4, a < h, b < g, b < h, c < f, d < e, d < g, e < h, f < g
fn select_194([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_193([b, c, a, d, e, f, h])
    } else {
        select_124([b, d, f, a, e, g])
    }
}
/// n = 10, i = 4, a < e, b < h, b < i, b < j, c < f, d < f, d < j, e < j, f < g, f < h, f < i
fn select_195([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_6([g, a, b, h, i, e])
    } else {
        select_85([b, e, c, d, f, g, j])
    }
}
/// n = 10, i = 4, a < f, b < g, c < e, c < i, c < j, d < g, d < j, f < j, g < h, g < i
fn select_196([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_194([b, d, a, c, e, f, j, g])
    } else {
        select_195([a, c, b, d, f, g, h, e, i, j])
    }
}
/// n = 7, i = 3, a < g, b < d, b < f, c < e, e < f, e < g
fn select_197([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_80([b, e, d, a, g, f])
    } else {
        select_76([b, a, c, d, e, g])
    }
}
/// n = 8, i = 3, a < h, b < g, c < f, c < h, d < e, d < h, e < f, e < g
fn select_198([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_45([c, a, d, b, e, f, h])
    } else {
        select_197([c, b, d, a, e, g, f])
    }
}
/// n = 5, i = 1, c < d, c < e
fn select_199([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_4([a, c, d, e, b])
    } else {
        select_4([b, c, d, e, a])
    }
}
/// n = 8, i = 2, a < e, a < g, a < h, b < c, b < d, b < g, b < h, e < f
fn select_200([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_199([c, d, a, g, h])
    } else {
        select_139([b, a, d, e, f, g, h])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < g, c < d, d < g, d < h, e < f
fn select_201([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_12([c, e, f])
    } else {
        select_28([a, b, d, g, h])
    }
}
/// n = 10, i = 3, a < j, b < e, b < i, c < d, c < g, c < h, d < i, d < j, e < f, e < g, e < h
fn select_202([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_200([c, e, a, f, d, i, g, h])
    } else {
        select_201([a, b, c, d, e, f, i, j])
    }
}
/// n = 11, i = 4, a < k, b < g, b < j, c < e, c < f, c < i, d < f, d < g, f < j, f < k, g < h, g < i
fn select_203([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_198([e, a, b, d, f, j, k, g])
    } else {
        select_202([a, b, c, f, g, h, e, i, j, k])
    }
}
/// n = 11, i = 4, a < f, b < h, b < k, c < e, c < g, c < j, d < g, d < h, g < k, h < i, h < j
fn select_204([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_196([a, b, c, d, e, f, h, i, j, g])
    } else {
        select_203([a, b, c, d, e, g, h, i, j, k, f])
    }
}
/// n = 13, i = 5, a < e, a < m, b < i, b < j, c < g, c < j, d < f, d < j, f < i, g < h, g < l, i < m, j < k, j < l
fn select_205([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_192([a, b, f, c, e, i, g, j, h, k, m])
    } else {
        select_204([a, b, g, d, h, e, f, j, k, l, i])
    }
}
/// n = 8, i = 4, a < e, b < g, c < f, c < h, d < f, d < g, g < h
fn select_206([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_197([a, c, b, f, g, h, e])
    } else {
        select_193([b, c, d, a, e, f, g])
    }
}
/// n = 9, i = 3, a < d, b < f, b < i, c < e, c < f, e < i, f < g, f < h
fn select_207([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_52([e, f, a, g, h, i])
    } else {
        select_45([b, a, c, d, e, i, f])
    }
}
/// n = 10, i = 4, a < g, b < e, c < g, c < h, d < f, f < h, h < i, h < j
fn select_208([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_206([b, d, c, a, e, g, f, h])
    } else {
        select_207([b, c, f, e, a, h, i, j, g])
    }
}
/// n = 6, i = 3, a < e, a < f, b < d, b < f, c < d, c < e
fn select_209([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_21([a, c, d, f])
    } else {
        select_21([b, c, e, f])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < e
fn select_210([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_209([a, b, c, e, d, f])
    } else {
        select_21([b, d, a, e])
    }
}
/// n = 6, i = 3, b < f, c < e, d < e, d < f
fn select_211([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_20([c, d, a, b, e, f])
    } else {
        select_210([c, d, b, a, f, e])
    }
}
/// n = 8, i = 3, a < d, b < f, b < h, c < e, c < f, e < h, f < g
fn select_212([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_4([e, f, a, g, h])
    } else {
        select_45([b, a, c, d, e, h, f])
    }
}
/// n = 9, i = 4, a < e, b < g, b < i, c < f, c < h, d < f, d < g, f < i, g < h
fn select_213([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < c {
        select_211([e, b, c, d, f, g])
    } else {
        select_212([a, b, d, e, f, g, h, i])
    }
}
/// n = 11, i = 4, a < e, b < g, c < g, c < h, d < f, d < k, f < h, g < k, h < i, h < j
fn select_214([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_207([a, c, f, e, b, h, i, j, g])
    } else {
        select_213([a, c, d, b, e, f, g, k, h])
    }
}
/// n = 11, i = 4, a < f, b < g, c < h, d < h, d < k, e < g, e < h, g < k, h < i, h < j
fn select_215([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_208([b, a, e, c, f, d, g, h, i, j])
    } else {
        select_214([a, b, e, d, f, c, g, h, i, j, k])
    }
}
/// n = 9, i = 3, a < d, a < e, b < f, b < i, c < e, c < f, d < i, f < g, f < h
fn select_216([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_3([a, g, h])
    } else {
        select_63([d, b, c, e, f, i])
    }
}
/// n = 5, i = 1, b < c, b < d, b < e
fn select_217([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < a {
        select_3([c, d, e])
    } else {
        select_13([a, b, d, e])
    }
}
/// n = 9, i = 3, a < e, a < i, b < e, b < f, c < d, c < f, d < i, f < g, f < h
fn select_218([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_217([d, f, a, g, h])
    } else {
        select_61([b, a, c, d, e, i, f])
    }
}
/// n = 9, i = 3, a < e, a < f, b < f, b < g, c < d, c < g, g < h, g < i
fn select_219([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_216([a, c, b, e, f, g, h, i, d])
    } else {
        select_218([a, b, c, d, f, g, h, i, e])
    }
}
/// n = 7, i = 3, a < g, b < e, c < d, c < f, d < g, e < f
fn select_220([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_20([b, a, c, d, e, g])
    } else {
        select_60([c, e, a, d, f])
    }
}
/// n = 10, i = 4, a < g, a < j, b < f, b < h, b < i, c < e, c < g, d < e, d < f, e < h, f < j, g < i
fn select_221([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_66([b, a, d, e, g, h, i])
    } else {
        select_220([a, c, b, f, e, h, j])
    }
}
/// n = 12, i = 4, a < k, a < l, b < f, b < g, c < g, c < h, d < e, d < f, e < h, f < k, g < l, h < i, h < j
fn select_222([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_219([b, c, e, a, f, g, h, i, j])
    } else {
        select_221([c, a, b, d, f, e, g, k, l, h])
    }
}
/// n = 7, i = 3, a < e, b < e, b < f, b < g, c < d, d < f, d < g
fn select_223([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_13([a, b, f, g])
    } else {
        select_21([b, c, d, e])
    }
}
/// n = 8, i = 3, a < f, a < g, b < d, c < e, c < h, d < h, e < f, e < g
fn select_224([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_60([c, d, a, e, h])
    } else {
        select_223([b, a, c, e, d, f, g])
    }
}
/// n = 10, i = 4, a < f, a < j, b < g, b < i, c < e, c < h, d < f, d < g, d < j, e < j, f < i, g < h
fn select_225([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_224([c, a, b, f, g, e, h, i])
    } else {
        select_95([b, a, c, d, e, f, j, i, g])
    }
}
/// n = 10, i = 4, a < e, b < g, b < j, c < f, c < g, d < f, d < g, f < j, g < h, g < i
fn select_226([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < c {
        select_47([b, c, d, f])
    } else {
        select_207([a, b, d, e, f, g, h, i, j])
    }
}
/// n = 13, i = 5, a < e, a < l, b < h, b < i, c < g, c < m, d < f, d < i, e < m, f < h, f < m, g < i, h < l, i < j, i < k
fn select_227([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_225([c, b, a, f, e, g, h, l, i, m])
    } else {
        select_226([a, b, d, g, e, f, i, j, k, h])
    }
}
/// n = 13, i = 5, a < e, a < m, b < i, b < j, c < g, c < h, d < f, d < j, f < h, f < i, g < j, i < m, j < k, j < l
fn select_228([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_222([a, f, b, c, g, h, i, j, k, l, e, m])
    } else {
        select_227([a, b, c, d, e, f, g, i, j, k, l, m, h])
    }
}
/// n = 13, i = 5, a < f, a < m, b < i, b < j, c < e, c < h, d < g, d < j, g < i, h < j, i < m, j < k, j < l
fn select_229([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < g {
        select_215([a, e, h, b, d, f, g, j, k, l, i])
    } else {
        select_228([a, b, c, d, f, g, h, e, i, j, k, l, m])
    }
}
/// n = 13, i = 5, a < g, a < m, b < f, b < l, c < j, d < i, d < j, e < h, e < j, h < i, i < l, j < k, j < m
fn select_230([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < a {
        select_205([b, d, c, e, f, h, a, g, i, j, k, m, l])
    } else {
        select_229([b, d, a, e, g, f, h, c, i, j, k, m, l])
    }
}
/// n = 7, i = 2, b < d, b < g, c < e, c < g, e < f
fn select_231([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < b {
        select_12([a, e, f])
    } else {
        select_144([a, b, c, e, d, g])
    }
}
/// n = 10, i = 3, a < g, a < j, b < e, b < g, b < h, c < d, c < f, f < h, f < j, g < i
fn select_232([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_231([d, b, f, e, a, j, h])
    } else {
        select_50([b, c, a, d, e, f, g, j, i])
    }
}
/// n = 7, i = 2, b < e, c < d, c < f, f < g
fn select_233([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < b {
        select_199([a, d, f, b, g])
    } else {
        select_144([a, b, c, d, e, f])
    }
}
/// n = 7, i = 3, a < f, b < f, b < g, c < d, d < e, d < g
fn select_234([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_165([a, e, b, g])
    } else {
        select_25([b, c, f, d, e])
    }
}
/// n = 8, i = 3, a < d, a < h, b < c, b < g, c < e, c < h, d < f, d < g
fn select_235([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_13([e, a, d, h])
    } else {
        select_13([f, b, c, g])
    }
}
/// n = 8, i = 3, b < d, b < g, c < e, c < h, d < h, e < f, e < g
fn select_236([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < d {
        select_234([a, b, c, e, f, d, g])
    } else {
        select_235([b, c, e, d, f, a, h, g])
    }
}
/// n = 10, i = 4, a < d, a < j, b < e, b < h, c < f, c < j, e < j, f < g, f < h, h < i
fn select_237([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_233([g, a, b, e, d, h, i])
    } else {
        select_236([d, b, c, e, f, g, h, j])
    }
}
/// n = 11, i = 4, a < e, a < i, b < d, b < h, c < f, c < h, f < g, f < i, f < k, h < k, i < j
fn select_238([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_232([a, f, b, d, g, h, i, k, j, e])
    } else {
        select_237([b, a, c, d, e, f, g, i, j, h])
    }
}
/// n = 7, i = 2, a < g, b < f, b < g, c < d, c < f, d < e
fn select_239([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_4([b, d, a, e, f])
    } else {
        select_44([a, b, c, f, d, g])
    }
}
/// n = 9, i = 4, a < i, b < e, b < g, c < f, c < g, d < f, d < g, f < i, g < h
fn select_240([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_239([a, f, b, g, h, e, i])
    } else {
        select_106([a, e, c, d, g, h, f])
    }
}
/// n = 11, i = 4, a < g, b < e, b < i, c < f, c < i, c < k, d < f, d < g, d < i, g < h, g < k, i < j
fn select_241([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_240([a, b, c, d, e, f, i, j, g])
    } else {
        select_88([b, c, a, d, e, g, h, i, f, k, j])
    }
}
/// n = 12, i = 4, a < e, a < k, b < g, b < h, c < f, c < h, c < k, d < f, d < g, d < k, g < i, h < j, k < l
fn select_242([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_241([b, a, c, d, e, f, g, i, k, l, h])
    } else {
        select_241([b, a, d, c, e, f, h, j, k, l, g])
    }
}
/// n = 13, i = 5, a < d, a < l, b < e, b < i, b < l, c < f, c < g, e < h, f < j, g < i, g < j, g < l, j < k, l < m
fn select_243([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_238([a, e, c, h, d, g, i, f, l, m, j])
    } else {
        select_242([a, f, b, g, d, i, j, e, k, h, l, m])
    }
}
/// n = 8, i = 5, a < h, b < g, c < g, c < h, d < g, d < h, e < f, f < h
fn select_244([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < b {
        select_84([a, b, c, d, g, h])
    } else {
        select_126([a, c, d, e, f, g])
    }
}
/// n = 9, i = 5, a < i, b < h, b < i, c < g, d < g, d < i, e < f, e < h, f < i, g < h
fn select_245([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_244([b, c, a, d, e, f, g, i])
    } else {
        select_124([b, e, g, a, f, h])
    }
}
/// n = 11, i = 5, a < g, b < h, c < f, c < j, c < k, d < h, d < k, e < g, e < h, g < k, h < i, h < j
fn select_246([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_245([b, d, a, e, c, f, g, k, h])
    } else {
        select_195([a, c, b, d, g, h, i, f, j, k])
    }
}
/// n = 7, i = 3, a < g, b < f, b < g, c < e, c < g, d < e, d < f
fn select_247([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_63([a, c, d, f, e, g])
    } else {
        select_63([a, b, d, e, f, g])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < e, d < g
fn select_248([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_124([a, c, d, e, f, g])
    } else {
        select_210([b, c, d, e, g, f])
    }
}
/// n = 9, i = 5, a < h, a < i, b < e, b < h, c < g, c < h, d < f, d < g, f < i, g < i
fn select_249([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_247([f, a, g, b, e, h, i])
    } else {
        select_248([e, a, c, d, f, h, g])
    }
}
/// n = 12, i = 6, a < f, b < e, b < i, b < l, c < i, c < j, d < g, d < h, f < i, g < j, h < j, j < k, j < l
fn select_250([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_246([a, h, b, c, g, e, f, j, k, l, i])
    } else {
        select_249([c, b, f, d, e, h, g, i, j])
    }
}
/// n = 7, i = 3, a < f, b < e, b < f, b < g, c < d, c < e, d < g
fn select_251([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < d {
        select_209([a, b, c, e, d, f])
    } else {
        select_60([b, d, a, e, g])
    }
}
/// n = 9, i = 3, b < e, b < f, b < i, c < d, c < f, c < g, c < h, d < i, e < g, e < h
fn select_252([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_117([e, c, f, a, d, i, g, h])
    } else {
        select_251([a, b, c, d, f, e, i])
    }
}
/// n = 7, i = 4, a < g, b < f, b < g, c < e, c < g, d < e, d < f
fn select_253([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_20([a, b, d, e, g, f])
    } else {
        select_20([a, c, d, f, g, e])
    }
}
/// n = 9, i = 4, b < f, b < g, c < e, c < g, d < e, d < f, d < h, d < i, g < h, g < i
fn select_254([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_253([a, b, c, d, e, f, g])
    } else {
        select_140([g, d, e, f, a, h, i])
    }
}
/// n = 10, i = 4, b < f, b < j, c < f, c < g, c < h, c < j, d < e, d < g, e < h, e < i, e < j, g < i
fn select_255([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_252([a, e, c, g, b, h, f, j, i])
    } else {
        select_254([a, b, d, c, g, f, e, h, j])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < f, c < g, d < e
fn select_256([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_12([b, c, d])
    } else {
        select_6([a, d, b, f, g, e])
    }
}
/// n = 4, i = 3
fn select_257([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_18([b, c, d])
    } else {
        select_18([a, c, d])
    }
}
/// n = 7, i = 4, a < e, b < f, c < f, d < f, f < g
fn select_258([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_12([a, f, g])
    } else {
        select_257([b, c, d, e])
    }
}
/// n = 7, i = 4, c < f, d < f, e < f, f < g
fn select_259([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_258([a, c, d, e, b, f, g])
    } else {
        select_258([b, c, d, e, a, f, g])
    }
}
/// n = 9, i = 4, b < i, c < g, c < h, d < f, e < f, f < g, f < h, f < i
fn select_260([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_256([a, c, f, b, i, g, h])
    } else {
        select_259([a, c, b, d, e, f, i])
    }
}
/// n = 5, i = 4
fn select_261([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_257([b, c, d, e])
    } else {
        select_257([a, c, d, e])
    }
}
/// n = 8, i = 5, a < f, b < g, c < g, d < g, e < g, g < h
fn select_262([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_12([a, g, h])
    } else {
        select_261([b, c, d, e, f])
    }
}
/// n = 10, i = 5, b < f, b < i, b < j, c < g, d < g, e < g, g < h, g < i, g < j
fn select_263([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_262([b, a, c, d, e, f, g, h])
    } else {
        select_140([b, g, a, h, f, i, j])
    }
}
/// n = 12, i = 5, a < l, b < h, b < k, c < f, c < g, c < j, d < g, d < h, e < g, e < h, g < k, g < l, h < i, h < j
fn select_264([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_260([f, a, b, d, e, g, h, k, l])
    } else {
        select_263([a, c, b, d, e, f, h, i, g, j])
    }
}
/// n = 14, i = 6, a < i, a < n, b < e, b < h, b < m, c < h, c < j, d < f, d < g, f < j, g < j, g < l, g < n, h < i, h < l, h < n, j < k, j < m
fn select_265([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < f {
        select_255([e, a, h, d, g, i, f, l, j, n])
    } else {
        select_264([a, g, b, c, f, e, h, j, k, m, l, i])
    }
}
/// n = 14, i = 6, a < f, a < n, b < e, b < i, b < m, c < i, c < j, d < g, d < h, g < j, h < j, h < l, h < n, i < l, i < n, j < k, j < m
fn select_266([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < i {
        select_250([a, b, c, d, e, f, g, h, i, j, k, m])
    } else {
        select_265([a, b, c, d, e, g, h, i, f, j, k, l, m, n])
    }
}
/// n = 15, i = 6, a < f, a < o, b < e, b < m, c < h, c < k, d < g, d < i, g < k, h < j, h < m, i < j, i < k, i < m, k < l, k < o, m < n
fn select_267([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < a {
        select_243([b, h, d, e, a, g, i, f, j, k, l, m, n])
    } else {
        select_266([b, a, c, d, f, e, g, i, h, k, l, j, o, m])
    }
}
/// n = 15, i = 6, a < f, a < o, b < e, b < m, c < g, c < k, d < h, d < i, g < m, h < k, i < j, i < k, i < m, k < l, k < o, m < n
fn select_268([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < g {
        select_230([a, b, h, c, i, e, f, j, g, k, l, m, o])
    } else {
        select_267([a, b, c, d, e, f, h, g, i, j, k, l, m, n, o])
    }
}
/// n = 8, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_269([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_25([d, c, a, e, g])
    } else {
        select_26([a, b, c, d, e, f, g, h])
    }
}
/// n = 10, i = 3, a < d, a < g, b < e, b < f, c < f, c < g, c < i, e < i, f < h, g < j
fn select_270([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_212([a, c, b, d, e, f, h, i])
    } else {
        select_269([b, a, c, d, g, f, j, h])
    }
}
/// n = 11, i = 3, a < d, a < f, b < e, b < g, c < f, c < g, c < j, e < j, f < k, g < h, g < i
fn select_271([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_207([a, c, b, d, e, g, h, i, j])
    } else {
        select_49([b, a, c, d, f, g, k, h, i])
    }
}
/// n = 12, i = 3, a < g, b < h, c < e, c < f, d < f, d < g, d < h, f < i, g < l, h < j, h < k
fn select_272([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_270([c, a, d, e, b, g, f, l, h, i])
    } else {
        select_271([c, b, d, e, a, f, h, j, k, g, i])
    }
}
/// n = 10, i = 2, a < f, b < c, b < d, b < e, e < g, e < h, f < i, f < j
fn select_273([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_3([b, i, j])
    } else {
        select_9([c, d, a, e, g, h, f])
    }
}
/// n = 9, i = 2, a < d, a < i, b < c, b < i, c < e, c < f, d < g, d < h
fn select_274([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_4([a, c, e, f, i])
    } else {
        select_4([b, d, g, h, i])
    }
}
/// n = 11, i = 2, a < e, b < c, b < d, c < h, c < i, d < f, d < g, e < j, e < k
fn select_275([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < a {
        select_150([d, c, a, h, i, f, g])
    } else {
        select_274([a, b, d, e, f, g, j, k, c])
    }
}
/// n = 9, i = 3, a < d, a < i, b < e, b < i, c < e, c < h, c < i, d < h, e < f, e < g
fn select_276([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_4([a, e, f, g, i])
    } else {
        select_28([b, d, c, h, i])
    }
}
/// n = 11, i = 3, a < f, b < d, b < e, c < e, c < f, c < i, d < i, e < g, e < h, f < j, f < k
fn select_277([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_52([d, e, a, g, h, i])
    } else {
        select_276([b, a, c, d, f, j, k, i, e])
    }
}
/// n = 12, i = 3, a < g, b < d, b < e, b < f, c < e, c < f, c < g, e < j, f < h, f < i, g < k, g < l
fn select_278([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < d {
        select_275([a, b, e, f, g, h, i, d, j, k, l])
    } else {
        select_277([a, b, c, d, f, g, h, i, e, k, l])
    }
}
/// n = 12, i = 3, a < i, b < d, b < h, c < e, c < h, c < i, e < f, e < g, h < j, i < k, i < l
fn select_279([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < b {
        select_273([a, e, f, g, b, i, d, h, k, l])
    } else {
        select_278([a, b, c, d, h, e, i, f, g, j, k, l])
    }
}
/// n = 12, i = 3, b < i, c < e, c < h, d < f, d < h, d < i, f < g, h < j, i < k, i < l
fn select_280([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < f {
        select_272([a, b, c, d, e, h, f, i, j, k, l, g])
    } else {
        select_279([b, c, d, e, f, a, g, h, i, j, k, l])
    }
}
/// n = 12, i = 3, b < i, c < g, c < h, d < e, d < f, d < h, d < i, h < j, i < k, i < l
fn select_281([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_280([a, b, c, d, g, e, f, h, i, j, k, l])
    } else {
        select_280([a, b, c, d, g, f, e, h, i, j, k, l])
    }
}
/// n = 8, i = 2, c < h, d < e, d < f, d < h, f < g
fn select_282([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_233([b, a, d, e, c, f, g])
    } else {
        select_146([b, c, d, e, a, f, g, h])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < i, e < f, e < g, e < h
fn select_283([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_33([b, d, e, f, g, h, a, i])
    } else {
        select_153([a, b, c, e, f, g, h, i, d])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, e < j, f < g, f < h, f < i
fn select_284([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_152([b, d, e, f, g, h, i, a, j])
    } else {
        select_283([b, a, c, d, f, g, h, i, e])
    }
}
/// n = 10, i = 2, d < e, d < f, d < g, f < j, g < h, g < i
fn select_285([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_282([b, c, a, d, e, f, j, g])
    } else {
        select_284([b, c, d, e, f, g, a, h, i, j])
    }
}
/// n = 11, i = 3, b < f, b < g, b < h, c < d, c < e, c < g, c < h, g < k, h < i, h < j
fn select_286([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < h {
        select_148([a, b, c, e, f, d, g, k, h])
    } else {
        select_155([a, b, c, e, f, g, h, d, i, j, k])
    }
}
/// n = 12, i = 3, b < f, b < j, c < d, c < e, c < g, c < j, g < h, g < i, i < k, j < l
fn select_287([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_285([a, d, e, g, h, i, b, f, j, k])
    } else {
        select_286([a, b, c, d, e, f, j, g, h, i, l])
    }
}
/// n = 12, i = 3, b < l, c < g, c < j, d < e, d < f, d < h, d < j, h < i, h < l, j < k
fn select_288([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < h {
        select_281([a, b, c, d, e, f, g, j, h, k, i, l])
    } else {
        select_287([a, c, d, e, f, g, h, i, b, j, l, k])
    }
}
/// n = 9, i = 1, b < i, c < g, c < h, d < e, d < f
fn select_289([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < c {
        select_8([a, b, d, e, f, i, c])
    } else {
        select_151([b, d, c, a, g, h, e, f, i])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < i, d < h, e < f, e < g
fn select_290([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_35([a, d, e, b, f, g, h])
    } else {
        select_256([a, b, c, d, h, e, i])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, d < j, e < i, f < g, f < h
fn select_291([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_289([b, e, d, f, g, h, a, j, i])
    } else {
        select_290([b, a, c, e, f, g, h, i, d])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, c < g, f < j, g < h, g < i
fn select_292([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_291([a, b, c, d, f, g, h, i, j, e])
    } else {
        select_291([a, b, c, e, f, g, h, i, j, d])
    }
}
/// n = 12, i = 5, a < h, b < g, b < h, c < f, c < h, d < e, d < h, h < i, h < j, h < k, h < l
fn select_293([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < c {
        select_124([a, c, d, e, f, h])
    } else {
        select_161([f, a, b, d, e, g, h, i, j, k, l])
    }
}
/// n = 13, i = 5, a < g, a < m, b < h, c < f, c < h, d < e, d < h, h < i, h < j, h < k, h < m, k < l
fn select_294([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_292([e, f, h, i, j, k, a, g, m, l])
    } else {
        select_293([b, a, c, d, e, f, g, h, i, j, k, m])
    }
}
/// n = 8, i = 4, b < g, b < h, c < f, d < e, d < g, d < h, f < g, f < h
fn select_295([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < h {
        select_127([a, b, c, d, e, f, g])
    } else {
        select_127([a, b, c, d, e, f, h])
    }
}
/// n = 7, i = 4, b < f, b < g, c < f, c < g, d < e, e < g
fn select_296([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < g {
        select_126([a, b, c, d, e, f])
    } else {
        select_12([f, e, g])
    }
}
/// n = 8, i = 4, b < f, b < g, b < h, c < f, c < g, c < h, d < e, e < g, e < h
fn select_297([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < g {
        select_296([a, b, c, d, e, f, h])
    } else {
        select_13([f, e, g, h])
    }
}
/// n = 10, i = 5, b < h, b < i, b < j, c < e, d < f, e < h, e < i, e < j, f < g, f < i, f < j
fn select_298([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_295([a, b, c, f, g, e, i, j])
    } else {
        select_297([a, b, e, d, f, h, i, j])
    }
}
/// n = 5, i = 3, b < e, c < e, d < e
fn select_299([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < a {
        select_47([a, c, d, e])
    } else {
        select_18([b, c, d])
    }
}
/// n = 8, i = 5, a < h, b < g, c < g, d < e, e < f, g < h
fn select_300([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_20([a, g, d, e, h, f])
    } else {
        select_299([a, b, c, f, g])
    }
}
/// n = 7, i = 4, a < f, b < f, b < g, c < f, c < g, d < e, e < g
fn select_301([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_19([b, c, d, e, f])
    } else {
        select_47([a, b, c, g])
    }
}
/// n = 8, i = 5, a < h, b < g, b < h, c < g, c < h, d < e, e < f
fn select_302([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_301([a, b, c, e, f, h, g])
    } else {
        select_22([g, a, d, h, e, f])
    }
}
/// n = 9, i = 6, a < i, b < f, c < g, d < e, d < i, f < i, g < h
fn select_303([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_300([a, b, e, c, g, h, f, i])
    } else {
        select_302([a, d, f, c, g, h, e, i])
    }
}
/// n = 11, i = 6, a < i, b < f, b < j, b < k, c < g, d < e, d < i, g < h, g < j, g < k, i < j, i < k
fn select_304([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < f {
        select_298([e, b, a, c, i, g, h, f, j, k])
    } else {
        select_303([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 3, a < e, a < j, b < f, b < j, c < d, c < f, c < j, f < g, f < h, f < i
fn select_305([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_124([b, a, c, d, e, j])
    } else {
        select_39([d, a, b, f, g, h, i, j])
    }
}
/// n = 11, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_306([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_54([d, f, g, h, i, j, a, k])
    } else {
        select_305([a, b, c, d, e, g, h, i, j, f])
    }
}
/// n = 10, i = 4, a < f, a < j, b < f, b < h, b < j, c < d, d < e, d < h, d < j, f < g, h < i
fn select_307([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_12([g, c, d])
    } else {
        select_256([e, a, b, h, i, f, j])
    }
}
/// n = 11, i = 5, a < e, a < i, b < f, c < d, c < i, c < k, d < h, e < k, f < g, f < i, f < k, i < j
fn select_308([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_78([d, a, b, h, f, g, e, i, j])
    } else {
        select_307([e, c, b, f, g, d, h, i, j, k])
    }
}
/// n = 13, i = 5, a < d, a < i, a < j, b < f, c < e, c < i, c < j, e < h, f < g, f < i, f < j, i < m, j < k, j < l
fn select_309([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < d {
        select_306([e, a, f, g, h, i, j, d, k, l, m])
    } else {
        select_308([a, b, c, e, d, f, g, h, i, m, j])
    }
}
/// n = 14, i = 6, a < f, b < g, c < e, c < j, c < k, d < h, g < j, g < k, h < i, h < j, h < k, j < n, k < l, k < m
fn select_310([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < g {
        select_304([b, c, d, a, f, e, h, i, g, j, k])
    } else {
        select_309([c, d, g, e, a, h, i, f, j, k, l, m, n])
    }
}
/// n = 15, i = 6, a < f, a < o, b < g, b < j, c < e, c < j, c < m, d < h, g < m, h < i, h < j, h < m, j < k, j < l, j < o, m < n
fn select_311([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < g {
        select_294([a, b, c, h, i, e, f, j, k, l, g, m, o])
    } else {
        select_310([a, b, c, d, e, f, g, h, i, m, j, k, l, n])
    }
}
/// n = 16, i = 6, a < f, a < n, b < e, b < m, c < g, c < j, d < h, g < n, h < i, h < j, h < n, j < k, j < l, j < m, m < o, n < p
fn select_312([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < a {
        select_288([i, g, b, j, k, l, e, a, f, m, o, n])
    } else {
        select_311([b, c, a, d, f, e, g, h, i, j, k, l, n, p, m])
    }
}
/// n = 16, i = 6, a < g, a < n, b < f, b < m, c < h, c < k, d < e, d < i, h < n, i < j, i < k, i < n, k < l, k < m, m < o, n < p
fn select_313([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if e < k {
        select_268([b, a, c, d, g, f, h, e, i, j, k, l, n, p, m])
    } else {
        select_312([a, b, c, d, f, g, h, i, j, k, e, l, m, n, o, p])
    }
}
/// n = 16, i = 6, a < h, a < o, b < g, b < k, c < f, c < l, d < e, d < i, i < j, i < k, i < l, k < n, l < m, l < o, o < p
fn select_314([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < f {
        select_187([e, a, b, c, i, j, g, h, k, l, m, n, f, o, p])
    } else {
        select_313([b, a, c, d, e, h, g, f, i, j, l, m, o, k, p, n])
    }
}
/// n = 16, i = 6, a < h, a < n, b < g, b < m, c < f, c < i, d < e, d < j, i < l, i < n, j < k, j < m, m < o, n < p
fn select_315([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_314([b, a, d, c, f, e, h, g, i, l, n, j, k, p, m, o])
    } else {
        select_314([a, b, c, d, e, f, g, h, j, k, m, i, l, o, n, p])
    }
}
/// n = 16, i = 6, a < h, a < o, b < g, b < j, c < f, c < i, d < e, d < k, i < m, j < n, k < l, k < o, o < p
fn select_316([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_315([a, b, d, c, f, e, g, h, k, i, m, l, j, o, n, p])
    } else {
        select_315([a, c, d, b, g, e, f, h, k, j, n, l, i, o, m, p])
    }
}
/// n = 16, i = 6, a < j, b < i, c < h, c < o, d < g, d < k, e < f, e < l, k < n, l < m, l < o, o < p
fn select_317([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_316([c, a, d, e, f, g, j, h, k, b, l, m, n, i, o, p])
    } else {
        select_316([c, b, d, e, f, g, i, h, k, a, l, m, n, j, o, p])
    }
}
/// n = 16, i = 6, c < j, d < i, d < o, e < h, e < k, f < g, f < l, k < n, l < m, l < o, o < p
fn select_318([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_317([a, c, d, e, f, g, h, i, j, b, k, l, m, n, o, p])
    } else {
        select_317([b, c, d, e, f, g, h, i, j, a, k, l, m, n, o, p])
    }
}
/// n = 16, i = 6, c < l, d < k, e < j, f < i, f < o, g < h, g < m, m < n, m < o, o < p
fn select_319([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if c < d {
        select_318([a, b, e, f, c, g, h, l, i, j, d, m, n, k, o, p])
    } else {
        select_318([a, b, e, f, d, g, h, k, i, j, c, m, n, l, o, p])
    }
}
/// n = 16, i = 6, e < l, f < k, g < j, g < o, h < i, h < m, m < n, m < o, o < p
fn select_320([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_319([c, d, a, e, f, g, h, i, j, k, l, b, m, n, o, p])
    } else {
        select_319([c, d, b, e, f, g, h, i, j, k, l, a, m, n, o, p])
    }
}
/// n = 16, i = 6, e < l, f < k, g < j, g < n, h < i, h < m, m < o, n < p
fn select_321([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if m < n {
        select_320([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n, p])
    } else {
        select_320([a, b, c, d, e, f, h, g, j, i, k, l, n, p, m, o])
    }
}
/// n = 16, i = 6, g < l, h < k, h < n, i < j, i < m, m < o, n < p
fn select_322([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_321([c, d, e, f, a, g, h, i, j, k, l, b, m, n, o, p])
    } else {
        select_321([c, d, e, f, b, g, h, i, j, k, l, a, m, n, o, p])
    }
}
/// n = 16, i = 6, i < l, i < n, j < k, j < m, m < o, n < p
fn select_323([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_322([c, d, e, f, g, h, a, i, j, k, l, b, m, n, o, p])
    } else {
        select_322([c, d, e, f, g, h, b, i, j, k, l, a, m, n, o, p])
    }
}
/// n = 16, i = 6, i < n, j < m, k < l, k < o, o < p
fn select_324([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_323([a, b, c, d, e, f, g, h, i, k, l, n, o, j, p, m])
    } else {
        select_323([a, b, c, d, e, f, g, h, j, k, l, m, o, i, p, n])
    }
}
/// n = 16, i = 6, k < n, l < m, l < o, o < p
fn select_325([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_324([c, d, e, f, g, h, i, j, a, k, l, m, n, b, o, p])
    } else {
        select_324([c, d, e, f, g, h, i, j, b, k, l, m, n, a, o, p])
    }
}
/// n = 16, i = 6, m < n, m < o, o < p
fn select_326([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_325([c, d, e, f, g, h, i, j, k, l, a, m, n, b, o, p])
    } else {
        select_325([c, d, e, f, g, h, i, j, k, l, b, m, n, a, o, p])
    }
}
/// n = 16, i = 6, m < p, n < o
fn select_327([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if m < n {
        select_326([a, b, c, d, e, f, g, h, i, j, k, l, m, p, n, o])
    } else {
        select_326([a, b, c, d, e, f, g, h, i, j, k, l, n, o, m, p])
    }
}
/// n = 16, i = 6, o < p
fn select_328([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_327([c, d, e, f, g, h, i, j, k, l, m, n, a, o, p, b])
    } else {
        select_327([c, d, e, f, g, h, i, j, k, l, m, n, b, o, p, a])
    }
}
/// n = 16, i = 6
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_328([c, d, e, f, g, h, i, j, k, l, m, n, o, p, a, b])
    } else {
        select_328([c, d, e, f, g, h, i, j, k, l, m, n, o, p, b, a])
    }
}
