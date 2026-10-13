#[allow(unused)]
pub const N: usize = 16;
#[allow(unused)]
pub const I: usize = 7;

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
/// n = 3, i = 1, b < c
fn select_3([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_2([a, c])
    }
}
/// n = 5, i = 2, a < d, a < e, b < c, b < e
fn select_4([a, b, c, d, e]: [usize; 5]) -> usize {
    if c < d {
        select_3([c, a, e])
    } else {
        select_3([d, b, e])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < e, c < f
fn select_5([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_4([a, c, d, b, f])
    } else {
        select_4([b, c, d, a, e])
    }
}
/// n = 2, i = 1
fn select_6([a, b]: [usize; 2]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_1([a])
    }
}
/// n = 3, i = 2
fn select_7([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_6([b, c])
    } else {
        select_6([a, c])
    }
}
/// n = 4, i = 3
fn select_8([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_7([b, c, d])
    } else {
        select_7([a, c, d])
    }
}
/// n = 5, i = 3, b < e, c < e, d < e
fn select_9([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < e {
        select_8([a, b, c, d])
    } else {
        select_1([e])
    }
}
/// n = 8, i = 4, a < h, b < e, c < f, d < f, f < g, f < h
fn select_10([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_5([a, b, f, g, e, h])
    } else {
        select_9([a, c, d, e, f])
    }
}
/// n = 5, i = 2, a < e, b < d, c < d, c < e
fn select_11([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_3([b, c, e])
    } else {
        select_3([a, c, d])
    }
}
/// n = 7, i = 4, a < g, b < e, c < f, d < f, f < g
fn select_12([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_11([a, b, f, e, g])
    } else {
        select_9([a, c, d, e, f])
    }
}
/// n = 4, i = 2, a < d, b < c
fn select_13([a, b, c, d]: [usize; 4]) -> usize {
    if c < d {
        select_6([a, c])
    } else {
        select_6([b, d])
    }
}
/// n = 6, i = 3, a < e, a < f, b < d, b < f, c < d, c < e
fn select_14([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_13([a, c, d, f])
    } else {
        select_13([b, c, e, f])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < e
fn select_15([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_14([a, b, c, e, d, f])
    } else {
        select_13([b, d, a, e])
    }
}
/// n = 6, i = 3, a < f, b < e, c < d, c < f
fn select_16([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_15([b, a, c, d, f, e])
    } else {
        select_13([c, e, a, d])
    }
}
/// n = 7, i = 4, b < g, c < e, d < f, f < g
fn select_17([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_12([b, c, a, d, e, f, g])
    } else {
        select_16([b, c, f, a, e, g])
    }
}
/// n = 8, i = 4, a < h, b < f, c < g, d < e, d < g, g < h
fn select_18([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_10([a, b, c, d, f, g, e, h])
    } else {
        select_17([a, c, b, d, f, e, g])
    }
}
/// n = 4, i = 1, b < c, b < d
fn select_19([a, b, c, d]: [usize; 4]) -> usize {
    if c < a {
        select_2([c, d])
    } else {
        select_3([a, b, d])
    }
}
/// n = 4, i = 1, a < d, b < c
fn select_20([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_2([b, d])
    } else {
        select_2([a, c])
    }
}
/// n = 6, i = 2, a < e, a < f, b < d, b < f, c < d, c < e
fn select_21([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_20([b, c, e, f])
    } else {
        select_20([a, c, d, f])
    }
}
/// n = 6, i = 2, a < f, b < e, b < f, c < d, c < e
fn select_22([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_20([b, d, a, e])
    } else {
        select_21([a, b, c, e, d, f])
    }
}
/// n = 6, i = 2, a < f, b < e, c < d, c < f
fn select_23([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < b {
        select_20([a, d, b, f])
    } else {
        select_22([a, c, b, e, d, f])
    }
}
/// n = 7, i = 2, a < e, b < f, b < g, c < d, c < f, c < g
fn select_24([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_19([d, b, f, g])
    } else {
        select_23([b, a, c, d, e, g])
    }
}
/// n = 5, i = 2, a < c, b < d, d < e
fn select_25([a, b, c, d, e]: [usize; 5]) -> usize {
    if e < a {
        select_1([e])
    } else {
        select_13([a, b, d, c])
    }
}
/// n = 6, i = 3, a < d, b < e, c < e, e < f
fn select_26([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < d {
        select_25([a, c, d, e, f])
    } else {
        select_6([b, c])
    }
}
/// n = 3, i = 0
fn select_27([a, b, c]: [usize; 3]) -> usize {
    if a < b {
        select_2([a, c])
    } else {
        select_2([b, c])
    }
}
/// n = 5, i = 1, a < e, b < c, b < d
fn select_28([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_2([b, e])
    } else {
        select_27([a, c, d])
    }
}
/// n = 7, i = 2, a < c, a < f, a < g, b < d, b < f, b < g, d < e
fn select_29([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_19([c, b, f, g])
    } else {
        select_28([d, a, f, g, e])
    }
}
/// n = 8, i = 3, a < h, b < d, b < f, b < g, c < e, e < f, e < g, e < h
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_26([b, a, c, d, e, h])
    } else {
        select_29([b, e, d, a, h, f, g])
    }
}
/// n = 8, i = 3, a < f, b < d, b < g, b < h, c < e, c < f, e < g, e < h
fn select_31([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_26([b, a, c, d, f, e])
    } else {
        select_30([a, b, c, d, e, g, h, f])
    }
}
/// n = 8, i = 3, b < e, b < g, b < h, c < d, c < f, f < g, f < h
fn select_32([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_24([d, f, b, e, a, g, h])
    } else {
        select_31([a, b, c, e, f, d, g, h])
    }
}
/// n = 4, i = 0
fn select_33([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_27([a, c, d])
    } else {
        select_27([b, c, d])
    }
}
/// n = 5, i = 1, b < c, b < d, b < e
fn select_34([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_1([b])
    } else {
        select_33([a, c, d, e])
    }
}
/// n = 6, i = 2, a < d, a < e, a < f, b < c, b < e, b < f
fn select_35([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_19([c, a, e, f])
    } else {
        select_19([d, b, e, f])
    }
}
/// n = 8, i = 2, a < d, a < h, b < c, b < e, b < h, e < f, e < g
fn select_36([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_34([c, e, a, f, g])
    } else {
        select_35([a, b, c, d, e, h])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, b < g, c < d, c < e, c < f
fn select_37([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_22([a, b, c, d, e, g])
    } else {
        select_22([a, b, c, d, f, g])
    }
}
/// n = 8, i = 3, a < h, b < d, b < g, b < h, c < e, c < h, e < f, e < g
fn select_38([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_11([a, d, c, e, h])
    } else {
        select_37([a, b, e, f, d, g, h])
    }
}
/// n = 9, i = 3, b < d, b < g, b < i, c < e, c < g, e < f, e < i, g < h
fn select_39([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_36([e, b, d, f, g, a, h, i])
    } else {
        select_38([a, b, c, d, e, f, i, g])
    }
}
/// n = 9, i = 3, b < f, b < i, c < g, c < i, d < e, d < g, d < i, g < h
fn select_40([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_32([a, d, b, f, e, c, g, i])
    } else {
        select_39([a, d, c, e, b, f, g, h, i])
    }
}
/// n = 10, i = 4, b < f, b < j, c < h, c < j, d < e, d < g, g < h, g < j, h < i
fn select_41([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_18([c, b, a, d, e, f, g, h])
    } else {
        select_40([e, b, c, g, a, f, h, i, j])
    }
}
/// n = 6, i = 2, a < c, a < f, b < d, b < f, d < e
fn select_42([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if f < c {
        select_20([a, d, e, f])
    } else {
        select_25([a, b, c, d, e])
    }
}
/// n = 6, i = 1, a < e, a < f, b < c, b < d
fn select_43([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_27([b, e, f])
    } else {
        select_27([a, c, d])
    }
}
/// n = 7, i = 2, a < c, a < g, b < d, b < g, d < e, d < f
fn select_44([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < b {
        select_1([b])
    } else {
        select_43([a, d, e, f, c, g])
    }
}
/// n = 7, i = 2, a < d, a < g, b < c, b < e, b < g, e < f
fn select_45([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_42([a, b, d, c, e, g])
    } else {
        select_44([a, b, d, e, c, f, g])
    }
}
/// n = 6, i = 2, a < c, b < d, d < e, d < f
fn select_46([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_2([e, f])
    } else {
        select_25([a, b, c, d, f])
    }
}
/// n = 6, i = 2, c < d, d < e, d < f
fn select_47([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_46([a, c, b, d, e, f])
    } else {
        select_46([b, c, a, d, e, f])
    }
}
/// n = 7, i = 2, b < g, c < d, c < g, d < e, d < f
fn select_48([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_46([a, c, b, d, e, f])
    } else {
        select_44([b, c, a, d, e, f, g])
    }
}
/// n = 9, i = 4, a < i, b < e, c < d, c < i, d < g, d < h, e < f, e < h, e < i
fn select_49([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_47([a, g, b, e, f, h])
    } else {
        select_48([f, a, c, d, g, h, i])
    }
}
/// n = 9, i = 4, b < e, c < d, c < h, d < g, d < i, e < f, e < h, e < i
fn select_50([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < a {
        select_45([d, e, f, g, h, a, i])
    } else {
        select_49([a, b, c, d, e, f, g, i, h])
    }
}
/// n = 6, i = 2, a < d, b < c, b < e, e < f
fn select_51([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_19([c, e, a, f])
    } else {
        select_4([a, b, c, d, e])
    }
}
/// n = 6, i = 3, a < e, b < e, b < f, c < d, d < f
fn select_52([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < a {
        select_25([a, c, e, d, f])
    } else {
        select_13([b, c, d, e])
    }
}
/// n = 6, i = 3, b < e, c < d, c < f, e < f
fn select_53([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_52([a, c, b, e, d, f])
    } else {
        select_25([d, b, a, e, f])
    }
}
/// n = 8, i = 4, a < e, b < f, c < d, c < g, f < g, g < h
fn select_54([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_51([a, c, d, e, g, h])
    } else {
        select_53([e, b, c, d, f, g])
    }
}
/// n = 8, i = 4, a < f, b < g, c < g, d < e, d < g, g < h
fn select_55([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_54([a, b, d, e, f, c, g, h])
    } else {
        select_54([a, c, d, e, f, b, g, h])
    }
}
/// n = 10, i = 5, a < h, b < e, b < h, c < d, c < f, f < g, f < j, h < i, h < j
fn select_56([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_50([e, a, c, f, h, i, g, d, j])
    } else {
        select_55([f, a, d, b, e, g, h, i])
    }
}
/// n = 8, i = 3, b < e, b < f, c < d, c < f, f < g, f < h
fn select_57([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_48([a, d, b, f, g, h, e])
    } else {
        select_48([a, e, c, f, g, h, d])
    }
}
/// n = 6, i = 2, b < f, c < d, c < f, d < e
fn select_58([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_25([a, c, b, d, e])
    } else {
        select_42([b, c, a, d, e, f])
    }
}
/// n = 7, i = 2, b < g, c < d, c < e, c < g, e < f
fn select_59([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_58([a, b, c, d, e, g])
    } else {
        select_48([a, b, c, e, d, f, g])
    }
}
/// n = 8, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, g < h
fn select_60([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_59([a, d, b, f, g, h, e])
    } else {
        select_59([a, e, c, f, g, h, d])
    }
}
/// n = 9, i = 4, a < d, a < f, b < c, b < e, b < i, e < h, f < g, f < i
fn select_61([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_57([c, a, e, h, d, f, g, i])
    } else {
        select_60([d, b, f, g, c, i, e, h])
    }
}
/// n = 10, i = 5, a < e, b < g, c < d, c < f, f < h, f < j, g < i, g < j
fn select_62([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_56([b, a, c, d, e, f, h, g, i, j])
    } else {
        select_61([c, g, i, d, a, f, h, e, j])
    }
}
/// n = 11, i = 5, a < e, a < k, b < f, b < k, c < d, c < g, f < i, g < h, g < j, j < k
fn select_63([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if j < f {
        select_41([d, a, b, g, h, e, j, f, i, k])
    } else {
        select_62([a, b, c, d, e, g, f, h, i, j])
    }
}
/// n = 7, i = 2, a < d, b < c, b < e, e < f, e < g
fn select_64([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_34([c, e, a, f, g])
    } else {
        select_4([a, b, c, d, e])
    }
}
/// n = 7, i = 2, c < d, c < e, e < f, e < g
fn select_65([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_64([a, c, d, b, e, f, g])
    } else {
        select_64([b, c, d, a, e, f, g])
    }
}
/// n = 8, i = 3, b < e, b < h, c < d, c < f, f < g, f < h
fn select_66([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < b {
        select_65([a, d, f, g, b, e, h])
    } else {
        select_57([a, b, c, d, e, f, g, h])
    }
}
/// n = 6, i = 1, a < f, b < c, b < d, b < e
fn select_67([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_2([b, f])
    } else {
        select_33([a, c, d, e])
    }
}
/// n = 7, i = 1, a < g, b < c, b < d, b < e, b < f
fn select_68([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < a {
        select_33([c, d, e, f])
    } else {
        select_67([a, b, d, e, f, g])
    }
}
/// n = 7, i = 2, a < c, b < d, d < e, d < f, d < g
fn select_69([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_33([a, e, f, g])
    } else {
        select_3([c, b, d])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < h, d < e, d < f, d < g
fn select_70([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_33([a, e, f, g])
    } else {
        select_19([c, b, d, h])
    }
}
/// n = 8, i = 2, b < h, c < d, c < h, d < e, d < f, d < g
fn select_71([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_69([a, c, b, d, e, f, g])
    } else {
        select_70([b, c, a, d, e, f, g, h])
    }
}
/// n = 9, i = 2, b < i, c < d, c < i, d < e, d < f, d < g, d < h
fn select_72([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_68([b, d, e, f, g, h, i])
    } else {
        select_71([a, b, c, d, f, g, h, i])
    }
}
/// n = 10, i = 3, b < e, b < f, c < d, c < f, f < g, f < h, f < i, f < j
fn select_73([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_72([a, d, b, f, g, h, i, j, e])
    } else {
        select_72([a, e, c, f, g, h, i, j, d])
    }
}
/// n = 11, i = 3, b < e, b < h, c < d, c < f, c < h, f < g, h < i, h < j, h < k
fn select_74([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_66([a, b, c, d, e, f, g, h])
    } else {
        select_73([a, b, c, d, e, h, f, i, j, k])
    }
}
/// n = 6, i = 3, a < e, a < f, b < e, b < f, c < d, c < f
fn select_75([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_4([b, c, d, e, f])
    } else {
        select_6([a, b])
    }
}
/// n = 8, i = 3, a < e, b < e, b < f, c < d, c < f, f < g, f < h
fn select_76([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_34([d, f, a, g, h])
    } else {
        select_75([a, b, c, d, e, f])
    }
}
/// n = 8, i = 3, a < g, b < d, b < h, c < e, c < g, e < f, e < h
fn select_77([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_64([a, e, f, g, b, d, h])
    } else {
        select_76([a, c, b, d, g, e, f, h])
    }
}
/// n = 9, i = 4, a < f, a < i, b < e, b < g, c < d, c < g, g < h, g < i
fn select_78([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_77([d, a, b, f, g, h, e, i])
    } else {
        select_77([e, a, c, f, g, h, d, i])
    }
}
/// n = 12, i = 4, a < f, b < e, b < i, c < d, c < g, g < h, g < i, i < j, i < k, i < l
fn select_79([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_74([d, b, g, h, e, a, f, i, j, k, l])
    } else {
        select_78([b, a, c, d, f, e, g, h, i])
    }
}
/// n = 8, i = 3, a < e, a < h, b < d, b < h, c < e, c < g, c < h, d < g, e < f
fn select_80([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_20([b, e, f, h])
    } else {
        select_11([a, d, c, g, h])
    }
}
/// n = 9, i = 4, a < g, a < i, b < d, b < i, c < e, c < g, d < f, e < h, e < i, g < h
fn select_81([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_15([a, c, d, f, e, g])
    } else {
        select_80([b, a, e, g, d, f, h, i])
    }
}
/// n = 5, i = 2, b < e, c < d, c < e
fn select_82([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_13([a, c, d, b])
    } else {
        select_4([b, c, d, a, e])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < i, c < h, d < e, d < f, d < g
fn select_83([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_33([a, e, f, g])
    } else {
        select_28([c, b, d, i, h])
    }
}
/// n = 11, i = 3, a < c, a < g, b < d, b < g, c < e, d < f, f < h, g < i, g < j, g < k
fn select_84([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_70([c, b, e, g, i, j, k, d])
    } else {
        select_83([d, a, f, g, i, j, k, h, c])
    }
}
/// n = 12, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < f, e < g, g < i, h < j, h < k, h < l
fn select_85([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < a {
        select_82([f, a, c, e, h])
    } else {
        select_84([b, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 12, i = 4, a < g, b < h, b < l, c < g, c < h, d < e, d < h, e < f, g < l, h < i, h < j, h < k
fn select_86([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_81([c, d, a, e, b, f, g, l, h])
    } else {
        select_85([c, d, b, e, a, f, g, h, l, i, j, k])
    }
}
/// n = 4, i = 2, b < d, c < d
fn select_87([a, b, c, d]: [usize; 4]) -> usize {
    if b < a {
        select_3([a, c, d])
    } else {
        select_6([b, c])
    }
}
/// n = 5, i = 2, c < d, d < e
fn select_88([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < d {
        select_87([b, a, c, d])
    } else {
        select_19([b, d, a, e])
    }
}
/// n = 8, i = 2, a < c, b < d, c < h, d < e, d < f, d < g
fn select_89([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_33([a, e, f, g])
    } else {
        select_20([b, c, h, d])
    }
}
/// n = 10, i = 3, a < d, a < e, b < f, c < e, c < f, e < j, f < g, f < h, f < i
fn select_90([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_88([b, d, c, e, j])
    } else {
        select_89([a, b, e, f, g, h, i, j])
    }
}
/// n = 6, i = 2, b < d, c < e, e < f
fn select_91([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < c {
        select_25([b, a, d, c, e])
    } else {
        select_51([b, c, a, d, e, f])
    }
}
/// n = 10, i = 3, a < d, a < e, b < d, b < f, c < e, c < f, e < j, f < g, f < h, f < i
fn select_92([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_33([a, g, h, i])
    } else {
        select_42([b, c, d, e, j, f])
    }
}
/// n = 11, i = 3, a < d, a < e, a < f, b < e, b < g, c < f, c < g, f < k, g < h, g < i, g < j
fn select_93([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < g {
        select_91([d, b, c, e, f, k])
    } else {
        select_92([a, b, c, e, f, g, h, i, j, k])
    }
}
/// n = 12, i = 4, a < h, a < i, b < d, b < i, c < e, c < i, d < g, e < f, e < h, i < j, i < k, i < l
fn select_94([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_90([d, a, c, g, e, i, j, k, l, f])
    } else {
        select_93([e, a, b, f, h, d, i, j, k, l, g])
    }
}
/// n = 12, i = 4, b < h, b < i, c < e, c < i, d < f, d < i, e < h, f < g, i < j, i < k, i < l
fn select_95([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < e {
        select_86([a, b, c, d, f, g, e, i, j, k, l, h])
    } else {
        select_94([b, d, c, f, e, a, g, h, i, j, k, l])
    }
}
/// n = 11, i = 4, a < d, a < f, b < f, b < g, c < e, c < k, e < g, f < k, g < h, g < i, g < j
fn select_96([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < g {
        select_53([d, b, c, e, f, k])
    } else {
        select_89([a, e, f, g, h, i, j, k])
    }
}
/// n = 8, i = 3, a < d, a < h, b < c, b < g, c < e, c < h, d < f, d < g
fn select_97([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_19([e, a, d, h])
    } else {
        select_19([f, b, c, g])
    }
}
/// n = 8, i = 3, a < d, a < h, b < c, b < e, d < g, e < f, e < h
fn select_98([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_22([c, a, e, f, h, d])
    } else {
        select_97([a, b, e, d, f, g, c, h])
    }
}
/// n = 12, i = 4, a < e, a < f, a < g, b < f, b < i, c < d, c < e, d < g, d < i, f < h, i < j, i < k, i < l
fn select_99([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < a {
        select_33([a, j, k, l])
    } else {
        select_98([b, c, e, f, d, g, h, i])
    }
}
/// n = 13, i = 5, a < d, a < j, b < f, b < h, c < e, c < j, d < g, e < h, e < i, f < i, f < j, j < k, j < l, j < m
fn select_100([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < e {
        select_96([d, c, b, g, f, e, j, k, l, m, h])
    } else {
        select_99([e, a, b, f, h, d, i, g, j, k, l, m])
    }
}
/// n = 13, i = 5, a < e, a < j, b < f, b < j, c < d, c < g, e < i, f < h, g < i, g < j, j < k, j < l, j < m
fn select_101([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < e {
        select_86([d, g, a, b, f, h, e, j, k, l, m, i])
    } else {
        select_100([b, c, a, f, e, g, h, d, i, j, k, l, m])
    }
}
/// n = 13, i = 5, a < e, a < j, b < f, b < j, c < d, c < g, f < i, g < h, g < j, j < k, j < l, j < m
fn select_102([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_95([d, a, g, b, h, f, i, e, j, k, l, m])
    } else {
        select_101([a, b, c, d, e, f, g, i, h, j, k, l, m])
    }
}
/// n = 13, i = 5, a < e, a < j, b < d, b < g, c < f, c < j, f < h, g < i, g < l, g < m, j < k, j < l, j < m
fn select_103([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < g {
        select_79([f, b, a, e, d, h, j, k, g, i, l, m])
    } else {
        select_102([a, c, b, d, e, f, g, i, h, j, k, l, m])
    }
}
/// n = 13, i = 5, a < e, a < k, b < f, b < k, c < d, c < g, f < j, g < h, g < i, g < m, k < l, k < m
fn select_104([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < k {
        select_63([a, b, c, d, e, f, g, i, j, h, k])
    } else {
        select_103([a, c, b, d, e, f, g, j, i, k, l, h, m])
    }
}
/// n = 6, i = 2, a < e, a < f, b < c, b < d, b < f
fn select_105([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_42([a, b, e, c, d, f])
    } else {
        select_42([a, b, e, d, c, f])
    }
}
/// n = 7, i = 3, b < d, b < g, c < e, e < f, e < g
fn select_106([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_105([b, e, a, f, d, g])
    } else {
        select_26([b, a, c, d, e, f])
    }
}
/// n = 9, i = 3, b < d, b < i, c < e, e < f, e < g, e < i, g < h
fn select_107([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_59([a, b, e, f, g, h, i])
    } else {
        select_106([a, b, c, d, e, f, i])
    }
}
/// n = 7, i = 3, a < f, a < g, b < d, b < g, c < e, c < g, e < f
fn select_108([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < a {
        select_22([e, a, b, d, g, f])
    } else {
        select_42([b, c, d, e, f, g])
    }
}
/// n = 8, i = 3, a < d, b < f, b < h, c < e, c < f, e < h, f < g
fn select_109([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_28([e, f, a, g, h])
    } else {
        select_108([b, a, c, d, e, h, f])
    }
}
/// n = 7, i = 2, a < c, b < d, b < e, d < g, e < f
fn select_110([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_46([a, b, c, d, e, g])
    } else {
        select_46([a, b, c, e, d, f])
    }
}
/// n = 9, i = 4, a < d, b < f, c < e, c < g, e < i, f < g, f < i, g < h
fn select_111([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_53([d, b, c, e, f, g])
    } else {
        select_110([a, c, d, e, g, h, i])
    }
}
/// n = 9, i = 4, a < e, b < d, b < h, c < f, f < g, f < h, h < i
fn select_112([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_109([a, b, f, e, g, h, i, d])
    } else {
        select_111([a, c, b, e, d, f, h, i, g])
    }
}
/// n = 10, i = 4, a < e, a < j, b < d, b < h, c < f, c < h, f < g, h < i, h < j
fn select_113([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_107([d, a, c, e, h, i, f, g, j])
    } else {
        select_112([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, a < g, a < j, b < f, b < h, c < d, c < e, c < h, h < i, h < j
fn select_114([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_113([a, b, c, f, g, d, e, h, i, j])
    } else {
        select_113([a, b, c, f, g, e, d, h, i, j])
    }
}
/// n = 8, i = 2, a < c, b < d, b < e, d < h, e < f, e < g
fn select_115([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_43([d, e, f, g, a, h])
    } else {
        select_44([a, b, c, e, f, g, d])
    }
}
/// n = 8, i = 2, a < e, b < c, b < d, b < f, f < g, f < h
fn select_116([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_115([a, b, e, c, f, g, h, d])
    } else {
        select_115([a, b, e, d, f, g, h, c])
    }
}
/// n = 8, i = 2, c < d, c < e, c < f, f < g, f < h
fn select_117([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_116([a, c, d, e, b, f, g, h])
    } else {
        select_116([b, c, d, e, a, f, g, h])
    }
}
/// n = 8, i = 2, b < d, c < e, e < f, e < g, e < h
fn select_118([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < c {
        select_3([a, d, c])
    } else {
        select_71([a, b, c, e, f, g, h, d])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_119([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_71([a, d, b, f, g, h, i, e])
    } else {
        select_71([a, e, c, f, g, h, i, d])
    }
}
/// n = 10, i = 3, b < d, b < g, c < e, c < g, e < f, g < h, g < i, g < j
fn select_120([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_118([a, e, b, f, g, h, i, j])
    } else {
        select_119([a, b, c, e, d, g, h, i, j])
    }
}
/// n = 10, i = 3, b < e, b < j, c < d, c < f, e < i, f < g, f < h, f < j
fn select_121([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_117([a, d, f, g, h, b, e, j])
    } else {
        select_120([a, c, b, d, e, i, f, g, h, j])
    }
}
/// n = 5, i = 1, b < e, c < d
fn select_122([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < c {
        select_19([a, b, c, e])
    } else {
        select_19([a, c, b, d])
    }
}
/// n = 7, i = 2, a < g, b < e, c < d, c < g, e < f
fn select_123([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_122([a, c, e, f, d])
    } else {
        select_122([d, a, b, e, g])
    }
}
/// n = 6, i = 3, a < f, b < d, c < e, e < f
fn select_124([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < d {
        select_11([a, b, e, d, f])
    } else {
        select_87([a, c, d, e])
    }
}
/// n = 8, i = 4, a < f, b < e, b < h, c < f, c < g, d < e, d < g, g < h
fn select_125([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_124([b, a, d, f, g, h])
    } else {
        select_53([a, c, b, e, g, h])
    }
}
/// n = 7, i = 3, a < d, a < g, b < e, b < g, c < e, c < g, e < f
fn select_126([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_20([a, e, f, g])
    } else {
        select_87([d, b, c, g])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < g, c < d, c < f, c < g
fn select_127([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < f {
        select_108([a, b, c, e, d, f, g])
    } else {
        select_126([b, a, c, e, f, d, g])
    }
}
/// n = 9, i = 4, a < i, b < f, b < h, c < f, c < g, c < h, d < e, d < g, e < h, g < i
fn select_128([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_125([b, a, c, d, e, f, g, i])
    } else {
        select_127([b, e, c, g, a, f, h])
    }
}
/// n = 9, i = 4, b < f, b < i, c < f, c < g, c < i, d < e, d < g, e < i, g < h
fn select_129([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < a {
        select_123([e, g, b, f, h, a, i])
    } else {
        select_128([a, b, c, d, e, f, g, i, h])
    }
}
/// n = 11, i = 4, b < e, b < g, c < f, c < h, d < f, d < g, d < h, g < k, h < i, h < j
fn select_130([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_121([a, b, c, f, g, h, i, j, k, e])
    } else {
        select_129([a, c, d, b, e, f, g, k, h])
    }
}
/// n = 12, i = 5, a < e, a < k, b < f, b < j, c < d, c < g, f < h, f < i, g < j, g < k, k < l
fn select_131([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_114([a, c, f, h, i, d, e, g, j, k])
    } else {
        select_130([d, a, b, g, e, j, k, f, h, i, l])
    }
}
/// n = 7, i = 3, a < d, b < f, b < g, c < e, c < f, e < g
fn select_132([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_15([b, c, a, d, e, f])
    } else {
        select_42([b, e, f, a, d, g])
    }
}
/// n = 8, i = 3, a < f, b < e, c < g, d < f, d < g, g < h
fn select_133([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_132([b, d, a, e, c, f, g])
    } else {
        select_109([b, d, c, e, a, g, h, f])
    }
}
/// n = 8, i = 3, a < d, b < e, b < f, c < e, c < f, e < h, f < g
fn select_134([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_28([f, e, a, h, g])
    } else {
        select_126([a, b, c, d, f, g, e])
    }
}
/// n = 7, i = 3, a < d, b < e, b < f, c < e, c < f, f < g
fn select_135([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_75([b, c, a, d, e, f])
    } else {
        select_19([e, f, a, g])
    }
}
/// n = 9, i = 4, a < d, b < f, b < g, c < e, c < h, e < g, f < h, h < i
fn select_136([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_134([a, c, f, d, e, h, i, g])
    } else {
        select_135([a, b, e, d, g, f, h])
    }
}
/// n = 9, i = 4, a < d, b < f, b < i, c < e, c < g, c < h, e < i, f < g, f < h
fn select_137([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < h {
        select_136([a, b, c, d, e, f, i, g, h])
    } else {
        select_136([a, b, c, d, e, f, i, h, g])
    }
}
/// n = 9, i = 4, a < e, b < g, b < i, c < d, c < f, c < h, f < i, g < h
fn select_138([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_133([f, a, d, b, e, i, g, h])
    } else {
        select_137([a, b, c, e, f, g, d, h, i])
    }
}
/// n = 7, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g
fn select_139([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_25([a, c, e, f, g])
    } else {
        select_13([b, c, d, e])
    }
}
/// n = 8, i = 3, a < f, b < d, b < e, c < e, c < f, c < g, d < g, f < h
fn select_140([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_4([c, d, a, e, g])
    } else {
        select_139([b, a, c, e, d, f, h])
    }
}
/// n = 7, i = 4, a < e, b < f, c < f, d < f, f < g
fn select_141([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_3([a, f, g])
    } else {
        select_8([b, c, d, e])
    }
}
/// n = 10, i = 5, a < f, b < e, b < h, c < g, d < g, e < i, f < j, g < h, g < i, g < j
fn select_142([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_140([a, b, g, e, h, f, i, j])
    } else {
        select_141([b, c, d, f, e, g, j])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, c < d, c < e, d < g
fn select_143([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if g < b {
        select_1([g])
    } else {
        select_15([a, b, c, d, e, f])
    }
}
/// n = 8, i = 3, a < h, b < e, b < f, c < d, c < f, c < g, d < h, e < g
fn select_144([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_105([e, c, d, f, a, g])
    } else {
        select_143([a, b, c, d, f, e, h])
    }
}
/// n = 10, i = 5, a < e, b < f, b < i, c < g, d < g, f < j, g < h, g < i, g < j
fn select_145([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_142([a, b, c, d, f, e, g, i, j, h])
    } else {
        select_144([a, b, g, h, f, i, j, e])
    }
}
/// n = 9, i = 4, b < e, b < i, c < f, d < f, f < g, f < i, g < h
fn select_146([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_45([b, f, a, e, g, h, i])
    } else {
        select_141([b, a, c, d, e, f, g])
    }
}
/// n = 11, i = 5, a < e, a < k, b < f, b < i, c < g, c < i, d < g, d < i, f < j, g < h, g < j, i < k
fn select_147([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_145([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_146([f, a, c, d, e, i, g, h, k])
    }
}
/// n = 11, i = 5, a < i, b < e, b < k, c < f, c < i, d < g, d < i, f < j, g < h, g < j, i < k
fn select_148([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_138([b, c, g, h, e, a, f, j, i])
    } else {
        select_147([b, c, a, d, e, f, g, h, i, j, k])
    }
}
/// n = 7, i = 3, a < g, b < f, c < e, d < f, d < g
fn select_149([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_132([c, d, a, e, b, g, f])
    } else {
        select_132([c, d, b, e, a, f, g])
    }
}
/// n = 7, i = 3, a < e, b < e, b < f, c < d, c < f, f < g
fn select_150([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < d {
        select_46([a, c, e, f, d, g])
    } else {
        select_52([a, b, c, d, e, f])
    }
}
/// n = 8, i = 3, a < d, b < g, b < h, c < e, c < g, e < f, e < h
fn select_151([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_45([b, e, f, g, a, d, h])
    } else {
        select_150([b, c, a, d, g, e, f])
    }
}
/// n = 8, i = 3, b < e, c < g, c < h, d < f, d < g, f < h
fn select_152([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_149([a, c, b, d, e, g, f])
    } else {
        select_151([b, c, d, e, f, a, g, h])
    }
}
/// n = 6, i = 3, a < f, b < e, c < e, c < f, d < e, d < f
fn select_153([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_87([b, c, d, f])
    } else {
        select_87([a, c, d, e])
    }
}
/// n = 6, i = 2, a < f, b < d, b < e, c < d, c < e, c < f
fn select_154([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_2([d, e])
    } else {
        select_11([a, b, c, e, f])
    }
}
/// n = 9, i = 4, a < h, a < i, b < f, c < e, c < g, d < f, d < i, e < i, f < g, f < h
fn select_155([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_153([a, b, d, e, f, i])
    } else {
        select_154([a, c, f, e, g, h])
    }
}
/// n = 6, i = 4, a < f, b < f, c < e, d < e
fn select_156([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_7([a, b, e])
    } else {
        select_7([c, d, f])
    }
}
/// n = 7, i = 4, a < e, b < f, c < f, d < e, d < g, f < g
fn select_157([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < g {
        select_156([b, c, a, d, e, f])
    } else {
        select_1([g])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < e, e < g
fn select_158([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_157([b, a, d, c, f, e, g])
    } else {
        select_52([b, c, e, a, f, g])
    }
}
/// n = 6, i = 3, a < f, b < e, b < f, c < d, c < f
fn select_159([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_11([a, d, b, e, f])
    } else {
        select_11([a, e, c, d, f])
    }
}
/// n = 8, i = 4, a < h, b < g, b < h, c < f, d < e, d < g, e < h, f < g
fn select_160([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_158([b, c, a, d, e, f, h])
    } else {
        select_159([b, d, f, a, e, g])
    }
}
/// n = 9, i = 4, a < g, b < e, b < h, c < g, c < h, d < f, d < i, f < h, g < i
fn select_161([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_155([b, a, d, c, f, g, i, e, h])
    } else {
        select_160([a, c, d, b, e, f, h, g])
    }
}
/// n = 5, i = 3, a < e, b < e, c < d
fn select_162([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_7([a, b, d])
    } else {
        select_6([c, e])
    }
}
/// n = 7, i = 3, b < e, c < d, c < f, c < g, e < f, e < g
fn select_163([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_35([c, e, a, d, f, g])
    } else {
        select_162([a, b, c, d, e])
    }
}
/// n = 8, i = 3, b < h, c < f, c < g, d < e, e < f, e < g, e < h
fn select_164([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_53([a, d, b, c, e, h])
    } else {
        select_163([a, d, c, b, e, f, g])
    }
}
/// n = 8, i = 4, a < h, b < e, b < g, c < f, d < f, f < g, f < h
fn select_165([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_154([a, b, f, e, g, h])
    } else {
        select_9([a, c, d, e, f])
    }
}
/// n = 10, i = 4, a < i, a < j, b < f, b < h, c < e, c < g, d < f, d < g, f < j, g < h, g < i
fn select_166([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_164([e, a, b, d, g, f, h, i])
    } else {
        select_165([a, c, b, d, e, f, g, j])
    }
}
/// n = 10, i = 4, a < f, a < j, b < h, b < i, c < e, c < g, d < g, d < h, g < i, h < j
fn select_167([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_161([b, c, d, a, e, f, h, g, j])
    } else {
        select_166([a, b, c, d, e, h, g, i, f, j])
    }
}
/// n = 10, i = 4, a < h, b < f, b < j, c < h, c < i, d < e, d < g, g < i, h < j
fn select_168([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_152([e, b, c, g, f, a, i, h])
    } else {
        select_167([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 11, i = 5, a < j, b < f, b < k, c < e, c < j, d < g, d < j, g < h, g < i, j < k
fn select_169([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_148([a, b, c, d, f, e, g, i, j, h, k])
    } else {
        select_168([a, b, c, g, i, f, h, j, e, k])
    }
}
/// n = 13, i = 6, a < g, a < l, b < f, b < h, c < e, c < i, d < h, d < i, h < l, i < j, i < k, l < m
fn select_170([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_131([a, c, b, f, g, i, h, j, k, e, l, m])
    } else {
        select_169([e, a, b, d, f, g, i, j, k, h, l])
    }
}
/// n = 14, i = 6, a < g, b < f, b < l, c < e, c < i, d < h, d < i, h < l, i < j, i < k, i < n, l < m, l < n
fn select_171([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < a {
        select_104([b, h, c, e, f, a, i, j, k, g, l, m, n])
    } else {
        select_170([b, a, c, d, e, g, f, h, i, j, k, l, m])
    }
}
/// n = 6, i = 1, b < f, c < d, c < e
fn select_172([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_28([a, c, d, e, b])
    } else {
        select_43([b, c, d, e, a, f])
    }
}
/// n = 7, i = 1, b < g, c < d, c < e, c < f
fn select_173([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < b {
        select_34([a, c, d, e, f])
    } else {
        select_172([a, b, c, e, f, g])
    }
}
/// n = 7, i = 1, d < e, d < f, d < g
fn select_174([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_173([c, a, d, e, f, g, b])
    } else {
        select_173([c, b, d, e, f, g, a])
    }
}
/// n = 9, i = 1, d < h, d < i, e < f, e < g
fn select_175([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_174([a, b, c, d, e, h, i])
    } else {
        select_174([a, b, c, e, d, f, g])
    }
}
/// n = 7, i = 2, a < g, b < e, b < f, c < d, c < e, c < f, c < g
fn select_176([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_4([a, c, d, b, g])
    } else {
        select_35([b, c, d, a, e, f])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < e, c < f, c < g
fn select_177([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_172([e, d, b, f, g, a])
    } else {
        select_176([a, b, c, e, f, g, d])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < f, c < i, f < g, f < h
fn select_178([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_174([a, d, e, f, b, g, h])
    } else {
        select_177([a, b, c, d, e, f, i])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, c < g, f < j, g < h, g < i
fn select_179([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_175([b, d, e, f, g, h, i, a, j])
    } else {
        select_178([b, a, c, d, e, g, h, i, f])
    }
}
/// n = 12, i = 3, b < e, b < f, c < d, c < g, f < j, g < h, g < i, g < k, k < l
fn select_180([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_179([a, d, g, h, i, k, b, e, f, l])
    } else {
        select_74([a, c, b, e, d, f, j, g, h, i, k])
    }
}
/// n = 8, i = 4, a < f, a < h, b < e, b < g, c < d, c < g, c < h, e < h, f < g
fn select_181([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_75([a, e, c, d, f, h])
    } else {
        select_75([b, f, c, d, e, g])
    }
}
/// n = 7, i = 5, a < g, b < g, c < f, d < e
fn select_182([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_156([a, b, c, e, f, g])
    } else {
        select_156([a, b, d, f, e, g])
    }
}
/// n = 9, i = 5, b < g, c < f, c < i, d < e, d < h, e < i, f < h, g < h, g < i
fn select_183([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_181([c, d, g, a, e, f, h, i])
    } else {
        select_182([a, b, c, d, e, f, g])
    }
}
/// n = 7, i = 3, a < g, b < d, c < e, e < f, e < g
fn select_184([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_122([f, a, b, d, g])
    } else {
        select_88([a, d, c, e, f])
    }
}
/// n = 8, i = 3, a < d, a < h, b < e, b < h, c < e, c < h, e < f, e < g
fn select_185([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_87([d, b, c, h])
    } else {
        select_28([a, e, f, g, h])
    }
}
/// n = 8, i = 3, a < g, a < h, b < d, b < h, c < e, c < h, e < f, e < g
fn select_186([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_51([b, e, f, d, a, g])
    } else {
        select_185([b, a, c, d, e, f, g, h])
    }
}
/// n = 8, i = 3, b < h, c < g, c < h, d < e, d < h, e < f, e < g
fn select_187([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_184([c, a, d, b, e, f, g])
    } else {
        select_186([c, b, d, a, e, f, g, h])
    }
}
/// n = 10, i = 5, b < e, b < j, c < g, d < f, d < h, f < j, g < h, g < j, h < i
fn select_188([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_183([a, c, b, d, f, e, g, h, j])
    } else {
        select_187([a, f, b, g, h, i, e, j])
    }
}
/// n = 5, i = 0
fn select_189([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_33([a, c, d, e])
    } else {
        select_33([b, c, d, e])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < i, d < e, d < f, d < g, d < h
fn select_190([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_189([a, e, f, g, h])
    } else {
        select_19([c, b, d, i])
    }
}
/// n = 5, i = 2, b < d, b < e, c < d, c < e
fn select_191([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_6([b, c])
    } else {
        select_19([a, c, d, e])
    }
}
/// n = 10, i = 3, a < d, a < j, b < e, b < j, c < e, c < j, e < f, e < g, e < h, e < i
fn select_192([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_189([a, f, g, h, i])
    } else {
        select_191([d, b, c, e, j])
    }
}
/// n = 12, i = 4, a < f, a < h, b < f, b < h, c < d, c < h, d < e, f < g, h < i, h < j, h < k, h < l
fn select_193([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < d {
        select_190([f, c, g, h, i, j, k, l, d])
    } else {
        select_192([d, a, b, e, h, i, j, k, l, f])
    }
}
/// n = 4, i = 1, c < d
fn select_194([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_20([a, c, d, b])
    } else {
        select_20([b, c, d, a])
    }
}
/// n = 9, i = 2, a < c, a < d, b < e, e < f, e < g, e < h, e < i
fn select_195([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_189([a, f, g, h, i])
    } else {
        select_194([c, d, b, e])
    }
}
/// n = 12, i = 4, a < e, a < f, b < e, b < g, c < f, c < g, d < f, d < g, f < l, g < h, g < i, g < j, g < k
fn select_196([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_189([a, h, i, j, k])
    } else {
        select_126([b, c, d, e, f, l, g])
    }
}
/// n = 12, i = 4, a < e, b < e, b < h, c < f, c < h, d < f, d < h, f < g, h < i, h < j, h < k, h < l
fn select_197([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_195([f, b, a, g, h, i, j, k, l])
    } else {
        select_196([a, b, c, d, e, f, h, i, j, k, l, g])
    }
}
/// n = 12, i = 4, b < e, b < h, c < f, c < h, d < f, d < h, f < g, h < i, h < j, h < k, h < l
fn select_198([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_193([c, d, b, e, a, f, g, h, i, j, k, l])
    } else {
        select_197([a, b, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 13, i = 5, b < f, b < j, c < g, d < e, d < h, d < j, g < h, g < j, h < i, j < k, j < l, j < m
fn select_199([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < j {
        select_188([a, b, c, d, f, e, g, h, i, j])
    } else {
        select_198([a, b, d, g, f, h, i, j, e, k, l, m])
    }
}
/// n = 14, i = 5, b < f, b < h, c < e, c < i, d < g, g < h, g < i, h < l, i < j, i < k, i < m, m < n
fn select_200([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < b {
        select_179([a, e, i, j, k, m, b, f, h, n])
    } else {
        select_199([a, c, d, b, f, e, g, h, l, i, j, k, m])
    }
}
/// n = 14, i = 5, b < f, c < e, c < j, d < g, g < h, g < j, h < i, j < k, j < l, j < m, m < n
fn select_201([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < b {
        select_180([a, h, c, e, i, b, j, k, l, f, m, n])
    } else {
        select_200([a, b, c, d, e, f, g, h, j, k, l, i, m, n])
    }
}
/// n = 14, i = 5, b < f, c < e, c < j, d < g, g < h, g < i, g < j, j < k, j < l, j < m, m < n
fn select_202([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_201([a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    } else {
        select_201([a, b, c, d, e, f, g, i, h, j, k, l, m, n])
    }
}
/// n = 15, i = 6, a < g, b < f, b < i, c < e, c < h, d < h, d < i, h < j, h < k, h < n, i < l, i < m, i < n, n < o
fn select_203([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_202([e, a, b, d, f, g, h, j, k, i, l, m, n, o])
    } else {
        select_202([f, a, c, d, e, g, i, l, m, h, j, k, n, o])
    }
}
/// n = 15, i = 6, a < h, b < g, b < i, c < f, c < j, d < e, d < i, d < j, i < m, i < n, j < k, j < l, j < n, n < o
fn select_204([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < i {
        select_171([a, b, c, d, f, g, h, e, j, k, l, i, m, n])
    } else {
        select_203([a, b, c, d, f, g, h, j, i, k, l, e, m, n, o])
    }
}
/// n = 15, i = 6, c < h, c < i, d < g, d < j, e < f, e < i, e < j, i < m, i < n, j < k, j < l, j < n, n < o
fn select_205([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < b {
        select_204([a, c, d, e, f, g, h, b, i, j, k, l, m, n, o])
    } else {
        select_204([b, c, d, e, f, g, h, a, i, j, k, l, m, n, o])
    }
}
/// n = 8, i = 3, a < f, a < h, b < e, b < f, c < d, c < e, d < h, f < g
fn select_206([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < c {
        select_2([c, g])
    } else {
        select_22([d, a, b, e, f, h])
    }
}
/// n = 9, i = 3, a < f, a < i, b < e, b < f, c < d, c < e, d < i, e < g, f < h
fn select_207([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_19([d, e, a, g])
    } else {
        select_80([b, c, a, d, f, h, i, e])
    }
}
/// n = 9, i = 3, a < e, b < g, c < e, c < f, d < f, d < g, f < h, g < i
fn select_208([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_206([d, c, a, b, e, f, h, g])
    } else {
        select_207([c, d, b, a, g, f, i, h, e])
    }
}
/// n = 7, i = 2, a < d, a < g, b < c, b < g, c < e, d < f
fn select_209([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_20([a, c, e, g])
    } else {
        select_20([b, d, f, g])
    }
}
/// n = 7, i = 2, a < e, a < g, b < c, b < d, b < g, e < f
fn select_210([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_209([a, b, c, e, d, f, g])
    } else {
        select_209([a, b, d, e, c, f, g])
    }
}
/// n = 9, i = 3, a < e, a < f, b < d, b < f, c < d, c < e, d < g, e < h, f < i
fn select_211([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_2([a, g])
    } else {
        select_209([b, c, e, f, h, i, d])
    }
}
/// n = 10, i = 3, a < g, b < f, b < g, b < i, c < d, c < f, d < e, d < i, f < h, g < j
fn select_212([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_210([b, d, a, e, f, h, i])
    } else {
        select_211([a, b, c, f, d, g, h, e, j])
    }
}
/// n = 10, i = 3, b < g, c < e, c < f, d < f, d < g, d < i, e < i, f < h, g < j
fn select_213([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_208([a, b, c, d, e, f, g, h, j])
    } else {
        select_212([b, d, c, e, a, f, g, h, i, j])
    }
}
/// n = 7, i = 3, a < g, b < e, b < g, c < d, c < f, c < g, e < f
fn select_214([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_4([c, e, a, d, f])
    } else {
        select_75([a, b, c, d, e, g])
    }
}
/// n = 8, i = 3, b < e, b < f, c < d, c < f, c < h, e < h, f < g
fn select_215([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_172([d, e, f, a, g, h])
    } else {
        select_214([a, b, c, d, e, h, f])
    }
}
/// n = 7, i = 1, a < g, b < f, c < d, c < e
fn select_216([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_43([a, c, d, e, b, g])
    } else {
        select_43([b, c, d, e, a, f])
    }
}
/// n = 8, i = 3, a < h, b < e, b < g, b < h, c < d, c < f, c < h, d < g, e < f
fn select_217([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_154([a, d, b, e, g, h])
    } else {
        select_154([a, e, c, d, f, h])
    }
}
/// n = 9, i = 3, b < e, b < f, b < i, c < d, c < f, c < h, d < i, e < h, f < g
fn select_218([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_216([d, e, f, a, g, h, i])
    } else {
        select_217([a, b, c, d, e, h, i, f])
    }
}
/// n = 10, i = 3, b < g, c < f, c < i, d < e, d < f, d < g, e < i, f < h, g < j
fn select_219([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_215([a, b, d, e, c, g, j, f])
    } else {
        select_218([a, c, d, e, b, f, h, g, i])
    }
}
/// n = 10, i = 3, b < h, c < f, c < g, d < e, d < g, d < h, g < i, h < j
fn select_220([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_213([a, b, c, d, f, g, h, i, e, j])
    } else {
        select_219([a, b, c, d, e, g, h, i, f, j])
    }
}
/// n = 8, i = 2, c < d, c < e, e < f, e < g, e < h
fn select_221([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_173([b, d, e, f, g, h, a])
    } else {
        select_71([b, a, c, e, f, g, h, d])
    }
}
/// n = 7, i = 2, a < e, a < f, a < g, b < c, b < d, b < f, b < g
fn select_222([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_29([a, b, e, c, d, f, g])
    } else {
        select_29([a, b, e, d, c, f, g])
    }
}
/// n = 8, i = 3, b < e, b < f, c < d, c < f, c < g, c < h, e < g, e < h
fn select_223([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_15([a, b, c, d, f, e])
    } else {
        select_222([e, c, d, f, a, g, h])
    }
}
/// n = 10, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, c < j, e < j, g < h, g < i
fn select_224([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_221([a, d, b, f, g, e, h, i])
    } else {
        select_223([a, b, c, d, e, f, g, j])
    }
}
/// n = 10, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < j, g < h, g < i
fn select_225([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_57([a, b, c, d, e, f, g, j])
    } else {
        select_119([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 12, i = 4, a < d, a < f, a < i, b < c, b < e, b < j, d < l, e < g, e < i, e < l, f < h, f < j, j < k
fn select_226([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_224([c, a, e, g, d, i, f, h, j, l])
    } else {
        select_225([d, b, f, h, c, j, e, g, i, k])
    }
}
/// n = 12, i = 4, a < d, a < f, a < j, b < c, b < e, b < g, e < l, f < i, f < l, g < h, g < j, j < k
fn select_227([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < f {
        select_220([e, c, g, a, d, h, j, f, k, i])
    } else {
        select_226([b, a, d, e, f, g, i, h, c, j, k, l])
    }
}
/// n = 8, i = 2, b < h, c < d, c < e, c < h, d < g, e < f
fn select_228([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_48([a, b, c, d, e, g, h])
    } else {
        select_48([a, b, c, e, d, f, h])
    }
}
/// n = 8, i = 2, a < h, b < d, b < g, c < e, c < g, c < h, e < f
fn select_229([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_209([a, c, e, b, f, d, h])
    } else {
        select_210([c, b, a, d, e, f, g])
    }
}
/// n = 9, i = 2, a < c, a < i, b < d, b < e, b < i, d < h, e < f, e < g
fn select_230([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_67([d, e, a, f, g, h])
    } else {
        select_29([a, b, c, d, h, e, i])
    }
}
/// n = 9, i = 2, b < d, b < i, c < e, c < f, c < i, e < h, f < g
fn select_231([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_229([a, b, c, d, e, h, i, f])
    } else {
        select_230([b, c, d, e, f, a, g, h, i])
    }
}
/// n = 10, i = 3, b < d, b < g, b < h, c < e, c < g, c < h, e < f, g < j, h < i
fn select_232([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_228([a, d, c, g, h, i, j, e])
    } else {
        select_231([a, e, b, f, g, h, i, j, d])
    }
}
/// n = 10, i = 3, b < f, c < g, c < h, d < e, d < g, d < h, g < j, h < i
fn select_233([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_32([a, d, b, f, e, c, g, h])
    } else {
        select_232([a, d, c, e, b, f, g, h, i, j])
    }
}
/// n = 11, i = 4, a < d, a < f, b < c, b < e, b < i, e < h, e < k, f < g, f < i, f < k, i < j
fn select_234([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_119([c, a, e, h, d, f, g, i, k])
    } else {
        select_225([d, b, f, g, c, i, e, h, k, j])
    }
}
/// n = 12, i = 4, a < d, a < g, a < j, b < c, b < e, b < f, e < j, f < h, f < l, g < i, g < l, j < k
fn select_235([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < g {
        select_233([c, f, e, a, d, h, g, j, k, i])
    } else {
        select_234([a, b, c, d, f, g, i, h, e, j, l])
    }
}
/// n = 13, i = 4, a < f, a < k, b < e, b < j, c < d, c < g, c < h, g < k, h < i, h < j, j < l, k < m
fn select_236([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < a {
        select_227([b, c, d, e, g, a, h, i, f, j, l, k])
    } else {
        select_235([a, c, d, f, g, h, b, i, e, k, m, j])
    }
}
/// n = 10, i = 4, a < d, a < f, b < c, b < e, e < g, e < i, e < j, f < h, f < i, f < j
fn select_237([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_119([c, a, e, g, d, f, h, i, j])
    } else {
        select_119([d, b, f, h, c, e, g, i, j])
    }
}
/// n = 12, i = 4, a < d, a < f, b < c, b < e, b < i, e < h, e < l, f < g, f < i, f < l, i < j, i < k
fn select_238([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < e {
        select_119([d, b, f, g, c, i, e, j, k])
    } else {
        select_237([a, b, c, d, e, f, h, g, i, l])
    }
}
/// n = 13, i = 4, a < f, a < i, b < e, b < j, c < d, c < g, g < h, g < i, g < j, i < m, j < k, j < l
fn select_239([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_234([c, a, f, d, b, g, h, e, i, m, j])
    } else {
        select_238([c, b, e, d, a, g, h, f, j, k, l, i])
    }
}
/// n = 7, i = 4, a < f, b < f, b < g, c < f, c < g, d < e, e < g
fn select_240([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_162([b, c, d, e, f])
    } else {
        select_87([a, b, c, g])
    }
}
/// n = 8, i = 4, a < e, b < g, b < h, c < f, c < g, d < f, d < g, f < h
fn select_241([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_240([b, c, d, a, e, g, f])
    } else {
        select_22([a, f, b, g, h, e])
    }
}
/// n = 8, i = 4, a < h, b < e, c < g, c < h, d < f, d < h, f < g
fn select_242([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_241([b, c, a, d, e, f, h, g])
    } else {
        select_132([b, c, f, e, a, g, h])
    }
}
/// n = 8, i = 2, a < c, a < g, a < h, b < d, b < g, b < h, d < e, d < f
fn select_243([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_19([c, b, g, h])
    } else {
        select_43([a, d, e, f, g, h])
    }
}
/// n = 8, i = 2, a < d, a < g, a < h, b < c, b < e, b < g, b < h, e < f
fn select_244([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_29([a, b, d, c, e, g, h])
    } else {
        select_243([a, b, d, e, c, f, g, h])
    }
}
/// n = 9, i = 3, a < d, b < g, b < h, b < i, c < e, c < g, e < f, e < h, e < i
fn select_245([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_244([b, e, f, g, a, d, h, i])
    } else {
        select_150([b, c, a, d, g, e, f])
    }
}
/// n = 10, i = 4, b < e, c < g, c < h, d < f, d < g, f < h, f < i, f < j, g < i, g < j
fn select_246([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_242([a, b, c, d, e, f, h, g])
    } else {
        select_245([b, f, c, e, g, a, h, i, j])
    }
}
/// n = 8, i = 3, a < d, b < e, b < f, c < e, c < f, f < g, f < h
fn select_247([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_34([e, f, a, g, h])
    } else {
        select_75([b, c, a, d, e, f])
    }
}
/// n = 8, i = 3, c < e, c < f, d < e, d < f, f < g, f < h
fn select_248([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_247([a, c, d, b, e, f, g, h])
    } else {
        select_247([b, c, d, a, e, f, g, h])
    }
}
/// n = 10, i = 4, b < e, c < g, c < h, d < f, f < g, f < h, h < i, h < j
fn select_249([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_17([a, c, b, d, e, f, g])
    } else {
        select_248([a, b, c, f, g, h, i, j])
    }
}
/// n = 11, i = 4, b < e, c < g, c < h, c < i, d < f, d < g, f < h, f < i, f < k, g < k, i < j
fn select_250([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_246([a, b, c, d, e, f, g, h, i, k])
    } else {
        select_249([a, b, c, d, e, f, h, i, g, j])
    }
}
/// n = 8, i = 2, a < d, a < f, b < c, b < e, e < g, f < h
fn select_251([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_51([a, b, c, d, e, g])
    } else {
        select_51([b, a, d, c, f, h])
    }
}
/// n = 8, i = 2, a < f, b < e, c < d, c < g, g < h
fn select_252([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_251([a, c, d, f, g, b, h, e])
    } else {
        select_251([b, c, d, e, g, a, h, f])
    }
}
/// n = 10, i = 4, a < h, b < e, c < g, c < i, d < f, f < g, f < h, f < i, i < j
fn select_253([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_252([a, b, c, g, e, h, i, j])
    } else {
        select_164([e, a, c, d, f, g, i, h])
    }
}
/// n = 10, i = 3, a < d, a < g, a < h, b < c, b < g, b < h, b < j, c < f, d < e, d < j, h < i
fn select_254([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_45([c, a, g, f, h, i, d])
    } else {
        select_244([d, b, g, e, h, i, c, j])
    }
}
/// n = 7, i = 3, a < e, b < e, b < f, b < g, c < d, d < f, d < g
fn select_255([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_19([a, b, f, g])
    } else {
        select_13([b, c, d, e])
    }
}
/// n = 8, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g, f < h
fn select_256([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_27([a, g, h])
    } else {
        select_4([b, c, d, e, f])
    }
}
/// n = 9, i = 4, a < f, a < g, b < e, b < h, b < i, c < d, c < f, e < g, f < h, f < i
fn select_257([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_255([d, b, a, f, e, h, i])
    } else {
        select_256([e, a, c, d, g, f, h, i])
    }
}
/// n = 11, i = 4, a < d, b < f, b < h, c < e, c < g, c < i, e < h, e < k, f < g, f < i, f < k, i < j
fn select_258([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_254([c, f, a, e, h, d, g, i, j, k])
    } else {
        select_257([b, c, a, d, e, f, h, g, i])
    }
}
/// n = 12, i = 5, a < e, b < g, c < d, c < f, f < i, f < j, f < l, g < h, g < i, g < j, h < l, j < k
fn select_259([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < h {
        select_253([d, a, f, b, e, g, i, h, j, k])
    } else {
        select_258([a, c, g, e, h, f, i, d, j, k, l])
    }
}
/// n = 12, i = 5, a < e, b < g, b < i, c < d, c < f, f < i, f < j, f < k, g < h, g < j, g < k, k < l
fn select_260([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_250([d, a, f, b, e, g, i, j, k, l, h])
    } else {
        select_259([a, b, c, d, e, f, g, h, j, k, l, i])
    }
}
/// n = 14, i = 5, a < e, a < m, b < d, b < f, c < g, c < k, f < k, f < l, g < h, g < i, g < l, i < j, i < m, m < n
fn select_261([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < f {
        select_239([a, b, g, h, d, e, i, j, m, f, k, l, n])
    } else {
        select_260([a, c, b, d, e, f, g, h, k, l, i, j])
    }
}
/// n = 14, i = 5, a < e, a < m, b < d, b < f, c < g, c < l, f < l, g < h, g < i, g < j, j < k, j < m, m < n
fn select_262([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < f {
        select_236([b, a, g, i, e, d, h, j, k, m, f, n, l])
    } else {
        select_261([a, b, c, d, e, f, g, i, j, k, l, h, m, n])
    }
}
/// n = 7, i = 2, b < g, c < e, c < f, d < e, d < f, d < g
fn select_263([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_47([a, b, c, d, e, f])
    } else {
        select_48([a, b, d, c, e, f, g])
    }
}
/// n = 9, i = 3, a < g, b < e, c < f, c < h, d < f, d < g, d < h, h < i
fn select_264([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < d {
        select_263([e, a, c, d, f, h, g])
    } else {
        select_252([a, b, c, f, e, g, h, i])
    }
}
/// n = 11, i = 4, a < e, b < f, b < h, b < i, c < d, c < g, f < k, g < h, g < i, g < k, i < j
fn select_265([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < f {
        select_264([d, a, g, b, e, h, f, i, j])
    } else {
        select_258([a, c, b, e, f, g, h, d, i, j, k])
    }
}
/// n = 12, i = 5, a < d, b < f, b < i, c < e, c < i, e < g, e < j, e < k, f < h, f < j, f < k, k < l
fn select_266([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_265([a, e, b, i, d, g, f, j, k, l, h])
    } else {
        select_265([a, f, c, i, d, h, e, j, k, l, g])
    }
}
/// n = 14, i = 5, a < d, a < m, b < e, b < i, c < f, c < i, e < h, e < l, f < g, f < j, f < l, j < k, j < m, m < n
fn select_267([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < e {
        select_239([a, b, f, g, i, d, j, k, m, e, h, l, n])
    } else {
        select_266([a, b, c, d, f, e, g, h, i, l, j, k])
    }
}
/// n = 14, i = 5, a < e, a < j, b < d, b < m, c < f, c < j, e < i, f < g, f < h, f < k, k < l, k < m, m < n
fn select_268([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < e {
        select_236([a, b, f, h, d, j, g, k, l, m, e, n, i])
    } else {
        select_267([b, a, c, d, e, f, h, i, j, k, l, g, m, n])
    }
}
/// n = 14, i = 5, a < j, b < f, c < e, c < m, d < g, d < j, g < h, g < i, g < k, k < l, k < m, m < n
fn select_269([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < a {
        select_262([c, b, d, f, e, a, g, h, i, k, l, j, m, n])
    } else {
        select_268([a, c, d, e, b, g, h, i, f, j, k, l, m, n])
    }
}
/// n = 14, i = 5, c < j, d < f, d < m, e < g, e < j, g < h, g < i, g < k, k < l, k < m, m < n
fn select_270([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_269([c, a, d, e, f, b, g, h, i, j, k, l, m, n])
    } else {
        select_269([c, b, d, e, f, a, g, h, i, j, k, l, m, n])
    }
}
/// n = 15, i = 6, c < h, c < n, d < g, d < i, e < f, e < i, i < j, i < k, i < l, l < m, l < n, n < o
fn select_271([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < g {
        select_270([a, b, f, c, d, h, i, j, k, g, l, m, n, o])
    } else {
        select_270([a, b, g, c, e, h, i, j, k, f, l, m, n, o])
    }
}
/// n = 15, i = 6, c < h, c < i, d < g, d < j, e < f, e < i, e < j, i < m, j < k, j < l, j < n, n < o
fn select_272([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < n {
        select_205([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o])
    } else {
        select_271([a, b, c, d, e, f, g, h, j, k, l, n, o, i, m])
    }
}
/// n = 7, i = 4, a < g, b < f, b < g, c < f, c < g, d < e, d < f
fn select_273([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_15([a, c, d, e, f, g])
    } else {
        select_15([a, b, d, e, f, g])
    }
}
/// n = 8, i = 4, a < g, b < f, c < f, c < g, d < e, d < g, d < h, f < h
fn select_274([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_273([b, a, c, d, e, g, f])
    } else {
        select_4([d, f, a, e, h])
    }
}
/// n = 7, i = 2, b < d, b < g, c < e, c < g, e < f
fn select_275([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_22([a, c, b, d, g, e])
    } else {
        select_44([b, c, d, e, a, f, g])
    }
}
/// n = 9, i = 4, b < g, b < i, c < f, c < g, d < e, d < f, d < h, f < i, g < h
fn select_276([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_274([a, b, c, d, e, g, f, h])
    } else {
        select_275([e, f, b, a, g, h, i])
    }
}
/// n = 6, i = 3, b < f, c < e, d < e, d < f
fn select_277([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_52([c, d, a, b, e, f])
    } else {
        select_15([c, d, b, a, f, e])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < f
fn select_278([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < d {
        select_22([a, c, b, e, f, d])
    } else {
        select_42([b, c, e, d, a, f])
    }
}
/// n = 7, i = 3, c < f, c < g, d < e, d < f, e < g
fn select_279([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_277([b, a, c, d, f, e])
    } else {
        select_278([b, c, e, a, f, g])
    }
}
/// n = 9, i = 4, b < g, c < g, c < h, d < e, d < f, d < i, f < h, g < i
fn select_280([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_276([a, c, b, d, e, f, g, i, h])
    } else {
        select_279([a, e, c, f, b, h, g])
    }
}
/// n = 9, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_281([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_33([a, g, h, i])
    } else {
        select_4([b, c, d, e, f])
    }
}
/// n = 9, i = 3, a < e, b < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_282([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_69([d, b, a, f, g, h, i])
    } else {
        select_281([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 9, i = 3, a < h, b < d, b < i, c < e, c < h, e < f, e < g, e < i
fn select_283([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_116([a, e, f, g, h, b, d, i])
    } else {
        select_282([a, c, b, d, h, e, f, g, i])
    }
}
/// n = 8, i = 3, a < e, a < g, a < h, b < c, b < d, b < g, d < h, e < f
fn select_284([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_5([c, d, a, g, h, e])
    } else {
        select_45([e, b, g, f, d, h, c])
    }
}
/// n = 9, i = 4, a < f, a < h, b < e, b < f, c < d, c < e, c < i, d < h, f < g, f < i
fn select_285([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_75([a, d, b, e, h, f])
    } else {
        select_35([c, f, g, e, d, i])
    }
}
/// n = 9, i = 4, a < g, b < e, b < i, c < d, c < g, c < h, d < i, e < f, e < h
fn select_286([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_284([c, e, f, a, d, i, h, g])
    } else {
        select_285([b, a, c, d, g, e, f, i, h])
    }
}
/// n = 10, i = 4, b < f, b < i, c < d, c < e, c < h, d < j, e < i, f < g, f < h, f < j
fn select_287([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_283([e, d, b, a, f, g, h, i, j])
    } else {
        select_286([a, b, c, e, f, g, d, h, i])
    }
}
/// n = 10, i = 4, c < g, c < i, d < e, d < f, d < h, e < j, f < i, g < h, g < j
fn select_288([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_280([b, a, c, d, e, f, g, i, h])
    } else {
        select_287([b, c, d, e, f, g, a, h, i, j])
    }
}
/// n = 8, i = 3, a < d, b < f, b < g, b < h, c < e, c < f, e < g, e < h
fn select_289([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_15([b, c, a, d, e, f])
    } else {
        select_29([b, e, f, a, d, g, h])
    }
}
/// n = 8, i = 3, c < f, c < g, c < h, d < e, d < f, e < g, e < h
fn select_290([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_289([a, c, d, b, e, f, g, h])
    } else {
        select_289([b, c, d, a, e, f, g, h])
    }
}
/// n = 7, i = 3, a < f, b < e, b < f, b < g, c < d, c < e, d < g
fn select_291([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if g < e {
        select_52([a, b, c, d, f, g])
    } else {
        select_15([a, b, c, d, e, f])
    }
}
/// n = 8, i = 4, a < d, b < f, c < e, c < g, e < h, f < g, f < h
fn select_292([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_291([a, f, c, e, g, d, h])
    } else {
        select_162([b, d, c, e, f])
    }
}
/// n = 8, i = 4, c < f, d < e, d < g, e < h, f < g, f < h
fn select_293([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_292([a, c, d, b, e, f, g, h])
    } else {
        select_292([b, c, d, a, e, f, g, h])
    }
}
/// n = 9, i = 4, c < e, c < h, c < i, d < f, f < g, f < h, g < i
fn select_294([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_290([a, b, c, f, g, h, e, i])
    } else {
        select_293([a, b, d, c, e, f, h, g])
    }
}
/// n = 9, i = 4, a < d, b < f, c < e, c < g, e < h, e < i, f < g, f < h, f < i
fn select_295([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_64([a, c, g, d, e, h, i])
    } else {
        select_53([d, b, c, e, f, g])
    }
}
/// n = 9, i = 4, c < f, d < e, d < g, e < h, e < i, f < g, f < h, f < i
fn select_296([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_295([a, c, d, b, e, f, g, h, i])
    } else {
        select_295([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 10, i = 4, c < e, c < h, c < i, d < f, e < j, f < g, f < h, f < j, g < i
fn select_297([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_290([a, b, c, f, g, h, e, i])
    } else {
        select_296([a, b, d, c, e, f, h, g, j])
    }
}
/// n = 10, i = 4, c < f, c < h, c < i, d < e, d < g, e < i, f < j, g < h, g < j
fn select_298([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_294([a, b, c, d, f, e, g, i, h])
    } else {
        select_297([a, b, c, d, f, g, e, h, i, j])
    }
}
/// n = 10, i = 4, c < f, c < h, d < e, d < g, d < i, g < j, h < i, h < j
fn select_299([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_288([a, b, c, d, g, e, h, i, f, j])
    } else {
        select_298([a, b, d, c, f, g, h, i, e, j])
    }
}
/// n = 9, i = 2, a < c, a < h, a < i, b < d, b < h, b < i, d < e, d < f, d < g
fn select_300([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_33([a, e, f, g])
    } else {
        select_34([c, b, d, h, i])
    }
}
/// n = 9, i = 2, b < h, b < i, c < d, c < h, c < i, d < e, d < f, d < g
fn select_301([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_69([a, c, b, d, e, f, g])
    } else {
        select_300([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 6, i = 3, a < f, b < f, c < e, d < e, d < f
fn select_302([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_52([c, d, a, b, e, f])
    } else {
        select_52([c, d, b, a, e, f])
    }
}
/// n = 8, i = 3, b < h, c < f, c < g, c < h, d < e, d < h, e < f, e < g
fn select_303([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_302([b, c, a, d, e, h])
    } else {
        select_37([b, c, e, a, f, g, h])
    }
}
/// n = 10, i = 3, c < f, c < i, c < j, d < e, d < f, e < i, e < j, f < g, f < h
fn select_304([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_301([b, e, c, f, a, g, h, i, j])
    } else {
        select_303([b, a, c, d, e, i, j, f])
    }
}
/// n = 11, i = 3, c < f, c < g, c < k, d < e, d < g, d < j, e < k, f < j, g < h, g < i
fn select_305([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_304([a, b, c, d, e, g, h, i, f, k])
    } else {
        select_304([a, b, d, c, f, g, h, i, e, j])
    }
}
/// n = 7, i = 3, a < d, b < f, b < g, c < e, e < f, e < g
fn select_306([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_87([b, c, d, e])
    } else {
        select_154([a, b, e, f, g, d])
    }
}
/// n = 7, i = 3, c < f, c < g, d < e, e < f, e < g
fn select_307([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_306([a, c, d, b, e, f, g])
    } else {
        select_306([b, c, d, a, e, f, g])
    }
}
/// n = 8, i = 4, b < e, b < h, c < f, d < f, f < g, f < h
fn select_308([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_141([b, a, c, d, e, f, g])
    } else {
        select_105([b, f, a, g, e, h])
    }
}
/// n = 9, i = 4, b < g, b < i, c < e, c < f, d < f, d < g, f < i, g < h
fn select_309([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_307([a, e, b, d, f, g, i])
    } else {
        select_308([a, c, b, d, e, g, h, f])
    }
}
/// n = 5, i = 1, c < d, c < e
fn select_310([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_28([a, c, d, e, b])
    } else {
        select_28([b, c, d, e, a])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < g, c < h, d < e, d < f
fn select_311([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_46([a, c, b, d, e, f])
    } else {
        select_243([b, c, a, d, e, f, g, h])
    }
}
/// n = 9, i = 3, b < e, b < i, c < d, c < g, c < h, d < f, d < i, e < g, e < h
fn select_312([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < b {
        select_310([a, f, b, e, i])
    } else {
        select_311([a, e, c, d, f, i, g, h])
    }
}
/// n = 10, i = 4, b < f, b < i, b < j, c < e, c < g, d < f, d < g, f < h, g < i, g < j
fn select_313([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_303([a, e, b, d, g, i, j, f])
    } else {
        select_312([a, c, b, f, g, h, i, j, e])
    }
}
/// n = 10, i = 4, b < g, b < h, c < f, c < h, c < j, d < e, d < f, d < g, g < j, h < i
fn select_314([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_309([a, b, d, c, e, f, h, i, g])
    } else {
        select_313([a, c, d, b, e, h, g, i, f, j])
    }
}
/// n = 13, i = 4, b < h, c < g, c < h, c < j, d < e, d < f, d < g, f < j, f < m, g < m, h < i, j < k, j < l
fn select_315([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < b {
        select_305([a, e, c, f, b, g, j, k, l, m, h])
    } else {
        select_314([a, b, c, d, e, g, f, h, i, j])
    }
}
/// n = 7, i = 2, b < g, c < f, d < e, d < f, d < g
fn select_316([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_278([a, b, d, e, c, g])
    } else {
        select_278([a, c, d, e, b, f])
    }
}
/// n = 6, i = 2, a < e, b < c, b < d, e < f
fn select_317([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_3([d, c, a])
    } else {
        select_42([b, a, d, e, f, c])
    }
}
/// n = 7, i = 2, a < d, a < e, b < c, b < f, f < g
fn select_318([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < c {
        select_317([b, a, d, e, f, g])
    } else {
        select_51([a, b, c, e, f, g])
    }
}
/// n = 9, i = 3, a < g, b < e, b < f, c < d, c < g, c < h, f < h, g < i
fn select_319([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_316([e, a, f, c, d, h, g])
    } else {
        select_318([b, c, d, e, f, g, i])
    }
}
/// n = 8, i = 3, a < h, b < d, b < g, c < e, c < f, e < h, f < g
fn select_320([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_278([e, b, f, a, d, g])
    } else {
        select_132([b, a, c, d, e, f, h])
    }
}
/// n = 8, i = 4, a < g, b < d, c < e, d < g, d < h, e < f, e < h
fn select_321([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_255([a, d, c, e, g, f, h])
    } else {
        select_87([a, b, f, d])
    }
}
/// n = 9, i = 4, a < f, b < f, c < d, c < h, c < i, d < e, f < g, f < h, f < i
fn select_322([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_34([g, c, d, h, i])
    } else {
        select_87([e, a, b, f])
    }
}
/// n = 10, i = 4, a < g, b < e, b < i, b < j, c < d, c < g, d < h, e < f, e < h, g < i, g < j
fn select_323([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < g {
        select_321([a, c, b, d, e, f, g, h])
    } else {
        select_322([a, c, b, e, f, g, d, i, j])
    }
}
/// n = 10, i = 4, a < h, b < e, b < g, b < j, c < d, c < g, c < h, d < i, e < f, e < i, h < j
fn select_324([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_320([a, e, c, f, g, d, i, h])
    } else {
        select_323([a, b, c, d, e, f, h, i, g, j])
    }
}
/// n = 10, i = 4, a < i, b < d, b < h, b < i, c < e, c < h, c < j, e < f, e < g, i < j
fn select_325([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_319([a, e, b, h, g, f, i, d, j])
    } else {
        select_324([a, c, b, d, e, g, h, i, f, j])
    }
}
/// n = 8, i = 2, b < h, c < d, c < e, c < h, e < f, e < g
fn select_326([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_64([a, c, d, b, e, f, g])
    } else {
        select_36([b, c, d, a, e, f, g, h])
    }
}
/// n = 9, i = 3, a < e, a < f, a < g, b < c, b < d, b < g, g < h, g < i
fn select_327([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < e {
        select_326([d, c, a, f, g, h, i, e])
    } else {
        select_326([f, e, b, d, g, h, i, c])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < j, d < h, d < i, e < f, e < g
fn select_328([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_71([a, b, c, d, e, h, i, j])
    } else {
        select_71([a, b, c, e, d, f, g, j])
    }
}
/// n = 8, i = 1, a < f, a < g, a < h, b < c, b < d, b < e
fn select_329([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_33([b, f, g, h])
    } else {
        select_33([a, c, d, e])
    }
}
/// n = 10, i = 1, a < i, a < j, b < g, b < h, c < d, c < e, c < f
fn select_330([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_329([a, c, d, e, f, b, i, j])
    } else {
        select_329([b, c, d, e, f, a, g, h])
    }
}
/// n = 9, i = 2, a < i, b < g, b < h, c < d, c < g, c < h, c < i, d < e, d < f
fn select_331([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_44([a, c, b, d, e, f, i])
    } else {
        select_243([b, c, a, d, e, f, g, h])
    }
}
/// n = 11, i = 2, b < j, b < k, c < d, c < e, c < j, c < k, d < h, d < i, e < f, e < g
fn select_332([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_330([b, e, d, a, h, i, f, g, j, k])
    } else {
        select_331([a, b, c, e, f, g, j, k, d])
    }
}
/// n = 12, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, c < l, e < l, f < j, f < k, g < h, g < i
fn select_333([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_328([a, d, b, f, g, h, i, j, k, e])
    } else {
        select_332([a, e, c, f, g, h, i, j, k, d, l])
    }
}
/// n = 13, i = 4, a < d, a < i, a < j, b < c, b < e, b < i, d < g, d < h, e < f, e < j, e < m, i < m, j < k, j < l
fn select_334([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < e {
        select_327([b, d, g, h, c, i, e, f, j])
    } else {
        select_333([c, a, e, f, i, d, j, k, l, g, h, m])
    }
}
/// n = 13, i = 4, b < d, b < f, b < i, c < e, c < i, c < j, e < g, e < h, f < j, f < m, i < m, j < k, j < l
fn select_335([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < f {
        select_325([a, b, c, d, e, g, h, i, f, j])
    } else {
        select_334([c, b, d, e, f, a, g, h, i, j, k, l, m])
    }
}
/// n = 13, i = 4, c < f, c < i, c < j, d < e, d < g, d < i, f < h, g < j, g < m, i < m, j < k, j < l
fn select_336([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < f {
        select_315([b, a, c, d, e, g, i, f, h, j, k, l, m])
    } else {
        select_335([b, d, c, e, f, g, a, h, i, j, k, l, m])
    }
}
/// n = 13, i = 4, c < g, d < i, d < j, e < f, e < h, e < i, h < j, h < m, i < m, j < k, j < l
fn select_337([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < d {
        select_299([a, b, c, e, f, g, h, d, i, j])
    } else {
        select_336([a, b, d, e, f, c, h, g, i, j, k, l, m])
    }
}
/// n = 13, i = 4, e < i, e < j, f < g, f < h, f < i, h < j, h < m, i < m, j < k, j < l
fn select_338([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_337([c, d, a, e, f, g, b, h, i, j, k, l, m])
    } else {
        select_337([c, d, b, e, f, g, a, h, i, j, k, l, m])
    }
}
/// n = 7, i = 1, c < g, d < e, d < f
fn select_339([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_216([a, c, d, e, f, g, b])
    } else {
        select_216([b, c, d, e, f, g, a])
    }
}
/// n = 8, i = 1, b < g, b < h, c < d, c < e, c < f
fn select_340([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_67([a, c, d, e, f, b])
    } else {
        select_329([b, c, d, e, f, a, g, h])
    }
}
/// n = 8, i = 1, c < g, c < h, d < e, d < f
fn select_341([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_172([b, a, d, e, f, c])
    } else {
        select_340([b, d, c, a, g, h, e, f])
    }
}
/// n = 10, i = 2, b < e, b < f, c < d, c < g, f < j, g < h, g < i
fn select_342([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_339([a, e, f, c, d, g, j])
    } else {
        select_341([a, d, b, g, h, i, e, f])
    }
}
/// n = 10, i = 2, b < g, c < f, d < e, d < h, h < i, h < j
fn select_343([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_342([a, b, d, e, g, c, h, i, j, f])
    } else {
        select_342([a, c, d, e, f, b, h, i, j, g])
    }
}
/// n = 8, i = 2, a < e, a < h, b < c, b < d, b < f, b < h, f < g
fn select_344([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_310([c, d, f, a, g])
    } else {
        select_222([a, b, c, d, e, f, h])
    }
}
/// n = 9, i = 2, b < h, b < i, c < d, c < e, c < h, c < i, e < f, e < g
fn select_345([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_340([d, b, e, a, f, g, h, i])
    } else {
        select_176([a, b, c, d, h, i, e])
    }
}
/// n = 11, i = 3, a < f, a < g, b < e, b < h, c < d, c < f, c < g, c < h, g < k, h < i, h < j
fn select_346([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_344([b, c, d, f, e, g, k, h])
    } else {
        select_345([e, a, c, d, h, i, j, f, g])
    }
}
/// n = 11, i = 3, a < h, b < e, b < i, c < d, c < f, c < h, c < i, f < g, i < j, i < k
fn select_347([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_342([d, f, b, e, g, a, i, j, k, h])
    } else {
        select_346([a, b, c, d, e, h, f, i, j, k, g])
    }
}
/// n = 11, i = 3, b < f, b < i, c < d, c < e, c < g, c < i, g < h, i < j, i < k
fn select_348([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_343([e, d, g, b, f, h, a, i, j, k])
    } else {
        select_347([a, b, c, e, f, g, h, d, i, j, k])
    }
}
/// n = 7, i = 1, e < f, e < g
fn select_349([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_339([c, d, a, e, f, g, b])
    } else {
        select_339([c, d, b, e, f, g, a])
    }
}
/// n = 10, i = 4, a < e, b < f, c < f, d < f, f < g, f < h, f < i, f < j
fn select_350([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_189([a, g, h, i, j])
    } else {
        select_9([e, b, c, d, f])
    }
}
/// n = 10, i = 4, c < f, d < f, e < f, f < g, f < h, f < i, f < j
fn select_351([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_350([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_350([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 11, i = 5, b < f, b < k, c < g, d < g, e < g, g < h, g < i, g < j, g < k
fn select_352([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_349([a, h, i, j, b, f, k])
    } else {
        select_351([a, f, c, d, e, g, h, i, j, k])
    }
}
/// n = 10, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, h < i, h < j
fn select_353([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_47([a, g, f, h, i, j])
    } else {
        select_182([b, c, a, d, f, e, g])
    }
}
/// n = 10, i = 5, a < f, b < h, c < h, d < g, d < h, e < g, e < h, h < i, h < j
fn select_354([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_353([a, d, e, b, f, c, g, h, i, j])
    } else {
        select_353([a, d, e, c, f, b, g, h, i, j])
    }
}
/// n = 8, i = 5, a < e, b < g, c < g, d < f, f < h, g < h
fn select_355([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_157([a, b, c, f, e, g, h])
    } else {
        select_156([b, c, d, e, f, g])
    }
}
/// n = 8, i = 5, a < h, b < h, c < f, d < g, e < g, g < h
fn select_356([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_355([c, d, e, a, f, b, g, h])
    } else {
        select_355([c, d, e, b, f, a, g, h])
    }
}
/// n = 11, i = 6, a < g, b < i, c < i, d < h, e < f, e < i, h < i, i < j, i < k
fn select_357([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_354([a, b, c, e, h, g, f, i, j, k])
    } else {
        select_356([b, c, a, d, f, g, h, i])
    }
}
/// n = 12, i = 6, a < h, a < l, b < i, c < i, d < g, d < i, e < f, e < i, i < j, i < k, i < l
fn select_358([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_352([g, a, b, c, e, h, i, f, j, k, l])
    } else {
        select_357([a, b, c, e, d, g, h, f, i, j, k])
    }
}
/// n = 14, i = 6, a < h, b < g, b < l, c < i, d < i, e < f, e < i, i < j, i < k, i < l, l < m, l < n
fn select_359([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_348([f, b, i, j, k, g, a, h, l, m, n])
    } else {
        select_358([b, c, d, a, e, f, h, g, i, j, k, l])
    }
}
/// n = 14, i = 6, c < h, c < l, d < i, e < i, f < g, f < i, i < j, i < k, i < l, l < m, l < n
fn select_360([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_359([a, c, d, e, f, g, h, b, i, j, k, l, m, n])
    } else {
        select_359([b, c, d, e, f, g, h, a, i, j, k, l, m, n])
    }
}
/// n = 9, i = 1, b < i, c < g, c < h, d < e, d < f
fn select_361([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_173([a, b, c, d, g, h, i])
    } else {
        select_173([a, b, d, c, e, f, i])
    }
}
/// n = 11, i = 2, b < d, b < f, c < e, c < g, e < j, f < k, g < h, g < i
fn select_362([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_339([a, d, f, c, e, g, k])
    } else {
        select_361([a, e, b, g, h, i, d, f, j])
    }
}
/// n = 8, i = 2, a < h, b < f, b < g, c < d, c < f, c < g, c < h, d < e
fn select_363([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_42([a, c, b, d, e, h])
    } else {
        select_29([b, c, a, d, e, f, g])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < e, c < g, c < h, e < f
fn select_364([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_216([d, e, b, g, h, f, a])
    } else {
        select_363([a, b, c, e, f, g, h, d])
    }
}
/// n = 10, i = 2, a < d, a < j, b < c, b < e, b < f, b < j, e < i, f < g, f < h
fn select_365([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_173([c, e, f, a, g, h, i])
    } else {
        select_244([a, b, c, d, e, i, f, j])
    }
}
/// n = 12, i = 3, a < e, a < g, b < f, b < h, c < d, c < f, c < g, c < h, f < k, g < l, h < i, h < j
fn select_366([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_364([e, b, c, d, g, l, f, h])
    } else {
        select_365([a, c, d, e, f, h, i, j, k, g])
    }
}
/// n = 12, i = 3, a < e, b < h, b < i, c < d, c < f, c < h, c < i, f < g, h < l, i < j, i < k
fn select_367([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_362([d, f, b, g, h, a, i, j, k, l, e])
    } else {
        select_366([a, b, c, d, e, h, f, i, j, k, l, g])
    }
}
/// n = 6, i = 3, c < e, d < e, e < f
fn select_368([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_26([a, c, d, b, e, f])
    } else {
        select_26([b, c, d, a, e, f])
    }
}
/// n = 10, i = 4, a < e, b < i, b < j, c < f, d < f, f < g, f < i, f < j, g < h
fn select_369([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_216([a, g, b, i, j, h, e])
    } else {
        select_368([b, e, c, d, f, g])
    }
}
/// n = 10, i = 4, a < e, b < i, b < j, c < f, d < f, f < g, f < h, f < i, f < j
fn select_370([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_369([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_369([a, b, c, d, e, f, h, g, i, j])
    }
}
/// n = 7, i = 3, b < e, c < f, d < e, d < f, f < g
fn select_371([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_46([b, c, e, f, a, g])
    } else {
        select_302([a, c, b, d, e, f])
    }
}
/// n = 7, i = 4, b < f, c < f, d < e, d < g, f < g
fn select_372([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_157([a, b, c, d, e, f, g])
    } else {
        select_26([e, b, c, a, f, g])
    }
}
/// n = 7, i = 4, b < g, c < f, d < e, d < g, f < g
fn select_373([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < f {
        select_372([a, b, c, d, e, f, g])
    } else {
        select_53([a, f, d, e, b, g])
    }
}
/// n = 9, i = 5, b < g, c < h, d < g, d < h, e < f, f < h, h < i
fn select_374([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_371([a, b, c, d, g, h, i])
    } else {
        select_373([a, c, e, d, g, f, h])
    }
}
/// n = 11, i = 5, a < g, b < j, b < k, c < h, d < g, d < h, e < f, e < h, h < i, h < j, h < k
fn select_375([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_370([a, b, c, e, g, h, f, i, j, k])
    } else {
        select_374([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 14, i = 5, a < g, b < f, c < j, c < k, d < h, e < g, e < h, h < i, h < j, h < k, j < n, k < l, k < m
fn select_376([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < b {
        select_367([a, c, h, i, g, b, f, j, k, l, m, n])
    } else {
        select_375([a, c, d, e, b, f, g, h, i, j, k])
    }
}
/// n = 14, i = 5, c < g, d < j, d < k, e < h, f < g, f < h, h < i, h < j, h < k, j < n, k < l, k < m
fn select_377([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_376([c, a, d, e, f, b, g, h, i, j, k, l, m, n])
    } else {
        select_376([c, b, d, e, f, a, g, h, i, j, k, l, m, n])
    }
}
/// n = 15, i = 6, c < k, c < l, d < i, e < h, e < i, f < g, f < i, i < j, i < k, i < l, k < o, l < m, l < n
fn select_378([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < h {
        select_377([a, b, g, c, d, e, h, i, j, k, l, m, n, o])
    } else {
        select_377([a, b, h, c, d, f, g, i, j, k, l, m, n, o])
    }
}
/// n = 15, i = 6, c < i, c < n, d < h, d < j, e < i, e < j, f < g, f < i, f < j, i < l, i < m, j < k, j < n, n < o
fn select_379([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < j {
        select_360([a, b, d, c, e, f, g, h, i, l, m, j, k, n])
    } else {
        select_378([a, b, c, e, d, f, g, h, j, k, n, i, l, m, o])
    }
}
/// n = 15, i = 6, c < k, c < o, d < k, d < n, e < h, e < i, f < g, f < i, f < k, i < j, i < n, k < l, k < m, n < o
fn select_380([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < c {
        select_338([a, b, g, h, d, i, j, c, n, k, l, m, o])
    } else {
        select_379([a, b, d, e, c, f, g, h, k, i, j, l, m, n, o])
    }
}
/// n = 8, i = 2, b < h, c < d, c < e, c < f, c < h, f < g
fn select_381([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_228([a, b, c, d, f, g, e, h])
    } else {
        select_228([a, b, c, e, f, g, d, h])
    }
}
/// n = 10, i = 4, a < e, b < f, b < g, c < f, c < g, d < f, d < g, f < j, g < h, g < i
fn select_382([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_172([a, f, g, h, i, j])
    } else {
        select_141([a, b, c, d, e, f, j])
    }
}
/// n = 10, i = 4, c < f, c < g, d < f, d < g, e < f, e < g, f < j, g < h, g < i
fn select_383([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_382([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_382([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 11, i = 5, c < g, c < h, d < g, d < h, e < g, e < h, f < g, f < h, g < k, h < i, h < j
fn select_384([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_383([a, b, d, e, f, g, h, i, j, k])
    } else {
        select_383([a, b, c, e, f, g, h, i, j, k])
    }
}
/// n = 12, i = 5, b < l, c < j, d < g, d < j, e < g, e < j, f < g, f < j, g < h, g < i, g < l, j < k
fn select_385([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < c {
        select_381([a, b, g, h, i, c, j, l])
    } else {
        select_384([a, b, c, d, e, f, j, g, h, i, k])
    }
}
/// n = 8, i = 5, a < f, b < g, c < g, d < g, e < g, g < h
fn select_386([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < f {
        select_141([a, c, d, e, f, g, h])
    } else {
        select_8([b, c, d, e])
    }
}
/// n = 8, i = 5, c < g, d < g, e < g, f < g, g < h
fn select_387([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_386([a, c, d, e, f, b, g, h])
    } else {
        select_386([b, c, d, e, f, a, g, h])
    }
}
/// n = 7, i = 5, c < g, d < g, e < f
fn select_388([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_182([c, d, a, e, f, b, g])
    } else {
        select_182([c, d, b, e, f, a, g])
    }
}
/// n = 7, i = 5, e < g, f < g
fn select_389([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_388([c, d, e, f, a, b, g])
    } else {
        select_388([c, d, e, f, b, a, g])
    }
}
/// n = 9, i = 5, b < g, c < h, d < h, e < h, f < g, f < h, h < i
fn select_390([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_387([a, b, c, d, e, f, h, i])
    } else {
        select_389([a, c, d, e, b, f, g])
    }
}
/// n = 7, i = 4, a < g, b < g, c < f, d < f, d < g, e < f, e < g
fn select_391([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_240([c, d, e, a, b, f, g])
    } else {
        select_240([c, d, e, b, a, f, g])
    }
}
/// n = 8, i = 5, a < h, b < h, c < g, c < h, d < g, d < h, e < f, e < h
fn select_392([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_391([a, b, f, c, d, g, h])
    } else {
        select_302([a, b, g, e, f, h])
    }
}
/// n = 8, i = 5, b < h, c < h, d < h, e < g, e < h, f < g, f < h
fn select_393([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_392([c, d, e, f, b, a, g, h])
    } else {
        select_388([c, d, e, f, a, b, g])
    }
}
/// n = 11, i = 6, b < g, c < i, c < k, d < i, d < k, e < i, e < k, f < h, f < k, g < k, h < i, i < j
fn select_394([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_390([a, b, c, d, e, h, g, i, j])
    } else {
        select_393([a, c, d, e, f, g, h, k])
    }
}
/// n = 13, i = 6, b < g, c < i, c < l, d < i, d < l, e < i, e < l, f < h, f < i, h < l, i < j, i < k, l < m
fn select_395([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_385([a, b, h, c, d, e, i, j, k, l, m, g])
    } else {
        select_394([a, b, c, d, e, f, g, h, l, m, i])
    }
}
/// n = 9, i = 2, a < d, b < c, b < e, b < f, e < i, f < g, f < h
fn select_396([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < a {
        select_216([c, e, f, g, h, i, a])
    } else {
        select_230([a, b, d, e, f, g, h, i, c])
    }
}
/// n = 9, i = 2, a < f, b < c, b < d, b < e, b < g, g < h, g < i
fn select_397([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_396([a, b, e, f, c, g, h, i, d])
    } else {
        select_396([a, b, e, f, d, g, h, i, c])
    }
}
/// n = 7, i = 1, c < d, c < e, c < f, c < g
fn select_398([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_68([a, c, d, e, f, g, b])
    } else {
        select_68([b, c, d, e, f, g, a])
    }
}
/// n = 8, i = 3, a < e, a < g, a < h, b < e, b < f, b < g, b < h, c < d, c < g, c < h, d < f
fn select_399([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_19([e, c, g, h])
    } else {
        select_154([d, a, b, g, h, f])
    }
}
/// n = 8, i = 3, a < f, a < g, a < h, b < e, b < g, b < h, c < d, c < f, c < g, c < h
fn select_400([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_37([d, b, a, f, g, h, e])
    } else {
        select_399([a, c, b, e, f, d, g, h])
    }
}
/// n = 11, i = 3, a < e, a < k, b < f, b < g, b < k, c < d, c < f, c < g, c < k, g < h, g < i, g < j
fn select_401([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_398([d, f, g, a, h, i, j])
    } else {
        select_400([b, a, c, d, e, f, g, k])
    }
}
/// n = 11, i = 3, a < e, a < k, b < d, b < j, b < k, c < f, c < j, c < k, f < g, f < h, f < i
fn select_402([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_397([a, f, g, h, i, e, b, d, j])
    } else {
        select_401([a, c, b, d, e, j, f, g, h, i, k])
    }
}
/// n = 8, i = 2, a < e, a < g, a < h, b < c, b < d, b < g, b < h, e < f
fn select_403([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_310([c, d, a, g, h])
    } else {
        select_29([b, a, d, e, f, g, h])
    }
}
/// n = 9, i = 5, a < g, b < f, c < g, c < i, d < g, d < i, e < g, e < i, f < i, g < h
fn select_404([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_141([b, c, d, e, f, g, h])
    } else {
        select_9([a, c, d, e, i])
    }
}
/// n = 11, i = 5, b < f, b < i, b < j, c < g, c < k, d < g, d < k, e < g, e < k, f < k, g < h, g < i, g < j
fn select_405([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_403([b, g, a, h, f, k, i, j])
    } else {
        select_404([a, b, c, d, e, f, g, h, k])
    }
}
/// n = 7, i = 3, a < f, b < d, b < e, c < e, c < f, f < g
fn select_406([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < d {
        select_139([b, a, c, e, d, f, g])
    } else {
        select_13([c, d, a, e])
    }
}
/// n = 10, i = 6, a < g, b < h, b < j, c < h, c < j, d < h, d < j, e < f, e < g, g < j, h < i
fn select_407([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_404([f, a, b, c, d, g, h, i, j])
    } else {
        select_406([a, h, e, i, f, g, j])
    }
}
/// n = 12, i = 6, a < l, b < h, b < l, c < h, c < l, d < h, d < l, e < f, e < g, g < j, g < k, g < l, h < i, h < j, h < k
fn select_408([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_405([f, g, b, c, d, a, h, i, j, k, l])
    } else {
        select_407([a, b, c, d, e, f, g, h, i, l])
    }
}
/// n = 6, i = 2, b < e, b < f, c < d, c < e, c < f
fn select_409([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_28([d, b, e, f, a])
    } else {
        select_154([a, b, c, e, f, d])
    }
}
/// n = 9, i = 3, a < e, a < g, a < i, b < c, b < d, b < g, b < h, d < i, e < f, e < h
fn select_410([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_409([c, d, a, g, e, i])
    } else {
        select_222([e, b, c, g, f, d, h])
    }
}
/// n = 5, i = 4
fn select_411([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_8([b, c, d, e])
    } else {
        select_8([a, c, d, e])
    }
}
/// n = 9, i = 5, a < f, b < g, c < g, d < g, e < g, g < h, g < i
fn select_412([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_19([a, g, h, i])
    } else {
        select_411([b, c, d, e, f])
    }
}
/// n = 13, i = 6, a < h, a < j, b < h, b < j, c < h, c < j, d < h, d < j, e < f, e < j, e < m, f < g, h < i, h < m, j < k, j < l
fn select_413([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_36([h, e, f, i, j, k, l, m])
    } else {
        select_412([f, a, b, c, d, g, j, k, l])
    }
}
/// n = 14, i = 6, a < k, b < h, b < k, c < h, c < k, d < h, d < k, e < f, e < j, e < k, f < g, f < l, h < i, h < j, h < l, k < m, k < n
fn select_414([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < a {
        select_410([e, h, i, a, f, g, j, l, k])
    } else {
        select_413([a, b, c, d, e, f, g, h, i, k, m, n, j])
    }
}
/// n = 14, i = 6, a < g, a < n, b < k, c < j, c < k, c < n, d < h, d < k, e < h, e < k, f < h, f < k, h < i, h < j, h < n, k < l, k < m
fn select_415([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < c {
        select_408([b, d, e, f, a, g, c, h, i, j, n, k])
    } else {
        select_414([b, d, e, f, c, a, g, h, i, j, k, n, l, m])
    }
}
/// n = 14, i = 6, b < g, b < n, c < j, c < m, c < n, d < h, d < j, e < h, e < j, f < h, f < j, h < i, h < m, h < n, j < k, j < l
fn select_416([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < a {
        select_402([b, h, c, i, g, j, a, k, l, m, n])
    } else {
        select_415([b, a, c, d, e, f, g, h, i, m, j, k, l, n])
    }
}
/// n = 14, i = 6, b < h, b < n, c < g, c < k, c < n, d < i, d < k, e < i, e < k, f < i, f < k, i < j, i < n, k < l, k < m
fn select_417([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < i {
        select_395([a, b, d, e, f, c, h, g, k, l, m, i, j])
    } else {
        select_416([a, b, c, d, e, f, h, i, j, k, l, m, g, n])
    }
}
/// n = 10, i = 3, b < e, b < h, b < j, c < d, c < f, c < h, f < g, f < j, h < i
fn select_418([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_221([a, d, f, g, b, e, h, j])
    } else {
        select_225([a, b, c, d, e, h, f, g, j, i])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < j, f < k, g < h, g < i
fn select_419([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_119([a, b, c, d, e, f, g, j, k])
    } else {
        select_119([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 11, i = 3, b < e, b < h, b < k, c < d, c < f, c < h, f < g, f < k, h < i, h < j
fn select_420([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_221([a, d, f, g, b, e, h, k])
    } else {
        select_419([a, b, c, d, e, f, h, i, j, g, k])
    }
}
/// n = 12, i = 3, b < g, b < h, c < f, c < i, d < e, d < h, d < i, h < l, i < j, i < k
fn select_421([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < c {
        select_418([a, d, b, g, e, c, f, h, l, i])
    } else {
        select_420([a, d, c, f, e, b, g, i, j, k, h])
    }
}
/// n = 8, i = 2, a < d, b < c, b < e, e < f, e < g, e < h
fn select_422([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < a {
        select_67([c, e, f, g, h, a])
    } else {
        select_70([a, b, d, e, f, g, h, c])
    }
}
/// n = 9, i = 2, a < f, b < e, b < i, c < d, c < g, c < i, g < h
fn select_423([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_422([a, b, e, f, c, d, g, i])
    } else {
        select_396([a, c, d, f, g, b, e, i, h])
    }
}
/// n = 9, i = 2, c < f, c < i, d < e, d < g, d < i, g < h
fn select_424([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_423([a, c, d, e, f, b, g, h, i])
    } else {
        select_423([b, c, d, e, f, a, g, h, i])
    }
}
/// n = 7, i = 1, a < f, a < g, b < c, b < d, b < e
fn select_425([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_27([b, f, g])
    } else {
        select_33([a, c, d, e])
    }
}
/// n = 9, i = 2, a < c, b < d, b < e, d < i, e < f, e < g, e < h
fn select_426([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_425([d, e, f, g, h, a, i])
    } else {
        select_70([a, b, c, e, f, g, h, d])
    }
}
/// n = 9, i = 2, c < d, c < e, d < i, e < f, e < g, e < h
fn select_427([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_426([a, c, b, d, e, f, g, h, i])
    } else {
        select_426([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 9, i = 4, a < i, b < e, c < f, d < f, f < g, f < i, g < h
fn select_428([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_58([b, a, f, g, h, i])
    } else {
        select_12([a, b, c, d, e, f, i])
    }
}
/// n = 9, i = 4, c < i, d < f, e < f, f < g, f < i, g < h
fn select_429([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_428([c, a, d, e, b, f, g, h, i])
    } else {
        select_428([c, b, d, e, a, f, g, h, i])
    }
}
/// n = 11, i = 4, d < f, e < f, f < g, f < h, g < k, h < i, h < j
fn select_430([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_427([b, c, f, g, h, a, i, j, k])
    } else {
        select_429([b, c, a, d, e, f, g, k, h])
    }
}
/// n = 12, i = 5, c < f, c < j, c < l, d < g, e < g, g < h, g < j, h < i, h < l, j < k
fn select_431([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < c {
        select_424([a, b, h, c, f, i, j, k, l])
    } else {
        select_430([a, b, f, d, e, g, j, h, i, l, k])
    }
}
/// n = 15, i = 6, b < g, b < k, c < f, c < k, c < l, d < h, e < h, h < i, h < k, i < j, i < l, k < o, l < m, l < n
fn select_432([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < b {
        select_421([a, b, i, c, f, j, g, k, l, m, n, o])
    } else {
        select_431([a, g, c, d, e, f, h, i, j, k, o, l])
    }
}
/// n = 15, i = 6, b < h, b < n, c < j, c < k, d < g, d < k, d < n, e < i, e < k, f < i, f < k, i < j, i < n, k < l, k < m, n < o
fn select_433([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if c < i {
        select_417([a, b, d, c, e, f, g, h, i, j, k, l, m, n])
    } else {
        select_432([a, b, d, e, f, g, h, i, c, j, n, k, l, m, o])
    }
}
/// n = 7, i = 2, b < e, c < d, c < f, f < g
fn select_434([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_23([a, b, c, d, e, f])
    } else {
        select_64([b, c, d, e, f, a, g])
    }
}
/// n = 8, i = 2, b < e, c < d, c < f, f < g, f < h
fn select_435([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_23([a, b, c, d, e, f])
    } else {
        select_422([b, c, d, e, f, a, g, h])
    }
}
/// n = 8, i = 2, b < f, c < d, c < e, c < g, g < h
fn select_436([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_434([a, b, c, e, f, d, g])
    } else {
        select_435([a, b, c, e, f, g, d, h])
    }
}
/// n = 9, i = 3, a < d, a < f, b < e, b < g, c < e, c < f, c < g, g < h, g < i
fn select_437([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_15([b, c, a, d, f, e])
    } else {
        select_64([a, b, e, f, g, h, i])
    }
}
/// n = 10, i = 3, a < e, a < j, b < d, b < f, b < j, c < e, c < f, c < j, e < i, f < g, f < h
fn select_438([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_83([a, b, e, f, d, g, h, i, j])
    } else {
        select_80([a, b, c, d, e, i, f, j])
    }
}
/// n = 11, i = 4, a < g, a < h, a < k, b < d, b < g, b < h, c < e, c < h, c < k, d < k, e < f, h < i, h < j
fn select_439([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < d {
        select_437([e, a, b, f, g, d, h, i, j])
    } else {
        select_438([d, a, c, g, e, h, i, j, f, k])
    }
}
/// n = 11, i = 4, a < g, a < k, b < d, b < g, b < h, c < e, c < h, c < k, d < k, e < f, h < i, h < j
fn select_440([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_436([d, e, h, i, j, f, a, g])
    } else {
        select_439([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 9, i = 4, a < e, a < g, b < e, b < h, c < d, d < g, d < h, e < f, h < i
fn select_441([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_42([a, b, g, h, i, e])
    } else {
        select_3([f, c, d])
    }
}
/// n = 10, i = 4, a < f, a < g, b < f, b < i, c < d, d < e, d < g, d < i, f < h, i < j
fn select_442([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_91([e, a, b, g, i, j])
    } else {
        select_441([a, b, c, d, f, h, g, i, j])
    }
}
/// n = 8, i = 4, a < f, b < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_443([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_240([a, b, c, d, e, f, g])
    } else {
        select_46([a, d, f, g, e, h])
    }
}
/// n = 8, i = 5, a < h, b < g, b < h, c < g, c < h, d < e, e < f
fn select_444([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_240([a, b, c, e, f, h, g])
    } else {
        select_124([g, a, d, h, e, f])
    }
}
/// n = 10, i = 5, a < g, a < h, b < h, b < i, c < g, c < h, c < i, d < e, e < f, e < i, i < j
fn select_445([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < g {
        select_443([a, b, c, e, f, h, i, j])
    } else {
        select_444([b, a, c, d, e, f, g, h])
    }
}
/// n = 12, i = 6, a < h, a < k, b < e, b < k, c < h, c < i, c < k, d < f, e < i, f < g, f < j, f < k, h < j, k < l
fn select_446([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_442([h, b, d, f, g, e, j, i, k, l])
    } else {
        select_445([e, a, c, d, f, g, i, h, k, l])
    }
}
/// n = 8, i = 3, a < f, a < h, b < d, b < e, b < h, c < e, c < f, c < h, f < g
fn select_447([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_209([a, b, e, f, d, g, h])
    } else {
        select_80([a, b, c, d, f, g, e, h])
    }
}
/// n = 9, i = 3, a < f, a < i, b < f, b < g, b < i, c < d, c < e, c < i, e < g, f < h
fn select_448([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_82([d, b, e, a, g])
    } else {
        select_447([b, c, a, d, e, f, h, i])
    }
}
/// n = 8, i = 4, a < f, a < h, b < f, b < h, c < d, d < e, d < h, f < g
fn select_449([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_25([f, c, g, d, h])
    } else {
        select_87([e, a, b, h])
    }
}
/// n = 8, i = 4, a < e, a < h, b < e, b < g, c < d, d < g, d < h, e < f
fn select_450([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_4([a, b, g, h, e])
    } else {
        select_3([f, c, d])
    }
}
/// n = 10, i = 5, a < f, b < e, b < j, c < d, c < g, d < i, d < j, e < h, e < i, f < g, f < h, f < j
fn select_451([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_449([b, d, a, f, g, e, i, j])
    } else {
        select_450([c, e, a, f, d, i, h, g])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, c < d, c < f, d < h, e < f, e < g
fn select_452([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < h {
        select_15([a, e, c, d, f, g])
    } else {
        select_6([b, h])
    }
}
/// n = 10, i = 5, a < d, a < h, b < f, c < e, c < j, d < j, e < g, e < i, f < h, f < i, f < j
fn select_453([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < g {
        select_451([b, c, a, d, e, f, h, i, g, j])
    } else {
        select_452([a, b, e, g, f, i, h, d])
    }
}
/// n = 11, i = 5, a < d, a < i, a < j, b < e, b < j, b < k, c < f, d < k, e < g, f < h, f < i, f < k, h < j
fn select_454([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_448([h, b, a, i, d, e, k, g, j])
    } else {
        select_453([a, c, b, d, e, f, g, i, h, k])
    }
}
/// n = 13, i = 6, a < k, b < e, b < k, b < l, c < g, d < f, d < i, d < k, e < i, f < j, f < l, g < h, g < j, g < k, h < l, k < m
fn select_455([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < f {
        select_446([a, b, d, c, e, g, h, f, i, j, k, m])
    } else {
        select_454([f, b, c, a, e, g, i, h, j, l, k])
    }
}
/// n = 13, i = 6, b < e, b < k, b < m, c < g, d < f, d < i, d < k, e < i, f < j, f < m, g < h, g < j, g < k, h < m, k < l
fn select_456([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if k < a {
        select_440([f, g, b, h, e, i, j, k, a, l, m])
    } else {
        select_455([a, b, c, d, e, f, g, h, i, j, k, m, l])
    }
}
/// n = 7, i = 3, a < e, a < g, b < e, b < f, c < d, d < f, d < g
fn select_457([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_20([a, b, f, g])
    } else {
        select_25([b, c, e, d, g])
    }
}
/// n = 8, i = 3, a < e, b < f, b < g, b < h, c < d, c < f, d < g, e < h
fn select_458([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_22([a, d, b, f, g, e])
    } else {
        select_457([b, c, a, e, f, d, h])
    }
}
/// n = 9, i = 3, a < e, a < f, b < d, b < g, c < f, c < g, c < i, e < i, g < h
fn select_459([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_150([a, c, b, d, f, g, h])
    } else {
        select_132([b, c, a, d, e, f, i])
    }
}
/// n = 9, i = 3, b < g, c < e, c < f, d < f, d < g, d < h, e < h, g < i
fn select_460([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_458([a, d, c, e, b, f, h, g])
    } else {
        select_459([c, b, d, a, e, f, g, i, h])
    }
}
/// n = 7, i = 2, a < g, b < f, b < g, c < d, c < f, d < e
fn select_461([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < b {
        select_3([a, d, e])
    } else {
        select_22([a, b, c, d, f, g])
    }
}
/// n = 8, i = 2, a < h, b < d, c < e, c < h, e < f, e < g
fn select_462([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_67([a, e, b, f, g, h])
    } else {
        select_22([a, c, b, d, e, h])
    }
}
/// n = 10, i = 3, a < e, b < f, c < e, c < g, d < f, d < g, f < j, g < h, g < i
fn select_463([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_461([a, c, d, f, j, g, e])
    } else {
        select_462([b, a, d, e, g, h, i, f])
    }
}
/// n = 9, i = 3, a < f, b < f, b < g, b < h, c < d, d < e, d < g, d < h, f < i
fn select_464([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < b {
        select_310([a, e, b, g, h])
    } else {
        select_89([a, c, f, d, e, g, h, i])
    }
}
/// n = 11, i = 3, a < f, b < f, b < g, b < j, c < d, c < g, d < e, d < j, f < k, g < h, g < i
fn select_465([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_89([a, c, f, g, d, h, i, k])
    } else {
        select_464([a, b, c, d, e, f, g, j, k])
    }
}
/// n = 11, i = 3, b < f, c < e, c < g, d < f, d < g, d < j, e < j, f < k, g < h, g < i
fn select_466([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < e {
        select_463([a, b, c, d, e, f, g, h, i, k])
    } else {
        select_465([b, d, c, e, a, f, g, h, i, j, k])
    }
}
/// n = 11, i = 3, b < f, c < g, d < h, e < f, e < g, e < h, g < k, h < i, h < j
fn select_467([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < d {
        select_460([a, c, b, e, d, f, g, h, k])
    } else {
        select_466([a, c, d, e, b, g, h, i, j, f, k])
    }
}
/// n = 10, i = 4, a < e, a < h, b < e, b < g, c < d, c < i, c < j, d < g, d < h, e < f, e < i, e < j
fn select_468([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_34([f, c, d, i, j])
    } else {
        select_4([a, b, g, h, e])
    }
}
/// n = 11, i = 4, a < e, a < h, a < i, b < e, b < g, b < i, c < d, c < i, c < k, d < g, d < h, e < f, e < k, i < j
fn select_469([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < e {
        select_256([d, a, b, g, h, i, e, j])
    } else {
        select_468([a, b, c, d, e, f, g, h, i, k])
    }
}
/// n = 7, i = 3, a < e, a < g, b < e, b < f, b < g, c < d, c < g, d < f
fn select_470([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_3([a, b, f])
    } else {
        select_4([b, c, d, e, g])
    }
}
/// n = 9, i = 3, a < e, a < i, b < e, b < f, b < i, c < d, c < f, c < i, f < g, f < h
fn select_471([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_70([a, c, e, f, d, g, h, i])
    } else {
        select_470([a, b, c, d, e, f, i])
    }
}
/// n = 11, i = 4, a < f, a < g, a < h, b < e, b < h, b < k, c < d, c < f, c < h, e < g, f < k, h < i, h < j
fn select_472([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_311([d, f, b, h, i, j, e, k])
    } else {
        select_471([e, a, c, d, g, h, i, j, f])
    }
}
/// n = 14, i = 5, a < f, a < i, a < j, b < e, b < i, b < k, c < d, c < j, c < k, d < h, d < i, e < h, e < j, f < g, f < k, j < n, k < l, k < m
fn select_473([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < f {
        select_469([a, e, c, d, f, g, h, i, j, n, k])
    } else {
        select_472([b, c, f, g, d, e, i, k, l, m, j])
    }
}
/// n = 7, i = 3, a < d, b < c, b < g, c < f, d < e, d < g
fn select_474([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_3([f, a, d])
    } else {
        select_19([e, b, c, g])
    }
}
/// n = 10, i = 5, a < f, b < e, c < d, c < j, d < h, d < i, e < g, e < h, e < j, f < g, f < i, f < j
fn select_475([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_474([b, c, d, e, g, i, j])
    } else {
        select_474([a, c, d, f, g, h, j])
    }
}
/// n = 8, i = 4, a < f, a < g, a < h, b < e, c < d, c < h, d < g, e < f, e < h
fn select_476([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_4([a, c, d, f, h])
    } else {
        select_13([a, b, e, g])
    }
}
/// n = 12, i = 6, a < g, a < j, a < k, b < g, b < h, c < f, d < e, d < k, d < l, e < i, e < j, f < h, f < j, f < l, g < i, g < l, h < k
fn select_477([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_475([b, c, d, e, f, g, h, j, i, l])
    } else {
        select_476([a, b, d, e, h, g, j, k])
    }
}
/// n = 9, i = 3, a < d, a < g, b < c, b < g, b < i, c < f, d < e, d < i, g < h
fn select_478([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_42([c, a, f, g, h, d])
    } else {
        select_29([d, b, e, g, h, c, i])
    }
}
/// n = 7, i = 3, a < e, b < c, b < d, b < g, e < f, e < g
fn select_479([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_474([a, b, c, e, f, d, g])
    } else {
        select_474([a, b, d, e, f, c, g])
    }
}
/// n = 11, i = 5, a < f, b < e, b < j, c < d, c < g, c < j, d < i, e < g, e < h, f < h, f < i, f < j, j < k
fn select_480([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_478([b, c, d, e, h, i, j, k, g])
    } else {
        select_479([a, c, d, g, f, h, j])
    }
}
/// n = 13, i = 6, a < h, b < g, c < f, c < j, c < m, d < e, d < k, d < m, e < i, e < j, f < i, f < k, g < h, g < j, g < k, h < m, k < l
fn select_481([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < f {
        select_477([c, a, b, d, e, g, f, h, i, j, m, k])
    } else {
        select_480([b, d, f, a, e, g, i, j, h, k, l])
    }
}
/// n = 15, i = 6, b < g, c < f, c < j, c < l, d < e, d < k, d < l, e < i, e < j, f < i, f < k, g < h, g < j, g < k, h < l, k < o, l < m, l < n
fn select_482([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < a {
        select_473([g, c, d, e, f, h, a, i, j, k, l, m, n, o])
    } else {
        select_481([a, b, c, d, e, f, g, h, i, j, k, o, l])
    }
}
/// n = 15, i = 6, b < e, b < k, b < l, c < g, d < f, d < i, d < l, e < j, f < j, f < k, g < h, g < i, g < k, h < l, k < o, l < m, l < n
fn select_483([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < e {
        select_467([a, i, f, h, b, e, k, l, m, n, o])
    } else {
        select_482([a, c, d, b, e, f, g, h, j, i, k, l, m, n, o])
    }
}
/// n = 15, i = 6, b < j, b < k, c < i, c < m, d < f, d < k, d < m, e < g, f < j, g < h, g < i, g < k, h < m, k < l, m < n, m < o
fn select_484([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if b < c {
        select_456([a, d, e, b, f, c, g, h, j, i, k, l, m])
    } else {
        select_483([a, d, e, c, f, b, g, h, i, j, k, m, n, o, l])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < g, c < e, c < f, d < e, d < f, d < g
fn select_485([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_11([b, c, d, f, g])
    } else {
        select_11([a, c, d, e, g])
    }
}
/// n = 9, i = 4, a < g, a < h, a < i, b < f, b < i, c < f, c < h, d < e, e < h, e < i, f < g
fn select_486([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_485([a, b, c, e, f, h, i])
    } else {
        select_11([a, d, f, e, g])
    }
}
/// n = 9, i = 5, a < g, b < e, c < g, c < i, d < f, e < i, f < h, f < i, g < h
fn select_487([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_240([a, c, e, d, f, g, i])
    } else {
        select_52([b, g, d, f, e, h])
    }
}
/// n = 8, i = 4, a < g, a < h, b < f, c < e, d < f, d < h, e < h, f < g
fn select_488([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_153([a, b, d, e, f, h])
    } else {
        select_11([a, c, f, e, g])
    }
}
/// n = 9, i = 5, a < g, a < i, b < g, b < h, c < e, d < f, e < i, f < h, f < i
fn select_489([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < h {
        select_487([b, c, a, d, e, f, g, h, i])
    } else {
        select_488([a, b, c, f, e, h, g, i])
    }
}
/// n = 11, i = 6, a < i, a < j, a < k, b < f, c < e, c < k, d < g, e < i, f < j, f < k, g < h, g < k, h < j
fn select_490([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_486([a, c, h, b, f, e, i, j, k])
    } else {
        select_489([a, e, b, d, f, g, i, h, k])
    }
}
/// n = 7, i = 3, b < g, c < e, c < g, d < e, d < g, e < f
fn select_491([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_28([b, e, a, f, g])
    } else {
        select_153([a, b, c, d, g, e])
    }
}
/// n = 8, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_492([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_491([a, e, b, c, g, h, f])
    } else {
        select_58([a, f, d, g, h, e])
    }
}
/// n = 9, i = 4, a < i, b < f, c < f, c < g, d < e, d < g, g < h, g < i
fn select_493([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < b {
        select_59([e, a, g, h, b, f, i])
    } else {
        select_492([a, b, c, d, e, f, g, h])
    }
}
/// n = 12, i = 6, a < f, b < i, b < j, b < l, c < e, c < j, d < g, e < i, f < l, g < h, g < j, h < l, j < k
fn select_494([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < j {
        select_490([b, a, c, d, e, f, g, h, i, l, j])
    } else {
        select_493([a, e, b, g, h, i, j, k, f])
    }
}
/// n = 9, i = 3, a < d, b < h, b < i, c < e, e < f, e < h, e < i, f < g
fn select_495([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_216([a, f, b, h, i, g, d])
    } else {
        select_88([b, d, c, e, f])
    }
}
/// n = 9, i = 3, a < d, b < h, b < i, c < e, e < f, e < g, e < h, e < i
fn select_496([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_495([a, b, c, d, e, f, g, h, i])
    } else {
        select_495([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 8, i = 4, a < d, a < f, b < f, b < g, c < e, e < g, g < h
fn select_497([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_139([a, b, e, d, f, g, h])
    } else {
        select_162([c, d, b, f, e])
    }
}
/// n = 7, i = 2, a < c, b < d, c < g, d < e, d < f
fn select_498([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_27([a, e, f])
    } else {
        select_20([b, c, g, d])
    }
}
/// n = 8, i = 4, a < d, b < f, b < g, c < e, d < f, e < g, g < h
fn select_499([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_25([a, b, d, g, h])
    } else {
        select_13([b, c, e, f])
    }
}
/// n = 12, i = 5, a < f, a < h, b < f, b < i, c < g, c < h, c < i, d < e, e < h, e < i, f < g, h < l, i < j, i < k
fn select_500([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_498([a, b, h, i, j, k, l])
    } else {
        select_499([b, c, d, f, e, g, h, l])
    }
}
/// n = 12, i = 5, a < i, b < e, b < h, c < g, c < h, c < i, d < f, e < g, f < h, f < i, h < j, i < k, i < l
fn select_501([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_497([e, c, d, a, f, g, h, j])
    } else {
        select_500([b, a, c, d, f, e, g, h, i, k, l, j])
    }
}
/// n = 14, i = 5, a < j, a < m, a < n, b < e, b < h, c < g, c < h, c < j, d < f, d < h, e < g, f < j, h < i, h < m, h < n, j < k, j < l
fn select_502([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < f {
        select_496([e, a, d, g, h, f, i, m, n])
    } else {
        select_501([a, b, c, d, e, f, g, h, j, i, k, l])
    }
}
/// n = 8, i = 3, a < h, b < h, c < e, d < e, d < h, e < f, e < g
fn select_503([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_34([b, e, a, f, g])
    } else {
        select_153([b, c, a, d, e, h])
    }
}
/// n = 11, i = 4, a < f, a < k, b < g, b < j, b < k, c < f, c < g, d < e, d < g, e < k, f < j, g < h, g < i
fn select_504([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_488([b, c, d, a, e, f, j, k])
    } else {
        select_503([a, e, c, b, g, h, i, k])
    }
}
/// n = 13, i = 6, a < f, a < m, b < i, b < j, b < m, c < e, c < j, d < g, e < m, f < i, g < h, g < m, h < j, j < k, j < l
fn select_505([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < h {
        select_489([b, f, c, d, e, g, i, h, m])
    } else {
        select_504([a, b, h, c, e, f, j, k, l, i, m])
    }
}
/// n = 15, i = 6, a < f, a < j, b < e, b < l, b < o, c < i, c < j, c < l, d < g, f < i, g < h, g < j, h < l, j < k, j < o, l < m, l < n
fn select_506([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < e {
        select_502([b, a, c, g, f, h, i, j, k, l, m, n, e, o])
    } else {
        select_505([a, c, b, d, e, f, g, h, i, l, m, n, j])
    }
}
/// n = 15, i = 6, b < l, b < o, c < f, c < j, d < i, d < j, d < l, e < g, f < i, g < h, g < j, h < l, j < k, j < o, l < m, l < n
fn select_507([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < b {
        select_494([a, d, c, e, f, b, g, h, i, j, k, l])
    } else {
        select_506([c, b, d, e, a, f, g, h, i, j, k, l, m, n, o])
    }
}
/// n = 7, i = 2, a < f, a < g, b < e, b < g, c < d, c < e, c < f
fn select_508([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_42([c, a, d, b, g, f])
    } else {
        select_42([c, b, d, a, g, e])
    }
}
/// n = 8, i = 3, b < g, b < h, c < f, c < h, d < e, d < h, e < f, e < g
fn select_509([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_302([b, c, a, d, e, h])
    } else {
        select_508([b, c, e, a, f, g, h])
    }
}
/// n = 9, i = 4, b < h, b < i, c < f, c < i, d < e, d < h, e < g, e < i, f < g, f < h
fn select_510([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_491([a, b, c, e, f, g, i])
    } else {
        select_491([a, b, d, f, e, g, h])
    }
}
/// n = 10, i = 4, b < g, b < j, c < e, c < j, d < f, d < g, e < i, f < h, f < i, f < j, g < h
fn select_511([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_509([a, c, f, b, g, h, e, j])
    } else {
        select_510([a, b, c, d, f, e, i, g, j])
    }
}
/// n = 6, i = 2, a < f, b < f, c < d, d < e
fn select_512([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < a {
        select_19([b, d, a, e])
    } else {
        select_11([b, c, a, d, f])
    }
}
/// n = 10, i = 3, a < f, a < j, b < d, b < e, c < e, c < f, d < j, e < g, f < h, f < i
fn select_513([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_512([a, d, c, e, g, j])
    } else {
        select_498([b, a, e, f, h, i, g])
    }
}
/// n = 7, i = 3, a < d, b < e, c < e, e < f, e < g
fn select_514([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_27([a, f, g])
    } else {
        select_87([d, b, c, e])
    }
}
/// n = 10, i = 4, a < e, a < f, b < g, b < j, c < e, c < g, d < f, d < g, f < j, g < h, g < i
fn select_515([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_124([b, c, d, e, f, j])
    } else {
        select_514([a, b, d, e, g, h, i])
    }
}
/// n = 11, i = 4, a < f, b < f, b < h, c < g, c < h, d < e, d < h, e < g, g < i, h < j, h < k
fn select_516([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_513([b, e, c, a, g, h, i, j, k, f])
    } else {
        select_515([a, c, b, d, f, e, h, j, k, g])
    }
}
/// n = 8, i = 3, a < d, b < e, c < e, e < f, e < g, e < h
fn select_517([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_33([a, f, g, h])
    } else {
        select_87([d, b, c, e])
    }
}
/// n = 8, i = 3, a < c, a < h, b < d, c < e, d < f, d < h, f < g
fn select_518([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < c {
        select_20([a, f, g, h])
    } else {
        select_25([c, b, e, d, h])
    }
}
/// n = 10, i = 4, a < g, a < j, b < d, b < j, c < e, d < f, d < h, e < g, e < h, e < j, g < i
fn select_519([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_88([a, f, c, e, h])
    } else {
        select_518([b, c, d, e, f, g, i, j])
    }
}
/// n = 12, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < f, d < i, e < g, e < i, g < j, h < k, h < l
fn select_520([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_517([d, a, c, f, h, e, k, l])
    } else {
        select_519([a, b, c, d, e, f, g, i, j, h])
    }
}
/// n = 12, i = 4, b < g, b < h, c < e, c < h, d < f, d < h, e < j, f < g, f < j, g < i, h < k, h < l
fn select_521([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < e {
        select_516([a, c, b, d, f, e, g, h, i, k, l])
    } else {
        select_520([b, c, d, e, f, a, g, h, j, i, k, l])
    }
}
/// n = 12, i = 4, b < g, b < l, c < h, c < k, c < l, d < g, d < h, e < f, e < h, f < l, g < k, h < i, h < j
fn select_522([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < c {
        select_511([a, d, e, b, f, c, g, k, l, h])
    } else {
        select_521([a, d, e, c, f, b, g, h, k, l, i, j])
    }
}
/// n = 8, i = 5, a < g, b < g, b < h, c < g, c < h, d < f, e < f, f < h
fn select_523([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_156([b, c, d, e, f, g])
    } else {
        select_87([a, b, c, h])
    }
}
/// n = 8, i = 5, b < g, c < g, d < f, d < h, e < f, e < h, g < h
fn select_524([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_523([a, d, e, b, c, g, f, h])
    } else {
        select_26([f, b, c, a, g, h])
    }
}
/// n = 10, i = 6, b < h, c < h, d < g, d < j, e < f, e < i, f < j, g < i, h < i, h < j
fn select_525([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_524([a, b, c, d, f, g, h, j])
    } else {
        select_524([a, b, c, e, g, f, h, i])
    }
}
/// n = 7, i = 4, c < g, d < f, e < f, f < g
fn select_526([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_12([c, a, d, e, b, f, g])
    } else {
        select_12([c, b, d, e, a, f, g])
    }
}
/// n = 10, i = 6, b < h, c < h, c < i, d < g, d < j, e < f, e < i, f < j, g < i, h < j
fn select_527([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_525([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_526([a, f, b, c, g, i, h])
    }
}
/// n = 8, i = 5, a < g, b < g, b < h, c < f, d < e, e < h, f < h
fn select_528([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_157([a, c, e, b, g, f, h])
    } else {
        select_157([a, d, f, b, g, e, h])
    }
}
/// n = 8, i = 5, a < g, b < e, c < g, c < h, d < f, f < h
fn select_529([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < h {
        select_528([a, c, b, d, f, e, g, h])
    } else {
        select_124([b, a, f, g, h, e])
    }
}
/// n = 8, i = 5, c < g, d < g, d < h, e < f, f < h
fn select_530([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_529([c, a, d, e, b, f, g, h])
    } else {
        select_529([c, b, d, e, a, f, g, h])
    }
}
/// n = 12, i = 7, b < i, c < h, d < g, d < l, e < f, e < k, f < j, f < l, g < j, g < k, h < i, h < k, i < l
fn select_531([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < j {
        select_527([a, b, h, d, e, f, g, i, k, l])
    } else {
        select_530([a, b, f, g, c, h, j, k])
    }
}
/// n = 15, i = 7, b < f, b < o, c < i, c < l, d < h, e < g, e < j, f < k, g < k, g < l, g < o, h < i, h < j, i < o, j < l, l < m, l < n
fn select_532([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < f {
        select_522([a, b, g, j, c, i, f, l, m, n, k, o])
    } else {
        select_531([a, c, d, b, e, g, f, h, i, k, j, o])
    }
}
/// n = 16, i = 7, b < i, b < n, c < f, c < l, d < h, e < g, e < j, f < k, g < k, g < l, g < n, h < i, h < j, h < l, j < n, l < m, n < o, n < p
fn select_533([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < i {
        select_507([a, b, c, g, d, f, h, j, k, l, m, n, o, p, i])
    } else {
        select_532([a, c, b, d, e, f, g, h, i, j, k, n, o, p, l])
    }
}
/// n = 16, i = 7, b < f, b < l, c < j, c < n, d < h, e < g, e < k, g < i, g < l, g < n, h < j, h < k, h < l, k < n, l < m, n < o, n < p
fn select_534([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < f {
        select_484([a, b, c, g, d, i, h, k, j, f, l, m, n, o, p])
    } else {
        select_533([a, c, b, d, e, f, g, h, j, k, i, l, m, n, o, p])
    }
}
/// n = 16, i = 7, b < f, b < l, c < k, c < m, d < h, d < m, e < g, e < i, g < m, h < k, h < l, i < j, i < l, i < m, l < p, m < n, m < o
fn select_535([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < h {
        select_433([a, b, c, i, d, g, j, f, h, k, m, n, o, l, p])
    } else {
        select_534([a, b, c, d, e, f, i, h, j, k, g, l, p, m, n, o])
    }
}
/// n = 10, i = 2, d < g, e < f, e < h, h < i, h < j
fn select_536([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_343([c, a, d, e, f, g, b, h, i, j])
    } else {
        select_343([c, b, d, e, f, g, a, h, i, j])
    }
}
/// n = 10, i = 2, f < g, f < h, h < i, h < j
fn select_537([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_536([c, d, e, a, f, g, b, h, i, j])
    } else {
        select_536([c, d, e, b, f, g, a, h, i, j])
    }
}
/// n = 10, i = 3, b < e, b < f, b < j, c < d, c < f, c < i, d < j, e < i, f < g, f < h
fn select_538([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_311([a, d, b, f, g, h, e, j])
    } else {
        select_311([a, e, c, f, g, h, d, i])
    }
}
/// n = 11, i = 3, b < f, c < g, c < j, d < e, d < f, d < g, e < j, f < k, g < h, g < i
fn select_539([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_215([a, b, d, e, c, f, k, g])
    } else {
        select_538([a, c, d, e, b, g, h, i, f, j])
    }
}
/// n = 11, i = 3, b < g, c < f, c < h, d < e, d < g, d < h, g < k, h < i, h < j
fn select_540([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_466([a, b, c, d, f, g, h, i, j, e, k])
    } else {
        select_539([a, b, c, d, e, g, h, i, j, f, k])
    }
}
/// n = 11, i = 3, b < e, b < i, c < d, c < f, c < i, f < g, f < h, i < j, i < k
fn select_541([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_117([a, d, f, g, h, b, e, i])
    } else {
        select_419([a, b, c, d, e, f, i, j, k, g, h])
    }
}
/// n = 11, i = 3, c < f, c < i, d < e, d < g, d < i, g < h, i < j, i < k
fn select_542([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_540([b, a, c, d, e, f, g, i, j, k, h])
    } else {
        select_541([b, c, d, e, f, g, a, h, i, j, k])
    }
}
/// n = 11, i = 3, c < h, c < i, d < e, d < f, d < g, d < i, i < j, i < k
fn select_543([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_542([a, b, c, d, g, h, e, f, i, j, k])
    } else {
        select_542([a, b, c, d, g, h, f, e, i, j, k])
    }
}
/// n = 11, i = 3, c < h, c < k, d < e, d < f, d < g, d < i, i < j, i < k
fn select_544([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < c {
        select_537([a, b, e, f, g, i, j, c, h, k])
    } else {
        select_543([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, c < g, c < h, h < i, h < j
fn select_545([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_179([a, b, c, f, g, d, h, i, j, e])
    } else {
        select_179([a, b, c, f, g, e, h, i, j, d])
    }
}
/// n = 8, i = 4, a < f, b < e, c < g, d < f, d < g, g < h
fn select_546([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_10([a, b, c, d, e, g, h, f])
    } else {
        select_12([c, b, a, d, e, f, g])
    }
}
/// n = 8, i = 2, a < c, b < d, d < e, d < f, d < g, d < h
fn select_547([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_33([e, f, g, h])
    } else {
        select_69([a, b, c, d, f, g, h])
    }
}
/// n = 10, i = 3, a < d, a < g, b < c, b < g, c < e, d < f, g < h, g < i, g < j
fn select_548([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_70([c, a, e, g, h, i, j, d])
    } else {
        select_70([d, b, f, g, h, i, j, c])
    }
}
/// n = 11, i = 3, a < d, a < g, b < c, b < g, c < e, d < f, g < h, g < i, g < j, g < k
fn select_549([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < c {
        select_547([d, b, f, g, h, i, j, k])
    } else {
        select_548([a, b, c, d, e, f, g, i, j, k])
    }
}
/// n = 11, i = 3, a < d, a < e, b < d, b < f, c < e, c < f, e < k, f < g, f < h, f < i, f < j
fn select_550([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_189([a, g, h, i, j])
    } else {
        select_42([b, c, d, e, k, f])
    }
}
/// n = 11, i = 3, a < f, b < f, b < g, c < d, c < g, d < e, g < h, g < i, g < j, g < k
fn select_551([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_195([d, b, a, e, g, h, i, j, k])
    } else {
        select_550([a, b, c, f, d, g, h, i, j, k, e])
    }
}
/// n = 11, i = 3, b < d, b < g, c < e, c < g, e < f, g < h, g < i, g < j, g < k
fn select_552([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_549([b, c, e, d, f, a, g, h, i, j, k])
    } else {
        select_551([a, b, c, e, f, d, g, h, i, j, k])
    }
}
/// n = 13, i = 4, a < e, b < f, b < i, c < g, c < i, d < f, d < g, d < i, g < h, i < j, i < k, i < l, i < m
fn select_553([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < i {
        select_546([b, a, c, d, e, f, g, h])
    } else {
        select_552([a, b, c, f, g, h, i, j, k, l, m])
    }
}
/// n = 8, i = 4, a < h, b < f, c < e, d < f, d < h, e < h, f < g
fn select_554([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < g {
        select_488([a, b, c, d, e, f, g, h])
    } else {
        select_25([c, f, e, g, a])
    }
}
/// n = 9, i = 5, a < g, b < f, c < h, d < g, d < h, e < f, e < h, h < i
fn select_555([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_554([a, c, b, d, f, h, i, g])
    } else {
        select_554([b, c, a, e, g, h, i, f])
    }
}
/// n = 14, i = 5, a < g, b < f, b < j, c < h, c < j, d < g, d < h, d < j, e < f, e < h, e < j, h < i, j < k, j < l, j < m, j < n
fn select_556([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < j {
        select_555([a, b, c, d, e, f, g, h, i])
    } else {
        select_198([a, b, c, d, f, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 5, b < g, b < j, c < h, c < j, d < g, d < h, d < j, e < f, e < h, e < j, h < i, j < k, j < l, j < m, j < n
fn select_557([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < a {
        select_553([f, b, c, d, a, g, h, i, j, k, l, m, n])
    } else {
        select_556([a, b, c, e, d, g, f, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 5, b < g, b < h, c < g, c < i, d < h, d < i, e < f, e < h, e < i, h < n, i < j, i < k, i < l, i < m
fn select_558([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < b {
        select_545([a, f, i, j, k, l, m, b, g, h])
    } else {
        select_557([a, c, d, b, e, f, g, h, n, i, j, k, l, m])
    }
}
/// n = 9, i = 6, a < h, b < h, c < h, d < g, d < i, e < f, f < g, h < i
fn select_559([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_162([d, h, e, f, i])
    } else {
        select_8([a, b, c, g])
    }
}
/// n = 6, i = 4, a < f, b < f, c < f, d < e
fn select_560([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_8([a, b, c, e])
    } else {
        select_6([d, f])
    }
}
/// n = 9, i = 6, a < i, b < h, c < h, d < h, e < f, f < g, h < i
fn select_561([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_559([b, c, d, a, e, f, g, h, i])
    } else {
        select_560([b, c, d, g, a, h])
    }
}
/// n = 7, i = 4, c < f, d < f, e < f, f < g
fn select_562([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_141([a, c, d, e, b, f, g])
    } else {
        select_141([b, c, d, e, a, f, g])
    }
}
/// n = 11, i = 6, a < h, b < i, b < j, c < h, c < j, d < h, d < j, e < f, f < g, f < j, h < i, j < k
fn select_563([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < j {
        select_561([b, a, c, d, e, f, g, h, i])
    } else {
        select_562([a, g, b, c, d, j, k])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, b < i, c < g, c < i, d < e, e < f, e < i, g < h
fn select_564([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_457([a, g, d, e, h, f, i])
    } else {
        select_153([a, f, b, c, g, i])
    }
}
/// n = 13, i = 7, a < i, b < j, b < k, b < m, c < i, c < k, d < g, e < f, e < m, f < j, f < k, g < h, g < k, g < m, i < j, i < m, k < l
fn select_565([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < i {
        select_563([a, b, c, f, d, g, h, i, j, k, l])
    } else {
        select_564([b, e, i, d, g, h, f, j, m])
    }
}
/// n = 8, i = 3, a < c, b < d, c < f, d < e, d < g, g < h
fn select_566([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < a {
        select_19([e, g, a, h])
    } else {
        select_474([b, a, c, d, e, f, g])
    }
}
/// n = 7, i = 3, a < f, b < f, b < g, c < d, d < e, d < g
fn select_567([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_20([b, e, a, g])
    } else {
        select_457([a, b, c, d, f, g, e])
    }
}
/// n = 9, i = 4, a < d, b < g, b < h, c < e, d < g, e < f, e < h, h < i
fn select_568([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_566([a, c, d, e, f, g, h, i])
    } else {
        select_567([d, b, c, e, f, g, h])
    }
}
/// n = 11, i = 6, a < i, b < i, b < j, c < h, c < j, d < h, d < j, e < f, f < g, f < j, h < i, j < k
fn select_569([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_563([a, b, c, d, e, f, g, h, i, j, k])
    } else {
        select_568([h, b, e, a, f, g, i, j, k])
    }
}
/// n = 13, i = 7, a < i, b < j, b < k, b < m, c < i, c < k, d < g, e < f, e < m, f < j, f < k, g < h, g < k, g < m, i < m, k < l
fn select_570([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < j {
        select_565([a, b, c, d, e, f, g, h, i, j, k, l, m])
    } else {
        select_569([a, c, b, f, d, g, h, j, i, k, l])
    }
}
/// n = 8, i = 3, b < d, c < e, e < f, e < g, g < h
fn select_571([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_87([a, c, d, e])
    } else {
        select_59([a, b, e, f, g, h, d])
    }
}
/// n = 13, i = 7, a < j, a < m, b < i, c < i, c < k, d < g, e < f, e < m, f < j, f < k, g < h, g < k, g < m, i < m, k < l
fn select_572([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < k {
        select_570([b, a, c, d, e, f, g, h, i, j, k, l, m])
    } else {
        select_571([h, b, f, i, k, l, a, j])
    }
}
/// n = 9, i = 5, a < g, a < h, b < g, b < h, c < f, c < h, d < e, d < h, h < i
fn select_573([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_443([e, a, b, c, f, g, h, i])
    } else {
        select_150([g, d, c, f, e, h, i])
    }
}
/// n = 10, i = 5, a < g, b < g, b < h, c < f, c < h, d < e, d < h, f < j, h < i, h < j
fn select_574([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < h {
        select_573([a, b, c, d, e, f, g, h, i])
    } else {
        select_59([e, f, h, i, a, g, j])
    }
}
/// n = 8, i = 5, a < g, a < h, b < g, b < h, c < f, d < e, d < h, f < h
fn select_575([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_240([e, a, b, c, f, g, h])
    } else {
        select_52([g, d, c, f, e, h])
    }
}
/// n = 6, i = 1, b < c, b < d, b < e, b < f
fn select_576([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < a {
        select_33([c, d, e, f])
    } else {
        select_34([a, b, d, e, f])
    }
}
/// n = 11, i = 5, a < g, b < g, b < h, c < f, d < e, d < h, f < h, h < i, h < j, h < k
fn select_577([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_575([a, b, c, d, e, f, g, h])
    } else {
        select_576([e, h, a, i, j, k])
    }
}
/// n = 14, i = 5, a < g, a < h, b < g, b < j, c < f, c < h, d < e, d < h, d < j, f < j, f < n, h < i, h < n, j < k, j < l, j < m
fn select_578([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_574([b, a, c, d, e, f, g, h, i, n])
    } else {
        select_577([a, b, c, d, e, f, g, j, k, l, m])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, c < d, c < h, d < g, e < f, e < h
fn select_579([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_457([a, d, b, e, g, f, h])
    } else {
        select_11([a, f, c, d, h])
    }
}
/// n = 7, i = 4, a < d, b < f, c < e, e < g, f < g
fn select_580([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_26([a, b, e, d, f, g])
    } else {
        select_26([a, c, f, d, e, g])
    }
}
/// n = 10, i = 6, a < h, a < j, b < g, c < f, d < e, d < j, e < h, f < i, f < j, g < i, g < j
fn select_581([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_579([a, c, d, e, f, i, h, j])
    } else {
        select_580([a, b, c, h, f, g, i])
    }
}
/// n = 9, i = 3, a < d, b < e, c < e, e < f, e < g, e < h, e < i
fn select_582([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_189([a, f, g, h, i])
    } else {
        select_87([d, b, c, e])
    }
}
/// n = 13, i = 6, a < h, a < j, b < g, c < e, c < j, d < f, d < j, e < i, f < h, g < i, g < j, j < k, j < l, j < m
fn select_583([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < j {
        select_581([a, b, c, d, f, e, g, h, i, j])
    } else {
        select_582([f, c, g, h, j, e, k, l, m])
    }
}
/// n = 8, i = 4, a < e, a < h, b < f, b < h, c < f, c < h, d < e, d < g, d < h, f < g
fn select_584([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_11([a, f, d, g, h])
    } else {
        select_87([e, b, c, h])
    }
}
/// n = 9, i = 5, a < g, a < i, b < g, b < h, b < i, c < f, d < e, d < i, e < h, f < h, f < i
fn select_585([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_584([a, d, f, b, g, e, h, i])
    } else {
        select_157([a, c, e, b, g, f, h])
    }
}
/// n = 8, i = 4, a < h, b < f, b < g, c < f, c < g, c < h, d < e, e < g, e < h
fn select_586([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_470([b, c, e, a, f, h, g])
    } else {
        select_157([b, a, d, c, f, e, h])
    }
}
/// n = 11, i = 6, a < h, a < k, b < f, c < g, d < e, d < k, e < i, e < j, f < i, f < j, f < k, g < h, g < j, g < k
fn select_587([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_585([a, g, b, d, e, f, h, j, k])
    } else {
        select_586([a, e, f, c, g, i, j, k])
    }
}
/// n = 14, i = 6, a < h, a < j, b < g, c < e, c < j, d < f, d < j, e < i, e < k, f < h, f < k, g < i, g < j, g < k, j < l, j < m, j < n
fn select_588([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < f {
        select_582([e, a, d, i, j, f, l, m, n])
    } else {
        select_587([a, b, d, c, e, g, f, h, i, k, j])
    }
}
/// n = 15, i = 7, a < j, a < k, b < f, c < g, d < e, d < k, e < j, e < l, f < i, f < k, g < h, g < k, g < l, i < l, k < m, k < n, k < o
fn select_589([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_583([a, b, g, d, h, e, f, j, i, k, m, n, o])
    } else {
        select_588([a, c, f, d, i, e, g, j, h, k, l, m, n, o])
    }
}
/// n = 16, i = 7, a < j, a < m, b < g, c < f, c < k, d < e, d < m, e < j, e < k, f < i, f < m, g < h, g < k, g < m, k < l, m < n, m < o, m < p
fn select_590([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < i {
        select_578([e, a, c, g, h, f, j, k, l, m, n, o, p, i])
    } else {
        select_589([a, c, b, d, e, f, g, h, i, j, m, k, n, o, p])
    }
}
/// n = 16, i = 7, b < j, b < m, c < g, c < k, d < h, e < f, e < m, f < j, f < k, g < m, h < i, h < k, h < m, k < l, m < n, m < o, m < p
fn select_591([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < g {
        select_572([b, a, c, d, e, f, h, i, g, j, k, l, m])
    } else {
        select_590([b, d, c, e, f, g, h, i, a, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 7, b < j, b < l, c < f, c < k, c < l, d < h, e < g, e < l, g < j, g < k, h < i, h < k, h < l, k < m, l < n, l < o, l < p
fn select_592([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < f {
        select_558([a, g, b, c, h, i, j, k, l, f, n, o, p, m])
    } else {
        select_591([a, b, c, d, e, g, f, h, i, j, k, m, l, n, o, p])
    }
}
/// n = 16, i = 7, b < f, b < o, c < j, c < k, d < h, e < g, e < k, g < j, g < o, h < i, h < k, h < o, k < l, k < m, k < n, o < p
fn select_593([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < b {
        select_544([a, i, g, k, l, m, n, j, b, f, o])
    } else {
        select_592([a, c, b, d, e, f, g, h, i, j, o, k, p, l, m, n])
    }
}
/// n = 16, i = 7, b < g, b < o, c < k, c < l, d < h, d < l, e < f, e < i, h < k, h < o, i < j, i < l, i < o, l < m, l < n, o < p
fn select_594([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < l {
        select_535([a, b, c, d, e, g, f, h, i, j, k, o, l, m, n, p])
    } else {
        select_593([a, b, c, e, d, g, h, i, j, k, l, f, m, n, o, p])
    }
}
/// n = 16, i = 7, b < g, b < l, c < k, c < n, d < h, d < n, e < f, e < i, h < k, i < j, i < l, i < n, l < m, n < o, n < p
fn select_595([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < h {
        select_380([a, f, c, d, b, i, j, g, l, m, n, o, p, h, k])
    } else {
        select_594([a, b, c, d, e, f, g, h, i, j, k, n, o, p, l, m])
    }
}
/// n = 16, i = 7, b < i, b < l, c < h, c < n, d < g, d < n, e < f, e < j, j < k, j < l, j < n, l < m, n < o, n < p
fn select_596([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < h {
        select_595([a, b, c, d, e, f, i, g, j, k, h, l, m, n, o, p])
    } else {
        select_595([a, b, d, c, e, f, i, h, j, k, g, l, m, n, o, p])
    }
}
/// n = 16, i = 7, b < i, c < h, c < l, d < g, d < m, e < f, e < j, j < k, j < l, j < m, l < p, m < n, m < o
fn select_597([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if m < b {
        select_272([a, f, c, d, j, k, g, h, l, m, n, o, p, b, i])
    } else {
        select_596([a, c, b, d, e, f, g, i, h, j, k, l, p, m, n, o])
    }
}
/// n = 8, i = 3, a < e, a < h, b < c, b < d, b < g, d < h, e < f, e < g
fn select_598([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_11([c, d, a, h, e])
    } else {
        select_29([e, b, f, d, h, c, g])
    }
}
/// n = 9, i = 3, a < e, a < g, b < c, b < d, b < g, b < i, e < f, e < i, g < h
fn select_599([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_36([e, b, d, f, g, c, h, i])
    } else {
        select_598([a, b, d, c, e, f, i, g])
    }
}
/// n = 11, i = 3, a < e, a < g, a < h, b < c, b < d, b < g, b < h, e < f, g < k, h < i, h < j
fn select_600([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_427([c, d, a, g, h, e, i, j, k])
    } else {
        select_599([a, b, c, d, e, f, g, k, h])
    }
}
/// n = 11, i = 3, a < c, a < d, a < i, b < e, b < f, b < i, e < h, f < g, i < j, i < k
fn select_601([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_422([f, e, h, g, a, c, d, i])
    } else {
        select_600([b, a, c, d, f, g, e, i, j, k, h])
    }
}
/// n = 10, i = 3, a < e, a < g, a < j, b < c, b < d, b < g, d < j, e < f, g < h, g < i
fn select_602([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_36([e, b, c, f, g, h, i, d])
    } else {
        select_311([c, d, a, g, h, i, e, j])
    }
}
/// n = 10, i = 3, a < f, b < f, b < g, b < j, c < d, c < e, c < g, e < j, g < h, g < i
fn select_603([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_479([a, c, d, e, b, f, g])
    } else {
        select_602([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 10, i = 2, a < d, a < e, a < j, b < c, b < f, b < j, e < i, f < g, f < h
fn select_604([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_45([b, a, d, c, e, i, j])
    } else {
        select_36([a, b, c, d, f, g, h, j])
    }
}
/// n = 10, i = 3, a < e, a < f, b < d, b < g, c < e, c < f, c < g, f < j, g < h, g < i
fn select_605([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_45([b, c, e, d, f, j, g])
    } else {
        select_311([d, a, c, g, h, i, e, f])
    }
}
/// n = 11, i = 3, a < g, b < d, b < h, b < k, c < e, c < g, c < h, e < f, e < k, h < i, h < j
fn select_606([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_604([e, b, d, f, a, h, i, j, g, k])
    } else {
        select_605([a, b, c, d, g, e, h, i, j, f])
    }
}
/// n = 11, i = 3, a < h, b < d, b < e, b < i, c < f, c < h, c < i, f < g, i < j, i < k
fn select_607([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < f {
        select_603([a, c, b, e, d, h, i, j, k, f])
    } else {
        select_606([a, b, c, e, f, g, h, i, j, k, d])
    }
}
/// n = 11, i = 3, b < e, b < f, b < i, c < d, c < g, c < i, g < h, i < j, i < k
fn select_608([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_601([b, c, e, f, d, g, h, a, i, j, k])
    } else {
        select_607([a, b, c, e, f, g, h, d, i, j, k])
    }
}
/// n = 8, i = 3, a < e, b < f, c < d, c < f, f < g, f < h
fn select_609([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_34([d, f, a, g, h])
    } else {
        select_159([b, a, c, d, e, f])
    }
}
/// n = 9, i = 3, a < e, b < f, c < d, c < f, f < g, f < h, f < i
fn select_610([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_34([d, f, g, h, i])
    } else {
        select_609([a, b, c, d, e, f, h, i])
    }
}
/// n = 9, i = 3, c < f, d < e, d < f, f < g, f < h, f < i
fn select_611([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_610([a, c, d, e, b, f, g, h, i])
    } else {
        select_610([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 12, i = 4, b < f, c < d, c < e, c < i, e < g, e < l, f < h, f < i, f < l, i < j, i < k
fn select_612([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_611([a, d, b, e, g, f, h, i, l])
    } else {
        select_419([a, c, f, h, d, e, i, j, k, g, l])
    }
}
/// n = 13, i = 4, b < d, b < e, b < k, c < f, e < h, f < g, f < i, f < k, i < j, k < l, k < m
fn select_613([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < e {
        select_420([a, b, f, g, d, i, j, k, l, m, e])
    } else {
        select_612([a, c, b, d, e, f, h, g, k, l, m, i])
    }
}
/// n = 14, i = 5, a < e, b < c, b < d, b < m, d < f, d < h, e < g, e < i, e < m, h < j, i < k, i < l, m < n
fn select_614([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < d {
        select_608([g, i, b, c, k, l, m, n, d, f, h])
    } else {
        select_613([c, d, a, f, h, e, g, j, m, n, i, k, l])
    }
}
/// n = 11, i = 4, b < f, c < d, c < e, c < i, e < g, e < k, f < h, f < i, f < k, i < j
fn select_615([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_611([a, d, b, e, g, f, h, i, k])
    } else {
        select_225([a, c, f, h, d, i, e, g, k, j])
    }
}
/// n = 13, i = 4, b < f, b < i, c < e, c < j, d < g, g < h, g < i, g < j, i < m, j < k, j < l
fn select_616([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < c {
        select_615([a, d, b, f, c, g, e, h, i, m, j])
    } else {
        select_612([a, d, c, e, b, g, f, h, j, k, l, i])
    }
}
/// n = 9, i = 3, a < d, a < i, b < f, c < e, c < f, e < i, f < g, f < h
fn select_617([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_34([e, f, a, g, h])
    } else {
        select_214([b, c, a, d, e, i, f])
    }
}
/// n = 10, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, e < h, f < i, f < j
fn select_618([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_34([d, e, a, g, h])
    } else {
        select_185([b, a, c, d, f, i, j, e])
    }
}
/// n = 10, i = 3, a < f, b < d, b < e, c < f, c < g, c < h, e < g, e < h, f < i, f < j
fn select_619([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < c {
        select_47([a, d, e, c, g, h])
    } else {
        select_618([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 10, i = 3, a < g, b < e, b < f, c < d, c < g, c < h, f < h, g < i, g < j
fn select_620([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_617([b, a, c, e, d, g, i, j, f])
    } else {
        select_619([a, b, c, e, f, g, d, h, i, j])
    }
}
/// n = 11, i = 3, a < g, b < e, b < f, c < d, c < f, c < g, f < h, f < i, g < j, g < k
fn select_621([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_609([b, a, c, d, e, g, j, k])
    } else {
        select_57([a, b, c, d, e, f, h, i])
    }
}
/// n = 11, i = 3, a < h, b < e, b < f, b < g, c < d, c < g, c < h, g < i, h < j, h < k
fn select_622([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_620([a, b, c, d, f, e, h, g, j, k])
    } else {
        select_621([a, b, c, d, f, g, h, e, i, j, k])
    }
}
/// n = 9, i = 1, a < i, b < g, b < h, c < d, c < e, c < f
fn select_623([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_425([a, c, d, e, f, b, i])
    } else {
        select_329([b, c, d, e, f, a, g, h])
    }
}
/// n = 9, i = 1, c < h, c < i, d < e, d < f, d < g
fn select_624([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_623([a, c, d, e, f, g, h, i, b])
    } else {
        select_623([b, c, d, e, f, g, h, i, a])
    }
}
/// n = 8, i = 1, a < h, b < g, c < d, c < e, c < f
fn select_625([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_425([a, c, d, e, f, b, h])
    } else {
        select_425([b, c, d, e, f, a, g])
    }
}
/// n = 9, i = 2, a < i, b < h, c < d, c < h, c < i, d < e, d < f, d < g
fn select_626([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_70([a, c, b, d, e, f, g, i])
    } else {
        select_70([b, c, a, d, e, f, g, h])
    }
}
/// n = 9, i = 2, b < i, c < d, c < e, c < i, e < f, e < g, e < h
fn select_627([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_625([b, d, e, f, g, h, a, i])
    } else {
        select_626([a, b, c, e, f, g, h, i, d])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, e < j, f < g, f < h, f < i
fn select_628([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_624([b, d, e, f, g, h, i, a, j])
    } else {
        select_627([b, a, c, d, f, g, h, i, e])
    }
}
/// n = 13, i = 4, a < e, a < i, a < k, b < c, b < d, b < j, d < f, d < i, d < k, e < g, e < h, e < j, j < l, k < m
fn select_629([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < e {
        select_628([c, f, a, i, k, e, g, h, j, m])
    } else {
        select_628([g, h, b, c, j, d, f, i, k, l])
    }
}
/// n = 13, i = 4, a < d, a < e, a < j, b < c, b < f, b < k, e < i, e < k, f < g, f < h, f < j, j < l, k < m
fn select_630([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < e {
        select_622([c, f, a, d, g, h, j, e, l, i, k])
    } else {
        select_629([b, a, d, e, f, i, g, h, c, j, k, l, m])
    }
}
/// n = 14, i = 5, a < f, b < c, b < d, b < e, d < h, e < g, e < m, f < i, f < j, f < m, j < k, j < l, m < n
fn select_631([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < j {
        select_616([c, e, d, a, h, g, f, i, m, j, k, l, n])
    } else {
        select_630([b, f, i, c, e, j, k, l, g, d, m, h, n])
    }
}
/// n = 14, i = 5, a < e, a < m, b < d, b < f, c < g, f < i, g < h, g < j, g < m, j < k, j < l, m < n
fn select_632([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_614([c, a, e, b, g, d, h, f, j, i, k, l, m, n])
    } else {
        select_631([c, b, d, f, a, g, e, i, h, j, k, l, m, n])
    }
}
/// n = 14, i = 5, a < g, b < f, c < e, c < m, d < h, h < i, h < j, h < m, j < k, j < l, m < n
fn select_633([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_632([c, a, d, g, e, b, h, i, f, j, k, l, m, n])
    } else {
        select_632([c, b, d, f, e, a, h, i, g, j, k, l, m, n])
    }
}
/// n = 14, i = 5, c < g, d < f, d < m, e < h, h < i, h < j, h < m, j < k, j < l, m < n
fn select_634([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_633([a, c, d, e, f, g, b, h, i, j, k, l, m, n])
    } else {
        select_633([b, c, d, e, f, g, a, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 5, e < g, e < m, f < h, h < i, h < j, h < m, j < k, j < l, m < n
fn select_635([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_634([c, d, a, e, f, g, b, h, i, j, k, l, m, n])
    } else {
        select_634([c, d, b, e, f, g, a, h, i, j, k, l, m, n])
    }
}
/// n = 7, i = 2, a < e, b < c, b < d, b < f, f < g
fn select_636([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_110([a, b, e, c, f, g, d])
    } else {
        select_110([a, b, e, d, f, g, c])
    }
}
/// n = 6, i = 4, b < f, c < f, d < e
fn select_637([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_156([b, c, a, d, e, f])
    } else {
        select_162([b, c, e, a, f])
    }
}
/// n = 9, i = 4, a < e, b < g, c < f, c < g, d < f, d < g, g < h, g < i
fn select_638([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_47([a, f, b, g, h, i])
    } else {
        select_637([b, c, d, a, e, f])
    }
}
/// n = 9, i = 4, a < f, b < e, c < g, d < f, d < g, g < h, g < i
fn select_639([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_636([b, g, h, i, e, a, f])
    } else {
        select_638([b, c, a, d, e, f, g, h, i])
    }
}
/// n = 7, i = 2, a < f, a < g, b < d, b < e, b < g, c < d, c < e, c < f
fn select_640([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_20([b, c, f, g])
    } else {
        select_28([a, c, d, e, g])
    }
}
/// n = 8, i = 2, a < h, b < f, b < g, b < h, c < d, c < f, c < g, d < e
fn select_641([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_43([b, d, a, e, f, g])
    } else {
        select_640([a, b, c, f, g, d, h])
    }
}
/// n = 7, i = 3, a < e, b < d, c < e, c < g, d < g, e < f
fn select_642([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_87([a, c, d, g])
    } else {
        select_3([b, e, f])
    }
}
/// n = 9, i = 3, a < d, b < f, b < h, b < i, c < e, c < f, e < h, e < i, f < g
fn select_643([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_641([a, e, b, f, g, h, i, d])
    } else {
        select_642([b, a, c, d, f, g, e])
    }
}
/// n = 9, i = 4, a < e, b < g, c < f, c < h, c < i, d < f, d < g, g < h, g < i
fn select_644([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_30([a, c, b, f, g, h, i, e])
    } else {
        select_158([b, c, d, a, e, f, g])
    }
}
/// n = 10, i = 4, a < e, b < g, b < j, c < g, c < h, c < j, d < f, f < h, f < j, h < i
fn select_645([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_643([a, c, f, e, b, h, i, g, j])
    } else {
        select_644([a, d, c, b, e, g, f, h, j])
    }
}
/// n = 10, i = 4, a < e, b < g, b < j, c < g, c < h, c < j, d < f, d < h, f < j, h < i
fn select_646([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_639([b, a, d, c, e, g, h, f, i])
    } else {
        select_645([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 10, i = 3, a < c, a < h, a < i, b < d, b < j, c < e, c < j, d < f, d < h, d < i, f < g
fn select_647([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_19([e, b, d, j])
    } else {
        select_67([f, a, c, h, i, g])
    }
}
/// n = 10, i = 3, a < d, a < i, a < j, b < c, b < e, d < f, e < g, e < i, e < j, g < h
fn select_648([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_641([c, a, e, g, h, i, j, d])
    } else {
        select_647([a, b, d, e, f, g, h, i, j, c])
    }
}
/// n = 11, i = 3, a < d, a < j, a < k, b < c, b < e, d < g, e < f, e < h, e < j, e < k, h < i
fn select_649([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_364([c, a, e, f, h, i, j, k])
    } else {
        select_648([a, b, c, d, e, g, h, i, j, k])
    }
}
/// n = 10, i = 4, a < g, b < f, b < i, c < e, c < i, d < f, d < g, d < i, g < h, i < j
fn select_650([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < g {
        select_492([a, b, d, c, e, f, i, j])
    } else {
        select_546([b, c, a, d, e, f, g, h])
    }
}
/// n = 12, i = 4, a < e, b < g, b < k, b < l, c < f, c < h, d < f, d < g, d < h, g < j, h < i, h < k, h < l
fn select_651([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_649([b, c, f, g, h, i, j, a, e, k, l])
    } else {
        select_650([b, c, a, d, e, f, g, j, h, i])
    }
}
/// n = 12, i = 4, a < f, b < g, b < i, c < e, c < h, c < l, d < g, d < h, d < i, h < j, i < k, i < l
fn select_652([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_646([a, b, d, c, f, e, g, h, j, i])
    } else {
        select_651([a, c, b, d, f, g, h, i, k, j, e, l])
    }
}
/// n = 12, i = 4, c < g, c < i, d < f, d < h, d < l, e < g, e < h, e < i, h < j, i < k, i < l
fn select_653([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_652([a, c, d, e, f, b, g, h, i, j, k, l])
    } else {
        select_652([b, c, d, e, f, a, g, h, i, j, k, l])
    }
}
/// n = 9, i = 2, a < e, b < c, b < d, b < f, f < g, f < h, f < i
fn select_654([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_426([a, b, e, c, f, g, h, i, d])
    } else {
        select_426([a, b, e, d, f, g, h, i, c])
    }
}
/// n = 9, i = 2, c < d, c < e, c < f, f < g, f < h, f < i
fn select_655([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_654([a, c, d, e, b, f, g, h, i])
    } else {
        select_654([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 10, i = 3, a < j, b < e, b < f, b < j, c < d, c < f, c < j, f < g, f < h, f < i
fn select_656([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_626([a, d, b, f, g, h, i, e, j])
    } else {
        select_626([a, e, c, f, g, h, i, d, j])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, f < k, g < h, g < i, g < j
fn select_657([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_624([d, e, f, g, h, i, j, a, k])
    } else {
        select_656([a, b, c, d, e, g, h, i, j, f])
    }
}
/// n = 11, i = 3, b < e, b < i, b < k, c < d, c < f, c < i, f < g, f < h, f < k, i < j
fn select_658([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_655([a, d, f, g, h, b, e, i, k])
    } else {
        select_657([a, b, c, d, e, i, f, g, h, k, j])
    }
}
/// n = 7, i = 2, b < f, b < g, c < d, c < f, c < g, d < e
fn select_659([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_43([b, d, a, e, f, g])
    } else {
        select_154([a, b, c, f, g, d])
    }
}
/// n = 9, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, d < i, f < i, g < h
fn select_660([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_491([a, e, b, c, g, h, f])
    } else {
        select_659([a, f, d, g, h, e, i])
    }
}
/// n = 11, i = 4, b < h, c < f, c < h, d < e, d < g, d < h, e < j, f < g, f < j, g < i, h < k
fn select_661([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_218([a, d, f, b, e, g, i, j, h])
    } else {
        select_660([a, b, c, d, e, f, h, k, g])
    }
}
/// n = 11, i = 4, b < i, c < e, c < h, c < i, d < f, d < i, f < g, f < h, h < j, i < k
fn select_662([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_213([a, b, f, c, g, h, i, j, e, k])
    } else {
        select_661([a, b, d, c, e, f, h, i, j, g, k])
    }
}
/// n = 12, i = 4, c < e, c < h, c < j, d < f, d < h, f < g, f < j, f < l, h < i, h < l, j < k
fn select_663([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_658([b, f, c, e, g, h, a, i, j, k, l])
    } else {
        select_662([b, a, c, d, e, f, g, j, h, k, i])
    }
}
/// n = 8, i = 3, a < d, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_664([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_25([a, c, d, e, g])
    } else {
        select_25([b, c, d, f, h])
    }
}
/// n = 9, i = 4, a < f, a < g, b < d, b < f, c < e, d < i, e < g, e < i, g < h
fn select_665([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_26([a, c, d, f, e, i])
    } else {
        select_664([a, b, e, f, d, g, i, h])
    }
}
/// n = 7, i = 3, a < c, b < d, c < e, d < f, f < g
fn select_666([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_3([e, b, d])
    } else {
        select_20([a, f, g, c])
    }
}
/// n = 10, i = 4, a < e, a < h, b < e, b < g, c < d, d < g, d < h, e < f, g < i, h < j
fn select_667([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_666([a, c, e, d, f, g, i])
    } else {
        select_666([b, c, e, d, f, h, j])
    }
}
/// n = 10, i = 4, a < f, a < h, b < d, b < g, c < e, d < f, e < g, e < h, g < i, h < j
fn select_668([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_666([d, c, a, e, f, g, i])
    } else {
        select_667([a, b, c, e, d, f, g, h, i, j])
    }
}
/// n = 10, i = 4, a < f, b < h, c < f, c < g, d < e, e < g, e < h, g < i, h < j
fn select_669([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_665([c, a, d, b, e, f, g, i, h])
    } else {
        select_668([c, b, d, a, e, f, h, g, j, i])
    }
}
/// n = 8, i = 3, a < c, a < d, a < h, b < e, e < f, e < h, f < g
fn select_670([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_87([d, b, c, e])
    } else {
        select_29([a, e, d, f, g, c, h])
    }
}
/// n = 11, i = 4, a < h, b < d, b < g, c < e, d < f, d < j, e < g, e < h, e < j, g < i, h < k
fn select_671([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_670([d, c, a, f, e, g, i, j])
    } else {
        select_667([a, b, c, e, d, f, g, h, i, k])
    }
}
/// n = 11, i = 4, b < h, c < e, c < g, d < f, e < j, f < g, f < h, f < j, g < i, h < k
fn select_672([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < e {
        select_669([a, b, c, d, f, e, g, h, i, k])
    } else {
        select_671([b, c, d, e, f, a, g, h, i, j, k])
    }
}
/// n = 11, i = 4, b < i, c < e, c < h, d < f, f < g, f < h, f < i, h < j, i < k
fn select_673([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_219([a, b, c, f, g, h, i, j, e, k])
    } else {
        select_672([a, b, c, d, e, f, h, i, j, g, k])
    }
}
/// n = 12, i = 4, c < e, c < j, c < l, d < f, f < g, f < h, f < j, h < i, h < l, j < k
fn select_674([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_658([b, c, f, g, e, h, a, i, j, k, l])
    } else {
        select_673([b, a, c, d, e, f, g, j, h, k, i])
    }
}
/// n = 13, i = 4, c < e, c < i, c < l, d < f, d < i, f < g, f < h, f < l, h < j, i < k, l < m
fn select_675([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < h {
        select_663([a, b, c, d, e, f, g, i, k, l, m, h])
    } else {
        select_674([a, b, c, d, e, f, g, h, j, l, m, i])
    }
}
/// n = 13, i = 4, c < i, d < f, d < j, d < l, e < g, e < i, e < j, g < h, g < l, j < k, l < m
fn select_676([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < g {
        select_653([a, b, c, d, e, f, i, j, g, k, h, l])
    } else {
        select_675([a, b, d, e, f, g, h, c, j, i, k, l, m])
    }
}
/// n = 6, i = 1, d < e, d < f
fn select_677([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_172([c, a, d, e, f, b])
    } else {
        select_172([c, b, d, e, f, a])
    }
}
/// n = 6, i = 2, b < f, c < d, c < e, c < f
fn select_678([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < e {
        select_58([a, b, c, d, e, f])
    } else {
        select_58([a, b, c, e, d, f])
    }
}
/// n = 7, i = 2, c < d, c < e, c < f, f < g
fn select_679([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < a {
        select_677([b, d, e, f, a, g])
    } else {
        select_678([b, a, c, d, e, f])
    }
}
/// n = 8, i = 3, b < e, c < d, c < f, f < g, f < h
fn select_680([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < b {
        select_679([a, d, f, g, h, b, e])
    } else {
        select_57([a, b, c, d, e, f, g, h])
    }
}
/// n = 8, i = 2, c < d, c < e, d < h, e < f, e < g
fn select_681([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_173([b, d, e, a, f, g, h])
    } else {
        select_58([b, a, c, d, h, e])
    }
}
/// n = 9, i = 3, b < e, b < i, c < d, c < f, f < g, f < i, g < h
fn select_682([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_681([a, d, f, g, b, e, i, h])
    } else {
        select_57([a, b, c, d, e, f, g, i])
    }
}
/// n = 9, i = 3, c < e, c < f, d < e, d < f, e < i, f < g, f < h
fn select_683([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_173([b, e, f, a, g, h, i])
    } else {
        select_491([b, a, c, d, e, i, f])
    }
}
/// n = 11, i = 4, b < e, b < i, c < f, c < i, d < f, d < i, f < g, f < h, h < j, i < k
fn select_684([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_339([a, g, h, b, e, i, j])
    } else {
        select_683([a, e, c, d, i, f, g, h, k])
    }
}
/// n = 11, i = 4, b < e, b < i, c < h, c < i, d < f, d < i, f < g, f < h, h < j, i < k
fn select_685([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < c {
        select_682([a, b, f, g, e, c, h, j, i])
    } else {
        select_684([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 11, i = 4, b < j, c < e, c < h, d < f, d < h, f < g, f < j, h < i, j < k
fn select_686([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < b {
        select_680([a, f, c, e, g, h, b, i])
    } else {
        select_685([a, c, b, d, e, f, g, j, h, k, i])
    }
}
/// n = 10, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, g < h, g < i, g < j
fn select_687([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_627([a, d, b, f, g, h, i, j, e])
    } else {
        select_627([a, e, c, f, g, h, i, j, d])
    }
}
/// n = 12, i = 4, b < d, b < h, c < e, c < h, c < k, d < g, d < k, d < l, e < f, e < i, e < l, i < j
fn select_688([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_687([a, c, d, g, h, k, e, f, i, l])
    } else {
        select_74([a, b, e, f, h, i, j, d, g, k, l])
    }
}
/// n = 13, i = 5, b < f, b < k, c < d, c < e, c < j, e < g, e < k, e < m, f < h, f < j, h < i, h < m, k < l
fn select_689([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < h {
        select_686([a, d, e, b, g, f, j, k, l, h, i])
    } else {
        select_688([a, f, c, h, e, g, i, j, k, l, d, m])
    }
}
/// n = 12, i = 4, a < d, a < f, a < i, b < c, b < e, b < j, e < h, e < i, e < l, f < g, f < j, f < l, j < k
fn select_690([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_687([c, a, e, h, d, i, f, g, j, l])
    } else {
        select_657([d, b, f, g, c, j, e, h, i, l, k])
    }
}
/// n = 8, i = 3, a < e, b < e, b < f, b < h, c < d, c < f, d < h, f < g
fn select_691([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_42([d, b, a, f, g, h])
    } else {
        select_139([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 4, a < f, a < g, b < e, b < g, b < i, c < d, c < f, c < h, f < i, g < h
fn select_692([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_691([d, b, a, f, e, g, h, i])
    } else {
        select_691([e, c, a, g, d, f, i, h])
    }
}
/// n = 11, i = 4, a < g, b < e, b < g, b < h, c < d, c < f, c < k, e < j, f < h, f < j, g < k, h < i
fn select_693([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_218([d, b, f, a, e, h, i, j, g])
    } else {
        select_692([a, b, c, d, e, f, g, k, h])
    }
}
/// n = 12, i = 5, a < i, b < f, c < d, c < e, c < l, e < g, e < j, f < h, f < i, f < j, i < l, j < k
fn select_694([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_693([a, f, c, d, h, e, i, j, k, g, l])
    } else {
        select_672([d, a, e, b, g, f, j, i, k, h, l])
    }
}
/// n = 13, i = 5, b < f, c < d, c < e, c < j, e < g, e < k, e < m, f < h, f < i, f < k, i < j, i < m, k < l
fn select_695([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < a {
        select_690([c, f, h, d, i, e, g, a, j, k, l, m])
    } else {
        select_694([a, b, c, d, e, f, g, h, i, k, l, j])
    }
}
/// n = 14, i = 5, b < f, b < j, c < d, c < e, c < k, e < g, e < j, e < m, f < h, f < i, f < m, i < k, j < l, m < n
fn select_696([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < i {
        select_663([a, d, e, b, g, f, h, j, l, m, n, i])
    } else {
        select_695([a, b, c, d, e, f, g, h, i, k, m, n, j])
    }
}
/// n = 14, i = 5, b < e, b < f, b < k, c < d, c < g, c < j, e < j, f < i, f < m, g < h, g < k, g < m, k < l, m < n
fn select_697([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < f {
        select_689([a, b, c, d, g, e, h, f, i, j, k, l, m])
    } else {
        select_696([a, b, c, d, g, f, h, i, e, k, j, l, m, n])
    }
}
/// n = 14, i = 5, b < f, b < g, b < k, c < d, c < e, c < h, g < j, g < m, h < i, h < k, h < m, k < l, m < n
fn select_698([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < f {
        select_676([a, e, d, h, b, i, g, j, f, k, l, m, n])
    } else {
        select_697([a, b, c, e, f, g, h, i, j, d, k, l, m, n])
    }
}
/// n = 13, i = 5, b < f, b < j, c < e, c < i, c < j, d < g, d < j, g < h, g < i, g < m, i < k, j < l, j < m
fn select_699([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < b {
        select_542([a, h, b, c, e, f, i, k, j, l, m])
    } else {
        select_663([a, f, c, d, e, g, h, j, l, i, k, m])
    }
}
/// n = 15, i = 6, b < e, b < h, b < l, c < h, c < i, d < f, d < i, f < g, h < k, h < n, i < j, i < l, i < n, l < m, n < o
fn select_700([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < f {
        select_675([a, e, h, d, k, i, j, l, f, m, g, n, o])
    } else {
        select_699([a, f, b, c, e, g, i, j, l, h, m, k, n])
    }
}
/// n = 15, i = 6, b < g, b < h, b < l, c < h, c < i, d < e, d < f, d < i, h < k, h < n, i < j, i < l, i < n, l < m, n < o
fn select_701([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < f {
        select_700([a, b, c, d, g, e, f, h, i, j, k, l, m, n, o])
    } else {
        select_700([a, b, c, d, g, f, e, h, i, j, k, l, m, n, o])
    }
}
/// n = 15, i = 6, b < g, b < m, c < h, c < j, d < e, d < f, d < j, h < i, h < l, j < k, j < l, j < m, l < n, m < o
fn select_702([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < b {
        select_698([a, h, d, e, f, i, b, j, k, g, l, n, m, o])
    } else {
        select_701([a, b, c, d, e, f, g, h, j, k, i, m, o, l, n])
    }
}
/// n = 8, i = 4, c < h, d < f, e < f, f < g, f < h
fn select_703([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_10([c, a, d, e, b, f, g, h])
    } else {
        select_10([c, b, d, e, a, f, g, h])
    }
}
/// n = 10, i = 4, d < f, e < f, f < g, f < h, h < i, h < j
fn select_704([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_221([b, c, f, g, h, a, i, j])
    } else {
        select_703([b, c, a, d, e, f, g, h])
    }
}
/// n = 10, i = 4, d < g, d < h, e < f, f < g, f < h, h < i, h < j
fn select_705([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_249([c, a, d, e, b, f, g, h, i, j])
    } else {
        select_249([c, b, d, e, a, f, g, h, i, j])
    }
}
/// n = 11, i = 4, d < g, d < h, d < i, e < f, f < g, f < h, f < i, h < k, i < j
fn select_706([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_705([a, b, c, d, e, f, g, h, i, k])
    } else {
        select_705([a, b, c, d, e, f, g, i, h, j])
    }
}
/// n = 11, i = 4, d < g, d < i, d < j, e < f, e < g, f < i, f < j, g < h, j < k
fn select_707([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_704([a, b, c, d, e, g, h, f, i, j])
    } else {
        select_706([a, b, c, d, e, f, i, g, j, k, h])
    }
}
/// n = 7, i = 3, b < e, b < g, c < d, c < f, d < g, e < f
fn select_708([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < d {
        select_52([a, c, b, e, d, f])
    } else {
        select_42([d, b, a, e, f, g])
    }
}
/// n = 8, i = 3, b < e, b < h, c < d, c < f, c < g, d < h, e < f, e < g
fn select_709([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_44([d, b, a, e, f, g, h])
    } else {
        select_255([a, c, b, e, d, f, g])
    }
}
/// n = 9, i = 3, b < e, b < f, c < d, c < f, c < h, c < i, e < h, e < i, f < g
fn select_710([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_311([a, e, c, f, d, g, h, i])
    } else {
        select_709([a, b, c, d, e, h, i, f])
    }
}
/// n = 10, i = 3, b < j, c < f, c < h, c < i, d < e, d < f, e < h, e < i, e < j, f < g
fn select_711([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_708([a, b, d, e, c, f, j])
    } else {
        select_710([a, d, c, b, e, f, g, h, i])
    }
}
/// n = 9, i = 4, b < f, c < f, c < g, d < e, d < i, e < g, f < i, g < h
fn select_712([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_372([a, b, c, d, e, f, i])
    } else {
        select_368([a, b, c, e, g, h])
    }
}
/// n = 10, i = 5, b < e, c < g, d < f, d < j, e < j, f < h, g < h, g < j, h < i
fn select_713([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_712([a, b, g, d, f, e, h, i, j])
    } else {
        select_372([a, c, e, d, f, g, j])
    }
}
/// n = 12, i = 5, b < e, c < g, d < f, d < h, f < i, f < k, f < l, g < h, g < i, h < k, h < l, i < j
fn select_714([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_711([a, b, f, g, h, i, j, k, l, e])
    } else {
        select_713([a, b, c, d, e, f, g, i, j, h])
    }
}
/// n = 12, i = 5, d < g, e < f, e < h, f < i, f < k, f < l, g < h, g < i, h < k, h < l, i < j
fn select_715([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_714([c, a, d, e, b, f, g, h, i, j, k, l])
    } else {
        select_714([c, b, d, e, a, f, g, h, i, j, k, l])
    }
}
/// n = 10, i = 3, a < d, b < e, b < f, c < e, c < f, e < j, f < g, f < h, f < i
fn select_716([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_68([e, f, a, g, h, i, j])
    } else {
        select_126([a, b, c, d, e, j, f])
    }
}
/// n = 10, i = 3, a < d, b < h, b < j, c < e, c < h, e < f, e < g, e < j, h < i
fn select_717([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_116([a, e, f, g, d, b, h, j])
    } else {
        select_716([a, b, c, d, h, e, f, g, j, i])
    }
}
/// n = 10, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < j, g < j, h < i
fn select_718([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_355([a, b, c, d, e, f, g, j])
    } else {
        select_58([a, g, f, h, i, j])
    }
}
/// n = 11, i = 5, a < e, b < i, c < g, d < f, d < i, f < h, g < h, g < i, h < j, i < k
fn select_719([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < f {
        select_718([a, b, d, c, e, g, f, i, k, h])
    } else {
        select_111([a, c, f, e, b, g, h, j, i])
    }
}
/// n = 12, i = 5, b < e, c < g, d < f, d < h, f < j, f < l, g < h, g < j, h < i, h < l, j < k
fn select_720([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_717([b, f, g, e, h, a, i, j, k, l])
    } else {
        select_719([b, a, c, d, e, f, g, j, h, k, i])
    }
}
/// n = 12, i = 5, d < g, e < f, e < h, f < j, f < l, g < h, g < j, h < i, h < l, j < k
fn select_721([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_720([c, a, d, e, b, f, g, h, i, j, k, l])
    } else {
        select_720([c, b, d, e, a, f, g, h, i, j, k, l])
    }
}
/// n = 13, i = 5, d < g, d < i, e < f, e < h, f < i, f < k, g < h, g < k, g < m, h < j, i < m, k < l
fn select_722([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < h {
        select_715([a, b, c, e, d, g, f, i, k, l, h, m])
    } else {
        select_721([a, b, c, d, e, f, g, h, j, k, l, i])
    }
}
/// n = 13, i = 5, d < f, d < i, d < l, e < g, e < j, f < j, g < h, g < i, g < m, h < l, i < k, j < m
fn select_723([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < f {
        select_707([a, b, c, d, g, h, i, k, l, f, j])
    } else {
        select_722([a, b, c, e, d, f, g, i, j, k, h, l, m])
    }
}
/// n = 7, i = 3, b < e, b < f, c < d, c < f, c < g, e < g
fn select_724([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_15([a, b, c, d, f, e])
    } else {
        select_105([e, c, d, f, a, g])
    }
}
/// n = 9, i = 3, b < f, c < g, c < i, d < e, d < f, d < g, e < i, g < h
fn select_725([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_724([a, b, d, e, c, f, g])
    } else {
        select_218([a, c, d, e, b, g, h, f, i])
    }
}
/// n = 8, i = 4, a < f, b < g, b < h, c < f, c < h, d < e, e < g, e < h
fn select_726([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_124([b, c, d, f, e, g])
    } else {
        select_22([a, c, b, g, h, f])
    }
}
/// n = 9, i = 4, a < g, b < f, c < f, c < h, d < e, e < g, e < h, h < i
fn select_727([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < a {
        select_46([b, e, f, h, a, i])
    } else {
        select_726([b, a, c, d, e, f, g, h])
    }
}
/// n = 10, i = 4, a < g, b < e, c < d, c < h, d < f, d < j, e < g, e < h, e < j, h < i
fn select_728([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_691([a, e, c, d, g, h, i, j])
    } else {
        select_321([a, b, c, e, d, f, g, j])
    }
}
/// n = 10, i = 4, b < g, c < e, c < h, d < f, e < j, f < g, f < h, f < j, h < i
fn select_729([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_727([b, a, c, d, f, e, g, h, i])
    } else {
        select_728([b, d, c, e, f, a, g, h, i, j])
    }
}
/// n = 10, i = 4, b < h, c < e, c < i, d < f, f < g, f < h, f < i, i < j
fn select_730([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_725([a, b, c, f, g, h, i, j, e])
    } else {
        select_729([a, b, c, d, e, f, h, i, j, g])
    }
}
/// n = 11, i = 4, c < e, c < i, c < k, d < f, f < g, f < h, f < i, h < k, i < j
fn select_731([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_418([b, c, f, g, e, h, a, i, j, k])
    } else {
        select_730([b, a, c, d, e, f, g, h, i, j])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < j, d < i, e < f, e < g, e < h
fn select_732([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_623([b, d, e, f, g, h, a, i, j])
    } else {
        select_626([a, b, c, e, f, g, h, j, d])
    }
}
/// n = 11, i = 3, b < d, b < h, b < j, c < e, c < h, c < k, d < k, e < f, e < g, e < j, h < i
fn select_733([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_174([a, f, g, b, d, h, j])
    } else {
        select_732([a, d, c, h, e, f, g, j, i, k])
    }
}
/// n = 12, i = 4, c < e, c < i, c < k, d < f, e < l, f < g, f < i, f < l, g < h, g < k, i < j
fn select_734([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_733([b, c, f, e, g, a, h, i, j, k, l])
    } else {
        select_672([b, a, c, d, e, f, i, g, j, l, h])
    }
}
/// n = 13, i = 4, c < e, c < i, c < j, d < f, d < g, e < m, f < j, f < m, g < h, g < i, i < k, j < l
fn select_735([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_731([a, b, c, d, e, g, h, f, i, k, j])
    } else {
        select_734([a, b, c, d, e, f, g, h, j, l, i, m])
    }
}
/// n = 8, i = 4, b < h, c < g, d < g, d < h, e < f, f < g, f < h
fn select_736([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_373([a, d, e, b, c, f, h])
    } else {
        select_373([a, d, e, c, b, f, g])
    }
}
/// n = 8, i = 4, a < e, b < g, b < h, c < f, d < f, f < g, f < h
fn select_737([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_154([a, b, f, g, h, e])
    } else {
        select_9([b, c, d, e, f])
    }
}
/// n = 8, i = 4, c < g, c < h, d < f, e < f, f < g, f < h
fn select_738([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_737([a, c, d, e, b, f, g, h])
    } else {
        select_737([b, c, d, e, a, f, g, h])
    }
}
/// n = 9, i = 4, b < i, c < g, d < g, d < i, e < f, e < h, f < i, g < h
fn select_739([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_736([a, b, c, d, e, f, g, i])
    } else {
        select_738([a, b, e, c, d, g, f, h])
    }
}
/// n = 8, i = 4, b < f, c < f, d < e, d < g, d < h, f < g, f < h
fn select_740([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_35([d, f, a, e, g, h])
    } else {
        select_560([a, b, c, d, e, f])
    }
}
/// n = 10, i = 5, b < j, c < g, d < g, e < f, e < j, f < h, f < i, g < h, g < i, g < j
fn select_741([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_740([a, c, d, f, b, g, h, i])
    } else {
        select_524([a, c, d, b, e, f, g, j])
    }
}
/// n = 8, i = 4, b < h, c < g, d < f, e < f, f < g, f < h
fn select_742([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_372([a, d, e, b, c, f, h])
    } else {
        select_372([a, d, e, c, b, f, g])
    }
}
/// n = 11, i = 5, b < h, c < g, d < g, d < h, e < f, e < h, f < i, f < j, f < k, g < i, g < j, h < k
fn select_743([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_741([a, b, c, d, e, f, g, i, j, h])
    } else {
        select_742([a, c, f, b, d, h, k, g])
    }
}
/// n = 11, i = 5, b < i, c < h, d < h, d < i, e < f, e < i, f < g, f < j, f < k, h < k, i < j
fn select_744([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_739([a, c, b, d, f, g, i, j, h])
    } else {
        select_743([a, b, c, d, e, f, h, i, g, k, j])
    }
}
/// n = 9, i = 2, a < i, b < d, c < e, c < f, c < i, e < h, f < g
fn select_745([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_216([a, f, e, b, h, g, i])
    } else {
        select_229([a, b, c, d, f, g, e, i])
    }
}
/// n = 10, i = 3, a < f, b < d, c < e, c < f, e < g, e < h, g < j, h < i
fn select_746([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_52([a, c, b, d, f, e])
    } else {
        select_745([b, a, e, f, g, h, i, j, d])
    }
}
/// n = 10, i = 3, a < c, a < f, b < d, b < f, c < e, d < g, d < h, g < j, h < i
fn select_747([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_216([g, h, a, c, f, i, j])
    } else {
        select_64([c, b, f, e, d, g, h])
    }
}
/// n = 11, i = 4, a < e, a < g, b < e, b < g, b < h, b < j, c < d, c < k, d < f, d < h, f < j, g < k, h < i
fn select_748([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_363([f, a, b, h, i, e, g, j])
    } else {
        select_598([c, b, e, g, d, f, h, k])
    }
}
/// n = 11, i = 4, a < f, b < e, b < h, c < d, c < f, c < i, c < k, d < h, e < g, e < i, g < k, i < j
fn select_749([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_747([d, b, a, e, f, h, g, i, j, k])
    } else {
        select_748([a, c, b, e, f, g, d, i, j, k, h])
    }
}
/// n = 11, i = 4, b < f, b < h, c < d, c < e, c < i, c < k, e < h, f < g, f < i, g < k, i < j
fn select_750([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_746([e, d, b, a, f, h, g, i, j, k])
    } else {
        select_749([a, b, c, e, f, d, g, h, i, j, k])
    }
}
/// n = 8, i = 3, b < h, c < e, c < h, d < e, d < h, e < f, e < g
fn select_751([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_67([b, e, a, f, g, h])
    } else {
        select_153([a, b, c, d, h, e])
    }
}
/// n = 10, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, d < j, f < j, g < h, g < i
fn select_752([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_311([a, f, d, g, h, i, e, j])
    } else {
        select_751([a, e, b, c, g, h, i, f])
    }
}
/// n = 11, i = 5, b < g, b < i, c < g, c < i, d < e, d < i, e < f, e < j, e < k, g < h, g < k, i < j
fn select_753([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_57([a, d, g, h, i, e, f, k])
    } else {
        select_752([a, b, c, e, f, i, g, h, k, j])
    }
}
/// n = 13, i = 5, b < i, c < f, c < i, d < e, d < i, e < g, e < j, e < l, f < h, f < j, f < m, h < l, i < m, j < k
fn select_754([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < b {
        select_750([a, c, e, g, b, f, h, i, j, k, l])
    } else {
        select_753([a, b, d, c, f, h, e, g, i, m, j])
    }
}
/// n = 13, i = 5, c < i, d < f, d < i, e < g, e < i, f < j, f < l, g < h, g < j, g < m, h < l, i < m, j < k
fn select_755([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < f {
        select_744([b, c, a, d, e, g, h, f, i, m, j])
    } else {
        select_754([b, c, e, d, f, g, a, h, i, j, k, l, m])
    }
}
/// n = 14, i = 5, d < f, d < i, e < g, e < i, f < k, f < n, g < h, g < j, g < k, h < n, i < j, j < l, k < m
fn select_756([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_735([b, c, g, d, h, f, i, a, j, k, l, m, n])
    } else {
        select_755([b, c, a, d, e, f, g, h, i, k, m, n, j])
    }
}
/// n = 14, i = 5, d < j, d < l, e < i, f < g, f < i, g < h, g < j, g < m, h < l, i < m, j < k, m < n
fn select_757([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < e {
        select_723([a, b, c, d, f, e, g, h, j, i, k, l, m])
    } else {
        select_756([a, b, c, e, f, d, g, h, i, m, j, n, k, l])
    }
}
/// n = 8, i = 3, a < d, b < c, b < g, b < h, c < f, d < e, d < g, d < h
fn select_758([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < c {
        select_19([e, b, g, h])
    } else {
        select_46([c, a, f, d, g, h])
    }
}
/// n = 10, i = 3, a < d, a < g, b < c, b < g, b < j, c < f, d < e, d < j, g < h, g < i
fn select_759([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_69([c, a, f, g, d, h, i])
    } else {
        select_758([a, b, c, d, e, f, g, j])
    }
}
/// n = 10, i = 3, a < c, a < j, b < d, b < e, c < i, d < h, d < j, e < f, e < g
fn select_760([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_636([d, e, f, g, h, a, c])
    } else {
        select_759([b, a, c, d, h, i, e, f, g, j])
    }
}
/// n = 8, i = 2, a < d, b < c, b < e, d < h, e < f, e < g
fn select_761([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_34([c, e, a, f, g])
    } else {
        select_42([b, a, c, d, h, e])
    }
}
/// n = 10, i = 3, a < f, b < f, b < g, c < d, c < g, d < e, f < j, g < h, g < i
fn select_762([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < c {
        select_2([c, j])
    } else {
        select_462([a, d, b, e, g, h, i, f])
    }
}
/// n = 10, i = 3, a < h, b < d, b < i, c < e, c < h, d < g, e < f, e < i, h < j
fn select_763([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_761([a, e, f, h, b, d, i, j])
    } else {
        select_762([a, c, b, d, g, h, e, f, i, j])
    }
}
/// n = 10, i = 3, b < e, b < j, c < d, c < f, d < h, e < i, f < g, f < j
fn select_764([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_760([b, c, e, f, d, a, h, g, i, j])
    } else {
        select_763([a, b, c, e, f, g, i, d, j, h])
    }
}
/// n = 9, i = 2, b < d, c < e, c < f, e < i, f < g, f < h
fn select_765([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_173([a, e, f, b, g, h, i])
    } else {
        select_275([a, b, c, d, e, i, f])
    }
}
/// n = 8, i = 2, b < d, b < h, c < e, c < h, e < f, e < g
fn select_766([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_44([b, c, d, e, f, g, h])
    } else {
        select_275([a, b, c, d, e, g, h])
    }
}
/// n = 10, i = 3, b < e, b < h, c < d, c < h, d < f, e < g, h < i, h < j
fn select_767([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_766([a, d, b, f, h, i, j, e])
    } else {
        select_766([a, e, c, g, h, i, j, d])
    }
}
/// n = 11, i = 3, b < e, b < k, c < d, c < f, d < g, e < h, f < i, f < k, i < j
fn select_768([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_765([a, d, f, g, i, b, e, k, j])
    } else {
        select_767([a, b, c, d, e, g, h, f, i, k])
    }
}
/// n = 12, i = 3, b < e, b < f, c < d, c < g, d < h, e < j, f < i, g < k, k < l
fn select_769([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_764([a, c, b, e, d, f, i, j, h, g])
    } else {
        select_768([a, b, c, d, e, g, h, j, k, l, f])
    }
}
/// n = 9, i = 3, c < h, c < i, d < e, e < f, e < h, e < i, f < g
fn select_770([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_311([b, c, e, f, a, g, h, i])
    } else {
        select_164([b, a, c, d, e, h, i, f])
    }
}
/// n = 11, i = 4, b < f, b < k, c < e, c < g, d < f, d < g, f < h, g < i, g < k, i < j
fn select_771([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_308([a, c, b, d, e, f, h, g])
    } else {
        select_770([a, e, b, d, g, i, j, f, k])
    }
}
/// n = 9, i = 3, a < d, a < h, b < c, b < h, b < i, c < e, c < f, d < g, d < i
fn select_772([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_105([a, c, e, f, h, d])
    } else {
        select_35([b, d, g, h, c, i])
    }
}
/// n = 10, i = 4, a < h, b < d, b < g, c < e, d < f, d < i, e < g, e < h, e < i, h < j
fn select_773([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_479([c, d, a, f, e, g, i])
    } else {
        select_441([b, a, c, e, d, f, g, h, j])
    }
}
/// n = 10, i = 4, b < d, b < i, c < e, d < f, d < j, e < g, e < i, e < j, g < h
fn select_774([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_772([b, e, g, d, a, h, f, i, j])
    } else {
        select_773([a, b, c, d, e, f, i, g, j, h])
    }
}
/// n = 7, i = 4, b < f, b < g, c < f, c < g, d < e, e < g
fn select_775([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_75([b, c, e, a, f, g])
    } else {
        select_156([b, c, a, d, e, f])
    }
}
/// n = 9, i = 4, b < i, c < f, d < e, d < i, e < g, e < h, f < g, f < h, f < i
fn select_776([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < e {
        select_775([a, b, d, c, f, e, i])
    } else {
        select_163([a, c, e, b, f, g, h])
    }
}
/// n = 11, i = 5, b < e, b < g, c < g, c < h, d < f, f < h, f < j, f < k, g < j, g < k, h < i
fn select_777([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_312([a, b, f, h, g, i, j, k, e])
    } else {
        select_776([a, e, d, c, g, f, j, k, h])
    }
}
/// n = 11, i = 5, b < e, b < k, c < g, d < f, d < h, f < j, f < k, g < h, g < j, g < k, h < i
fn select_778([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_774([a, f, c, b, g, e, h, i, j, k])
    } else {
        select_777([a, b, d, c, e, g, f, h, i, j, k])
    }
}
/// n = 13, i = 5, b < f, b < h, c < e, c < i, d < g, f < k, g < h, g < i, g < k, h < j, i < l, l < m
fn select_779([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_771([a, b, c, g, e, h, i, j, l, m, f])
    } else {
        select_778([a, c, d, b, e, f, g, h, j, k, i])
    }
}
/// n = 14, i = 5, b < e, c < f, c < i, d < g, f < l, g < h, g < i, g < l, h < j, i < k, j < m, m < n
fn select_780([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < b {
        select_769([a, c, h, b, f, i, j, e, k, l, m, n])
    } else {
        select_779([a, c, b, d, e, f, g, i, h, k, l, j, m])
    }
}
/// n = 14, i = 5, d < f, d < i, e < g, f < l, g < h, g < i, g < l, h < j, i < k, j < m, m < n
fn select_781([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_780([c, a, d, e, b, f, g, h, i, j, k, l, m, n])
    } else {
        select_780([c, b, d, e, a, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 8, i = 5, a < f, b < g, c < g, d < g, e < f, e < h, g < h
fn select_782([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_87([a, e, g, h])
    } else {
        select_8([b, c, d, f])
    }
}
/// n = 9, i = 5, b < f, c < g, d < g, e < f, e < h, g < h, h < i
fn select_783([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_150([b, e, g, a, f, h, i])
    } else {
        select_782([b, a, c, d, e, f, g, h])
    }
}
/// n = 10, i = 5, c < g, d < g, e < f, e < h, f < j, g < h, g < j, h < i
fn select_784([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_146([b, f, c, d, a, g, h, i, j])
    } else {
        select_783([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 7, i = 4, a < g, b < f, b < g, c < e, c < g, d < e, d < f
fn select_785([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_52([a, b, d, e, g, f])
    } else {
        select_52([a, c, d, f, g, e])
    }
}
/// n = 9, i = 6, a < h, b < h, b < i, c < g, d < g, e < f, e < h, g < i
fn select_786([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_782([a, c, d, f, b, h, g, i])
    } else {
        select_785([a, b, e, g, f, i, h])
    }
}
/// n = 10, i = 6, b < h, c < h, d < g, d < i, e < f, e < g, g < j, h < i, h < j
fn select_787([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_308([f, g, b, c, a, h, i, j])
    } else {
        select_786([a, d, b, c, e, f, h, g, i])
    }
}
/// n = 11, i = 6, b < i, c < h, d < h, e < f, e < g, g < j, h < i, h < j, j < k
fn select_788([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_784([a, f, c, d, g, b, h, j, k, i])
    } else {
        select_787([a, c, d, b, e, f, g, h, i, j])
    }
}
/// n = 10, i = 5, c < j, d < g, e < g, f < g, g < h, g < j, h < i
fn select_789([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_387([b, c, a, d, e, f, g, h])
    } else {
        select_59([b, c, g, a, h, i, j])
    }
}
/// n = 12, i = 6, b < i, b < l, c < h, c < j, d < h, d < j, e < f, e < g, g < j, h < i, h < k, j < k, j < l
fn select_790([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < j {
        select_788([a, b, c, d, e, f, g, h, i, j, k])
    } else {
        select_789([a, f, b, c, d, g, j, h, k, l])
    }
}
/// n = 8, i = 4, b < f, b < h, c < f, c < h, d < e, d < g, e < h, f < g
fn select_791([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_126([e, b, c, a, f, g, h])
    } else {
        select_157([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 4, a < g, b < f, b < i, c < f, c < i, d < e, d < h, e < g, e < i, f < h
fn select_792([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_302([b, c, a, e, g, i])
    } else {
        select_791([a, b, c, d, e, f, h, i])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, c < g, d < f, e < g, e < h, f < h, g < i
fn select_793([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_488([a, c, d, e, f, g, i, h])
    } else {
        select_302([b, c, a, e, h, g])
    }
}
/// n = 10, i = 5, a < f, b < h, b < j, c < h, c < j, d < g, d < i, e < g, e < h, g < j, h < i
fn select_794([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_792([a, b, c, d, g, h, f, i, j])
    } else {
        select_793([d, b, c, a, e, f, h, g, i])
    }
}
/// n = 7, i = 3, a < g, b < d, b < f, c < e, e < f, e < g
fn select_795([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < a {
        select_42([b, e, d, a, g, f])
    } else {
        select_26([b, a, c, d, e, g])
    }
}
/// n = 6, i = 3, a < f, b < f, c < d, d < e
fn select_796([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_52([b, a, c, d, f, e])
    } else {
        select_3([b, e, a])
    }
}
/// n = 8, i = 4, a < g, b < f, b < h, c < f, c < h, d < e, e < g, e < h
fn select_797([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_795([a, c, d, f, e, h, g])
    } else {
        select_796([b, c, d, e, g, f])
    }
}
/// n = 4, i = 2, c < d
fn select_798([a, b, c, d]: [usize; 4]) -> usize {
    if a < b {
        select_13([a, c, d, b])
    } else {
        select_13([b, c, d, a])
    }
}
/// n = 8, i = 5, a < g, b < g, c < g, c < h, d < f, e < f, f < h
fn select_799([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_157([b, d, e, c, g, f, h])
    } else {
        select_798([a, b, c, h])
    }
}
/// n = 9, i = 5, a < f, b < h, b < i, c < h, c < i, d < g, d < h, e < f, e < g, g < i
fn select_800([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_797([a, b, c, e, g, h, f, i])
    } else {
        select_799([b, c, d, a, e, f, h, g])
    }
}
/// n = 11, i = 6, a < h, b < i, b < j, c < i, c < j, d < f, d < k, e < g, e < h, f < i, g < j, j < k
fn select_801([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_794([a, b, c, d, g, h, f, j, k, i])
    } else {
        select_800([a, b, c, f, e, h, g, i, j])
    }
}
/// n = 8, i = 4, a < e, b < g, b < h, c < g, c < h, d < f, d < g, f < h
fn select_802([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_292([a, b, d, e, f, c, g, h])
    } else {
        select_292([a, c, d, e, f, b, g, h])
    }
}
/// n = 10, i = 5, a < f, b < h, c < h, c < i, d < h, d < i, e < g, e < j, g < i, h < j
fn select_803([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < g {
        select_794([a, c, d, e, b, f, g, h, j, i])
    } else {
        select_802([a, c, d, g, f, b, i, h])
    }
}
/// n = 11, i = 6, b < i, b < j, c < i, c < j, d < g, d < k, e < f, e < h, g < i, h < j, j < k
fn select_804([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_801([a, b, c, d, e, g, h, f, i, j, k])
    } else {
        select_803([f, h, b, c, d, a, g, j, i, k])
    }
}
/// n = 12, i = 6, b < g, b < l, c < i, c < j, d < i, d < j, e < f, e < h, h < j, i < k, j < k, j < l
fn select_805([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_790([a, b, c, d, e, f, h, i, g, j, k, l])
    } else {
        select_804([a, c, d, b, e, f, g, h, i, j, l])
    }
}
/// n = 11, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, c < k, e < k, f < j, g < h, g < i
fn select_806([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_427([a, d, b, f, g, e, h, i, j])
    } else {
        select_710([a, b, c, d, e, f, j, g, k])
    }
}
/// n = 12, i = 4, a < d, a < e, a < i, b < c, b < f, d < l, e < h, e < k, f < g, f < i, f < k, f < l, i < j
fn select_807([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_119([d, b, e, h, c, f, g, i, k])
    } else {
        select_806([c, a, f, g, d, i, e, h, k, j, l])
    }
}
/// n = 13, i = 4, a < e, a < j, b < f, b < i, c < d, c < g, f < l, g < h, g < i, g < j, g < l, i < k, j < m
fn select_808([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_234([c, a, e, d, b, g, h, f, j, m, i])
    } else {
        select_807([b, c, d, f, a, g, h, e, i, k, j, l])
    }
}
/// n = 10, i = 3, b < e, b < f, b < g, c < d, c < f, c < g, c < j, e < j, f < i, g < h
fn select_809([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_681([a, d, b, g, f, e, i, h])
    } else {
        select_710([a, b, c, d, e, g, h, f, j])
    }
}
/// n = 11, i = 3, b < e, b < h, c < d, c < f, c < h, c < k, e < k, f < g, g < i, h < j
fn select_810([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_681([a, d, f, g, b, e, h, i])
    } else {
        select_809([a, b, c, d, e, f, h, j, g, k])
    }
}
/// n = 7, i = 2, c < d, d < e, d < f, d < g
fn select_811([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_69([a, c, b, d, e, f, g])
    } else {
        select_69([b, c, a, d, e, f, g])
    }
}
/// n = 10, i = 5, a < g, a < h, b < g, b < h, c < f, c < h, d < e, d < h, h < i, h < j
fn select_812([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_575([a, b, d, c, f, e, g, h])
    } else {
        select_811([f, g, d, h, e, i, j])
    }
}
/// n = 7, i = 4, a < f, a < g, b < f, b < g, c < f, c < g, d < e, d < g
fn select_813([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_9([e, a, b, c, g])
    } else {
        select_3([f, d, g])
    }
}
/// n = 11, i = 6, a < h, a < i, b < h, b < i, c < h, c < i, d < g, d < i, e < f, e < i, i < j, i < k
fn select_814([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_812([b, c, d, e, f, g, h, i, j, k])
    } else {
        select_813([a, b, c, d, g, h, i])
    }
}
/// n = 12, i = 6, a < g, a < l, b < h, b < i, c < h, c < i, d < h, d < i, e < f, e < i, h < k, i < j, i < k, i < l
fn select_815([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < i {
        select_814([b, c, d, a, e, f, g, h, i, j, l])
    } else {
        select_326([f, h, i, j, a, g, l, k])
    }
}
/// n = 14, i = 6, a < h, b < g, b < k, c < h, c < i, d < h, d < i, e < f, e < i, g < m, h < n, i < j, i < k, i < m, i < n, k < l
fn select_816([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_810([f, b, i, j, g, a, h, k, n, l, m])
    } else {
        select_815([b, a, c, d, e, f, g, h, i, j, n, k])
    }
}
/// n = 15, i = 6, b < g, b < l, c < h, c < i, d < h, d < i, e < f, e < i, g < o, h < k, i < j, i < k, i < l, i < o, k < m, l < n
fn select_817([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < a {
        select_808([h, b, e, f, a, g, i, j, l, k, n, o, m])
    } else {
        select_816([a, b, c, d, e, f, g, h, i, j, l, n, o, k])
    }
}
/// n = 15, i = 6, b < h, b < l, c < i, c < j, d < i, d < j, e < f, e < g, e < j, h < o, i < k, j < k, j < l, j < o, k < m, l < n
fn select_818([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < j {
        select_805([a, b, c, d, e, g, h, f, i, j, k, l])
    } else {
        select_817([a, b, c, d, e, g, h, i, j, f, k, l, m, n, o])
    }
}
/// n = 15, i = 6, b < i, c < h, c < k, d < i, d < j, e < f, e < g, e < j, h < m, i < n, j < k, j < m, j < n, k < l, n < o
fn select_819([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < b {
        select_781([a, f, g, c, d, h, j, b, k, i, l, m, n, o])
    } else {
        select_818([a, c, b, d, e, f, g, h, i, j, n, k, o, l, m])
    }
}
/// n = 15, i = 6, b < i, c < h, c < l, d < i, d < j, e < f, e < g, e < j, i < n, j < k, j < l, j < n, l < m, n < o
fn select_820([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < h {
        select_757([a, f, g, c, b, d, j, k, i, l, m, h, n, o])
    } else {
        select_819([a, b, c, d, e, f, g, h, i, j, l, m, k, n, o])
    }
}
/// n = 15, i = 6, c < h, c < m, d < i, d < j, e < f, e < g, e < j, i < l, j < k, j < l, j < m, l < n, m < o
fn select_821([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < a {
        select_702([b, c, d, e, f, g, h, i, a, j, k, l, m, n, o])
    } else {
        select_820([b, a, c, d, e, f, g, h, i, j, k, m, o, l, n])
    }
}
/// n = 15, i = 6, c < i, c < n, d < h, d < j, e < f, e < g, e < j, j < k, j < l, j < n, l < m, n < o
fn select_822([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if l < h {
        select_635([a, b, f, g, c, d, i, j, k, l, h, m, n, o])
    } else {
        select_821([a, b, c, d, e, f, g, i, h, j, k, l, n, m, o])
    }
}
/// n = 9, i = 4, a < g, b < e, c < h, d < f, f < g, f < h, h < i
fn select_823([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < a {
        select_111([b, d, c, e, a, f, h, i, g])
    } else {
        select_292([b, d, a, e, c, f, g, h])
    }
}
/// n = 10, i = 5, a < i, b < g, c < f, d < h, e < g, e < h, f < j, h < i, h < j
fn select_824([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_823([a, b, c, d, g, h, i, f, j])
    } else {
        select_374([a, b, d, e, c, f, g, h, j])
    }
}
/// n = 8, i = 5, a < h, b < g, c < e, d < f, e < h, f < g
fn select_825([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_124([b, a, d, h, f, g])
    } else {
        select_124([a, b, c, g, e, h])
    }
}
/// n = 9, i = 5, a < h, b < g, b < i, c < e, d < f, e < h, e < i, f < g, f < i
fn select_826([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < i {
        select_825([a, b, c, d, e, f, g, h])
    } else {
        select_3([g, e, i])
    }
}
/// n = 9, i = 6, a < h, b < f, c < e, d < g, e < i, f < i, g < h
fn select_827([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < i {
        select_528([a, g, b, c, e, f, h, i])
    } else {
        select_87([a, d, i, g])
    }
}
/// n = 10, i = 6, a < i, b < e, b < j, c < f, d < g, f < i, f < j, g < h, h < j
fn select_828([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_826([a, b, c, g, f, h, e, i, j])
    } else {
        select_827([a, b, d, c, g, e, f, i, h])
    }
}
/// n = 9, i = 6, a < i, b < e, c < f, d < g, f < i, g < h
fn select_829([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < h {
        select_827([a, b, d, c, g, e, f, i, h])
    } else {
        select_825([a, b, c, g, f, h, e, i])
    }
}
/// n = 10, i = 6, b < j, c < i, d < f, e < g, f < i, f < j, g < h, h < j
fn select_830([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < a {
        select_828([c, b, d, e, a, f, g, h, i, j])
    } else {
        select_829([c, a, d, e, b, f, g, h, i])
    }
}
/// n = 11, i = 6, a < k, b < i, c < j, d < f, e < g, f < h, g < i, g < j, j < k
fn select_831([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if j < h {
        select_824([a, b, d, c, g, f, i, j, k, h])
    } else {
        select_830([a, c, b, e, d, g, f, h, i, j])
    }
}
/// n = 11, i = 6, a < k, b < i, c < j, d < f, d < g, e < h, h < i, h < j, j < k
fn select_832([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_831([a, b, c, d, e, f, h, g, i, j, k])
    } else {
        select_831([a, b, c, d, e, g, h, f, i, j, k])
    }
}
/// n = 11, i = 6, a < k, b < j, c < h, d < i, e < f, e < g, h < k, i < j
fn select_833([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_832([b, a, d, e, c, f, g, h, k, i, j])
    } else {
        select_832([a, b, c, e, d, f, g, i, j, h, k])
    }
}
/// n = 9, i = 5, a < i, b < f, c < g, d < g, e < g, g < h, g < i
fn select_834([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < f {
        select_10([a, b, d, e, f, g, h, i])
    } else {
        select_9([a, c, d, e, g])
    }
}
/// n = 9, i = 5, a < g, b < f, c < h, d < h, e < g, e < h, h < i
fn select_835([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_834([a, b, c, d, e, f, h, i, g])
    } else {
        select_356([c, d, b, a, e, f, g, h])
    }
}
/// n = 8, i = 2, a < c, a < h, b < d, b < e, b < h, d < g, e < f
fn select_836([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_44([a, b, c, d, e, g, h])
    } else {
        select_44([a, b, c, e, d, f, h])
    }
}
/// n = 10, i = 4, a < d, a < j, b < f, c < e, c < g, c < j, e < i, f < g, f < i, f < j, g < h
fn select_837([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_836([a, c, d, e, g, h, i, j])
    } else {
        select_163([d, b, c, e, f, g, j])
    }
}
/// n = 10, i = 4, a < d, a < j, b < e, b < j, c < f, e < i, f < g, f < i, f < j, g < h
fn select_838([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_51([a, g, h, d, b, e])
    } else {
        select_837([a, c, b, d, e, f, g, h, i, j])
    }
}
/// n = 11, i = 5, a < i, b < g, b < k, c < f, c < k, d < h, e < g, e < h, f < j, h < i, h < j, h < k
fn select_839([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_835([b, c, a, d, e, f, g, h, i])
    } else {
        select_838([b, c, d, g, f, h, a, i, j, k])
    }
}
/// n = 12, i = 6, a < k, b < i, b < l, c < j, d < f, d < l, e < g, f < h, g < i, g < j, j < k, j < l
fn select_840([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < h {
        select_839([a, b, d, c, g, f, i, j, k, h, l])
    } else {
        select_830([a, c, b, e, d, g, f, h, i, j])
    }
}
/// n = 12, i = 6, a < k, b < i, b < l, c < j, d < f, d < g, d < l, e < h, h < i, h < j, j < k, j < l
fn select_841([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_840([a, b, c, d, e, f, h, g, i, j, k, l])
    } else {
        select_840([a, b, c, d, e, g, h, f, i, j, k, l])
    }
}
/// n = 9, i = 4, a < f, a < g, b < e, c < d, c < f, d < h, d < i, e < g, e < h, e < i
fn select_842([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_514([a, b, d, f, e, h, i])
    } else {
        select_256([a, c, e, g, f, d, h, i])
    }
}
/// n = 5, i = 3, b < e, c < d
fn select_843([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_87([a, b, d, e])
    } else {
        select_87([a, c, e, d])
    }
}
/// n = 8, i = 4, a < f, b < g, b < h, c < f, c < g, c < h, d < e, e < g, e < h
fn select_844([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_154([a, b, c, g, h, f])
    } else {
        select_843([b, c, d, e, f])
    }
}
/// n = 10, i = 5, a < g, b < e, b < i, b < j, c < d, c < f, e < h, f < i, f < j, g < h, g < i, g < j
fn select_845([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < d {
        select_842([e, a, c, f, g, d, h, i, j])
    } else {
        select_844([d, f, b, a, g, e, i, j])
    }
}
/// n = 8, i = 4, a < g, b < e, c < d, c < g, d < h, e < f, e < h
fn select_846([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_124([d, a, b, g, e, h])
    } else {
        select_53([a, c, e, f, d, h])
    }
}
/// n = 11, i = 6, a < e, b < f, c < d, c < i, c < k, d < j, e < i, e < j, e < k, f < g, f < h, h < k
fn select_847([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_845([a, c, f, g, d, h, e, j, i, k])
    } else {
        select_846([d, b, e, i, f, g, j, h])
    }
}
/// n = 8, i = 4, a < f, a < g, b < e, c < d, c < f, d < h, e < g, e < h
fn select_848([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_139([a, c, e, g, f, d, h])
    } else {
        select_26([a, b, d, f, e, h])
    }
}
/// n = 9, i = 5, a < g, b < f, b < i, c < d, c < e, e < i, f < h, g < h, g < i
fn select_849([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_848([f, a, c, e, g, d, h, i])
    } else {
        select_158([e, d, b, a, g, f, i])
    }
}
/// n = 10, i = 6, a < e, b < f, c < d, c < i, d < j, e < i, e < j, f < g, f < h
fn select_850([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < i {
        select_849([a, c, f, h, g, d, e, j, i])
    } else {
        select_846([d, b, e, i, f, h, j, g])
    }
}
/// n = 11, i = 6, a < j, b < i, b < k, c < e, d < f, e < i, e < j, e < k, f < g, f < h, h < k
fn select_851([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < a {
        select_847([c, d, b, a, e, f, g, h, i, j, k])
    } else {
        select_850([c, d, a, b, e, f, g, h, j, i])
    }
}
/// n = 8, i = 4, a < g, a < h, b < f, b < h, c < d, c < e, e < h, f < g
fn select_852([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_143([a, f, c, e, d, g, h])
    } else {
        select_302([a, e, d, b, f, h])
    }
}
/// n = 11, i = 4, a < f, a < h, b < f, b < h, c < d, c < h, c < k, d < e, f < g, f < k, h < i, h < j
fn select_853([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_243([f, c, g, h, i, j, d, k])
    } else {
        select_185([d, a, b, e, h, i, j, f])
    }
}
/// n = 11, i = 4, a < g, a < h, a < k, b < e, b < h, c < d, c < h, c < k, d < f, e < g, e < k, h < i, h < j
fn select_854([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_758([e, c, d, a, g, f, h, k])
    } else {
        select_853([a, b, c, d, f, e, g, h, i, j, k])
    }
}
/// n = 11, i = 4, a < e, b < h, b < k, c < g, c < h, c < k, d < f, d < h, f < g, f < k, h < i, h < j
fn select_855([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_852([c, d, a, e, b, f, g, h])
    } else {
        select_854([c, d, b, a, f, e, g, h, i, j, k])
    }
}
/// n = 7, i = 4, a < g, b < f, b < g, c < f, c < g, d < e, d < g
fn select_856([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_153([a, e, b, c, f, g])
    } else {
        select_11([a, f, d, e, g])
    }
}
/// n = 10, i = 6, a < g, a < j, b < g, b < j, c < f, d < e, d < h, e < j, f < h, f < j, g < i
fn select_857([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < h {
        select_580([g, c, d, i, e, f, j])
    } else {
        select_856([e, a, b, f, h, g, j])
    }
}
/// n = 13, i = 6, a < g, a < j, b < g, b < j, c < f, d < e, d < j, d < m, e < i, f < i, f < j, f < m, g < h, g < m, j < k, j < l
fn select_858([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_163([h, c, d, e, f, j, m])
    } else {
        select_853([a, b, d, e, i, g, h, j, k, l, m])
    }
}
/// n = 13, i = 6, a < i, b < j, b < m, c < g, c < j, d < g, d < j, e < f, f < i, f < j, f < m, g < h, g < m, j < k, j < l
fn select_859([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_857([c, d, e, a, b, f, g, i, h, j])
    } else {
        select_858([c, d, e, b, a, f, g, h, i, j, k, l, m])
    }
}
/// n = 14, i = 6, a < i, b < j, b < l, c < g, c < l, d < g, d < k, d < l, e < f, f < i, f < j, f < l, g < h, j < k, l < m, l < n
fn select_860([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < g {
        select_855([a, c, d, b, i, j, k, l, m, n, g])
    } else {
        select_859([a, b, c, d, e, f, g, h, i, l, m, n, j])
    }
}
/// n = 14, i = 6, a < i, b < k, b < l, c < j, c < l, d < f, d < l, e < g, f < h, g < i, g < j, g < l, j < k, l < m, l < n
fn select_861([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < b {
        select_851([a, c, e, d, g, f, h, b, j, i, l])
    } else {
        select_860([a, c, d, b, e, g, f, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 6, a < i, b < k, b < l, c < j, c < l, d < f, d < g, d < l, e < h, h < i, h < j, h < l, j < k, l < m, l < n
fn select_862([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < g {
        select_861([a, b, c, d, e, f, h, g, i, j, k, l, m, n])
    } else {
        select_861([a, b, c, d, e, g, h, f, i, j, k, l, m, n])
    }
}
/// n = 14, i = 6, a < k, b < j, b < l, c < i, d < h, d < l, e < f, e < g, e < l, h < j, i < k, i < l, l < m, l < n
fn select_863([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_841([a, b, c, e, d, f, g, h, j, i, k, l])
    } else {
        select_862([a, b, d, e, c, f, g, i, k, h, j, l, m, n])
    }
}
/// n = 14, i = 6, a < h, b < k, b < l, c < j, d < i, d < l, e < f, e < g, e < l, i < k, j < l, l < m, l < n
fn select_864([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_833([b, c, d, a, e, f, g, i, h, j, k])
    } else {
        select_863([a, b, c, d, e, f, g, i, j, k, h, l, m, n])
    }
}
/// n = 9, i = 4, a < i, b < f, b < h, c < f, c < g, d < e, d < g, g < h, g < i
fn select_865([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_372([a, b, c, d, e, f, g])
    } else {
        select_164([e, a, b, c, g, f, h, i])
    }
}
/// n = 10, i = 6, a < h, b < f, c < e, d < g, e < i, e < j, f < i, f < j, g < h, g < j
fn select_866([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_528([a, g, b, c, e, f, h, j])
    } else {
        select_775([a, e, f, d, g, i, j])
    }
}
/// n = 10, i = 6, a < i, b < e, c < f, d < g, e < j, f < i, f < j, g < h, g < j
fn select_867([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_866([a, b, d, c, g, e, f, i, h, j])
    } else {
        select_825([a, b, c, g, f, h, e, i])
    }
}
/// n = 11, i = 6, a < e, b < i, b < k, c < f, d < g, f < i, f < j, g < h, g < j, j < k
fn select_868([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if j < e {
        select_865([a, b, f, g, h, i, j, k, e])
    } else {
        select_867([b, a, c, d, e, f, g, h, i, j])
    }
}
/// n = 8, i = 3, a < e, a < h, b < f, b < h, c < d, c < f, c < h, f < g
fn select_869([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_45([a, c, d, e, f, g, h])
    } else {
        select_275([d, a, b, e, f, g, h])
    }
}
/// n = 8, i = 3, b < e, b < h, c < f, c < h, d < f, d < h, f < g
fn select_870([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_124([d, b, a, e, c, f])
    } else {
        select_869([b, d, c, a, e, f, g, h])
    }
}
/// n = 9, i = 2, a < d, b < c, b < e, e < f, e < g, e < h, e < i
fn select_871([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < a {
        select_68([c, e, f, g, h, i, a])
    } else {
        select_190([a, b, d, e, f, g, h, i, c])
    }
}
/// n = 8, i = 3, a < g, a < h, b < d, b < h, c < e, c < h, d < g, e < f
fn select_872([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_21([a, c, d, e, g, h])
    } else {
        select_22([a, b, e, f, d, h])
    }
}
/// n = 8, i = 3, a < e, a < h, b < d, b < h, c < f, c < h, f < g
fn select_873([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_872([a, b, c, d, f, g, e, h])
    } else {
        select_872([b, a, c, e, f, g, d, h])
    }
}
/// n = 11, i = 3, a < e, b < d, b < h, c < f, c < h, f < g, h < i, h < j, h < k
fn select_874([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_871([f, b, d, g, h, a, i, j, k])
    } else {
        select_873([a, b, c, d, e, f, g, h])
    }
}
/// n = 12, i = 4, a < f, b < e, b < i, c < g, c < i, d < g, d < i, g < h, i < j, i < k, i < l
fn select_875([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < c {
        select_870([f, b, c, d, e, g, h, i])
    } else {
        select_874([a, b, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 14, i = 6, a < e, b < j, b < k, c < f, c < k, d < g, f < h, g < i, g < j, i < k, k < l, k < m, k < n
fn select_876([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < i {
        select_868([a, b, d, c, e, g, f, h, j, i, k])
    } else {
        select_875([a, b, c, i, j, e, f, h, k, l, m, n])
    }
}
/// n = 14, i = 6, a < g, b < j, b < k, c < e, c < f, c < k, d < h, h < i, h < j, i < k, k < l, k < m, k < n
fn select_877([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < f {
        select_876([a, b, c, d, g, e, h, f, i, j, k, l, m, n])
    } else {
        select_876([a, b, c, d, g, f, h, e, i, j, k, l, m, n])
    }
}
/// n = 8, i = 4, a < e, a < h, b < e, b < g, c < f, d < f, f < g, f < h
fn select_878([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_11([a, b, f, g, h])
    } else {
        select_7([c, d, e])
    }
}
/// n = 9, i = 4, a < e, a < i, b < e, b < g, c < f, c < g, d < f, d < g, f < i, g < h
fn select_879([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_878([a, b, c, d, e, f, g, i])
    } else {
        select_514([a, c, d, e, g, f, h])
    }
}
/// n = 10, i = 5, a < h, a < i, b < f, b < i, c < f, c < i, d < e, d < i, e < h, f < g, i < j
fn select_880([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_879([e, a, b, c, h, f, i, j, g])
    } else {
        select_642([a, f, d, g, i, j, e])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < e, d < g
fn select_881([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_159([a, c, d, e, f, g])
    } else {
        select_15([b, c, d, e, g, f])
    }
}
/// n = 6, i = 3, b < f, c < e, c < f, d < e, d < f
fn select_882([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < a {
        select_75([c, d, b, a, e, f])
    } else {
        select_162([c, d, a, b, e])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, c < f, c < i, d < e, d < g, f < h, g < i
fn select_883([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_881([b, a, f, d, e, h, g])
    } else {
        select_882([e, a, c, g, f, i])
    }
}
/// n = 11, i = 6, a < i, a < j, b < g, c < e, c < j, d < f, d < j, e < i, f < h, g < j, j < k
fn select_884([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_880([a, d, g, c, e, f, h, i, j, k])
    } else {
        select_883([a, b, c, f, h, e, g, i, j])
    }
}
/// n = 11, i = 6, a < i, a < j, b < h, c < g, c < j, d < e, d < f, d < j, g < i, h < j, j < k
fn select_885([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_884([a, b, c, d, g, e, h, f, i, j, k])
    } else {
        select_884([a, b, c, d, g, f, h, e, i, j, k])
    }
}
/// n = 10, i = 4, a < e, b < g, c < f, c < g, d < f, d < g, g < h, g < i, g < j
fn select_886([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_637([b, c, d, a, e, f])
    } else {
        select_811([a, f, b, g, h, i, j])
    }
}
/// n = 10, i = 4, c < g, d < f, d < g, e < f, e < g, g < h, g < i, g < j
fn select_887([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_886([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_886([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 10, i = 4, c < f, c < j, d < g, e < f, e < g, g < h, g < i, g < j
fn select_888([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < c {
        select_117([a, b, g, h, i, c, f, j])
    } else {
        select_887([a, b, d, c, e, f, g, h, i, j])
    }
}
/// n = 12, i = 6, a < i, a < j, b < g, b < j, c < h, c < j, d < e, d < f, d < j, h < i, h < l, j < k, j < l
fn select_889([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < j {
        select_885([a, b, c, d, e, f, h, g, i, j, k])
    } else {
        select_888([e, f, h, b, a, i, j, g, k, l])
    }
}
/// n = 14, i = 6, a < g, b < i, b < j, c < h, c < j, d < e, d < f, d < j, h < i, h < l, j < k, j < l, l < m, l < n
fn select_890([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < a {
        select_542([e, f, h, j, k, i, a, g, l, m, n])
    } else {
        select_889([b, a, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 14, i = 6, a < g, b < j, b < k, c < e, c < f, c < k, d < h, d < k, h < i, h < j, i < m, i < n, k < l, k < m, k < n
fn select_891([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < k {
        select_877([a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    } else {
        select_890([a, b, d, c, e, f, g, h, j, k, l, i, m, n])
    }
}
/// n = 8, i = 1, c < h, d < e, d < f, d < g
fn select_892([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_173([b, c, d, e, f, g, h])
    } else {
        select_339([a, b, c, d, f, g, h])
    }
}
/// n = 10, i = 2, b < e, b < g, b < j, c < d, c < f, c < j, f < h, g < i
fn select_893([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_892([a, e, g, c, d, f, j, i])
    } else {
        select_892([a, d, f, b, e, g, j, h])
    }
}
/// n = 10, i = 3, a < f, a < h, b < e, b < g, c < d, c < f, c < g, c < h, g < i, h < j
fn select_894([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_344([b, c, d, f, e, h, j, g])
    } else {
        select_364([e, a, c, d, g, i, f, h])
    }
}
/// n = 11, i = 3, a < e, b < h, b < i, b < k, c < d, c < f, c < h, c < i, f < g, f < k, i < j
fn select_895([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_893([d, b, f, g, h, a, i, e, j, k])
    } else {
        select_894([b, a, c, d, e, h, f, i, g, j])
    }
}
/// n = 11, i = 3, a < c, a < f, a < k, b < d, b < f, c < e, d < g, d < h, d < k, g < j, h < i
fn select_896([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_625([g, h, a, c, f, k, i, j])
    } else {
        select_422([c, b, f, e, d, g, h, k])
    }
}
/// n = 11, i = 3, a < c, a < h, a < k, b < d, b < h, c < g, d < e, d < f, d < i, d < k, i < j
fn select_897([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_896([a, b, c, d, g, h, e, i, j, f, k])
    } else {
        select_896([a, b, c, d, g, h, f, i, j, e, k])
    }
}
/// n = 9, i = 3, a < g, b < d, b < e, b < f, c < e, c < f, c < g, f < h, g < i
fn select_898([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_761([a, b, e, g, f, d, h, i])
    } else {
        select_140([a, b, c, d, e, g, f, i])
    }
}
/// n = 8, i = 4, a < f, a < g, b < e, b < h, c < d, c < f, e < g, f < h
fn select_899([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_139([e, a, c, d, g, f, h])
    } else {
        select_52([d, b, a, f, e, h])
    }
}
/// n = 9, i = 4, a < g, a < h, b < d, b < g, c < e, c < i, e < f, e < h, g < i
fn select_900([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_899([a, c, b, d, e, g, h, i])
    } else {
        select_16([a, b, e, f, d, h])
    }
}
/// n = 11, i = 4, a < d, a < g, b < g, b < h, b < i, c < e, c < k, e < f, e < h, e < i, g < k, i < j
fn select_901([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < d {
        select_898([a, e, b, f, h, i, g, j, k])
    } else {
        select_900([b, a, c, d, e, f, g, h, k])
    }
}
/// n = 12, i = 4, a < d, b < e, b < h, b < j, c < f, c < i, e < i, e < l, f < g, f < h, f < j, f < l, j < k
fn select_902([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_897([e, c, a, f, g, h, d, i, j, k, l])
    } else {
        select_901([a, b, c, d, f, g, e, h, j, k, i])
    }
}
/// n = 13, i = 4, a < d, b < e, b < j, b < l, c < f, c < k, e < k, f < g, f < h, f < j, f < l, h < i, l < m
fn select_903([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_895([a, b, f, g, d, h, i, j, l, m, e])
    } else {
        select_902([a, b, c, d, e, f, g, j, k, l, m, h])
    }
}
/// n = 10, i = 6, a < i, b < f, c < h, c < i, d < g, e < g, f < h, g < i, i < j
fn select_904([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_642([a, b, c, f, i, j, h])
    } else {
        select_637([a, d, e, c, h, g])
    }
}
/// n = 11, i = 6, a < i, b < i, b < j, c < f, d < h, d < j, e < h, e < j, f < g, g < j, h < i, i < k
fn select_905([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_904([b, c, a, d, e, f, h, g, i, k])
    } else {
        select_856([b, d, e, g, a, h, j])
    }
}
/// n = 5, i = 3, c < e, d < e
fn select_906([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_162([c, d, a, b, e])
    } else {
        select_162([c, d, b, a, e])
    }
}
/// n = 6, i = 4, c < f, d < e
fn select_907([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_906([a, b, c, e, f])
    } else {
        select_906([a, b, d, f, e])
    }
}
/// n = 9, i = 5, a < h, b < h, c < f, d < e, d < h, e < i, f < g, h < i
fn select_908([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_12([e, c, a, b, f, h, i])
    } else {
        select_907([a, b, d, f, g, e])
    }
}
/// n = 11, i = 6, a < j, a < k, b < i, c < i, c < j, d < g, e < f, e < j, f < i, g < h, h < j, i < k
fn select_909([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_905([b, c, d, a, e, g, h, f, i, j, k])
    } else {
        select_908([b, c, d, f, a, g, h, i, k])
    }
}
/// n = 8, i = 2, a < h, b < f, b < g, c < d, c < e, c < f, c < g, c < h
fn select_910([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_105([a, c, d, e, b, h])
    } else {
        select_222([b, c, d, e, a, f, g])
    }
}
/// n = 8, i = 5, a < h, b < f, c < g, d < g, e < g, g < h
fn select_911([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_782([b, c, d, e, a, f, g, h])
    } else {
        select_560([c, d, e, f, a, g])
    }
}
/// n = 10, i = 5, a < g, b < h, b < j, c < f, c < h, d < g, d < h, e < g, e < h, g < j, h < i
fn select_912([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_911([b, c, a, d, e, f, g, j])
    } else {
        select_562([a, f, b, d, e, h, i])
    }
}
/// n = 13, i = 5, a < g, b < g, b < k, b < l, c < h, c < m, d < f, d < h, e < g, e < h, f < j, g < m, h < i, h < j, h < k, h < l
fn select_913([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_910([f, b, h, a, i, k, l, j])
    } else {
        select_912([b, c, d, a, e, f, g, h, i, m])
    }
}
/// n = 14, i = 6, a < i, a < m, a < n, b < j, b < l, c < i, c < j, d < f, d < j, e < g, e < j, f < h, g < i, i < l, j < k, j < m, j < n
fn select_914([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_909([b, a, c, d, e, g, f, h, i, j, l])
    } else {
        select_913([g, a, b, d, c, f, i, j, k, h, m, n, l])
    }
}
/// n = 10, i = 5, a < f, b < i, b < j, c < g, d < g, e < g, g < h, g < i, g < j
fn select_915([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_562([b, f, c, d, e, g, h])
    } else {
        select_172([h, a, b, i, j, f])
    }
}
/// n = 13, i = 5, a < f, a < g, b < h, b < m, c < h, c < l, d < g, d < j, d < k, e < g, e < h, g < l, g < m, h < i, h < j, h < k
fn select_916([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_742([f, b, c, d, e, g, l, m])
    } else {
        select_915([a, d, b, c, e, f, h, i, j, k])
    }
}
/// n = 13, i = 5, a < g, a < i, b < f, b < i, c < h, c < i, d < e, d < l, d < m, e < f, e < g, e < h, h < j, i < k, i < l, i < m
fn select_917([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < e {
        select_34([k, d, e, l, m])
    } else {
        select_873([a, b, c, f, g, h, j, i])
    }
}
/// n = 14, i = 6, a < j, a < k, b < i, b < k, c < h, c < m, c < n, d < f, d < k, e < h, e < k, f < g, h < i, h < j, k < l, k < m, k < n
fn select_918([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < h {
        select_916([f, a, b, c, e, g, h, k, l, m, n, i, j])
    } else {
        select_917([a, b, d, c, h, i, j, f, k, g, l, m, n])
    }
}
/// n = 14, i = 6, a < i, a < m, a < n, b < j, b < k, c < f, c < k, d < i, d < k, e < g, e < k, g < h, i < j, k < l, k < m, k < n
fn select_919([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < i {
        select_914([a, b, d, e, c, g, f, h, i, k, l, j, m, n])
    } else {
        select_918([b, c, a, e, d, g, h, i, f, j, k, l, m, n])
    }
}
/// n = 15, i = 6, a < f, b < i, b < m, b < n, c < j, c < l, d < i, d < j, e < g, e < j, g < h, i < l, j < k, j < m, j < n, n < o
fn select_920([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < a {
        select_903([g, b, c, h, i, j, k, a, f, m, l, n, o])
    } else {
        select_919([b, c, a, d, e, f, g, h, i, l, j, k, m, n])
    }
}
/// n = 15, i = 6, a < h, b < i, b < m, b < n, c < j, c < l, d < i, d < j, e < f, e < g, e < j, i < l, j < k, j < m, j < n, n < o
fn select_921([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < g {
        select_920([a, b, c, d, e, h, f, g, i, j, k, l, m, n, o])
    } else {
        select_920([a, b, c, d, e, h, g, f, i, j, k, l, m, n, o])
    }
}
/// n = 15, i = 6, a < h, b < m, b < n, c < j, c < k, d < i, d < k, e < f, e < g, e < k, i < j, k < l, k < m, k < n, n < o
fn select_922([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < b {
        select_891([a, c, e, d, f, g, h, i, b, j, k, l, m, n])
    } else {
        select_921([a, b, c, d, e, f, g, h, i, k, l, j, m, n, o])
    }
}
/// n = 15, i = 6, a < i, b < h, b < n, c < k, c < l, d < j, d < l, e < f, e < g, e < l, j < k, l < m, l < n, n < o
fn select_923([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < l {
        select_864([a, c, b, d, e, f, g, i, j, h, k, l, m, n])
    } else {
        select_922([a, b, c, d, e, f, g, i, j, k, l, m, h, n, o])
    }
}
/// n = 15, i = 6, a < k, b < j, b < n, c < i, c < l, d < h, d < l, e < f, e < g, e < l, l < m, l < n, n < o
fn select_924([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_923([a, b, c, d, e, f, g, j, k, h, i, l, m, n, o])
    } else {
        select_923([a, b, d, c, e, f, g, j, k, i, h, l, m, n, o])
    }
}
/// n = 8, i = 5, a < h, b < f, c < g, d < g, e < f, e < h, g < h
fn select_925([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_153([a, b, e, g, f, h])
    } else {
        select_9([a, c, d, f, g])
    }
}
/// n = 8, i = 5, b < h, c < g, d < g, e < f, e < h, g < h
fn select_926([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_925([b, a, c, d, e, f, g, h])
    } else {
        select_12([b, f, c, d, a, g, h])
    }
}
/// n = 9, i = 4, b < e, b < h, b < i, c < f, d < f, f < g, f < h, f < i
fn select_927([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_222([b, f, a, g, e, h, i])
    } else {
        select_141([b, a, c, d, e, f, g])
    }
}
/// n = 10, i = 4, b < j, c < h, c < i, d < f, e < f, f < g, f < h, f < i, f < j
fn select_928([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_308([a, b, d, e, c, f, g, j])
    } else {
        select_927([a, c, d, e, b, f, g, h, i])
    }
}
/// n = 11, i = 5, a < k, b < g, b < j, c < h, d < g, d < h, e < f, e < h, h < i, h < j, h < k
fn select_929([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_926([a, c, b, d, e, f, g, h])
    } else {
        select_928([f, a, b, c, d, h, i, g, j, k])
    }
}
/// n = 10, i = 4, a < f, b < f, b < g, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_930([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_240([a, b, c, d, e, f, g])
    } else {
        select_547([a, d, f, g, e, h, i, j])
    }
}
/// n = 10, i = 4, a < i, b < e, b < j, c < f, c < i, d < f, d < i, f < g, f < h, f < j
fn select_931([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_116([a, f, g, h, i, b, e, j])
    } else {
        select_930([a, c, d, b, e, i, f, g, h, j])
    }
}
/// n = 12, i = 6, a < k, b < h, b < l, c < i, d < h, d < i, e < f, e < k, f < g, f < j, f < l, i < j, i < k, i < l
fn select_932([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_929([a, b, c, d, f, g, h, i, j, l, k])
    } else {
        select_931([a, b, e, i, h, f, g, j, k, l])
    }
}
/// n = 8, i = 5, b < g, c < f, d < e, d < h, f < h, g < h
fn select_933([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_372([a, b, f, d, e, g, h])
    } else {
        select_372([a, c, g, d, e, f, h])
    }
}
/// n = 9, i = 5, b < i, c < h, d < g, e < f, f < h, f < i, g < h, g < i
fn select_934([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_933([a, d, e, b, c, f, g, i])
    } else {
        select_933([a, d, e, c, b, f, g, h])
    }
}
/// n = 10, i = 6, a < j, b < i, c < h, d < g, e < f, e < h, g < i, g < j, h < j
fn select_935([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_934([f, a, b, c, d, g, h, i, j])
    } else {
        select_926([a, c, b, g, e, f, i, h])
    }
}
/// n = 9, i = 5, b < i, c < f, c < h, d < g, e < g, g < h, g < i
fn select_936([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_308([a, c, d, e, f, g, i, h])
    } else {
        select_703([a, f, b, d, e, g, h, i])
    }
}
/// n = 8, i = 4, a < f, b < g, c < e, c < h, d < f, d < g, g < h
fn select_937([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_165([a, c, b, d, e, g, h, f])
    } else {
        select_12([b, c, a, d, e, f, g])
    }
}
/// n = 7, i = 3, a < f, b < e, b < g, c < d, c < g
fn select_938([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_132([a, b, c, f, d, g, e])
    } else {
        select_132([a, c, b, f, e, g, d])
    }
}
/// n = 8, i = 4, b < g, c < e, c < h, d < f, d < g, f < h
fn select_939([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_937([b, a, c, d, e, g, f, h])
    } else {
        select_938([b, c, f, a, e, g, h])
    }
}
/// n = 9, i = 5, b < i, c < h, d < f, d < i, e < g, g < h, g < i
fn select_940([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < g {
        select_936([a, c, d, b, e, f, g, i, h])
    } else {
        select_939([a, c, d, g, f, b, h, i])
    }
}
/// n = 10, i = 6, b < j, c < i, d < h, e < f, e < g, g < j, h < i, h < j
fn select_941([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_935([b, c, a, d, e, f, h, g, i, j])
    } else {
        select_940([f, b, c, g, d, a, h, i, j])
    }
}
/// n = 13, i = 7, a < l, b < j, b < m, c < k, d < g, e < f, e < l, f < h, f < i, f < m, g < j, g < k, k < l, k < m
fn select_942([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if k < h {
        select_932([a, b, c, g, e, f, i, j, k, h, l, m])
    } else {
        select_941([a, c, b, d, f, i, h, g, j, k])
    }
}
/// n = 9, i = 4, a < f, b < e, b < h, b < i, c < f, c < g, d < e, d < g, g < h, g < i
fn select_943([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_30([a, b, c, e, g, h, i, f])
    } else {
        select_157([b, a, c, d, e, f, g])
    }
}
/// n = 11, i = 4, a < e, b < g, b < j, b < k, c < f, c < g, d < f, d < g, f < j, f < k, g < h, g < i
fn select_944([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_311([a, f, b, g, h, i, j, k])
    } else {
        select_737([a, b, c, d, e, f, j, k])
    }
}
/// n = 12, i = 5, a < g, b < h, b < i, b < l, c < f, c < g, d < e, d < i, e < h, e < l, f < i, f < l, i < j, i < k
fn select_945([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_943([a, b, c, e, h, g, f, i, l])
    } else {
        select_944([a, b, d, f, g, e, i, j, k, h, l])
    }
}
/// n = 15, i = 7, a < k, b < i, b < m, c < h, c < l, c < m, d < g, e < f, e < k, f < h, f < m, g < i, g < k, g < m, h < j, i < l, m < n, m < o
fn select_946([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_859([a, b, c, f, d, g, h, j, k, m, n, o, i])
    } else {
        select_945([a, c, e, b, i, f, k, l, m, n, o, h])
    }
}
/// n = 15, i = 7, a < k, b < l, b < m, c < j, c < m, d < g, e < f, e < k, f < h, f < m, g < j, g < k, g < m, h < i, j < l, m < n, m < o
fn select_947([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if b < h {
        select_946([a, c, b, d, e, f, g, h, j, i, k, l, m, n, o])
    } else {
        select_851([a, c, d, f, g, h, i, b, j, k, m])
    }
}
/// n = 15, i = 7, a < k, b < l, b < m, c < j, c < m, d < g, e < f, e < k, f < h, f < i, f < m, g < j, g < k, g < m, j < l, m < n, m < o
fn select_948([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_947([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o])
    } else {
        select_947([a, b, c, d, e, f, g, i, h, j, k, l, m, n, o])
    }
}
/// n = 15, i = 7, a < l, b < k, b < m, c < g, d < f, d < m, e < h, e < l, f < k, g < l, g < m, h < i, h < j, h < m, m < n, m < o
fn select_949([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < g {
        select_942([a, b, c, d, e, h, f, i, j, k, g, l, m])
    } else {
        select_948([a, b, d, c, e, h, g, i, j, f, l, k, m, n, o])
    }
}
/// n = 15, i = 7, a < k, b < l, b < m, c < g, d < f, d < m, e < h, e < k, f < l, g < m, h < i, h < j, h < m, m < n, m < o
fn select_950([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < g {
        select_833([b, c, d, a, h, i, j, f, k, g, l])
    } else {
        select_949([a, b, c, d, e, f, g, h, i, j, l, k, m, n, o])
    }
}
/// n = 16, i = 7, a < k, b < f, b < o, c < l, c < m, d < g, d < m, e < h, e < k, g < l, h < i, h < j, h < m, m < n, m < o, o < p
fn select_951([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if m < f {
        select_922([a, b, c, d, h, i, j, k, g, l, m, n, f, o, p])
    } else {
        select_950([a, c, b, d, e, g, f, h, i, j, k, l, m, n, o])
    }
}
/// n = 16, i = 7, a < l, b < h, b < o, c < g, c < m, d < f, d < m, e < i, e < l, i < j, i < k, i < m, m < n, m < o, o < p
fn select_952([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < g {
        select_951([a, b, c, d, e, h, f, i, j, k, l, g, m, n, o, p])
    } else {
        select_951([a, b, d, c, e, h, g, i, j, k, l, f, m, n, o, p])
    }
}
/// n = 16, i = 7, b < i, b < o, c < h, c < m, d < g, d < m, e < f, e < j, j < k, j < l, j < m, m < n, m < o, o < p
fn select_953([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < a {
        select_924([f, b, c, d, j, k, l, g, h, i, a, m, n, o, p])
    } else {
        select_952([a, b, c, d, e, g, h, i, j, k, l, f, m, n, o, p])
    }
}
/// n = 16, i = 7, b < i, c < h, c < o, d < g, d < m, e < f, e < j, j < k, j < l, j < m, m < n, m < o, o < p
fn select_954([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if m < b {
        select_822([a, f, c, d, j, k, l, g, h, m, n, b, i, o, p])
    } else {
        select_953([a, c, b, d, e, f, g, i, h, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 7, b < i, c < h, c < o, d < g, d < j, e < f, e < k, j < m, j < n, k < l, k < o, o < p
fn select_955([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < j {
        select_597([a, b, c, d, e, f, g, h, i, k, l, o, j, m, n, p])
    } else {
        select_954([a, b, c, e, d, g, f, h, i, j, m, n, k, l, o, p])
    }
}
/// n = 6, i = 2, c < e, c < f, d < e, d < f
fn select_956([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_47([a, b, c, d, e, f])
    } else {
        select_47([a, b, d, c, e, f])
    }
}
/// n = 8, i = 3, b < e, c < f, c < g, d < f, d < g, g < h
fn select_957([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_956([a, e, c, d, f, g])
    } else {
        select_434([a, b, d, f, e, g, h])
    }
}
/// n = 8, i = 3, d < f, d < g, e < f, e < g, g < h
fn select_958([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_957([c, a, d, e, b, f, g, h])
    } else {
        select_957([c, b, d, e, a, f, g, h])
    }
}
/// n = 9, i = 4, a < g, b < e, c < d, c < g, c < h, e < f, e < h, h < i
fn select_959([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_479([b, c, d, g, e, f, h])
    } else {
        select_59([f, a, c, d, h, i, g])
    }
}
/// n = 10, i = 4, b < f, c < d, c < e, c < h, e < j, f < g, f < h, f < j, h < i
fn select_960([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_107([d, e, b, a, f, g, h, i, j])
    } else {
        select_959([a, b, c, d, f, g, e, h, i])
    }
}
/// n = 10, i = 4, b < g, b < i, c < d, c < e, c < f, e < j, f < i, g < h, g < j
fn select_961([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_958([a, d, e, b, f, i, g, h])
    } else {
        select_960([a, b, c, d, e, g, h, f, i, j])
    }
}
/// n = 9, i = 4, b < f, c < d, c < e, d < i, e < h, f < g, f < h, f < i
fn select_962([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_708([a, c, f, g, e, h, d])
    } else {
        select_164([a, e, d, b, f, g, i, h])
    }
}
/// n = 9, i = 3, b < e, b < h, b < i, c < d, c < f, c < g, d < h, d < i, e < f, e < g
fn select_963([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_243([d, b, a, e, f, g, h, i])
    } else {
        select_255([a, c, b, e, d, f, g])
    }
}
/// n = 10, i = 4, b < f, c < d, c < e, c < h, e < i, e < j, f < g, f < i, f < j, g < h
fn select_964([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < g {
        select_164([a, d, e, b, f, i, j, g])
    } else {
        select_963([a, c, f, g, e, i, j, d, h])
    }
}
/// n = 11, i = 5, b < e, c < d, d < f, d < h, d < j, e < g, e < i, e < k, h < k, i < j
fn select_965([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < i {
        select_962([a, b, d, f, h, e, g, k, i])
    } else {
        select_964([a, c, e, g, i, d, h, k, f, j])
    }
}
/// n = 6, i = 3, b < d, c < e, e < f
fn select_966([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < f {
        select_124([a, b, c, d, e, f])
    } else {
        select_25([b, e, d, f, a])
    }
}
/// n = 7, i = 3, c < g, d < e, e < f, e < g
fn select_967([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_966([b, a, d, c, e, f])
    } else {
        select_106([b, c, d, a, e, f, g])
    }
}
/// n = 10, i = 5, b < d, c < e, d < h, d < j, e < f, e < g, e < j, h < i
fn select_968([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_50([a, c, d, h, e, g, i, j, f])
    } else {
        select_967([a, g, f, b, d, j, h])
    }
}
/// n = 11, i = 5, b < f, c < d, c < e, d < j, e < g, e < k, f < h, f < i, f < j, i < k
fn select_969([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < d {
        select_965([a, b, c, e, f, g, h, d, i, k, j])
    } else {
        select_968([a, c, b, d, f, h, i, e, g, j])
    }
}
/// n = 11, i = 5, b < d, b < e, c < f, e < i, e < k, f < g, f < h, f < j, j < k
fn select_970([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_961([a, b, f, h, j, g, e, i, d, k])
    } else {
        select_969([a, c, b, d, e, f, i, h, j, g, k])
    }
}
/// n = 9, i = 3, a < e, b < f, b < g, c < d, c < g, c < i, f < i, g < h
fn select_971([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_109([a, b, c, e, d, g, h, f])
    } else {
        select_643([a, c, b, e, f, g, h, d, i])
    }
}
/// n = 9, i = 3, c < f, c < g, d < e, d < g, d < i, f < i, g < h
fn select_972([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_971([a, c, d, e, b, f, g, h, i])
    } else {
        select_971([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 8, i = 4, a < f, b < f, c < d, c < e, c < h, f < g, f < h
fn select_973([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_35([c, f, g, e, d, h])
    } else {
        select_9([e, a, b, d, f])
    }
}
/// n = 8, i = 3, a < d, b < f, b < g, b < h, c < e, e < f, e < g, e < h
fn select_974([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_798([b, d, c, e])
    } else {
        select_67([a, b, f, g, h, d])
    }
}
/// n = 8, i = 3, c < f, c < g, c < h, d < e, e < f, e < g, e < h
fn select_975([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_974([a, c, d, b, e, f, g, h])
    } else {
        select_974([b, c, d, a, e, f, g, h])
    }
}
/// n = 10, i = 4, a < f, a < i, a < j, b < f, b < g, c < d, c < e, c < g, f < h, g < i, g < j
fn select_976([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_973([a, b, c, d, e, f, h, g])
    } else {
        select_975([d, e, a, b, g, f, i, j])
    }
}
/// n = 7, i = 3, b < g, c < f, c < g, d < e, d < g, e < f
fn select_977([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_302([b, c, a, d, e, g])
    } else {
        select_22([b, c, e, a, f, g])
    }
}
/// n = 10, i = 5, a < h, a < j, b < e, c < d, c < h, d < j, e < f, e < g, e < j, h < i
fn select_978([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_49([d, b, a, h, e, g, i, f, j])
    } else {
        select_977([g, f, a, c, d, j, h])
    }
}
/// n = 11, i = 5, a < h, a < j, b < d, b < h, b < k, c < e, e < f, e < g, e < j, h < i, j < k
fn select_979([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if j < d {
        select_976([b, a, e, f, g, h, j, i, d, k])
    } else {
        select_978([a, c, b, d, e, f, g, h, i, j])
    }
}
/// n = 12, i = 5, a < h, b < d, b < h, b < k, c < e, e < f, e < g, e < i, h < j, i < k, k < l
fn select_980([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < a {
        select_972([f, g, i, b, d, a, k, l, h])
    } else {
        select_979([a, b, c, d, e, f, g, h, j, i, k])
    }
}
/// n = 7, i = 2, a < g, b < f, c < d, c < f, c < g, d < e
fn select_981([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_42([a, c, b, d, e, g])
    } else {
        select_42([b, c, a, d, e, f])
    }
}
/// n = 8, i = 3, a < d, a < f, a < h, b < c, b < e, b < g, e < h, f < g
fn select_982([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_981([c, e, a, f, g, h, d])
    } else {
        select_981([d, f, b, e, h, g, c])
    }
}
/// n = 8, i = 3, a < e, a < g, b < c, b < d, b < g, e < f, g < h
fn select_983([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_58([d, c, a, g, h, e])
    } else {
        select_45([e, b, d, f, g, h, c])
    }
}
/// n = 9, i = 3, a < g, b < e, b < h, c < d, c < f, c < g, f < h, g < i
fn select_984([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_982([b, c, d, e, f, a, g, h])
    } else {
        select_983([a, c, d, f, b, e, g, i])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, c < d, c < f, d < h, f < g
fn select_985([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_46([a, c, e, f, d, g])
    } else {
        select_457([a, b, c, d, e, f, h])
    }
}
/// n = 9, i = 4, a < d, a < g, b < g, b < h, c < e, e < f, e < h, h < i
fn select_986([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_985([a, b, e, f, g, h, i, d])
    } else {
        select_665([b, a, c, d, e, g, h, i, f])
    }
}
/// n = 10, i = 4, a < h, b < d, b < e, c < f, e < i, f < g, f < h, f < i, h < j
fn select_987([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_106([d, e, c, a, f, g, i])
    } else {
        select_986([b, a, c, d, f, g, e, h, j])
    }
}
/// n = 10, i = 4, a < i, b < d, b < e, c < f, f < g, f < h, f < i, i < j
fn select_988([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_984([a, b, f, h, e, g, i, d, j])
    } else {
        select_987([a, b, c, e, d, f, h, i, g, j])
    }
}
/// n = 8, i = 3, b < h, c < e, c < f, c < h, d < e, d < f, d < h, f < g
fn select_989([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_491([a, b, c, d, e, f, h])
    } else {
        select_751([a, b, c, d, f, e, g, h])
    }
}
/// n = 10, i = 3, c < e, c < f, c < g, d < e, d < f, d < g, f < j, g < h, g < i
fn select_990([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_892([b, e, f, g, a, h, i, j])
    } else {
        select_989([b, a, c, d, e, f, j, g])
    }
}
/// n = 11, i = 4, a < e, a < i, a < j, b < c, b < d, b < f, e < g, e < h, f < i, f < j, j < k
fn select_991([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_327([b, e, g, h, c, d, f, i, j])
    } else {
        select_990([c, d, a, f, i, j, e, g, h, k])
    }
}
/// n = 12, i = 5, a < e, b < c, b < d, b < k, d < f, d < g, e < h, e < i, e < j, j < k, k < l
fn select_992([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < j {
        select_988([c, d, a, f, g, e, h, i, j, k])
    } else {
        select_991([b, e, h, i, d, j, f, g, c, k, l])
    }
}
/// n = 12, i = 5, b < d, b < e, b < k, c < f, e < i, f < g, f < h, f < j, j < k, k < l
fn select_993([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < e {
        select_980([a, b, c, d, f, g, h, e, j, i, k, l])
    } else {
        select_992([c, b, d, e, f, a, i, g, h, j, k, l])
    }
}
/// n = 12, i = 5, b < f, c < e, c < k, d < g, g < h, g < i, g < j, j < k, k < l
fn select_994([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < c {
        select_970([a, b, d, f, c, g, h, i, e, j, k])
    } else {
        select_993([a, c, d, e, b, g, h, i, f, j, k, l])
    }
}
/// n = 12, i = 5, d < f, d < k, e < g, g < h, g < i, g < j, j < k, k < l
fn select_995([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_994([c, a, d, e, f, b, g, h, i, j, k, l])
    } else {
        select_994([c, b, d, e, f, a, g, h, i, j, k, l])
    }
}
/// n = 9, i = 3, b < d, c < e, e < f, e < g, g < h, g < i
fn select_996([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_339([a, f, b, g, h, i, d])
    } else {
        select_47([a, d, c, e, f, g])
    }
}
/// n = 9, i = 3, d < e, e < f, e < g, g < h, g < i
fn select_997([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_996([c, a, d, b, e, f, g, h, i])
    } else {
        select_996([c, b, d, a, e, f, g, h, i])
    }
}
/// n = 10, i = 3, d < e, e < f, e < g, e < h, g < j, h < i
fn select_998([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_997([a, b, c, d, e, f, g, h, j])
    } else {
        select_997([a, b, c, d, e, f, h, g, i])
    }
}
/// n = 10, i = 3, d < e, d < f, e < h, f < g, f < i, i < j
fn select_999([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_997([a, b, c, d, e, h, f, g, i])
    } else {
        select_998([a, b, c, d, f, g, e, i, j, h])
    }
}
/// n = 10, i = 3, d < e, d < f, d < g, g < h, g < i, i < j
fn select_1000([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_999([a, b, c, d, e, g, h, f, i, j])
    } else {
        select_999([a, b, c, d, f, g, h, e, i, j])
    }
}
/// n = 9, i = 2, a < i, b < f, c < d, c < e, c < g, c < i, g < h
fn select_1001([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_745([a, b, c, f, d, g, h, e, i])
    } else {
        select_745([a, b, c, f, e, g, h, d, i])
    }
}
/// n = 9, i = 2, c < i, d < e, d < f, d < g, d < i, g < h
fn select_1002([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1001([c, a, d, e, f, b, g, h, i])
    } else {
        select_1001([c, b, d, e, f, a, g, h, i])
    }
}
/// n = 9, i = 2, c < i, d < e, d < f, d < g, d < h, d < i
fn select_1003([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_1002([a, b, c, d, g, h, e, f, i])
    } else {
        select_1002([a, b, c, d, g, h, f, e, i])
    }
}
/// n = 11, i = 4, a < e, b < g, b < k, c < f, c < g, d < f, d < g, f < k, g < h, g < i, g < j
fn select_1004([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_12([b, a, c, d, e, f, k])
    } else {
        select_71([a, f, b, g, h, i, j, k])
    }
}
/// n = 11, i = 4, c < g, c < k, d < f, d < g, e < f, e < g, f < k, g < h, g < i, g < j
fn select_1005([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_1004([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_1004([b, c, d, e, a, f, g, h, i, j, k])
    }
}
/// n = 14, i = 6, a < i, a < k, b < h, b < k, c < h, c < k, d < e, d < f, f < g, f < k, h < i, i < j, k < l, k < m, k < n
fn select_1006([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < f {
        select_98([a, d, e, i, f, g, j, k])
    } else {
        select_1005([e, g, a, b, c, h, k, l, m, n, i])
    }
}
/// n = 15, i = 6, a < i, b < h, b < j, c < h, c < j, d < e, d < f, f < g, f < j, h < i, h < o, i < n, j < k, j < l, j < m, j < o
fn select_1007([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < a {
        select_1003([e, g, h, j, a, k, l, m, o])
    } else {
        select_1006([a, b, c, d, e, f, g, h, i, n, j, k, l, m])
    }
}
/// n = 7, i = 3, a < g, b < g, c < f, d < e, d < g, e < f
fn select_1008([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_159([b, a, d, e, c, g])
    } else {
        select_53([b, d, c, a, e, f])
    }
}
/// n = 8, i = 3, a < h, b < e, c < d, c < g, d < h, e < f, e < g
fn select_1009([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_105([c, e, a, f, d, g])
    } else {
        select_642([b, c, a, d, e, f, h])
    }
}
/// n = 9, i = 4, a < i, b < g, c < e, c < f, d < f, d < g, f < i, g < h
fn select_1010([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_1008([b, e, a, d, f, i, g])
    } else {
        select_1009([a, b, c, f, g, h, e, i])
    }
}
/// n = 8, i = 4, b < h, c < g, c < h, d < f, d < h, e < f, e < h, f < g
fn select_1011([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_391([b, c, a, d, e, f, h])
    } else {
        select_22([b, c, f, a, g, h])
    }
}
/// n = 10, i = 4, b < j, c < g, d < f, d < g, e < f, e < g, f < j, g < h, g < i
fn select_1012([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_811([a, f, c, g, b, h, i])
    } else {
        select_1011([a, c, b, d, e, f, j, g])
    }
}
/// n = 12, i = 5, a < h, b < j, c < e, c < g, d < f, d < j, f < h, g < j, h < i, j < k, j < l
fn select_1013([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_1010([b, a, c, f, e, g, h, i, j])
    } else {
        select_1012([e, a, b, d, g, f, j, k, l, h])
    }
}
/// n = 11, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < k, g < k, h < i, h < j
fn select_1014([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_355([a, b, c, d, e, f, g, k])
    } else {
        select_48([a, g, f, h, i, j, k])
    }
}
/// n = 11, i = 5, c < g, c < h, d < g, d < h, e < f, f < h, f < k, g < k, h < i, h < j
fn select_1015([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_1014([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_1014([b, c, d, e, a, f, g, h, i, j, k])
    }
}
/// n = 7, i = 4, a < f, a < g, b < f, b < g, c < e, c < g, d < e, d < f
fn select_1016([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_162([a, b, d, e, g])
    } else {
        select_13([c, d, f, g])
    }
}
/// n = 8, i = 5, a < g, a < h, b < g, b < h, c < f, d < e, d < g, f < h
fn select_1017([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_1016([a, b, d, f, e, h, g])
    } else {
        select_156([a, b, c, e, f, g])
    }
}
/// n = 9, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < i, g < i
fn select_1018([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_474([d, g, a, f, h, e, i])
    } else {
        select_1017([b, c, d, a, e, f, g, h])
    }
}
/// n = 9, i = 5, c < g, c < h, d < g, d < h, e < f, f < h, f < i, g < i
fn select_1019([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1018([a, c, d, e, b, f, g, h, i])
    } else {
        select_1018([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 12, i = 6, c < h, d < g, d < i, e < f, e < j, f < i, g < j, h < i, h < j, j < k, j < l
fn select_1020([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_1015([a, b, e, g, c, h, f, j, k, l, i])
    } else {
        select_1019([a, b, d, f, c, h, g, i, j])
    }
}
/// n = 10, i = 4, a < e, b < g, b < j, c < f, c < g, d < f, d < g, f < j, g < h, g < i
fn select_1021([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_48([a, f, b, g, h, i, j])
    } else {
        select_12([b, a, c, d, e, f, j])
    }
}
/// n = 10, i = 4, c < g, c < j, d < f, d < g, e < f, e < g, f < j, g < h, g < i
fn select_1022([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1021([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_1021([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 10, i = 4, c < h, d < g, d < h, e < f, e < h, f < g, h < i, h < j
fn select_1023([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < c {
        select_279([a, b, d, f, c, g, h])
    } else {
        select_1022([a, b, d, c, e, f, h, i, j, g])
    }
}
/// n = 12, i = 6, c < f, c < j, d < h, e < g, e < i, g < j, h < i, h < j, j < k, j < l
fn select_1024([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_1020([a, b, d, e, c, f, g, h, i, j, k, l])
    } else {
        select_1023([a, b, g, c, h, i, f, j, k, l])
    }
}
/// n = 15, i = 7, a < k, b < e, b < g, c < h, d < f, d < j, f < m, g < i, g < m, h < j, h < m, j < k, k < l, m < n, m < o
fn select_1025([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < i {
        select_1013([a, f, b, h, e, j, g, k, l, m, n, o])
    } else {
        select_1024([a, e, g, c, d, i, f, h, j, m, n, o])
    }
}
/// n = 16, i = 7, a < k, b < f, b < g, c < h, d < e, d < j, d < l, g < i, g < l, h < j, h < l, j < k, j < p, k < o, l < m, l < n, l < p
fn select_1026([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < e {
        select_1007([a, d, h, b, f, g, i, j, k, l, e, m, n, o, p])
    } else {
        select_1025([a, b, c, d, f, e, g, h, i, j, k, o, l, m, n])
    }
}
/// n = 16, i = 7, a < n, b < f, b < j, c < e, c < g, d < h, g < i, g < k, h < j, h < k, j < n, j < p, k < l, k < m, k < p, n < o
fn select_1027([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < b {
        select_1000([a, e, i, k, l, m, b, f, j, p])
    } else {
        select_1026([a, c, d, b, f, e, g, h, i, j, n, k, l, m, o, p])
    }
}
/// n = 16, i = 7, a < n, b < f, b < k, c < e, c < h, d < g, d < j, g < k, h < i, h < j, j < l, j < m, j < p, k < n, k < p, n < o
fn select_1028([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < g {
        select_995([a, e, i, b, d, f, j, l, m, g, k, p])
    } else {
        select_1027([a, b, c, d, e, f, h, g, i, k, j, l, m, n, o, p])
    }
}
/// n = 8, i = 2, a < e, a < h, b < c, b < d, b < h, d < f, e < g
fn select_1029([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_19([c, d, a, f])
    } else {
        select_29([b, a, c, e, g, d, h])
    }
}
/// n = 9, i = 2, a < i, b < f, c < d, c < e, c < i, e < g, f < h
fn select_1030([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1029([a, c, d, e, b, g, f, i])
    } else {
        select_251([b, c, d, a, e, f, g, h])
    }
}
/// n = 9, i = 2, a < f, b < c, b < d, b < e, d < h, e < g, f < i
fn select_1031([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_172([c, e, d, a, h, g])
    } else {
        select_1029([a, b, c, e, f, g, i, d])
    }
}
/// n = 9, i = 2, b < g, c < d, c < e, c < f, f < h, g < i
fn select_1032([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < d {
        select_1030([a, b, c, e, f, g, h, i, d])
    } else {
        select_1031([b, c, e, d, f, g, h, a, i])
    }
}
/// n = 8, i = 1, d < h, e < f, e < g
fn select_1033([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_677([a, b, c, d, e, h])
    } else {
        select_174([a, b, c, e, d, f, g])
    }
}
/// n = 10, i = 3, b < d, b < h, c < e, c < h, e < f, e < g, e < i, i < j
fn select_1034([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_1033([a, f, g, i, b, d, h, j])
    } else {
        select_221([a, d, c, h, e, f, g, i])
    }
}
/// n = 10, i = 3, b < e, c < d, c < f, f < g, f < h, f < i, i < j
fn select_1035([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < b {
        select_1032([a, d, f, g, h, i, b, j, e])
    } else {
        select_1034([a, b, c, e, f, g, h, d, i, j])
    }
}
/// n = 10, i = 3, d < e, d < f, f < g, f < h, f < i, i < j
fn select_1036([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1035([c, a, d, e, b, f, g, h, i, j])
    } else {
        select_1035([c, b, d, e, a, f, g, h, i, j])
    }
}
/// n = 7, i = 3, a < f, b < d, b < g, c < e, c < f, e < g
fn select_1037([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_25([d, c, a, e, g])
    } else {
        select_143([b, a, c, e, f, d, g])
    }
}
/// n = 7, i = 3, b < e, b < g, c < d, c < f, f < g
fn select_1038([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < d {
        select_1037([a, b, c, e, f, d, g])
    } else {
        select_23([f, d, b, e, a, g])
    }
}
/// n = 10, i = 4, b < f, b < j, c < f, c < g, c < j, d < e, e < g, e < j, g < h, g < i
fn select_1039([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_775([a, b, c, d, e, f, j])
    } else {
        select_751([a, b, c, e, g, h, i, j])
    }
}
/// n = 12, i = 5, b < e, b < j, c < g, c < h, c < j, d < f, e < g, f < h, f < j, h < i, j < k, j < l
fn select_1040([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_766([a, h, b, i, j, k, l, e])
    } else {
        select_1039([a, e, c, d, f, g, j, k, l, h])
    }
}
/// n = 12, i = 5, b < i, b < j, c < e, c < j, d < f, e < i, f < g, f < j, g < h, j < k, j < l
fn select_1041([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_1038([a, c, g, h, e, b, j])
    } else {
        select_1040([a, c, b, d, e, f, i, g, h, j, k, l])
    }
}
/// n = 12, i = 5, b < f, b < j, c < e, c < j, d < g, g < h, g < j, h < i, j < k, j < l
fn select_1042([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_1041([a, b, c, d, e, g, h, i, f, j, k, l])
    } else {
        select_1041([a, c, b, d, f, g, h, i, e, j, k, l])
    }
}
/// n = 12, i = 5, b < f, b < j, c < e, c < j, d < g, g < h, g < i, g < j, j < k, j < l
fn select_1043([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < i {
        select_1042([a, b, c, d, e, f, g, h, i, j, k, l])
    } else {
        select_1042([a, b, c, d, e, f, g, i, h, j, k, l])
    }
}
/// n = 12, i = 5, b < f, c < e, c < j, d < g, g < h, g < i, g < j, j < k, j < l
fn select_1044([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < b {
        select_1036([a, h, i, c, e, j, k, l, b, f])
    } else {
        select_1043([a, b, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 8, i = 2, a < g, b < f, b < h, c < d, c < e, c < h
fn select_1045([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_422([a, b, f, g, c, d, e, h])
    } else {
        select_116([a, c, d, e, g, b, f, h])
    }
}
/// n = 10, i = 2, a < g, b < f, c < d, c < e, c < h, h < i, h < j
fn select_1046([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < h {
        select_1045([a, b, c, d, e, f, g, h])
    } else {
        select_654([a, c, d, e, g, h, b, i, j])
    }
}
/// n = 10, i = 2, c < g, d < e, d < f, d < h, h < i, h < j
fn select_1047([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1046([a, c, d, e, f, g, b, h, i, j])
    } else {
        select_1046([b, c, d, e, f, g, a, h, i, j])
    }
}
/// n = 8, i = 3, a < d, a < g, b < c, b < g, b < h, c < f, d < e, d < h
fn select_1048([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_4([a, c, f, g, d])
    } else {
        select_35([b, d, e, g, c, h])
    }
}
/// n = 10, i = 3, a < c, a < g, b < d, b < g, c < f, d < e, d < h, h < i, h < j
fn select_1049([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_34([e, h, a, i, j])
    } else {
        select_1048([b, a, c, d, e, f, g, h])
    }
}
/// n = 11, i = 3, a < c, a < d, a < j, b < e, b < k, d < k, e < f, e < g, e < j, g < h, g < i
fn select_1050([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_36([a, e, f, c, g, h, i, j])
    } else {
        select_598([b, a, c, d, e, f, j, k])
    }
}
/// n = 11, i = 3, a < g, b < d, b < k, c < e, c < g, e < f, e < h, e < k, h < i, h < j
fn select_1051([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_1049([a, c, b, e, f, d, g, h, i, j])
    } else {
        select_1050([b, c, d, a, e, f, h, i, j, k, g])
    }
}
/// n = 11, i = 3, a < h, b < d, c < e, c < h, e < f, e < g, e < i, i < j, i < k
fn select_1052([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_396([a, e, g, h, f, i, j, k, b])
    } else {
        select_1051([a, b, c, d, e, g, h, i, j, k, f])
    }
}
/// n = 11, i = 3, c < h, d < e, d < h, e < f, e < g, e < i, i < j, i < k
fn select_1053([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_1052([c, a, d, b, e, f, g, h, i, j, k])
    } else {
        select_1052([c, b, d, a, e, f, g, h, i, j, k])
    }
}
/// n = 11, i = 3, d < e, d < f, f < g, f < h, f < i, i < j, i < k
fn select_1054([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_1047([b, c, e, f, g, h, a, i, j, k])
    } else {
        select_1053([b, c, a, d, f, g, h, e, i, j, k])
    }
}
/// n = 10, i = 3, a < d, b < f, b < j, c < e, c < f, e < j, f < g, f < h, f < i
fn select_1055([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_68([e, f, a, g, h, i, j])
    } else {
        select_108([b, a, c, d, e, j, f])
    }
}
/// n = 10, i = 3, c < f, c < j, d < e, d < f, e < j, f < g, f < h, f < i
fn select_1056([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1055([a, c, d, b, e, f, g, h, i, j])
    } else {
        select_1055([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 10, i = 3, c < f, d < g, e < f, e < g, g < h, g < i, g < j
fn select_1057([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_279([a, b, e, c, d, f, g])
    } else {
        select_1056([a, b, e, d, c, g, h, i, j, f])
    }
}
/// n = 9, i = 3, a < d, a < i, b < e, b < i, c < e, c < i, e < f, e < g, e < h
fn select_1058([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_87([d, b, c, i])
    } else {
        select_67([a, e, f, g, h, i])
    }
}
/// n = 12, i = 4, a < f, a < h, b < f, b < h, c < d, c < h, c < l, d < e, f < g, f < l, h < i, h < j, h < k
fn select_1059([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < d {
        select_300([f, c, g, h, i, j, k, d, l])
    } else {
        select_1058([d, a, b, e, h, i, j, k, f])
    }
}
/// n = 12, i = 4, a < h, a < i, b < e, b < i, c < d, c < g, c < i, d < f, e < g, e < h, i < j, i < k, i < l
fn select_1060([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_1048([e, c, d, a, h, f, g, i])
    } else {
        select_1059([a, b, c, d, f, e, h, i, j, k, l, g])
    }
}
/// n = 10, i = 3, a < d, a < e, b < e, b < f, c < d, c < f, f < g, f < h, f < i, f < j
fn select_1061([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_189([a, g, h, i, j])
    } else {
        select_4([b, c, d, e, f])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, c < d, c < i, d < f, d < h, e < g, e < h, e < i
fn select_1062([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_88([a, f, b, e, h])
    } else {
        select_474([b, c, d, e, g, f, i])
    }
}
/// n = 10, i = 5, a < h, a < j, b < g, b < j, c < f, d < e, d < j, e < h, e < i, f < g, f < i, f < j
fn select_1063([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_1062([b, c, d, e, f, h, g, i, j])
    } else {
        select_186([a, b, d, g, e, i, h, j])
    }
}
/// n = 13, i = 5, a < h, b < g, b < j, c < f, d < e, d < j, e < g, e < i, f < h, f < i, f < j, j < k, j < l, j < m
fn select_1064([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < a {
        select_1061([e, b, f, i, g, j, a, k, l, m])
    } else {
        select_1063([b, a, c, d, e, f, h, g, i, j])
    }
}
/// n = 13, i = 5, b < h, b < j, c < f, d < e, d < j, e < h, e < i, f < g, f < i, f < j, j < k, j < l, j < m
fn select_1065([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_1060([b, d, f, g, e, a, i, h, j, k, l, m])
    } else {
        select_1064([a, b, c, d, e, f, h, g, i, j, k, l, m])
    }
}
/// n = 13, i = 5, b < i, b < j, c < e, c < j, d < f, e < i, f < g, f < h, f < j, j < k, j < l, j < m
fn select_1066([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_1057([a, h, g, b, c, e, j, k, l, m])
    } else {
        select_1065([a, b, d, c, e, f, h, i, g, j, k, l, m])
    }
}
/// n = 13, i = 5, b < f, b < j, c < e, c < j, d < g, g < h, g < i, g < j, j < k, j < l, j < m
fn select_1067([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_1066([a, b, c, d, e, g, h, i, f, j, k, l, m])
    } else {
        select_1066([a, c, b, d, f, g, h, i, e, j, k, l, m])
    }
}
/// n = 13, i = 5, b < f, b < m, c < e, c < j, d < g, g < h, g < i, g < j, j < k, j < l, j < m
fn select_1068([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < b {
        select_1054([a, h, i, c, e, j, k, l, b, f, m])
    } else {
        select_1067([a, b, c, d, e, f, g, h, i, j, k, l, m])
    }
}
/// n = 13, i = 5, c < m, d < f, d < j, e < g, g < h, g < i, g < j, j < k, j < l, j < m
fn select_1069([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < c {
        select_1044([b, a, d, e, f, c, g, h, i, j, k, l])
    } else {
        select_1068([b, c, d, e, f, a, g, h, i, j, k, l, m])
    }
}
/// n = 8, i = 1, f < g, f < h
fn select_1070([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_1033([c, d, e, a, f, g, h, b])
    } else {
        select_1033([c, d, e, b, f, g, h, a])
    }
}
/// n = 7, i = 3, a < g, b < f, b < g, c < e, c < f, d < e, d < f, d < g
fn select_1071([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_22([a, b, c, e, f, g])
    } else {
        select_11([b, c, d, e, g])
    }
}
/// n = 8, i = 3, b < e, b < h, c < f, c < h, d < e, d < f, d < h, f < g
fn select_1072([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_44([b, c, e, f, a, g, h])
    } else {
        select_1071([a, c, b, d, e, h, f])
    }
}
/// n = 11, i = 3, b < f, c < e, c < g, d < e, d < f, d < g, f < k, g < h, g < i, g < j
fn select_1073([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_398([a, e, g, b, h, i, j])
    } else {
        select_1072([a, c, b, d, e, f, k, g])
    }
}
/// n = 12, i = 3, b < f, c < e, c < g, d < e, d < f, d < g, f < l, g < h, g < i, g < j, g < k
fn select_1074([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < b {
        select_398([a, e, g, h, i, j, k])
    } else {
        select_1073([a, b, c, d, e, f, g, i, j, k, l])
    }
}
/// n = 13, i = 4, b < e, b < k, c < f, c < k, d < f, d < k, d < l, e < l, f < g, f < h, f < i, f < j, l < m
fn select_1075([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < b {
        select_1070([a, g, h, i, j, b, e, k])
    } else {
        select_1074([a, e, c, d, k, l, f, g, h, i, j, m])
    }
}
/// n = 6, i = 3, c < f, d < e, d < f
fn select_1076([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_16([c, a, d, e, b, f])
    } else {
        select_16([c, b, d, e, a, f])
    }
}
/// n = 10, i = 3, a < d, b < e, c < e, c < j, d < j, e < f, e < g, e < h, e < i
fn select_1077([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_189([a, f, g, h, i])
    } else {
        select_11([b, d, c, j, e])
    }
}
/// n = 10, i = 3, a < d, a < e, b < f, c < e, c < f, f < g, f < h, f < i, f < j
fn select_1078([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_189([a, g, h, i, j])
    } else {
        select_82([d, b, c, e, f])
    }
}
/// n = 10, i = 3, b < e, c < f, d < e, d < f, f < g, f < h, f < i, f < j
fn select_1079([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1077([a, c, d, b, f, g, h, i, j, e])
    } else {
        select_1078([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 11, i = 3, b < e, b < f, c < g, d < f, d < g, g < h, g < i, g < j, g < k
fn select_1080([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_1076([a, c, d, b, e, f])
    } else {
        select_1079([a, b, c, d, f, g, h, i, j, k])
    }
}
/// n = 7, i = 1, b < c, b < d, b < e, b < f, b < g
fn select_1081([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < a {
        select_189([c, d, e, f, g])
    } else {
        select_576([a, b, d, e, f, g])
    }
}
/// n = 8, i = 3, b < g, b < h, c < e, c < g, c < h, d < e, d < g, d < h, e < f
fn select_1082([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_87([b, c, d, e])
    } else {
        select_659([a, b, d, e, f, g, h])
    }
}
/// n = 12, i = 3, b < e, b < l, c < f, c < l, d < e, d < f, d < l, e < k, f < g, f < h, f < i, f < j
fn select_1083([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_1081([a, f, b, g, h, i, j])
    } else {
        select_1082([a, c, b, d, e, k, f, l])
    }
}
/// n = 13, i = 4, b < h, b < m, c < f, c < h, d < e, d < h, d < m, e < g, f < m, h < i, h < j, h < k, h < l
fn select_1084([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_1080([a, e, b, c, g, f, h, i, j, k, l])
    } else {
        select_1083([a, f, b, d, e, h, i, j, k, l, g, m])
    }
}
/// n = 14, i = 5, b < f, b < g, c < i, c < n, d < g, d < i, e < f, e < i, e < n, f < h, g < n, i < j, i < k, i < l, i < m
fn select_1085([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < b {
        select_1070([a, j, k, l, m, b, f, g])
    } else {
        select_1084([a, c, d, e, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 5, b < g, c < i, c < n, d < f, d < i, e < g, e < i, e < n, f < n, g < h, i < j, i < k, i < l, i < m
fn select_1086([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < b {
        select_1075([a, f, c, e, b, i, j, k, l, m, n, g, h])
    } else {
        select_1085([a, b, c, d, e, g, f, h, i, j, k, l, m, n])
    }
}
/// n = 8, i = 3, a < d, a < h, b < f, b < g, b < h, c < e, c < f, c < h, e < g
fn select_1087([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_5([a, e, b, f, g, h])
    } else {
        select_108([b, a, c, d, e, g, h])
    }
}
/// n = 9, i = 3, a < g, b < f, b < i, c < e, c < i, d < f, d < g, d < i, g < h
fn select_1088([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_1087([c, d, b, e, a, f, g, i])
    } else {
        select_109([c, d, a, e, b, g, h, f])
    }
}
/// n = 8, i = 4, a < g, b < e, b < f, c < f, c < g, d < e, d < g, g < h
fn select_1089([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_642([a, b, c, e, g, h, f])
    } else {
        select_642([a, b, d, f, g, h, e])
    }
}
/// n = 9, i = 4, a < f, b < g, b < i, c < f, c < g, c < i, d < e, d < g, e < i, g < h
fn select_1090([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_80([b, e, c, a, g, h, f, i])
    } else {
        select_1089([b, a, c, d, e, f, g, h])
    }
}
/// n = 8, i = 4, a < f, b < g, b < h, c < f, c < g, c < h, d < e, d < g, e < h
fn select_1091([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_291([a, c, d, e, g, f, h])
    } else {
        select_159([b, c, d, e, f, g])
    }
}
/// n = 9, i = 4, a < g, b < f, c < h, d < g, d < h, e < f, e < g, e < h, h < i
fn select_1092([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < a {
        select_1090([b, d, e, c, a, f, h, i, g])
    } else {
        select_1091([b, d, e, a, c, f, g, h])
    }
}
/// n = 10, i = 4, b < h, c < g, c < j, d < g, d < h, d < j, e < f, e < g, e < h, f < j, h < i
fn select_1093([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_1088([b, c, f, d, a, g, h, i, j])
    } else {
        select_1092([c, a, b, d, e, f, g, h, i])
    }
}
/// n = 8, i = 5, a < h, b < g, c < g, c < h, d < g, d < h, e < f, f < h
fn select_1094([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_523([b, c, d, a, e, f, g, h])
    } else {
        select_240([b, c, d, f, a, g, h])
    }
}
/// n = 9, i = 5, b < h, b < i, c < g, c < i, d < g, d < i, e < f, e < h, f < i, g < h
fn select_1095([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_108([b, g, e, a, f, i, h])
    } else {
        select_1094([b, a, c, d, e, f, g, i])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, b < g, c < d, c < f, c < g
fn select_1096([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_154([d, a, b, f, g, e])
    } else {
        select_154([e, a, c, f, g, d])
    }
}
/// n = 7, i = 5, b < g, c < g, d < g, e < f
fn select_1097([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < f {
        select_637([a, c, d, e, f, g])
    } else {
        select_9([a, b, c, d, g])
    }
}
/// n = 9, i = 5, b < h, b < i, c < g, d < g, e < f, e < h, e < i, g < h, g < i
fn select_1098([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_1096([b, e, g, a, f, h, i])
    } else {
        select_1097([b, a, c, d, e, f, g])
    }
}
/// n = 12, i = 5, b < h, b < l, c < g, c < h, d < g, d < h, e < f, e < h, e < l, g < l, h < i, h < j, h < k
fn select_1099([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_1098([a, b, c, d, e, f, g, h, l])
    } else {
        select_351([a, f, b, c, d, h, g, i, j, k])
    }
}
/// n = 13, i = 5, b < m, c < h, c < l, d < h, d < l, d < m, e < g, e < h, f < g, f < h, g < l, g < m, h < i, h < j, h < k
fn select_1100([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < c {
        select_1095([a, d, e, f, b, c, g, m, h])
    } else {
        select_1099([a, d, e, f, c, b, g, h, i, j, k, l])
    }
}
/// n = 14, i = 5, b < i, c < j, d < h, d < j, e < h, e < i, e < j, f < g, f < j, g < h, g < i, i < n, j < k, j < l, j < m
fn select_1101([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < c {
        select_1093([a, b, d, e, g, c, h, i, n, j])
    } else {
        select_1100([a, b, d, e, c, f, g, j, k, l, m, h, i])
    }
}
/// n = 7, i = 3, a < g, b < d, c < e, c < g, e < f
fn select_1102([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_52([a, c, b, d, g, e])
    } else {
        select_23([b, a, e, f, g, d])
    }
}
/// n = 8, i = 4, a < f, b < e, b < g, c < d, c < g, g < h
fn select_1103([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_1102([d, a, b, f, g, h, e])
    } else {
        select_1102([e, a, c, f, g, h, d])
    }
}
/// n = 7, i = 4, a < g, b < f, c < f, c < g, d < f, d < g, e < f, e < g
fn select_1104([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_9([b, c, d, e, g])
    } else {
        select_9([a, c, d, e, f])
    }
}
/// n = 10, i = 4, b < j, c < f, c < j, d < f, d < j, e < f, e < j, f < g, f < h, f < i
fn select_1105([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_68([b, f, a, g, h, i, j])
    } else {
        select_1104([a, b, c, d, e, j, f])
    }
}
/// n = 13, i = 5, b < g, c < f, d < g, d < j, e < g, e < j, f < j, g < h, g < i, j < k, j < l, j < m
fn select_1106([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_678([a, c, g, h, i, f])
    } else {
        select_1105([a, b, d, e, f, j, k, l, m, g])
    }
}
/// n = 14, i = 6, a < h, b < f, c < h, c < k, d < e, d < g, f < k, g < j, g < k, h < i, h < j, k < l, k < m, k < n
fn select_1107([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_1103([b, d, h, i, e, f, g, j])
    } else {
        select_1106([e, a, b, c, g, f, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 6, a < h, b < k, c < k, d < h, d < k, e < f, e < g, g < j, g < k, h < i, h < j, k < l, k < m, k < n
fn select_1108([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < c {
        select_1107([a, b, d, e, f, c, g, h, i, j, k, l, m, n])
    } else {
        select_1107([a, c, d, e, f, b, g, h, i, j, k, l, m, n])
    }
}
/// n = 15, i = 6, a < i, b < l, c < k, c < l, d < i, d < k, d < l, e < f, e < g, g < h, g < l, h < k, i < j, l < m, l < n, l < o
fn select_1109([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_1101([f, a, b, c, d, g, h, k, i, l, m, n, o, j])
    } else {
        select_1108([a, b, c, d, e, f, g, i, j, h, l, m, n, o])
    }
}
/// n = 15, i = 6, b < i, c < k, c < o, d < i, d < k, d < o, e < f, e < g, g < h, g < k, h < o, i < j, k < l, k < m, k < n
fn select_1110([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < a {
        select_1086([f, b, c, g, d, h, i, j, k, a, l, m, n, o])
    } else {
        select_1109([b, a, c, d, e, f, g, h, i, j, o, k, l, m, n])
    }
}
/// n = 10, i = 5, c < i, c < j, d < g, e < g, f < g, g < h, g < i, g < j
fn select_1111([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_915([a, c, d, e, f, b, g, h, i, j])
    } else {
        select_915([b, c, d, e, f, a, g, h, i, j])
    }
}
/// n = 7, i = 5, a < g, b < g, c < g, d < g, e < f
fn select_1112([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_560([b, c, d, e, f, g])
    } else {
        select_8([a, b, c, d])
    }
}
/// n = 9, i = 5, b < g, c < g, d < g, e < f, e < h, e < i, g < h, g < i
fn select_1113([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_35([e, g, a, f, h, i])
    } else {
        select_1112([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 5, c < h, c < i, d < g, e < g, f < g, g < h, g < i
fn select_1114([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < a {
        select_1113([b, d, e, f, c, a, g, h, i])
    } else {
        select_1097([b, d, e, f, a, c, g])
    }
}
/// n = 9, i = 5, c < i, d < h, d < i, e < g, f < g, g < h, g < i
fn select_1115([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < g {
        select_1114([a, b, d, c, e, f, g, h, i])
    } else {
        select_279([a, b, d, g, c, h, i])
    }
}
/// n = 11, i = 5, c < h, d < g, d < k, e < h, e < j, f < g, f < h, g < j, h < i, h < k
fn select_1116([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_1111([a, b, d, c, e, f, h, i, g, k])
    } else {
        select_1115([a, b, c, e, d, f, g, j, h])
    }
}
/// n = 9, i = 5, a < e, b < g, c < g, d < f, f < h, f < i, g < h, g < i
fn select_1117([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_255([a, g, d, f, e, h, i])
    } else {
        select_560([b, c, e, d, f, g])
    }
}
/// n = 10, i = 5, a < g, b < e, b < i, c < g, c < h, d < f, f < h, f < j, g < j, h < i
fn select_1118([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_1117([b, a, c, d, e, f, g, h, j])
    } else {
        select_165([a, b, c, f, e, h, i, g])
    }
}
/// n = 8, i = 3, a < f, a < h, b < d, b < g, c < e, c < f, e < g, e < h
fn select_1119([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_58([d, a, c, e, g, f])
    } else {
        select_51([b, c, f, d, e, h])
    }
}
/// n = 9, i = 4, a < e, a < i, b < d, b < h, c < f, f < g, f < h, g < i
fn select_1120([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_1119([b, a, f, e, g, h, i, d])
    } else {
        select_292([a, c, b, e, d, f, h, g])
    }
}
/// n = 10, i = 5, b < e, b < j, c < g, d < f, d < h, f < i, g < h, g < i, h < j
fn select_1121([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_1118([a, b, d, c, e, g, f, h, j, i])
    } else {
        select_1120([b, f, c, a, e, g, h, i, j])
    }
}
/// n = 8, i = 4, a < e, b < g, c < f, c < h, d < f, d < g, g < h
fn select_1122([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_795([a, c, b, f, g, h, e])
    } else {
        select_158([b, c, d, a, e, f, g])
    }
}
/// n = 8, i = 4, b < e, c < g, c < h, d < f, d < g, f < h
fn select_1123([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_1122([b, a, c, d, e, g, f, h])
    } else {
        select_938([b, c, f, a, g, e, h])
    }
}
/// n = 9, i = 4, b < e, b < i, c < g, c < h, d < f, d < g, f < h, g < i
fn select_1124([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < i {
        select_1123([a, b, c, d, e, f, g, h])
    } else {
        select_372([a, c, d, b, e, g, i])
    }
}
/// n = 10, i = 5, b < f, b < j, c < e, c < i, d < g, g < h, g < i, i < j
fn select_1125([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_1121([a, b, d, c, f, e, g, i, h, j])
    } else {
        select_1124([a, b, c, g, f, h, i, e, j])
    }
}
/// n = 12, i = 6, b < i, c < h, c < l, d < f, d < g, e < h, e < i, g < k, g < l, h < k, i < j, i < l
fn select_1126([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_1116([a, f, b, c, g, e, h, i, j, k, l])
    } else {
        select_1125([a, c, d, b, f, h, i, j, g, l])
    }
}
/// n = 7, i = 1, b < g, c < f, d < e
fn select_1127([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_172([a, d, b, c, g, e])
    } else {
        select_172([a, d, c, b, f, e])
    }
}
/// n = 9, i = 2, b < e, c < d, c < f, c < g, f < i, g < h
fn select_1128([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < b {
        select_1127([a, d, f, g, h, i, b])
    } else {
        select_231([a, b, c, e, f, g, h, i, d])
    }
}
/// n = 9, i = 2, b < g, c < d, c < e, c < f, c < h, h < i
fn select_1129([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_1128([a, b, c, f, g, d, h, i, e])
    } else {
        select_1128([a, b, c, f, g, e, h, i, d])
    }
}
/// n = 9, i = 3, b < e, c < f, d < e, d < f, f < g, f < h, f < i
fn select_1130([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_302([a, c, b, d, e, f])
    } else {
        select_547([b, c, e, f, a, g, h, i])
    }
}
/// n = 10, i = 3, b < e, b < f, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_1131([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_277([a, c, d, b, e, f])
    } else {
        select_1130([a, b, d, c, f, g, h, i, j])
    }
}
/// n = 10, i = 4, b < f, c < g, d < f, d < g, e < f, e < g, g < h, g < i, g < j
fn select_1132([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_391([a, c, b, d, e, f, g])
    } else {
        select_547([b, c, f, g, a, h, i, j])
    }
}
/// n = 12, i = 5, b < h, b < i, c < f, c < i, d < g, d < i, e < g, e < i, g < h, i < j, i < k, i < l
fn select_1133([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_1131([a, g, b, c, f, h, i, j, k, l])
    } else {
        select_1132([a, f, b, d, e, g, i, j, k, l])
    }
}
/// n = 12, i = 5, b < f, c < h, c < l, d < g, d < h, e < g, e < h, g < l, h < i, h < j, h < k
fn select_1134([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < b {
        select_1129([a, g, h, i, j, k, l, b, f])
    } else {
        select_1133([a, c, b, d, e, f, g, l, h, i, j, k])
    }
}
/// n = 8, i = 1, e < f, e < g, e < h
fn select_1135([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_892([c, d, a, e, f, g, h, b])
    } else {
        select_892([c, d, b, e, f, g, h, a])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < e, c < f, c < h
fn select_1136([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_398([a, g, c, d, e, f, h])
    } else {
        select_349([a, d, e, f, b, g, h])
    }
}
/// n = 10, i = 2, b < g, c < d, c < e, c < f, c < h, h < i, h < j
fn select_1137([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < b {
        select_1135([a, d, e, f, h, b, i, j])
    } else {
        select_1136([a, b, c, d, e, f, g, h])
    }
}
/// n = 9, i = 5, b < f, b < i, c < g, d < g, e < g, g < h, g < i
fn select_1138([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_386([b, a, c, d, e, f, g, h])
    } else {
        select_105([b, g, a, h, f, i])
    }
}
/// n = 9, i = 5, b < h, b < i, c < f, c < i, d < g, e < g, g < h, g < i
fn select_1139([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < g {
        select_1138([a, c, b, d, e, f, g, h, i])
    } else {
        select_106([a, c, g, f, b, h, i])
    }
}
/// n = 11, i = 4, c < f, d < f, e < f, f < g, f < h, f < i, f < j, f < k
fn select_1140([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_351([a, b, c, d, e, f, g, i, j, k])
    } else {
        select_351([a, b, c, d, e, f, h, i, j, k])
    }
}
/// n = 13, i = 5, b < h, b < i, c < f, c < i, d < g, d < i, e < g, e < i, g < h, i < j, i < k, i < l, i < m
fn select_1141([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < i {
        select_1139([a, b, c, d, e, f, g, h, i])
    } else {
        select_1140([a, f, b, d, e, i, g, j, k, l, m])
    }
}
/// n = 13, i = 5, b < f, b < m, c < h, c < l, d < g, d < h, e < g, e < h, g < l, h < i, h < j, h < k, h < m
fn select_1142([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_1137([a, g, h, i, j, k, l, b, f, m])
    } else {
        select_1141([a, c, b, d, e, f, g, l, h, i, j, k, m])
    }
}
/// n = 13, i = 5, c < m, d < h, d < l, e < g, e < h, f < g, f < h, g < l, h < i, h < j, h < k, h < m
fn select_1143([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < c {
        select_1134([b, a, d, e, f, c, g, h, i, j, k, l])
    } else {
        select_1142([b, c, d, e, f, a, g, h, i, j, k, l, m])
    }
}
/// n = 15, i = 6, b < i, c < h, c < k, d < f, d < g, e < h, e < i, e < k, g < k, g < o, h < o, i < j, k < l, k < m, k < n
fn select_1144([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < k {
        select_1126([a, b, c, d, e, f, g, h, i, j, o, k])
    } else {
        select_1143([a, f, b, g, c, e, h, k, l, m, n, o, i])
    }
}
/// n = 15, i = 6, b < j, c < i, c < l, d < i, d < j, d < l, e < f, e < g, g < h, g < l, j < k, l < m, l < n, l < o
fn select_1145([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_1110([a, b, c, d, e, f, g, h, j, k, l, m, n, o, i])
    } else {
        select_1144([a, b, c, e, d, f, g, i, j, k, l, m, n, o, h])
    }
}
/// n = 16, i = 7, a < k, b < j, b < l, c < f, c < g, d < e, d < h, g < j, g < k, g < p, h < i, h < l, k < o, l < m, l < n, l < p
fn select_1146([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < g {
        select_1069([e, i, a, c, b, f, l, m, n, g, j, p, k])
    } else {
        select_1145([f, a, b, g, d, e, h, i, j, k, o, l, m, n, p])
    }
}
/// n = 16, i = 7, a < k, b < g, b < l, c < f, c < h, d < e, d < i, h < k, h < p, i < j, i < l, k < o, l < m, l < n, l < p
fn select_1147([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < h {
        select_1028([a, c, d, b, e, f, g, i, j, l, h, m, n, k, o, p])
    } else {
        select_1146([a, b, c, d, e, f, h, i, j, g, k, l, m, n, o, p])
    }
}
/// n = 8, i = 2, b < e, b < h, c < d, c < f, c < h, f < g
fn select_1148([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_275([a, b, c, e, d, f, h])
    } else {
        select_766([a, b, c, e, f, d, g, h])
    }
}
/// n = 9, i = 3, a < d, a < g, a < h, a < i, b < c, b < e, b < g, d < f, e < h, e < i
fn select_1149([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_176([c, e, a, g, h, i, d])
    } else {
        select_36([d, b, g, f, e, h, i, c])
    }
}
/// n = 9, i = 3, a < e, a < h, a < i, b < c, b < d, b < f, e < g, f < h, f < i
fn select_1150([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < a {
        select_88([d, f, c, a, e])
    } else {
        select_1149([a, b, d, e, f, g, c, h, i])
    }
}
/// n = 11, i = 4, a < f, a < g, b < g, b < h, b < j, c < d, c < e, c < f, e < h, e < j, g < i, j < k
fn select_1151([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_1148([d, a, b, h, f, j, k, g])
    } else {
        select_1150([b, c, d, f, g, e, i, h, j])
    }
}
/// n = 10, i = 3, a < d, a < f, b < e, b < g, c < e, c < f, c < g, f < j, g < h, g < i
fn select_1152([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_659([d, b, c, f, j, e, g])
    } else {
        select_36([a, c, e, d, g, h, i, f])
    }
}
/// n = 10, i = 3, a < d, a < h, b < g, b < h, b < i, c < e, c < g, e < f, e < i, h < j
fn select_1153([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_64([a, e, f, d, b, g, i])
    } else {
        select_1152([a, c, b, d, g, h, e, f, i, j])
    }
}
/// n = 9, i = 4, a < g, a < i, b < d, b < g, c < e, c < i, d < h, d < i, e < f, e < h
fn select_1154([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_150([a, b, e, f, g, d, h])
    } else {
        select_185([a, c, d, g, e, f, h, i])
    }
}
/// n = 10, i = 4, a < g, a < j, b < e, b < j, c < d, c < g, c < h, d < j, e < f, e < h, h < i
fn select_1155([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_76([a, c, e, f, g, h, d, i])
    } else {
        select_1154([a, c, b, d, e, f, g, h, j])
    }
}
/// n = 11, i = 4, a < g, a < h, b < d, b < g, b < j, c < e, c < h, e < f, e < j, h < i, j < k
fn select_1156([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < d {
        select_1153([e, b, a, f, h, i, g, j, d, k])
    } else {
        select_1155([a, c, b, d, e, f, g, j, k, h])
    }
}
/// n = 11, i = 4, a < h, a < i, b < d, b < f, b < h, c < e, c < i, c < j, e < g, f < j, i < k
fn select_1157([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_1151([a, c, b, d, f, h, i, j, k, e, g])
    } else {
        select_1156([a, b, c, d, e, g, h, i, k, f, j])
    }
}
/// n = 8, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h
fn select_1158([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_19([d, e, a, g])
    } else {
        select_126([b, a, c, d, f, h, e])
    }
}
/// n = 8, i = 3, b < d, b < g, c < e, c < g, e < f, g < h
fn select_1159([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_318([e, b, d, a, f, g, h])
    } else {
        select_1158([a, b, c, d, g, e, h, f])
    }
}
/// n = 8, i = 3, b < f, c < g, d < e, d < g, g < h
fn select_1160([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_1038([a, d, b, f, e, c, g])
    } else {
        select_1159([a, d, c, e, b, f, g, h])
    }
}
/// n = 9, i = 2, b < e, b < h, b < i, c < d, c < h, c < i, d < f, e < g
fn select_1161([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_659([a, b, c, d, f, h, i])
    } else {
        select_659([a, c, b, e, g, h, i])
    }
}
/// n = 10, i = 3, b < d, b < g, b < j, c < e, c < g, e < f, e < j, f < h, g < i
fn select_1162([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_58([a, d, c, g, i, e])
    } else {
        select_1161([a, b, e, f, g, h, i, d, j])
    }
}
/// n = 10, i = 4, a < e, a < f, a < j, b < f, b < g, c < d, c < g, g < h, g < j, h < i
fn select_1163([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_173([d, h, a, e, f, j, i])
    } else {
        select_57([e, b, c, d, f, g, h, j])
    }
}
/// n = 11, i = 4, a < g, a < i, b < e, b < g, b < h, c < d, c < f, f < h, f < i, h < j, i < k
fn select_1164([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_1162([d, b, f, e, a, i, h, k, j, g])
    } else {
        select_1163([b, a, c, d, e, g, f, i, k, h])
    }
}
/// n = 11, i = 4, a < h, a < i, b < e, b < f, b < h, c < d, c < g, f < j, g < i, g < j, i < k
fn select_1165([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1160([e, a, f, c, d, h, g, j])
    } else {
        select_1164([a, b, c, d, e, g, h, f, i, j, k])
    }
}
/// n = 11, i = 4, a < f, b < i, b < j, c < h, c < i, d < e, d < g, d < h, g < j, i < k
fn select_1166([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < a {
        select_1157([c, d, b, e, a, g, f, h, i, j, k])
    } else {
        select_1165([c, d, a, f, e, g, b, h, i, j, k])
    }
}
/// n = 8, i = 3, a < h, b < d, b < g, c < e, e < f, e < g, e < h
fn select_1167([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_172([f, a, b, d, g, h])
    } else {
        select_47([a, d, c, e, f, g])
    }
}
/// n = 9, i = 4, a < e, b < f, b < i, c < g, d < f, d < g, g < h, g < i
fn select_1168([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_158([c, b, d, a, e, f, g])
    } else {
        select_1167([a, b, c, f, g, h, i, e])
    }
}
/// n = 8, i = 4, a < h, b < e, c < g, c < h, d < f, d < g, f < h
fn select_1169([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_160([a, c, b, d, f, e, g, h])
    } else {
        select_1008([a, f, b, c, g, e, h])
    }
}
/// n = 9, i = 4, a < f, b < h, c < e, c < g, d < g, d < h, h < i
fn select_1170([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < e {
        select_1168([a, c, b, d, f, g, h, i, e])
    } else {
        select_1169([b, a, d, c, f, e, g, h])
    }
}
/// n = 9, i = 4, a < h, a < i, b < e, c < f, d < f, d < i, e < i, f < g, f < h
fn select_1171([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_153([a, c, d, e, f, i])
    } else {
        select_5([a, b, f, g, e, h])
    }
}
/// n = 9, i = 4, a < g, b < f, c < e, c < i, d < g, d < i, f < i, g < h
fn select_1172([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_1171([c, b, a, d, f, g, h, e, i])
    } else {
        select_160([a, d, b, c, e, f, i, g])
    }
}
/// n = 9, i = 3, b < h, b < i, c < f, c < g, d < e, e < f, e < g, e < h, e < i
fn select_1173([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_163([a, d, b, c, e, h, i])
    } else {
        select_163([a, d, c, b, e, f, g])
    }
}
/// n = 11, i = 4, a < j, a < k, b < f, b < i, c < e, c < g, d < f, d < g, f < h, g < i, g < j, g < k
fn select_1174([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_308([a, c, b, d, e, f, h, g])
    } else {
        select_1173([e, a, b, d, g, f, i, j, k])
    }
}
/// n = 11, i = 4, a < f, a < k, b < g, b < j, c < e, c < h, d < g, d < h, g < i, h < j, h < k
fn select_1175([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_1172([b, a, c, d, e, f, g, i, h])
    } else {
        select_1174([a, b, c, d, e, g, h, i, j, f, k])
    }
}
/// n = 12, i = 4, a < f, a < g, a < l, b < g, b < h, b < k, c < e, c < i, d < h, d < i, h < j, i < k, i < l
fn select_1176([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_1170([c, d, a, b, f, e, g, h, j])
    } else {
        select_1175([a, b, c, d, e, f, h, i, j, k, l])
    }
}
/// n = 10, i = 3, a < e, a < g, b < c, b < d, b < g, e < f, g < h, g < i, g < j
fn select_1177([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_548([a, b, c, e, d, f, g, h, i, j])
    } else {
        select_548([a, b, d, e, c, f, g, h, i, j])
    }
}
/// n = 12, i = 4, a < e, a < f, a < l, b < f, b < g, c < d, c < e, c < g, f < i, g < h, g < j, g < l, j < k
fn select_1178([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_892([d, h, j, a, e, f, l, k])
    } else {
        select_1177([b, c, d, e, f, i, g, h, j, l])
    }
}
/// n = 11, i = 3, a < d, a < g, b < c, b < g, b < k, c < f, d < e, d < k, g < h, g < i, g < j
fn select_1179([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_547([c, a, f, g, d, h, i, j])
    } else {
        select_758([a, b, c, d, e, f, g, k])
    }
}
/// n = 11, i = 3, a < c, a < j, a < k, b < d, b < e, c < i, d < h, d < k, e < f, e < g, e < j
fn select_1180([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_116([d, e, f, g, h, a, c, j])
    } else {
        select_1179([b, a, c, d, h, i, e, f, g, j, k])
    }
}
/// n = 10, i = 4, a < f, a < j, b < f, b < j, c < d, c < h, c < i, d < e, d < j, f < g, f < h, f < i
fn select_1181([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_87([e, a, b, j])
    } else {
        select_29([f, c, g, d, j, h, i])
    }
}
/// n = 11, i = 4, a < d, a < i, a < j, b < f, b < k, c < f, c < k, d < e, d < k, f < g, f < h, f < i, f < j
fn select_1182([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_310([g, h, a, i, j])
    } else {
        select_1181([b, c, a, d, e, f, h, i, j, k])
    }
}
/// n = 12, i = 4, a < g, a < l, b < d, b < f, b < k, c < f, c < g, c < l, d < e, d < l, f < h, g < i, g < j, g < k
fn select_1183([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_186([a, d, c, e, f, h, g, l])
    } else {
        select_1182([b, a, c, d, e, g, i, j, f, k, l])
    }
}
/// n = 13, i = 4, a < d, a < g, a < m, b < f, b < h, c < f, c < g, c < h, d < e, f < k, g < l, h < i, h < j, h < m
fn select_1184([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < d {
        select_1180([a, b, g, f, h, i, j, k, l, m, d])
    } else {
        select_1183([b, a, c, d, e, g, h, l, i, j, m, f])
    }
}
/// n = 14, i = 5, a < g, a < i, a < k, b < d, b < j, c < e, c < g, d < f, d < i, d < m, e < j, e < k, e < m, g < h, j < l, k < n
fn select_1185([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < e {
        select_1178([a, c, d, f, i, g, e, m, h, j, l, k])
    } else {
        select_1184([a, b, e, g, h, j, k, d, f, m, l, n, i])
    }
}
/// n = 14, i = 5, a < i, a < j, a < l, b < f, b < i, c < d, c < e, d < h, e < g, e < j, e < m, f < l, f < m, i < k, l < n
fn select_1186([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < f {
        select_1176([e, a, d, b, h, g, j, i, f, k, l, m])
    } else {
        select_1185([a, c, b, e, f, g, i, k, j, d, l, h, m, n])
    }
}
/// n = 14, i = 5, a < j, a < k, a < m, b < e, b < j, c < d, c < f, d < i, e < m, f < g, f < h, f < k, j < l, m < n
fn select_1187([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < e {
        select_1166([d, b, a, f, h, i, g, k, j, e, l])
    } else {
        select_1186([a, b, c, d, f, e, h, i, j, k, l, m, g, n])
    }
}
/// n = 11, i = 4, a < d, a < i, b < e, b < h, c < f, e < g, e < j, f < h, f < i, f < j, i < k
fn select_1188([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_609([a, c, e, g, d, f, h, j])
    } else {
        select_1152([a, b, f, d, h, i, e, g, j, k])
    }
}
/// n = 12, i = 5, a < h, a < j, b < e, c < d, c < h, d < f, d < k, e < g, e < j, e < k, h < i, k < l
fn select_1189([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_1155([a, c, e, g, d, f, j, k, l, h])
    } else {
        select_1188([d, a, b, f, h, e, i, j, k, g, l])
    }
}
/// n = 12, i = 5, a < i, a < k, b < e, c < d, c < i, c < l, d < g, e < f, e < h, e < k, h < l, i < j
fn select_1190([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < d {
        select_1151([a, c, e, f, h, k, i, l, j, d, g])
    } else {
        select_1189([a, b, c, d, e, g, f, i, j, k, h, l])
    }
}
/// n = 10, i = 4, a < e, a < g, b < e, b < h, c < d, d < g, d < h, e < f, h < i, h < j
fn select_1191([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_27([a, i, j])
    } else {
        select_474([c, b, e, d, g, f, h])
    }
}
/// n = 11, i = 4, a < e, a < g, b < e, b < h, b < i, c < d, d < g, d < h, d < i, e < f, h < k, i < j
fn select_1192([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_1191([a, b, c, d, e, f, g, h, i, k])
    } else {
        select_1191([a, b, c, d, e, f, g, i, h, j])
    }
}
/// n = 12, i = 5, a < g, a < h, b < e, c < d, c < i, d < h, d < k, e < f, e < g, e < k, f < i, h < j, k < l
fn select_1193([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < d {
        select_135([a, c, f, g, i, d, k])
    } else {
        select_1192([a, d, b, e, h, j, g, f, k, l, i])
    }
}
/// n = 12, i = 5, a < h, a < i, b < d, b < j, c < e, d < i, d < l, e < f, e < g, e < h, f < l, g < j, i < k
fn select_1194([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < d {
        select_133([g, a, f, b, h, j, d, l])
    } else {
        select_1193([a, c, b, d, e, g, h, i, j, k, f, l])
    }
}
/// n = 10, i = 4, a < f, a < g, b < d, c < e, d < f, d < h, d < i, e < g, e < h, e < i, g < j
fn select_1195([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_46([a, c, f, e, h, i])
    } else {
        select_666([b, c, d, e, f, g, j])
    }
}
/// n = 10, i = 4, a < g, a < h, b < e, c < d, c < f, d < h, d < i, e < f, e < g, e < i, h < j
fn select_1196([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_25([a, c, g, f, d])
    } else {
        select_1195([a, b, c, e, d, g, h, f, i, j])
    }
}
/// n = 11, i = 5, a < h, a < j, b < d, c < e, d < f, d < h, d < i, e < g, e < i, g < j, j < k
fn select_1197([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_1196([a, b, e, g, d, i, h, j, f, k])
    } else {
        select_184([f, a, c, h, e, i, g])
    }
}
/// n = 9, i = 4, a < f, b < e, c < d, c < f, c < h, d < i, e < g, e < i, g < h
fn select_1198([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_306([a, d, b, f, e, g, i])
    } else {
        select_255([a, c, e, g, f, d, h])
    }
}
/// n = 9, i = 4, a < f, a < h, b < e, c < d, c < f, d < i, e < h, e < i, f < g
fn select_1199([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_82([d, b, f, g, e])
    } else {
        select_470([a, e, c, d, h, i, f])
    }
}
/// n = 12, i = 5, a < h, a < i, b < d, c < e, d < g, d < h, d < l, e < f, e < i, e < k, f < l, g < k, i < j
fn select_1200([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_1198([a, c, d, g, e, h, f, l, k])
    } else {
        select_1199([a, b, e, f, d, i, j, h, l])
    }
}
/// n = 12, i = 5, a < h, a < i, b < f, c < d, c < e, d < j, e < i, e < l, f < g, f < h, f < j, g < l, i < k
fn select_1201([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_1197([a, b, c, f, d, g, e, h, j, i, k])
    } else {
        select_1200([a, b, c, f, e, d, g, h, i, k, l, j])
    }
}
/// n = 12, i = 5, a < i, a < j, b < f, c < d, c < e, e < j, e < l, f < g, f < h, f < i, h < l, j < k
fn select_1202([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < d {
        select_1194([a, c, b, e, f, h, g, i, j, d, k, l])
    } else {
        select_1201([a, b, c, d, e, f, h, i, j, g, k, l])
    }
}
/// n = 12, i = 5, a < e, b < i, b < l, c < i, c < j, d < f, f < g, f < h, f < j, h < l, i < k
fn select_1203([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < a {
        select_1190([c, d, b, a, f, g, e, h, i, k, j, l])
    } else {
        select_1202([c, d, a, e, b, f, g, h, j, i, k, l])
    }
}
/// n = 9, i = 3, a < d, b < e, b < f, c < e, c < f, e < i, f < g, f < h
fn select_1204([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_67([e, f, a, g, h, i])
    } else {
        select_126([a, b, c, d, e, i, f])
    }
}
/// n = 9, i = 3, a < d, b < g, b < i, c < e, c < g, e < f, e < i, g < h
fn select_1205([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_64([a, e, f, d, b, g, i])
    } else {
        select_1204([a, b, c, d, g, e, f, i, h])
    }
}
/// n = 7, i = 4, b < f, b < g, c < f, c < g, d < e, d < g
fn select_1206([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_372([a, b, c, d, e, f, g])
    } else {
        select_368([a, e, b, c, g, f])
    }
}
/// n = 9, i = 4, a < e, b < h, b < i, c < f, c < g, d < f, d < g, g < h, g < i
fn select_1207([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_659([f, b, g, a, e, h, i])
    } else {
        select_1206([b, c, d, a, e, f, g])
    }
}
/// n = 10, i = 4, a < e, b < g, b < j, c < h, c < j, d < f, d < g, f < h, f < j, h < i
fn select_1208([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_1205([a, c, f, e, b, g, h, i, j])
    } else {
        select_1207([a, c, b, d, e, g, f, h, j])
    }
}
/// n = 10, i = 4, a < e, b < f, b < j, c < g, d < f, d < g, g < h, g < i, g < j
fn select_1209([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_116([a, g, h, i, e, b, f, j])
    } else {
        select_886([a, c, b, d, e, f, g, h, i, j])
    }
}
/// n = 11, i = 4, a < f, a < h, b < f, b < h, c < d, c < h, d < e, f < g, h < i, h < j, h < k
fn select_1210([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_70([f, c, g, h, i, j, k, d])
    } else {
        select_1058([d, a, b, e, h, i, j, k, f])
    }
}
/// n = 11, i = 4, a < f, a < k, b < f, b < g, c < d, c < g, d < e, f < j, g < h, g < i, g < k
fn select_1211([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_116([d, g, h, i, e, a, f, k])
    } else {
        select_1210([a, b, c, d, e, f, j, g, h, i, k])
    }
}
/// n = 12, i = 5, a < h, a < i, b < f, c < d, c < e, d < g, e < h, e < j, e < k, f < i, f < j, f < k, i < l
fn select_1212([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_1209([d, a, b, e, g, h, f, j, k, i])
    } else {
        select_1211([a, f, c, d, g, i, e, j, k, l, h])
    }
}
/// n = 13, i = 5, a < i, a < j, a < m, b < e, b < m, c < d, c < f, d < g, e < j, e < k, f < h, f < i, f < k, h < m, j < l
fn select_1213([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_1208([d, a, b, f, g, h, i, e, k, m])
    } else {
        select_1212([a, b, c, d, f, e, g, i, j, h, k, l])
    }
}
/// n = 10, i = 3, a < e, b < g, b < j, c < d, c < g, e < f, e < j, g < h, g < i
fn select_1214([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_47([a, d, b, g, h, i])
    } else {
        select_184([b, c, a, d, e, f, j])
    }
}
/// n = 10, i = 5, a < f, b < e, b < j, c < d, c < i, d < h, d < j, e < h, e < i, f < g, f < i, f < j
fn select_1215([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_449([b, d, a, f, g, e, h, j])
    } else {
        select_449([c, e, a, f, g, d, h, i])
    }
}
/// n = 10, i = 5, a < d, a < i, b < f, c < e, c < j, d < j, e < h, e < i, f < g, f < i, f < j
fn select_1216([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < h {
        select_1215([b, a, c, e, d, f, g, h, j, i])
    } else {
        select_579([a, b, e, h, f, g, d, i])
    }
}
/// n = 12, i = 5, a < d, a < i, b < f, b < l, c < e, c < i, d < l, e < h, e < l, f < g, f < i, i < j, i < k
fn select_1217([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < e {
        select_811([d, g, c, i, e, j, k])
    } else {
        select_1216([a, c, b, d, f, e, h, g, i, l])
    }
}
/// n = 13, i = 5, a < d, a < k, b < e, b < k, c < f, c < i, d < h, e < i, f < g, f < k, i < j, k < l, k < m
fn select_1218([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < d {
        select_1214([e, a, f, g, i, j, k, l, m, d])
    } else {
        select_1217([b, c, a, e, d, f, g, h, k, l, m, i])
    }
}
/// n = 10, i = 4, a < d, a < f, b < f, b < g, c < e, c < j, e < g, f < j, g < h, g < i
fn select_1219([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_191([d, c, f, e, j])
    } else {
        select_185([a, b, e, d, g, h, i, f])
    }
}
/// n = 11, i = 4, a < g, a < i, b < g, b < h, c < e, c < f, d < f, d < g, f < h, f < i, g < j, g < k
fn select_1220([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_1219([c, d, a, e, b, f, g, j, k, i])
    } else {
        select_1219([c, d, b, e, a, f, g, j, k, h])
    }
}
/// n = 12, i = 4, a < e, a < g, b < f, b < h, b < l, c < f, c < h, c < k, d < g, d < h, g < k, g < l, h < i, h < j
fn select_1221([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_638([a, d, b, c, e, f, h, i, j])
    } else {
        select_1220([b, c, a, d, e, g, h, k, l, i, j])
    }
}
/// n = 10, i = 4, a < e, a < f, b < d, b < f, c < d, c < e, c < j, d < h, e < i, f < g, f < j
fn select_1222([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_97([a, c, d, f, h, g, j, e])
    } else {
        select_97([b, c, e, f, i, g, j, d])
    }
}
/// n = 10, i = 4, a < f, a < j, b < e, b < f, b < h, c < d, c < e, d < h, d < j, e < g, f < i
fn select_1223([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_1048([d, b, e, a, j, g, h, f])
    } else {
        select_1222([a, c, b, e, f, d, j, g, i, h])
    }
}
/// n = 11, i = 4, a < e, a < h, b < e, b < f, b < i, c < d, d < f, d < h, d < i, e < g, i < j, i < k
fn select_1224([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < d {
        select_3([g, c, d])
    } else {
        select_36([a, b, f, h, i, j, k, e])
    }
}
/// n = 13, i = 4, a < e, a < f, a < i, b < f, b < g, b < k, c < d, c < e, d < g, d < i, d < k, e < h, f < j, k < l, k < m
fn select_1225([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < k {
        select_1223([b, a, c, d, e, f, h, i, j, g])
    } else {
        select_1224([a, b, c, d, f, g, j, i, k, l, m])
    }
}
/// n = 14, i = 5, a < h, a < i, a < l, b < e, b < l, c < d, c < f, d < g, e < i, e < j, f < h, f < j, f < l, i < k, l < m, l < n
fn select_1226([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < e {
        select_1221([d, a, f, b, g, h, e, l, m, n, j, i])
    } else {
        select_1225([e, a, c, f, d, i, h, g, j, k, l, m, n])
    }
}
/// n = 15, i = 6, a < g, a < k, a < l, b < e, b < i, c < h, c < i, d < g, d < h, e < f, g < j, g < o, h < l, h < m, i < k, i < m, i < o, l < n
fn select_1227([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < i {
        select_1218([b, c, a, e, h, g, j, f, l, n, i, k, o])
    } else {
        select_1226([a, d, b, e, h, i, f, k, l, m, n, g, j, o])
    }
}
/// n = 15, i = 6, a < g, a < l, a < m, b < g, b < i, c < e, c < h, d < h, d < i, e < f, g < k, h < j, h < l, h < n, i < m, i < n, m < o
fn select_1228([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < g {
        select_1213([a, b, c, e, i, h, f, j, l, m, n, o, g])
    } else {
        select_1227([a, c, d, b, e, f, g, i, h, k, l, m, n, o, j])
    }
}
/// n = 15, i = 6, a < g, a < m, a < n, b < g, b < h, c < e, c < i, d < h, d < i, e < f, g < l, h < n, i < j, i < k, i < m, n < o
fn select_1229([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < h {
        select_1203([e, b, a, d, f, i, k, j, g, m, l, h])
    } else {
        select_1228([a, b, c, d, e, f, g, i, h, k, l, m, n, j, o])
    }
}
/// n = 15, i = 6, a < h, b < h, b < m, b < o, c < e, c < i, d < f, d < i, e < g, f < m, h < l, i < j, i < k, i < o, m < n
fn select_1230([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < a {
        select_1187([b, f, c, e, a, i, j, k, g, m, o, n, h, l])
    } else {
        select_1229([b, a, c, d, e, g, h, f, i, j, k, l, o, m, n])
    }
}
/// n = 9, i = 3, a < d, a < i, b < f, b < h, c < e, c < f, e < h, e < i, f < g
fn select_1231([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_58([d, b, c, e, i, f])
    } else {
        select_110([a, c, d, e, f, g, h])
    }
}
/// n = 9, i = 3, a < f, a < i, b < e, b < i, c < g, c < i, d < f, d < g, g < h
fn select_1232([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < c {
        select_1119([d, b, a, e, c, f, i, g])
    } else {
        select_1231([b, d, c, e, a, g, h, f, i])
    }
}
/// n = 9, i = 3, a < g, b < d, b < h, c < e, c < g, e < f, e < h, g < i
fn select_1233([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_45([b, e, f, d, a, g, h])
    } else {
        select_134([b, a, c, d, e, g, i, f])
    }
}
/// n = 10, i = 3, a < j, b < g, b < j, c < e, c < h, c < j, d < f, d < g, f < h, g < i
fn select_1234([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_1232([a, c, b, d, e, f, g, i, j])
    } else {
        select_1233([b, c, d, e, f, a, g, h, i])
    }
}
/// n = 8, i = 3, a < h, b < g, b < h, c < f, c < g, d < e, d < g, d < h, e < f
fn select_1235([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_159([b, a, d, e, c, h])
    } else {
        select_214([b, d, c, a, e, f, g])
    }
}
/// n = 9, i = 4, b < e, c < d, c < h, d < f, d < i, e < g, e < i, g < h
fn select_1236([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_47([a, f, b, e, g, i])
    } else {
        select_48([a, g, c, d, f, i, h])
    }
}
/// n = 11, i = 5, a < k, b < g, b < j, c < g, c < i, c < k, d < e, e < f, e < i, f < k, g < h, i < j
fn select_1237([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1235([a, f, b, c, i, j, g, k])
    } else {
        select_1236([a, d, b, g, e, h, i, j, f])
    }
}
/// n = 9, i = 3, a < i, b < h, b < i, c < f, c < g, c < i, d < e, e < f, e < g, e < h
fn select_1238([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_302([b, c, d, a, e, i])
    } else {
        select_176([b, c, e, a, f, g, h])
    }
}
/// n = 7, i = 4, a < f, b < f, c < d, d < e, f < g
fn select_1239([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_13([c, f, g, d])
    } else {
        select_7([a, b, e])
    }
}
/// n = 9, i = 5, a < g, b < g, b < i, c < g, c < i, d < e, e < f, f < i, g < h
fn select_1240([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_1239([b, c, d, e, f, g, h])
    } else {
        select_87([a, b, c, i])
    }
}
/// n = 10, i = 5, a < j, b < g, c < g, c < j, d < e, e < f, e < i, f < j, g < h, g < i
fn select_1241([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_1240([b, a, c, d, e, f, g, h, j])
    } else {
        select_479([d, g, a, h, e, f, i])
    }
}
/// n = 12, i = 5, a < k, a < l, b < g, b < j, b < l, c < g, c < i, d < e, e < f, e < i, f < l, g < h, i < j, i < k
fn select_1242([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_1238([f, a, b, c, i, g, j, k, l])
    } else {
        select_1241([a, c, b, d, e, f, g, h, i, l])
    }
}
/// n = 12, i = 5, a < k, a < l, b < g, b < j, c < g, c < h, c < l, d < e, e < f, e < h, e < j, f < l, g < i, j < k
fn select_1243([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < j {
        select_1237([a, b, c, d, e, f, g, i, h, j, l])
    } else {
        select_1242([a, c, b, d, e, f, g, i, j, h, k, l])
    }
}
/// n = 13, i = 5, a < g, b < k, b < m, c < g, c < i, c < m, d < e, e < f, e < h, e < i, f < m, g < j, h < k, k < l
fn select_1244([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_1234([f, b, c, h, i, a, k, g, l, m])
    } else {
        select_1243([b, a, c, d, e, f, g, i, j, h, k, m])
    }
}
/// n = 10, i = 4, a < g, a < j, b < e, b < j, c < d, c < g, c < h, d < i, d < j, e < f, e < h, e < i
fn select_1245([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_59([f, a, c, h, d, i, g])
    } else {
        select_598([b, c, g, d, e, f, h, j])
    }
}
/// n = 10, i = 4, a < e, a < f, b < f, b < h, c < d, c < e, c < j, d < h, f < g, f < j, h < i
fn select_1246([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_35([c, f, g, e, d, j])
    } else {
        select_126([a, b, d, e, h, i, f])
    }
}
/// n = 11, i = 4, a < h, a < i, b < e, b < f, c < d, c < f, c < g, d < i, d < j, e < g, e < h, e < j, i < k
fn select_1247([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_284([e, c, f, d, a, h, g, j])
    } else {
        select_1246([b, a, c, d, f, e, h, i, k, g])
    }
}
/// n = 13, i = 5, a < i, a < k, a < m, b < d, b < h, c < e, c < m, d < f, d < i, d < j, e < g, e < h, e < j, f < m, g < k, k < l
fn select_1248([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_1245([a, c, d, f, e, h, i, j, g, m])
    } else {
        select_1247([a, b, e, g, d, h, j, i, k, f, l])
    }
}
/// n = 7, i = 3, a < g, b < f, c < f, c < g, d < e, d < f, d < g
fn select_1249([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_159([c, a, d, e, b, g])
    } else {
        select_159([c, b, d, e, a, f])
    }
}
/// n = 9, i = 4, a < f, a < i, b < f, b < h, c < e, d < f, d < h, d < i, e < h, e < i, f < g
fn select_1250([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_642([b, c, d, e, f, g, i])
    } else {
        select_11([a, b, d, h, i])
    }
}
/// n = 10, i = 4, a < i, a < j, b < f, b < j, c < f, c < i, d < e, d < h, e < i, e < j, f < g, f < h
fn select_1251([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_105([d, f, a, g, e, h])
    } else {
        select_1250([b, c, d, a, e, f, g, i, j])
    }
}
/// n = 11, i = 4, a < f, a < j, a < k, b < g, b < k, c < g, c < j, d < e, d < f, d < i, e < j, e < k, g < h, g < i
fn select_1252([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1249([b, c, e, a, f, j, k])
    } else {
        select_1251([a, b, c, d, e, g, h, i, j, k])
    }
}
/// n = 10, i = 4, a < d, b < g, b < h, c < e, d < g, e < f, e < h, h < i, h < j
fn select_1253([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_34([f, h, a, i, j])
    } else {
        select_579([b, c, a, d, e, f, g, h])
    }
}
/// n = 9, i = 3, a < f, b < d, c < f, c < i, d < e, d < i, f < g, f < h
fn select_1254([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_27([b, g, h])
    } else {
        select_22([a, c, d, e, i, f])
    }
}
/// n = 11, i = 4, a < f, b < e, b < h, c < e, c < h, c < k, d < f, d < h, f < g, f < k, h < i, h < j
fn select_1255([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_47([a, e, d, h, i, j])
    } else {
        select_1254([b, a, c, f, g, h, i, j, k])
    }
}
/// n = 12, i = 5, a < g, b < f, b < i, c < e, c < j, d < f, d < g, e < i, e < k, e < l, f < h, f < k, g < j, g < l
fn select_1256([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_1253([a, c, b, g, f, h, j, e, i, k])
    } else {
        select_1255([a, b, e, d, i, g, j, f, h, k, l])
    }
}
/// n = 13, i = 5, a < h, a < m, b < g, b < i, b < m, c < g, c < h, d < e, d < k, e < f, e < i, e < l, f < m, g < j, h < k, h < l
fn select_1257([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_1252([b, a, c, e, f, i, h, k, l, g, m])
    } else {
        select_1256([a, b, d, c, e, g, h, j, i, k, f, l])
    }
}
/// n = 13, i = 5, a < h, b < h, b < i, b < m, c < e, c < m, d < f, d < j, e < j, e < l, f < g, f < i, f < l, g < m, h < k
fn select_1258([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < a {
        select_1248([b, d, c, f, e, g, a, j, i, l, h, k, m])
    } else {
        select_1257([c, b, a, d, f, g, h, e, i, k, j, l, m])
    }
}
/// n = 13, i = 5, a < i, b < e, b < m, c < i, c < j, c < m, d < f, d < k, e < k, f < g, f < h, f < j, h < m, i < l
fn select_1259([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_1244([a, b, c, d, f, h, i, g, j, l, e, k, m])
    } else {
        select_1258([a, c, b, d, e, f, h, i, j, k, l, g, m])
    }
}
/// n = 8, i = 2, a < h, b < e, b < g, c < d, c < g, c < h, e < f
fn select_1260([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_172([a, e, c, d, g, f])
    } else {
        select_172([d, a, b, e, g, h])
    }
}
/// n = 10, i = 3, a < c, a < f, a < i, b < d, b < f, b < j, c < e, c < j, d < g, d < i, g < h
fn select_1261([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_35([b, c, e, f, d, j])
    } else {
        select_29([a, d, f, g, h, c, i])
    }
}
/// n = 10, i = 3, a < d, a < g, a < j, b < c, b < e, b < g, d < f, e < h, e < j, h < i
fn select_1262([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_1260([c, e, a, g, h, i, j, d])
    } else {
        select_1261([a, b, d, e, f, g, h, i, j, c])
    }
}
/// n = 9, i = 5, a < h, a < i, b < f, c < f, d < e, d < i, e < h, f < g, f < i
fn select_1263([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_878([a, e, b, c, h, f, g, i])
    } else {
        select_11([a, g, d, e, i])
    }
}
/// n = 9, i = 3, a < d, b < g, b < h, b < i, c < e, e < f, e < g, e < h, e < i
fn select_1264([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_173([f, a, b, g, h, i, d])
    } else {
        select_88([b, d, c, e, f])
    }
}
/// n = 12, i = 5, a < f, a < k, a < l, b < g, b < i, c < f, c < i, d < e, d < i, e < g, f < h, i < j, i < k, i < l
fn select_1265([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_1263([b, a, c, d, e, f, h, g, i])
    } else {
        select_1264([e, a, c, g, i, j, f, k, l])
    }
}
/// n = 12, i = 5, a < e, a < l, b < f, b < k, b < l, c < g, c < j, d < f, d < g, e < j, f < i, g < h, g < k, g < l
fn select_1266([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_1262([b, g, h, f, a, i, k, e, j, l])
    } else {
        select_1265([b, c, d, a, e, f, j, i, g, h, k, l])
    }
}
/// n = 9, i = 5, a < h, b < g, b < h, c < g, c < h, d < e, d < i, e < f, h < i
fn select_1267([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_53([g, a, d, e, h, i])
    } else {
        select_637([a, b, c, e, f, g])
    }
}
/// n = 12, i = 6, a < g, b < g, b < i, c < f, c < j, d < e, d < f, e < i, f < h, f < l, g < j, g < l, j < k
fn select_1268([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_1222([c, d, g, e, j, f, h, i, k, l])
    } else {
        select_1267([a, b, e, c, f, h, i, g, j])
    }
}
/// n = 8, i = 3, a < h, b < d, b < g, c < e, c < g, e < f, e < h
fn select_1269([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < a {
        select_51([e, b, d, f, g, a])
    } else {
        select_186([a, b, c, d, e, f, h, g])
    }
}
/// n = 10, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < g, d < i, e < f, e < i, h < j
fn select_1270([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_985([d, a, c, e, g, h, j, i])
    } else {
        select_321([a, b, c, d, e, f, g, i])
    }
}
/// n = 11, i = 5, a < f, a < g, b < e, b < f, c < d, c < g, d < j, e < j, e < k, f < i, g < h, g < k
fn select_1271([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_1269([e, f, c, i, g, h, d, k])
    } else {
        select_1270([d, b, a, e, g, h, j, f, k, i])
    }
}
/// n = 12, i = 6, a < i, a < l, b < f, c < g, c < j, d < e, d < g, e < i, f < j, f < l, g < h, g < l, j < k
fn select_1272([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < f {
        select_1268([b, a, c, d, e, g, f, h, i, j, k, l])
    } else {
        select_1271([c, f, d, e, a, j, g, h, k, i, l])
    }
}
/// n = 14, i = 6, a < i, a < k, b < e, b < g, c < g, c < j, c < n, d < f, e < i, f < j, f < k, g < h, j < l, k < m, k < n
fn select_1273([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < g {
        select_1266([b, c, a, f, e, j, k, m, l, i, n, g])
    } else {
        select_1272([a, d, c, b, e, f, g, h, i, j, l, k])
    }
}
/// n = 9, i = 3, a < i, b < d, b < f, b < h, c < e, c < f, e < h, e < i, f < g
fn select_1274([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_58([a, d, c, f, g, e])
    } else {
        select_641([a, e, b, f, g, d, h, i])
    }
}
/// n = 8, i = 2, a < g, a < h, b < f, b < g, b < h, c < d, c < f, d < e
fn select_1275([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_28([b, d, a, e, f])
    } else {
        select_640([c, a, b, g, h, f, d])
    }
}
/// n = 11, i = 3, a < e, a < j, a < k, b < c, b < d, b < i, c < j, c < k, d < g, e < f, e < i, g < h
fn select_1276([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_1275([c, a, d, g, h, e, j, k])
    } else {
        select_243([e, b, f, c, j, k, d, i])
    }
}
/// n = 11, i = 4, a < e, a < f, b < f, b < h, b < k, c < d, c < e, c < j, d < h, e < k, f < g, f < j, h < i
fn select_1277([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_29([f, c, g, e, k, d, j])
    } else {
        select_80([d, a, b, e, h, i, k, f])
    }
}
/// n = 12, i = 4, a < e, a < g, a < l, b < f, b < g, b < i, c < d, c < e, c < f, c < k, d < i, f < l, g < h, g < k, i < j
fn select_1278([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_1276([a, c, f, d, g, h, i, j, k, e, l])
    } else {
        select_1277([a, b, c, d, e, g, h, i, j, k, f])
    }
}
/// n = 13, i = 4, a < g, a < j, b < f, b < g, b < i, c < d, c < e, c < f, c < m, d < j, e < i, g < h, g < m, i < k, j < l
fn select_1279([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < a {
        select_1274([d, b, e, f, a, i, k, g, j])
    } else {
        select_1278([b, a, c, d, f, e, g, h, j, l, m, i])
    }
}
/// n = 10, i = 3, a < d, a < j, b < c, b < h, b < i, c < f, c < j, d < e, d < h, d < i, f < g
fn select_1280([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_28([f, a, d, j, g])
    } else {
        select_34([e, b, c, h, i])
    }
}
/// n = 10, i = 3, a < d, a < i, b < c, b < e, b < j, d < f, d < j, e < g, e < i, g < h
fn select_1281([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_461([c, a, e, g, h, i, d])
    } else {
        select_1280([a, b, e, d, f, g, h, c, j, i])
    }
}
/// n = 11, i = 4, a < g, a < i, a < k, b < d, b < k, c < e, c < g, c < h, d < f, d < h, e < i, e < k, i < j
fn select_1282([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_678([f, a, c, e, h, g])
    } else {
        select_1281([b, c, g, d, e, f, i, j, k, h])
    }
}
/// n = 7, i = 2, a < d, a < e, a < f, a < g, b < c, b < e, b < f, b < g
fn select_1283([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_34([c, a, e, f, g])
    } else {
        select_34([d, b, e, f, g])
    }
}
/// n = 10, i = 3, a < d, a < h, b < c, b < h, b < i, b < j, c < e, c < f, d < g, d < i, d < j
fn select_1284([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_105([a, c, e, f, h, d])
    } else {
        select_1283([b, d, g, h, c, i, j])
    }
}
/// n = 10, i = 5, a < h, b < e, b < j, c < f, c < h, d < f, d < h, e < g, h < i, h < j
fn select_1285([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_856([a, c, d, e, g, f, h])
    } else {
        select_409([f, b, h, i, e, j])
    }
}
/// n = 12, i = 5, a < g, a < j, b < h, c < e, c < i, d < g, d < h, e < f, e < k, e < l, h < i, h < j, h < k, h < l
fn select_1286([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_1284([c, h, a, e, g, j, f, i, k, l])
    } else {
        select_1285([b, c, a, d, e, g, f, h, j, i])
    }
}
/// n = 15, i = 6, a < f, a < k, a < m, b < e, b < f, c < g, d < g, d < h, e < h, e < l, f < i, f < o, g < j, g < k, g < l, g < o, j < m, m < n
fn select_1287([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < f {
        select_1282([a, b, g, e, j, h, k, l, m, n, f])
    } else {
        select_1286([e, c, a, d, f, i, h, g, k, l, j, o])
    }
}
/// n = 15, i = 6, a < e, a < f, b < f, b < l, b < m, c < g, d < g, d < i, e < i, f < j, f < o, g < h, g < k, g < l, g < o, k < m, m < n
fn select_1288([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < e {
        select_1279([a, b, g, h, k, l, f, j, m, e, n, i, o])
    } else {
        select_1287([b, a, c, d, e, f, g, i, j, k, l, h, m, n, o])
    }
}
/// n = 15, i = 6, a < h, a < k, b < e, b < g, c < g, c < l, c < n, d < f, d < h, e < k, f < l, g < j, g < o, h < i, h < n, h < o, l < m
fn select_1289([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < h {
        select_1273([a, b, c, d, e, f, g, j, k, l, h, m, i, n])
    } else {
        select_1288([b, c, d, a, e, g, h, i, k, j, f, n, l, m, o])
    }
}
/// n = 15, i = 6, a < e, a < g, b < h, b < l, c < g, c < m, c < o, d < f, d < h, e < l, f < m, g < k, h < i, h < j, h < o, m < n
fn select_1290([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < g {
        select_1259([f, a, c, b, e, h, j, i, m, o, l, n, g])
    } else {
        select_1289([b, a, c, d, e, f, g, h, j, k, l, m, n, o, i])
    }
}
/// n = 8, i = 3, a < e, a < f, a < h, b < c, b < d, b < g, g < h
fn select_1291([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_316([d, c, g, a, f, h, e])
    } else {
        select_982([a, b, d, f, g, e, c, h])
    }
}
/// n = 10, i = 4, a < d, a < f, b < c, b < f, b < j, c < h, d < e, d < g, g < j, h < i
fn select_1292([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < c {
        select_409([e, g, b, f, c, j])
    } else {
        select_98([c, a, f, h, d, e, i, g])
    }
}
/// n = 11, i = 4, a < c, a < g, a < k, b < d, b < g, c < i, d < e, d < f, d < h, h < k, i < j
fn select_1293([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < i {
        select_1291([a, d, e, f, c, g, h, k])
    } else {
        select_1292([b, a, c, d, f, g, h, i, j, k])
    }
}
/// n = 7, i = 3, b < f, b < g, c < f, c < g, d < e, d < f, d < g
fn select_1294([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_163([a, b, d, e, c, f, g])
    } else {
        select_163([a, c, d, e, b, f, g])
    }
}
/// n = 9, i = 4, a < g, a < i, b < f, b < g, b < i, c < d, c < e, c < f, e < i, g < h
fn select_1295([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_479([a, c, d, f, g, h, e])
    } else {
        select_1294([d, a, e, b, f, g, i])
    }
}
/// n = 9, i = 4, a < d, b < c, b < i, c < e, c < f, d < g, d < i, g < h
fn select_1296([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_4([b, g, h, i, c])
    } else {
        select_479([a, c, e, f, d, i, g])
    }
}
/// n = 10, i = 4, a < d, b < c, b < i, c < f, c < g, c < j, d < e, d < h, d < j, h < i
fn select_1297([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_479([a, c, f, g, d, e, j])
    } else {
        select_598([b, d, e, h, c, g, j, i])
    }
}
/// n = 10, i = 4, a < e, a < i, b < c, b < d, c < i, d < f, d < j, e < g, e < h, e < j
fn select_1298([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_1296([b, a, e, c, g, h, d, f, i])
    } else {
        select_1297([b, a, e, d, f, g, h, c, i, j])
    }
}
/// n = 11, i = 5, a < g, a < h, b < g, b < h, b < i, c < d, c < k, d < e, d < f, d < i, g < k, h < j
fn select_1299([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_1295([a, b, d, f, e, i, g, k, h])
    } else {
        select_1298([c, a, g, h, d, j, f, i, k, e])
    }
}
/// n = 12, i = 5, a < h, b < e, b < j, c < d, c < h, c < i, d < j, d < l, e < f, e < g, e < i, g < l, h < k
fn select_1300([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < a {
        select_1293([d, b, a, e, f, i, j, g, h, k, l])
    } else {
        select_1299([a, c, b, e, f, g, d, h, i, k, j])
    }
}
/// n = 7, i = 2, a < g, b < f, c < d, c < e
fn select_1301([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_64([b, a, g, f, c, d, e])
    } else {
        select_636([b, c, d, e, f, a, g])
    }
}
/// n = 8, i = 3, a < h, b < d, b < g, c < e, c < g, c < h, e < f
fn select_1302([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_1048([a, c, e, b, d, f, h, g])
    } else {
        select_284([c, b, d, a, e, f, g, h])
    }
}
/// n = 8, i = 3, b < e, b < h, c < d, c < f, c < h, f < g
fn select_1303([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_1301([d, f, b, e, h, g, a])
    } else {
        select_1302([a, b, c, e, f, g, h, d])
    }
}
/// n = 7, i = 3, a < g, b < e, c < d, c < f, d < g, e < f
fn select_1304([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_52([b, a, c, d, e, g])
    } else {
        select_4([c, e, a, d, f])
    }
}
/// n = 9, i = 3, a < e, a < i, b < e, b < f, b < g, c < d, d < f, d < g, d < i, e < h
fn select_1305([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_3([c, e, h])
    } else {
        select_154([a, b, d, f, g, i])
    }
}
/// n = 10, i = 4, a < f, a < i, b < d, b < f, b < h, c < e, c < j, e < g, f < j, g < h, g < i
fn select_1306([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < g {
        select_1304([d, a, c, e, f, j, g])
    } else {
        select_1305([a, b, e, g, f, d, h, j, i])
    }
}
/// n = 10, i = 4, a < f, a < h, b < e, b < f, c < d, c < e, d < h, d < j, e < j, f < g, h < i
fn select_1307([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_4([c, f, g, e, d])
    } else {
        select_80([a, b, d, e, h, i, j, f])
    }
}
/// n = 11, i = 4, a < f, a < i, b < e, b < f, b < g, c < d, c < e, d < g, d < i, d < j, e < j, f < h, i < k
fn select_1308([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_140([a, b, d, e, g, i, j, k])
    } else {
        select_1307([a, b, c, d, e, f, h, i, k, j])
    }
}
/// n = 9, i = 4, a < f, a < i, b < f, b < h, b < i, c < d, d < e, d < i, e < h, f < g
fn select_1309([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_11([a, e, b, h, i])
    } else {
        select_25([f, c, g, d, i])
    }
}
/// n = 9, i = 4, a < f, b < f, b < g, b < i, c < d, d < e, d < g, e < i, f < h
fn select_1310([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_1309([a, b, c, d, e, f, h, i, g])
    } else {
        select_20([e, g, a, i])
    }
}
/// n = 11, i = 5, a < g, a < h, b < e, c < d, c < k, d < h, d < j, e < f, e < g, f < j, g < k, h < i
fn select_1311([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_642([f, a, c, g, d, j, k])
    } else {
        select_667([a, d, b, e, h, i, f, g, j, k])
    }
}
/// n = 12, i = 5, a < g, a < h, b < e, c < d, c < l, d < h, d < i, d < k, e < f, e < g, e < i, f < k, g < l, h < j
fn select_1312([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_1310([a, d, b, e, f, h, i, j, k])
    } else {
        select_1311([a, b, c, d, e, f, g, h, j, k, l])
    }
}
/// n = 12, i = 5, a < h, a < i, b < d, b < l, c < e, c < g, d < g, d < i, d < j, e < f, e < h, e < j, h < l, i < k
fn select_1313([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_1308([a, d, c, e, g, i, j, k, h, f, l])
    } else {
        select_1312([a, c, b, d, e, f, h, i, j, k, g, l])
    }
}
/// n = 12, i = 5, a < h, a < i, b < e, b < g, c < d, c < g, c < l, d < i, d < j, e < f, e < h, e < j, h < l, i < k
fn select_1314([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < d {
        select_1306([a, e, c, f, g, h, d, j, i, l])
    } else {
        select_1313([a, c, b, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 12, i = 5, a < i, b < e, b < g, c < d, c < g, c < j, d < i, d < k, e < f, e < h, e < k, h < j, i < l
fn select_1315([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_1303([f, h, c, g, a, d, k, j])
    } else {
        select_1314([a, b, c, d, e, f, g, h, i, k, l, j])
    }
}
/// n = 12, i = 5, a < i, b < g, b < l, c < i, c < j, d < e, d < g, e < f, e < h, e < j, h < l, i < k
fn select_1316([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < b {
        select_1300([a, d, c, b, e, f, h, i, j, g, k, l])
    } else {
        select_1315([a, d, b, c, e, f, g, h, i, l, j, k])
    }
}
/// n = 6, i = 3, b < f, c < f, d < e
fn select_1317([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_124([c, d, a, e, b, f])
    } else {
        select_16([c, d, b, a, e, f])
    }
}
/// n = 11, i = 5, a < g, a < h, b < f, c < e, d < f, d < j, e < g, e < k, f < h, f < k, g < j, h < i
fn select_1318([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_87([b, d, g, j])
    } else {
        select_642([a, c, f, e, h, i, k])
    }
}
/// n = 11, i = 5, a < i, b < g, c < e, d < g, d < h, e < f, e < j, f < h, g < i, g < j, i < k
fn select_1319([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_1318([a, b, c, d, e, g, f, i, k, h, j])
    } else {
        select_406([b, f, d, a, h, g, j])
    }
}
/// n = 12, i = 5, a < h, a < j, b < g, c < g, c < i, d < e, e < f, e < h, e < k, f < i, g < j, g < k, j < l
fn select_1320([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_1317([b, c, f, a, h, i])
    } else {
        select_1319([a, b, d, c, e, f, g, i, j, k, l])
    }
}
/// n = 7, i = 2, a < f, a < g, b < d, b < e, c < e, c < f, c < g
fn select_1321([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_194([a, d, c, e])
    } else {
        select_28([b, c, f, g, d])
    }
}
/// n = 9, i = 3, a < f, a < h, a < i, b < d, b < g, c < e, c < f, d < i, e < g, e < h
fn select_1322([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_1321([b, a, e, f, h, d, g])
    } else {
        select_457([a, c, b, d, f, e, i])
    }
}
/// n = 8, i = 3, a < g, a < h, b < d, b < f, c < e, e < f, e < g, e < h
fn select_1323([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_44([b, e, d, a, g, h, f])
    } else {
        select_514([b, a, c, d, e, g, h])
    }
}
/// n = 11, i = 5, a < g, a < h, a < k, b < e, b < k, c < f, d < f, e < g, e < j, f < h, f < j, f < k, h < i
fn select_1324([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_26([a, c, d, g, f, j])
    } else {
        select_209([a, b, e, h, j, i, k])
    }
}
/// n = 13, i = 5, a < f, a < m, b < g, b < h, b < i, c < e, c < h, d < f, d < h, e < g, e < j, e < l, f < i, f < l, h < j, h < m, i < k
fn select_1325([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < f {
        select_1323([a, e, d, g, h, j, f, m])
    } else {
        select_1324([b, c, a, d, e, f, g, i, k, l, h])
    }
}
/// n = 13, i = 5, a < h, a < j, a < m, b < g, b < m, c < e, c < f, d < f, d < g, e < h, e < i, e < k, f < i, f < m, g < j, g < k, j < l
fn select_1326([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_1322([e, b, f, g, a, i, m, h, k])
    } else {
        select_1325([b, a, c, d, e, g, h, f, j, i, l, k, m])
    }
}
/// n = 13, i = 5, a < i, a < j, a < m, b < h, b < m, c < g, c < h, d < e, d < g, e < f, e < i, e < k, g < m, h < j, h < k, j < l
fn select_1327([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_1320([a, b, c, d, e, f, h, i, g, j, k, l])
    } else {
        select_1326([a, b, d, c, e, g, h, i, f, j, k, l, m])
    }
}
/// n = 8, i = 4, a < e, b < f, c < f, d < e, d < g, d < h, f < g, f < h
fn select_1328([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_162([b, c, d, e, f])
    } else {
        select_19([a, d, g, h])
    }
}
/// n = 8, i = 4, a < f, b < g, c < e, c < g, d < e, d < f, d < h, g < h
fn select_1329([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_1328([a, b, c, d, f, g, e, h])
    } else {
        select_124([b, a, c, f, e, g])
    }
}
/// n = 11, i = 5, a < h, a < i, b < f, c < e, d < f, d < g, e < g, e < h, e < j, f < i, f < j, i < k
fn select_1330([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < i {
        select_1329([a, b, d, e, g, h, f, j])
    } else {
        select_3([c, i, k])
    }
}
/// n = 12, i = 5, a < h, a < i, a < k, b < g, c < g, c < k, d < e, e < f, e < h, e < j, f < k, g < i, g < j, i < l
fn select_1331([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_108([c, a, f, h, b, g, k])
    } else {
        select_1330([a, c, d, b, e, g, f, h, i, j, l])
    }
}
/// n = 8, i = 2, a < e, b < c, b < d, c < g, d < f, e < h
fn select_1332([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < a {
        select_28([d, c, a, g, f])
    } else {
        select_209([a, b, d, e, f, h, c])
    }
}
/// n = 11, i = 4, a < h, a < k, b < d, b < g, c < e, c < k, d < f, d < k, e < g, e < h, g < i, h < j
fn select_1333([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_58([f, a, c, e, g, k])
    } else {
        select_1332([a, b, d, g, h, i, f, j])
    }
}
/// n = 9, i = 3, a < e, b < d, b < f, c < e, c < f, e < i, f < g, f < h
fn select_1334([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_34([d, f, a, g, h])
    } else {
        select_126([b, a, c, d, e, i, f])
    }
}
/// n = 12, i = 5, a < i, a < l, b < f, b < h, c < f, c < i, d < e, d < l, e < h, e < i, e < j, f < g, f < j, f < l, i < k
fn select_1335([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_48([g, a, d, e, h, j, l])
    } else {
        select_1334([a, b, c, h, i, f, g, j, k])
    }
}
/// n = 14, i = 5, a < g, a < n, b < e, b < n, c < f, c < i, c < j, d < f, d < g, e < i, e < l, e < m, f < h, f < l, f < n, g < j, g < m, j < k
fn select_1336([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < e {
        select_1333([b, c, a, f, g, h, j, e, k, m, n])
    } else {
        select_1335([a, c, d, b, e, f, h, i, g, l, m, n])
    }
}
/// n = 14, i = 5, a < h, a < n, b < e, b < n, c < g, c < j, c < k, d < g, d < h, e < f, e < j, e < m, g < i, g < n, h < k, h < m, k < l
fn select_1337([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < g {
        select_1331([c, a, d, b, e, f, h, j, k, m, g, l])
    } else {
        select_1336([a, b, c, d, e, g, h, i, j, k, l, f, m, n])
    }
}
/// n = 14, i = 5, a < h, a < k, a < l, b < g, b < i, c < e, c < g, d < h, d < i, e < f, e < k, e < n, h < j, i < l, i < n, l < m
fn select_1338([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < h {
        select_1327([a, d, b, c, e, f, g, i, k, l, n, m, h])
    } else {
        select_1337([b, c, a, d, e, f, h, i, j, k, l, m, n, g])
    }
}
/// n = 8, i = 2, a < d, a < e, a < h, b < c, b < f, b < h, f < g
fn select_1339([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_105([b, a, d, e, c, h])
    } else {
        select_45([a, b, c, e, f, g, h])
    }
}
/// n = 8, i = 3, a < d, a < f, b < e, b < g, c < e, c < f, c < g, g < h
fn select_1340([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_15([b, c, a, d, f, e])
    } else {
        select_51([a, b, e, f, g, h])
    }
}
/// n = 9, i = 3, a < h, b < d, b < g, b < i, c < e, c < g, c < h, e < f, e < i
fn select_1341([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_1339([b, e, f, d, g, a, h, i])
    } else {
        select_1340([b, a, c, d, h, g, e, f])
    }
}
/// n = 10, i = 4, a < d, a < f, b < c, b < e, b < i, e < h, e < j, f < g, f < i, f < j
fn select_1342([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < c {
        select_1341([d, f, b, g, e, h, i, c, j])
    } else {
        select_283([c, e, a, h, f, g, i, d, j])
    }
}
/// n = 10, i = 4, a < d, a < j, b < f, b < j, c < e, c < h, e < g, e < j, f < h, h < i
fn select_1343([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_869([a, b, e, g, d, f, h, j])
    } else {
        select_134([a, c, f, d, e, h, i, g])
    }
}
/// n = 11, i = 3, a < e, b < d, b < f, c < e, c < f, e < k, f < g, f < h, f < i, f < j
fn select_1344([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_547([d, c, a, f, g, h, i, j])
    } else {
        select_550([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 13, i = 5, a < e, a < g, b < g, b < j, c < d, c < e, d < f, d < l, d < m, f < j, g < h, g < i, g < l, g < m, j < k
fn select_1345([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < d {
        select_221([h, i, c, e, d, f, l, m])
    } else {
        select_1344([f, a, b, e, j, g, h, i, l, m, k])
    }
}
/// n = 15, i = 6, a < f, a < i, b < h, b < i, c < g, c < i, c < m, d < e, d < f, e < g, e < h, e < o, g < j, h < m, i < k, i < l, i < o, m < n
fn select_1346([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < i {
        select_1343([a, b, c, f, g, h, j, m, n, i])
    } else {
        select_1345([a, c, d, e, f, h, i, k, l, m, n, g, o])
    }
}
/// n = 15, i = 6, a < i, a < l, a < n, b < f, b < g, c < g, c < j, d < e, d < f, e < i, e < j, e < m, g < h, g < l, g < m, i < k, j < n, n < o
fn select_1347([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < a {
        select_1342([d, g, h, f, a, e, j, l, m, i])
    } else {
        select_1346([b, c, a, d, e, f, i, j, g, k, h, l, n, o, m])
    }
}
/// n = 15, i = 6, a < j, a < k, a < n, b < g, b < h, c < f, c < g, d < e, d < h, e < j, f < k, f < o, h < i, h < n, h < o, j < l, k < m
fn select_1348([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < f {
        select_1338([a, c, b, e, h, i, g, j, f, l, n, k, m, o])
    } else {
        select_1347([a, b, d, c, f, g, h, i, k, e, m, n, o, j, l])
    }
}
/// n = 15, i = 6, a < f, a < g, b < k, b < l, b < n, c < g, c < h, d < e, d < h, e < k, f < l, h < i, h < j, h < n, k < m, l < o
fn select_1349([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_1316([e, a, b, c, h, j, g, i, k, n, m, f])
    } else {
        select_1348([b, c, a, d, e, f, g, h, j, k, l, m, o, n, i])
    }
}
/// n = 15, i = 6, a < g, b < h, c < h, c < m, c < o, d < g, d < i, e < f, e < i, f < m, h < l, i < j, i < k, i < o, m < n
fn select_1350([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if b < a {
        select_1290([b, d, c, e, a, f, h, i, j, k, l, g, m, n, o])
    } else {
        select_1349([a, c, d, e, f, b, g, i, j, k, m, h, n, o, l])
    }
}
/// n = 15, i = 6, b < h, c < h, c < m, c < o, d < f, d < i, e < g, e < i, g < m, h < l, i < j, i < k, i < o, m < n
fn select_1351([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < a {
        select_1230([b, c, d, e, f, g, a, h, i, j, k, l, m, n, o])
    } else {
        select_1350([a, b, c, d, e, g, f, h, i, j, k, l, m, n, o])
    }
}
/// n = 8, i = 2, a < c, a < f, a < g, a < h, b < d, b < f, b < g, b < h, d < e
fn select_1352([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_34([c, b, f, g, h])
    } else {
        select_67([d, a, f, g, h, e])
    }
}
/// n = 8, i = 2, b < f, b < g, b < h, c < d, c < f, c < g, c < h, d < e
fn select_1353([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_25([a, c, b, d, e])
    } else {
        select_1352([b, c, a, d, e, f, g, h])
    }
}
/// n = 10, i = 4, a < f, b < h, b < i, b < j, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_1354([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_1206([b, a, c, d, e, f, g])
    } else {
        select_1353([e, b, g, a, f, h, i, j])
    }
}
/// n = 10, i = 6, a < h, a < j, b < f, c < g, d < e, d < j, e < i, f < i, f < j, g < h, g < j
fn select_1355([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_575([a, g, b, d, e, f, h, j])
    } else {
        select_158([a, e, f, c, g, i, j])
    }
}
/// n = 10, i = 6, a < g, b < e, b < j, c < h, d < f, d < j, f < i, g < j, h < i, h < j
fn select_1356([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_1355([b, c, a, d, f, h, g, e, i, j])
    } else {
        select_827([a, c, d, b, f, h, e, g, i])
    }
}
/// n = 12, i = 6, a < f, a < k, a < l, b < e, b < j, c < h, d < g, d < j, g < i, h < i, h < j, j < k, j < l
fn select_1357([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < f {
        select_1354([g, a, h, b, e, i, j, f, k, l])
    } else {
        select_1356([a, b, c, d, e, g, f, h, i, j])
    }
}
/// n = 9, i = 3, a < e, b < d, b < h, b < i, c < f, c < h, f < g, f < i
fn select_1358([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_132([a, c, b, e, d, h, f])
    } else {
        select_245([a, b, c, e, f, g, h, d, i])
    }
}
/// n = 12, i = 6, a < f, a < l, b < e, b < k, b < l, c < h, d < g, d < i, g < j, h < i, h < j, i < k, i < l
fn select_1359([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < i {
        select_1357([b, a, c, d, f, e, g, h, j, i, k, l])
    } else {
        select_1358([g, b, i, e, j, a, f, k, l])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, b < i, c < f, d < e, d < i, e < h, f < g, f < i
fn select_1360([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_108([a, b, d, g, e, h, i])
    } else {
        select_124([b, a, c, h, f, g])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, c < e, c < i, d < f, d < i, f < h, g < i
fn select_1361([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_1360([a, c, b, d, f, g, e, h, i])
    } else {
        select_825([a, b, d, c, f, e, g, h])
    }
}
/// n = 11, i = 5, a < f, a < j, a < k, b < h, b < i, c < e, c < i, d < g, d < i, g < h, i < j, i < k
fn select_1362([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < f {
        select_1354([g, a, b, c, e, h, i, f, j, k])
    } else {
        select_1361([b, a, c, d, e, g, f, h, i])
    }
}
/// n = 11, i = 5, a < f, a < k, b < e, b < j, b < k, c < h, c < i, d < g, d < h, g < i, h < j, h < k
fn select_1363([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_1358([g, b, h, e, i, a, f, j, k])
    } else {
        select_1362([b, c, a, d, f, e, g, i, h, j, k])
    }
}
/// n = 12, i = 6, a < g, a < l, b < f, b < k, b < l, c < e, c < j, d < h, h < i, h < j, j < k, j < l
fn select_1364([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_1359([a, b, d, c, f, g, e, h, j, i, k, l])
    } else {
        select_1363([a, b, c, h, f, g, i, j, e, k, l])
    }
}
/// n = 9, i = 3, b < e, b < h, b < i, c < d, c < f, c < h, f < g, f < i
fn select_1365([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_1045([d, f, b, e, h, g, a, i])
    } else {
        select_1341([a, b, c, e, f, g, h, d, i])
    }
}
/// n = 8, i = 4, a < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_1366([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_368([a, e, b, c, g, h])
    } else {
        select_637([b, a, c, d, e, f])
    }
}
/// n = 10, i = 4, a < e, b < f, b < i, b < j, c < g, d < f, d < g, g < h, g < i, g < j
fn select_1367([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_244([b, g, h, f, a, e, i, j])
    } else {
        select_1366([b, c, d, a, e, f, g, h])
    }
}
/// n = 10, i = 4, a < e, b < h, b < i, b < j, c < f, d < f, f < g, f < h, f < i, f < j
fn select_1368([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_173([g, a, b, h, i, j, e])
    } else {
        select_368([b, e, c, d, f, g])
    }
}
/// n = 11, i = 5, a < g, b < f, b < j, b < k, c < h, d < g, d < h, e < f, e < h, h < i, h < j, h < k
fn select_1369([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_1368([a, b, c, e, g, h, i, f, j, k])
    } else {
        select_925([c, a, b, e, d, g, f, h])
    }
}
/// n = 11, i = 5, b < g, b < j, b < k, c < h, d < g, d < h, e < f, e < h, h < i, h < j, h < k
fn select_1370([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_1367([f, b, c, d, a, g, h, i, j, k])
    } else {
        select_1369([a, b, c, e, d, g, f, h, i, j, k])
    }
}
/// n = 11, i = 5, b < f, b < k, c < g, c < j, c < k, d < h, e < g, e < h, h < i, h < j, h < k
fn select_1371([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < b {
        select_1365([a, c, h, i, g, b, f, j, k])
    } else {
        select_1370([a, c, d, e, b, f, g, h, i, j, k])
    }
}
/// n = 9, i = 4, a < e, b < g, b < i, c < f, c < g, d < f, d < g, f < i, g < h
fn select_1372([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_58([a, f, b, g, h, i])
    } else {
        select_12([b, a, c, d, e, f, i])
    }
}
/// n = 10, i = 5, a < e, b < h, b < i, c < g, c < h, d < f, d < i, f < h, g < i, i < j
fn select_1373([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_1372([a, b, d, g, e, f, i, j, h])
    } else {
        select_241([a, b, c, f, e, g, h, i])
    }
}
/// n = 9, i = 4, a < e, b < h, c < g, c < h, d < f, d < h, f < g, h < i
fn select_1374([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_1372([a, c, b, d, e, f, h, i, g])
    } else {
        select_132([a, c, f, e, b, g, h])
    }
}
/// n = 10, i = 5, a < f, b < h, b < i, c < e, c < i, d < g, d < h, g < i, i < j
fn select_1375([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_1373([a, b, d, c, f, e, g, h, i, j])
    } else {
        select_1374([a, g, c, b, f, h, e, i, j])
    }
}
/// n = 9, i = 5, a < e, b < h, b < i, c < g, d < f, d < h, f < i, g < h, g < i
fn select_1376([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_292([a, c, d, e, f, g, h, i])
    } else {
        select_373([e, b, c, d, f, g, h])
    }
}
/// n = 9, i = 5, a < e, b < h, b < i, c < g, d < f, d < h, f < i, g < i
fn select_1377([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < h {
        select_1376([a, b, c, d, e, f, g, h, i])
    } else {
        select_17([f, c, a, b, e, h, g])
    }
}
/// n = 10, i = 5, b < f, c < i, d < h, d < i, e < g, e < h, g < i, i < j
fn select_1378([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_1375([b, d, c, e, a, f, g, h, i, j])
    } else {
        select_1377([b, d, a, e, f, g, c, h, i])
    }
}
/// n = 11, i = 5, b < g, b < k, c < i, d < f, d < h, d < k, e < h, e < i, i < j, i < k
fn select_1379([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < f {
        select_1371([a, b, d, c, e, g, h, i, j, f, k])
    } else {
        select_1378([a, b, c, e, d, g, f, h, i, j])
    }
}
/// n = 12, i = 6, a < g, a < l, b < i, c < f, c < k, c < l, d < e, d < h, h < k, h < l, i < j, i < l
fn select_1380([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_1364([a, c, d, b, e, f, g, i, j, h, k, l])
    } else {
        select_1379([e, a, b, c, h, f, g, k, i, j, l])
    }
}
/// n = 8, i = 2, b < g, b < h, c < d, c < e, c < f, c < g, c < h
fn select_1381([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_339([e, f, d, b, g, h, a])
    } else {
        select_910([a, b, c, e, f, g, h, d])
    }
}
/// n = 10, i = 2, b < j, c < d, c < e, c < f, c < g, c < j, g < h, g < i
fn select_1382([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_1135([a, d, e, f, g, b, h, i])
    } else {
        select_1381([a, b, c, d, e, f, g, j])
    }
}
/// n = 9, i = 3, c < e, d < e, e < f, e < g, e < h, e < i
fn select_1383([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_582([a, c, d, b, e, f, g, h, i])
    } else {
        select_582([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 10, i = 4, b < e, b < j, c < f, d < f, f < g, f < h, f < i, f < j
fn select_1384([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_349([a, g, h, i, b, e, j])
    } else {
        select_1383([a, e, c, d, f, g, h, i, j])
    }
}
/// n = 11, i = 4, b < k, c < j, d < f, e < f, f < g, f < h, f < i, f < j, f < k
fn select_1385([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_1384([a, b, d, e, c, f, g, h, i, k])
    } else {
        select_1384([a, c, d, e, b, f, g, h, i, j])
    }
}
/// n = 12, i = 4, c < l, d < f, e < f, f < g, f < h, f < i, f < j, f < l, j < k
fn select_1386([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < a {
        select_1382([b, c, f, g, h, i, j, a, k, l])
    } else {
        select_1385([b, a, c, d, e, f, g, h, i, l, j])
    }
}
/// n = 8, i = 5, a < h, b < f, c < e, d < g, g < h
fn select_1387([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_825([a, b, d, c, g, e, f, h])
    } else {
        select_825([a, c, d, b, g, f, e, h])
    }
}
/// n = 9, i = 3, b < i, c < e, c < i, d < e, d < i, e < f, e < g, e < h
fn select_1388([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_68([b, e, a, f, g, h, i])
    } else {
        select_153([a, b, c, d, i, e])
    }
}
/// n = 10, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, g < h, g < i, g < j
fn select_1389([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_71([a, f, d, g, h, i, j, e])
    } else {
        select_1388([a, e, b, c, g, h, i, j, f])
    }
}
/// n = 12, i = 5, a < f, b < h, b < i, c < e, c < i, d < g, g < h, g < i, i < j, i < k, i < l
fn select_1390([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_1387([b, a, c, d, e, f, g, h])
    } else {
        select_1389([a, b, g, c, e, h, i, j, k, l])
    }
}
/// n = 12, i = 5, c < h, c < i, d < f, d < i, e < g, g < h, g < i, i < j, i < k, i < l
fn select_1391([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_1390([a, c, d, e, f, b, g, h, i, j, k, l])
    } else {
        select_1390([b, c, d, e, f, a, g, h, i, j, k, l])
    }
}
/// n = 13, i = 5, b < m, c < h, c < i, d < f, d < i, e < g, e < i, g < h, i < j, i < k, i < l, i < m
fn select_1392([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_1386([a, f, b, c, e, i, j, k, l, g, h, m])
    } else {
        select_1391([a, b, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 9, i = 5, b < i, c < g, d < g, e < f, e < i, f < h, g < h, g < i
fn select_1393([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_214([b, e, g, a, f, h, i])
    } else {
        select_1097([b, a, c, d, e, f, g])
    }
}
/// n = 12, i = 5, c < g, d < g, e < f, e < h, f < l, g < h, g < l, h < i, h < j, h < k
fn select_1394([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_72([b, f, g, h, a, i, j, k, l])
    } else {
        select_1393([b, a, c, d, e, f, g, l, h])
    }
}
/// n = 12, i = 5, c < f, c < i, d < g, e < g, g < h, g < i, i < j, i < k, i < l
fn select_1395([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_1056([a, b, c, g, h, i, j, k, l, f])
    } else {
        select_1394([a, b, d, e, c, f, g, i, j, k, l, h])
    }
}
/// n = 13, i = 5, b < m, c < f, c < i, d < g, d < i, e < g, e < i, g < h, i < j, i < k, i < l, i < m
fn select_1396([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_1386([a, f, b, d, e, i, j, k, l, g, h, m])
    } else {
        select_1395([a, b, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 13, i = 5, b < m, c < h, c < i, d < g, d < i, e < f, e < h, e < i, i < j, i < k, i < l, i < m
fn select_1397([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < h {
        select_1392([a, b, c, d, e, g, f, h, i, j, k, l, m])
    } else {
        select_1396([a, b, d, c, e, g, h, f, i, j, k, l, m])
    }
}
/// n = 15, i = 6, a < h, b < g, b < l, c < f, c < k, c < l, d < e, d < i, h < j, i < k, i < l, l < m, l < n, l < o
fn select_1398([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < l {
        select_1380([b, a, c, d, e, f, g, i, h, j, k, l])
    } else {
        select_1397([e, a, i, b, c, f, g, k, l, m, n, o, h])
    }
}
/// n = 8, i = 1, b < h, c < d, c < e, c < f, c < g
fn select_1399([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_68([b, c, d, e, f, g, h])
    } else {
        select_173([a, b, c, e, f, g, h])
    }
}
/// n = 8, i = 3, a < d, a < h, b < f, b < g, b < h, c < e, c < h, e < f, e < g
fn select_1400([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_11([b, d, c, e, h])
    } else {
        select_640([a, b, e, f, g, d, h])
    }
}
/// n = 8, i = 3, a < e, a < h, b < f, b < h, c < d, c < g, c < h, f < g
fn select_1401([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_108([b, a, c, e, d, f, h])
    } else {
        select_1400([a, c, b, e, f, d, g, h])
    }
}
/// n = 11, i = 3, a < e, b < f, b < g, c < d, c < g, c < k, f < k, g < h, g < i, g < j
fn select_1402([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_1399([d, f, g, a, h, i, j, k])
    } else {
        select_1401([a, b, c, d, e, f, k, g])
    }
}
/// n = 15, i = 6, a < h, a < j, a < l, b < e, b < l, c < f, d < f, e < h, f < g, f < j, f < l, g < i, h < k, l < m, l < n, l < o
fn select_1403([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < f {
        select_87([k, c, d, f])
    } else {
        select_1402([g, b, a, j, i, e, l, m, n, o, h])
    }
}
/// n = 15, i = 6, a < e, b < h, b < j, b < l, c < f, d < f, e < h, f < g, f < j, f < l, g < i, h < k, l < m, l < n, l < o
fn select_1404([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if l < a {
        select_1129([j, g, l, m, n, o, i, a, e])
    } else {
        select_1403([b, a, c, d, e, f, g, h, i, j, k, l, m, n, o])
    }
}
/// n = 15, i = 6, a < e, b < i, b < j, b < l, c < f, d < f, e < i, f < g, f < h, f < j, f < l, i < k, l < m, l < n, l < o
fn select_1405([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < h {
        select_1404([a, b, c, d, e, f, g, i, h, j, k, l, m, n, o])
    } else {
        select_1404([a, b, c, d, e, f, h, i, g, j, k, l, m, n, o])
    }
}
/// n = 10, i = 2, d < e, d < f, d < g, d < h, h < i, h < j
fn select_1406([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1137([c, a, d, e, f, g, b, h, i, j])
    } else {
        select_1137([c, b, d, e, f, g, a, h, i, j])
    }
}
/// n = 10, i = 2, a < c, a < i, a < j, b < d, b < i, b < j, d < e, d < f, d < g, d < h
fn select_1407([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_189([a, e, f, g, h])
    } else {
        select_34([c, b, d, i, j])
    }
}
/// n = 10, i = 2, b < d, b < i, b < j, c < e, c < i, c < j, e < f, e < g, e < h
fn select_1408([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_37([a, c, b, d, i, j, e])
    } else {
        select_1407([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 11, i = 3, c < f, c < j, c < k, d < e, d < f, e < j, e < k, f < g, f < h, f < i
fn select_1409([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_1408([b, e, c, a, f, g, h, i, j, k])
    } else {
        select_1130([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 12, i = 4, a < f, a < h, b < f, b < h, c < d, c < e, c < h, c < l, f < g, f < l, h < i, h < j, h < k
fn select_1410([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_1059([a, b, c, d, e, f, g, h, i, j, k, l])
    } else {
        select_1059([a, b, c, e, d, f, g, h, i, j, k, l])
    }
}
/// n = 13, i = 4, a < g, a < i, a < m, b < f, b < g, b < i, c < d, c < e, c < f, c < i, f < m, g < h, i < j, i < k, i < l
fn select_1411([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_1409([d, e, a, b, f, i, j, k, l, g, m])
    } else {
        select_1410([a, b, c, d, e, g, h, i, j, k, l, f])
    }
}
/// n = 8, i = 3, a < f, a < g, a < h, b < d, b < g, b < h, c < e, c < g, c < h, e < f
fn select_1412([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_154([d, a, c, g, h, e])
    } else {
        select_640([e, a, b, g, h, d, f])
    }
}
/// n = 11, i = 3, a < d, a < k, b < f, b < j, b < k, c < e, c < f, c < k, e < j, f < g, f < h, f < i
fn select_1413([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_68([e, f, a, g, h, i, j])
    } else {
        select_1412([b, a, c, d, e, j, f, k])
    }
}
/// n = 11, i = 3, b < k, c < f, c < j, c < k, d < e, d < f, d < k, e < j, f < g, f < h, f < i
fn select_1414([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_1055([a, c, d, b, e, f, g, h, i, j])
    } else {
        select_1413([b, c, d, a, e, f, g, h, i, j, k])
    }
}
/// n = 8, i = 3, a < h, b < e, b < h, c < d, c < f, c < g, c < h, e < f, e < g
fn select_1415([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_35([c, e, a, d, f, g])
    } else {
        select_75([a, b, c, d, e, h])
    }
}
/// n = 9, i = 4, a < i, b < e, b < g, b < h, c < f, d < f, f < g, f < h, f < i
fn select_1416([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_29([b, f, e, a, i, g, h])
    } else {
        select_141([b, a, c, d, e, f, i])
    }
}
/// n = 11, i = 5, a < k, b < g, c < g, d < e, d < k, e < f, e < i, e < j, g < h, g < i, g < j, g < k
fn select_1417([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_1415([a, d, g, h, e, i, j, k])
    } else {
        select_1416([a, e, b, c, f, g, i, j, k])
    }
}
/// n = 14, i = 5, a < n, b < g, c < g, d < e, d < i, d < n, e < f, e < m, g < h, g < i, g < m, g < n, i < j, i < k, i < l
fn select_1418([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < e {
        select_72([h, a, d, i, e, j, k, l, n])
    } else {
        select_1417([a, b, c, d, e, f, g, h, i, m, n])
    }
}
/// n = 15, i = 6, a < f, a < k, a < o, b < g, c < g, d < e, d < f, d < k, e < o, f < j, g < h, g < i, g < k, g < o, k < l, k < m, k < n
fn select_1419([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < f {
        select_1414([i, h, a, d, e, k, l, m, n, o, f])
    } else {
        select_1418([e, b, c, a, f, j, g, i, k, l, m, n, h, o])
    }
}
/// n = 15, i = 6, a < f, a < k, a < l, b < e, b < f, b < l, c < g, d < g, f < j, g < h, g < i, g < k, g < l, l < m, l < n, l < o
fn select_1420([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < e {
        select_1411([b, a, g, h, i, k, f, j, l, m, n, o, e])
    } else {
        select_1419([a, c, d, b, e, f, g, h, i, j, l, m, n, o, k])
    }
}
/// n = 15, i = 6, a < e, a < f, b < f, b < k, b < l, c < g, d < g, f < j, g < h, g < i, g < k, g < l, l < m, l < n, l < o
fn select_1421([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if l < a {
        select_1406([h, i, k, l, m, n, o, a, e, f])
    } else {
        select_1420([b, a, c, d, e, f, g, h, i, j, k, l, m, n, o])
    }
}
/// n = 15, i = 6, b < f, c < f, c < k, c < l, d < g, e < g, f < j, g < h, g < i, g < k, g < l, l < m, l < n, l < o
fn select_1422([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < b {
        select_1405([a, c, d, e, b, g, h, i, f, k, j, l, m, n, o])
    } else {
        select_1421([b, c, d, e, a, f, g, h, i, j, k, l, m, n, o])
    }
}
/// n = 6, i = 2, b < f, c < e, d < e, d < f
fn select_1423([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < d {
        select_88([a, c, b, d, f])
    } else {
        select_58([a, c, d, b, f, e])
    }
}
/// n = 8, i = 2, a < e, b < d, c < f, f < g, f < h
fn select_1424([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_23([c, b, a, e, d, f])
    } else {
        select_69([b, c, d, f, a, g, h])
    }
}
/// n = 9, i = 3, a < f, b < e, c < g, d < f, d < g, g < h, g < i
fn select_1425([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < d {
        select_1423([e, a, c, d, g, f])
    } else {
        select_1424([a, b, c, e, f, g, h, i])
    }
}
/// n = 10, i = 3, a < d, b < g, b < j, c < e, c < g, e < f, e < j, g < h, g < i
fn select_1426([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_44([e, b, f, g, h, i, j])
    } else {
        select_1205([a, b, c, d, e, f, g, i, j])
    }
}
/// n = 10, i = 3, b < e, c < g, c < j, d < f, d < g, f < j, g < h, g < i
fn select_1427([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_1425([a, b, c, d, e, f, g, h, i])
    } else {
        select_1426([b, c, d, e, f, a, g, h, i, j])
    }
}
/// n = 10, i = 3, d < g, d < j, e < f, e < g, f < j, g < h, g < i
fn select_1428([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1427([c, a, d, e, b, f, g, h, i, j])
    } else {
        select_1427([c, b, d, e, a, f, g, h, i, j])
    }
}
/// n = 10, i = 4, a < g, a < j, b < e, c < d, c < j, d < g, d < h, e < f, e < h, e < j, g < i
fn select_1429([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_11([a, f, c, d, j])
    } else {
        select_1305([a, d, b, e, g, f, h, i, j])
    }
}
/// n = 11, i = 5, a < h, a < k, b < e, c < d, c < h, d < i, d < k, e < f, e < g, e < k, h < i, i < j
fn select_1430([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_977([g, f, a, c, d, k, h])
    } else {
        select_1429([d, b, a, h, e, g, i, f, j, k])
    }
}
/// n = 8, i = 3, c < f, d < e, d < f, f < g, f < h
fn select_1431([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_609([a, c, d, e, b, f, g, h])
    } else {
        select_609([b, c, d, e, a, f, g, h])
    }
}
/// n = 9, i = 4, a < d, b < f, b < i, c < f, c < i, d < e, d < i, f < g, g < h
fn select_1432([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_3([a, g, h])
    } else {
        select_75([b, c, d, e, f, i])
    }
}
/// n = 11, i = 4, a < f, b < d, c < f, c < h, d < e, d < h, f < g, g < k, h < i, h < j
fn select_1433([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_34([e, h, a, i, j])
    } else {
        select_1432([b, a, c, d, e, f, g, k, h])
    }
}
/// n = 13, i = 5, a < h, a < j, b < e, c < d, c < j, d < h, e < f, e < g, e < j, h < i, i < k, j < l, j < m
fn select_1434([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < h {
        select_1431([f, g, a, c, d, j, l, m])
    } else {
        select_1433([d, b, a, e, g, h, i, j, l, m, k])
    }
}
/// n = 13, i = 5, a < h, b < i, b < j, c < h, c < j, d < e, e < f, e < g, e < j, h < i, i < m, j < k, j < l
fn select_1435([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_1430([c, d, a, b, e, f, g, h, i, m, j])
    } else {
        select_1434([c, d, b, a, e, f, g, h, i, j, m, k, l])
    }
}
/// n = 8, i = 3, a < e, a < f, b < c, b < d, b < g, b < h, f < g, f < h
fn select_1436([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < c {
        select_176([e, f, b, d, g, h, c])
    } else {
        select_48([d, c, a, f, g, h, e])
    }
}
/// n = 11, i = 4, a < f, a < h, b < f, b < h, c < d, c < e, c < h, c < k, f < g, f < k, h < i, h < j
fn select_1437([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_853([a, b, c, d, e, f, g, h, i, j, k])
    } else {
        select_853([a, b, c, e, d, f, g, h, i, j, k])
    }
}
/// n = 11, i = 4, a < h, a < k, b < f, b < h, c < d, c < e, c < h, c < k, f < g, f < k, h < i, h < j
fn select_1438([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_1436([f, c, d, e, g, a, h, k])
    } else {
        select_1437([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, c < d, c < g, c < i, e < f, e < i, g < h
fn select_1439([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_82([d, b, g, h, e])
    } else {
        select_1096([a, c, e, f, d, g, i])
    }
}
/// n = 11, i = 4, a < f, a < k, b < f, b < h, b < k, c < d, d < e, d < h, d < k, f < g, h < i, h < j
fn select_1440([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_48([e, a, b, h, i, j, k])
    } else {
        select_449([a, b, c, d, e, f, g, k])
    }
}
/// n = 12, i = 4, a < d, a < g, a < l, b < g, b < i, b < l, c < e, e < f, e < i, e < l, g < h, i < j, i < k
fn select_1441([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < i {
        select_1439([b, c, a, d, e, f, g, h, l])
    } else {
        select_1440([a, b, c, e, f, g, h, i, j, k, l])
    }
}
/// n = 14, i = 5, a < i, a < k, a < n, b < e, c < d, c < k, d < f, d < i, d < n, e < g, e < h, e < k, e < n, i < j, k < l, k < m
fn select_1442([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < i {
        select_1438([a, c, e, g, h, d, f, k, l, m, n])
    } else {
        select_1441([d, a, b, f, e, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 5, b < i, b < k, b < n, c < f, d < e, d < k, e < i, e < n, f < g, f < h, f < k, f < n, i < j, k < l, k < m
fn select_1443([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < e {
        select_1435([a, b, d, c, f, g, h, e, i, k, l, m, j])
    } else {
        select_1442([b, c, d, e, f, a, g, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 5, b < i, b < j, b < l, c < e, c < l, d < f, e < j, f < g, f < h, f < i, f < l, j < k, l < m, l < n
fn select_1444([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < e {
        select_1428([a, g, h, c, b, i, l, m, n, e])
    } else {
        select_1443([a, b, d, c, e, f, g, h, j, k, l, m, n, i])
    }
}
/// n = 7, i = 3, b < g, c < f, d < e, e < f, e < g
fn select_1445([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_906([b, c, a, d, e])
    } else {
        select_5([b, c, e, a, f, g])
    }
}
/// n = 9, i = 6, b < g, c < g, d < f, e < f, f < i, g < h
fn select_1446([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < i {
        select_637([a, d, e, g, h, f])
    } else {
        select_637([a, b, c, f, i, g])
    }
}
/// n = 12, i = 6, a < j, b < f, b < l, c < f, c < k, d < g, e < g, f < i, g < h, g < j, j < k, j < l
fn select_1447([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < f {
        select_1445([h, b, c, a, j, k, l])
    } else {
        select_1446([a, d, e, b, c, f, g, h, i])
    }
}
/// n = 10, i = 6, b < g, c < g, d < f, e < f, f < i, f < j, g < h, g < j
fn select_1448([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_372([a, d, e, g, h, f, j])
    } else {
        select_372([a, b, c, f, i, g, j])
    }
}
/// n = 11, i = 6, a < k, b < f, c < g, d < g, e < f, e < j, f < i, g < h, g < j, j < k
fn select_1449([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < j {
        select_1448([a, c, d, b, e, f, g, h, i, j])
    } else {
        select_1445([h, a, b, e, j, f, k])
    }
}
/// n = 12, i = 6, a < k, b < f, b < l, c < f, c < j, d < g, e < g, f < i, g < h, g < j, g < k, k < l
fn select_1450([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if k < j {
        select_1447([a, b, c, d, e, f, g, h, i, k, j, l])
    } else {
        select_1449([a, b, d, e, c, f, g, h, i, j, k])
    }
}
/// n = 7, i = 2, c < f, c < g, d < e, d < g
fn select_1451([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_65([a, b, c, f, d, e, g])
    } else {
        select_65([a, b, d, e, c, f, g])
    }
}
/// n = 9, i = 3, a < e, a < g, b < c, b < d, b < g, e < f, g < h, g < i
fn select_1452([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_811([c, d, a, g, e, h, i])
    } else {
        select_479([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 3, a < e, a < i, b < c, b < d, b < f, e < h, f < g, f < i
fn select_1453([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_65([c, d, f, g, a, e, i])
    } else {
        select_1452([a, b, c, d, e, h, f, g, i])
    }
}
/// n = 10, i = 4, a < h, a < i, b < g, b < h, c < d, c < e, c < g, e < f, e < i, h < j
fn select_1454([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_1451([d, f, a, b, g, i, h])
    } else {
        select_1453([a, c, d, g, h, e, f, j, i])
    }
}
/// n = 12, i = 6, b < f, b < l, c < f, c < k, d < g, e < g, f < i, g < h, g < j, g < k, j < l
fn select_1455([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < j {
        select_1450([a, b, c, d, e, f, g, h, i, k, j, l])
    } else {
        select_1454([b, c, g, h, j, a, k, f, l, i])
    }
}
/// n = 9, i = 3, a < e, a < i, b < d, b < h, b < i, c < f, c < i, f < g, f < h
fn select_1456([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_45([b, f, g, d, a, e, h])
    } else {
        select_869([b, c, a, e, d, f, g, i])
    }
}
/// n = 7, i = 3, a < e, a < f, a < g, b < d, b < f, b < g, c < d, c < e, c < g
fn select_1457([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_4([a, c, d, f, g])
    } else {
        select_4([b, c, e, f, g])
    }
}
/// n = 8, i = 3, a < f, a < h, b < e, b < f, b < g, b < h, c < d, c < e, c < h, d < g
fn select_1458([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_4([b, d, a, e, g])
    } else {
        select_1457([a, b, c, e, d, f, h])
    }
}
/// n = 9, i = 4, a < g, b < f, b < i, c < e, d < f, d < g, d < h, d < i, e < h, e < i
fn select_1459([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_1329([b, c, a, d, g, f, e, h])
    } else {
        select_1458([b, d, e, a, h, f, g, i])
    }
}
/// n = 10, i = 4, b < g, b < i, b < j, c < f, c < j, d < e, d < g, d < h, d < i, e < j, f < h, f < i
fn select_1460([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_1456([e, b, c, g, a, f, h, i, j])
    } else {
        select_1459([a, b, c, d, f, g, e, h, i])
    }
}
/// n = 8, i = 3, b < e, c < f, d < e, d < f, f < g, f < h
fn select_1461([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_302([a, c, b, d, e, f])
    } else {
        select_69([b, c, e, f, a, g, h])
    }
}
/// n = 9, i = 4, b < g, c < f, c < g, d < e, d < g, g < h, g < i
fn select_1462([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_1461([a, e, b, c, f, g, h, i])
    } else {
        select_1461([a, f, b, d, e, g, h, i])
    }
}
/// n = 7, i = 3, a < f, a < g, b < e, b < f, b < g, c < d, c < e, c < g
fn select_1463([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if g < d {
        select_4([a, c, e, f, g])
    } else {
        select_15([a, b, c, d, e, f])
    }
}
/// n = 10, i = 3, a < d, a < f, a < j, b < e, b < g, b < j, c < e, c < f, c < g, c < j, g < h, g < i
fn select_1464([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_36([a, b, e, f, g, h, i, j])
    } else {
        select_1463([b, c, a, d, f, e, j])
    }
}
/// n = 10, i = 4, a < f, b < e, b < g, c < f, c < g, d < e, d < g, d < j, f < j, g < h, g < i
fn select_1465([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_47([a, e, c, g, h, i])
    } else {
        select_157([b, a, c, d, e, f, j])
    }
}
/// n = 11, i = 4, b < f, b < h, b < k, c < f, c < g, c < h, c < k, d < e, d < h, e < g, e < k, h < i, h < j
fn select_1466([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_1464([e, b, c, a, f, g, h, i, j, k])
    } else {
        select_1465([a, b, d, c, f, e, h, i, j, g])
    }
}
/// n = 13, i = 6, b < h, c < g, c < j, c < m, d < f, d < h, e < f, e < g, f < k, f < m, g < i, g < l, h < j, h < k, h < l, h < m
fn select_1467([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_1462([a, b, d, g, i, f, h, j, l])
    } else {
        select_1466([a, c, h, e, f, j, k, g, i, l, m])
    }
}
/// n = 13, i = 6, b < f, b < k, b < m, c < h, d < f, d < g, e < g, e < h, f < j, g < l, g < m, h < i, h < k, h < l, h < m
fn select_1468([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_1460([a, b, d, h, i, g, k, l, m, f])
    } else {
        select_1467([a, c, b, e, d, g, f, h, j, k, l, i, m])
    }
}
/// n = 13, i = 6, b < g, b < l, b < m, c < f, c < g, d < h, e < f, e < h, f < m, g < k, h < i, h < j, h < l, h < m
fn select_1469([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_1455([a, c, b, d, e, g, h, j, k, i, l, f])
    } else {
        select_1468([a, b, d, c, e, g, f, h, j, k, l, i, m])
    }
}
/// n = 15, i = 6, b < g, c < g, c < l, c < m, d < h, e < f, e < h, f < m, g < k, h < i, h < j, h < l, h < m, m < n, m < o
fn select_1470([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < b {
        select_1444([a, c, f, d, b, h, i, j, l, g, k, m, n, o])
    } else {
        select_1469([a, c, b, d, e, f, g, h, i, j, k, l, m])
    }
}
/// n = 15, i = 6, b < g, c < g, c < l, c < m, d < h, e < f, e < h, g < k, h < i, h < j, h < l, h < m, m < n, m < o
fn select_1471([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if m < f {
        select_1422([a, b, c, d, e, g, h, i, j, k, l, m, f, n, o])
    } else {
        select_1470([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o])
    }
}
/// n = 16, i = 7, a < j, b < f, b < l, c < g, c < j, c < p, d < e, d < h, g < i, g < m, h < l, h < m, j < k, l < n, l < o, l < p
fn select_1472([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < l {
        select_1398([a, b, g, d, e, i, f, j, h, k, m, l, n, o, p])
    } else {
        select_1471([e, a, c, h, b, f, j, l, n, o, k, p, g, i, m])
    }
}
/// n = 16, i = 7, a < k, b < g, b < k, b < p, c < f, c < m, d < e, d < h, g < j, h < i, h < m, k < l, m < n, m < o, m < p
fn select_1473([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < g {
        select_1351([e, a, b, c, h, f, i, k, m, n, o, l, g, j, p])
    } else {
        select_1472([a, c, b, d, e, f, g, h, j, k, l, m, i, n, o, p])
    }
}
/// n = 16, i = 7, a < h, b < k, c < k, c < p, d < g, d < m, e < f, e < i, i < j, i < m, k < l, m < n, m < o, m < p
fn select_1474([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < c {
        select_1147([b, d, a, e, f, h, g, c, i, j, k, m, n, o, l, p])
    } else {
        select_1473([b, c, d, e, f, g, a, i, j, h, k, l, m, n, o, p])
    }
}
/// n = 8, i = 2, a < d, b < c, b < e, b < f, e < h, f < g
fn select_1475([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_110([a, b, d, c, f, g, e])
    } else {
        select_115([a, b, d, f, e, c, h, g])
    }
}
/// n = 9, i = 3, a < e, b < e, b < f, b < i, c < d, c < f, d < i, f < g, f < h
fn select_1476([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_67([d, f, a, g, h, i])
    } else {
        select_470([a, b, c, d, e, i, f])
    }
}
/// n = 10, i = 3, a < g, b < d, c < e, c < g, c < j, d < j, e < f, e < h, h < i
fn select_1477([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_1475([a, e, f, g, b, h, i, d])
    } else {
        select_1476([a, c, b, d, g, e, f, h, j])
    }
}
/// n = 9, i = 3, a < d, b < e, b < f, c < e, c < f, c < i, d < i, e < h, f < g
fn select_1478([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_28([f, e, a, h, g])
    } else {
        select_80([b, a, c, d, f, g, i, e])
    }
}
/// n = 12, i = 6, a < g, b < f, b < i, c < h, d < h, e < h, f < k, g < l, h < i, h < k, h < l, i < j
fn select_1479([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_412([a, c, d, e, f, g, h, i, k])
    } else {
        select_1478([a, b, h, g, f, i, j, k, l])
    }
}
/// n = 11, i = 6, a < g, b < f, b < i, c < h, d < h, e < h, f < j, g < k, h < i, h < j, h < k
fn select_1480([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_140([a, b, h, f, i, g, j, k])
    } else {
        select_386([b, c, d, e, g, f, h, k])
    }
}
/// n = 12, i = 6, a < i, b < j, c < g, d < h, e < h, f < h, g < l, h < i, h < j, h < l, j < k
fn select_1481([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < a {
        select_1479([c, b, d, e, f, a, g, h, j, k, i, l])
    } else {
        select_1480([c, a, d, e, f, b, g, h, i, j, l])
    }
}
/// n = 13, i = 6, b < j, c < g, d < h, e < h, f < h, g < m, h < i, h < j, h < m, i < k, k < l
fn select_1482([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < a {
        select_1477([b, c, h, g, i, a, j, k, l, m])
    } else {
        select_1481([b, a, c, d, e, f, g, h, j, i, k, m])
    }
}
/// n = 8, i = 2, c < f, d < e, d < g, g < h
fn select_1483([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_252([a, c, d, e, f, b, g, h])
    } else {
        select_252([b, c, d, e, f, a, g, h])
    }
}
/// n = 10, i = 3, b < j, c < g, d < e, d < f, d < g, e < j, f < h, h < i
fn select_1484([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_1483([a, e, c, f, b, g, h, i])
    } else {
        select_725([a, c, b, d, e, g, f, h, j])
    }
}
/// n = 13, i = 6, b < k, c < g, d < h, e < h, f < h, h < i, h < j, h < k, j < l, l < m
fn select_1485([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < i {
        select_1482([a, b, c, d, e, f, g, h, j, k, l, m, i])
    } else {
        select_1484([a, c, b, h, i, j, k, l, m, g])
    }
}
/// n = 13, i = 6, b < m, c < g, d < h, e < h, f < h, h < i, h < j, h < m, j < k, j < l
fn select_1486([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if k < l {
        select_1485([a, b, c, d, e, f, g, h, i, j, m, k, l])
    } else {
        select_1485([a, b, c, d, e, f, g, h, i, j, m, l, k])
    }
}
/// n = 10, i = 6, a < i, a < j, b < e, b < j, c < f, d < g, f < i, f < j, g < h, g < j
fn select_1487([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_1355([a, d, c, b, e, g, f, i, h, j])
    } else {
        select_1360([b, a, c, g, h, f, i, e, j])
    }
}
/// n = 10, i = 6, b < j, c < i, c < j, d < f, e < g, f < i, f < j, g < h, g < j
fn select_1488([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < a {
        select_1487([c, b, d, e, a, f, g, h, i, j])
    } else {
        select_829([c, a, d, e, b, f, g, h, i])
    }
}
/// n = 8, i = 6, a < h, b < h, c < h, d < g, e < g, f < g
fn select_1489([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < h {
        select_8([a, b, c, g])
    } else {
        select_8([d, e, f, h])
    }
}
/// n = 8, i = 6, b < h, c < h, d < h, e < g, f < g
fn select_1490([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < g {
        select_1489([b, c, d, a, e, f, g, h])
    } else {
        select_560([b, c, d, g, a, h])
    }
}
/// n = 8, i = 6, c < h, d < h, e < g, f < g
fn select_1491([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < g {
        select_1490([b, a, e, f, c, d, h, g])
    } else {
        select_637([b, c, d, g, a, h])
    }
}
/// n = 9, i = 6, c < i, d < h, e < h, f < g, g < i, h < i
fn select_1492([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_926([b, c, d, e, g, a, h, i])
    } else {
        select_1491([b, c, d, e, a, f, g, h])
    }
}
/// n = 8, i = 5, c < h, d < h, e < g, f < g, g < h
fn select_1493([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_356([c, d, a, e, f, b, g, h])
    } else {
        select_356([c, d, b, e, f, a, g, h])
    }
}
/// n = 9, i = 6, c < i, d < h, e < h, f < g, g < i
fn select_1494([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < i {
        select_1492([a, b, c, d, e, f, g, h, i])
    } else {
        select_1493([a, b, d, e, c, g, i, h])
    }
}
/// n = 10, i = 6, c < j, d < i, d < j, e < g, f < h, g < j, h < i, h < j
fn select_1495([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_1488([b, c, d, f, e, h, g, a, i, j])
    } else {
        select_1494([b, c, d, a, e, f, h, g, i])
    }
}
/// n = 9, i = 3, b < i, c < e, d < f, f < g, f < h, f < i
fn select_1496([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < c {
        select_381([a, b, f, g, h, c, e, i])
    } else {
        select_1431([a, b, d, c, e, f, g, h])
    }
}
/// n = 11, i = 5, a < f, b < h, c < g, c < h, d < g, d < h, e < g, e < h, h < i, h < j, h < k
fn select_1497([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_811([a, g, b, h, i, j, k])
    } else {
        select_1097([b, c, d, e, a, f, g])
    }
}
/// n = 11, i = 5, c < h, d < g, d < h, e < g, e < h, f < g, f < h, h < i, h < j, h < k
fn select_1498([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_1497([a, c, d, e, f, b, g, h, i, j, k])
    } else {
        select_1497([b, c, d, e, f, a, g, h, i, j, k])
    }
}
/// n = 12, i = 5, b < l, c < g, c < k, d < h, e < g, e < h, f < g, f < h, h < i, h < j, h < k, h < l
fn select_1499([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < c {
        select_178([a, b, h, i, j, c, g, k, l])
    } else {
        select_1498([a, b, d, c, e, f, g, h, i, j, k])
    }
}
/// n = 12, i = 5, b < l, c < k, d < h, e < g, e < h, f < g, f < h, h < i, h < j, h < k, h < l
fn select_1500([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_1496([a, c, g, d, b, h, i, j, k])
    } else {
        select_1499([a, c, b, d, e, f, g, h, i, j, l, k])
    }
}
/// n = 13, i = 6, b < m, c < g, d < j, e < i, e < j, f < h, h < i, h < j, j < k, j < l, j < m
fn select_1501([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < j {
        select_1495([a, b, d, e, c, f, g, h, i, j])
    } else {
        select_1500([a, b, c, d, e, h, i, j, k, l, g, m])
    }
}
/// n = 13, i = 6, b < m, c < g, d < j, e < i, e < j, f < h, f < j, h < i, h < l, j < k, j < l, j < m
fn select_1502([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < h {
        select_1486([a, b, c, d, e, f, g, j, k, h, i, l, m])
    } else {
        select_1501([a, b, c, d, e, f, g, h, i, j, k, l, m])
    }
}
/// n = 8, i = 5, b < g, c < g, d < g, e < f, e < h, g < h
fn select_1503([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_782([a, b, c, d, e, f, g, h])
    } else {
        select_141([f, b, c, d, a, g, h])
    }
}
/// n = 9, i = 5, b < i, c < h, d < g, e < g, f < g, g < h, g < i
fn select_1504([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_1503([a, d, e, f, b, c, g, i])
    } else {
        select_1503([a, d, e, f, c, b, g, h])
    }
}
/// n = 10, i = 6, b < j, c < i, d < h, e < h, f < g, g < i, g < j, h < i, h < j
fn select_1505([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_1504([a, b, c, d, e, g, h, i, j])
    } else {
        select_742([a, b, c, f, h, g, i, j])
    }
}
/// n = 10, i = 6, b < j, c < i, d < h, e < h, f < g, g < i, g < j, h < j
fn select_1506([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_1505([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_1493([a, b, d, e, c, g, i, h])
    }
}
/// n = 8, i = 4, b < h, c < f, d < e, d < h, e < g, f < g, f < h
fn select_1507([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_214([b, d, f, a, e, g, h])
    } else {
        select_637([b, a, c, d, e, f])
    }
}
/// n = 10, i = 6, b < f, c < g, d < e, d < i, e < j, f < i, f < j, g < h, g < j
fn select_1508([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_183([a, b, d, g, h, e, f, j, i])
    } else {
        select_1507([a, e, c, f, i, g, h, j])
    }
}
/// n = 7, i = 4, a < f, a < g, b < e, c < d, d < f, e < g
fn select_1509([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < d {
        select_13([a, c, d, g])
    } else {
        select_13([a, b, e, f])
    }
}
/// n = 9, i = 5, a < f, b < e, c < d, d < g, d < h, e < g, e < i, f < h, f < i
fn select_1510([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_1509([f, b, c, d, e, h, i])
    } else {
        select_6([a, g])
    }
}
/// n = 7, i = 4, a < g, b < g, c < f, c < g, d < e, e < f
fn select_1511([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_1509([c, a, d, e, b, f, g])
    } else {
        select_1509([c, b, d, e, a, f, g])
    }
}
/// n = 10, i = 6, a < h, b < h, b < i, c < e, d < f, e < i, e < j, f < g, f < i, h < j
fn select_1512([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_1510([a, c, d, f, e, h, i, g, j])
    } else {
        select_1511([a, g, b, c, e, i, h])
    }
}
/// n = 9, i = 5, a < e, b < f, c < d, c < g, d < i, e < h, e < i, f < g, f < h
fn select_1513([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_476([e, b, c, d, f, h, i, g])
    } else {
        select_87([d, a, g, e])
    }
}
/// n = 9, i = 5, a < d, a < h, b < e, c < f, e < h, e < i, f < g, f < i
fn select_1514([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_1513([c, b, a, d, f, e, h, i, g])
    } else {
        select_452([a, b, f, g, e, i, h, d])
    }
}
/// n = 10, i = 6, b < f, c < g, d < e, d < j, e < i, f < i, f < j, g < h, g < j
fn select_1515([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_1512([a, d, b, c, f, g, h, e, j, i])
    } else {
        select_1514([e, b, c, a, f, g, h, i, j])
    }
}
/// n = 10, i = 6, b < j, c < i, d < f, e < g, f < i, f < j, g < h, g < j
fn select_1516([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < b {
        select_1508([a, d, e, c, b, f, g, h, i, j])
    } else {
        select_1515([a, d, e, b, c, f, g, h, i, j])
    }
}
/// n = 10, i = 6, c < j, d < i, e < g, f < h, g < j, h < i, h < j
fn select_1517([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_1506([b, c, d, a, e, f, h, g, i, j])
    } else {
        select_1516([b, c, d, f, e, h, g, a, i, j])
    }
}
/// n = 8, i = 3, a < e, a < g, a < h, b < c, b < d, c < h, d < g, e < f
fn select_1518([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_474([b, a, e, c, d, f, h])
    } else {
        select_474([b, a, e, d, c, f, g])
    }
}
/// n = 10, i = 3, a < d, b < g, b < i, c < e, c < g, c < j, d < j, e < f, e < i, g < h
fn select_1519([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1518([c, a, b, d, e, f, j, g])
    } else {
        select_478([c, b, a, e, f, d, g, h, i])
    }
}
/// n = 8, i = 3, a < e, b < e, b < h, c < d, d < f, d < h, f < g
fn select_1520([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_28([b, f, a, g, h])
    } else {
        select_457([a, b, c, d, e, h, f])
    }
}
/// n = 9, i = 4, a < f, a < g, b < d, c < e, d < f, d < i, e < g, e < i, g < h
fn select_1521([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_3([b, g, h])
    } else {
        select_52([a, d, c, e, f, i])
    }
}
/// n = 10, i = 4, a < e, b < d, b < g, c < f, d < i, e < j, f < g, f < i, f < j, g < h
fn select_1522([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_1520([a, d, c, f, e, g, h, i])
    } else {
        select_1521([b, a, c, e, f, d, g, h, j])
    }
}
/// n = 11, i = 4, a < e, b < d, b < i, c < f, e < k, f < g, f < i, f < k, g < h, i < j
fn select_1523([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_1519([a, b, f, e, g, h, i, j, d, k])
    } else {
        select_1522([a, b, c, d, e, f, i, j, g, k])
    }
}
/// n = 11, i = 4, a < e, b < c, b < d, b < i, d < f, d < k, e < g, e < i, e < k, g < h, i < j
fn select_1524([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_811([c, f, a, e, g, i, k])
    } else {
        select_396([g, b, c, h, i, d, f, k, j])
    }
}
/// n = 12, i = 4, a < e, a < j, b < d, b < i, c < f, f < g, f < i, f < j, g < h, i < k, j < l
fn select_1525([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_1524([c, a, e, b, f, d, g, h, j, l, i])
    } else {
        select_1524([c, b, d, a, f, e, g, h, i, k, j])
    }
}
/// n = 12, i = 4, b < j, c < e, c < i, d < f, f < g, f < i, f < j, g < h, i < k, j < l
fn select_1526([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_1523([a, c, d, e, b, f, g, h, i, k, j])
    } else {
        select_1525([b, c, d, e, a, f, g, h, i, j, k, l])
    }
}
/// n = 10, i = 5, b < f, b < i, b < j, c < g, d < g, e < g, g < h, g < i, g < j
fn select_1527([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_386([b, a, c, d, e, f, g, h])
    } else {
        select_222([b, g, a, h, f, i, j])
    }
}
/// n = 11, i = 5, b < k, c < i, c < j, d < g, e < g, f < g, g < h, g < i, g < j, g < k
fn select_1528([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_1138([a, b, d, e, f, c, g, h, k])
    } else {
        select_1527([a, c, d, e, f, b, g, h, i, j])
    }
}
/// n = 11, i = 5, b < k, c < g, c < j, d < h, e < h, f < g, f < h, h < i, h < j, h < k
fn select_1529([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_1528([a, b, c, d, e, f, h, i, g, j, k])
    } else {
        select_1493([a, b, d, e, c, f, g, h])
    }
}
/// n = 13, i = 5, b < i, c < k, d < g, d < j, e < h, f < g, f < h, h < i, h < j, h < k, j < l, k < m
fn select_1530([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_1526([a, c, d, e, g, h, b, i, j, k, l, m])
    } else {
        select_1529([a, c, d, b, e, f, g, h, i, j, k])
    }
}
/// n = 14, i = 6, b < g, c < l, d < i, d < k, e < j, f < h, h < i, h < j, j < k, j < l, k < m, l < n
fn select_1531([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < j {
        select_1517([a, c, e, d, b, f, g, h, i, j])
    } else {
        select_1530([a, b, c, d, e, h, i, j, g, k, l, m, n])
    }
}
/// n = 15, i = 6, b < g, c < m, c < n, d < i, d < k, e < j, e < k, f < h, f < k, h < i, h < j, j < n, k < l, k < m, n < o
fn select_1532([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < j {
        select_1502([a, c, b, e, d, f, g, h, i, k, l, j, m])
    } else {
        select_1531([a, b, c, d, e, f, g, h, i, j, k, n, l, o])
    }
}
/// n = 9, i = 3, a < d, b < c, b < g, b < h, b < i, c < f, d < e, d < g, d < h, d < i
fn select_1533([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < c {
        select_34([e, b, g, h, i])
    } else {
        select_69([c, a, f, d, g, h, i])
    }
}
/// n = 11, i = 3, a < d, a < g, b < c, b < g, b < j, b < k, c < f, d < e, d < j, d < k, g < h, g < i
fn select_1534([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_69([c, a, f, g, d, h, i])
    } else {
        select_1533([a, b, c, d, e, f, g, j, k])
    }
}
/// n = 11, i = 3, a < c, a < j, a < k, b < d, b < e, c < i, d < g, d < h, e < f, e < j, e < k
fn select_1535([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_636([e, d, g, h, f, a, c])
    } else {
        select_1534([b, a, c, e, f, i, d, g, h, j, k])
    }
}
/// n = 10, i = 3, a < e, a < j, b < c, b < d, b < h, b < i, c < j, d < f, e < g, e < h, e < i
fn select_1536([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_22([c, a, d, f, e, j])
    } else {
        select_1352([e, b, g, c, j, d, h, i])
    }
}
/// n = 10, i = 4, a < d, a < h, b < f, b < h, c < f, c < h, d < e, f < g, g < i, h < j
fn select_1537([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_209([a, f, g, h, i, j, d])
    } else {
        select_126([d, b, c, e, h, j, f])
    }
}
/// n = 12, i = 4, a < i, b < d, b < i, c < e, c < i, d < f, d < j, d < k, e < g, e < j, e < k, g < h, i < l
fn select_1538([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_1536([b, e, a, g, d, h, f, j, k, i])
    } else {
        select_1537([b, a, c, d, f, e, g, i, h, l])
    }
}
/// n = 12, i = 4, b < d, b < h, c < e, c < h, d < f, d < k, d < l, e < g, e < k, e < l, g < i, h < j
fn select_1539([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_1535([e, b, g, h, d, f, a, j, i, k, l])
    } else {
        select_1538([a, b, c, d, e, f, g, i, h, k, l, j])
    }
}
/// n = 9, i = 2, c < d, c < e, c < f, e < i, f < g, f < h
fn select_1540([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_341([b, d, e, f, g, h, a, i])
    } else {
        select_326([b, a, c, d, f, g, h, e])
    }
}
/// n = 10, i = 2, b < d, c < e, c < f, e < j, f < g, f < h, f < i
fn select_1541([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_1399([a, e, f, b, g, h, i, j])
    } else {
        select_275([a, b, c, d, e, j, f])
    }
}
/// n = 13, i = 4, b < e, b < k, c < d, c < i, c < j, d < g, d < k, e < f, e < i, e < j, g < h, j < l, k < m
fn select_1542([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < d {
        select_1540([a, f, c, i, j, d, g, k, l])
    } else {
        select_1541([a, g, b, h, k, e, f, i, j, m])
    }
}
/// n = 13, i = 4, b < g, b < h, c < h, c < i, c < l, d < e, d < g, e < f, e < i, e < l, g < j, h < k, l < m
fn select_1543([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < c {
        select_1539([a, d, b, e, c, f, h, g, k, j, i, l])
    } else {
        select_1542([a, d, c, b, e, f, h, k, i, l, g, m, j])
    }
}
/// n = 9, i = 3, b < e, b < i, c < d, c < h, d < f, d < i, e < g, e < h
fn select_1544([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_409([a, b, d, f, e, i])
    } else {
        select_409([a, c, e, g, d, h])
    }
}
/// n = 9, i = 3, c < f, c < i, d < e, d < f, e < i, f < g, f < h
fn select_1545([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_766([b, e, c, a, f, g, h, i])
    } else {
        select_1461([b, a, c, d, e, f, g, h])
    }
}
/// n = 13, i = 5, b < h, b < j, c < f, c < m, d < e, d < j, e < h, f < g, f < j, h < i, h < m, j < k, j < l
fn select_1546([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_1544([a, b, c, f, h, g, i, m, j])
    } else {
        select_1545([a, g, b, d, e, j, k, l, h])
    }
}
/// n = 11, i = 3, a < e, a < g, b < d, b < f, c < f, c < g, f < h, f < i, g < j, g < k
fn select_1547([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_609([b, c, a, e, d, g, j, k])
    } else {
        select_618([a, b, c, d, f, g, h, i, j, k])
    }
}
/// n = 10, i = 4, a < f, b < g, b < j, c < e, c < g, d < f, d < g, f < j, g < h, g < i
fn select_1548([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_108([b, c, d, e, f, j, g])
    } else {
        select_1334([a, c, d, e, f, g, h, i, j])
    }
}
/// n = 12, i = 4, b < e, b < h, c < g, c < h, d < f, d < h, f < g, g < i, g < j, h < k, h < l
fn select_1549([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_1547([b, f, c, a, e, g, h, i, j, k, l])
    } else {
        select_1548([a, c, b, d, e, f, h, k, l, g])
    }
}
/// n = 13, i = 5, a < e, a < j, b < d, b < g, c < f, c < j, f < h, g < j, g < k, h < i, h < k, j < l, j < m
fn select_1550([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_112([a, b, f, d, e, h, i, g, k])
    } else {
        select_1549([d, a, g, c, e, f, h, j, i, k, l, m])
    }
}
/// n = 15, i = 6, a < f, a < i, b < d, b < g, c < e, c < m, e < i, e < k, f < h, f < m, g < k, g < m, i < j, k < l, m < n, m < o
fn select_1551([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < i {
        select_1546([d, g, a, c, e, f, h, k, l, m, n, o, i])
    } else {
        select_1550([f, b, c, d, h, e, g, i, j, m, k, n, o])
    }
}
/// n = 15, i = 6, a < e, a < i, a < o, b < d, b < g, c < f, c < j, e < h, f < i, f < m, g < j, g < m, i < l, j < k, j < o, m < n
fn select_1552([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < e {
        select_1543([d, f, a, g, j, k, m, i, o, n, l, e, h])
    } else {
        select_1551([a, b, c, d, f, e, g, h, i, l, m, n, j, k, o])
    }
}
/// n = 9, i = 2, a < c, a < f, b < d, b < e, d < h, e < g, f < i
fn select_1553([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_19([c, f, b, i])
    } else {
        select_836([a, b, c, d, e, g, h, f])
    }
}
/// n = 9, i = 2, a < e, b < g, c < d, c < f, f < h, g < i
fn select_1554([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_251([a, c, d, e, f, b, h, g])
    } else {
        select_1553([c, b, d, a, g, f, i, e, h])
    }
}
/// n = 9, i = 3, a < f, a < g, b < d, b < e, c < e, c < f, c < g, c < i, d < i, g < h
fn select_1555([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_51([b, a, f, e, g, h])
    } else {
        select_291([a, c, b, d, e, f, i])
    }
}
/// n = 10, i = 3, a < g, b < d, b < f, c < e, c < f, c < g, c < j, d < j, e < h, h < i
fn select_1556([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_1553([b, e, f, a, h, d, i, g, j])
    } else {
        select_1555([a, b, c, d, f, g, e, h, j])
    }
}
/// n = 10, i = 3, b < e, b < g, c < d, c < f, c < g, c < j, e < j, f < h, h < i
fn select_1557([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_1554([d, f, b, g, a, e, h, j, i])
    } else {
        select_1556([a, b, c, e, f, g, d, h, i, j])
    }
}
/// n = 12, i = 4, a < d, b < c, b < i, c < f, d < e, d < i, e < g, f < h, i < j, i < k, i < l
fn select_1558([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < d {
        select_3([h, a, d])
    } else {
        select_426([e, b, g, c, i, j, k, l, f])
    }
}
/// n = 12, i = 4, a < c, a < l, b < d, c < f, d < e, d < g, e < j, f < k, g < h, g < i, g < l
fn select_1559([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < a {
        select_116([e, g, h, i, j, a, c, l])
    } else {
        select_1558([b, a, c, d, e, f, j, k, g, h, i, l])
    }
}
/// n = 8, i = 4, a < f, a < h, b < f, b < h, c < d, c < e, e < h, f < g
fn select_1560([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_42([f, c, g, e, h, d])
    } else {
        select_153([d, e, a, b, h, f])
    }
}
/// n = 11, i = 5, a < f, b < g, b < i, b < k, c < d, c < e, d < i, e < i, e < k, f < g, f < k, g < h, i < j
fn select_1561([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < i {
        select_1560([b, f, c, d, e, g, h, k])
    } else {
        select_3([a, i, j])
    }
}
/// n = 8, i = 3, a < h, b < f, c < e, c < h, d < e, d < f, d < h, f < g
fn select_1562([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < b {
        select_5([a, b, d, e, f, h])
    } else {
        select_58([a, c, d, f, g, e])
    }
}
/// n = 13, i = 6, a < g, b < h, b < i, b < l, c < g, c < j, c < l, d < e, d < f, e < l, f < h, f < l, g < i, h < j, i < k, l < m
fn select_1563([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_1561([a, b, d, e, f, g, i, k, l, m, h])
    } else {
        select_1562([a, e, h, c, j, l, m, g])
    }
}
/// n = 14, i = 6, a < g, a < n, b < h, b < i, b < j, c < g, c < j, c < l, d < e, d < f, f < h, f < j, g < i, h < l, i < m, j < k, j < n
fn select_1564([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < e {
        select_1559([a, d, g, f, h, i, j, e, k, l, m, n])
    } else {
        select_1563([a, b, c, d, e, f, g, h, i, l, m, j, k])
    }
}
/// n = 15, i = 6, a < h, a < k, b < g, b < l, c < g, c < i, c < m, d < e, d < f, f < h, f < i, g < k, g < o, h < m, i < j, i < l, i < o, k < n
fn select_1565([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < a {
        select_1557([e, b, i, j, g, a, l, h, m, o])
    } else {
        select_1564([b, a, c, d, e, f, g, h, k, i, j, m, n, l])
    }
}
/// n = 10, i = 5, a < g, a < i, b < g, b < i, c < g, c < i, d < e, d < i, e < f, g < h, i < j
fn select_1566([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_141([e, a, b, c, f, i, j])
    } else {
        select_51([g, d, e, h, i, j])
    }
}
/// n = 10, i = 4, a < f, a < h, a < j, b < e, b < f, c < d, c < e, c < i, d < h, e < j, f < g, f < i
fn select_1567([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_29([f, c, g, e, j, d, i])
    } else {
        select_470([d, a, b, e, h, j, f])
    }
}
/// n = 13, i = 6, a < g, a < k, b < g, b < k, c < f, c < j, c < k, d < e, d < f, e < k, f < h, f < l, g < i, g < l, i < j, k < m
fn select_1568([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < g {
        select_1566([a, b, e, c, f, h, g, i, k, m])
    } else {
        select_1567([c, d, g, i, e, f, h, j, l, k])
    }
}
/// n = 11, i = 5, a < g, a < i, a < k, b < e, c < d, c < g, d < k, e < f, e < j, e < k, f < i, g < h, g < j
fn select_1569([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_470([f, a, c, d, i, k, g])
    } else {
        select_30([d, g, b, h, e, f, j, k])
    }
}
/// n = 13, i = 6, a < g, a < i, b < g, b < j, c < f, c < l, c < m, d < e, d < f, e < m, f < i, f < j, g < h, g < m, h < l, j < k
fn select_1570([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < g {
        select_106([e, a, b, i, j, k, g])
    } else {
        select_1569([c, b, d, e, g, h, f, i, l, j, m])
    }
}
/// n = 15, i = 7, a < g, a < i, b < h, b < m, b < n, c < f, c < n, d < e, d < h, e < n, f < j, f < k, g < j, g < n, h < i, h < k, j < m, k < l, n < o
fn select_1571([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < f {
        select_1568([c, g, b, d, e, h, f, i, j, m, n, k, o])
    } else {
        select_1570([a, f, b, d, e, h, g, j, i, k, l, m, n])
    }
}
/// n = 7, i = 3, a < g, b < g, c < e, d < e, d < g, e < f
fn select_1572([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_642([c, a, d, b, e, f, g])
    } else {
        select_642([c, b, d, a, e, f, g])
    }
}
/// n = 10, i = 5, a < g, b < h, b < i, c < g, c < i, d < e, e < f, e < i, g < h, i < j
fn select_1573([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_1521([b, a, d, g, e, h, i, j, f])
    } else {
        select_1572([a, f, b, c, i, j, g])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, c < g, d < f, e < f, e < i, f < h, g < i
fn select_1574([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_153([a, d, e, g, f, i])
    } else {
        select_302([b, c, a, f, h, g])
    }
}
/// n = 10, i = 5, b < g, c < h, c < i, d < f, e < g, e < i, f < i, g < h, i < j
fn select_1575([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_1573([b, c, e, d, f, a, g, h, i, j])
    } else {
        select_1574([c, a, d, b, e, g, f, h, i])
    }
}
/// n = 9, i = 5, a < g, b < h, c < g, c < h, d < g, d < h, e < f, e < h, h < i
fn select_1576([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_562([a, f, b, c, d, h, i])
    } else {
        select_1097([b, a, c, d, e, f, g])
    }
}
/// n = 10, i = 5, a < h, b < h, c < g, d < g, d < h, e < f, e < h, f < i, g < i, h < j
fn select_1577([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_1576([c, b, a, d, e, f, g, h, j])
    } else {
        select_53([b, e, g, a, f, i])
    }
}
/// n = 13, i = 6, a < i, b < h, b < k, c < i, c < j, c < k, d < f, d < i, e < g, e < k, f < k, g < h, g < m, h < j, i < m, k < l
fn select_1578([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_1575([a, g, c, d, b, f, h, j, k, l])
    } else {
        select_1577([b, f, a, c, e, g, i, k, m, l])
    }
}
/// n = 15, i = 7, a < i, a < l, a < n, b < f, b < i, c < e, c < h, d < g, d < n, f < n, g < j, g < k, h < j, h < n, i < k, j < l, k < m, n < o
fn select_1579([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < e {
        select_1571([c, a, d, b, f, g, h, i, e, j, k, m, l, n, o])
    } else {
        select_1578([e, h, a, b, d, f, g, j, i, l, n, o, k])
    }
}
/// n = 16, i = 7, a < f, a < i, b < i, b < k, b < n, c < e, c < h, d < g, d < k, g < j, g < m, h < j, h < k, i < m, i < p, j < n, k < l, k < p, m < o
fn select_1580([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < f {
        select_1565([g, a, b, c, e, h, i, j, k, l, m, f, n, o, p])
    } else {
        select_1579([b, a, c, d, e, f, g, h, i, j, m, n, o, k, l])
    }
}
/// n = 16, i = 7, a < f, b < g, b < l, b < o, c < e, c < i, d < h, d < l, g < k, g < p, h < j, h < k, i < j, i < l, j < o, k < m, l < n, l < p
fn select_1581([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < a {
        select_1552([g, c, d, e, a, h, i, f, k, l, n, m, j, o, p])
    } else {
        select_1580([a, b, c, d, e, f, h, i, g, j, l, n, k, o, m, p])
    }
}
/// n = 8, i = 4, a < d, a < e, b < f, c < f, f < g, f < h
fn select_1582([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_105([a, f, g, h, e, d])
    } else {
        select_9([e, b, c, d, f])
    }
}
/// n = 8, i = 4, a < f, b < f, c < d, c < h, d < e, f < g, f < h
fn select_1583([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_87([e, a, b, f])
    } else {
        select_19([g, c, d, h])
    }
}
/// n = 9, i = 4, a < d, a < i, b < f, c < f, d < e, f < g, f < h, f < i
fn select_1584([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_194([g, h, a, i])
    } else {
        select_1583([b, c, a, d, e, f, h, i])
    }
}
/// n = 9, i = 4, a < i, b < e, c < f, d < f, f < g, f < h, f < i
fn select_1585([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_1582([b, c, d, a, e, f, g, h])
    } else {
        select_1584([a, c, d, b, e, f, g, h, i])
    }
}
/// n = 9, i = 2, b < d, b < i, c < e, c < i, d < h, e < f, e < g
fn select_1586([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_58([a, c, b, d, h, i])
    } else {
        select_48([a, b, c, e, f, g, i])
    }
}
/// n = 12, i = 5, b < f, b < h, c < f, c < i, d < g, e < g, g < h, g < i, h < l, i < j, i < k
fn select_1587([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_1586([a, b, c, h, i, j, k, l, f])
    } else {
        select_146([a, c, d, e, f, g, h, l, i])
    }
}
/// n = 13, i = 5, a < m, b < f, b < j, c < f, c < h, d < g, d < h, e < g, e < h, g < j, h < i, h < m, j < k, j < l
fn select_1588([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_1585([a, b, d, e, f, h, g, i, m])
    } else {
        select_1587([a, c, b, d, e, f, g, h, j, k, l, i])
    }
}
/// n = 10, i = 6, a < h, b < i, b < j, c < i, c < j, d < f, e < g, f < i, f < j, g < h, g < j
fn select_1589([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_866([a, b, d, e, f, c, g, h, i, j])
    } else {
        select_866([a, c, d, e, f, b, g, h, i, j])
    }
}
/// n = 10, i = 6, a < i, a < j, b < h, b < j, c < g, d < g, e < f, e < j, f < h, g < i, g < j
fn select_1590([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_108([b, a, e, i, f, h, j])
    } else {
        select_12([a, b, c, d, h, g, i])
    }
}
/// n = 12, i = 6, a < i, b < h, b < j, c < g, d < g, e < f, e < j, f < h, g < i, g < j, j < k, j < l
fn select_1591([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < j {
        select_1590([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_69([f, g, h, j, a, k, l])
    }
}
/// n = 13, i = 6, a < f, b < i, b < j, c < h, c < m, d < h, d < m, e < g, e < j, f < m, g < i, g < m, h < j, j < k, j < l
fn select_1592([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < h {
        select_1589([b, c, d, a, e, f, g, i, h, m])
    } else {
        select_1591([a, b, c, d, e, g, h, i, f, j, k, l])
    }
}
/// n = 14, i = 6, a < f, b < i, b < l, c < h, c < j, d < h, d < j, e < g, e < l, g < i, g < j, h < l, j < k, l < m, l < n
fn select_1593([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < f {
        select_1588([a, b, g, c, d, i, h, j, k, l, m, n, f])
    } else {
        select_1592([a, b, c, d, e, f, g, h, i, l, m, n, j])
    }
}
/// n = 8, i = 2, a < f, b < c, b < d, b < e, b < g, g < h
fn select_1594([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_1475([a, b, e, f, c, g, h, d])
    } else {
        select_1475([a, b, e, f, d, g, h, c])
    }
}
/// n = 9, i = 4, a < e, b < f, c < f, d < f, f < g, f < h, f < i
fn select_1595([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_33([a, g, h, i])
    } else {
        select_9([e, b, c, d, f])
    }
}
/// n = 10, i = 4, b < e, c < f, d < f, f < g, f < h, f < i, i < j
fn select_1596([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_1594([b, f, a, g, h, e, i, j])
    } else {
        select_1595([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 12, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < k, f < l, g < k, g < l, h < i, h < j
fn select_1597([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_311([a, g, f, h, i, j, k, l])
    } else {
        select_1117([a, b, c, d, e, f, g, k, l])
    }
}
/// n = 10, i = 4, a < d, b < f, c < e, c < g, e < i, e < j, f < g, f < i, f < j, g < h
fn select_1598([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_53([d, b, c, e, f, g])
    } else {
        select_115([a, c, d, g, e, i, j, h])
    }
}
/// n = 13, i = 5, a < e, b < i, b < m, c < g, d < f, d < i, f < h, f < m, g < h, g < i, g < m, h < j, i < k, i < l
fn select_1599([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < b {
        select_1598([a, c, f, e, b, g, h, j, i, m])
    } else {
        select_1597([a, b, d, c, e, g, f, i, k, l, h, m])
    }
}
/// n = 15, i = 6, a < e, b < i, b < k, c < f, d < g, f < i, f < j, f < k, g < h, g < k, g < o, h < j, i < o, j < l, k < m, k < n
fn select_1600([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_1597([a, b, f, d, e, g, i, k, m, n, h, o])
    } else {
        select_1599([a, b, c, g, e, h, f, j, k, l, m, n, i])
    }
}
/// n = 15, i = 6, a < e, b < i, b < j, c < g, d < f, d < j, f < i, f < m, g < h, g < j, g < o, h < m, i < o, j < k, j < l, m < n
fn select_1601([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < f {
        select_1596([h, a, b, d, e, j, k, l, f, i])
    } else {
        select_1600([a, b, d, c, e, f, g, h, i, m, j, n, k, l, o])
    }
}
/// n = 15, i = 6, a < f, b < j, c < i, c < k, d < h, d < j, e < g, e < k, g < i, g < j, h < k, h < o, i < o, j < n, k < l, k < m
fn select_1602([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if b < h {
        select_1593([a, c, b, d, e, f, g, h, i, j, n, k, l, m])
    } else {
        select_1601([a, c, d, e, f, g, h, b, i, k, l, m, j, n, o])
    }
}
/// n = 9, i = 3, a < c, a < d, b < e, b < i, d < i, e < f, e < g, g < h
fn select_1603([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_172([f, g, a, c, d, h])
    } else {
        select_48([c, d, b, e, f, g, i])
    }
}
/// n = 9, i = 3, a < c, a < g, b < d, b < g, c < f, d < e, d < h, h < i
fn select_1604([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_4([b, c, f, g, d])
    } else {
        select_45([a, d, e, g, h, i, c])
    }
}
/// n = 9, i = 3, a < g, b < d, c < e, c < g, e < f, e < h, h < i
fn select_1605([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_1603([b, c, d, a, e, f, h, i, g])
    } else {
        select_1604([a, c, b, e, f, d, g, h, i])
    }
}
/// n = 9, i = 3, b < e, c < d, c < f, f < g, f < h, h < i
fn select_1606([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_252([b, d, f, g, a, e, h, i])
    } else {
        select_1605([a, b, c, e, f, g, d, h, i])
    }
}
/// n = 10, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < g, e < f, h < i, h < j
fn select_1607([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_69([d, c, g, h, e, i, j])
    } else {
        select_579([a, c, b, d, e, f, g, h])
    }
}
/// n = 10, i = 4, a < e, b < h, c < g, c < h, d < f, d < h, f < g, h < i, h < j
fn select_1608([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_852([c, d, a, e, b, f, g, h])
    } else {
        select_1607([c, d, b, f, a, e, g, h, i, j])
    }
}
/// n = 10, i = 4, a < e, b < h, c < f, c < h, d < f, d < h, f < g, h < i, h < j
fn select_1609([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_51([a, f, g, e, b, h])
    } else {
        select_382([a, b, c, d, e, f, h, i, j, g])
    }
}
/// n = 10, i = 4, a < f, b < h, c < g, c < h, d < e, d < g, d < h, h < i, h < j
fn select_1610([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_1608([a, b, c, d, f, e, g, h, i, j])
    } else {
        select_1609([a, b, c, d, f, g, e, h, i, j])
    }
}
/// n = 10, i = 4, a < j, b < f, c < g, c < h, d < e, d < g, d < h, h < i, h < j
fn select_1611([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_1606([e, b, c, g, f, h, i, a, j])
    } else {
        select_1610([b, a, c, d, e, f, g, h, i, j])
    }
}
/// n = 8, i = 3, b < d, b < h, c < e, e < f, e < g, e < h
fn select_1612([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_678([a, b, e, f, g, h])
    } else {
        select_106([a, b, c, d, e, g, h])
    }
}
/// n = 9, i = 3, b < d, c < e, e < f, e < g, e < h, h < i
fn select_1613([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < b {
        select_677([a, f, g, h, b, i])
    } else {
        select_1612([a, b, c, d, e, f, g, h])
    }
}
/// n = 12, i = 4, a < e, b < f, b < g, c < f, c < g, d < f, d < g, f < k, f < l, g < h, g < i, g < j
fn select_1614([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_340([a, f, g, h, i, j, k, l])
    } else {
        select_1595([a, b, c, d, e, g, h, i, j])
    }
}
/// n = 12, i = 4, a < e, b < i, b < l, c < f, c < i, d < f, d < i, f < g, f < h, f < l, i < j, i < k
fn select_1615([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_116([a, f, g, h, e, b, i, l])
    } else {
        select_1614([a, b, c, d, e, i, f, g, h, l, j, k])
    }
}
/// n = 14, i = 5, a < e, b < g, b < k, c < g, c < h, c < k, d < f, f < h, f < k, f < n, g < n, h < i, h < j, k < l, k < m
fn select_1616([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_1615([a, b, c, f, e, h, i, j, k, l, m, g])
    } else {
        select_1597([a, b, c, d, e, f, g, k, l, m, h, n])
    }
}
/// n = 14, i = 5, a < e, b < g, b < i, c < g, c < h, c < i, d < f, d < i, f < h, f < n, g < n, h < j, h < k, i < l, i < m
fn select_1617([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < f {
        select_1613([g, a, d, e, i, l, m, f, h])
    } else {
        select_1616([a, b, c, d, e, f, g, h, j, k, i, l, m, n])
    }
}
/// n = 15, i = 6, a < e, b < h, b < j, c < g, c < k, d < f, d < h, f < l, f < o, g < j, g < l, h < i, h < k, j < o, l < m, l < n
fn select_1618([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < g {
        select_1611([f, a, c, h, i, e, k, g, j, l])
    } else {
        select_1617([a, b, g, d, e, f, j, l, h, m, n, i, k, o])
    }
}
/// n = 16, i = 7, a < f, b < h, b < j, c < e, c < i, d < g, d < m, g < j, g < l, h < m, h < p, i < l, i < m, j < k, l < p, m < n, m < o
fn select_1619([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if e < j {
        select_1602([a, e, i, b, d, f, g, h, l, j, m, n, o, k, p])
    } else {
        select_1618([a, g, c, b, f, h, i, j, k, l, e, m, n, o, p])
    }
}
/// n = 16, i = 7, a < g, b < j, b < p, c < m, c < o, d < f, d < i, e < h, e < m, h < j, h < l, i < l, i < m, j < k, l < o, m < n, m < p
fn select_1620([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if c < b {
        select_1581([a, c, d, e, f, g, b, h, i, l, j, m, k, n, o, p])
    } else {
        select_1619([a, b, d, e, f, g, h, c, i, j, k, l, m, n, p, o])
    }
}
/// n = 16, i = 7, a < g, b < k, b < p, c < l, c < n, d < h, d < n, e < f, e < i, h < k, i < j, i < n, j < l, k < m, n < o, n < p
fn select_1621([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < h {
        select_1532([f, a, b, c, d, i, g, j, l, h, n, o, p, k, m])
    } else {
        select_1620([a, b, c, e, d, f, g, h, i, k, m, j, n, o, l, p])
    }
}
/// n = 9, i = 5, c < g, c < h, d < g, d < h, e < f, f < h, h < i
fn select_1622([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_492([b, c, d, f, a, g, h, i])
    } else {
        select_524([b, a, e, c, d, g, f, h])
    }
}
/// n = 11, i = 5, a < f, a < k, b < g, b < h, c < g, c < h, d < e, d < h, h < i, h < k, i < j
fn select_1623([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_681([e, g, h, i, a, f, k, j])
    } else {
        select_812([b, c, a, d, e, f, g, h, i, k])
    }
}
/// n = 12, i = 5, a < l, b < k, c < g, c < h, d < g, d < h, e < f, e < h, h < i, h < k, h < l, i < j
fn select_1624([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_1623([a, c, d, e, f, b, g, h, i, j, l])
    } else {
        select_1623([b, c, d, e, f, a, g, h, i, j, k])
    }
}
/// n = 12, i = 5, b < l, c < k, d < h, e < g, e < h, f < g, f < h, h < i, h < k, h < l, i < j
fn select_1625([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < d {
        select_1622([b, c, e, f, a, d, g, h, i])
    } else {
        select_1624([b, c, e, f, d, a, g, h, i, j, k, l])
    }
}
/// n = 13, i = 6, b < m, c < g, d < j, e < i, e < j, f < h, h < i, h < j, j < k, j < m, k < l
fn select_1626([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < j {
        select_1495([a, b, d, e, c, f, g, h, i, j])
    } else {
        select_1625([a, b, c, d, e, h, i, j, k, l, g, m])
    }
}
/// n = 8, i = 4, a < e, a < h, b < f, b < h, c < f, c < h, d < f, d < h, f < g
fn select_1627([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_9([e, b, c, d, h])
    } else {
        select_20([a, f, g, h])
    }
}
/// n = 11, i = 4, a < e, b < f, b < g, c < f, c < g, d < f, d < g, f < k, g < h, g < i, g < j
fn select_1628([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_68([f, g, a, h, i, j, k])
    } else {
        select_1627([a, b, c, d, e, f, k, g])
    }
}
/// n = 11, i = 4, a < e, b < h, c < f, c < h, d < f, d < h, f < g, h < i, h < j, h < k
fn select_1629([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_51([a, f, g, e, b, h])
    } else {
        select_1628([a, b, c, d, e, f, h, i, j, k, g])
    }
}
/// n = 11, i = 4, a < e, b < f, b < k, c < g, d < f, d < g, f < j, g < h, g < i, g < k
fn select_1630([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_116([a, g, h, i, e, b, f, k])
    } else {
        select_1629([a, c, b, d, e, f, j, g, h, i, k])
    }
}
/// n = 10, i = 6, a < f, b < h, c < h, d < h, e < g, g < i, g < j, h < i, h < j
fn select_1631([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_255([a, h, e, g, f, i, j])
    } else {
        select_1112([b, c, d, f, e, g, h])
    }
}
/// n = 11, i = 6, a < k, b < f, c < h, d < h, e < g, g < i, g < k, h < i, h < k, i < j
fn select_1632([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_111([b, e, h, f, a, g, i, j, k])
    } else {
        select_1631([b, a, c, d, e, f, g, h, i, k])
    }
}
/// n = 12, i = 6, a < f, b < i, c < g, d < h, e < h, g < k, h < i, h < k, i < j, k < l
fn select_1633([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_1632([b, a, d, e, c, f, g, h, k, l, i])
    } else {
        select_10([c, a, b, h, f, i, j, g])
    }
}
/// n = 13, i = 6, a < g, b < i, b < m, c < f, c < j, d < h, e < h, h < i, h < j, i < l, j < k, j < m
fn select_1634([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < f {
        select_1630([a, b, c, h, g, i, j, f, k, l, m])
    } else {
        select_1633([a, b, c, d, e, g, f, h, i, l, j, k])
    }
}
/// n = 14, i = 6, a < g, b < i, c < f, c < j, d < h, e < h, h < i, h < j, i < l, j < k, j < m, m < n
fn select_1635([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if m < b {
        select_996([f, a, h, g, j, k, m, b, n])
    } else {
        select_1634([a, b, c, d, e, f, g, h, i, j, k, l, m])
    }
}
/// n = 15, i = 7, a < g, b < l, c < j, d < f, d < h, e < i, h < k, h < n, i < j, i < k, j < l, j < n, l < m, n < o
fn select_1636([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < j {
        select_1626([f, b, a, c, h, e, g, i, k, j, n, o, l])
    } else {
        select_1635([a, b, d, c, i, f, g, j, l, h, k, m, n, o])
    }
}
/// n = 16, i = 7, a < g, b < n, b < p, c < j, c < l, d < f, d < i, e < h, e < l, h < j, h < k, i < k, i < l, j < n, l < m, l < p, n < o
fn select_1637([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < j {
        select_1502([f, b, a, c, i, e, g, h, k, l, m, j, p])
    } else {
        select_1636([a, b, c, d, e, f, g, i, h, j, k, n, o, l, m])
    }
}
/// n = 8, i = 4, a < f, a < g, b < e, b < g, c < d, c < f, c < h, g < h
fn select_1638([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_150([d, b, a, f, e, g, h])
    } else {
        select_291([e, c, a, g, f, d, h])
    }
}
/// n = 8, i = 3, b < e, b < f, b < h, c < d, c < f, c < g, d < h, e < g
fn select_1639([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_45([d, b, f, a, e, g, h])
    } else {
        select_291([a, c, b, e, f, d, g])
    }
}
/// n = 9, i = 4, a < g, b < e, b < g, b < h, c < d, c < f, e < i, f < h, f < i
fn select_1640([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_1638([a, c, b, e, d, g, f, h])
    } else {
        select_1639([d, b, f, a, e, h, i, g])
    }
}
/// n = 8, i = 5, a < g, b < g, c < g, d < e, e < f, g < h
fn select_1641([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_13([d, g, h, e])
    } else {
        select_8([a, b, c, f])
    }
}
/// n = 9, i = 6, a < h, b < h, c < h, d < g, d < i, e < f, e < g, h < i
fn select_1642([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_559([a, b, c, d, e, f, g, h, i])
    } else {
        select_1641([a, b, c, e, g, f, h, i])
    }
}
/// n = 11, i = 6, a < i, b < h, c < h, d < h, e < f, e < g, g < j, g < k, h < i, h < j, h < k
fn select_1643([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_1113([f, b, c, d, g, a, h, j, k])
    } else {
        select_1642([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 13, i = 6, a < k, b < h, b < l, c < h, c < i, d < h, d < i, e < f, e < g, g < i, g < m, h < k, h < m, i < j, i < l
fn select_1644([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < h {
        select_1111([a, f, b, c, d, g, i, j, h, l])
    } else {
        select_1643([a, b, c, d, e, f, g, h, k, i, m])
    }
}
/// n = 13, i = 6, a < i, b < j, c < h, c < m, d < h, d < j, e < f, e < g, g < j, g < k, h < i, h < k, j < l, j < m
fn select_1645([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_1640([a, h, e, f, b, g, i, k, j])
    } else {
        select_1644([a, c, b, d, e, f, g, h, j, l, i, m, k])
    }
}
/// n = 10, i = 5, b < j, c < i, d < g, e < g, f < g, g < h, g < i, g < j
fn select_1646([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_1138([a, b, d, e, f, c, g, h, j])
    } else {
        select_1138([a, c, d, e, f, b, g, h, i])
    }
}
/// n = 12, i = 6, b < k, b < l, c < h, d < i, e < h, e < i, f < g, g < i, g < l, h < l, i < j, i < k
fn select_1647([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_1646([a, b, c, d, e, g, i, j, h, k])
    } else {
        select_1505([a, b, d, c, e, f, g, h, i, l])
    }
}
/// n = 9, i = 5, a < i, b < g, c < g, c < h, d < g, d < h, e < f, e < h, h < i
fn select_1648([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_742([f, a, b, c, d, h, g, i])
    } else {
        select_1503([a, b, c, d, e, f, g, h])
    }
}
/// n = 10, i = 6, a < j, b < i, c < h, c < i, d < g, e < f, e < h, g < i, g < j, h < j
fn select_1649([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_1648([a, b, c, g, e, f, i, h, j])
    } else {
        select_742([f, a, b, d, h, g, i, j])
    }
}
/// n = 13, i = 7, a < j, a < m, b < g, c < k, d < i, e < f, e < h, g < j, h < k, i < j, i < k, k < l, k < m
fn select_1650([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_1647([f, a, b, c, h, d, i, g, k, l, m, j])
    } else {
        select_1649([c, a, g, d, e, f, i, h, j, k])
    }
}
/// n = 14, i = 7, a < g, b < k, c < j, c < n, d < i, e < f, e < h, h < k, h < m, i < j, i < k, j < m, k < l, k < n
fn select_1651([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < g {
        select_1645([a, b, c, i, e, f, h, j, g, k, m, l, n])
    } else {
        select_1650([c, a, b, d, e, f, g, h, i, j, k, l, n])
    }
}
/// n = 7, i = 4, b < f, c < f, c < g, d < e, e < g
fn select_1652([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_157([b, a, d, c, f, e, g])
    } else {
        select_15([b, c, e, a, g, f])
    }
}
/// n = 9, i = 6, b < f, c < h, d < h, e < g, g < i, h < i
fn select_1653([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_1652([a, b, h, e, g, f, i])
    } else {
        select_1097([a, c, d, f, e, g, h])
    }
}
/// n = 6, i = 3, c < e, c < f, d < e, d < f
fn select_1654([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_368([a, b, c, d, e, f])
    } else {
        select_368([a, b, c, d, f, e])
    }
}
/// n = 10, i = 6, b < f, c < h, d < h, e < g, g < i, g < j, h < i, h < j
fn select_1655([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < i {
        select_1653([a, b, c, d, e, f, g, h, j])
    } else {
        select_1654([a, b, g, h, i, j])
    }
}
/// n = 11, i = 6, a < k, b < f, c < h, d < h, e < g, e < i, g < j, h < i, h < j, i < k
fn select_1656([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < g {
        select_428([a, b, e, h, f, i, g, j, k])
    } else {
        select_1655([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 8, i = 3, a < e, b < d, b < h, c < f, c < h, f < g
fn select_1657([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < f {
        select_132([a, c, b, e, d, h, f])
    } else {
        select_151([a, b, c, e, f, g, h, d])
    }
}
/// n = 10, i = 4, a < e, b < f, b < i, c < g, c < j, d < f, d < g, f < j, g < h, g < i
fn select_1658([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_64([a, g, h, e, b, f, i])
    } else {
        select_1021([a, c, b, d, e, f, g, h, i, j])
    }
}
/// n = 10, i = 4, a < e, b < i, b < j, c < g, c < h, d < f, d < g, f < h, f < i, g < j
fn select_1659([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_1657([a, c, f, g, e, b, i, h])
    } else {
        select_1658([a, c, b, d, e, g, f, i, h, j])
    }
}
/// n = 12, i = 6, a < g, b < k, b < l, c < f, c < j, d < h, e < h, h < i, h < j, i < k, j < l
fn select_1660([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_1656([b, a, d, e, c, g, f, h, j, i, l])
    } else {
        select_1659([a, b, c, h, g, i, j, f, k, l])
    }
}
/// n = 10, i = 5, a < e, b < g, c < g, c < j, d < f, e < j, f < h, f < i, f < j, g < h, g < i
fn select_1661([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_255([a, g, d, f, e, h, i])
    } else {
        select_240([b, c, e, d, f, g, j])
    }
}
/// n = 8, i = 3, a < h, b < e, c < d, c < f, c < g, d < h, e < f, e < g
fn select_1662([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_35([c, e, a, d, f, g])
    } else {
        select_52([b, a, c, d, e, h])
    }
}
/// n = 10, i = 5, a < e, b < g, c < g, c < h, d < f, f < h, f < i, f < j, g < i, g < j
fn select_1663([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_1661([a, b, c, d, e, f, g, i, j, h])
    } else {
        select_1662([a, b, f, h, g, i, j, e])
    }
}
/// n = 10, i = 5, a < e, b < h, b < j, c < f, d < g, f < i, f < j, g < h, g < i, g < j
fn select_1664([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < f {
        select_1663([a, c, b, d, e, g, f, h, i, j])
    } else {
        select_295([a, d, f, e, b, g, i, h, j])
    }
}
/// n = 11, i = 5, a < e, b < h, b < i, c < f, c < k, d < g, f < j, g < h, g < i, g < j, i < k
fn select_1665([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < f {
        select_1207([a, c, b, g, e, h, i, f, k])
    } else {
        select_1664([a, b, c, d, e, f, g, h, j, i])
    }
}
/// n = 9, i = 6, a < i, b < h, c < h, c < i, d < g, e < f, f < i, g < i
fn select_1666([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_925([a, b, d, f, c, h, g, i])
    } else {
        select_925([a, b, e, g, c, h, f, i])
    }
}
/// n = 10, i = 6, a < h, b < f, c < i, d < h, d < i, e < g, g < i, i < j
fn select_1667([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < f {
        select_10([b, a, c, g, h, i, j, f])
    } else {
        select_1666([c, a, d, b, e, g, f, h, i])
    }
}
/// n = 12, i = 6, a < g, b < h, b < l, c < f, c < j, d < i, e < h, e < i, h < k, i < j, i < k, j < l
fn select_1668([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_1665([a, c, b, d, g, h, i, f, j, k, l])
    } else {
        select_1667([b, a, d, e, c, g, f, h, i, j])
    }
}
/// n = 12, i = 6, a < g, b < h, b < l, c < f, c < k, d < i, e < h, e < i, i < j, i < k, k < l
fn select_1669([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < h {
        select_1660([a, b, c, d, e, f, g, i, j, k, h, l])
    } else {
        select_1668([a, b, c, d, e, f, g, h, i, k, j, l])
    }
}
/// n = 14, i = 7, a < g, b < k, c < j, c < n, d < f, d < h, e < i, h < m, h < n, i < j, i < k, j < m, k < l, k < n
fn select_1670([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < k {
        select_1651([a, b, c, e, d, f, g, h, i, j, k, l, m, n])
    } else {
        select_1669([a, c, d, b, i, f, g, j, k, l, h, n])
    }
}
/// n = 7, i = 3, a < e, a < f, b < c, b < d, b < g, f < g
fn select_1671([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_58([d, c, a, f, g, e])
    } else {
        select_5([e, f, b, d, g, c])
    }
}
/// n = 8, i = 3, a < e, b < g, b < h, c < d, c < f, c < g, f < h
fn select_1672([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_1671([a, c, d, f, e, b, g])
    } else {
        select_284([b, c, d, f, a, e, g, h])
    }
}
/// n = 10, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, g < h, g < i, g < j
fn select_1673([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_1055([a, b, c, e, d, g, h, i, j, f])
    } else {
        select_716([a, b, c, e, f, g, h, i, j, d])
    }
}
/// n = 10, i = 3, a < f, b < g, b < h, c < d, c < e, c < g, c < h, h < i, h < j
fn select_1674([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < h {
        select_1672([a, b, c, e, f, d, g, h])
    } else {
        select_1673([a, b, c, e, f, g, h, d, i, j])
    }
}
/// n = 13, i = 5, a < e, b < g, b < h, c < g, c < h, d < f, f < h, f < m, g < m, h < i, h < j, h < k, h < l
fn select_1675([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < h {
        select_355([a, b, c, d, e, f, g, m])
    } else {
        select_72([a, g, f, h, i, j, k, l, m])
    }
}
/// n = 14, i = 6, a < e, b < f, b < l, c < h, c < i, d < g, f < h, f < m, g < i, g < n, h < n, i < j, i < k, i < l, i < m
fn select_1676([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < f {
        select_1674([a, b, i, j, k, e, l, f, h, m])
    } else {
        select_1675([a, c, f, d, e, g, h, i, j, k, l, m, n])
    }
}
/// n = 10, i = 3, a < c, a < d, a < h, a < i, a < j, b < e, b < f, e < g, f < h, f < i, f < j
fn select_1677([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_310([f, g, a, c, d])
    } else {
        select_427([c, d, b, e, f, h, i, j, g])
    }
}
/// n = 11, i = 3, a < f, a < g, b < c, b < d, b < e, b < j, b < k, e < h, f < i, g < j, g < k
fn select_1678([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_1453([a, b, c, d, f, e, h, i, g])
    } else {
        select_1677([b, a, c, d, f, g, i, e, j, k])
    }
}
/// n = 9, i = 4, b < e, b < i, c < f, d < f, f < g, f < h, f < i
fn select_1679([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_678([a, b, f, g, h, i])
    } else {
        select_308([a, b, c, d, e, f, h, i])
    }
}
/// n = 8, i = 4, a < e, a < h, b < f, c < d, c < g, c < h, f < g, f < h
fn select_1680([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_476([c, b, a, e, f, g, d, h])
    } else {
        select_567([d, a, b, f, g, e, h])
    }
}
/// n = 9, i = 5, b < e, b < i, c < g, d < f, f < h, f < i, g < h, g < i
fn select_1681([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_1680([b, d, g, a, e, f, h, i])
    } else {
        select_355([b, a, c, d, e, f, g, h])
    }
}
/// n = 12, i = 6, a < f, a < h, b < e, b < i, c < h, c < i, d < g, g < i, g < l, h < l, i < j, i < k
fn select_1682([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_1679([e, a, c, g, f, i, j, k, h])
    } else {
        select_1681([f, b, c, d, e, g, h, l, i])
    }
}
/// n = 14, i = 6, a < e, b < f, b < h, c < h, c < i, d < g, f < m, f < n, g < i, g < l, h < l, i < j, i < k, i < m, i < n
fn select_1683([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_1678([b, i, j, k, a, h, f, e, l, m, n])
    } else {
        select_1682([b, a, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 14, i = 6, a < f, b < m, b < n, c < h, c < n, d < h, d < i, e < g, g < i, g < l, h < l, i < j, i < k, i < m, i < n
fn select_1684([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < c {
        select_1676([a, b, d, e, f, c, g, h, i, j, k, m, n, l])
    } else {
        select_1683([a, c, d, e, f, b, g, h, i, j, k, l, m, n])
    }
}
/// n = 9, i = 6, a < g, b < h, c < h, d < g, d < i, e < f, f < i, h < i
fn select_1685([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_240([a, d, h, e, f, g, i])
    } else {
        select_560([b, c, g, e, f, h])
    }
}
/// n = 9, i = 6, a < i, b < f, c < h, d < h, e < g, g < i, h < i
fn select_1686([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_1685([b, c, d, a, e, g, f, h, i])
    } else {
        select_182([c, d, e, f, a, g, h])
    }
}
/// n = 11, i = 6, a < f, b < h, b < k, c < i, d < h, d < i, e < g, g < i, i < j, i < k
fn select_1687([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_915([a, b, c, d, g, f, i, j, h, k])
    } else {
        select_1686([c, a, b, d, e, f, g, h, i])
    }
}
/// n = 11, i = 6, a < g, b < h, b < k, c < i, d < i, e < i, f < h, f < i, i < j, i < k
fn select_1688([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_1687([a, b, e, f, c, g, d, h, i, j, k])
    } else {
        select_1687([a, b, e, f, d, g, c, h, i, j, k])
    }
}
/// n = 6, i = 4, b < f, c < f, d < f, e < f
fn select_1689([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < a {
        select_9([a, c, d, e, f])
    } else {
        select_8([b, c, d, e])
    }
}
/// n = 10, i = 5, a < j, b < f, b < i, c < g, d < g, e < g, g < h, g < i, g < j
fn select_1690([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_176([a, b, g, h, f, i, j])
    } else {
        select_1689([a, c, d, e, f, g])
    }
}
/// n = 8, i = 5, a < h, b < e, c < g, d < f, f < h, g < h
fn select_1691([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_355([b, a, d, c, e, g, f, h])
    } else {
        select_580([b, c, f, e, a, g, h])
    }
}
/// n = 8, i = 5, a < h, b < h, c < h, d < f, e < g, g < h
fn select_1692([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_1691([c, d, a, e, f, g, b, h])
    } else {
        select_1691([c, d, b, e, f, g, a, h])
    }
}
/// n = 10, i = 5, a < g, b < f, b < j, c < h, d < h, e < h, h < i, h < j
fn select_1693([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_1690([a, b, c, d, e, f, h, i, j, g])
    } else {
        select_1692([c, d, e, b, a, f, g, h])
    }
}
/// n = 12, i = 6, a < l, b < g, c < i, d < i, e < i, f < h, f < i, h < k, i < j, i < k, i < l
fn select_1694([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < h {
        select_1688([b, a, c, d, e, f, g, h, i, j, l])
    } else {
        select_1693([b, h, c, d, e, a, g, i, j, k])
    }
}
/// n = 10, i = 5, b < j, c < g, d < g, e < f, e < h, e < j, f < i, g < h, g < i, g < j
fn select_1695([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < g {
        select_1113([a, b, c, d, e, f, g, h, j])
    } else {
        select_1639([a, e, g, b, f, h, i, j])
    }
}
/// n = 11, i = 6, b < i, c < h, d < h, e < f, e < g, f < k, g < j, g < k, h < i, h < j, h < k
fn select_1696([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_1695([a, f, c, d, g, b, h, j, i, k])
    } else {
        select_787([a, c, d, b, e, f, g, h, i, j])
    }
}
/// n = 9, i = 6, a < i, b < g, c < h, d < h, e < f, e < g, h < i
fn select_1697([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < i {
        select_388([a, f, c, d, b, g, h])
    } else {
        select_637([b, a, h, e, f, i])
    }
}
/// n = 10, i = 7, a < j, b < i, c < h, d < g, e < f, e < i, g < j, h < j
fn select_1698([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_1697([a, b, c, g, e, f, i, h, j])
    } else {
        select_1697([a, b, d, h, e, f, i, g, j])
    }
}
/// n = 8, i = 5, a < g, b < g, b < h, c < f, d < e, d < h, f < h
fn select_1699([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_1511([a, e, b, c, f, h, g])
    } else {
        select_1509([d, a, c, f, g, h, e])
    }
}
/// n = 9, i = 6, b < h, c < h, c < i, d < g, e < f, f < i, g < i
fn select_1700([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_1685([b, a, d, c, e, f, h, g, i])
    } else {
        select_1699([b, c, e, g, a, f, h, i])
    }
}
/// n = 7, i = 5, b < g, c < f, d < e
fn select_1701([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if e < f {
        select_637([a, c, e, b, g, f])
    } else {
        select_637([a, d, f, b, g, e])
    }
}
/// n = 9, i = 6, b < i, c < f, d < h, e < g, g < i, h < i
fn select_1702([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_1700([a, c, b, d, e, g, h, f, i])
    } else {
        select_1701([a, d, e, f, b, g, h])
    }
}
/// n = 10, i = 7, b < j, c < i, d < h, e < f, e < g, h < j, i < j
fn select_1703([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_1698([b, a, c, d, e, g, h, i, f, j])
    } else {
        select_1702([g, b, f, c, d, a, h, i, j])
    }
}
/// n = 12, i = 7, b < f, c < j, d < i, e < g, e < h, g < l, h < k, h < l, i < j, j < k, j < l
fn select_1704([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < f {
        select_1696([a, b, c, i, e, g, h, j, f, k, l])
    } else {
        select_1703([a, c, b, d, e, g, h, i, f, j])
    }
}
/// n = 14, i = 7, a < n, b < f, c < j, c < k, d < h, d < i, e < g, e < k, g < j, h < k, i < k, i < m, j < m, k < l, k < n
fn select_1705([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < j {
        select_1694([a, b, c, h, i, e, f, g, k, l, j, n])
    } else {
        select_1704([a, b, c, e, d, f, h, i, g, j, m, k])
    }
}
/// n = 15, i = 7, a < g, b < n, b < o, c < j, c < k, d < h, d < k, e < f, e < i, h < j, h < o, i < k, i < m, j < m, k < l, k < n, k < o
fn select_1706([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < f {
        select_1684([a, b, h, c, e, g, i, j, k, f, l, m, n, o])
    } else {
        select_1705([b, a, c, e, d, g, h, f, i, j, k, l, m, n])
    }
}
/// n = 16, i = 7, a < g, b < k, b < p, c < j, c < m, d < f, d < i, e < h, e < m, h < j, h < k, i < m, i < o, j < o, k < l, m < n, m < p
fn select_1707([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < m {
        select_1670([a, b, c, d, e, f, g, i, h, j, k, l, o, m])
    } else {
        select_1706([a, b, c, e, d, f, g, h, i, j, m, n, o, p, k])
    }
}
/// n = 16, i = 7, a < g, b < k, b < p, c < h, c < n, d < i, d < n, e < f, e < j, h < l, i < k, j < l, j < n, k < m, n < o, n < p
fn select_1708([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < i {
        select_1637([a, b, d, e, c, f, g, h, j, i, l, n, o, k, m, p])
    } else {
        select_1707([a, b, c, e, d, f, g, i, j, h, k, m, n, o, l, p])
    }
}
/// n = 16, i = 7, a < h, b < l, b < p, c < g, c < n, d < i, d < n, e < f, e < j, i < l, j < k, j < n, l < m, n < o, n < p
fn select_1709([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < g {
        select_1621([a, b, c, d, e, f, h, i, j, k, l, g, m, n, o, p])
    } else {
        select_1708([a, b, c, d, e, f, h, g, i, j, l, k, m, n, o, p])
    }
}
/// n = 9, i = 3, c < i, d < e, e < f, e < g, e < i, g < h
fn select_1710([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < c {
        select_571([b, a, d, c, e, f, g, h])
    } else {
        select_107([b, c, d, a, e, f, g, h, i])
    }
}
/// n = 9, i = 4, c < g, c < i, d < f, d < g, e < f, e < g, f < i, g < h
fn select_1711([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1372([a, c, d, e, b, f, g, h, i])
    } else {
        select_1372([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 8, i = 3, c < e, d < e, e < f, e < g, e < h
fn select_1712([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_517([a, c, d, b, e, f, g, h])
    } else {
        select_517([b, c, d, a, e, f, g, h])
    }
}
/// n = 11, i = 5, c < g, c < h, d < g, d < h, e < f, f < h, f < j, f < k, g < j, g < k, h < i
fn select_1713([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_1712([a, b, e, g, f, h, j, k])
    } else {
        select_383([a, b, c, d, f, h, g, j, k, i])
    }
}
/// n = 11, i = 5, c < g, c < i, d < g, d < h, d < i, e < f, f < h, f < i, f < k, g < k, i < j
fn select_1714([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_1711([a, b, c, d, f, h, i, j, g])
    } else {
        select_1713([a, b, c, d, e, f, g, i, j, h, k])
    }
}
/// n = 12, i = 5, b < l, c < g, c < i, d < g, d < h, d < i, e < f, e < i, f < h, f < k, g < k, i < j, i < l
fn select_1715([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_1710([a, g, b, e, i, j, f, h, l])
    } else {
        select_1714([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 9, i = 5, c < g, d < g, e < f, f < h, f < i, g < h, g < i
fn select_1716([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_163([b, e, g, a, f, h, i])
    } else {
        select_1097([b, a, c, d, e, f, g])
    }
}
/// n = 11, i = 6, b < h, c < i, d < f, e < g, f < i, g < h, g < j, g < k, i < j, i < k
fn select_1717([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_1716([a, b, c, f, e, g, i, j, k])
    } else {
        select_388([a, c, b, g, d, f, h])
    }
}
/// n = 8, i = 5, c < g, d < g, e < f, f < h, g < h
fn select_1718([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < a {
        select_53([b, e, g, a, f, h])
    } else {
        select_1097([b, a, c, d, e, f, g])
    }
}
/// n = 9, i = 5, b < i, c < g, d < g, d < h, e < f, f < h, h < i
fn select_1719([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_742([a, b, c, d, f, h, g, i])
    } else {
        select_1718([a, b, c, d, e, f, g, h])
    }
}
/// n = 11, i = 6, b < h, c < i, d < f, e < g, f < i, f < j, g < h, g < j, g < k, i < k
fn select_1720([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < j {
        select_1717([a, b, c, d, e, f, g, h, i, j, k])
    } else {
        select_1719([a, c, b, g, d, f, h, j, i])
    }
}
/// n = 9, i = 5, b < i, c < g, c < i, d < g, d < i, e < f, f < h, f < i, g < h
fn select_1721([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_791([a, c, d, f, b, g, h, i])
    } else {
        select_524([a, b, e, c, d, g, f, i])
    }
}
/// n = 8, i = 5, b < h, c < g, c < h, d < g, d < h, e < f, f < h
fn select_1722([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_575([c, d, e, b, a, f, g, h])
    } else {
        select_182([c, d, a, e, f, b, g])
    }
}
/// n = 11, i = 6, b < h, b < k, c < i, c < k, d < f, e < g, f < i, f < k, g < h, g < j, g < k, i < j
fn select_1723([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_1721([a, b, c, f, e, g, i, j, k])
    } else {
        select_1722([a, c, b, g, d, f, h, k])
    }
}
/// n = 12, i = 6, b < f, c < i, c < l, d < g, d < l, e < h, f < l, g < j, g < k, h < i, h < j, h < l, i < k
fn select_1724([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_1720([a, b, c, e, d, h, g, f, i, j, k])
    } else {
        select_1723([a, c, d, b, e, f, h, i, g, j, l])
    }
}
/// n = 13, i = 6, b < f, c < i, c < k, d < g, d < k, e < h, g < j, g < m, h < i, h < j, h < k, i < m, k < l
fn select_1725([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if k < f {
        select_1715([a, b, c, h, d, g, i, j, k, l, m, f])
    } else {
        select_1724([a, b, c, d, e, f, g, h, i, j, m, k])
    }
}
/// n = 9, i = 4, a < g, b < h, b < i, c < f, d < e, d < h, d < i, e < g, f < h, f < i
fn select_1726([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_1096([b, d, f, a, e, h, i])
    } else {
        select_1652([b, c, a, d, e, f, g])
    }
}
/// n = 10, i = 4, a < j, b < g, c < f, d < e, d < h, d < j, e < g, f < h, f < j, h < i
fn select_1727([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_371([a, b, c, e, g, f, h])
    } else {
        select_989([b, a, d, f, e, h, i, j])
    }
}
/// n = 12, i = 5, a < i, b < g, c < h, c < l, d < f, d < h, d < l, e < f, e < g, e < j, f < i, g < l, h < j, j < k
fn select_1728([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_1726([a, c, b, d, f, g, i, h, l])
    } else {
        select_1727([b, a, c, e, f, h, i, j, k, g])
    }
}
/// n = 8, i = 4, a < g, b < g, b < h, c < f, c < g, c < h, d < e, e < f, e < h
fn select_1729([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_476([c, d, b, a, e, f, g, h])
    } else {
        select_1509([c, a, d, e, b, f, g])
    }
}
/// n = 11, i = 6, a < g, b < h, c < g, c < j, d < h, d < k, e < f, f < i, f < j, f < k, g < k, h < i, h < j
fn select_1730([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_1729([a, c, h, e, f, i, g, j])
    } else {
        select_240([b, d, g, e, f, h, k])
    }
}
/// n = 8, i = 4, a < f, a < g, b < e, c < d, d < f, d < h, e < g, e < h
fn select_1731([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_25([a, c, g, d, h])
    } else {
        select_25([a, b, f, e, h])
    }
}
/// n = 9, i = 5, a < f, a < h, b < g, b < h, c < f, c < g, d < e, e < h, e < i, g < i
fn select_1732([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_1731([a, c, d, e, g, h, f, i])
    } else {
        select_1509([b, c, d, e, f, h, g])
    }
}
/// n = 9, i = 5, a < g, b < g, b < i, c < f, d < e, d < i, e < h, f < h, f < i
fn select_1733([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_1732([b, d, a, c, f, g, e, i, h])
    } else {
        select_452([b, c, e, a, f, h, i, g])
    }
}
/// n = 11, i = 6, a < j, a < k, b < h, c < h, c < i, d < f, e < g, f < i, f < j, g < i, g < j, g < k, h < k
fn select_1734([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_1730([b, d, c, a, e, g, h, f, j, i, k])
    } else {
        select_1733([b, c, e, f, a, g, h, j, i])
    }
}
/// n = 7, i = 2, a < g, b < d, c < e, c < g, e < f
fn select_1735([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < a {
        select_317([c, b, a, d, e, f])
    } else {
        select_209([a, c, e, b, f, d, g])
    }
}
/// n = 9, i = 4, a < i, b < e, b < g, c < f, c < h, d < f, d < h, e < h, f < g, f < i
fn select_1736([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_878([a, b, c, d, e, f, g, i])
    } else {
        select_126([e, c, d, a, f, g, h])
    }
}
/// n = 11, i = 5, a < h, b < f, c < f, c < i, d < g, d < k, e < g, e < k, f < k, g < h, g < i, i < j
fn select_1737([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_1735([b, a, c, h, i, j, f])
    } else {
        select_1736([a, c, d, e, f, g, i, k, h])
    }
}
/// n = 13, i = 6, a < h, b < f, c < i, c < m, d < h, d < j, d < k, e < g, f < j, g < i, g < j, g < m, h < m, i < k, k < l
fn select_1738([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < i {
        select_1734([c, a, d, b, e, f, g, h, j, i, m])
    } else {
        select_1737([b, a, d, c, g, h, i, f, k, l, m])
    }
}
/// n = 13, i = 6, a < f, b < h, c < j, c < m, d < h, d < i, d < k, e < g, g < i, g < j, g < m, h < m, j < k, k < l
fn select_1739([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_1728([a, b, c, g, d, i, h, j, f, k, l, m])
    } else {
        select_1738([b, a, c, d, e, f, g, h, j, i, k, l, m])
    }
}
/// n = 10, i = 4, a < e, a < g, a < j, b < e, b < h, b < j, c < d, d < g, d < h, d < j, e < f, h < i
fn select_1740([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_42([a, b, g, h, i, j])
    } else {
        select_474([c, b, e, d, g, f, j])
    }
}
/// n = 11, i = 5, a < f, b < e, b < i, c < d, c < i, c < k, d < g, e < h, e < k, f < h, f < i, f < k, i < j
fn select_1741([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_497([d, b, a, g, f, e, i, j])
    } else {
        select_1740([e, c, a, f, d, g, h, i, j, k])
    }
}
/// n = 11, i = 5, a < e, b < i, b < k, c < g, d < f, d < i, f < h, f < k, g < h, g < i, g < k, i < j
fn select_1742([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < a {
        select_1741([c, d, b, a, f, g, e, h, i, j, k])
    } else {
        select_849([c, d, a, e, b, f, g, h, i])
    }
}
/// n = 11, i = 5, a < e, b < i, b < k, c < f, c < i, d < g, f < k, g < h, g < i, g < k, i < j
fn select_1743([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_1374([a, b, c, g, e, h, f, i, j])
    } else {
        select_1742([a, b, d, c, e, f, g, h, i, j, k])
    }
}
/// n = 10, i = 4, a < e, b < h, c < g, c < h, d < f, d < h, f < g, g < i, h < j
fn select_1744([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_109([a, c, f, e, b, g, i, h])
    } else {
        select_1372([a, c, b, d, e, f, h, j, g])
    }
}
/// n = 8, i = 3, a < d, a < h, b < e, b < f, b < h, c < e, c < f, c < h, f < g
fn select_1745([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_122([e, a, f, g, h])
    } else {
        select_75([b, c, a, d, e, h])
    }
}
/// n = 10, i = 3, a < d, b < e, b < f, b < g, c < e, c < f, c < g, f < j, g < h, g < i
fn select_1746([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_173([e, f, g, a, h, i, j])
    } else {
        select_1745([a, b, c, d, e, f, j, g])
    }
}
/// n = 13, i = 5, a < e, b < g, b < j, c < g, c < j, d < f, d < h, d < i, f < j, f < m, g < h, g < i, g < m, i < k, j < l
fn select_1747([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_1746([a, d, g, e, h, i, f, j, m, k])
    } else {
        select_1628([a, b, c, f, e, j, g, h, i, m, l])
    }
}
/// n = 13, i = 5, a < e, b < h, b < j, c < g, c < h, c < j, d < f, d < g, d < i, f < j, f < m, h < i, h < m, i < k, j < l
fn select_1748([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_1744([a, f, b, c, e, g, h, j, i, l])
    } else {
        select_1747([a, b, c, d, e, f, h, g, i, j, k, l, m])
    }
}
/// n = 14, i = 6, a < e, b < j, b < l, c < g, d < f, d < i, d < k, f < h, f < l, g < i, g < j, g < l, j < k, k < m, l < n
fn select_1749([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_1743([a, b, f, c, e, h, g, i, l, n, j])
    } else {
        select_1748([a, b, g, d, e, f, i, j, k, l, m, n, h])
    }
}
/// n = 14, i = 6, b < f, c < j, c < l, d < g, d < i, d < k, e < h, g < l, h < i, h < j, h < l, j < k, k < m, l < n
fn select_1750([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < g {
        select_1739([b, a, c, d, e, f, h, g, i, j, k, m, l])
    } else {
        select_1749([b, c, e, d, f, g, h, a, i, j, k, l, m, n])
    }
}
/// n = 14, i = 6, b < g, c < k, d < i, d < m, e < j, e < k, f < h, h < i, h < j, h < k, j < m, k < l, m < n
fn select_1751([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if c < d {
        select_1725([a, b, e, c, f, g, d, h, j, i, k, l, m])
    } else {
        select_1750([a, b, e, d, f, g, c, h, i, j, m, k, n, l])
    }
}
/// n = 10, i = 3, a < d, b < h, b < i, c < e, c < h, c < i, e < f, e < g, i < j
fn select_1752([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_116([a, e, f, g, d, b, h, i])
    } else {
        select_1746([a, b, c, d, h, i, e, f, g, j])
    }
}
/// n = 9, i = 3, a < d, b < f, b < g, b < i, c < e, c < f, c < g, e < i, g < h
fn select_1753([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_59([a, e, b, f, g, h, i])
    } else {
        select_109([a, b, c, d, e, g, h, i])
    }
}
/// n = 11, i = 5, a < f, b < h, b < j, b < k, c < g, c < h, d < g, d < h, e < g, e < h, g < j, g < k, h < i
fn select_1754([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_641([a, g, b, h, i, j, k, f])
    } else {
        select_404([b, a, c, d, e, f, h, i, g])
    }
}
/// n = 12, i = 5, a < f, b < j, c < h, c < i, c < j, d < g, d < j, e < g, e < j, g < h, g < i, i < k, j < l
fn select_1755([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_1753([a, c, g, f, b, h, i, k, j])
    } else {
        select_1754([a, c, b, d, e, f, g, j, l, h, i])
    }
}
/// n = 12, i = 5, b < f, c < h, c < j, c < k, d < g, d < h, e < g, e < h, g < j, g < k, h < i, k < l
fn select_1756([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_1752([b, g, c, f, h, a, i, j, k, l])
    } else {
        select_1755([b, a, c, d, e, f, g, j, k, h, l, i])
    }
}
/// n = 10, i = 5, a < e, b < h, b < j, c < g, d < f, d < h, f < j, g < h, g < j, h < i
fn select_1757([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_109([a, b, d, e, f, h, i, j])
    } else {
        select_373([e, b, c, d, f, g, h])
    }
}
/// n = 10, i = 5, c < h, c < j, d < g, e < f, e < h, f < j, g < h, g < j, h < i
fn select_1758([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1757([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_1757([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 11, i = 3, a < d, b < e, b < f, c < e, c < f, e < j, e < k, f < g, f < h, f < i
fn select_1759([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_329([e, f, g, h, i, a, j, k])
    } else {
        select_1058([a, b, c, d, f, g, h, i, e])
    }
}
/// n = 11, i = 3, a < d, b < h, b < k, c < e, c < h, e < f, e < g, e < k, h < i, h < j
fn select_1760([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_116([a, e, f, g, d, b, h, k])
    } else {
        select_1759([a, b, c, d, h, e, f, g, k, i, j])
    }
}
/// n = 10, i = 4, a < d, b < f, c < e, c < g, e < j, f < g, f < j, g < h, g < i
fn select_1761([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_53([d, b, c, e, f, g])
    } else {
        select_115([a, c, d, e, g, h, i, j])
    }
}
/// n = 12, i = 5, a < e, b < h, c < g, d < f, d < h, f < i, g < h, g < i, h < l, i < j, i < k
fn select_1762([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < f {
        select_718([a, b, d, c, e, g, f, h, l, i])
    } else {
        select_1761([a, c, f, e, b, g, i, j, k, h])
    }
}
/// n = 13, i = 5, b < e, c < g, d < f, d < h, f < j, f < m, g < h, g < j, h < i, h < m, j < k, j < l
fn select_1763([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_1760([b, f, g, e, h, a, i, j, k, l, m])
    } else {
        select_1762([b, a, c, d, e, f, g, h, j, k, l, i])
    }
}
/// n = 14, i = 6, b < f, c < i, c < k, d < h, e < g, e < i, g < k, g < n, h < i, h < k, i < j, i < n, k < l, k < m
fn select_1764([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < c {
        select_1758([a, f, c, d, e, g, h, i, j, k])
    } else {
        select_1763([a, b, d, e, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 6, b < f, c < i, c < j, d < g, d < j, d < l, e < h, g < m, g < n, h < i, h < j, i < l, i < n, j < k, j < m
fn select_1765([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_1756([a, b, d, c, h, f, i, j, k, l, g, n])
    } else {
        select_1764([a, b, c, e, d, f, g, h, j, k, i, l, n, m])
    }
}
/// n = 9, i = 3, a < e, b < f, b < g, c < d, c < f, c < g, f < i, g < h
fn select_1766([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_109([a, b, c, e, d, g, h, f])
    } else {
        select_1204([a, b, c, e, g, f, d, i, h])
    }
}
/// n = 11, i = 4, a < e, b < f, b < g, c < f, c < g, d < f, d < g, f < j, f < k, g < h, g < i
fn select_1767([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1595([a, b, c, d, e, f, g, j, k])
    } else {
        select_1595([a, b, c, d, e, g, f, h, i])
    }
}
/// n = 13, i = 5, a < e, b < f, b < i, c < g, c < j, d < g, d < j, f < j, g < h, g < i, i < k, j < l, j < m
fn select_1768([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_1766([a, b, g, h, e, f, i, k, j])
    } else {
        select_1767([a, c, d, f, e, g, j, l, m, h, i])
    }
}
/// n = 10, i = 6, a < i, a < j, b < h, c < f, d < h, d < j, e < g, f < j, g < i, g < j, h < i
fn select_1769([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_1094([a, b, d, f, e, g, h, j])
    } else {
        select_158([a, c, h, e, g, f, i])
    }
}
/// n = 11, i = 6, a < f, b < i, b < k, c < h, c < j, d < h, d < i, e < g, g < i, g < k, h < k, i < j
fn select_1770([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < f {
        select_1736([a, c, b, g, h, i, j, k, f])
    } else {
        select_1769([b, c, a, d, e, f, g, h, k, i])
    }
}
/// n = 11, i = 5, a < e, b < h, b < k, c < g, d < f, d < h, f < k, g < h, g < k, h < i, h < j
fn select_1771([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_373([e, b, c, d, f, g, h])
    } else {
        select_1761([a, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 13, i = 6, a < f, b < i, c < i, c < j, d < g, d < m, e < h, g < j, h < i, h < j, i < m, j < k, j < l
fn select_1772([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < g {
        select_1770([a, c, d, b, e, f, h, g, i, m, j])
    } else {
        select_1771([a, c, e, g, f, b, h, j, k, l, i])
    }
}
/// n = 14, i = 6, b < f, c < i, c < k, d < g, d < j, e < h, g < k, h < i, h < k, i < j, j < l, k < m, k < n
fn select_1773([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < a {
        select_1768([b, d, c, h, f, g, i, a, j, k, l, m, n])
    } else {
        select_1772([b, a, c, d, e, f, g, h, i, k, m, n, j])
    }
}
/// n = 15, i = 6, b < f, c < i, c < k, d < g, d < j, e < h, g < k, g < o, h < i, h < k, i < j, i < o, j < l, k < m, k < n
fn select_1774([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if o < b {
        select_1586([a, i, g, j, k, m, n, l, o])
    } else {
        select_1773([a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 15, i = 6, b < g, c < m, c < o, d < j, d < l, e < i, e < j, f < h, h < i, h < j, i < l, i < m, j < k, j < o, m < n
fn select_1775([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if d < c {
        select_1765([a, b, e, d, f, g, c, h, i, j, k, l, o, m])
    } else {
        select_1774([a, b, e, c, f, g, d, h, i, m, j, n, k, o, l])
    }
}
/// n = 15, i = 6, b < g, c < k, c < o, d < j, d < l, e < i, e < k, f < h, f < l, h < j, h < k, i < l, k < n, l < m, l < o
fn select_1776([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_1751([a, b, c, d, e, f, g, h, j, i, k, n, l, m])
    } else {
        select_1775([a, b, c, d, f, e, g, i, h, l, m, j, k, n, o])
    }
}
/// n = 9, i = 4, a < d, b < e, c < f, d < i, e < g, e < h, f < g, f < h, f < i
fn select_1777([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_52([b, d, c, f, e, i])
    } else {
        select_255([a, e, c, f, d, g, h])
    }
}
/// n = 9, i = 5, a < i, b < g, b < h, b < i, c < f, d < e, d < i, e < h, f < g, f < i
fn select_1778([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_214([a, d, b, g, e, h, i])
    } else {
        select_53([a, c, b, h, f, g])
    }
}
/// n = 12, i = 6, a < i, b < f, c < h, c < i, c < j, d < e, d < i, e < h, f < g, f < l, g < j, h < k, i < k, i < l
fn select_1779([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_1777([e, b, a, h, f, i, g, l, k])
    } else {
        select_1778([a, c, f, d, e, g, j, h, i])
    }
}
/// n = 9, i = 4, a < d, b < f, c < e, c < g, d < i, e < h, e < i, f < g, f < h
fn select_1780([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_52([b, d, c, e, f, i])
    } else {
        select_291([a, f, c, e, g, d, h])
    }
}
/// n = 10, i = 4, a < e, b < d, b < i, c < f, d < j, e < j, f < g, f < i, g < h
fn select_1781([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_151([a, b, f, e, g, h, i, d])
    } else {
        select_1780([a, c, b, e, d, f, i, g, j])
    }
}
/// n = 12, i = 6, a < l, b < f, c < h, c < i, c < j, d < e, d < h, e < i, f < g, f < k, g < j, h < k, h < l, i < l
fn select_1782([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < h {
        select_1779([a, b, c, d, e, f, g, i, h, j, l, k])
    } else {
        select_1781([e, h, b, a, i, f, g, j, k, l])
    }
}
/// n = 11, i = 5, a < j, a < k, b < g, b < i, c < f, d < e, d < h, e < i, f < g, f < h, h < j, i < k
fn select_1783([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_320([a, b, d, g, h, e, i, j])
    } else {
        select_1009([a, c, b, i, f, h, g, k])
    }
}
/// n = 12, i = 5, a < k, a < l, b < i, c < f, d < e, d < h, e < i, e < j, f < g, f < h, g < j, h < k, i < l
fn select_1784([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_320([a, g, d, b, h, e, j, k])
    } else {
        select_1783([a, b, c, d, e, f, g, h, i, k, l])
    }
}
/// n = 13, i = 6, a < l, a < m, b < i, b < j, c < g, d < e, d < f, e < i, f < k, f < m, g < h, g < k, h < j, i < m, k < l
fn select_1785([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < f {
        select_1782([a, c, b, d, e, g, h, f, i, j, k, m])
    } else {
        select_1784([a, e, c, f, b, g, h, k, i, j, l, m])
    }
}
/// n = 7, i = 3, a < e, a < g, b < c, b < d, d < g, e < f
fn select_1786([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < e {
        select_11([c, d, a, g, e])
    } else {
        select_42([e, b, f, d, g, c])
    }
}
/// n = 10, i = 5, a < e, a < i, b < d, b < h, c < f, c < j, d < i, d < j, e < h, e < j, f < g, f < i
fn select_1787([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_708([g, a, b, d, e, h, i])
    } else {
        select_1786([c, b, h, d, f, g, j])
    }
}
/// n = 10, i = 5, a < d, a < j, b < e, b < i, c < f, c < h, d < i, e < h, e < j, f < g, f < j
fn select_1788([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < h {
        select_1787([a, b, c, e, d, f, g, i, j, h])
    } else {
        select_108([a, f, e, g, h, d, j])
    }
}
/// n = 7, i = 3, a < f, b < f, b < g, c < d, c < e, e < g
fn select_1789([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < e {
        select_15([b, a, c, d, e, f])
    } else {
        select_82([d, b, e, a, g])
    }
}
/// n = 10, i = 5, a < e, a < j, b < d, b < h, c < f, c < i, d < j, e < i, f < g, f < h
fn select_1790([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_1788([b, a, c, d, e, f, g, i, j, h])
    } else {
        select_1789([d, a, f, g, h, j, e])
    }
}
/// n = 8, i = 4, a < g, b < e, c < d, c < g, d < h, e < f, g < h
fn select_1791([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_642([b, a, d, g, e, f, h])
    } else {
        select_15([a, c, e, f, d, g])
    }
}
/// n = 9, i = 5, a < h, b < e, c < g, d < g, e < f, g < h, h < i
fn select_1792([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_642([a, b, g, e, h, i, f])
    } else {
        select_9([a, c, d, f, g])
    }
}
/// n = 11, i = 5, a < g, b < h, b < i, c < e, d < g, d < h, e < f, e < h, g < i, h < j, i < k
fn select_1793([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_1792([b, c, a, d, e, f, g, i, k])
    } else {
        select_368([a, f, b, d, h, j])
    }
}
/// n = 11, i = 5, a < i, a < k, b < e, c < h, c < i, d < f, e < h, f < g, f < i, h < k, i < j
fn select_1794([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_1791([c, d, e, a, f, g, h, k])
    } else {
        select_1793([b, c, d, a, f, g, e, i, h, j, k])
    }
}
/// n = 11, i = 5, a < g, a < h, b < f, c < e, d < g, d < h, d < k, e < g, e < k, f < h, f < k, g < i, h < j
fn select_1795([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_642([a, c, d, e, h, j, k])
    } else {
        select_642([a, b, d, f, g, i, k])
    }
}
/// n = 13, i = 6, a < i, a < j, a < m, b < e, c < g, d < f, e < i, e < j, f < h, f < i, g < h, g < j, h < m, i < k, j < l
fn select_1796([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < h {
        select_1795([a, c, d, e, f, g, i, j, k, l, h])
    } else {
        select_11([a, b, h, e, m])
    }
}
/// n = 9, i = 4, a < g, a < h, b < e, c < d, c < i, d < g, d < h, e < f, f < i
fn select_1797([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_82([a, f, c, d, i])
    } else {
        select_317([b, a, g, h, e, f])
    }
}
/// n = 13, i = 6, a < e, a < j, b < h, b < k, b < l, c < f, d < g, e < k, f < i, f < k, g < h, g < i, h < j, i < l, k < m
fn select_1798([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < h {
        select_1796([b, a, c, d, e, g, f, i, h, k, j, m, l])
    } else {
        select_1797([a, c, b, h, f, i, e, j, l])
    }
}
/// n = 14, i = 6, a < i, a < l, a < m, b < g, c < e, c < k, d < f, d < h, e < h, e < l, f < j, f < l, g < i, g < j, i < k, j < m, l < n
fn select_1799([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_1794([a, b, f, c, g, e, h, j, l, n, m])
    } else {
        select_1798([c, a, d, b, e, f, g, i, j, k, l, m, n])
    }
}
/// n = 14, i = 6, a < l, a < m, b < g, c < e, c < j, d < f, d < i, e < i, e < l, f < k, f < l, g < h, g < k, h < j, k < m, l < n
fn select_1800([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < a {
        select_1790([d, c, g, e, f, h, a, j, k, i])
    } else {
        select_1799([a, b, c, d, e, f, g, i, h, k, j, l, m, n])
    }
}
/// n = 14, i = 6, a < l, a < m, b < f, b < j, c < h, d < e, d < g, f < l, g < k, g < l, h < i, h < k, i < j, k < m, l < n
fn select_1801([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < f {
        select_1785([a, b, c, d, e, g, h, i, f, j, k, m, l])
    } else {
        select_1800([a, c, b, d, f, g, h, i, e, j, k, l, m, n])
    }
}
/// n = 10, i = 5, a < i, b < g, b < i, c < e, d < f, e < g, e < h, f < h, f < i, i < j
fn select_1802([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_1521([b, c, d, e, f, g, i, j, h])
    } else {
        select_966([a, b, d, g, f, h])
    }
}
/// n = 11, i = 5, a < i, a < j, b < g, b < i, c < f, d < e, d < h, e < i, f < g, f < h, h < j, i < k
fn select_1803([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_1802([a, b, c, d, f, e, g, h, i, k])
    } else {
        select_184([a, b, d, g, h, e, j])
    }
}
/// n = 9, i = 4, a < i, b < f, b < i, c < f, c < g, c < i, d < e, d < h, d < i, e < g, f < h
fn select_1804([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_82([a, b, c, g, f])
    } else {
        select_217([a, c, d, e, f, h, g, i])
    }
}
/// n = 12, i = 5, a < h, b < g, b < h, c < f, c < l, d < e, d < h, d < k, e < i, f < g, f < i, g < k, h < j, h < l
fn select_1805([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_409([e, c, h, j, f, l])
    } else {
        select_1804([a, b, f, d, e, g, i, k, h])
    }
}
/// n = 13, i = 5, a < h, b < i, b < m, c < i, c < k, d < g, d < i, e < f, e < l, f < g, f < h, g < k, h < m, i < j, i < l
fn select_1806([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < c {
        select_1803([b, d, e, a, c, f, g, h, i, m, j])
    } else {
        select_1805([b, d, e, c, a, f, g, i, h, j, k, l])
    }
}
/// n = 8, i = 3, a < h, b < d, b < f, c < e, c < f, e < h, f < g
fn select_1807([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_58([a, d, c, f, g, e])
    } else {
        select_461([a, e, b, f, g, d, h])
    }
}
/// n = 8, i = 2, a < h, b < g, b < h, c < d, c < g, d < e, d < f
fn select_1808([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < b {
        select_34([a, d, b, e, f])
    } else {
        select_154([a, c, b, d, g, h])
    }
}
/// n = 12, i = 4, a < f, a < k, b < f, b < h, c < d, c < l, d < e, d < h, e < k, f < g, f < l, h < i, h < j
fn select_1809([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < d {
        select_19([g, c, d, l])
    } else {
        select_1808([e, a, b, h, i, j, f, k])
    }
}
/// n = 13, i = 5, a < i, a < l, b < e, b < m, c < d, c < h, d < i, d < k, e < f, e < h, f < g, f < k, h < l, i < j, i < m
fn select_1810([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < d {
        select_1807([a, f, c, g, h, d, k, l])
    } else {
        select_1809([a, d, b, e, h, i, j, f, g, k, l, m])
    }
}
/// n = 11, i = 5, a < f, a < g, b < e, c < d, c < k, d < f, d < j, e < g, e < j, f < i, g < h, g < k
fn select_1811([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < g {
        select_474([b, a, f, e, j, i, g])
    } else {
        select_19([h, c, d, k])
    }
}
/// n = 12, i = 5, a < h, b < e, c < d, c < l, d < f, d < i, e < h, e < i, e < k, f < g, f < k, h < j, h < l
fn select_1812([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_479([b, f, a, g, e, i, k])
    } else {
        select_1811([a, b, c, d, e, f, h, j, g, i, l])
    }
}
/// n = 14, i = 6, a < j, b < g, b < h, c < g, c < i, d < e, d < n, e < f, e < l, f < h, f < i, g < j, g < l, i < k, j < m, j < n
fn select_1813([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_106([a, b, c, h, i, k, g])
    } else {
        select_1812([a, c, d, e, g, f, h, j, l, m, i, n])
    }
}
/// n = 15, i = 6, a < k, a < n, b < j, b < k, c < e, c < h, d < f, d < o, e < i, e < k, f < g, f < i, g < h, g < j, i < n, j < l, k < m, k < o
fn select_1814([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < b {
        select_1810([a, d, e, b, f, g, h, i, k, m, j, n, o])
    } else {
        select_1813([a, c, b, d, f, g, e, h, j, k, l, i, m, o])
    }
}
/// n = 15, i = 6, a < k, a < n, b < j, b < k, c < g, c < o, d < e, d < f, f < i, f < k, g < h, g < i, h < j, i < n, j < l, k < m, k < o
fn select_1815([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < h {
        select_1806([e, b, a, f, c, g, i, h, k, m, n, o, j])
    } else {
        select_1814([a, b, d, c, f, g, h, e, i, j, k, l, m, n, o])
    }
}
/// n = 8, i = 3, a < d, a < g, a < h, b < c, b < e, d < f, e < g, e < h
fn select_1816([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_154([c, e, a, g, h, d])
    } else {
        select_44([d, b, f, e, g, h, c])
    }
}
/// n = 7, i = 3, a < g, b < d, b < g, c < e, c < g, e < f
fn select_1817([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_11([a, d, c, e, g])
    } else {
        select_22([a, b, e, f, d, g])
    }
}
/// n = 9, i = 4, a < e, a < g, b < f, b < g, c < d, c < f, c < i, f < h, g < i
fn select_1818([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_1816([c, a, e, f, g, h, d, i])
    } else {
        select_1817([d, a, b, e, f, h, g])
    }
}
/// n = 9, i = 4, b < f, b < g, c < e, c < g, c < i, d < e, d < f, d < h, f < i, g < h
fn select_1819([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_163([a, b, c, e, f, g, i])
    } else {
        select_163([a, b, d, e, g, f, h])
    }
}
/// n = 10, i = 4, b < g, b < j, c < f, c < g, c < j, d < e, d < f, d < i, e < h, e < j, f < h, g < i
fn select_1820([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_709([a, e, c, f, b, g, j, h])
    } else {
        select_1819([a, b, c, d, f, e, g, i, j])
    }
}
/// n = 12, i = 5, a < f, a < l, b < d, b < g, c < e, c < h, c < i, e < k, e < l, f < i, g < h, g < l, h < k, i < j
fn select_1821([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_1818([b, f, c, e, d, i, g, j, h])
    } else {
        select_1820([d, a, g, c, e, h, f, k, i, l])
    }
}
/// n = 9, i = 4, a < e, b < h, b < i, c < f, d < f, f < g, f < h, f < i
fn select_1822([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_172([g, a, b, h, i, e])
    } else {
        select_368([b, e, c, d, f, g])
    }
}
/// n = 9, i = 4, c < h, c < i, d < f, e < f, f < g, f < h, f < i
fn select_1823([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1822([a, c, d, e, b, f, g, h, i])
    } else {
        select_1822([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 9, i = 5, a < e, a < f, b < g, c < g, d < f, d < h, g < h, h < i
fn select_1824([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_126([a, d, g, e, h, i, f])
    } else {
        select_9([e, b, c, f, g])
    }
}
/// n = 10, i = 5, b < f, b < j, c < g, d < g, e < f, e < h, e < j, g < h, g < j, h < i
fn select_1825([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < g {
        select_740([a, c, d, e, f, g, h, j])
    } else {
        select_659([a, b, e, h, i, f, j])
    }
}
/// n = 13, i = 6, a < f, a < m, b < e, b < g, c < h, d < h, f < j, g < i, g < m, h < i, h < j, h < m, i < k, j < l
fn select_1826([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_1824([b, c, d, f, e, g, h, j, l])
    } else {
        select_1825([e, a, c, d, g, f, h, i, k, m])
    }
}
/// n = 15, i = 6, a < h, a < o, b < f, b < i, c < e, c < g, d < h, d < i, f < l, g < i, g < k, h < k, h < l, i < j, i < o, k < m, l < n
fn select_1827([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_1823([e, f, a, d, g, i, j, h, o])
    } else {
        select_1826([b, c, a, d, e, f, g, h, k, l, m, n, i])
    }
}
/// n = 15, i = 6, a < k, a < n, b < g, b < o, c < f, c < k, d < e, d < h, f < j, g < i, g < j, h < i, h < k, i < n, j < l, k < m, k < o
fn select_1828([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < a {
        select_1821([c, d, g, e, a, f, h, i, j, l, n, k])
    } else {
        select_1827([b, c, d, a, e, f, h, g, k, m, i, j, n, l, o])
    }
}
/// n = 15, i = 6, a < k, a < n, b < f, b < k, c < g, c < o, d < e, d < h, f < l, g < i, g < j, h < j, h < k, i < l, j < n, k < m, k < o
fn select_1829([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_1815([a, b, c, d, e, h, g, i, j, f, k, l, m, n, o])
    } else {
        select_1828([a, c, b, d, e, f, g, h, j, i, k, l, m, n, o])
    }
}
/// n = 15, i = 6, a < j, b < l, c < l, c < n, d < g, d < o, e < f, e < h, g < i, g < k, h < k, h < l, i < j, k < n, l < m, l < o
fn select_1830([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < b {
        select_1801([c, a, d, e, f, b, h, g, i, j, k, l, n, m])
    } else {
        select_1829([c, b, d, e, f, a, g, h, i, k, l, j, m, n, o])
    }
}
/// n = 9, i = 6, a < g, b < h, c < h, d < h, e < g, e < i, f < g, f < i, h < i
fn select_1831([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_9([a, e, f, h, i])
    } else {
        select_8([b, c, d, g])
    }
}
/// n = 9, i = 6, b < h, c < h, c < i, d < h, d < i, e < g, f < g, g < i
fn select_1832([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_1831([b, a, e, f, c, d, h, g, i])
    } else {
        select_273([b, c, d, g, a, i, h])
    }
}
/// n = 11, i = 6, c < h, d < h, e < g, e < i, f < g, f < i, g < j, g < k, h < i, h < j, h < k
fn select_1833([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_927([b, g, c, d, a, h, i, j, k])
    } else {
        select_1832([b, a, e, f, c, d, h, g, i])
    }
}
/// n = 12, i = 6, b < l, c < g, d < h, e < h, f < g, f < i, g < j, g < k, h < i, h < j, h < k, i < l
fn select_1834([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < i {
        select_1833([a, b, d, e, c, f, g, h, i, j, k])
    } else {
        select_58([a, b, i, c, g, l])
    }
}
/// n = 10, i = 5, b < g, c < g, d < e, e < f, e < i, e < j, g < h, g < i, g < j
fn select_1835([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_740([a, b, c, e, f, g, i, j])
    } else {
        select_163([a, d, g, h, e, i, j])
    }
}
/// n = 9, i = 6, b < h, b < i, c < g, d < g, e < f, f < h, g < i
fn select_1836([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_372([a, c, d, b, h, g, i])
    } else {
        select_53([a, e, b, i, f, h])
    }
}
/// n = 11, i = 6, b < h, c < h, d < g, d < i, e < f, e < g, g < j, g < k, h < i, h < j, h < k
fn select_1837([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_1835([a, b, c, e, g, f, h, i, j, k])
    } else {
        select_1836([a, d, b, c, e, f, h, g, i])
    }
}
/// n = 8, i = 4, a < g, b < e, b < h, c < f, c < g, d < f, d < g, f < h
fn select_1838([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < f {
        select_443([a, c, d, b, e, g, f, h])
    } else {
        select_46([a, f, g, b, e, h])
    }
}
/// n = 8, i = 4, b < e, b < h, c < f, c < g, d < f, d < g, g < h
fn select_1839([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_1838([a, b, c, d, e, g, f, h])
    } else {
        select_23([g, f, b, e, a, h])
    }
}
/// n = 11, i = 6, b < f, b < g, c < h, d < h, e < g, e < i, e < j, g < k, h < i, h < j, h < k
fn select_1840([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_1837([a, c, d, e, b, f, g, h, j, i, k])
    } else {
        select_1839([a, b, e, h, f, j, i, g])
    }
}
/// n = 13, i = 7, a < m, b < h, c < f, c < g, d < i, e < i, g < l, h < j, h < k, i < j, i < k, i < l, l < m
fn select_1841([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_1834([f, a, b, d, e, g, h, i, l, j, k, m])
    } else {
        select_1840([a, c, d, e, h, f, g, i, j, k, l])
    }
}
/// n = 8, i = 4, a < f, b < f, b < g, c < d, c < e, e < g, f < h
fn select_1842([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_474([a, c, e, f, h, g, d])
    } else {
        select_302([a, d, e, b, g, f])
    }
}
/// n = 9, i = 4, a < g, b < g, b < h, c < d, c < e, d < h, e < f, g < i
fn select_1843([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if i < f {
        select_1842([a, b, c, e, d, g, h, i])
    } else {
        select_149([a, d, e, b, f, h, g])
    }
}
/// n = 10, i = 4, a < g, b < e, b < h, c < d, c < g, d < h, d < j, e < f, e < j, g < i
fn select_1844([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_567([d, b, a, g, i, h, e])
    } else {
        select_38([a, e, c, f, d, h, j, g])
    }
}
/// n = 10, i = 4, a < h, b < e, b < h, c < d, c < f, e < j, f < g, f < j, h < i
fn select_1845([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_1843([a, b, c, d, f, g, h, e, i])
    } else {
        select_1844([a, c, b, e, f, g, h, d, i, j])
    }
}
/// n = 12, i = 5, a < i, b < g, b < j, c < e, c < f, d < g, d < h, f < h, f < k, g < i, g < k, h < j, i < l
fn select_1846([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_1334([a, c, g, e, i, f, h, k, l])
    } else {
        select_1711([a, e, b, d, f, h, g, k, j])
    }
}
/// n = 12, i = 5, a < k, a < l, b < i, c < e, c < g, d < f, d < h, f < i, f < l, g < h, g < l, h < k, i < j
fn select_1847([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_1845([b, f, c, e, a, g, h, i, j, l])
    } else {
        select_1846([b, a, c, d, e, g, f, h, i, k, l, j])
    }
}
/// n = 14, i = 7, a < m, a < n, b < g, c < f, c < h, d < i, e < i, g < k, h < l, h < n, i < j, i < k, i < l, j < n, l < m
fn select_1848([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < j {
        select_1841([a, b, c, d, e, f, h, g, i, j, k, l, m])
    } else {
        select_1847([a, b, c, i, f, j, h, l, g, k, m, n])
    }
}
/// n = 12, i = 5, b < h, c < g, c < l, d < g, d < h, e < f, e < h, e < i, f < k, g < i, g < k, h < j, h < l
fn select_1849([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_1823([a, f, c, b, d, h, j, g, l])
    } else {
        select_1695([a, b, c, d, e, f, g, i, k, h])
    }
}
/// n = 10, i = 4, b < f, c < g, d < f, d < g, e < f, e < g, f < j, g < h, g < i
fn select_1850([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_491([a, c, d, e, f, j, g])
    } else {
        select_751([a, b, d, e, g, h, i, f])
    }
}
/// n = 9, i = 4, b < e, b < h, c < f, c < h, d < f, d < h, f < g, h < i
fn select_1851([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_491([a, e, c, d, h, i, f])
    } else {
        select_275([a, f, b, g, h, i, e])
    }
}
/// n = 13, i = 6, a < k, b < h, c < f, c < g, d < h, d < k, e < g, e < i, g < k, g < l, h < i, h < l, i < j, k < m
fn select_1852([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_1850([f, a, b, d, g, k, h, i, l, m])
    } else {
        select_1851([a, c, e, h, f, i, j, g, l])
    }
}
/// n = 14, i = 6, a < i, b < j, c < h, c < n, d < f, d < g, e < h, e < j, g < j, g < k, h < i, h < k, i < m, j < l, j < n
fn select_1853([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < a {
        select_1849([f, b, c, e, g, a, h, j, k, l, i, n])
    } else {
        select_1852([b, c, d, e, a, f, g, h, i, m, j, k, l])
    }
}
/// n = 10, i = 5, a < j, b < h, c < e, d < f, e < g, e < j, f < h, f < j, h < i
fn select_1854([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_554([a, b, c, f, e, h, i, j])
    } else {
        select_373([b, a, d, e, g, f, j])
    }
}
/// n = 8, i = 5, b < f, c < g, d < g, e < g, g < h
fn select_1855([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < h {
        select_1503([a, c, d, e, b, f, g, h])
    } else {
        select_3([a, h, b])
    }
}
/// n = 10, i = 6, b < i, c < f, d < h, d < i, e < g, f < h, g < i, i < j
fn select_1856([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < h {
        select_1855([a, c, b, d, g, f, i, j])
    } else {
        select_388([a, b, d, f, e, g, h])
    }
}
/// n = 11, i = 6, a < k, b < i, c < f, c < h, d < h, d < i, e < g, g < i, g < k, h < k, i < j
fn select_1857([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_1854([a, b, c, e, h, g, f, i, j, k])
    } else {
        select_1856([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 14, i = 7, a < k, b < j, b < n, c < g, d < f, d < h, e < i, g < j, h < k, i < j, i < k, j < m, k < l, k < n
fn select_1858([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_1647([f, b, c, a, h, e, i, g, k, l, n, j])
    } else {
        select_1857([a, b, d, g, e, f, i, h, j, m, k])
    }
}
/// n = 15, i = 7, a < g, b < k, c < j, c < o, d < f, d < h, e < i, g < n, h < k, h < l, i < j, i < k, j < l, j < n, k < m, k < o
fn select_1859([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < g {
        select_1853([a, b, c, d, i, f, h, j, g, k, l, m, n, o])
    } else {
        select_1858([b, c, a, d, e, f, g, h, i, j, k, m, n, o])
    }
}
/// n = 16, i = 7, a < g, b < m, b < o, c < j, c < p, d < f, d < i, e < h, e < j, g < k, h < m, i < l, i < m, j < k, j < l, l < o, m < n, m < p
fn select_1860([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < h {
        select_1848([b, a, d, c, e, f, g, i, j, h, k, l, o, m])
    } else {
        select_1859([a, b, c, d, e, f, g, i, h, j, m, l, n, k, p])
    }
}
/// n = 16, i = 7, a < g, b < m, b < o, c < j, c < p, d < f, d < i, e < h, e < j, h < m, i < l, i < m, j < k, j < l, l < o, m < n, m < p
fn select_1861([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < g {
        select_1830([a, h, b, c, d, f, j, i, k, g, l, m, n, o, p])
    } else {
        select_1860([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 7, a < g, b < k, b < p, c < m, c < n, d < h, d < k, e < f, e < i, h < n, i < j, i < n, j < m, k < l, n < o, n < p
fn select_1862([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < k {
        select_1776([f, a, b, c, d, i, g, j, h, m, k, n, o, l, p])
    } else {
        select_1861([a, c, b, e, d, f, g, h, i, k, l, j, n, o, m, p])
    }
}
/// n = 9, i = 4, a < f, a < h, b < e, c < g, c < i, d < f, d < g, d < i, e < i, g < h
fn select_1863([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_1329([b, c, d, a, f, e, g, h])
    } else {
        select_470([c, d, e, a, g, f, i])
    }
}
/// n = 10, i = 5, a < h, a < i, b < g, c < e, c < j, d < f, d < g, e < i, f < h, f < j, g < j
fn select_1864([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_1863([a, b, c, f, g, h, e, i, j])
    } else {
        select_125([b, a, d, e, i, g, f, h])
    }
}
/// n = 10, i = 5, a < j, b < g, b < h, c < f, d < e, d < i, e < h, f < g, f < i, i < j
fn select_1865([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_320([a, b, d, g, i, e, h, j])
    } else {
        select_106([a, b, c, h, f, i, g])
    }
}
/// n = 10, i = 5, a < g, b < i, b < j, c < e, c < h, d < f, d < g, e < j, f < h, f < i
fn select_1866([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_1864([b, a, c, d, e, f, g, i, j, h])
    } else {
        select_1865([a, b, d, c, e, f, i, j, h, g])
    }
}
/// n = 8, i = 3, a < g, a < h, b < e, c < d, c < f, d < h, e < f, e < g
fn select_1867([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_42([c, e, d, a, g, f])
    } else {
        select_642([b, c, a, d, e, g, h])
    }
}
/// n = 8, i = 4, a < h, b < f, c < f, c < h, d < e, d < g, e < h, f < g
fn select_1868([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_240([b, a, c, d, e, f, h])
    } else {
        select_4([d, f, a, e, g])
    }
}
/// n = 9, i = 4, a < g, b < e, b < h, c < f, c < i, d < f, d < g, f < h, g < i
fn select_1869([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_1867([b, a, c, f, g, i, e, h])
    } else {
        select_1868([a, c, d, b, e, f, h, g])
    }
}
/// n = 9, i = 4, b < i, c < g, d < f, d < h, e < f, e < g, f < i, g < h
fn select_1870([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_1869([c, b, d, e, a, f, g, i, h])
    } else {
        select_1122([a, c, d, e, b, f, g, h])
    }
}
/// n = 8, i = 4, a < f, b < g, c < f, c < h, d < e, d < g, g < h
fn select_1871([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_1329([a, b, d, c, e, f, g, h])
    } else {
        select_124([a, b, e, g, c, f])
    }
}
/// n = 10, i = 5, a < j, b < g, c < h, c < j, d < g, d < i, e < f, f < h, g < j, h < i
fn select_1872([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_1574([c, b, d, e, a, f, g, h, j])
    } else {
        select_1871([b, c, d, f, a, g, h, i])
    }
}
/// n = 12, i = 5, a < g, b < i, b < l, c < g, c < j, d < h, d < k, d < l, e < f, f < h, f < i, g < l, h < j, i < k
fn select_1873([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_1870([a, c, b, d, f, h, i, k, j])
    } else {
        select_1872([b, a, d, c, e, f, g, h, j, l])
    }
}
/// n = 13, i = 6, a < j, a < k, b < i, b < m, c < g, d < e, d < f, e < l, e < m, f < j, g < h, g < i, h < k, i < l, j < m
fn select_1874([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_1866([b, a, d, g, f, h, i, e, k, j])
    } else {
        select_1873([f, b, a, e, c, g, j, h, i, k, l, m])
    }
}
/// n = 9, i = 4, a < g, a < i, b < d, c < e, d < g, d < h, e < f, e < h, f < i
fn select_1875([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_11([a, b, f, d, i])
    } else {
        select_255([a, d, c, e, g, f, h])
    }
}
/// n = 11, i = 5, a < e, b < f, c < d, c < k, d < h, d < i, e < g, e < h, e < k, f < g, f < i, f < k, i < j
fn select_1876([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < d {
        select_474([b, c, d, f, g, h, k])
    } else {
        select_666([a, b, e, f, h, i, j])
    }
}
/// n = 12, i = 6, a < g, b < g, b < k, c < f, d < e, d < h, e < j, e < k, f < h, f < i, f < k, g < i, g < j, k < l
fn select_1877([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_1876([a, c, d, e, g, f, i, j, k, l, h])
    } else {
        select_1572([a, h, e, b, k, l, g])
    }
}
/// n = 10, i = 5, a < f, a < j, b < e, b < g, c < d, c < h, d < i, e < h, f < g, f < i, h < j
fn select_1878([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_1786([a, c, d, h, f, g, j])
    } else {
        select_1786([c, b, g, e, d, i, h])
    }
}
/// n = 13, i = 6, a < j, a < k, b < e, c < g, c < l, d < f, d < h, e < i, e < j, f < i, f < k, g < h, g < j, g < k, i < l, k < m
fn select_1879([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < e {
        select_1877([b, a, c, d, f, g, e, h, j, i, k, m])
    } else {
        select_1878([c, d, e, a, f, g, h, i, j, l])
    }
}
/// n = 14, i = 6, a < i, a < k, b < f, c < e, c < l, d < g, d < h, e < h, e < k, f < i, f < j, g < j, g < k, g < m, i < m, j < l, k < n
fn select_1880([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < e {
        select_1875([c, a, d, i, g, j, e, m, l])
    } else {
        select_1879([a, b, c, d, f, g, e, h, j, i, k, l, n])
    }
}
/// n = 14, i = 6, a < j, a < l, b < g, c < f, c < k, d < e, d < i, e < l, e < m, f < i, f < l, g < h, g < j, h < k, j < m, l < n
fn select_1881([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < e {
        select_1866([a, d, c, g, f, h, j, k, e, i])
    } else {
        select_1880([a, b, c, d, f, g, e, i, j, h, l, k, m, n])
    }
}
/// n = 14, i = 6, a < j, a < l, b < f, b < k, c < h, d < e, d < g, f < l, g < l, g < m, h < i, h < j, i < k, j < m, l < n
fn select_1882([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < f {
        select_1874([b, a, c, d, g, e, h, i, j, f, k, m, l])
    } else {
        select_1881([a, c, b, d, g, f, h, i, e, j, k, l, m, n])
    }
}
/// n = 11, i = 5, b < h, c < h, c < j, d < f, d < k, e < g, e < h, f < g, g < j, h < i, h < k
fn select_1883([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_1138([a, d, b, c, e, f, h, i, k])
    } else {
        select_526([a, b, c, e, f, g, j])
    }
}
/// n = 13, i = 5, b < i, b < l, c < h, c < i, d < g, d < i, d < k, e < f, e < m, f < g, f < h, g < l, h < k, i < j, i < m
fn select_1884([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_1870([a, b, c, d, f, g, h, k, l])
    } else {
        select_1883([a, c, b, e, d, f, g, i, j, l, m])
    }
}
/// n = 15, i = 6, a < j, a < k, b < i, b < k, c < e, c < f, d < g, d < o, f < k, f < n, g < h, g < i, h < j, i < n, j < l, k < m, k < o
fn select_1885([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < f {
        select_1170([b, a, c, h, e, i, f, j, l])
    } else {
        select_1884([e, a, b, f, d, g, h, i, k, m, n, j, o])
    }
}
/// n = 8, i = 3, a < d, b < g, b < h, c < e, e < f, e < g, e < h
fn select_1886([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_172([f, a, b, g, h, d])
    } else {
        select_88([b, d, c, e, f])
    }
}
/// n = 10, i = 5, a < g, b < e, b < j, c < g, c < h, d < f, f < h, h < i, h < j
fn select_1887([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_1886([a, b, f, g, h, i, e, j])
    } else {
        select_528([a, c, b, d, f, e, g, h])
    }
}
/// n = 8, i = 3, a < f, a < h, b < e, b < f, b < g, c < d, c < e, d < g, d < h
fn select_1888([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_42([b, d, e, a, h, g])
    } else {
        select_139([b, a, c, e, f, d, h])
    }
}
/// n = 9, i = 4, a < g, a < i, b < f, b < i, c < d, c < e, e < h, e < i, f < g, f < h
fn select_1889([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_302([a, e, d, b, f, i])
    } else {
        select_1888([a, f, c, e, d, g, h, i])
    }
}
/// n = 10, i = 5, a < h, a < j, b < g, b < h, c < f, c < j, d < e, d < g, f < h, f < i, g < i, g < j
fn select_1890([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_443([a, b, f, d, e, h, g, i])
    } else {
        select_491([e, a, c, g, f, i, j])
    }
}
/// n = 11, i = 5, a < g, a < k, b < f, b < k, c < g, c < h, d < e, d < h, f < j, g < i, h < i, h < j, h < k
fn select_1891([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_1889([b, a, d, e, h, g, f, i, k])
    } else {
        select_1890([a, c, b, d, e, f, h, g, j, k])
    }
}
/// n = 12, i = 5, a < g, a < l, b < f, b < l, c < e, c < h, d < g, d < h, d < i, f < i, g < k, h < k, h < l, i < j
fn select_1892([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_1887([a, c, d, b, e, f, g, i, j, h])
    } else {
        select_1891([a, b, d, c, e, f, g, h, k, i, l])
    }
}
/// n = 11, i = 5, b < h, b < k, c < f, c < k, d < g, e < g, f < j, g < h, g < j, g < k, h < i
fn select_1893([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_372([a, d, e, f, b, g, j])
    } else {
        select_1825([a, c, d, e, b, f, g, h, i, k])
    }
}
/// n = 13, i = 5, b < h, b < j, c < g, c < m, d < f, d < h, e < g, e < h, f < l, g < j, g < l, h < i, h < m, j < k
fn select_1894([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_1823([a, f, c, b, e, h, i, g, m])
    } else {
        select_1893([a, b, d, c, e, f, g, j, k, l, h])
    }
}
/// n = 15, i = 6, a < i, a < k, b < h, b < o, c < f, c < k, d < e, d < g, f < j, g < k, g < n, h < i, h < j, i < n, j < l, k < m, k < o
fn select_1895([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < g {
        select_1892([a, c, d, h, e, f, i, g, j, l, n, k])
    } else {
        select_1894([e, a, b, c, g, f, h, k, m, i, n, j, o])
    }
}
/// n = 15, i = 6, a < j, a < k, b < f, b < k, c < h, c < o, d < e, d < g, f < l, g < k, g < n, h < i, h < j, i < l, j < n, k < m, k < o
fn select_1896([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_1885([b, a, d, c, e, g, h, i, j, f, k, l, m, n, o])
    } else {
        select_1895([a, c, b, d, e, f, g, h, j, i, k, l, m, n, o])
    }
}
/// n = 15, i = 6, a < k, b < l, c < j, c < l, d < h, d < o, e < f, e < g, g < l, g < n, h < i, h < j, i < k, j < n, l < m, l < o
fn select_1897([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < b {
        select_1882([c, a, d, e, f, b, g, h, i, j, k, l, n, m])
    } else {
        select_1896([c, b, d, e, f, a, g, h, i, j, l, k, m, n, o])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < g, c < d, d < g, d < h, e < f
fn select_1898([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_3([c, e, f])
    } else {
        select_11([a, b, d, g, h])
    }
}
/// n = 8, i = 3, a < e, b < d, c < e, c < f, c < g, d < f, d < g, e < h
fn select_1899([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_19([a, c, f, g])
    } else {
        select_25([b, c, d, e, h])
    }
}
/// n = 10, i = 4, a < g, a < j, b < f, b < g, c < e, c < f, d < e, d < g, d < h, e < j, f < h, g < i
fn select_1900([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_1898([a, b, c, e, g, i, f, j])
    } else {
        select_1899([a, c, d, f, g, e, h, i])
    }
}
/// n = 10, i = 4, a < f, b < h, b < j, c < e, c < h, d < f, d < g, d < h, e < g, f < j, h < i
fn select_1901([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_140([b, e, d, a, g, h, f, i])
    } else {
        select_1900([b, c, a, d, f, e, h, g, i, j])
    }
}
/// n = 12, i = 6, a < i, b < g, b < l, c < f, c < g, d < h, e < h, f < j, g < k, h < i, h < j, h < k, i < l
fn select_1902([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_1901([a, b, c, h, f, i, j, g, k, l])
    } else {
        select_562([a, f, d, e, g, h, k])
    }
}
/// n = 9, i = 4, a < g, a < i, b < e, b < h, c < f, c < g, d < f, d < g, f < h, f < i
fn select_1903([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_44([a, f, g, b, e, h, i])
    } else {
        select_443([a, c, d, b, e, g, f, h])
    }
}
/// n = 11, i = 4, a < f, a < i, b < e, b < h, c < f, c < g, c < j, d < e, d < g, d < k, e < j, f < k, g < h, g < i
fn select_1904([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_691([b, d, c, f, e, g, i, k])
    } else {
        select_691([a, c, d, e, f, g, h, j])
    }
}
/// n = 12, i = 5, a < h, a < i, b < g, b < l, c < f, c < i, d < e, d < g, d < j, e < i, e < l, f < h, f < k, f < l, g < k, h < j
fn select_1905([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_1903([a, b, c, e, g, f, i, l, h])
    } else {
        select_1904([a, b, f, d, g, h, e, l, i, k, j])
    }
}
/// n = 14, i = 7, a < k, a < n, b < g, b < j, c < h, d < h, e < f, e < g, f < j, f < m, f < n, h < i, h < k, h < l, i < n, j < l, k < m
fn select_1906([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < i {
        select_1902([a, f, b, c, d, g, j, h, k, i, l, m])
    } else {
        select_1905([b, a, e, h, i, f, k, j, g, l, m, n])
    }
}
/// n = 10, i = 5, a < h, b < e, c < g, d < g, e < f, e < i, g < h, g < i, h < j
fn select_1907([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_372([a, c, d, e, f, g, i])
    } else {
        select_3([b, h, j])
    }
}
/// n = 8, i = 3, a < g, a < h, b < d, c < e, d < h, e < f, e < g
fn select_1908([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_11([a, c, d, e, h])
    } else {
        select_5([a, b, e, f, d, g])
    }
}
/// n = 11, i = 6, a < i, b < h, b < k, c < f, d < g, e < g, f < h, f < j, g < i, g < j, i < k
fn select_1909([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < g {
        select_1907([a, c, d, e, f, h, g, i, j, k])
    } else {
        select_1908([b, a, c, i, f, j, h, k])
    }
}
/// n = 8, i = 3, a < h, b < d, b < f, c < e, c < g, d < g, e < f, e < h
fn select_1910([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_42([d, c, a, e, f, g])
    } else {
        select_457([a, b, c, e, d, f, h])
    }
}
/// n = 8, i = 3, a < f, b < g, b < h, c < f, c < h, d < e, d < h, e < g
fn select_1911([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < b {
        select_1037([a, d, c, e, b, f, h])
    } else {
        select_1910([a, d, b, e, c, h, g, f])
    }
}
/// n = 12, i = 6, a < f, b < j, b < l, c < h, c < k, c < l, d < g, e < g, f < h, g < i, g < j, i < l, j < k
fn select_1912([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_1909([b, c, a, d, e, f, g, h, j, i, k])
    } else {
        select_1911([a, c, i, b, j, f, k, l])
    }
}
/// n = 14, i = 7, a < j, b < k, b < m, c < h, d < h, e < f, e < g, g < j, g < l, g < m, h < i, h < k, h < n, i < m, j < n, k < l
fn select_1913([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < f {
        select_1906([b, a, c, d, e, g, f, h, i, j, k, n, l, m])
    } else {
        select_1912([f, b, g, c, d, a, h, j, i, k, l, m])
    }
}
/// n = 10, i = 6, a < i, b < f, c < f, d < g, e < g, f < i, f < j, g < h, g < j
fn select_1914([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_1328([a, d, e, f, i, g, h, j])
    } else {
        select_9([a, b, c, h, f])
    }
}
/// n = 11, i = 6, a < j, b < f, c < g, d < g, e < f, e < i, f < h, f < k, g < i, g < j, g < k
fn select_1915([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_1914([a, c, d, b, e, g, f, h, j, k])
    } else {
        select_1329([a, b, e, g, i, j, f, k])
    }
}
/// n = 11, i = 6, a < i, b < h, c < h, d < f, d < g, e < f, e < g, f < k, g < j, h < i, h < j, h < k
fn select_1916([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1914([a, b, c, d, e, h, f, g, i, k])
    } else {
        select_1914([a, b, c, d, e, h, g, f, i, j])
    }
}
/// n = 9, i = 4, a < e, b < g, b < i, c < f, c < g, d < e, d < f, d < h, f < i, g < h
fn select_1917([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_708([a, b, d, f, g, h, i])
    } else {
        select_124([b, a, c, e, f, i])
    }
}
/// n = 11, i = 6, a < j, b < f, b < g, c < h, d < h, e < f, e < g, e < i, g < k, h < i, h < j, h < k
fn select_1918([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < i {
        select_1916([a, c, d, b, e, f, g, h, j, k, i])
    } else {
        select_1917([a, b, e, h, j, i, g, k, f])
    }
}
/// n = 12, i = 7, a < j, b < g, c < f, c < i, d < h, e < h, f < l, g < i, g < k, h < j, h < k, h < l
fn select_1919([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_1915([a, b, d, e, f, g, h, i, l, j, k])
    } else {
        select_1918([a, c, d, e, g, i, f, h, k, j, l])
    }
}
/// n = 9, i = 4, a < g, b < e, b < h, c < f, c < g, d < e, d < f, d < i, f < h, g < i
fn select_1920([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_1328([b, a, c, d, e, g, f, i])
    } else {
        select_795([a, b, c, e, f, h, g])
    }
}
/// n = 10, i = 5, a < h, a < i, b < g, c < f, d < e, d < g, e < h, f < i, f < j, g < j
fn select_1921([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_1920([b, a, d, f, i, e, g, h, j])
    } else {
        select_1122([b, c, a, e, g, h, f, i])
    }
}
/// n = 10, i = 5, a < g, b < i, b < j, c < f, d < e, d < g, d < h, e < i, f < h, f < j
fn select_1922([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_1865([a, b, c, d, e, f, j, i, h, g])
    } else {
        select_1921([b, a, c, d, e, f, g, i, j, h])
    }
}
/// n = 12, i = 7, a < k, b < f, b < j, c < g, d < h, e < h, g < j, g < l, h < i, h < k, h < l
fn select_1923([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_1919([a, c, b, d, e, f, g, h, j, k, l, i])
    } else {
        select_1922([a, b, c, h, i, g, k, l, f, j])
    }
}
/// n = 14, i = 7, a < g, b < k, b < n, c < f, c < h, d < i, e < i, g < l, h < m, h < n, i < j, i < k, i < l, j < n, k < m
fn select_1924([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_1913([a, b, d, e, c, f, h, i, j, g, k, m, n, l])
    } else {
        select_1923([b, c, a, d, e, f, g, i, j, h, k, l])
    }
}
/// n = 8, i = 3, a < c, a < d, b < e, e < f, e < g, g < h
fn select_1925([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < e {
        select_87([d, b, c, e])
    } else {
        select_45([a, e, f, d, g, h, c])
    }
}
/// n = 7, i = 4, a < f, a < g, b < e, c < d, c < f, e < g
fn select_1926([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < f {
        select_15([e, a, c, d, f, g])
    } else {
        select_3([d, b, e])
    }
}
/// n = 9, i = 4, a < d, a < f, b < f, b < g, b < h, c < e, e < g, e < h, h < i
fn select_1927([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < h {
        select_1926([b, c, a, d, e, f, g])
    } else {
        select_51([a, e, g, f, h, i])
    }
}
/// n = 9, i = 4, a < d, a < f, b < f, b < g, b < i, c < e, c < g, e < i, g < h
fn select_1928([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_1925([a, c, d, f, g, h, e, i])
    } else {
        select_1927([a, b, c, d, e, f, i, g, h])
    }
}
/// n = 10, i = 4, a < d, a < g, b < f, b < g, b < h, c < e, c < f, c < j, e < h, g < j, h < i
fn select_1929([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_409([d, g, c, f, e, j])
    } else {
        select_869([a, e, b, f, d, h, i, g])
    }
}
/// n = 10, i = 4, a < h, b < f, b < j, c < e, c < g, d < f, d < g, d < h, g < j, h < i
fn select_1930([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1928([c, d, a, e, b, g, h, i, f])
    } else {
        select_1929([c, d, b, e, a, f, g, h, i, j])
    }
}
/// n = 10, i = 5, b < j, c < h, c < i, d < g, e < g, f < g, g < h, g < i, g < j
fn select_1931([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < b {
        select_1113([a, d, e, f, c, b, g, h, i])
    } else {
        select_1503([a, d, e, f, b, c, g, j])
    }
}
/// n = 12, i = 5, b < l, c < g, c < j, d < h, d < k, e < g, e < h, f < g, f < h, g < k, g < l, h < i, h < j
fn select_1932([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_1931([a, b, d, c, e, f, g, h, k, l])
    } else {
        select_1111([a, b, c, d, e, f, h, i, g, j])
    }
}
/// n = 14, i = 6, a < j, b < i, b < k, c < h, c < n, d < f, d < g, e < h, e < k, g < k, h < i, h < j, j < m, k < l, k < n
fn select_1933([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_1930([a, b, d, h, f, i, g, j, m, k])
    } else {
        select_1932([f, a, c, b, e, g, h, k, l, n, i, j])
    }
}
/// n = 9, i = 4, a < e, b < f, c < f, c < i, d < f, d < i, e < i, f < g, f < h
fn select_1934([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_19([a, f, g, h])
    } else {
        select_9([b, c, d, e, i])
    }
}
/// n = 10, i = 4, a < i, a < j, b < e, b < h, c < f, d < f, d < j, e < j, f < g, f < h, f < i
fn select_1935([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_45([b, f, g, e, a, i, h])
    } else {
        select_1934([b, c, a, d, e, f, g, i, j])
    }
}
/// n = 8, i = 5, a < h, b < g, c < f, c < h, d < e, d < h, g < h
fn select_1936([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_158([a, e, c, b, g, f, h])
    } else {
        select_158([a, f, d, b, g, e, h])
    }
}
/// n = 9, i = 5, a < i, b < i, c < h, d < g, e < f, e < i, f < h, g < h, g < i
fn select_1937([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < a {
        select_933([b, d, e, c, a, f, g, h])
    } else {
        select_1936([b, d, a, e, f, c, g, i])
    }
}
/// n = 12, i = 6, a < k, a < l, b < i, c < f, c < h, d < h, d < i, e < g, g < i, g < k, h < k, i < j, i < l
fn select_1938([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_1935([a, c, b, g, h, i, j, f, l, k])
    } else {
        select_1937([b, f, a, e, d, h, g, k, i])
    }
}
/// n = 15, i = 7, a < k, a < n, b < j, b < o, c < h, d < f, d < g, e < i, g < j, h < k, i < j, i < k, j < l, j < n, k < m, k < o
fn select_1939([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < g {
        select_1938([b, a, d, h, e, f, i, g, k, m, j, o])
    } else {
        select_1647([f, a, c, b, g, e, i, h, j, l, n, k])
    }
}
/// n = 15, i = 7, a < g, b < k, b < l, c < j, c < o, d < f, d < h, e < i, g < n, h < l, i < j, i < l, j < k, j < n, l < m, l < o
fn select_1940([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < g {
        select_1933([a, b, c, d, i, f, h, j, k, g, l, m, n, o])
    } else {
        select_1939([c, b, a, d, e, f, h, g, i, l, j, m, n, o, k])
    }
}
/// n = 16, i = 7, a < g, b < k, b < m, c < j, c < p, d < f, d < i, e < h, e < j, g < l, h < m, i < m, i < o, j < k, j < l, k < o, m < n, m < p
fn select_1941([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < h {
        select_1924([a, b, d, c, e, f, g, i, j, h, k, l, o, m])
    } else {
        select_1940([a, b, c, d, e, f, g, i, h, j, k, m, n, l, p])
    }
}
/// n = 16, i = 7, a < g, b < l, b < m, c < j, c < p, d < f, d < i, e < h, e < j, h < m, i < m, i < o, j < k, j < l, l < o, m < n, m < p
fn select_1942([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < g {
        select_1897([a, h, b, c, d, f, i, j, k, l, g, m, n, o, p])
    } else {
        select_1941([a, b, c, d, e, f, g, h, i, j, l, k, m, n, o, p])
    }
}
/// n = 15, i = 6, b < f, c < i, c < m, d < h, d < k, e < g, e < j, e < k, g < m, g < n, h < j, h < m, i < k, k < l, k < n, m < o
fn select_1943([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_1765([a, b, d, e, c, f, g, i, h, k, l, j, n, m])
    } else {
        select_1750([a, b, c, e, d, f, g, h, j, i, k, m, l, o])
    }
}
/// n = 9, i = 2, a < c, b < d, b < e, d < h, d < i, e < f, e < g
fn select_1944([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_69([a, b, c, d, e, h, i])
    } else {
        select_69([a, b, c, e, d, f, g])
    }
}
/// n = 13, i = 6, a < f, b < h, c < h, d < h, e < g, e < i, g < l, g < m, h < i, h < l, h < m, i < j, i < k
fn select_1945([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < h {
        select_1503([f, b, c, d, e, g, h, i])
    } else {
        select_1944([a, e, f, g, i, j, k, l, m])
    }
}
/// n = 15, i = 6, a < f, b < h, b < n, c < h, c < i, d < h, d < i, e < g, e < k, g < i, g < o, h < k, h < o, i < j, i < n, k < l, k < m
fn select_1946([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_915([a, b, c, d, g, f, i, j, h, n])
    } else {
        select_1945([a, b, c, d, e, f, g, h, k, l, m, i, o])
    }
}
/// n = 15, i = 6, a < f, b < i, c < i, c < k, d < h, d < i, e < g, e < l, g < h, g < o, h < j, h < k, i < l, i < o, l < m, l < n
fn select_1947([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < b {
        select_1672([a, c, h, j, f, b, k, i])
    } else {
        select_1946([a, c, b, d, e, f, g, i, h, j, l, m, n, k, o])
    }
}
/// n = 9, i = 4, a < f, b < f, b < i, c < g, c < i, d < e, e < g, e < i, g < h
fn select_1948([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_124([c, b, d, f, e, g])
    } else {
        select_461([a, b, c, g, h, i, f])
    }
}
/// n = 8, i = 4, a < f, b < e, c < f, c < h, d < f, d < h, e < h, f < g
fn select_1949([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_26([b, c, d, e, f, g])
    } else {
        select_87([a, c, d, h])
    }
}
/// n = 10, i = 6, a < i, b < f, c < h, c < i, d < h, d < i, e < g, f < h, g < i, i < j
fn select_1950([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_1949([a, b, c, d, f, i, j, h])
    } else {
        select_637([a, c, d, e, g, h])
    }
}
/// n = 11, i = 6, a < f, b < i, b < k, c < h, c < i, d < h, d < i, e < g, g < i, g < k, h < k, i < j
fn select_1951([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_1948([a, h, b, e, g, f, i, j, k])
    } else {
        select_1950([b, a, c, d, e, f, g, h, i, j])
    }
}
/// n = 13, i = 7, a < f, b < j, b < k, c < h, c < m, d < i, e < g, e < k, g < j, h < k, i < j, i < k, j < m, k < l
fn select_1952([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_1770([a, b, c, g, d, f, i, h, j, m, k])
    } else {
        select_1951([a, b, e, h, d, f, i, g, k, l, j])
    }
}
/// n = 13, i = 6, a < f, b < g, b < j, c < h, d < h, e < h, g < m, h < i, h < j, h < m, j < k, j < l
fn select_1953([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_765([i, a, b, f, g, j, k, l, m])
    } else {
        select_1138([f, b, c, d, e, g, h, i, j])
    }
}
/// n = 16, i = 7, a < f, b < j, b < k, c < h, c < l, d < i, d < k, e < g, e < j, g < k, h < k, h < o, i < j, j < l, j < o, k < p, l < m, l < n
fn select_1954([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < j {
        select_1952([a, b, c, e, d, f, i, h, g, j, k, p, l])
    } else {
        select_1953([a, c, b, e, i, f, h, j, g, l, m, n, o])
    }
}
/// n = 16, i = 7, a < g, b < f, b < k, c < j, c < k, d < h, d < m, e < i, e < j, h < j, h < p, i < k, j < l, k < m, k < p, m < n, m < o
fn select_1955([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < f {
        select_1947([a, i, b, c, d, g, h, j, k, l, f, m, n, o, p])
    } else {
        select_1954([a, c, d, e, b, g, f, h, i, k, j, m, n, o, p, l])
    }
}
/// n = 11, i = 5, a < e, b < g, c < f, d < g, d < i, f < i, f < j, f < k, g < h, g < j, g < k
fn select_1956([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_610([a, c, g, h, e, f, i, j, k])
    } else {
        select_886([a, b, d, f, e, i, g, h, j, k])
    }
}
/// n = 9, i = 5, a < i, b < f, c < g, d < e, d < h, e < i, f < h, g < h, g < i
fn select_1957([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_775([a, d, f, c, g, e, h])
    } else {
        select_158([a, b, e, c, g, f, i])
    }
}
/// n = 9, i = 5, a < i, b < e, c < g, d < f, d < h, f < i, g < h, g < i
fn select_1958([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < h {
        select_1957([a, b, c, d, f, e, g, h, i])
    } else {
        select_1008([a, f, b, g, h, e, i])
    }
}
/// n = 12, i = 6, a < e, b < i, c < f, d < g, f < h, f < j, f < l, g < i, g < j, i < k, i < l
fn select_1959([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_1956([a, b, c, g, e, f, i, k, j, h, l])
    } else {
        select_1958([b, a, d, f, e, h, g, j, i])
    }
}
/// n = 9, i = 4, a < f, a < g, b < e, c < d, c < h, d < g, e < f, e < h, h < i
fn select_1960([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < h {
        select_1731([a, b, c, d, e, g, f, h])
    } else {
        select_46([a, c, f, h, d, i])
    }
}
/// n = 9, i = 4, a < f, b < h, c < f, c < g, d < e, e < g, e < h, h < i
fn select_1961([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < a {
        select_1960([c, d, b, a, e, g, f, h, i])
    } else {
        select_848([c, d, a, b, e, f, g, h])
    }
}
/// n = 11, i = 6, a < f, b < j, c < h, c < j, d < h, d < j, e < g, g < i, g < j, h < i, j < k
fn select_1962([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_1961([a, b, h, e, g, f, i, j, k])
    } else {
        select_1950([b, a, c, d, e, f, g, h, j, k])
    }
}
/// n = 7, i = 3, a < e, b < f, c < d, c < f, f < g
fn select_1963([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_88([a, d, b, f, g])
    } else {
        select_843([b, a, c, d, e])
    }
}
/// n = 12, i = 7, a < f, b < j, c < g, d < i, d < j, e < h, g < i, h < j, h < l, i < l, j < k
fn select_1964([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < j {
        select_1962([a, b, d, g, e, f, h, i, l, j, k])
    } else {
        select_1963([a, c, j, k, f, g, i])
    }
}
/// n = 13, i = 7, a < g, b < j, c < f, c < i, d < i, d < j, e < h, h < j, h < k, i < k, i < m, j < l, j < m
fn select_1965([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_1959([a, b, c, e, g, i, h, f, j, k, l, m])
    } else {
        select_1964([a, b, c, d, e, g, f, h, i, j, l, k])
    }
}
/// n = 6, i = 4, d < f, e < f
fn select_1966([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_637([c, d, e, a, b, f])
    } else {
        select_637([c, d, e, b, a, f])
    }
}
/// n = 10, i = 6, a < j, b < h, c < h, d < f, e < g, f < i, g < i, g < j, h < j
fn select_1967([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_1686([a, d, b, c, e, f, g, h, j])
    } else {
        select_1966([a, b, c, f, g, i])
    }
}
/// n = 8, i = 5, a < e, b < f, c < d, c < h, e < h, f < g
fn select_1968([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_1926([e, b, c, d, f, h, g])
    } else {
        select_162([a, g, c, d, e])
    }
}
/// n = 10, i = 6, a < j, b < e, c < f, d < g, e < i, f < i, f < j, g < h, g < j
fn select_1969([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_1512([b, a, c, d, f, g, h, e, j, i])
    } else {
        select_1968([c, d, e, a, f, g, h, i])
    }
}
/// n = 10, i = 6, b < j, c < f, d < g, e < h, f < j, g < i, h < i, h < j
fn select_1970([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_1967([b, a, c, d, e, g, h, f, i, j])
    } else {
        select_1969([b, d, e, c, g, h, f, a, i, j])
    }
}
/// n = 10, i = 5, b < h, c < h, d < f, d < j, e < g, e < h, f < g, h < i, h < j
fn select_1971([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_1138([a, d, b, c, e, f, h, i, j])
    } else {
        select_1966([a, b, c, e, f, g])
    }
}
/// n = 10, i = 3, b < d, b < i, b < j, c < e, e < f, e < i, e < j, f < g, f < h
fn select_1972([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_300([b, e, d, f, a, g, h, i, j])
    } else {
        select_30([a, b, c, d, e, i, j, f])
    }
}
/// n = 12, i = 4, b < d, b < i, c < e, d < f, d < l, e < g, e < i, e < l, g < h, i < j, i < k
fn select_1973([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < d {
        select_766([a, g, b, h, i, j, k, d])
    } else {
        select_1972([a, d, c, f, e, i, j, k, g, l])
    }
}
/// n = 13, i = 5, b < i, c < g, d < f, d < j, e < g, e < h, f < h, f < m, g < i, g < j, g < m, j < k, j < l
fn select_1974([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < g {
        select_1971([a, b, c, d, e, f, h, g, i, j])
    } else {
        select_1973([a, d, c, f, g, h, b, i, j, k, l, m])
    }
}
/// n = 14, i = 6, b < f, c < i, d < g, d < k, e < h, g < j, g < n, h < i, h < j, i < k, i < n, k < l, k < m
fn select_1975([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < i {
        select_1970([a, c, b, d, e, f, g, h, j, i])
    } else {
        select_1974([a, b, c, d, h, g, i, j, f, k, l, m, n])
    }
}
/// n = 16, i = 7, a < g, b < j, b < k, c < h, c < n, d < f, d < i, e < i, e < j, h < j, h < l, i < k, i < l, j < m, k < n, n < o, n < p
fn select_1976([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < k {
        select_1965([a, b, d, e, c, f, g, h, i, j, l, m, k])
    } else {
        select_1975([f, a, b, c, e, g, h, i, k, l, n, o, p, j])
    }
}
/// n = 16, i = 7, a < g, b < i, b < k, c < f, c < j, d < j, d < k, e < h, e < n, h < k, h < l, i < n, j < l, j < n, k < m, n < o, n < p
fn select_1977([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_1955([a, c, d, e, b, f, g, h, i, k, j, m, n, o, p, l])
    } else {
        select_1976([a, b, e, c, d, f, g, h, j, k, i, l, m, n, o, p])
    }
}
/// n = 16, i = 7, a < g, b < l, b < p, c < i, c < l, d < f, d < j, e < h, e < n, h < k, h < l, i < n, j < k, j < n, l < m, n < o, n < p
fn select_1978([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < b {
        select_1943([f, a, c, e, j, g, b, h, i, k, n, o, l, p, m])
    } else {
        select_1977([a, c, d, b, e, f, g, h, i, j, l, k, m, n, o, p])
    }
}
/// n = 16, i = 7, a < g, b < k, b < p, c < h, c < n, d < i, d < k, e < f, e < j, h < m, i < n, j < m, j < n, k < l, n < o, n < p
fn select_1979([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < h {
        select_1942([a, c, b, e, d, f, g, i, j, k, l, h, n, o, m, p])
    } else {
        select_1978([a, b, d, e, c, f, g, h, i, j, m, k, l, n, o, p])
    }
}
/// n = 16, i = 7, a < h, b < l, b < p, c < g, c < n, d < i, d < l, e < f, e < j, i < n, j < k, j < n, l < m, n < o, n < p
fn select_1980([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < g {
        select_1862([a, b, c, d, e, f, h, i, j, k, l, m, g, n, o, p])
    } else {
        select_1979([a, b, c, d, e, f, h, g, i, j, l, m, k, n, o, p])
    }
}
/// n = 16, i = 7, a < i, b < l, c < n, d < l, d < p, e < h, e < n, f < g, f < j, j < k, j < n, l < m, n < o, n < p
fn select_1981([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if c < b {
        select_1709([a, d, e, c, f, g, h, i, b, j, k, l, m, n, o, p])
    } else {
        select_1980([a, d, e, b, f, g, h, i, c, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 7, b < i, c < l, d < l, d < p, e < h, e < n, f < g, f < j, j < k, j < n, l < m, n < o, n < p
fn select_1982([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if n < a {
        select_1474([b, c, d, e, f, g, h, i, j, k, l, m, n, a, o, p])
    } else {
        select_1981([b, c, a, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }
}
/// n = 10, i = 3, a < c, a < h, a < i, b < d, b < h, b < i, c < g, d < e, d < f, i < j
fn select_1983([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_45([c, b, h, g, i, j, d])
    } else {
        select_1339([d, a, h, e, f, i, j, c])
    }
}
/// n = 11, i = 4, a < d, a < e, b < g, b < i, b < j, c < f, c < g, f < i, f < j, g < h, j < k
fn select_1984([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_1983([b, f, g, a, d, e, h, i, j, k])
    } else {
        select_976([b, c, a, d, e, g, f, h, i, j])
    }
}
/// n = 10, i = 3, a < c, a < d, a < j, b < e, e < f, e < g, e < j, f < i, g < h
fn select_1985([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < c {
        select_836([a, e, d, f, g, h, i, j])
    } else {
        select_670([a, b, c, d, e, f, i, j])
    }
}
/// n = 12, i = 4, a < d, b < c, b < i, b < j, c < e, c < f, d < g, d < i, d < j, g < h, i < l, j < k
fn select_1986([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < c {
        select_836([g, b, h, i, j, k, l, c])
    } else {
        select_1985([c, a, e, f, d, i, j, k, l, g])
    }
}
/// n = 10, i = 4, a < f, a < h, b < f, b < h, c < d, c < e, c < h, f < g, h < i, h < j
fn select_1987([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_36([f, c, e, g, h, i, j, d])
    } else {
        select_751([e, d, a, b, h, i, j, f])
    }
}
/// n = 11, i = 5, a < g, a < h, b < f, c < d, c < e, c < g, f < h, f < j, f < k, g < j, g < k, h < i
fn select_1988([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1987([a, f, c, d, e, h, i, g, j, k])
    } else {
        select_1712([d, e, b, g, f, h, j, k])
    }
}
/// n = 13, i = 5, a < d, a < e, b < g, c < f, c < h, f < j, f < k, g < h, g < j, g < k, h < i, j < m, k < l
fn select_1989([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_1986([b, f, a, g, d, e, h, i, j, k, l, m])
    } else {
        select_1988([c, b, a, d, e, g, f, h, i, j, k])
    }
}
/// n = 13, i = 5, a < d, a < e, b < f, b < i, b < k, c < g, f < l, g < h, g < i, g < l, h < k, i < j, l < m
fn select_1990([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < f {
        select_1984([a, b, g, d, e, h, i, j, k, f, l])
    } else {
        select_1989([a, c, b, d, e, f, g, i, j, h, l, m, k])
    }
}
/// n = 9, i = 3, a < c, b < d, c < e, d < f, d < g, f < i, g < h
fn select_1991([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_122([a, f, g, h, i])
    } else {
        select_666([a, b, c, d, e, f, i])
    }
}
/// n = 10, i = 3, b < d, b < j, c < e, e < f, e < g, e < j, f < i, g < h
fn select_1992([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1991([a, c, b, e, d, f, g, h, i])
    } else {
        select_1985([b, c, a, d, e, f, g, h, i, j])
    }
}
/// n = 11, i = 4, a < d, b < f, c < e, c < g, c < h, e < k, f < g, f < h, f < k, g < j, h < i
fn select_1993([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_216([e, h, g, a, j, i, k])
    } else {
        select_837([a, b, c, d, e, f, h, i, k, g])
    }
}
/// n = 13, i = 4, a < d, a < m, b < f, b < g, c < e, c < g, c < i, e < l, f < i, f < l, g < h, g < m, h < j, i < k
fn select_1994([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_1992([e, a, b, d, g, f, h, j, i, m])
    } else {
        select_1993([a, b, c, d, e, f, g, i, k, h, l])
    }
}
/// n = 9, i = 3, a < c, a < d, b < e, e < f, e < g, f < i, g < h
fn select_1995([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_216([f, g, a, c, d, h, i])
    } else {
        select_47([c, d, b, e, f, g])
    }
}
/// n = 9, i = 4, b < i, c < e, c < i, d < f, e < h, f < g, f < h, f < i
fn select_1996([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_977([a, b, c, f, g, e, i])
    } else {
        select_776([a, b, d, c, e, f, g, h, i])
    }
}
/// n = 12, i = 5, a < d, a < e, b < f, b < l, c < g, e < l, f < i, f < k, g < h, g < i, g < l, h < k, i < j
fn select_1997([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < a {
        select_1995([a, c, d, e, g, h, i, j, k])
    } else {
        select_1996([d, e, b, c, f, g, h, i, l])
    }
}
/// n = 14, i = 5, a < d, a < e, b < f, b < i, c < g, f < k, f < n, g < h, g < i, g < k, h < n, i < j, j < l, k < m
fn select_1998([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < d {
        select_1994([a, b, g, e, h, f, i, j, k, l, m, n, d])
    } else {
        select_1997([a, b, c, e, d, f, g, h, k, m, n, i])
    }
}
/// n = 14, i = 5, a < e, a < f, b < j, c < i, c < l, d < g, g < h, g < i, g < j, h < l, i < k, j < m, m < n
fn select_1999([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if c < b {
        select_1990([a, c, d, e, f, b, g, h, i, k, l, j, m])
    } else {
        select_1998([a, b, d, e, f, c, g, h, j, m, i, n, k, l])
    }
}
/// n = 9, i = 4, b < i, c < g, c < h, d < f, e < f, f < g, f < h, f < i
fn select_2000([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_659([a, c, f, b, i, g, h])
    } else {
        select_562([a, c, b, d, e, f, i])
    }
}
/// n = 9, i = 4, b < i, c < f, c < h, d < g, e < f, e < g, g < h, g < i
fn select_2001([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_2000([a, b, c, d, e, g, f, h, i])
    } else {
        select_526([a, b, d, c, e, f, g])
    }
}
/// n = 11, i = 4, a < d, a < f, b < f, b < g, b < h, c < e, c < k, e < g, e < h, f < k, g < j, h < i
fn select_2002([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_836([a, b, d, g, h, i, j, f])
    } else {
        select_709([d, c, b, f, e, g, h, k])
    }
}
/// n = 11, i = 4, a < d, a < g, b < f, b < g, b < i, c < e, c < f, c < k, e < i, f < h, g < k, i < j
fn select_2003([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_1603([a, c, d, g, f, h, e, i, k])
    } else {
        select_2002([a, b, c, d, e, g, f, i, j, h, k])
    }
}
/// n = 9, i = 3, a < d, b < g, c < e, c < g, e < f, f < h, g < i
fn select_2004([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_110([a, e, d, b, f, h, g])
    } else {
        select_134([a, b, c, d, e, g, i, f])
    }
}
/// n = 9, i = 4, a < e, a < f, b < f, b < g, c < d, c < g, g < h, h < i
fn select_2005([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_275([d, a, g, f, h, i, e])
    } else {
        select_159([e, b, c, d, f, g])
    }
}
/// n = 11, i = 4, a < d, a < f, b < f, b < g, b < i, c < e, c < g, e < i, g < h, h < j, i < k
fn select_2006([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < f {
        select_2004([a, e, b, d, g, h, i, j, k])
    } else {
        select_2005([a, b, c, e, d, f, g, h, j])
    }
}
/// n = 12, i = 4, a < h, b < g, b < l, c < e, c < f, d < f, d < g, d < h, f < l, g < i, h < j, j < k
fn select_2007([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < a {
        select_2003([c, d, b, e, a, g, f, i, h, j, l])
    } else {
        select_2006([c, d, a, e, b, f, h, j, g, k, i])
    }
}
/// n = 13, i = 5, a < j, b < h, b < i, c < e, c < f, d < g, f < h, g < i, g < j, i < k, j < l, l < m
fn select_2008([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_2001([e, a, b, d, f, h, g, i, j])
    } else {
        select_2007([a, b, c, g, e, f, i, j, k, l, m, h])
    }
}
/// n = 9, i = 3, b < d, b < i, c < e, d < f, e < g, e < i, g < h
fn select_2009([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_82([a, c, d, f, e])
    } else {
        select_659([a, b, e, g, h, d, i])
    }
}
/// n = 9, i = 3, b < d, b < e, b < i, c < f, f < g, f < i, g < h
fn select_2010([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_2009([a, b, c, d, f, e, g, h, i])
    } else {
        select_2009([a, b, c, e, f, d, g, h, i])
    }
}
/// n = 11, i = 4, a < d, a < g, b < e, b < g, b < j, c < f, e < k, f < h, f < j, f < k, h < i
fn select_2011([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_423([h, a, b, j, d, i, e, k, g])
    } else {
        select_2010([d, b, c, e, g, f, h, i, j])
    }
}
/// n = 11, i = 4, a < e, b < f, b < k, c < g, d < f, d < g, f < h, g < i, g < k, i < j
fn select_2012([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_10([c, a, b, d, e, f, h, g])
    } else {
        select_369([a, b, c, d, e, g, i, j, f, k])
    }
}
/// n = 9, i = 3, a < d, a < e, b < f, c < e, c < f, f < g, f < h, f < i
fn select_2013([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_33([a, g, h, i])
    } else {
        select_82([d, b, c, e, f])
    }
}
/// n = 12, i = 5, a < e, a < j, b < g, c < f, d < g, d < h, f < h, f < k, f < l, g < j, g < k, g < l, h < i
fn select_2014([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_2013([a, c, g, e, j, f, h, k, l])
    } else {
        select_1609([a, b, d, f, e, h, i, g, k, l])
    }
}
/// n = 14, i = 5, a < e, a < k, b < g, b < h, c < g, c < i, d < f, f < h, f < i, f < n, g < k, g < n, h < j, i < l, l < m
fn select_2015([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_2012([a, b, c, f, e, h, i, j, l, m, g])
    } else {
        select_2014([a, c, d, b, e, f, g, h, j, k, i, n])
    }
}
/// n = 14, i = 5, a < e, a < h, b < j, c < f, c < i, d < g, f < h, f < l, g < i, g < j, g < l, i < k, j < m, m < n
fn select_2016([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < b {
        select_2011([a, f, d, e, b, g, h, i, k, l, j])
    } else {
        select_2015([a, c, b, d, e, g, f, i, j, k, h, m, n, l])
    }
}
/// n = 14, i = 5, a < e, a < f, b < j, c < g, c < i, d < h, g < l, h < i, h < j, h < l, i < k, j < m, m < n
fn select_2017([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < g {
        select_2008([b, c, a, d, f, e, h, g, i, j, k, m, n])
    } else {
        select_2016([a, b, c, d, f, g, h, e, i, j, k, l, m, n])
    }
}
/// n = 14, i = 5, a < f, a < g, b < k, c < e, c < j, d < h, h < i, h < j, h < k, j < l, k < m, m < n
fn select_2018([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < e {
        select_1999([a, b, c, d, f, g, h, i, j, k, l, e, m, n])
    } else {
        select_2017([a, b, c, d, f, g, e, h, j, k, l, i, m, n])
    }
}
/// n = 11, i = 4, a < h, b < f, b < h, b < j, c < d, c < f, c < k, d < e, d < g, g < j, h < i, h < k
fn select_2019([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < a {
        select_316([e, a, g, b, f, j, h])
    } else {
        select_772([b, c, d, h, e, g, i, f, k])
    }
}
/// n = 9, i = 4, a < g, b < f, b < g, c < d, c < f, c < i, d < e, g < h, g < i
fn select_2020([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_159([a, b, d, e, f, g])
    } else {
        select_35([c, g, h, f, d, i])
    }
}
/// n = 12, i = 4, a < g, b < f, b < g, b < i, c < d, c < f, c < l, d < e, d < i, g < h, g < l, i < j, i < k
fn select_2021([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_57([a, b, d, e, f, i, j, k])
    } else {
        select_2020([a, b, c, d, e, f, g, h, l])
    }
}
/// n = 12, i = 4, a < h, b < g, b < h, b < j, c < d, c < g, c < l, d < e, d < f, d < j, h < i, h < l, j < k
fn select_2022([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < j {
        select_2019([a, b, c, d, f, g, e, h, i, j, l])
    } else {
        select_2021([a, b, c, d, f, g, h, i, j, e, k, l])
    }
}
/// n = 13, i = 4, a < d, a < i, a < k, b < c, b < i, b < j, c < e, c < f, c < k, d < g, d < h, d < j, j < l, k < m
fn select_2023([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < d {
        select_628([e, f, a, i, k, d, g, h, j, m])
    } else {
        select_628([g, h, b, i, j, c, e, f, k, l])
    }
}
/// n = 13, i = 4, b < d, b < i, b < j, c < e, c < i, c < k, d < h, d < k, e < f, e < g, e < j, j < l, k < m
fn select_2024([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < d {
        select_2022([a, b, c, e, f, g, i, d, h, j, l, k])
    } else {
        select_2023([b, c, e, d, f, g, a, h, i, k, j, m, l])
    }
}
/// n = 14, i = 5, a < d, a < e, a < l, b < c, b < f, d < h, e < g, e < m, f < i, f < l, f < m, i < j, i < k, m < n
fn select_2025([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < i {
        select_239([e, d, b, c, h, g, f, l, m, i, j, k, n])
    } else {
        select_2024([c, a, f, e, i, j, k, g, l, d, m, h, n])
    }
}
/// n = 11, i = 3, a < d, b < g, b < h, b < k, c < e, c < g, c < h, e < f, e < k, h < i, h < j
fn select_2026([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_604([e, b, g, f, a, h, i, j, d, k])
    } else {
        select_1152([a, b, c, d, g, e, h, i, j, f])
    }
}
/// n = 11, i = 3, c < g, c < h, c < k, d < e, d < g, d < h, e < f, e < k, h < i, h < j
fn select_2027([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_2026([a, c, d, b, e, f, g, h, i, j, k])
    } else {
        select_2026([b, c, d, a, e, f, g, h, i, j, k])
    }
}
/// n = 7, i = 2, b < e, b < f, b < g, c < d, c < f, c < g
fn select_2028([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < a {
        select_29([b, c, e, d, a, f, g])
    } else {
        select_37([a, c, b, e, f, g, d])
    }
}
/// n = 10, i = 2, b < e, b < j, c < d, c < f, c < j, f < g, f < h, f < i
fn select_2029([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_398([a, d, f, b, g, h, i])
    } else {
        select_2028([a, b, c, d, e, f, j])
    }
}
/// n = 7, i = 3, a < g, b < f, c < e, c < f, c < g, d < e, d < f, d < g
fn select_2030([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_75([c, d, a, b, e, g])
    } else {
        select_75([c, d, b, a, e, f])
    }
}
/// n = 9, i = 3, b < i, c < e, c < f, c < i, d < e, d < f, d < i, f < g, f < h
fn select_2031([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_173([e, b, f, a, g, h, i])
    } else {
        select_2030([a, b, c, d, e, i, f])
    }
}
/// n = 10, i = 4, a < j, b < e, b < h, b < j, c < d, c < f, e < g, e < i, f < h, f < i, f < j
fn select_2032([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_57([a, c, e, g, d, f, h, i])
    } else {
        select_2031([d, a, b, f, h, e, g, i, j])
    }
}
/// n = 12, i = 4, b < e, b < h, b < i, c < d, c < f, e < g, e < l, f < h, f < i, f < l, i < j, i < k
fn select_2033([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < a {
        select_2029([d, e, f, h, g, i, a, j, k, l])
    } else {
        select_2032([a, b, c, d, e, f, g, h, l, i])
    }
}
/// n = 13, i = 4, b < e, b < j, b < k, c < d, c < f, e < g, f < h, f < j, f < k, h < i, k < l, k < m
fn select_2034([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_2027([a, d, b, f, h, i, j, k, l, m, e])
    } else {
        select_2033([a, b, c, d, e, f, g, j, k, l, m, h])
    }
}
/// n = 12, i = 4, a < f, a < i, a < l, b < e, b < g, b < i, c < d, c < g, c < i, g < h, g < l, i < j, i < k
fn select_2035([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_606([d, a, b, f, g, h, e, i, j, k, l])
    } else {
        select_606([e, a, c, f, g, h, d, i, j, k, l])
    }
}
/// n = 9, i = 3, b < e, b < i, c < f, c < i, d < e, d < f, d < i, f < g, f < h
fn select_2036([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_70([b, c, e, f, a, g, h, i])
    } else {
        select_1071([a, c, b, d, e, i, f])
    }
}
/// n = 9, i = 3, a < e, a < f, b < f, b < g, c < d, c < e, c < g, g < h, g < i
fn select_2037([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_34([d, g, a, h, i])
    } else {
        select_1463([b, a, c, d, e, f, g])
    }
}
/// n = 11, i = 4, a < g, a < h, a < k, b < f, b < h, b < k, c < d, c < e, c < h, e < k, f < g, h < i, h < j
fn select_2038([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_2036([d, e, a, b, f, h, i, j, k])
    } else {
        select_2037([f, a, c, d, e, g, h, i, j])
    }
}
/// n = 10, i = 4, a < f, a < g, b < e, c < d, c < f, c < h, e < g, e < h, h < i, h < j
fn select_2039([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_1926([a, b, c, d, e, f, g])
    } else {
        select_76([a, e, c, d, g, h, i, j])
    }
}
/// n = 12, i = 5, a < g, b < f, b < i, b < l, c < d, c < e, c < i, e < l, f < h, g < h, g < i, g < l, i < j, i < k
fn select_2040([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_1039([d, e, b, a, g, f, i, j, k, l])
    } else {
        select_2039([f, a, c, d, g, e, h, i, j, k])
    }
}
/// n = 12, i = 5, a < e, a < i, a < l, b < g, c < d, c < f, c < i, f < l, g < h, g < i, g < l, i < j, i < k
fn select_2041([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_2038([a, g, c, d, f, h, e, i, j, k, l])
    } else {
        select_2040([b, a, c, d, f, e, g, h, i, j, k, l])
    }
}
/// n = 13, i = 5, a < e, a < f, a < k, b < d, b < i, b < k, c < g, g < h, g < i, g < k, i < j, k < l, k < m
fn select_2042([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < e {
        select_2035([a, b, g, h, d, f, i, j, k, l, m, e])
    } else {
        select_2041([b, c, a, f, d, e, g, h, k, l, m, i])
    }
}
/// n = 16, i = 7, a < d, a < k, a < n, b < f, c < e, c < j, e < g, e < n, e < p, f < j, f < k, f < n, g < h, g < i, k < l, k < p, l < m, n < o
fn select_2043([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < g {
        select_2034([d, k, c, j, l, e, m, n, o, p, g, h, i])
    } else {
        select_2042([g, a, b, d, h, i, f, j, n, o, k, l, p])
    }
}
/// n = 16, i = 7, a < d, a < o, b < f, c < e, c < k, e < g, e < n, e < o, f < h, f < k, f < o, g < i, g < j, h < l, h < n, l < m, o < p
fn select_2044([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < a {
        select_2025([h, c, k, l, a, e, d, m, g, i, j, n, o, p])
    } else {
        select_2043([a, b, c, d, e, f, g, i, j, k, h, l, m, o, p, n])
    }
}
/// n = 16, i = 7, a < d, a < o, b < f, c < e, c < l, e < g, e < h, e < o, f < i, f < l, f < o, h < j, h < k, i < m, m < n, o < p
fn select_2045([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < i {
        select_2018([h, g, a, b, d, j, k, f, l, o, i, p, m, n])
    } else {
        select_2044([a, b, c, d, e, f, h, i, j, k, l, m, n, g, o, p])
    }
}
/// n = 5, i = 1, d < e
fn select_2046([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_122([c, a, d, e, b])
    } else {
        select_122([c, b, d, e, a])
    }
}
/// n = 8, i = 3, a < d, b < c, b < e, d < h, e < f, e < g
fn select_2047([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_2046([c, f, g, a, d])
    } else {
        select_64([d, b, c, h, e, f, g])
    }
}
/// n = 8, i = 3, a < h, b < h, c < d, c < e, e < f, e < g
fn select_2048([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_2047([a, c, d, b, e, f, g, h])
    } else {
        select_2047([b, c, d, a, e, f, g, h])
    }
}
/// n = 7, i = 2, a < f, a < g, b < d, b < e, c < d, c < e, c < f, c < g
fn select_2049([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_19([b, c, f, g])
    } else {
        select_19([a, c, d, e])
    }
}
/// n = 9, i = 4, a < h, a < i, b < e, b < g, c < f, d < f, f < g, f < h, f < i
fn select_2050([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_9([a, c, d, e, f])
    } else {
        select_2049([a, b, f, e, g, h, i])
    }
}
/// n = 10, i = 4, a < e, a < j, b < g, c < f, c < i, d < f, d < g, f < h, g < i, g < j
fn select_2051([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_10([b, a, c, d, e, f, h, g])
    } else {
        select_2050([c, a, b, d, e, g, j, f, i])
    }
}
/// n = 10, i = 4, a < f, b < h, c < e, c < h, d < f, d < h, f < g, h < i, h < j
fn select_2052([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_503([a, e, b, d, h, i, j, f])
    } else {
        select_1254([b, a, c, f, g, h, i, j, e])
    }
}
/// n = 11, i = 4, a < g, a < k, b < e, b < i, c < f, c < i, d < f, d < g, d < i, f < k, g < h, i < j
fn select_2053([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < i {
        select_2051([b, c, a, d, e, g, f, h, k, i])
    } else {
        select_2052([a, c, b, d, e, g, h, i, f, j])
    }
}
/// n = 11, i = 4, a < g, a < i, b < e, b < j, c < g, c < j, d < f, d < j, f < i, g < h, j < k
fn select_2054([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < c {
        select_1119([a, b, f, e, c, i, j, g])
    } else {
        select_2053([a, b, d, c, e, f, g, h, j, k, i])
    }
}
/// n = 11, i = 4, a < h, b < h, b < j, c < e, c < g, d < f, d < g, f < j, g < i, h < k
fn select_2055([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_2048([b, f, c, e, g, a, i, j])
    } else {
        select_2054([b, c, a, d, e, f, h, k, j, g, i])
    }
}
/// n = 9, i = 3, a < d, a < i, b < c, b < e, d < f, e < g, e < i, g < h
fn select_2056([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_172([c, g, a, d, i, h])
    } else {
        select_64([d, b, c, f, e, g, i])
    }
}
/// n = 10, i = 3, a < c, a < h, b < d, b < h, c < f, d < e, d < g, g < i, h < j
fn select_2057([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_172([e, g, a, c, h, i])
    } else {
        select_115([c, b, f, h, d, e, g, j])
    }
}
/// n = 10, i = 3, a < d, a < h, b < g, b < h, c < e, c < g, e < f, g < i, h < j
fn select_2058([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_2056([c, a, d, e, b, f, h, j, g])
    } else {
        select_2057([c, b, e, a, d, f, h, g, j, i])
    }
}
/// n = 13, i = 4, a < d, a < i, a < j, b < f, b < g, c < e, c < g, e < i, e < k, f < j, f < k, g < h, i < l, j < m
fn select_2059([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_2058([a, e, b, d, g, h, f, i, k, l])
    } else {
        select_2058([a, f, c, d, g, h, e, j, k, m])
    }
}
/// n = 9, i = 3, a < g, b < g, b < h, c < d, c < e, e < f, e < h, g < i
fn select_2060([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_678([d, b, e, a, f, h])
    } else {
        select_1158([b, c, a, d, e, g, f, i])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < h, c < d, c < f, c < h, f < g
fn select_2061([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_44([a, c, e, f, d, g, h])
    } else {
        select_470([a, b, c, d, e, f, h])
    }
}
/// n = 10, i = 3, a < e, a < g, a < j, b < d, b < f, b < j, c < e, c < f, c < g, c < j, f < h, g < i
fn select_2062([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < d {
        select_1029([b, a, e, g, f, i, h, j])
    } else {
        select_2061([a, c, b, d, e, f, h, j])
    }
}
/// n = 12, i = 4, a < d, a < i, a < l, b < e, b < g, b < l, c < f, c < h, e < h, f < g, f < i, f < l, h < j, i < k
fn select_2063([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_134([a, c, e, d, f, h, j, g])
    } else {
        select_2062([b, a, f, d, g, i, e, k, h, l])
    }
}
/// n = 13, i = 4, a < d, a < g, a < k, b < f, b < j, c < e, c < g, c < h, e < j, f < h, f < k, g < i, j < l, k < m
fn select_2064([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_2060([e, b, a, d, g, i, j, f, l])
    } else {
        select_2063([a, c, b, d, e, f, h, j, k, l, m, g])
    }
}
/// n = 13, i = 4, a < h, b < g, b < i, c < e, c < g, c < l, d < f, d < h, f < i, f < l, g < j, h < k, l < m
fn select_2065([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_2059([c, a, d, e, f, b, h, k, l, g, i, m, j])
    } else {
        select_2064([c, d, b, e, a, f, g, i, j, h, l, k, m])
    }
}
/// n = 13, i = 4, a < i, b < f, b < h, c < g, c < i, d < e, d < h, d < l, g < l, h < j, i < k, l < m
fn select_2066([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_2055([a, c, d, b, e, f, h, i, j, g, k])
    } else {
        select_2065([a, b, d, c, e, g, h, i, f, j, k, l, m])
    }
}
/// n = 8, i = 3, a < f, b < e, b < g, c < d, c < g, g < h
fn select_2067([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_109([a, b, c, f, d, g, h, e])
    } else {
        select_109([a, c, b, f, e, g, h, d])
    }
}
/// n = 9, i = 3, a < f, b < e, b < i, c < d, c < g, g < h, g < i
fn select_2068([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_1148([d, b, g, h, e, a, f, i])
    } else {
        select_2067([b, a, c, d, f, e, g, h])
    }
}
/// n = 9, i = 2, b < e, b < i, c < d, c < f, c < i, f < g, f < h
fn select_2069([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_766([a, b, c, e, f, g, h, i])
    } else {
        select_326([a, b, c, d, f, g, h, i])
    }
}
/// n = 10, i = 4, a < e, b < g, b < h, c < f, c < h, d < f, d < g, d < h, h < i, h < j
fn select_2070([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_1021([a, b, c, d, e, f, h, i, j, g])
    } else {
        select_1021([a, c, b, d, e, g, h, i, j, f])
    }
}
/// n = 11, i = 4, a < g, a < k, b < e, b < j, c < f, c < h, d < f, d < g, d < h, h < i, h < j, h < k
fn select_2071([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_2069([f, b, h, i, e, a, g, k, j])
    } else {
        select_2070([b, a, c, d, e, f, g, h, i, k])
    }
}
/// n = 11, i = 4, a < k, b < f, b < j, c < g, c < h, d < e, d < g, d < h, h < i, h < j, h < k
fn select_2072([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_2068([e, b, c, g, f, a, h, i, j])
    } else {
        select_2071([a, b, c, d, f, g, e, h, i, j, k])
    }
}
/// n = 9, i = 3, a < f, b < d, b < e, c < e, c < f, e < g, f < h, f < i
fn select_2073([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_19([d, e, a, g])
    } else {
        select_185([b, a, c, d, f, h, i, e])
    }
}
/// n = 10, i = 3, a < g, b < g, b < h, c < d, c < e, e < f, e < h, g < i, g < j
fn select_2074([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_678([d, b, e, a, f, h])
    } else {
        select_2073([b, c, a, d, e, g, f, i, j])
    }
}
/// n = 9, i = 2, a < i, b < e, c < d, c < i, d < h, e < f, e < g
fn select_2075([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_461([b, a, c, d, h, i, e])
    } else {
        select_89([c, b, d, e, a, f, g, h])
    }
}
/// n = 7, i = 4, a < f, b < e, b < g, c < d, c < g, f < g
fn select_2076([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_52([d, b, a, f, e, g])
    } else {
        select_52([e, c, a, f, d, g])
    }
}
/// n = 9, i = 4, a < e, a < i, b < f, c < d, c < g, c < i, f < g, f < i, g < h
fn select_2077([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_2076([b, a, c, d, e, f, i])
    } else {
        select_58([d, a, f, g, h, i])
    }
}
/// n = 12, i = 4, a < d, a < l, b < f, b < h, c < e, c < g, c < l, e < h, f < g, f < l, g < k, h < i, h < j
fn select_2078([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < d {
        select_2075([a, e, f, g, h, i, j, k, l])
    } else {
        select_2077([a, b, c, e, d, f, g, k, l])
    }
}
/// n = 13, i = 4, a < d, a < g, b < e, b < j, c < f, c < g, c < i, e < i, f < j, g < h, i < m, j < k, j < l
fn select_2079([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_2074([f, b, a, d, g, h, j, e, k, l])
    } else {
        select_2078([a, b, c, d, f, e, i, j, k, l, m, g])
    }
}
/// n = 7, i = 2, a < f, b < c, b < d, b < e, f < g
fn select_2080([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < b {
        select_2([b, g])
    } else {
        select_2046([c, d, e, a, f])
    }
}
/// n = 10, i = 2, a < f, b < c, b < d, b < e, b < g, f < j, g < h, g < i
fn select_2081([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_349([a, c, d, e, g, h, i])
    } else {
        select_2080([a, b, c, d, e, f, j])
    }
}
/// n = 10, i = 4, a < e, a < j, b < d, b < f, c < f, c < h, e < h, f < g, f < j, h < i
fn select_2082([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_310([d, g, a, e, j])
    } else {
        select_1334([e, b, c, d, h, f, g, j, i])
    }
}
/// n = 13, i = 4, a < e, a < i, b < d, b < f, b < i, c < f, c < h, c < i, e < h, f < g, h < j, i < k, i < l, i < m
fn select_2083([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_1344([e, b, c, d, h, i, f, k, l, m, j])
    } else {
        select_2082([a, b, c, d, e, f, g, h, j, i])
    }
}
/// n = 13, i = 4, a < d, a < f, b < e, b < g, c < f, c < g, c < l, e < l, f < k, g < h, g < i, g < j, l < m
fn select_2084([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_2081([e, g, h, i, j, l, a, d, f, m])
    } else {
        select_2083([b, a, c, d, e, f, k, l, g, m, h, i, j])
    }
}
/// n = 13, i = 4, a < e, a < g, b < f, b < h, c < d, c < g, c < h, c < l, f < l, g < k, h < i, h < j, l < m
fn select_2085([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < h {
        select_2079([a, b, c, e, f, d, g, k, l, h, i, j, m])
    } else {
        select_2084([a, b, c, e, f, g, h, d, i, j, k, l, m])
    }
}
/// n = 14, i = 5, a < d, a < k, b < e, b < f, c < g, c < j, e < m, f < i, f < j, g < h, g < k, g < m, k < l, m < n
fn select_2086([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < g {
        select_2072([e, a, c, f, i, d, j, g, h, k, m])
    } else {
        select_2085([a, b, g, h, d, e, k, f, i, j, l, m, n])
    }
}
/// n = 14, i = 5, a < e, a < k, b < f, b < g, c < d, c < h, f < j, g < m, h < i, h < k, h < m, k < l, m < n
fn select_2087([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if d < f {
        select_2066([d, a, b, h, i, e, g, k, f, l, j, m, n])
    } else {
        select_2086([a, b, c, e, g, f, h, i, j, d, k, l, m, n])
    }
}
/// n = 7, i = 2, b < g, c < d, c < e, c < f, c < g
fn select_2088([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_59([a, b, c, f, d, e, g])
    } else {
        select_59([a, b, c, f, e, d, g])
    }
}
/// n = 10, i = 4, a < f, b < e, b < g, c < f, c < g, d < f, d < g, f < j, g < h, g < i
fn select_2089([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_34([e, g, a, h, i])
    } else {
        select_1627([b, a, c, d, e, f, j, g])
    }
}
/// n = 11, i = 4, a < i, b < i, b < j, c < e, c < f, d < f, d < i, f < g, f < h, f < j, i < k
fn select_2090([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_2088([e, b, f, a, g, h, j])
    } else {
        select_2089([b, c, a, d, e, i, f, g, h, k])
    }
}
/// n = 10, i = 5, a < e, a < j, b < g, b < h, c < g, c < h, d < f, d < j, f < h, g < j, h < i
fn select_2091([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_879([a, f, b, c, e, g, h, i, j])
    } else {
        select_157([e, b, c, d, f, g, j])
    }
}
/// n = 9, i = 4, a < d, a < g, b < f, b < g, c < e, c < g, e < h, f < h, g < i
fn select_2092([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_985([f, a, c, e, d, g, i, h])
    } else {
        select_150([d, b, c, e, f, g, i])
    }
}
/// n = 11, i = 5, a < i, b < e, b < h, c < g, c < i, d < f, d < h, f < i, g < h, h < j, i < k
fn select_2093([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_2091([b, a, c, d, e, f, g, i, k, h])
    } else {
        select_2092([b, d, g, e, a, f, h, i, j])
    }
}
/// n = 12, i = 5, a < j, a < l, b < f, b < h, c < g, c < h, d < e, d < h, d < j, g < j, h < i, h < l, j < k
fn select_2094([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_2090([g, a, b, d, f, h, e, i, j, l, k])
    } else {
        select_2093([a, b, d, c, f, g, e, h, j, i, k])
    }
}
/// n = 10, i = 4, a < j, b < f, b < i, c < g, d < e, d < g, g < h, g < i, g < j
fn select_2095([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < b {
        select_326([e, a, g, h, b, f, i, j])
    } else {
        select_1462([a, c, b, d, e, f, g, h, i])
    }
}
/// n = 11, i = 4, a < i, b < e, b < j, c < f, c < i, d < f, d < i, f < g, f < h, f < j, i < k
fn select_2096([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_344([b, f, g, h, e, a, i, j])
    } else {
        select_382([b, a, c, d, e, i, f, g, h, k])
    }
}
/// n = 11, i = 5, a < e, b < h, b < i, c < g, c < i, d < f, d < h, f < i, g < h, h < j, i < k
fn select_2097([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_1372([a, b, c, f, e, g, i, k, h])
    } else {
        select_1372([a, b, d, g, e, f, h, j, i])
    }
}
/// n = 12, i = 5, a < f, a < l, b < h, b < j, c < g, c < h, d < e, d < h, d < j, g < j, h < i, h < l, j < k
fn select_2098([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_2096([g, a, b, d, f, h, e, i, j, l, k])
    } else {
        select_2097([a, b, c, d, f, e, g, j, h, k, i])
    }
}
/// n = 13, i = 6, a < e, a < m, b < i, b < k, c < g, d < f, d < i, f < k, g < h, g < k, g < m, i < j, i < m, k < l
fn select_2099([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_2095([f, a, c, i, j, e, g, h, m, k])
    } else {
        select_2098([a, b, d, g, h, e, f, i, j, k, l, m])
    }
}
/// n = 14, i = 6, a < e, a < k, b < i, b < m, c < g, d < f, d < i, d < k, f < m, g < h, g < k, g < m, i < j, k < l, m < n
fn select_2100([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < i {
        select_2094([b, a, d, g, h, e, f, k, l, m, n, i])
    } else {
        select_2099([a, b, c, d, e, f, g, h, i, j, m, n, k])
    }
}
/// n = 7, i = 3, a < e, b < f, c < d, c < g, f < g
fn select_2101([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < f {
        select_16([b, c, a, e, d, f])
    } else {
        select_42([c, f, d, a, e, g])
    }
}
/// n = 9, i = 3, a < f, b < e, b < g, c < d, c < g, g < h, g < i
fn select_2102([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_2101([a, c, b, e, f, d, g])
    } else {
        select_610([a, c, b, e, f, g, d, h, i])
    }
}
/// n = 10, i = 3, b < e, b < h, b < j, c < d, c < h, c < i, d < f, d < j, e < g, e < i
fn select_2103([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_2028([a, b, d, f, h, e, j])
    } else {
        select_2028([a, c, e, g, h, d, i])
    }
}
/// n = 11, i = 3, b < e, b < h, b < i, c < d, c < h, c < i, c < k, d < g, e < f, e < k, i < j
fn select_2104([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < d {
        select_2069([a, e, c, h, f, i, d, j, k])
    } else {
        select_2103([a, b, c, d, e, g, f, h, k, i])
    }
}
/// n = 12, i = 4, a < d, a < e, a < k, b < c, b < f, d < h, e < g, e < l, f < i, f < k, f < l, i < j
fn select_2105([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < f {
        select_2102([e, b, d, h, c, g, f, i, k])
    } else {
        select_2104([c, a, f, i, e, g, j, k, d, h, l])
    }
}
/// n = 6, i = 2, c < d, c < e, e < f
fn select_2106([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < a {
        select_310([b, d, e, a, f])
    } else {
        select_82([b, a, c, d, e])
    }
}
/// n = 9, i = 3, b < e, c < f, d < e, d < f, e < i, f < g, f < h
fn select_2107([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_58([a, c, d, e, i, f])
    } else {
        select_48([a, b, d, f, g, h, e])
    }
}
/// n = 9, i = 3, b < g, c < g, c < h, d < e, e < f, e < h, g < i
fn select_2108([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < c {
        select_2106([a, b, e, f, c, h])
    } else {
        select_2107([a, b, d, c, g, e, f, h, i])
    }
}
/// n = 11, i = 5, b < g, c < g, c < i, d < e, e < f, e < k, f < i, g < h, g < k, i < j
fn select_2109([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_491([a, b, c, f, i, j, g])
    } else {
        select_163([a, d, g, h, e, f, k])
    }
}
/// n = 12, i = 6, a < h, b < e, b < j, b < l, c < h, c < j, d < f, f < g, f < l, g < j, h < i, h < l, j < k
fn select_2110([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < h {
        select_2109([e, a, c, d, f, g, h, i, j, k, l])
    } else {
        select_106([i, b, d, e, f, g, l])
    }
}
/// n = 13, i = 6, a < g, a < l, b < g, b < j, c < e, c < j, c < l, d < f, f < h, f < j, g < i, h < l, j < k, l < m
fn select_2111([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < g {
        select_2108([e, h, a, b, j, k, l, g, m])
    } else {
        select_2110([b, c, a, d, e, f, h, g, i, l, m, j])
    }
}
/// n = 9, i = 2, b < e, c < d, c < f, f < g, f < h, f < i
fn select_2112([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_398([a, d, f, b, g, h, i])
    } else {
        select_278([a, b, c, d, e, f])
    }
}
/// n = 10, i = 3, a < c, b < d, c < e, d < f, f < g, f < h, f < i, f < j
fn select_2113([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_189([a, g, h, i, j])
    } else {
        select_25([c, b, e, d, f])
    }
}
/// n = 9, i = 4, a < e, b < e, b < i, c < d, d < f, e < h, f < g, f < i
fn select_2114([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_194([a, g, b, i])
    } else {
        select_666([b, c, e, d, h, f, g])
    }
}
/// n = 12, i = 4, a < f, b < f, b < i, c < d, d < e, d < i, e < g, f < h, i < j, i < k, i < l
fn select_2115([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < e {
        select_2113([a, c, f, d, h, i, e, j, k, l])
    } else {
        select_2114([a, b, c, d, f, e, g, h, i])
    }
}
/// n = 9, i = 5, a < g, a < h, b < d, c < e, d < h, e < f, e < g, h < i
fn select_2116([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_449([a, d, c, e, f, h, i, g])
    } else {
        select_87([f, b, g, d])
    }
}
/// n = 13, i = 5, a < g, a < h, a < j, b < e, c < d, c < j, d < f, d < g, e < h, e < j, h < i, j < k, j < l, j < m
fn select_2117([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < j {
        select_2116([a, b, c, e, d, f, g, h, i])
    } else {
        select_548([c, e, h, d, i, g, j, k, l, m])
    }
}
/// n = 13, i = 5, a < e, a < j, b < f, c < d, c < h, c < j, e < g, f < h, f < j, h < i, j < k, j < l, j < m
fn select_2118([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < e {
        select_2115([d, a, b, f, h, e, i, g, j, k, l, m])
    } else {
        select_2117([c, b, a, e, f, g, d, h, i, j, k, l, m])
    }
}
/// n = 16, i = 7, a < h, a < j, b < f, b < o, c < g, d < e, d < j, d < l, f < h, f < l, f < p, g < j, g < l, h < i, j < k, j < o, j < p, k < m, l < n
fn select_2119([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < f {
        select_2112([e, k, b, o, m, f, h, l, p])
    } else {
        select_2118([a, c, d, e, h, g, i, l, n, j, k, o, p])
    }
}
/// n = 16, i = 7, a < i, a < k, b < f, b < h, c < e, c < k, c < l, d < g, f < i, f < l, f < p, g < h, g < k, g < l, i < j, k < m, k < p, l < n, m < o
fn select_2120([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < k {
        select_2111([a, f, c, d, e, g, i, h, j, l, n, k, m])
    } else {
        select_2119([a, b, d, c, e, f, g, i, j, k, m, l, o, n, h, p])
    }
}
/// n = 16, i = 7, a < e, a < n, b < i, b < j, c < f, c < h, d < g, f < i, f < n, f < o, g < h, g < j, g < n, i < k, j < l, j < o, l < m, n < p
fn select_2121([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < a {
        select_2105([j, c, h, l, a, f, e, m, i, k, o, n])
    } else {
        select_2120([b, c, a, d, e, f, g, h, i, k, j, n, l, p, m, o])
    }
}
/// n = 16, i = 7, a < e, a < o, b < j, b < k, c < g, d < f, d < i, f < h, f < j, f < o, g < i, g < k, g < o, j < l, k < m, m < n, o < p
fn select_2122([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < k {
        select_2100([a, b, c, f, e, h, g, i, j, l, o, p, k, m])
    } else {
        select_2121([a, b, d, c, e, f, g, i, j, k, l, m, n, o, h, p])
    }
}
/// n = 16, i = 7, a < l, b < e, b < o, c < g, d < f, d < j, f < h, f < l, f < o, g < i, g < j, g < o, i < k, k < m, l < n, o < p
fn select_2123([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < a {
        select_2087([b, i, d, j, e, k, a, f, h, m, o, p, l, n])
    } else {
        select_2122([b, a, c, d, e, f, g, h, j, l, i, n, k, m, o, p])
    }
}
/// n = 16, i = 7, b < e, b < o, c < g, d < f, d < l, f < h, f < i, f < o, g < j, g < l, g < o, i < k, j < m, m < n, o < p
fn select_2124([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < a {
        select_2045([b, c, d, e, f, g, h, i, j, a, k, l, m, n, o, p])
    } else {
        select_2123([a, b, c, d, e, f, g, h, j, l, m, i, n, k, o, p])
    }
}
/// n = 8, i = 3, a < e, b < d, b < h, c < f, f < g, f < h
fn select_2125([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < b {
        select_64([a, f, g, e, b, d, h])
    } else {
        select_609([a, c, b, d, e, f, g, h])
    }
}
/// n = 9, i = 3, a < e, b < d, b < h, c < f, c < h, f < g, h < i
fn select_2126([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < h {
        select_2125([a, b, c, d, e, f, g, h])
    } else {
        select_609([a, c, b, d, e, h, f, i])
    }
}
/// n = 10, i = 3, a < d, a < g, b < e, b < f, c < e, c < f, c < g, e < i, f < h, g < j
fn select_2127([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_659([d, b, c, g, j, e, f])
    } else {
        select_836([a, c, d, e, f, h, i, g])
    }
}
/// n = 10, i = 3, a < d, b < g, b < h, c < e, c < g, c < h, e < f, g < j, h < i
fn select_2128([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_1553([e, b, f, g, h, a, i, j, d])
    } else {
        select_2127([a, b, c, d, g, h, e, i, j, f])
    }
}
/// n = 11, i = 4, a < d, b < f, b < h, c < e, c < h, c < j, e < g, f < j, h < i, j < k
fn select_2129([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_2126([a, e, b, g, d, h, i, f, j])
    } else {
        select_2128([a, f, c, d, h, i, e, j, k, g])
    }
}
/// n = 11, i = 5, a < e, b < g, c < g, d < f, d < h, f < j, f < k, g < h, g < j, g < k, h < i
fn select_2130([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_372([e, b, c, d, f, g, h])
    } else {
        select_115([a, d, e, h, f, j, k, i])
    }
}
/// n = 11, i = 5, a < e, b < f, b < i, c < g, d < g, f < k, g < h, g < i, g < k, i < j
fn select_2131([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_109([a, b, g, e, h, i, j, f])
    } else {
        select_2130([a, c, d, b, e, f, g, i, j, h, k])
    }
}
/// n = 9, i = 5, a < h, b < g, b < i, c < e, d < f, e < g, f < h, h < i
fn select_2132([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < h {
        select_158([a, b, e, d, f, g, h])
    } else {
        select_11([b, c, h, e, i])
    }
}
/// n = 9, i = 5, a < g, b < h, c < f, d < e, d < i, e < g, f < h, h < i
fn select_2133([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_372([a, b, f, d, e, h, i])
    } else {
        select_637([b, a, e, c, f, g])
    }
}
/// n = 9, i = 5, a < f, b < h, c < e, c < i, d < g, g < h, h < i
fn select_2134([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_2132([b, c, a, d, f, g, e, h, i])
    } else {
        select_2133([a, b, d, c, e, g, f, h, i])
    }
}
/// n = 11, i = 5, a < f, b < h, c < g, c < i, d < e, d < h, g < k, h < i, h < k, i < j
fn select_2135([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_2131([a, c, b, d, f, g, h, e, i, j, k])
    } else {
        select_2134([a, b, c, d, g, f, e, h, i])
    }
}
/// n = 12, i = 5, a < f, b < e, c < h, d < g, d < i, g < k, h < i, h < k, i < j, k < l
fn select_2136([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < b {
        select_2129([a, d, h, f, b, g, e, i, j, k, l])
    } else {
        select_2135([a, c, d, b, e, f, g, h, i, j, k])
    }
}
/// n = 12, i = 5, a < g, b < f, c < i, c < k, d < i, d < k, e < h, e < i, h < k, i < j, k < l
fn select_2137([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < d {
        select_2136([a, b, c, e, f, g, h, d, i, j, k, l])
    } else {
        select_2136([a, b, d, e, f, g, h, c, i, j, k, l])
    }
}
/// n = 9, i = 2, a < d, a < e, a < h, a < i, b < c, b < f, b < h, b < i, f < g
fn select_2138([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_222([b, a, d, e, c, h, i])
    } else {
        select_244([a, b, c, e, f, g, h, i])
    }
}
/// n = 9, i = 2, a < g, b < f, b < h, b < i, c < d, c < e, c < h, c < i
fn select_2139([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_318([c, a, g, d, e, b, f])
    } else {
        select_2138([c, b, f, d, e, a, g, h, i])
    }
}
/// n = 9, i = 3, a < d, a < h, a < i, b < c, b < e, b < h, d < g, e < f, e < i
fn select_2140([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_278([c, a, e, f, h, i])
    } else {
        select_284([a, b, c, e, d, g, h, i])
    }
}
/// n = 11, i = 3, a < c, a < e, a < j, a < k, b < d, b < f, b < j, d < h, e < i, f < g, f < k
fn select_2141([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_2139([d, f, a, c, j, g, h, e, k])
    } else {
        select_2140([b, a, c, d, e, i, h, j, f])
    }
}
/// n = 10, i = 2, a < d, a < f, a < i, a < j, b < c, b < e, b < i, b < j, e < g, f < h
fn select_2142([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_244([a, b, c, d, e, g, i, j])
    } else {
        select_244([b, a, d, c, f, h, i, j])
    }
}
/// n = 11, i = 3, b < d, b < g, b < j, b < k, c < e, c < g, e < f, e < j, e < k, f < h, g < i
fn select_2143([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_2142([b, e, a, d, f, g, h, i, j, k])
    } else {
        select_1158([a, b, c, d, g, e, i, f])
    }
}
/// n = 13, i = 4, a < e, a < f, a < j, a < m, b < d, b < g, b < j, c < f, c < g, c < j, f < h, g < i, g < m, i < k, j < l
fn select_2144([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_39([e, b, c, d, f, h, j, l, g])
    } else {
        select_2143([d, a, c, e, g, i, j, k, l, f, m])
    }
}
/// n = 13, i = 4, a < e, a < l, b < d, b < h, b < k, b < l, c < f, c < h, c < l, f < g, f < k, g < i, h < j, l < m
fn select_2145([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_2141([b, f, d, g, h, a, e, i, j, k, l])
    } else {
        select_2144([b, a, c, e, d, h, f, j, g, l, i, m, k])
    }
}
/// n = 10, i = 5, a < h, b < f, c < e, c < j, d < g, f < j, g < h, g < j, h < i
fn select_2146([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_1171([c, b, a, g, f, h, i, e, j])
    } else {
        select_1957([a, b, d, c, e, f, g, j, h])
    }
}
/// n = 10, i = 5, a < h, b < g, c < f, c < j, d < h, d < j, e < h, e < j, g < j, h < i
fn select_2147([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_2146([a, b, c, d, f, g, e, h, i, j])
    } else {
        select_2146([a, b, c, e, f, g, d, h, i, j])
    }
}
/// n = 11, i = 4, b < j, b < k, c < h, c < i, d < f, e < f, f < g, f < h, f < i, f < j, f < k
fn select_2148([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_927([a, b, d, e, c, f, g, j, k])
    } else {
        select_927([a, c, d, e, b, f, g, h, i])
    }
}
/// n = 13, i = 5, a < l, a < m, b < g, b < k, c < f, c < h, d < g, d < h, e < g, e < h, g < j, h < i, h < k, h < l, h < m
fn select_2149([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_1138([a, c, b, d, e, f, g, j, h])
    } else {
        select_2148([f, a, b, d, e, h, i, g, k, l, m])
    }
}
/// n = 13, i = 5, a < g, a < m, b < h, b < l, c < f, c < i, d < h, d < i, e < h, e < i, h < k, i < j, i < l, i < m
fn select_2150([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < i {
        select_2147([b, a, c, d, e, f, g, h, k, i])
    } else {
        select_2149([a, b, c, d, e, f, h, i, j, k, l, g, m])
    }
}
/// n = 15, i = 6, a < e, a < n, b < i, b < k, c < i, c < k, d < f, d < g, d < n, f < h, g < k, g < m, i < j, i < m, i < n, k < l, n < o
fn select_2151([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < f {
        select_2145([a, i, d, j, e, g, k, f, l, h, m, n, o])
    } else {
        select_2150([a, g, f, b, c, h, e, k, i, j, l, m, n])
    }
}
/// n = 15, i = 6, a < e, a < n, b < j, b < l, c < j, c < l, d < f, d < g, d < n, f < i, g < h, g < l, j < k, j < n, l < m, n < o
fn select_2152([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < j {
        select_2137([a, f, b, c, g, i, e, h, l, m, j, k])
    } else {
        select_2151([a, b, c, d, e, f, g, i, j, k, l, m, h, n, o])
    }
}
/// n = 8, i = 2, b < h, c < g, d < e, d < g, d < h, e < f
fn select_2153([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_275([a, b, d, c, e, f, h])
    } else {
        select_275([a, c, d, b, e, f, g])
    }
}
/// n = 9, i = 3, a < i, b < f, c < f, c < g, c < i, d < e, d < g, d < i, g < h
fn select_2154([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < d {
        select_278([a, c, d, e, f, g])
    } else {
        select_2153([e, a, b, c, g, h, f, i])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < g, b < h, c < d, c < h, d < f, d < g
fn select_2155([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_19([a, b, f, g])
    } else {
        select_4([b, c, d, e, h])
    }
}
/// n = 10, i = 4, a < f, a < g, a < j, b < e, b < h, b < i, b < j, c < d, c < f, d < j, e < g, f < h, f < i
fn select_2156([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_2155([d, b, a, f, e, h, i, j])
    } else {
        select_256([e, a, c, d, g, f, h, i])
    }
}
/// n = 11, i = 4, a < f, a < g, a < h, b < e, b < h, b < j, b < k, c < d, c < f, c < h, e < g, f < j, f < k, h < i
fn select_2157([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < d {
        select_1944([e, c, g, f, h, d, i, j, k])
    } else {
        select_2156([a, b, c, d, e, f, g, j, k, h])
    }
}
/// n = 13, i = 4, a < e, a < i, a < l, a < m, b < f, b < g, b < i, c < d, c < f, c < i, e < g, f < h, f < l, f < m, h < j, i < k
fn select_2158([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < e {
        select_1161([d, a, f, h, i, j, k, l, m])
    } else {
        select_2157([b, a, c, d, e, f, g, i, k, l, m])
    }
}
/// n = 13, i = 4, a < e, a < f, a < j, a < m, b < g, b < h, b < j, c < d, c < g, c < j, f < h, g < i, g < m, i < k, j < l
fn select_2159([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < g {
        select_2154([e, f, b, c, d, h, j, l, g])
    } else {
        select_2158([a, b, c, d, f, g, h, i, j, k, l, e, m])
    }
}
/// n = 13, i = 4, a < e, a < l, b < d, b < f, b < j, b < l, c < g, c < k, c < l, f < k, g < h, g < j, h < i, l < m
fn select_2160([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_2141([b, g, d, h, f, a, e, i, k, j, l])
    } else {
        select_2159([b, c, a, e, d, f, g, k, h, l, i, m, j])
    }
}
/// n = 9, i = 4, a < i, b < e, c < f, c < h, d < f, d < h, e < h, f < g, f < i
fn select_2161([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < b {
        select_122([g, a, b, e, i])
    } else {
        select_491([a, e, c, d, f, g, h])
    }
}
/// n = 9, i = 5, a < h, a < i, b < h, b < i, c < g, d < f, e < g, e < i, f < i, g < h
fn select_2162([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_391([a, b, c, e, f, g, i])
    } else {
        select_302([a, b, d, g, f, h])
    }
}
/// n = 10, i = 5, a < f, b < g, c < h, c < j, d < h, d < j, e < g, e < h, g < j, h < i
fn select_2163([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_2161([a, b, c, d, g, h, i, j, f])
    } else {
        select_2162([c, d, b, a, e, f, g, j, h])
    }
}
/// n = 6, i = 3, b < f, c < f, d < e, d < f
fn select_2164([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < c {
        select_53([a, b, d, e, c, f])
    } else {
        select_53([a, c, d, e, b, f])
    }
}
/// n = 9, i = 4, a < e, b < g, b < i, c < g, c < i, d < f, d < g, f < i, g < h
fn select_2165([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_2164([e, b, c, d, f, g])
    } else {
        select_109([a, c, d, e, f, g, h, i])
    }
}
/// n = 11, i = 5, a < f, b < h, c < g, d < h, d < j, e < h, e < j, g < j, h < i, j < k
fn select_2166([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < g {
        select_2163([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_2165([a, d, e, g, f, b, j, k, h])
    }
}
/// n = 10, i = 3, a < d, a < j, b < f, b < i, b < j, c < e, c < f, c < j, e < i, f < g, f < h
fn select_2167([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_1400([a, b, c, d, e, f, i, j])
    } else {
        select_1058([a, b, c, d, f, e, g, h, j])
    }
}
/// n = 10, i = 3, a < d, a < j, b < e, b < f, b < j, c < e, c < f, c < j, e < i, f < g, f < h
fn select_2168([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_185([a, b, c, d, e, f, i, j])
    } else {
        select_1058([a, b, c, d, f, e, g, h, j])
    }
}
/// n = 10, i = 3, a < e, a < j, b < f, b < g, b < j, c < d, c < f, c < g, c < j, g < h, g < i
fn select_2169([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_2167([a, b, c, e, d, g, h, i, f, j])
    } else {
        select_2168([a, b, c, e, f, g, h, i, d, j])
    }
}
/// n = 12, i = 5, a < f, b < i, c < g, c < i, d < g, d < i, e < g, e < i, g < h, i < j, i < k, i < l
fn select_2170([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_118([a, g, b, h, i, j, k, l])
    } else {
        select_1855([b, a, c, d, e, f, g, h])
    }
}
/// n = 14, i = 6, a < f, a < n, b < h, c < g, c < l, c < n, d < h, d < j, e < h, e < j, g < j, g < m, h < i, h < l, h < m, h < n, j < k
fn select_2171([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < g {
        select_2169([a, c, h, i, f, l, g, j, m, n])
    } else {
        select_2170([a, b, d, e, g, f, j, k, h, i, l, m])
    }
}
/// n = 14, i = 6, a < f, a < n, b < h, b < i, c < i, c < k, d < i, d < k, e < g, e < h, e < n, g < k, g < m, i < j, i < m, i < n, k < l
fn select_2172([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_2166([a, g, b, c, d, f, h, k, l, i, j])
    } else {
        select_2171([a, b, e, c, d, f, g, i, j, k, l, h, m, n])
    }
}
/// n = 15, i = 6, a < h, b < f, b < n, c < i, c < k, d < i, d < k, e < g, e < h, e < n, g < k, g < m, i < j, i < m, i < n, k < l, n < o
fn select_2173([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < a {
        select_2160([b, i, e, j, f, a, g, k, l, m, h, n, o])
    } else {
        select_2172([b, a, c, d, e, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 15, i = 6, a < i, b < f, b < n, c < j, c < l, d < j, d < l, e < g, e < i, e < n, g < h, g < l, j < k, j < n, l < m, n < o
fn select_2174([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < j {
        select_2137([a, b, c, d, g, f, i, h, l, m, j, k])
    } else {
        select_2173([a, b, c, d, e, f, g, i, j, k, l, m, h, n, o])
    }
}
/// n = 15, i = 6, b < g, b < n, c < j, c < l, d < j, d < l, e < f, e < h, e < n, h < i, h < l, j < k, j < n, l < m, n < o
fn select_2175([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < a {
        select_2152([b, c, d, e, g, f, h, i, a, j, k, l, m, n, o])
    } else {
        select_2174([a, b, c, d, e, g, h, i, f, j, k, l, m, n, o])
    }
}
/// n = 8, i = 3, a < f, b < d, b < g, c < e, c < f, e < g, f < h
fn select_2176([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_58([d, a, c, e, g, f])
    } else {
        select_51([b, c, e, d, f, h])
    }
}
/// n = 8, i = 3, a < e, b < f, c < d, c < h, e < h, f < g
fn select_2177([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_51([c, f, g, d, a, e])
    } else {
        select_2176([b, c, a, d, e, f, h, g])
    }
}
/// n = 10, i = 5, a < i, a < j, b < g, c < f, c < h, d < g, d < j, e < g, e < j, f < j, g < h, g < i
fn select_2178([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_1104([a, b, d, e, f, g, j])
    } else {
        select_154([a, c, g, f, h, i])
    }
}
/// n = 9, i = 3, a < h, a < i, b < e, c < d, c < g, d < f, d < h, e < g, e < i
fn select_2179([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_5([a, b, d, f, e, h])
    } else {
        select_154([a, c, e, d, g, i])
    }
}
/// n = 11, i = 5, a < f, a < k, b < h, c < g, c < j, d < g, d < h, e < g, e < h, g < i, h < j, h < k
fn select_2180([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_2178([c, b, a, d, e, f, h, k, j, g])
    } else {
        select_2179([a, b, c, g, h, i, j, f, k])
    }
}
/// n = 11, i = 5, a < i, a < k, b < f, b < j, c < h, d < g, d < h, e < g, e < h, g < i, h < j, h < k
fn select_2181([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_2177([c, g, b, f, h, a, i, j])
    } else {
        select_2180([b, c, a, d, e, f, g, h, i, k, j])
    }
}
/// n = 7, i = 3, a < e, b < f, b < g, c < f, c < g, d < e, d < f, d < g
fn select_2182([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_255([a, d, b, c, e, f, g])
    } else {
        select_255([a, d, c, b, e, f, g])
    }
}
/// n = 10, i = 4, a < g, a < j, b < f, b < i, c < f, c < i, d < e, d < h, e < g, e < i, f < h, f < j
fn select_2183([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_154([a, d, f, e, h, j])
    } else {
        select_2182([a, b, c, e, g, f, i])
    }
}
/// n = 10, i = 5, a < h, a < j, b < g, c < g, d < f, d < i, e < g, e < h, f < h, g < i, g < j
fn select_2184([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_391([b, c, a, e, f, h, g])
    } else {
        select_154([a, d, g, f, i, j])
    }
}
/// n = 11, i = 5, a < f, a < k, b < g, b < i, c < h, c < j, d < h, d < j, e < g, e < h, g < j, h < i, h < k
fn select_2185([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_2183([a, c, d, b, g, h, f, i, j, k])
    } else {
        select_2184([b, c, d, a, e, f, h, g, k, i])
    }
}
/// n = 11, i = 5, a < f, a < k, b < h, c < g, c < j, d < h, d < i, e < h, e < i, g < i, h < j, h < k
fn select_2186([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < g {
        select_2185([a, c, d, e, b, f, g, h, j, i, k])
    } else {
        select_802([a, d, e, g, f, b, i, h])
    }
}
/// n = 11, i = 5, a < g, a < k, b < f, b < j, c < i, d < h, d < i, e < h, e < i, i < j, i < k
fn select_2187([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_2181([b, a, c, d, e, g, h, i, f, k, j])
    } else {
        select_2186([a, c, b, d, e, g, f, i, h, j, k])
    }
}
/// n = 10, i = 3, a < e, a < g, a < j, b < c, b < d, b < g, b < i, d < j, e < f, e < i, g < h
fn select_2188([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_659([c, d, a, g, h, e, j])
    } else {
        select_244([e, b, c, f, g, h, d, i])
    }
}
/// n = 10, i = 3, a < g, b < d, b < h, b < j, c < e, c < g, c < h, e < f, e < j, h < i
fn select_2189([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1048([a, c, e, b, d, f, g, h])
    } else {
        select_2188([c, b, d, a, e, f, h, i, j, g])
    }
}
/// n = 11, i = 4, a < h, b < e, b < k, c < f, c < h, d < f, d < h, f < g, f < i, f < k, i < j
fn select_2190([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_396([a, f, g, h, i, b, e, k, j])
    } else {
        select_930([a, c, d, b, e, h, f, g, i, k])
    }
}
/// n = 12, i = 5, a < g, a < h, b < g, b < h, c < d, c < e, c < j, e < f, e < l, h < i, h < j, h < l, j < k
fn select_2191([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < d {
        select_2189([g, h, c, i, e, f, d, j, k, l])
    } else {
        select_2190([d, e, a, b, f, h, i, g, j, k, l])
    }
}
/// n = 12, i = 4, a < h, a < l, b < e, b < k, c < f, c < h, d < f, d < h, f < g, f < i, f < k, f < l, i < j
fn select_2192([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_365([a, f, g, h, i, b, e, k, j, l])
    } else {
        select_930([a, c, d, b, e, h, f, g, i, k])
    }
}
/// n = 13, i = 5, a < g, a < h, b < g, b < h, c < d, c < e, c < j, d < m, e < f, e < l, h < i, h < j, h < l, h < m, j < k
fn select_2193([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < d {
        select_2189([g, h, c, i, e, f, d, j, k, l])
    } else {
        select_2192([d, e, a, b, f, h, i, g, j, k, l, m])
    }
}
/// n = 14, i = 5, a < e, a < k, b < f, b < j, c < g, c < h, d < g, d < h, f < m, h < i, h < j, h < k, h < m, j < l, k < n
fn select_2194([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_2191([c, d, a, e, b, f, g, h, i, k, n, j])
    } else {
        select_2193([c, d, b, f, a, e, g, h, i, j, l, k, m])
    }
}
/// n = 14, i = 5, b < f, b < k, c < g, c < j, d < h, d < i, e < h, e < i, g < m, i < j, i < k, i < m, j < l, k < n
fn select_2195([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < i {
        select_2187([b, c, a, d, e, g, f, h, i, j, k])
    } else {
        select_2194([b, c, d, e, f, g, h, i, a, j, k, l, m, n])
    }
}
/// n = 9, i = 3, b < d, b < i, c < e, c < f, e < h, f < g, f < i
fn select_2196([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_571([a, b, c, d, e, h, f, g])
    } else {
        select_107([a, b, c, d, f, g, e, h, i])
    }
}
/// n = 11, i = 3, b < e, b < k, c < d, c < f, e < h, f < g, f < i, f < k, i < j
fn select_2197([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_1540([a, d, f, g, i, b, e, k, j])
    } else {
        select_120([a, c, b, d, e, h, f, g, i, k])
    }
}
/// n = 12, i = 3, b < e, b < f, c < d, c < g, e < j, f < i, g < h, g < k, k < l
fn select_2198([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_2196([a, c, b, d, e, f, i, j, g])
    } else {
        select_2197([a, b, c, d, e, g, h, j, k, l, f])
    }
}
/// n = 8, i = 2, a < c, a < d, b < e, e < f, e < g, e < h
fn select_2199([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_89([a, b, c, e, f, g, h, d])
    } else {
        select_89([a, b, d, e, f, g, h, c])
    }
}
/// n = 11, i = 4, a < e, a < f, b < e, b < g, c < f, c < g, d < f, d < g, f < k, g < h, g < i, g < j
fn select_2200([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_33([a, h, i, j])
    } else {
        select_126([b, c, d, e, f, k, g])
    }
}
/// n = 11, i = 4, a < e, b < e, b < h, c < f, c < h, d < f, d < h, f < g, h < i, h < j, h < k
fn select_2201([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_2199([f, b, a, g, h, i, j, k])
    } else {
        select_2200([a, b, c, d, e, f, h, i, j, k, g])
    }
}
/// n = 11, i = 4, b < e, b < h, c < f, c < h, d < f, d < h, f < g, h < i, h < j, h < k
fn select_2202([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_1210([c, d, b, e, a, f, g, h, i, j, k])
    } else {
        select_2201([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 12, i = 4, b < f, b < l, c < e, c < g, d < f, d < g, f < i, g < h, g < j, g < l, j < k
fn select_2203([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_1540([a, e, g, h, j, b, f, l, k])
    } else {
        select_2202([a, c, b, d, e, f, i, g, h, j, l])
    }
}
/// n = 14, i = 5, b < f, b < h, c < e, c < i, d < g, f < l, g < h, g < i, g < l, h < k, i < j, i < m, m < n
fn select_2204([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < f {
        select_2203([a, b, c, g, e, h, i, j, k, m, n, f])
    } else {
        select_778([a, c, d, b, e, f, g, h, k, l, i])
    }
}
/// n = 14, i = 5, b < e, b < m, c < f, c < j, d < g, f < l, g < h, g < j, g < l, h < i, h < m, j < k, m < n
fn select_2205([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < b {
        select_2198([a, c, h, i, f, j, b, e, k, l, m, n])
    } else {
        select_2204([a, c, b, d, e, f, g, j, h, i, k, l, m, n])
    }
}
/// n = 14, i = 5, b < i, b < m, c < f, c < m, d < g, d < j, e < h, e < i, g < l, h < j, h < l, h < m, j < k, m < n
fn select_2206([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < h {
        select_2195([a, c, d, b, e, f, g, i, h, j, m, k, l, n])
    } else {
        select_2205([a, c, d, e, f, g, h, b, i, j, k, l, m, n])
    }
}
/// n = 10, i = 2, c < d, c < e, c < f, d < j, e < i, f < g, f < h
fn select_2207([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_681([a, b, c, e, d, f, j, i])
    } else {
        select_427([a, b, c, e, f, d, g, h, i])
    }
}
/// n = 11, i = 3, b < e, b < k, c < d, c < f, f < g, f < h, f < k, g < j, h < i
fn select_2208([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_2207([a, d, f, g, h, b, e, k, i, j])
    } else {
        select_119([a, b, c, d, e, f, g, h, k])
    }
}
/// n = 12, i = 4, b < d, b < g, c < e, c < g, d < f, d < l, e < h, e < i, e < l, h < k, i < j
fn select_2209([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < b {
        select_343([a, h, i, b, g, j, k, d, f, l])
    } else {
        select_2208([a, d, c, g, f, e, h, i, j, k, l])
    }
}
/// n = 9, i = 2, b < d, b < h, b < i, c < e, c < h, c < i, e < f, e < g
fn select_2210([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_37([a, c, b, d, h, i, e])
    } else {
        select_300([b, c, d, e, a, f, g, h, i])
    }
}
/// n = 12, i = 4, b < f, b < i, c < e, c < i, c < l, d < f, d < h, d < i, e < h, f < g, f < l, i < j, i < k
fn select_2211([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_2210([a, f, c, g, i, j, k, e, l])
    } else {
        select_2036([a, e, b, d, h, i, j, k, f])
    }
}
/// n = 12, i = 4, b < g, b < l, c < e, c < f, c < k, d < f, d < g, d < i, e < i, f < h, f < l, g < j, g < k
fn select_2212([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < b {
        select_435([a, e, f, h, i, b, g, l])
    } else {
        select_2211([a, b, c, d, e, g, j, i, f, h, l, k])
    }
}
/// n = 9, i = 4, a < e, a < h, b < f, b < i, c < d, c < g, c < h, c < i, e < i, f < g, f < h
fn select_2213([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_181([a, b, c, d, f, e, i, h])
    } else {
        select_82([d, a, f, g, h])
    }
}
/// n = 7, i = 5, a < g, b < g, c < g, d < f, e < f
fn select_2214([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_8([a, b, c, f])
    } else {
        select_7([d, e, g])
    }
}
/// n = 9, i = 6, a < f, b < h, c < h, d < h, e < g, g < i, h < i
fn select_2215([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_782([a, b, c, d, g, f, h, i])
    } else {
        select_2214([b, c, d, e, f, g, h])
    }
}
/// n = 11, i = 6, b < f, b < j, c < h, d < h, e < g, e < k, f < k, g < i, g < j, h < i, h < j, h < k
fn select_2216([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_2213([b, e, h, a, f, g, i, j, k])
    } else {
        select_2215([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, a < d, a < j, b < f, b < j, c < e, c < g, e < h, e < i, e < j, f < g, f < h, f < i
fn select_2217([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_64([a, c, g, d, e, h, i])
    } else {
        select_708([d, b, c, e, f, g, j])
    }
}
/// n = 10, i = 4, b < j, c < f, c < j, d < e, d < g, e < h, e < i, e < j, f < g, f < h, f < i
fn select_2218([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_295([a, c, d, b, e, f, g, h, i])
    } else {
        select_2217([b, c, d, a, e, f, g, h, i, j])
    }
}
/// n = 14, i = 7, b < f, b < j, c < i, d < i, e < g, e < l, f < n, g < h, g < m, g < n, h < k, i < j, i < k, i < n, j < l, j < m
fn select_2219([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_2216([a, b, c, d, g, f, h, i, k, j, n])
    } else {
        select_2218([a, f, i, e, g, j, l, h, m, n])
    }
}
/// n = 15, i = 7, b < f, b < j, c < i, d < i, e < g, e < n, g < h, g < k, g < o, h < m, i < j, i < k, i < m, j < n, j < o, k < l
fn select_2220([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < f {
        select_2212([a, b, g, i, h, k, j, l, m, n, o, f])
    } else {
        select_2219([a, b, c, d, e, f, g, h, i, j, m, n, o, k])
    }
}
/// n = 15, i = 7, b < f, b < o, c < i, d < i, e < g, e < k, g < h, g < l, g < o, h < n, i < j, i < l, i < n, j < k, j < o, l < m
fn select_2221([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < b {
        select_2209([a, j, e, b, g, f, k, h, l, m, n, o])
    } else {
        select_2220([a, b, c, d, e, f, g, h, i, j, l, m, n, k, o])
    }
}
/// n = 9, i = 2, a < e, a < h, a < i, b < c, b < d, b < h, b < i, d < f, e < g
fn select_2222([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_172([c, d, a, h, i, f])
    } else {
        select_29([b, a, c, e, g, h, i])
    }
}
/// n = 10, i = 2, a < c, a < f, a < j, b < d, b < e, b < j, d < h, e < g, f < i
fn select_2223([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_28([e, d, a, h, g])
    } else {
        select_2222([b, a, c, f, e, i, g, d, j])
    }
}
/// n = 10, i = 3, a < e, a < g, b < d, b < f, c < e, c < f, c < g, c < i, d < i, f < h, g < j
fn select_2224([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_45([b, c, e, d, g, j, f])
    } else {
        select_363([d, a, c, f, h, e, g, i])
    }
}
/// n = 11, i = 3, a < g, b < d, b < h, b < j, c < e, c < g, c < h, c < k, d < k, e < f, e < j, h < i
fn select_2225([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_2223([e, b, f, d, h, a, i, k, g, j])
    } else {
        select_2224([a, b, c, d, g, h, e, i, k, f])
    }
}
/// n = 11, i = 3, a < c, a < d, a < j, b < e, b < k, d < k, e < f, e < g, e < j, f < i, g < h
fn select_2226([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_625([f, g, a, c, d, j, h, i])
    } else {
        select_71([c, d, b, e, f, g, j, k])
    }
}
/// n = 11, i = 3, a < f, b < d, b < k, c < e, c < f, e < g, e < h, e < k, g < j, h < i
fn select_2227([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_747([a, c, b, e, d, f, g, h, i, j])
    } else {
        select_2226([b, c, d, a, e, g, h, i, j, k, f])
    }
}
/// n = 12, i = 4, a < d, a < f, b < c, b < e, b < i, b < k, e < g, e < l, f < h, f < i, f < l, h < k, i < j
fn select_2228([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < c {
        select_2225([d, f, b, h, e, g, c, i, j, l, k])
    } else {
        select_2227([c, e, a, g, f, d, h, i, j, k, l])
    }
}
/// n = 9, i = 4, a < f, a < g, b < e, b < g, b < i, c < d, c < g, c < i, f < i, g < h
fn select_2229([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_48([d, f, b, g, e, h, i])
    } else {
        select_181([a, b, c, d, e, f, i, g])
    }
}
/// n = 11, i = 4, a < f, a < g, a < h, b < e, b < g, b < h, c < d, c < g, c < h, g < k, h < i, h < j
fn select_2230([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < d {
        select_427([e, f, c, g, h, d, i, j, k])
    } else {
        select_2229([c, a, b, e, f, d, g, k, h])
    }
}
/// n = 12, i = 5, a < g, a < h, a < j, b < e, b < g, c < d, c < h, c < j, e < f, e < j, e < l, h < i, h < l, j < k
fn select_2231([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_119([d, b, h, i, g, e, f, j, l])
    } else {
        select_2230([a, c, e, f, d, g, j, h, i, l, k])
    }
}
/// n = 14, i = 5, a < d, a < k, b < f, b < h, c < e, c < h, c < k, e < g, e < j, e < m, f < i, f < j, f < k, i < m, j < l, k < n
fn select_2232([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < a {
        select_2228([b, e, g, h, a, f, d, i, j, l, m, k])
    } else {
        select_2231([c, b, a, d, f, i, h, e, g, k, n, j])
    }
}
/// n = 10, i = 3, a < d, a < g, b < c, b < g, b < j, c < f, d < e, d < j, f < h, g < i
fn select_2233([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < c {
        select_29([d, b, e, g, i, c, j])
    } else {
        select_209([a, c, f, g, h, i, d])
    }
}
/// n = 13, i = 6, a < f, a < k, b < g, c < g, d < e, d < h, d < k, e < j, f < h, f < i, g < i, g < j, g < k, j < l, k < m
fn select_2234([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_2233([a, d, e, f, i, j, k, l, m, h])
    } else {
        select_973([b, c, d, e, h, g, i, k])
    }
}
/// n = 9, i = 4, a < e, a < i, b < f, b < g, c < d, c < f, d < h, e < g, f < h, f < i
fn select_2235([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_457([a, d, b, f, e, h, i])
    } else {
        select_139([e, b, c, d, g, f, i])
    }
}
/// n = 9, i = 5, a < f, b < f, c < e, c < i, d < e, d < h, e < g, f < h, f < i
fn select_2236([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_4([c, d, h, i, e])
    } else {
        select_87([g, a, b, f])
    }
}
/// n = 13, i = 7, a < f, a < m, b < h, c < h, d < g, d < k, e < g, e < i, f < j, f < k, g < j, g < l, g < m, h < i, h < k, h < m, i < l
fn select_2237([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < f {
        select_2235([a, d, e, i, f, g, k, l, m])
    } else {
        select_2236([b, c, d, e, g, h, j, i, k])
    }
}
/// n = 14, i = 7, a < j, b < f, b < l, c < h, d < h, e < g, e < k, f < i, f < k, g < i, g < l, g < n, h < j, h < k, h < l, j < n, l < m
fn select_2238([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < a {
        select_2234([b, c, d, g, a, f, h, i, k, j, l, n, m])
    } else {
        select_2237([b, c, d, e, a, f, g, h, j, i, k, n, l])
    }
}
/// n = 9, i = 3, a < e, b < f, b < h, c < d, c < f, c < i, d < h, e < i, f < g
fn select_2239([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_5([b, e, c, d, i, f])
    } else {
        select_110([a, c, e, d, f, g, h])
    }
}
/// n = 12, i = 6, a < i, b < h, b < j, c < g, d < g, e < f, e < j, e < l, f < h, g < i, g < j, i < l, j < k
fn select_2240([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_2239([a, b, e, f, i, j, k, h, l])
    } else {
        select_165([a, b, c, d, h, g, j, i])
    }
}
/// n = 14, i = 7, a < j, b < f, b < l, c < i, d < i, e < g, e < k, f < k, g < h, g < l, g < n, i < j, i < k, i < l, j < n, l < m
fn select_2241([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < h {
        select_2238([a, b, c, d, e, f, g, i, h, j, k, l, m, n])
    } else {
        select_2240([a, b, c, d, g, h, i, f, j, l, m, n])
    }
}
/// n = 9, i = 4, a < d, a < i, b < f, b < i, c < e, c < g, e < h, e < i, f < g, f < h
fn select_2242([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_51([a, c, g, d, e, h])
    } else {
        select_708([d, b, c, e, f, g, i])
    }
}
/// n = 10, i = 4, a < d, a < i, a < j, b < f, b < g, c < e, c < g, c < j, e < h, f < h, f < j, g < i
fn select_2243([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_38([f, a, c, d, g, e, i, j])
    } else {
        select_2242([a, c, b, d, f, e, g, h, j])
    }
}
/// n = 9, i = 3, a < e, a < g, b < d, b < f, c < e, c < f, c < g, f < h, g < i
fn select_2244([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < c {
        select_3([a, c, f])
    } else {
        select_251([a, b, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, a < d, a < i, b < f, b < h, b < i, c < e, c < g, c < i, e < h, f < g, i < j
fn select_2245([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_2244([e, a, b, d, h, i, f, j, g])
    } else {
        select_2244([f, a, c, d, g, i, e, j, h])
    }
}
/// n = 11, i = 4, a < g, b < h, b < i, c < e, c < i, c < k, d < f, d < g, d < i, f < h, g < k, i < j
fn select_2246([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_2243([c, a, d, e, f, b, g, h, k, i])
    } else {
        select_2245([c, b, d, e, f, a, g, h, i, j])
    }
}
/// n = 14, i = 7, a < k, b < f, b < l, c < i, d < i, e < g, e < j, g < h, g < l, g < n, i < j, i < k, i < l, k < n, l < m
fn select_2247([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < j {
        select_2241([a, b, c, d, e, f, g, h, i, k, j, l, m, n])
    } else {
        select_2246([a, b, g, i, h, j, k, f, l, m, n])
    }
}
/// n = 16, i = 7, b < f, b < m, c < i, d < i, e < g, e < k, g < h, g < l, g < m, h < o, i < j, i < k, i < m, j < l, j < o, l < n, m < p
fn select_2248([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < a {
        select_2232([b, e, i, f, j, g, a, k, h, l, m, n, o, p])
    } else {
        select_2247([a, b, c, d, e, f, g, h, i, k, j, m, p, l])
    }
}
/// n = 16, i = 7, b < f, b < o, c < i, c < j, d < i, d < j, e < g, e < k, g < h, g < l, g < o, h < n, i < l, i < n, j < k, j < o, l < m, o < p
fn select_2249([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_2221([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o])
    } else {
        select_2248([a, b, c, d, e, f, g, h, j, i, k, l, o, m, n, p])
    }
}
/// n = 16, i = 7, b < f, b < o, c < g, c < l, d < i, d < j, e < i, e < j, g < h, g < m, g < o, i < l, i < o, j < k, j < m, m < n, o < p
fn select_2250([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < h {
        select_2206([a, i, b, j, c, f, k, g, l, m, n, h, o, p])
    } else {
        select_2249([a, b, d, e, c, f, g, h, j, i, l, m, n, k, o, p])
    }
}
/// n = 16, i = 7, b < f, b < o, c < j, c < k, d < j, d < k, e < g, e < n, g < h, g < i, g < o, i < l, j < m, k < n, k < o, o < p
fn select_2251([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < j {
        select_2175([a, b, c, d, g, h, f, i, l, k, n, j, m, o, p])
    } else {
        select_2250([a, b, e, c, d, f, g, h, k, j, m, n, i, l, o, p])
    }
}
/// n = 16, i = 7, b < k, c < f, c < o, d < g, d < k, e < h, e < m, g < m, g < o, h < i, h < j, h < o, j < l, k < n, o < p
fn select_2252([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < b {
        select_2124([a, c, d, e, f, h, g, i, j, b, l, m, k, n, o, p])
    } else {
        select_2251([a, c, b, d, e, f, h, i, j, k, g, l, n, m, o, p])
    }
}
/// n = 10, i = 5, a < f, b < h, c < g, c < h, d < g, d < h, e < g, e < h, h < i, h < j
fn select_2253([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_47([a, g, b, h, i, j])
    } else {
        select_1097([b, c, d, e, a, f, g])
    }
}
/// n = 11, i = 5, a < g, a < k, b < f, b < j, c < h, d < g, d < h, e < g, e < h, h < i, h < j, h < k
fn select_2254([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_36([b, h, i, f, a, g, k, j])
    } else {
        select_2253([b, c, a, d, e, f, g, h, i, k])
    }
}
/// n = 10, i = 5, a < f, b < g, b < j, c < h, d < g, d < h, e < g, e < h, h < i, h < j
fn select_2255([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_1167([a, b, c, g, h, i, j, f])
    } else {
        select_1094([c, b, d, e, a, f, g, h])
    }
}
/// n = 11, i = 5, b < k, c < g, c < j, d < h, e < g, e < h, f < g, f < h, h < i, h < j, h < k
fn select_2256([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < a {
        select_2254([c, b, d, e, f, a, g, h, i, k, j])
    } else {
        select_2255([a, c, d, e, f, b, g, h, i, j])
    }
}
/// n = 10, i = 4, a < f, b < h, c < g, d < g, d < h, e < f, e < g, e < h, g < i, h < j
fn select_2257([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_1090([a, d, e, b, c, f, h, j, g])
    } else {
        select_1090([a, d, e, c, b, f, g, i, h])
    }
}
/// n = 11, i = 4, b < f, c < i, c < k, d < g, d < i, e < f, e < g, e < i, g < h, g < k, i < j
fn select_2258([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_717([b, c, d, f, g, a, h, i, j, k])
    } else {
        select_2257([b, a, c, d, e, f, i, g, j, h])
    }
}
/// n = 13, i = 6, b < j, c < i, c < l, d < h, e < g, e < l, f < h, f < i, g < i, g < k, h < j, h < k, h < l, l < m
fn select_2259([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_2256([a, b, c, d, f, g, i, h, k, l, j])
    } else {
        select_2258([a, b, c, e, h, j, g, k, l, m, i])
    }
}
/// n = 9, i = 4, b < g, c < f, d < f, d < g, e < f, e < g, f < h, g < i
fn select_2260([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_491([a, c, d, e, g, i, f])
    } else {
        select_491([a, b, d, e, f, h, g])
    }
}
/// n = 11, i = 5, b < h, b < i, c < g, d < f, e < g, e < i, f < i, f < k, g < h, g < k, i < j
fn select_2261([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_2260([a, b, c, e, f, g, i, k, j])
    } else {
        select_371([a, b, d, g, h, f, k])
    }
}
/// n = 9, i = 4, a < g, b < f, c < f, c < g, d < e, d < g, d < i, f < i, g < h
fn select_2262([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_372([a, b, c, d, e, f, i])
    } else {
        select_368([b, e, a, c, g, h])
    }
}
/// n = 9, i = 4, b < g, c < f, d < g, d < i, e < f, e < g, f < i, g < h
fn select_2263([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < a {
        select_2262([b, c, e, d, a, f, g, h, i])
    } else {
        select_158([b, c, e, a, d, f, g])
    }
}
/// n = 11, i = 5, b < i, b < j, c < g, c < h, d < g, d < j, e < f, f < h, f < j, g < i, j < k
fn select_2264([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_2261([a, b, c, e, d, f, g, i, j, k, h])
    } else {
        select_2263([a, b, c, d, f, h, j, k, g])
    }
}
/// n = 9, i = 4, a < g, b < f, b < g, c < f, c < h, d < e, e < h, h < i
fn select_2265([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < e {
        select_2101([a, d, c, f, g, e, h])
    } else {
        select_461([a, b, c, h, i, f, g])
    }
}
/// n = 10, i = 4, a < g, a < h, b < e, c < d, c < g, d < f, d < i, e < h, e < i, h < j
fn select_2266([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_25([c, e, g, h, j])
    } else {
        select_2101([a, b, d, f, g, e, i])
    }
}
/// n = 10, i = 4, b < g, b < h, c < e, c < g, d < f, e < j, f < h, f < j, h < i
fn select_2267([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_2265([a, c, b, d, f, g, e, h, i])
    } else {
        select_2266([b, d, c, e, f, a, g, h, j, i])
    }
}
/// n = 11, i = 5, b < i, c < h, c < j, d < g, e < f, e < j, f < h, g < i, g < j, j < k
fn select_2268([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < f {
        select_2264([a, c, b, e, d, g, f, i, h, j, k])
    } else {
        select_2267([a, c, f, d, b, g, h, j, k, i])
    }
}
/// n = 13, i = 6, b < j, c < h, c < l, d < g, d < l, e < i, f < h, f < i, g < k, i < j, i < k, i < l, l < m
fn select_2269([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_2259([a, b, c, e, d, f, g, i, h, j, k, l, m])
    } else {
        select_2268([a, b, d, e, c, h, i, g, j, l, m])
    }
}
/// n = 8, i = 5, c < h, d < g, e < g, f < g, g < h
fn select_2270([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < a {
        select_1503([b, d, e, f, c, a, g, h])
    } else {
        select_1097([b, d, e, f, a, c, g])
    }
}
/// n = 10, i = 6, b < j, c < i, d < h, e < h, e < i, f < g, g < i, g < j, h < j
fn select_2271([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_1505([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_2270([a, b, d, c, e, g, i, h])
    }
}
/// n = 11, i = 7, b < k, c < j, d < i, e < g, f < h, g < j, g < k, h < i, h < k
fn select_2272([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < j {
        select_2271([a, c, b, d, h, e, g, i, k, j])
    } else {
        select_2271([a, d, b, c, g, f, h, j, k, i])
    }
}
/// n = 11, i = 7, b < k, c < i, d < j, e < g, f < h, g < j, h < i, h < k, j < k
fn select_2273([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if j < i {
        select_1505([a, b, c, d, g, f, h, j, i, k])
    } else {
        select_1492([a, b, d, c, h, e, g, i, j])
    }
}
/// n = 11, i = 7, b < k, c < j, d < g, e < h, f < i, h < k, i < j, i < k
fn select_2274([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_2272([a, b, c, d, f, e, i, h, g, j, k])
    } else {
        select_2273([a, b, c, e, d, f, g, i, j, h, k])
    }
}
/// n = 14, i = 7, b < g, c < j, c < m, d < k, e < h, e < m, f < i, h < l, i < j, i < k, k < l, k < m, m < n
fn select_2275([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < g {
        select_2269([a, b, c, e, d, i, h, j, k, g, l, m, n])
    } else {
        select_2274([a, d, c, e, b, f, h, g, i, j, k])
    }
}
/// n = 10, i = 7, a < g, b < g, c < h, d < h, e < f, f < j, g < j, h < i
fn select_2276([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < j {
        select_182([a, b, e, h, i, f, g])
    } else {
        select_7([c, d, j])
    }
}
/// n = 13, i = 7, a < g, b < g, b < l, b < m, c < h, d < h, e < f, f < j, f < k, g < i, h < j, h < k, k < l, k < m
fn select_2277([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < k {
        select_2276([c, d, a, b, e, f, h, g, i, j])
    } else {
        select_956([a, j, b, k, l, m])
    }
}
/// n = 11, i = 7, a < g, b < g, c < h, d < h, e < f, e < k, f < j, g < j, g < k, h < i, h < k
fn select_2278([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < k {
        select_2276([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_87([i, e, g, k])
    }
}
/// n = 13, i = 7, a < g, b < h, c < h, d < g, d < k, e < f, e < k, f < j, f < m, g < i, h < j, h < k, k < l, k < m
fn select_2279([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < k {
        select_2278([b, c, a, d, e, f, h, g, i, j, k])
    } else {
        select_1167([a, f, d, j, k, l, m, g])
    }
}
/// n = 14, i = 7, a < g, b < g, b < l, b < n, c < h, d < h, e < f, e < l, f < j, f < k, g < i, h < j, h < k, h < l, k < n, l < m
fn select_2280([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < l {
        select_2277([a, b, c, d, e, f, g, h, i, j, k, l, n])
    } else {
        select_2279([a, c, d, b, e, f, g, h, i, j, l, m, k])
    }
}
/// n = 11, i = 5, c < g, d < g, e < f, e < h, f < j, f < k, g < h, g < j, g < k, h < i
fn select_2281([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_2130([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_2130([b, c, d, e, a, f, g, h, i, j, k])
    }
}
/// n = 11, i = 4, a < d, a < i, b < c, b < i, b < k, c < f, c < g, d < e, d < h, h < k, i < j
fn select_2282([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_1452([a, c, f, g, i, j, d, e, h])
    } else {
        select_602([b, d, e, h, i, j, c, f, g, k])
    }
}
/// n = 14, i = 7, a < i, b < i, c < f, c < l, d < e, d < l, d < n, e < g, e < h, f < j, f < k, i < j, i < k, i < l, k < n, l < m
fn select_2283([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < i {
        select_2281([g, h, a, b, c, f, i, l, m, j, k])
    } else {
        select_2282([c, d, e, f, j, g, h, k, l, m, n])
    }
}
/// n = 14, i = 7, b < f, b < l, b < n, c < i, d < i, e < g, e < l, f < h, g < j, g < k, i < j, i < k, i < l, k < n, l < m
fn select_2284([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < f {
        select_2280([a, b, c, d, e, g, f, i, h, j, k, l, m, n])
    } else {
        select_2283([c, d, e, b, f, g, a, h, i, j, k, l, m, n])
    }
}
/// n = 10, i = 5, a < g, b < g, c < d, c < e, e < f, e < j, g < h, g < i, g < j
fn select_2285([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_308([d, e, a, b, f, g, i, j])
    } else {
        select_1671([c, g, h, i, d, e, j])
    }
}
/// n = 13, i = 7, a < i, b < i, c < f, c < l, d < e, d < g, f < j, f < k, g < h, g < l, i < j, i < k, i < l, l < m
fn select_2286([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < f {
        select_2281([e, h, a, b, c, f, i, l, m, j, k])
    } else {
        select_2285([a, b, d, e, g, h, i, j, k, l])
    }
}
/// n = 10, i = 5, a < h, b < g, b < h, b < i, c < f, c < j, d < e, d < h, e < i, f < g, f < i, h < j
fn select_2287([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_1868([e, a, b, c, f, h, j, i])
    } else {
        select_775([a, b, f, d, e, g, i])
    }
}
/// n = 8, i = 3, a < f, b < f, b < g, c < d, c < e, e < g, f < h
fn select_2288([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < b {
        select_88([a, d, e, b, g])
    } else {
        select_1158([a, c, b, d, e, f, g, h])
    }
}
/// n = 10, i = 5, a < i, b < e, b < i, c < g, c < h, c < i, d < f, d < j, f < g, f < h, i < j
fn select_2289([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_2287([a, c, d, b, e, f, h, i, g, j])
    } else {
        select_2288([a, b, f, h, g, i, e, j])
    }
}
/// n = 12, i = 7, a < h, b < f, b < h, c < i, d < i, e < g, e < l, g < j, g < k, h < l, i < j, i < k, i < l
fn select_2290([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_2289([a, b, i, e, f, g, j, k, h, l])
    } else {
        select_1503([f, c, d, h, e, g, i, l])
    }
}
/// n = 13, i = 7, b < f, b < h, c < i, d < i, e < g, e < l, g < j, g < k, h < l, i < j, i < k, i < l, l < m
fn select_2291([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_2286([c, d, e, b, f, g, h, a, i, j, k, l, m])
    } else {
        select_2290([a, b, c, d, e, f, g, h, i, j, k, l])
    }
}
/// n = 14, i = 7, b < g, c < l, c < n, d < i, e < i, f < h, f < l, h < j, h < k, i < j, i < k, i < l, k < n, l < m
fn select_2292([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if c < b {
        select_2284([a, c, d, e, f, b, h, g, i, j, k, l, m, n])
    } else {
        select_2291([a, b, d, e, f, g, h, c, i, j, k, l, m])
    }
}
/// n = 10, i = 4, b < j, c < f, d < g, d < i, e < f, e < g, f < i, g < h, g < j
fn select_2293([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < c {
        select_59([a, b, g, h, c, f, j])
    } else {
        select_1711([a, b, d, c, e, f, g, h, i])
    }
}
/// n = 10, i = 5, a < h, a < j, b < g, b < i, b < j, c < f, d < e, d < j, e < h, f < g, f < j, h < i
fn select_2294([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_488([b, e, c, a, f, h, i, j])
    } else {
        select_856([a, b, f, d, e, g, j])
    }
}
/// n = 10, i = 5, a < f, b < h, b < j, c < e, c < i, c < j, d < g, d < j, f < j, g < h, h < i
fn select_2295([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_2294([b, c, a, d, g, f, e, h, i, j])
    } else {
        select_2133([a, b, d, c, e, g, f, h, i])
    }
}
/// n = 10, i = 5, b < i, b < j, c < f, d < h, d < j, e < g, e < j, f < j, g < h, h < i
fn select_2296([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < a {
        select_2295([c, d, b, e, a, f, g, h, i, j])
    } else {
        select_1387([d, a, c, e, f, b, g, h])
    }
}
/// n = 11, i = 5, b < f, c < i, c < k, d < h, d < i, e < g, e < i, g < h, h < k, i < j
fn select_2297([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < f {
        select_2293([a, b, g, c, d, h, i, j, k, f])
    } else {
        select_2296([a, c, b, d, e, f, g, h, k, i])
    }
}
/// n = 11, i = 4, b < e, c < g, c < i, d < f, d < g, f < i, f < k, g < h, g < k, i < j
fn select_2298([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_717([b, f, c, e, g, a, h, i, j, k])
    } else {
        select_1744([b, a, c, d, e, f, i, g, j, h])
    }
}
/// n = 8, i = 3, b < d, b < g, c < e, c < h, d < h, e < f, e < g
fn select_2299([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < d {
        select_567([a, b, c, e, f, d, g])
    } else {
        select_97([b, c, e, d, f, a, h, g])
    }
}
/// n = 10, i = 5, b < h, b < j, c < f, c < g, d < g, d < h, e < g, e < h, g < j, h < i
fn select_2300([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_1011([a, f, b, d, e, g, j, h])
    } else {
        select_2299([a, c, b, g, h, i, f, j])
    }
}
/// n = 12, i = 5, b < f, c < h, c < j, d < g, d < h, e < g, e < h, g < j, h < i, j < k, j < l
fn select_2301([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < b {
        select_767([a, c, g, b, h, f, i, j, k, l])
    } else {
        select_2300([a, c, b, d, e, f, g, h, i, j])
    }
}
/// n = 12, i = 5, b < f, c < h, c < l, d < h, d < i, e < g, e < h, g < i, h < k, i < j, i < l
fn select_2302([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < c {
        select_2298([a, b, d, g, f, c, i, j, h, k, l])
    } else {
        select_2301([a, b, d, c, e, f, g, h, k, i, j, l])
    }
}
/// n = 12, i = 5, b < f, c < h, c < j, d < i, d < j, e < g, e < j, g < h, g < i, i < k, j < l
fn select_2303([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < i {
        select_2297([a, b, d, c, e, f, g, h, j, l, i])
    } else {
        select_2302([a, b, c, d, e, f, g, j, i, k, l, h])
    }
}
/// n = 14, i = 7, b < g, c < l, c < m, d < h, d < m, e < i, f < i, h < k, i < j, i < k, i < m, j < l, m < n
fn select_2304([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_2292([a, b, c, e, f, d, g, h, i, k, j, m, n, l])
    } else {
        select_2303([a, b, c, d, i, g, j, l, h, m, k, n])
    }
}
/// n = 14, i = 7, b < g, c < k, c < m, d < j, e < i, e < m, f < h, f < j, h < k, i < l, j < l, j < m, m < n
fn select_2305([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < j {
        select_2275([a, b, c, d, e, f, g, i, h, k, j, l, m, n])
    } else {
        select_2304([a, b, c, e, d, f, g, i, j, h, l, k, m, n])
    }
}
/// n = 11, i = 5, a < g, a < k, b < h, c < g, c < h, d < e, d < i, d < k, e < f, h < i, h < k, i < j
fn select_2306([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_837([a, b, d, g, e, h, i, j, f, k])
    } else {
        select_158([b, a, c, e, f, g, h])
    }
}
/// n = 12, i = 5, a < g, a < i, b < g, b < h, c < h, c < i, d < e, d < i, d < k, e < f, h < k, i < j, k < l
fn select_2307([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_1168([e, b, c, a, f, g, i, j, h])
    } else {
        select_2306([a, c, b, d, e, f, g, h, k, l, i])
    }
}
/// n = 12, i = 6, a < f, a < i, b < f, b < g, c < h, d < h, e < g, e < i, g < j, g < k, h < i, h < j, h < k, i < l
fn select_2308([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_1117([a, c, d, e, f, g, h, j, k])
    } else {
        select_26([b, e, h, f, i, l])
    }
}
/// n = 9, i = 4, a < f, a < i, b < g, b < i, c < f, c < g, d < e, d < g, e < i, g < h
fn select_2309([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_158([b, c, a, d, e, f, i])
    } else {
        select_1572([a, e, c, b, g, h, i])
    }
}
/// n = 12, i = 6, a < f, a < j, b < f, b < g, c < g, c < j, d < h, e < h, g < l, h < i, h < j, h < l, j < k
fn select_2310([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_2308([a, b, d, e, c, f, g, h, j, i, l, k])
    } else {
        select_2309([b, c, a, h, i, f, j, k, g])
    }
}
/// n = 10, i = 6, a < i, b < h, b < j, c < g, c < j, d < g, d < h, e < f, f < i, i < j
fn select_2311([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < i {
        select_1387([a, b, c, e, g, h, f, i])
    } else {
        select_21([b, c, d, g, h, j])
    }
}
/// n = 12, i = 6, a < g, a < j, b < g, b < h, c < i, d < h, d < j, e < f, e < i, h < l, i < j, i < l, j < k
fn select_2312([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < f {
        select_2310([a, b, d, c, e, g, h, i, f, j, k, l])
    } else {
        select_2311([c, a, d, b, e, f, h, g, i, j])
    }
}
/// n = 13, i = 6, a < f, b < h, b < j, c < h, c < i, d < i, d < j, e < g, g < j, g < l, i < l, j < k, l < m
fn select_2313([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < a {
        select_2307([b, c, d, g, a, f, h, i, j, k, l, m])
    } else {
        select_2312([b, c, e, d, a, f, h, i, g, j, k, l])
    }
}
/// n = 8, i = 3, a < h, b < d, c < e, c < g, d < g, e < f, e < h
fn select_2314([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_474([b, c, e, d, a, f, g])
    } else {
        select_567([b, a, c, e, f, d, h])
    }
}
/// n = 8, i = 4, a < f, b < e, c < f, c < g, d < e, d < h, e < g, f < h
fn select_2315([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_87([b, d, f, h])
    } else {
        select_87([a, c, e, g])
    }
}
/// n = 9, i = 4, a < h, a < i, b < f, b < g, c < e, d < f, d < g, d < i, e < g, e < i, f < h
fn select_2316([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_2315([b, c, d, a, e, f, i, h])
    } else {
        select_470([b, d, e, a, f, i, g])
    }
}
/// n = 10, i = 4, a < e, b < f, b < j, c < g, c < i, d < f, d < g, d < j, e < j, f < i, g < h
fn select_2317([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_2314([a, b, c, f, g, h, i, e])
    } else {
        select_2316([c, b, a, d, e, f, j, i, g])
    }
}
/// n = 11, i = 5, a < f, b < h, b < i, c < e, c < k, d < g, e < h, f < k, g < i, g < k, i < j
fn select_2318([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_1122([a, d, b, e, f, h, g, i])
    } else {
        select_2317([a, c, b, g, f, e, i, j, h, k])
    }
}
/// n = 9, i = 4, b < f, b < i, c < f, c < g, c < i, d < e, e < g, e < i, g < h
fn select_2319([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_659([a, b, c, g, h, f, i])
    } else {
        select_163([a, d, c, f, e, g, i])
    }
}
/// n = 11, i = 5, b < e, b < j, c < g, c < h, c < j, d < f, e < g, f < h, f < j, h < i, j < k
fn select_2320([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_2319([a, e, c, d, f, g, j, k, h])
    } else {
        select_275([a, h, b, i, j, k, e])
    }
}
/// n = 12, i = 5, a < l, b < g, b < h, c < e, c < i, d < f, e < g, f < h, f < i, h < k, i < j, i < l
fn select_2321([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < b {
        select_59([e, a, i, j, b, h, l])
    } else {
        select_2320([a, c, b, d, e, f, g, h, k, i, j])
    }
}
/// n = 12, i = 5, a < e, b < h, b < j, c < f, c < i, d < g, f < h, g < i, g < j, i < k, j < l
fn select_2322([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_2318([a, b, c, d, f, e, g, h, j, l, i])
    } else {
        select_2321([a, b, c, d, f, g, h, j, i, k, l, e])
    }
}
/// n = 13, i = 6, a < i, b < f, c < i, c < k, d < h, e < g, e < k, g < j, h < j, h < k, j < l, k < m
fn select_2323([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < g {
        select_2313([b, c, a, e, d, f, h, i, g, k, m, j, l])
    } else {
        select_2322([b, c, g, d, f, a, h, i, j, k, l, m])
    }
}
/// n = 9, i = 5, a < g, a < h, b < g, b < h, b < i, c < f, d < e, e < g, e < i, f < h, f < i
fn select_2324([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_52([a, b, d, e, h, i])
    } else {
        select_52([a, b, c, f, g, i])
    }
}
/// n = 10, i = 6, a < h, b < g, b < i, c < h, c < i, d < g, d < i, d < j, e < f, f < i, f < j, h < j
fn select_2325([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_2324([b, d, a, e, f, h, i, g, j])
    } else {
        select_1511([a, g, c, e, f, i, h])
    }
}
/// n = 11, i = 6, a < g, a < k, b < h, b < j, c < g, c < i, d < h, d < i, e < f, f < i, f < k, h < k, i < j
fn select_2326([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < i {
        select_2325([b, c, d, a, e, f, g, h, i, k])
    } else {
        select_42([b, i, h, a, g, j])
    }
}
/// n = 10, i = 5, a < g, a < j, b < g, b < h, c < f, d < e, d < h, e < j, f < h, f < j, h < i
fn select_2327([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_1572([a, e, b, f, h, i, j])
    } else {
        select_528([b, a, c, d, e, f, g, j])
    }
}
/// n = 10, i = 4, a < e, a < g, b < g, b < h, b < i, c < e, c < f, d < f, d < g, f < h, f < i, g < j
fn select_2328([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_642([b, c, d, e, g, j, f])
    } else {
        select_1305([a, b, c, f, g, h, i, j, e])
    }
}
/// n = 11, i = 5, a < g, a < i, b < g, b < h, c < e, c < i, c < k, d < f, f < h, f < i, h < k, i < j
fn select_2329([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_2327([b, a, d, c, e, f, g, i, j, h])
    } else {
        select_2328([a, c, b, f, g, h, i, e, k, j])
    }
}
/// n = 13, i = 6, a < m, b < h, b < j, c < h, c < i, d < g, e < f, e < i, f < j, f < k, g < i, g < j, i < k, i < m, j < l
fn select_2330([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < f {
        select_2326([b, a, c, e, d, g, h, f, i, m, j])
    } else {
        select_2329([b, c, f, d, a, g, h, i, j, l, k])
    }
}
/// n = 8, i = 3, a < f, a < g, a < h, b < e, b < f, b < g, c < d, c < e, d < h
fn select_2331([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_206([a, b, c, d, e, f, g, h])
    } else {
        select_206([a, b, c, d, e, g, f, h])
    }
}
/// n = 8, i = 3, a < g, a < h, b < e, b < f, c < d, c < f, c < g, e < h
fn select_2332([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_317([d, b, e, f, a, g])
    } else {
        select_2331([a, c, b, e, f, d, g, h])
    }
}
/// n = 8, i = 3, a < f, b < e, b < h, c < d, c < f, c < g, d < h, e < g
fn select_2333([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_42([e, c, a, d, h, g])
    } else {
        select_143([b, a, c, d, f, e, h])
    }
}
/// n = 10, i = 3, a < f, a < g, b < e, b < f, b < i, c < d, c < g, c < h, d < i, e < h, g < j
fn select_2334([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_985([b, a, c, d, f, g, j, i])
    } else {
        select_2333([a, c, b, e, d, f, i, h])
    }
}
/// n = 10, i = 3, a < g, b < h, b < i, c < f, c < h, d < e, d < f, d < g, e < i, h < j
fn select_2335([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_2332([c, a, d, e, b, g, f, h])
    } else {
        select_2334([c, d, b, a, e, f, h, i, g, j])
    }
}
/// n = 13, i = 6, a < k, b < h, c < h, c < l, d < g, e < f, e < i, f < j, f < l, g < i, g < l, i < j, i < k, l < m
fn select_2336([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < i {
        select_2330([a, c, b, d, e, f, g, h, i, l, j, m, k])
    } else {
        select_2335([a, c, f, i, b, j, k, l, h, m])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, c < f, d < g, d < h, e < f, e < i, f < h, g < i
fn select_2337([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_153([a, c, e, g, f, i])
    } else {
        select_153([a, b, d, f, g, h])
    }
}
/// n = 11, i = 5, a < k, b < h, b < k, c < g, d < f, e < g, e < h, f < h, f < j, g < j, g < k, h < i
fn select_2338([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_2262([b, c, e, f, a, g, h, i, j])
    } else {
        select_2337([b, c, d, e, a, f, g, h, k])
    }
}
/// n = 11, i = 6, a < k, b < h, c < f, d < h, d < i, e < g, f < i, g < i, g < k, h < k, i < j
fn select_2339([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_1686([a, c, b, d, e, f, g, h, k])
    } else {
        select_562([a, b, d, f, g, i, j])
    }
}
/// n = 12, i = 6, a < h, b < i, c < h, c < j, d < f, e < g, f < j, f < l, g < i, g < j, i < l, j < k
fn select_2340([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_2338([a, c, b, d, g, f, i, j, k, l, h])
    } else {
        select_2339([b, a, d, c, e, f, g, h, j, k, i])
    }
}
/// n = 8, i = 5, a < h, b < g, c < g, d < g, d < h, e < f, f < h
fn select_2341([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < f {
        select_158([a, c, d, e, f, g, h])
    } else {
        select_302([b, c, a, d, h, g])
    }
}
/// n = 10, i = 6, a < i, a < j, b < h, c < h, d < f, e < g, f < i, g < i, g < j, h < j
fn select_2342([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_2341([a, b, c, f, e, g, h, i])
    } else {
        select_158([a, d, h, e, g, f, j])
    }
}
/// n = 9, i = 5, a < i, b < h, b < i, c < f, d < g, d < h, e < g, e < h, f < h, g < i
fn select_2343([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_925([b, c, d, e, a, f, g, i])
    } else {
        select_856([b, d, e, f, a, g, h])
    }
}
/// n = 9, i = 5, a < i, b < i, c < f, d < g, d < h, e < g, e < h, f < h, g < i
fn select_2344([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < h {
        select_2343([b, a, c, d, e, f, g, h, i])
    } else {
        select_88([b, g, f, h, a])
    }
}
/// n = 10, i = 6, a < h, b < h, c < i, c < j, d < f, e < g, f < j, g < i, g < j
fn select_2345([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < i {
        select_2342([c, a, b, d, e, f, g, h, j, i])
    } else {
        select_2344([a, b, d, c, g, f, i, j, h])
    }
}
/// n = 12, i = 6, a < i, b < f, c < i, c < k, d < h, e < g, e < j, f < j, g < k, h < j, h < k, k < l
fn select_2346([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_2340([a, b, c, d, e, h, g, i, f, k, l, j])
    } else {
        select_2345([a, c, e, d, b, h, f, i, g, j])
    }
}
/// n = 13, i = 6, a < i, b < f, c < i, c < l, d < h, e < g, e < j, g < k, g < l, h < j, h < l, j < k, l < m
fn select_2347([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < f {
        select_2336([b, a, c, d, e, g, h, i, j, k, f, l, m])
    } else {
        select_2346([a, b, c, d, e, f, g, h, i, j, l, m])
    }
}
/// n = 13, i = 6, a < i, b < g, c < j, d < k, d < m, e < i, e < k, f < h, h < j, h < k, j < m, k < l
fn select_2348([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < c {
        select_2323([a, b, e, f, d, g, c, h, i, j, k, m, l])
    } else {
        select_2347([a, b, e, f, c, g, d, h, i, j, m, k, l])
    }
}
/// n = 8, i = 2, c < g, c < h, d < e, d < g, d < h, e < f
fn select_2349([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_811([a, b, c, d, e, g, h])
    } else {
        select_681([a, b, d, e, c, g, h, f])
    }
}
/// n = 10, i = 3, b < f, b < h, b < j, c < d, c < e, c < h, e < j, f < g, h < i
fn select_2350([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_2349([a, d, e, b, h, i, f, j])
    } else {
        select_39([a, c, b, d, f, g, h, i, e])
    }
}
/// n = 8, i = 4, a < e, a < h, b < f, b < g, c < d, c < g, c < h, f < h
fn select_2351([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_127([f, a, c, g, e, d, h])
    } else {
        select_16([d, a, b, g, e, f])
    }
}
/// n = 9, i = 4, a < f, a < h, b < e, b < g, b < i, c < d, c < g, c < h, h < i
fn select_2352([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < e {
        select_32([d, b, a, f, g, h, e, i])
    } else {
        select_2351([a, b, c, d, f, e, g, h])
    }
}
/// n = 11, i = 4, a < e, b < f, b < h, b < i, c < d, c < g, c < h, f < k, g < i, g < k, i < j
fn select_2353([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_2350([d, g, b, h, f, a, e, i, j, k])
    } else {
        select_2352([a, b, c, d, f, e, h, g, i])
    }
}
/// n = 10, i = 3, a < c, a < f, a < j, b < d, b < e, b < j, d < h, e < g, f < i
fn select_2354([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_1045([e, d, a, c, j, h, g, f])
    } else {
        select_2140([b, a, c, e, f, i, g, j, d])
    }
}
/// n = 9, i = 4, a < i, b < e, b < h, b < i, c < f, c < i, d < f, d < i, f < g, f < h
fn select_2355([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_105([b, f, a, g, e, h])
    } else {
        select_1627([b, a, c, d, e, f, g, i])
    }
}
/// n = 12, i = 6, a < f, a < k, a < l, b < h, b < i, b < l, c < g, d < e, d < i, e < l, f < h, g < i, g < l, i < j, i < k
fn select_2356([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_2355([e, a, b, g, f, i, j, k, l])
    } else {
        select_528([f, b, c, d, e, g, h, l])
    }
}
/// n = 11, i = 4, a < g, a < k, b < f, b < g, c < d, c < f, c < j, d < e, f < h, f < k, g < i, g < j
fn select_2357([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_64([d, f, h, e, a, g, k])
    } else {
        select_853([a, b, c, d, e, g, i, f, h, k, j])
    }
}
/// n = 13, i = 6, a < e, a < j, b < f, b < i, b < m, c < h, c < i, c < j, d < g, f < h, g < i, g < j, i < k, j < l, j < m
fn select_2358([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < i {
        select_2356([b, c, d, a, e, f, g, h, j, l, m, i])
    } else {
        select_2357([a, g, b, f, h, i, j, k, l, m, e])
    }
}
/// n = 13, i = 6, a < e, b < f, b < k, b < m, c < h, c < i, c < k, d < g, f < h, g < i, g < k, i < j, i < m, k < l
fn select_2359([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < a {
        select_2354([i, b, j, f, k, a, l, h, e, m])
    } else {
        select_2358([a, b, c, d, e, f, g, h, k, i, l, j, m])
    }
}
/// n = 13, i = 6, a < e, b < j, b < l, c < f, c < k, c < l, d < g, f < j, g < h, g < l, h < i, h < k, l < m
fn select_2360([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < b {
        select_2353([a, c, h, i, e, f, b, k, l, m, j])
    } else {
        select_2359([a, c, b, d, e, f, g, j, h, i, l, m, k])
    }
}
/// n = 10, i = 3, a < e, a < h, b < d, b < f, b < g, c < f, c < g, c < h, g < i, h < j
fn select_2361([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < g {
        select_459([b, a, c, e, d, f, h, j, g])
    } else {
        select_1152([a, b, c, e, f, h, g, d, i, j])
    }
}
/// n = 11, i = 4, a < e, a < g, b < d, b < h, c < f, f < g, f < h, g < k, h < i, h < j
fn select_2362([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_54([b, c, a, e, d, f, g, k])
    } else {
        select_1334([a, b, f, d, g, h, i, j, k])
    }
}
/// n = 12, i = 5, a < d, a < k, b < e, b < j, b < k, c < f, e < g, f < h, f < k, h < i, h < j, k < l
fn select_2363([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_2361([a, h, b, i, d, j, e, k, g, l])
    } else {
        select_2362([a, e, c, g, d, f, k, h, i, j, l])
    }
}
/// n = 10, i = 5, a < g, b < g, b < i, c < f, d < e, d < i, e < h, f < h, f < i, i < j
fn select_2364([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < e {
        select_111([a, c, d, g, e, f, i, j, h])
    } else {
        select_567([a, b, c, f, h, g, i])
    }
}
/// n = 9, i = 4, a < f, b < g, b < h, c < f, c < h, d < e, d < h, e < g, h < i
fn select_2365([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < h {
        select_726([a, b, c, d, e, f, g, h])
    } else {
        select_514([a, b, d, f, h, e, i])
    }
}
/// n = 10, i = 5, a < h, b < h, b < i, c < e, c < i, d < f, f < g, f < i, i < j
fn select_2366([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_2364([a, b, d, c, e, f, h, g, i, j])
    } else {
        select_2365([a, c, b, f, g, h, e, i, j])
    }
}
/// n = 9, i = 3, a < e, a < i, b < f, b < i, c < d, c < f, c < i, f < g, f < h
fn select_2367([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_159([b, a, c, d, e, i])
    } else {
        select_48([d, a, b, f, g, h, i])
    }
}
/// n = 10, i = 4, b < e, b < j, c < g, c < j, d < f, f < g, f < j, g < h, g < i
fn select_2368([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_12([c, b, a, d, e, f, g])
    } else {
        select_2367([b, c, f, a, e, g, h, i, j])
    }
}
/// n = 12, i = 6, a < g, a < l, b < g, b < i, c < e, c < i, d < f, f < h, f < l, h < j, h < k, i < k, i < l
fn select_2369([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < i {
        select_2366([a, b, c, f, e, h, j, g, i, k])
    } else {
        select_2368([e, a, i, d, g, f, h, j, k, l])
    }
}
/// n = 13, i = 6, a < e, b < h, b < l, c < f, c < h, d < g, f < k, f < l, g < i, g < l, i < j, i < k, l < m
fn select_2370([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_2363([b, f, d, h, a, g, e, i, j, k, l, m])
    } else {
        select_2369([b, c, a, d, e, g, h, i, f, j, k, l])
    }
}
/// n = 13, i = 6, a < h, b < f, c < k, c < l, d < h, d < l, e < g, g < i, g < l, i < j, i < k, l < m
fn select_2371([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < a {
        select_2360([b, d, c, e, f, a, g, i, j, h, k, l, m])
    } else {
        select_2370([b, d, a, e, f, c, g, h, i, j, k, l, m])
    }
}
/// n = 13, i = 6, b < i, c < g, d < k, d < l, e < i, e < l, f < h, h < j, h < l, j < k, l < m
fn select_2372([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < j {
        select_2348([b, c, a, d, e, f, g, h, i, j, l, m, k])
    } else {
        select_2371([b, c, d, e, f, g, h, i, j, a, k, l, m])
    }
}
/// n = 14, i = 7, b < h, c < l, c < m, d < g, d < m, e < j, f < i, f < j, i < l, j < k, j < m, m < n
fn select_2373([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < k {
        select_2305([a, b, c, e, d, f, h, i, g, j, l, k, m, n])
    } else {
        select_2372([a, i, b, d, c, e, h, j, l, k, g, m, n])
    }
}
/// n = 10, i = 4, a < f, a < h, b < f, b < h, c < d, c < h, c < j, d < e, f < g, f < j, h < i
fn select_2374([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_29([f, c, g, h, i, d, j])
    } else {
        select_126([d, a, b, e, h, i, f])
    }
}
/// n = 10, i = 4, a < e, b < h, b < j, c < f, c < h, d < f, d < h, f < g, f < j, h < i
fn select_2375([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1560([c, d, a, e, b, f, g, h])
    } else {
        select_2374([c, d, b, a, e, f, g, h, i, j])
    }
}
/// n = 9, i = 3, a < f, b < f, b < h, c < d, d < e, d < h, e < g, f < i
fn select_2376([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_19([a, e, b, g])
    } else {
        select_1899([a, c, b, d, f, e, h, i])
    }
}
/// n = 9, i = 3, a < f, b < f, b < h, c < e, c < h, d < e, d < h, e < g, f < i
fn select_2377([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_2376([a, b, c, d, e, f, g, h, i])
    } else {
        select_2376([a, b, d, c, e, f, g, h, i])
    }
}
/// n = 9, i = 4, a < g, b < g, c < e, d < f, d < g, e < f, f < h, g < i
fn select_2378([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_642([b, c, d, e, g, i, f])
    } else {
        select_88([a, b, d, f, h])
    }
}
/// n = 10, i = 4, a < e, b < g, b < i, c < g, c < i, d < f, d < g, e < j, f < i, f < j, g < h
fn select_2379([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_2377([a, f, b, c, g, e, h, i, j])
    } else {
        select_2378([b, c, a, d, e, f, g, j, h])
    }
}
/// n = 11, i = 4, a < f, b < g, b < i, c < g, c < i, d < e, d < i, e < k, f < k, g < h, i < j
fn select_2380([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_2375([a, d, b, c, f, g, h, i, j, e])
    } else {
        select_2379([a, b, c, d, f, e, i, j, g, k])
    }
}
/// n = 9, i = 5, a < f, b < h, b < i, c < g, d < g, e < f, e < h, e < i, g < h, g < i
fn select_2381([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_154([a, b, e, h, i, f])
    } else {
        select_637([b, c, d, e, f, g])
    }
}
/// n = 10, i = 5, a < f, b < g, c < h, c < j, d < g, d < h, e < f, e < h, e < j, g < j, h < i
fn select_2382([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_2381([a, c, b, d, e, f, g, h, j])
    } else {
        select_10([b, a, c, d, f, h, i, g])
    }
}
/// n = 10, i = 6, a < h, b < h, b < j, c < h, c < j, d < g, d < i, e < f, f < g, g < j, h < i
fn select_2383([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_9([a, b, c, g, j])
    } else {
        select_162([d, h, e, f, i])
    }
}
/// n = 10, i = 6, a < j, b < g, c < h, c < i, d < g, d < j, e < f, f < h, g < i, h < j
fn select_2384([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_2383([b, a, d, c, e, f, h, g, i, j])
    } else {
        select_1926([c, e, g, a, f, i, h])
    }
}
/// n = 12, i = 6, a < g, b < j, b < l, c < h, c < i, d < g, d < j, d < l, e < f, f < h, f < j, g < i, h < l, j < k
fn select_2385([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < i {
        select_2382([a, c, b, f, d, g, h, j, k, l])
    } else {
        select_2384([b, a, c, d, e, f, g, h, i, l])
    }
}
/// n = 9, i = 5, a < h, a < i, b < g, b < h, c < f, d < e, d < h, e < i, f < g, f < i
fn select_2386([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < h {
        select_158([a, b, f, d, e, g, i])
    } else {
        select_158([e, b, a, c, f, h, i])
    }
}
/// n = 9, i = 5, a < i, b < g, b < h, c < f, d < e, d < g, e < i, f < h, f < i
fn select_2387([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_2386([a, b, c, d, e, f, h, g, i])
    } else {
        select_966([e, g, c, a, f, h])
    }
}
/// n = 12, i = 6, a < f, b < j, b < l, c < j, c < l, d < h, d < i, e < g, f < h, g < i, g < j, i < l, j < k
fn select_2388([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < f {
        select_2385([a, c, d, b, e, g, f, i, h, j, k, l])
    } else {
        select_2387([c, d, e, f, b, g, h, i, j])
    }
}
/// n = 13, i = 6, a < i, b < f, c < j, c < l, d < j, d < l, e < g, f < i, g < h, g < j, h < l, j < k, l < m
fn select_2389([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_2380([b, c, d, h, a, f, j, k, l, m, i])
    } else {
        select_2388([b, c, d, a, e, f, g, i, h, j, k, l])
    }
}
/// n = 13, i = 6, a < g, b < f, c < j, c < l, d < j, d < l, e < h, h < i, h < j, i < l, j < k, l < m
fn select_2390([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_2389([a, b, c, d, e, f, h, i, g, j, k, l, m])
    } else {
        select_2389([b, a, c, d, e, g, h, i, f, j, k, l, m])
    }
}
/// n = 10, i = 4, a < e, b < g, b < j, c < g, c < j, d < f, d < g, f < j, g < h, g < i
fn select_2391([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_1761([a, b, d, e, f, c, g, h, i, j])
    } else {
        select_1761([a, c, d, e, f, b, g, h, i, j])
    }
}
/// n = 11, i = 4, a < e, b < h, b < k, c < f, c < h, d < f, d < h, f < g, f < k, h < i, h < j
fn select_2392([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_766([a, f, b, g, h, i, j, k])
    } else {
        select_10([b, a, c, d, e, f, g, k])
    }
}
/// n = 11, i = 4, a < f, b < e, b < i, c < g, c < i, d < g, d < i, g < h, i < j, i < k
fn select_2393([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_2391([a, c, d, b, f, e, i, j, k, g])
    } else {
        select_2392([a, b, c, d, f, g, h, i, j, k, e])
    }
}
/// n = 9, i = 2, a < d, a < f, a < i, b < c, b < e, b < i, e < g, f < h
fn select_2394([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_45([a, b, c, d, e, g, i])
    } else {
        select_45([b, a, d, c, f, h, i])
    }
}
/// n = 11, i = 3, a < d, a < k, b < g, b < h, b < j, b < k, c < e, c < g, c < h, c < k, e < f, e < j, h < i
fn select_2395([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < a {
        select_2394([b, e, f, g, a, h, d, i, j])
    } else {
        select_2062([b, a, c, d, g, e, h, f, i, k])
    }
}
/// n = 9, i = 3, a < d, a < i, b < e, b < f, b < i, c < e, c < f, c < i, f < g, f < h
fn select_2396([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_172([e, a, f, g, h, i])
    } else {
        select_75([b, c, a, d, e, i])
    }
}
/// n = 10, i = 4, a < d, a < j, b < f, c < e, c < g, c < j, e < h, e < i, f < g, f < h, f < i, f < j
fn select_2397([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_36([a, c, g, d, e, h, i, j])
    } else {
        select_163([d, b, c, e, f, g, j])
    }
}
/// n = 11, i = 4, a < d, a < k, b < f, c < e, c < g, c < h, c < k, e < j, f < g, f < h, f < j, f < k, h < i
fn select_2398([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_2396([a, c, f, d, g, h, e, i, k])
    } else {
        select_2397([a, b, c, d, e, f, g, h, j, k])
    }
}
/// n = 12, i = 4, a < e, a < l, b < d, b < i, b < j, b < l, c < f, f < g, f < i, f < j, f < l, g < h, j < k
fn select_2399([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < d {
        select_2395([a, b, f, e, g, h, i, j, k, d, l])
    } else {
        select_2398([a, c, b, e, d, f, i, j, k, g, l])
    }
}
/// n = 14, i = 6, a < e, a < n, b < h, b < k, c < h, c < k, d < f, f < g, f < k, f < m, f < n, g < i, h < j, h < m, h < n, k < l
fn select_2400([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < h {
        select_2393([a, g, b, c, i, e, k, l, h, j, m])
    } else {
        select_2399([a, h, d, j, e, f, k, l, m, g, i, n])
    }
}
/// n = 11, i = 5, a < e, b < g, b < i, c < g, c < h, c < i, d < f, f < h, f < i, i < j, i < k
fn select_2401([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_1014([a, b, c, d, e, f, g, i, j, k, h])
    } else {
        select_1021([a, b, c, f, e, h, i, j, k, g])
    }
}
/// n = 14, i = 7, a < f, b < i, b < j, c < h, c < l, d < h, d < l, e < g, g < i, g < k, h < j, h < k, h < n, i < l, i < n, l < m
fn select_2402([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_2401([a, b, h, e, f, g, j, k, i, l, n])
    } else {
        select_1628([a, c, d, i, f, l, h, j, k, n, m])
    }
}
/// n = 11, i = 5, a < f, b < h, c < h, c < i, d < h, d < i, e < g, e < k, g < i, h < k, i < j
fn select_2403([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < g {
        select_794([a, c, d, e, b, f, g, h, k, i])
    } else {
        select_2165([a, c, d, g, f, b, i, j, h])
    }
}
/// n = 14, i = 7, a < f, b < j, b < k, c < i, c < l, d < i, d < l, e < g, g < h, g < j, i < k, i < n, j < l, j < n, l < m
fn select_2404([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < h {
        select_2402([a, b, c, d, e, f, g, i, j, k, h, l, m, n])
    } else {
        select_2403([a, h, c, d, b, f, j, i, l, m, k])
    }
}
/// n = 15, i = 7, a < f, a < o, b < j, b < k, c < i, c < l, d < i, d < l, e < g, e < j, g < h, i < k, i < n, i < o, j < l, j < n, j < o, l < m
fn select_2405([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < g {
        select_2400([a, c, d, e, f, j, g, i, h, k, l, m, n, o])
    } else {
        select_2404([a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 15, i = 7, a < f, a < o, b < i, b < m, c < i, c < m, d < j, d < l, e < g, e < j, g < h, i < l, i < o, j < k, j < m, j < o, m < n
fn select_2406([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < i {
        select_2390([a, g, b, c, d, h, f, j, k, m, n, i, l])
    } else {
        select_2405([a, d, b, c, e, f, g, h, i, j, l, m, n, k, o])
    }
}
/// n = 14, i = 7, a < g, b < i, c < i, c < j, d < i, d < j, e < h, e < k, f < h, f < k, h < j, h < m, h < n, i < k, i < m, i < n, k < l
fn select_2407([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_1497([a, b, c, d, h, g, j, i, k, m, n])
    } else {
        select_1628([a, e, f, i, g, k, h, j, m, n, l])
    }
}
/// n = 9, i = 5, a < f, b < h, b < i, c < h, c < i, d < g, e < g, e < h, g < i
fn select_2408([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_797([a, b, c, d, g, h, f, i])
    } else {
        select_2341([d, b, c, e, a, f, h, g])
    }
}
/// n = 14, i = 7, a < g, b < i, b < k, c < i, c < k, d < i, d < j, e < h, e < l, f < h, f < l, h < j, h < k, h < n, i < l, i < n, l < m
fn select_2409([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < j {
        select_2407([a, d, b, c, e, f, g, h, i, k, l, m, j, n])
    } else {
        select_2408([a, b, c, d, h, g, j, k, i])
    }
}
/// n = 14, i = 7, a < g, b < j, b < k, c < i, c < l, d < i, d < l, e < h, e < j, f < h, f < j, i < k, i < n, j < l, j < n, l < m
fn select_2410([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < h {
        select_2409([a, e, f, b, c, d, g, i, j, k, h, l, m, n])
    } else {
        select_2403([a, h, c, d, b, g, j, i, l, m, k])
    }
}
/// n = 15, i = 7, a < h, b < g, b < o, c < j, c < k, d < i, d < l, e < i, e < l, f < h, f < j, i < k, i < n, i < o, j < l, j < n, j < o, l < m
fn select_2411([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < a {
        select_2400([b, d, e, f, g, j, a, i, h, k, l, m, n, o])
    } else {
        select_2410([b, c, d, e, a, f, g, h, i, j, k, l, m, n])
    }
}
/// n = 15, i = 7, a < h, b < g, b < o, c < i, c < m, d < i, d < m, e < j, e < l, f < h, f < j, i < l, i < o, j < k, j < m, j < o, m < n
fn select_2412([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < i {
        select_2390([a, b, c, d, e, g, h, j, k, m, n, i, l])
    } else {
        select_2411([a, b, e, c, d, f, g, h, i, j, l, m, n, k, o])
    }
}
/// n = 15, i = 7, b < h, b < o, c < i, c < m, d < i, d < m, e < j, e < l, f < g, f < j, i < l, i < o, j < k, j < m, j < o, m < n
fn select_2413([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < a {
        select_2406([b, c, d, e, f, h, g, a, i, j, k, l, m, n, o])
    } else {
        select_2412([a, b, c, d, e, f, h, g, i, j, k, l, m, n, o])
    }
}
/// n = 7, i = 2, c < f, c < g, d < e, d < f, d < g
fn select_2414([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_24([a, c, d, e, b, f, g])
    } else {
        select_24([b, c, d, e, a, f, g])
    }
}
/// n = 9, i = 2, c < i, d < e, d < f, d < i, f < g, f < h
fn select_2415([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < f {
        select_2414([a, b, c, d, e, f, i])
    } else {
        select_221([a, b, d, e, f, c, g, h])
    }
}
/// n = 8, i = 4, b < g, b < h, c < f, c < h, d < e, d < g, e < h, f < g
fn select_2416([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_108([b, e, c, a, f, g, h])
    } else {
        select_158([b, a, d, c, f, e, g])
    }
}
/// n = 10, i = 4, b < g, b < j, c < f, c < g, d < e, d < g, d < j, f < j, g < h, g < i
fn select_2417([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_2416([a, b, c, d, e, f, j, g])
    } else {
        select_1388([a, f, b, d, g, e, h, i, j])
    }
}
/// n = 10, i = 4, b < g, b < h, c < f, c < h, d < e, d < g, d < h, h < i, h < j
fn select_2418([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_1545([a, e, c, b, g, h, i, j, f])
    } else {
        select_2417([a, b, c, d, e, f, h, i, j, g])
    }
}
/// n = 11, i = 4, a < k, b < g, b < j, c < f, c < h, d < e, d < g, d < h, h < i, h < j, h < k
fn select_2419([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < b {
        select_2415([e, f, a, h, i, b, g, j, k])
    } else {
        select_2418([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 9, i = 2, a < e, b < d, c < f, c < g, f < i, g < h
fn select_2420([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_1553([a, c, e, f, g, b, h, i, d])
    } else {
        select_1553([b, c, d, f, g, a, h, i, e])
    }
}
/// n = 10, i = 3, a < g, b < e, b < h, c < d, c < g, c < h, c < j, d < f, e < j, h < i
fn select_2421([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_1553([d, b, f, e, h, a, i, j, g])
    } else {
        select_2224([a, b, c, e, g, h, d, i, j, f])
    }
}
/// n = 10, i = 3, b < f, b < h, c < d, c < e, c < h, c < j, e < g, f < j, h < i
fn select_2422([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < a {
        select_2420([d, e, b, g, a, f, h, i, j])
    } else {
        select_2421([a, b, c, e, f, g, d, h, i, j])
    }
}
/// n = 11, i = 4, b < f, b < h, c < f, c < g, c < h, d < e, d < h, d < k, e < g, f < k, g < i, h < j
fn select_2423([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < b {
        select_659([a, b, c, g, i, f, h])
    } else {
        select_218([a, c, d, e, f, h, j, k, g])
    }
}
/// n = 12, i = 4, b < f, b < i, b < k, c < f, c < g, c < i, d < e, d < g, d < i, d < l, f < l, g < h, g < k, i < j
fn select_2424([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_733([a, b, d, f, g, e, h, i, j, k, l])
    } else {
        select_2423([a, b, c, d, e, f, g, i, h, j, l])
    }
}
/// n = 13, i = 4, b < j, b < m, c < f, c < g, c < j, d < e, d < g, d < i, d < j, f < i, g < h, g < m, i < k, j < l
fn select_2425([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < b {
        select_2422([a, f, d, e, g, b, h, i, k, j])
    } else {
        select_2424([a, b, c, d, e, f, g, h, j, l, m, i])
    }
}
/// n = 14, i = 5, a < j, a < l, b < d, b < g, c < e, c < f, c < l, e < k, f < h, f < j, g < i, g < k, g < l, k < m, l < n
fn select_2426([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < g {
        select_2419([e, a, b, f, h, d, j, g, i, l, k])
    } else {
        select_2425([d, a, c, g, i, e, f, h, k, l, m, n, j])
    }
}
/// n = 8, i = 2, a < h, b < g, c < d, c < e, c < g, c < h, e < f
fn select_2427([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_45([a, c, d, b, e, f, h])
    } else {
        select_45([b, c, d, a, e, f, g])
    }
}
/// n = 10, i = 3, a < d, a < f, a < h, b < c, b < e, b < h, b < j, e < g, f < j, h < i
fn select_2428([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_2427([d, f, b, c, h, i, j, e])
    } else {
        select_231([c, e, a, g, f, h, i, j, d])
    }
}
/// n = 8, i = 4, a < h, b < e, c < f, c < g, d < f, d < g, e < g, f < h
fn select_2429([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_157([b, c, d, a, e, f, h])
    } else {
        select_75([c, d, e, a, f, g])
    }
}
/// n = 10, i = 5, a < f, a < h, b < g, b < h, c < g, c < h, d < e, d < f, d < j, e < i, g < i, h < j
fn select_2430([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_2429([e, a, b, c, f, g, h, i])
    } else {
        select_1662([g, a, d, e, h, f, j, i])
    }
}
/// n = 11, i = 5, a < f, a < h, b < e, b < f, b < k, c < g, c < h, d < g, d < h, e < j, g < j, h < i, h < k
fn select_2431([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < i {
        select_2430([a, c, d, b, e, f, g, h, j, k])
    } else {
        select_278([g, b, h, i, f, k])
    }
}
/// n = 13, i = 5, a < g, b < f, b < h, c < e, c < f, c < j, d < g, d < h, e < l, e < m, g < m, h < i, h < j, h < l, j < k
fn select_2432([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < a {
        select_2428([c, h, i, f, a, e, g, j, k, l])
    } else {
        select_2431([b, c, a, d, e, f, g, h, i, m, j])
    }
}
/// n = 8, i = 3, a < h, b < e, c < f, c < h, d < e, d < f, d < h, f < g
fn select_2433([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_406([c, b, d, a, e, f, g])
    } else {
        select_80([c, a, d, b, f, g, e, h])
    }
}
/// n = 9, i = 3, a < d, a < i, b < c, b < g, b < h, c < f, c < i, d < e, d < g, d < h
fn select_2434([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_19([f, a, d, i])
    } else {
        select_34([e, b, c, g, h])
    }
}
/// n = 10, i = 3, a < d, a < g, b < c, b < g, b < i, b < j, c < f, d < e, d < i, d < j, g < h
fn select_2435([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < c {
        select_243([d, b, e, g, c, h, i, j])
    } else {
        select_2434([a, b, c, d, e, f, i, j, g])
    }
}
/// n = 11, i = 4, a < f, a < i, b < f, b < g, b < i, c < d, c < e, c < i, c < k, e < g, f < h, f < k, i < j
fn select_2436([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < f {
        select_2433([d, e, a, b, g, i, j, f])
    } else {
        select_2435([a, c, e, f, h, g, i, j, d, k])
    }
}
/// n = 8, i = 3, a < e, a < h, b < e, b < f, b < g, c < d, d < f, d < g, d < h
fn select_2437([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < a {
        select_28([a, b, f, g, h])
    } else {
        select_25([b, c, e, d, h])
    }
}
/// n = 9, i = 4, a < g, a < h, a < i, b < e, c < d, c < h, c < i, d < g, e < f, e < h, e < i
fn select_2438([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_154([f, a, c, h, i, d])
    } else {
        select_2437([d, a, b, e, g, h, i, f])
    }
}
/// n = 8, i = 4, a < e, a < h, b < e, b < h, c < f, c < h, d < f, d < h, f < g
fn select_2439([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_449([c, d, a, b, e, f, g, h])
    } else {
        select_449([c, d, b, a, e, f, g, h])
    }
}
/// n = 12, i = 6, a < h, a < j, b < f, b < k, b < l, c < g, c < h, d < g, d < h, e < f, e < g, e < l, f < j, g < k, h < i, h < l
fn select_2440([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < f {
        select_2438([b, a, e, g, h, i, k, f, l])
    } else {
        select_2439([a, f, c, d, j, g, k, h])
    }
}
/// n = 14, i = 6, a < i, b < g, b < j, c < f, c < k, c < l, d < g, d < i, e < f, e < i, e < l, f < j, f < n, g < h, g < l, g < n, i < k, l < m
fn select_2441([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < a {
        select_2436([c, e, g, h, a, f, i, j, l, m, n])
    } else {
        select_2440([b, c, a, d, e, f, i, g, h, j, k, l])
    }
}
/// n = 15, i = 6, a < j, b < k, b < m, c < g, c < i, d < g, d < j, e < f, e < j, e < m, f < i, f < l, g < h, g < l, g < m, j < k, l < n, m < o
fn select_2442([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < b {
        select_2432([a, c, f, d, b, i, j, g, h, l, n, m, k])
    } else {
        select_2441([a, c, b, d, e, f, g, h, j, i, k, m, o, l])
    }
}
/// n = 9, i = 3, a < c, b < d, b < i, c < f, c < i, d < e, d < g, g < h
fn select_2443([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_194([a, e, g, h])
    } else {
        select_474([a, b, d, c, f, e, i])
    }
}
/// n = 11, i = 4, a < d, a < e, b < e, b < f, c < d, c < f, d < h, e < i, f < g, f < j, j < k
fn select_2444([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_2443([a, b, d, f, g, h, j, k, e])
    } else {
        select_2443([a, c, e, f, g, i, j, k, d])
    }
}
/// n = 8, i = 3, b < d, b < h, c < e, e < f, e < h, f < g
fn select_2445([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_58([a, b, e, f, g, h])
    } else {
        select_53([a, c, b, d, e, h])
    }
}
/// n = 13, i = 6, a < f, a < i, b < g, b < i, c < f, c < g, d < e, e < i, e < k, e < l, f < k, g < h, g < l, i < j, l < m
fn select_2446([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < g {
        select_2444([a, b, c, f, i, g, h, k, j, l, m])
    } else {
        select_2445([h, a, d, f, e, l, m, i])
    }
}
/// n = 11, i = 5, a < g, a < k, b < g, b < h, c < e, c < h, d < f, e < k, f < h, f < j, f < k, g < j, h < i
fn select_2447([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_1572([a, e, b, f, h, i, k])
    } else {
        select_487([b, c, a, d, e, f, g, j, k])
    }
}
/// n = 9, i = 4, a < e, b < f, b < g, b < i, c < d, c < i, d < g, e < f, e < i, g < h
fn select_2448([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_25([a, b, e, g, h])
    } else {
        select_4([b, c, d, f, i])
    }
}
/// n = 12, i = 5, a < f, a < h, b < f, b < g, c < g, c < h, c < l, d < e, d < l, e < h, e < i, e < j, f < l, g < i, g < j, h < k
fn select_2449([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_2448([b, c, d, e, f, g, h, k, l])
    } else {
        select_1195([a, b, d, g, e, f, h, i, j, k])
    }
}
/// n = 13, i = 6, a < h, a < j, b < h, b < i, c < f, d < e, d < m, e < g, e < j, e < l, f < i, f < j, f < m, h < m, i < l, j < k
fn select_2450([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < i {
        select_2447([b, a, e, c, g, f, h, j, k, m, i])
    } else {
        select_2449([a, b, f, d, e, h, i, j, g, l, k, m])
    }
}
/// n = 15, i = 7, a < f, a < m, b < h, b < k, c < g, d < e, d < h, e < j, f < k, g < j, g < k, g < m, h < i, h < m, h < n, j < n, k < l, m < o
fn select_2451([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if f < e {
        select_2446([f, b, d, c, g, e, h, i, k, l, j, m, o])
    } else {
        select_2450([a, e, c, b, h, g, i, f, j, m, o, n, k])
    }
}
/// n = 9, i = 4, a < g, b < e, c < d, c < h, d < f, e < g, e < h, h < i
fn select_2452([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_110([a, c, g, d, h, i, f])
    } else {
        select_88([a, f, b, e, h])
    }
}
/// n = 11, i = 5, a < g, b < e, b < h, c < g, c < h, c < j, d < f, e < j, f < h, f < j, f < k, g < k, h < i
fn select_2453([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_487([a, b, c, d, e, f, g, k, j])
    } else {
        select_491([a, e, c, f, h, i, j])
    }
}
/// n = 13, i = 6, a < f, a < i, b < h, b < i, b < m, c < g, c < i, d < e, d < g, e < h, f < j, f < m, g < k, g < l, g < m, h < k, i < j, i < l
fn select_2454([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_2452([f, c, d, e, i, h, j, g, l])
    } else {
        select_2453([e, a, b, c, f, g, h, i, l, m, k])
    }
}
/// n = 11, i = 6, a < h, b < e, c < f, d < g, e < i, e < j, f < i, f < j, f < k, g < h, g < j, h < k
fn select_2455([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_487([a, b, g, c, e, f, h, k, j])
    } else {
        select_775([a, e, f, d, g, i, j])
    }
}
/// n = 10, i = 5, a < h, b < g, b < j, c < f, d < e, d < i, e < g, e < j, f < h, f < j, h < i
fn select_2456([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_1868([b, a, f, d, e, h, i, j])
    } else {
        select_775([a, b, e, c, f, g, j])
    }
}
/// n = 12, i = 6, a < i, b < e, b < j, c < f, d < g, e < k, f < i, f < j, f < k, g < h, g < k, g < l, h < j, i < l
fn select_2457([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < h {
        select_2455([a, b, d, c, e, g, f, i, h, k, l])
    } else {
        select_2456([a, b, c, g, h, f, e, i, l, j])
    }
}
/// n = 14, i = 7, a < f, a < k, b < h, b < k, c < g, d < e, d < h, e < j, f < l, f < n, g < j, g < k, g < n, h < i, h < m, h < n, j < m, k < l
fn select_2458([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if k < i {
        select_2454([a, g, b, d, e, f, h, j, k, l, m, i, n])
    } else {
        select_2457([e, a, c, b, f, g, h, i, j, k, n, m])
    }
}
/// n = 15, i = 7, a < k, b < l, b < m, c < h, c < k, d < g, e < f, e < h, f < j, g < j, g < k, g < m, h < i, h < m, h < n, j < n, k < l, m < o
fn select_2459([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if b < a {
        select_2451([b, c, d, e, f, a, g, h, i, j, k, l, m, n, o])
    } else {
        select_2458([a, c, d, e, f, b, g, h, i, j, k, l, n, m])
    }
}
/// n = 16, i = 7, a < k, b < l, b < n, c < f, c < i, d < i, d < k, e < g, g < h, g < k, g < n, h < m, i < j, i < m, i < n, k < l, m < o, n < p
fn select_2460([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < f {
        select_2442([a, b, c, d, g, h, i, j, f, k, l, m, n, o, p])
    } else {
        select_2459([a, b, d, e, c, f, g, i, j, h, k, l, n, m, p])
    }
}
/// n = 16, i = 7, b < l, b < n, c < f, c < i, d < i, d < k, e < g, g < h, g < k, g < n, h < m, i < j, i < m, i < n, k < l, m < o, n < p
fn select_2461([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < a {
        select_2426([b, c, g, f, h, k, i, a, j, l, m, n, o, p])
    } else {
        select_2460([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }
}
/// n = 9, i = 4, b < e, c < d, d < f, d < h, d < i, e < g, e < h, e < i
fn select_2462([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_163([a, b, d, f, e, h, i])
    } else {
        select_163([a, c, e, g, d, h, i])
    }
}
/// n = 9, i = 6, b < h, b < i, c < h, c < i, d < g, e < f, f < h, g < i
fn select_2463([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_775([a, b, c, e, f, i, h])
    } else {
        select_775([a, b, c, d, g, h, i])
    }
}
/// n = 11, i = 6, b < h, b < i, c < h, c < i, d < g, e < f, e < h, g < i, g < j, g < k, h < j, h < k
fn select_2464([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_2462([a, d, e, h, g, f, i, j, k])
    } else {
        select_2463([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 13, i = 7, b < i, b < j, c < h, d < g, d < k, e < f, e < i, g < j, g < m, h < j, h < k, h < m, i < k, i < m, k < l
fn select_2465([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_1713([a, f, d, i, c, h, g, k, l, j, m])
    } else {
        select_2464([a, b, g, c, e, f, h, i, j, k, m])
    }
}
/// n = 13, i = 7, b < h, c < i, c < k, d < g, d < l, e < f, e < i, g < j, g < k, h < k, h < l, i < j, i < l, l < m
fn select_2466([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < j {
        select_2465([a, c, b, d, e, f, g, h, i, k, l, m, j])
    } else {
        select_526([a, f, b, g, i, j, h])
    }
}
/// n = 10, i = 4, a < e, b < f, b < j, c < g, d < f, d < g, g < h, g < j, h < i
fn select_2467([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_836([b, g, f, a, h, i, e, j])
    } else {
        select_1366([b, c, d, a, e, f, g, h])
    }
}
/// n = 11, i = 4, a < e, b < h, b < k, c < f, c < h, d < f, d < h, f < g, f < k, g < i, h < j
fn select_2468([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_115([a, f, e, g, b, h, k, i])
    } else {
        select_382([a, b, c, d, e, h, f, g, k, j])
    }
}
/// n = 12, i = 5, a < e, b < h, b < j, c < g, d < f, d < j, f < h, f < i, g < i, g < j, i < k, j < l
fn select_2469([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_2467([a, b, c, f, e, h, g, i, k, j])
    } else {
        select_2468([a, b, d, g, e, f, i, j, k, l, h])
    }
}
/// n = 12, i = 5, c < h, c < j, d < g, e < f, e < j, f < h, f < i, g < i, g < j, i < k, j < l
fn select_2470([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_2469([a, c, d, e, b, f, g, h, i, j, k, l])
    } else {
        select_2469([b, c, d, e, a, f, g, h, i, j, k, l])
    }
}
/// n = 15, i = 7, b < g, b < m, c < h, d < i, d < k, e < f, e < i, g < k, h < k, h < l, h < m, i < j, i < m, j < l, l < n, m < o
fn select_2471([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < j {
        select_2466([a, c, d, b, e, f, g, h, i, j, k, m, o])
    } else {
        select_2470([a, f, b, c, i, j, h, g, l, m, n, o])
    }
}
/// n = 9, i = 5, b < h, b < i, c < f, c < i, d < g, d < h, e < g, e < h, g < i
fn select_2472([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < h {
        select_1095([a, b, d, e, c, f, g, i, h])
    } else {
        select_977([a, g, c, b, h, f, i])
    }
}
/// n = 13, i = 7, b < i, b < k, c < f, c < i, d < h, e < g, e < l, g < j, g < k, h < j, h < k, h < l, i < l, l < m
fn select_2473([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < j {
        select_2465([a, b, d, e, c, f, g, h, i, k, l, m, j])
    } else {
        select_2472([a, b, c, g, h, f, j, k, i])
    }
}
/// n = 11, i = 3, a < c, a < h, a < k, b < d, b < h, c < f, d < e, d < g, d < k, g < i, h < j
fn select_2474([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_42([c, b, f, h, j, d])
    } else {
        select_2222([a, d, e, g, h, i, j, c, k])
    }
}
/// n = 10, i = 4, a < d, a < f, b < f, b < g, b < h, c < e, c < j, e < g, e < h, f < j, h < i
fn select_2475([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_45([a, b, g, d, h, i, f])
    } else {
        select_709([d, c, b, f, e, g, h, j])
    }
}
/// n = 12, i = 4, a < d, b < f, b < i, c < e, c < g, c < h, e < i, e < l, f < g, f < h, f < l, h < j, i < k
fn select_2476([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_2474([e, b, a, f, g, d, h, i, j, k, l])
    } else {
        select_2475([a, c, b, d, f, e, g, h, j, i])
    }
}
/// n = 12, i = 4, c < f, c < i, d < e, d < g, d < h, e < i, e < l, f < g, f < h, f < l, h < j, i < k
fn select_2477([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_2476([a, c, d, b, e, f, g, h, i, j, k, l])
    } else {
        select_2476([b, c, d, a, e, f, g, h, i, j, k, l])
    }
}
/// n = 12, i = 5, a < e, b < g, c < g, d < f, d < h, f < j, f < k, f < l, g < h, g < j, g < k, g < l, h < i
fn select_2478([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < g {
        select_372([e, b, c, d, f, g, h])
    } else {
        select_426([a, d, e, h, f, j, k, l, i])
    }
}
/// n = 12, i = 5, c < g, d < g, e < f, e < h, f < j, f < k, f < l, g < h, g < j, g < k, g < l, h < i
fn select_2479([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_2478([a, c, d, e, b, f, g, h, i, j, k, l])
    } else {
        select_2478([b, c, d, e, a, f, g, h, i, j, k, l])
    }
}
/// n = 14, i = 6, b < i, b < j, c < f, c < i, d < h, d < i, e < g, e < k, g < h, g < m, g < n, h < j, i < k, i < m, i < n, k < l
fn select_2480([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_1139([a, b, c, d, g, f, h, j, i])
    } else {
        select_2479([a, f, b, d, e, g, i, k, l, h, m, n])
    }
}
/// n = 15, i = 6, b < i, b < m, c < h, c < k, d < f, d < h, e < g, e < m, g < i, g < j, g < l, h < j, h < l, h < m, i < k, l < n, m < o
fn select_2481([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < b {
        select_2477([a, f, e, h, b, g, j, l, m, n, o, i])
    } else {
        select_2480([a, c, d, b, e, f, g, i, h, k, m, o, j, l])
    }
}
/// n = 16, i = 7, b < g, b < n, c < j, c < k, d < f, d < j, e < h, g < k, h < i, h < k, h < n, i < l, i < m, j < l, j < m, j < n, m < o, n < p
fn select_2482([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < i {
        select_2473([a, c, d, e, b, f, g, h, j, i, k, n, p])
    } else {
        select_2481([a, b, c, d, h, f, i, j, g, l, k, m, n, o, p])
    }
}
/// n = 16, i = 7, b < g, b < n, c < j, c < l, d < h, e < f, e < j, g < l, h < i, h < l, h < n, i < m, j < k, j < m, j < n, m < o, n < p
fn select_2483([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < i {
        select_2471([a, b, d, c, e, f, g, h, j, k, l, i, n, m, p])
    } else {
        select_2482([a, b, c, e, d, f, g, h, i, j, l, k, m, n, o, p])
    }
}
/// n = 16, i = 7, b < g, b < n, c < j, c < l, d < h, e < f, e < j, h < i, h < l, h < n, i < m, j < k, j < m, j < n, m < o, n < p
fn select_2484([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if l < g {
        select_2461([a, b, e, c, d, f, h, i, j, k, l, g, m, n, o, p])
    } else {
        select_2483([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 7, b < m, c < h, c < o, d < i, d < m, e < j, e < l, f < g, f < j, i < l, i < o, j < k, j < m, j < o, m < n, o < p
fn select_2485([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if b < i {
        select_2413([a, c, b, d, e, f, g, h, i, j, k, l, m, n, o])
    } else {
        select_2484([a, c, e, d, f, g, h, i, b, j, k, l, m, o, n, p])
    }
}
/// n = 16, i = 7, b < j, c < h, c < o, d < k, d < l, e < g, e < k, f < i, f < j, i < l, i < o, j < n, k < m, k < o, o < p
fn select_2486([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < k {
        select_2373([a, c, d, e, b, f, g, h, i, j, n, l, k, m])
    } else {
        select_2485([a, b, c, f, d, e, g, h, i, k, m, l, j, n, o, p])
    }
}
/// n = 16, i = 7, b < h, c < l, d < g, d < o, e < j, e < m, f < i, f < l, i < m, i < o, j < k, j < o, l < n, o < p
fn select_2487([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < b {
        select_2252([a, c, d, f, e, g, i, j, k, b, l, h, m, n, o, p])
    } else {
        select_2486([a, c, d, e, b, f, h, g, i, l, j, m, k, n, o, p])
    }
}
/// n = 10, i = 4, a < e, a < j, b < g, b < i, c < f, c < g, d < f, d < g, f < i, f < j, g < h
fn select_2488([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_491([e, b, c, d, f, j, g])
    } else {
        select_134([a, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, b < j, c < g, c < i, d < f, d < g, e < f, e < g, f < i, f < j, g < h
fn select_2489([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1372([a, c, d, e, b, f, g, h, i])
    } else {
        select_2488([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 12, i = 6, b < l, c < i, c < k, d < h, e < h, f < g, f < i, g < k, g < l, h < i, h < k, h < l, i < j
fn select_2490([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_1931([a, b, c, d, e, g, h, i, k, l])
    } else {
        select_2489([a, b, c, f, h, g, i, j, k, l])
    }
}
/// n = 12, i = 6, b < i, c < j, c < l, d < h, e < h, f < g, f < j, g < i, g < l, h < j, h < l, j < k
fn select_2491([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < i {
        select_2490([a, b, c, d, e, f, g, h, j, k, l, i])
    } else {
        select_1493([a, c, d, e, b, g, i, h])
    }
}
/// n = 10, i = 5, a < j, b < j, c < i, d < g, e < h, f < g, f < h, h < i, h < j
fn select_2492([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_824([c, d, a, e, f, b, g, h, i, j])
    } else {
        select_824([c, d, b, e, f, a, g, h, i, j])
    }
}
/// n = 13, i = 6, a < k, b < i, c < h, d < h, e < j, e < l, f < g, f < l, g < i, g < j, h < l, j < k, l < m
fn select_2493([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < j {
        select_2491([a, b, e, c, d, f, g, h, i, l, m, j])
    } else {
        select_2492([c, d, a, b, e, g, i, j, k, h])
    }
}
/// n = 13, i = 6, a < k, b < j, c < i, d < i, e < g, e < l, f < h, f < l, g < k, h < j, i < l, l < m
fn select_2494([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_2493([b, a, c, d, f, e, g, i, k, h, j, l, m])
    } else {
        select_2493([a, b, c, d, e, f, h, i, j, g, k, l, m])
    }
}
/// n = 9, i = 6, a < i, b < i, c < g, d < h, e < h, f < h, h < i
fn select_2495([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_2215([c, d, e, f, a, g, b, h, i])
    } else {
        select_2215([c, d, e, f, b, g, a, h, i])
    }
}
/// n = 12, i = 6, a < k, a < l, b < h, c < h, c < j, d < i, d < l, e < g, f < h, f < i, g < i, h < l, i < j, i < k
fn select_2496([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_1504([b, a, c, d, f, g, i, j, k])
    } else {
        select_2495([a, d, e, b, c, f, g, h, l])
    }
}
/// n = 13, i = 6, a < m, b < j, b < k, c < h, d < g, e < i, e < j, f < h, f < i, g < i, h < j, i < k, i < m, j < l
fn select_2497([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < h {
        select_2496([b, c, a, e, d, f, g, h, i, m, k, j])
    } else {
        select_1794([b, d, e, c, g, h, a, i, j, l, k])
    }
}
/// n = 8, i = 3, a < h, b < g, c < f, d < e, d < f, e < g, e < h
fn select_2498([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_1037([c, a, d, b, e, f, h])
    } else {
        select_1037([c, b, d, a, e, f, g])
    }
}
/// n = 8, i = 4, a < h, b < f, c < e, d < f, d < g, e < g, f < h
fn select_2499([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_2315([b, c, d, a, e, f, g, h])
    } else {
        select_15([b, d, e, a, g, f])
    }
}
/// n = 9, i = 4, a < i, b < f, c < e, c < h, d < f, d < g, e < g, f < h, f < i
fn select_2500([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < h {
        select_2499([a, b, c, d, e, f, g, i])
    } else {
        select_25([e, b, g, f, h])
    }
}
/// n = 10, i = 4, a < i, a < j, b < g, c < e, c < f, d < f, d < g, d < h, e < h, f < i, g < j
fn select_2501([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_2498([a, b, e, d, f, h, g, i])
    } else {
        select_2500([a, b, c, d, e, g, h, f, j])
    }
}
/// n = 15, i = 7, a < k, b < i, b < o, c < h, d < j, d < m, e < f, e < g, f < l, f < m, g < i, h < j, i < m, j < k, j < l, k < o, m < n
fn select_2502([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < j {
        select_2497([a, f, b, c, d, g, h, i, j, m, l, n, k])
    } else {
        select_2501([b, a, e, j, f, g, k, l, i, o])
    }
}
/// n = 11, i = 6, b < i, c < g, d < i, d < k, e < h, f < h, g < k, h < i, h < k, i < j
fn select_2503([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_2263([a, b, c, d, h, g, i, j, k])
    } else {
        select_2270([a, b, d, e, f, g, h, k])
    }
}
/// n = 7, i = 3, a < g, b < g, c < f, d < e, d < f, d < g
fn select_2504([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < e {
        select_1572([a, b, c, d, f, e, g])
    } else {
        select_1008([a, b, c, d, e, f, g])
    }
}
/// n = 9, i = 5, b < i, c < i, d < h, e < g, f < g, g < h, g < i
fn select_2505([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_2504([b, c, d, g, a, h, i])
    } else {
        select_2270([b, c, d, a, e, f, g, h])
    }
}
/// n = 12, i = 6, b < j, b < k, c < h, d < h, e < g, f < i, f < k, g < i, h < k, i < j, k < l
fn select_2506([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < i {
        select_2503([a, b, e, f, c, d, g, h, k, l, i])
    } else {
        select_2505([a, c, d, b, f, g, i, j, h])
    }
}
/// n = 15, i = 7, a < l, b < j, b < o, c < i, c < m, d < h, e < f, e < g, f < k, f < m, g < j, h < i, h < k, i < l, j < m, l < o, m < n
fn select_2507([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < k {
        select_2502([a, b, d, c, e, f, g, h, j, i, l, k, m, n, o])
    } else {
        select_2506([a, c, b, g, d, f, h, j, k, i, m, n])
    }
}
/// n = 10, i = 5, a < j, b < i, c < h, d < g, e < g, f < g, g < h, g < i, g < j
fn select_2508([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < h {
        select_1504([c, a, b, d, e, f, g, i, j])
    } else {
        select_11([b, c, g, h, i])
    }
}
/// n = 10, i = 5, a < j, b < i, c < g, d < h, e < h, f < g, f < h, h < i, h < j
fn select_2509([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_2508([a, b, c, d, e, f, h, g, i, j])
    } else {
        select_1493([a, b, d, e, c, f, g, h])
    }
}
/// n = 9, i = 4, a < i, b < e, b < g, c < f, c < g, d < f, d < g, f < i, g < h
fn select_2510([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_135([e, c, d, a, f, g, h])
    } else {
        select_879([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, a < j, b < i, c < g, d < f, d < g, e < f, e < g, f < i, f < j, g < h
fn select_2511([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_1736([a, b, d, e, c, f, i, g, j])
    } else {
        select_2510([a, c, d, e, b, f, g, h, j])
    }
}
/// n = 12, i = 6, a < l, b < i, c < j, d < h, e < h, f < g, f < j, g < i, g < l, h < j, h < l, j < k
fn select_2512([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_2509([a, c, b, d, e, g, i, h, j, l])
    } else {
        select_2511([a, b, c, f, h, g, j, k, i, l])
    }
}
/// n = 9, i = 5, b < i, c < g, d < h, e < g, e < h, f < g, f < h, h < i
fn select_2513([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_1504([a, b, c, d, e, f, h, g, i])
    } else {
        select_2270([a, b, d, c, e, f, g, h])
    }
}
/// n = 7, i = 3, a < g, b < f, c < e, c < g, d < e, d < f, d < g
fn select_2514([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < c {
        select_255([b, d, a, c, f, e, g])
    } else {
        select_291([b, d, c, a, e, f, g])
    }
}
/// n = 8, i = 3, a < h, b < f, c < g, c < h, d < e, d < f, d < h, e < g
fn select_2515([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_23([c, b, e, a, f, g])
    } else {
        select_2514([c, b, a, d, e, f, h])
    }
}
/// n = 10, i = 5, a < j, b < i, c < h, d < h, d < j, e < g, f < g, f < h, g < i, g < j
fn select_2516([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < g {
        select_2513([a, b, d, e, c, f, h, g, i])
    } else {
        select_2515([a, b, d, g, c, i, h, j])
    }
}
/// n = 8, i = 4, a < g, b < f, b < h, c < e, c < f, d < e, d < g, g < h
fn select_2517([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_124([b, c, a, e, g, h])
    } else {
        select_843([a, b, d, e, f])
    }
}
/// n = 8, i = 4, a < h, b < g, c < e, c < f, d < f, d < g, g < h
fn select_2518([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_2517([b, a, c, d, f, e, g, h])
    } else {
        select_16([b, e, d, f, a, g])
    }
}
/// n = 10, i = 5, a < j, b < f, c < g, d < e, d < h, e < i, f < i, f < j, g < h, g < j
fn select_2519([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_2500([a, c, d, f, e, g, i, h, j])
    } else {
        select_2518([a, b, d, g, e, h, f, j])
    }
}
/// n = 12, i = 6, a < k, a < l, b < h, c < i, d < f, d < g, e < h, e < i, f < k, g < j, h < l, i < j, i < k, i < l
fn select_2520([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < h {
        select_2516([a, g, b, f, c, e, i, h, j, k])
    } else {
        select_2519([a, c, b, d, g, i, h, f, j, l])
    }
}
/// n = 15, i = 7, a < i, b < k, c < j, c < o, d < f, d < g, e < h, e < m, f < l, f < m, g < j, h < i, h < k, i < o, j < m, k < l, k < o, m < n
fn select_2521([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < k {
        select_2512([b, a, f, c, g, e, h, j, i, m, n, k])
    } else {
        select_2520([c, a, b, d, h, g, f, i, k, l, j, o])
    }
}
/// n = 15, i = 7, a < j, b < g, c < k, c < o, d < f, d < m, e < h, e < i, f < j, g < l, g < o, h < l, h < m, i < k, j < o, k < m, m < n
fn select_2522([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < f {
        select_2507([a, c, d, b, e, h, i, g, f, k, l, j, m, n, o])
    } else {
        select_2521([a, b, c, e, d, h, i, f, j, k, g, l, m, n, o])
    }
}
/// n = 15, i = 7, a < k, b < f, c < l, c < o, d < g, d < m, e < h, e < i, f < o, g < k, h < l, i < j, i < m, k < o, l < m, m < n
fn select_2523([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < f {
        select_2494([a, b, c, h, d, i, g, j, l, f, k, m, n])
    } else {
        select_2522([a, b, c, d, e, g, f, i, h, k, l, j, m, n, o])
    }
}
/// n = 11, i = 6, a < i, b < i, c < f, d < h, e < h, f < g, g < j, h < i, h < j, i < k
fn select_2524([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_2378([a, b, c, h, f, g, i, j, k])
    } else {
        select_562([a, b, d, e, g, h, j])
    }
}
/// n = 11, i = 6, a < j, b < j, c < f, d < h, e < h, f < g, f < i, h < i, h < j, j < k
fn select_2525([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_2524([a, b, c, d, e, f, g, h, j, i, k])
    } else {
        select_2378([a, b, c, h, f, i, j, g, k])
    }
}
/// n = 11, i = 6, a < j, b < j, c < f, c < g, d < h, e < h, g < i, h < i, h < j, j < k
fn select_2526([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_2525([a, b, c, d, e, g, f, h, i, j, k])
    } else {
        select_2524([a, b, c, d, e, f, g, h, j, i, k])
    }
}
/// n = 10, i = 5, a < j, b < j, c < f, c < h, d < g, e < g, f < i, g < h, g < i, g < j
fn select_2527([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_142([a, c, d, e, f, b, g, h, i, j])
    } else {
        select_142([b, c, d, e, f, a, g, h, i, j])
    }
}
/// n = 10, i = 5, a < j, b < j, c < i, d < h, e < g, f < g, g < h, g < i, g < j
fn select_2528([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_2527([a, b, c, e, f, d, g, i, h, j])
    } else {
        select_2527([a, b, d, e, f, c, g, h, i, j])
    }
}
/// n = 13, i = 6, a < k, a < l, b < j, b < k, c < h, d < f, d < g, e < h, e < i, f < l, g < i, h < k, i < j, i < l, k < m
fn select_2529([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_2526([a, b, d, c, e, f, g, h, i, k, m])
    } else {
        select_2528([a, f, b, c, e, g, i, h, j, l])
    }
}
/// n = 9, i = 4, a < g, b < g, b < h, c < e, c < f, d < f, d < g, d < h, e < h, g < i
fn select_2530([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_1089([b, c, a, d, f, e, g, i])
    } else {
        select_159([b, d, e, a, f, h])
    }
}
/// n = 7, i = 5, d < g, e < g, f < g
fn select_2531([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_1097([c, d, e, f, a, b, g])
    } else {
        select_1097([c, d, e, f, b, a, g])
    }
}
/// n = 11, i = 6, a < i, b < i, b < j, c < f, c < g, d < h, e < h, g < j, h < i, h < j, i < k
fn select_2532([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_2530([a, b, c, h, g, f, i, j, k])
    } else {
        select_2531([a, b, g, d, e, f, h])
    }
}
/// n = 15, i = 7, a < l, b < i, c < j, c < l, d < f, d < h, e < g, e < k, f < k, f < n, g < i, g < n, h < j, i < l, j < k, j < n, k < m, l < o
fn select_2533([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < i {
        select_2490([a, b, f, c, h, e, g, j, k, m, n, i])
    } else {
        select_2532([a, c, d, b, g, f, h, i, l, j, o])
    }
}
/// n = 16, i = 7, a < l, a < n, b < j, c < i, c < n, d < f, d < g, e < h, e < m, f < i, g < k, g < m, h < j, h < k, i < m, j < n, k < l, m < o, n < p
fn select_2534([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < i {
        select_2529([c, a, b, d, h, f, g, j, k, l, n, i, p])
    } else {
        select_2533([a, b, c, d, e, g, h, f, j, i, m, n, o, k, p])
    }
}
/// n = 16, i = 7, a < l, a < n, b < k, c < j, c < n, d < g, d < m, e < f, e < h, f < j, g < k, h < i, h < m, i < l, j < m, k < n, m < o, n < p
fn select_2535([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < g {
        select_2493([b, a, c, f, d, h, i, j, l, g, k, m, o])
    } else {
        select_2534([a, b, c, e, d, f, h, g, j, k, i, l, m, n, o, p])
    }
}
/// n = 10, i = 5, a < j, b < f, b < j, c < f, c < h, d < g, e < g, g < h, g < j, h < i
fn select_2536([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < g {
        select_165([a, c, d, e, f, g, h, j])
    } else {
        select_461([a, b, c, h, i, f, j])
    }
}
/// n = 10, i = 5, a < j, b < h, c < f, c < j, d < g, e < g, g < h, g < j, h < i
fn select_2537([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < f {
        select_2536([a, c, b, d, e, f, g, h, i, j])
    } else {
        select_12([a, f, d, e, b, g, j])
    }
}
/// n = 10, i = 6, b < g, c < i, d < h, e < h, f < g, f < i, h < i, i < j
fn select_2538([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < b {
        select_371([a, b, c, f, g, i, j])
    } else {
        select_926([a, c, d, e, f, g, h, i])
    }
}
/// n = 11, i = 6, b < k, c < i, d < h, e < h, f < g, f < i, g < k, h < i, h < k, i < j
fn select_2539([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_2537([b, c, g, d, e, a, h, i, j, k])
    } else {
        select_2538([b, a, c, d, e, f, g, h, i, j])
    }
}
/// n = 10, i = 4, a < f, b < g, b < i, b < j, c < e, c < g, d < f, d < g, d < i, e < i, f < j, g < h
fn select_2540([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_491([a, e, b, d, g, h, i])
    } else {
        select_488([b, a, c, d, e, f, j, i])
    }
}
/// n = 12, i = 6, a < i, b < g, b < k, b < l, c < f, c < g, d < h, e < h, f < k, g < j, h < i, h < j, h < k, i < l
fn select_2541([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_2540([a, b, c, h, f, i, g, j, k, l])
    } else {
        select_562([a, f, d, e, g, h, j])
    }
}
/// n = 12, i = 6, a < k, a < l, b < i, c < f, c < g, d < h, e < h, f < k, g < j, h < i, h < j, h < k, i < l
fn select_2542([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < g {
        select_2541([b, a, c, d, e, f, g, h, i, j, k, l])
    } else {
        select_936([f, b, g, d, e, a, h, j, i])
    }
}
/// n = 15, i = 7, a < k, b < j, c < i, c < o, d < f, d < g, e < h, e < m, f < l, f < m, g < i, h < j, i < m, j < k, j < l, k < o, m < n
fn select_2543([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < j {
        select_2539([a, b, f, c, g, e, h, i, m, n, j])
    } else {
        select_2542([c, a, d, b, h, g, f, j, k, l, i, o])
    }
}
/// n = 9, i = 3, a < f, a < i, b < e, b < h, c < d, c < f, d < h, e < i, f < g
fn select_2544([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < c {
        select_20([a, c, d, i])
    } else {
        select_123([d, a, b, e, f, g, h])
    }
}
/// n = 8, i = 4, a < e, a < f, b < f, b < g, c < f, c < g, d < e, d < g, g < h
fn select_2545([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < e {
        select_25([a, d, f, g, h])
    } else {
        select_162([b, c, d, e, f])
    }
}
/// n = 9, i = 4, a < g, b < e, b < f, c < f, c < g, c < h, d < f, d < g, d < h, e < h, g < i
fn select_2546([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_2545([b, c, d, a, e, f, g, i])
    } else {
        select_75([c, d, e, a, f, h])
    }
}
/// n = 10, i = 4, a < g, a < i, b < g, b < h, c < e, c < f, d < f, d < g, d < i, e < i, f < h, g < j
fn select_2547([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_82([e, b, f, a, h])
    } else {
        select_2546([b, c, a, d, e, f, g, i, j])
    }
}
/// n = 10, i = 4, a < h, a < i, b < f, b < h, c < e, c < g, d < f, d < g, d < h, g < i, h < j
fn select_2548([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_2544([a, c, b, f, g, h, j, e, i])
    } else {
        select_2547([b, a, c, d, e, g, h, i, f, j])
    }
}
/// n = 12, i = 6, a < j, a < k, b < i, b < j, c < f, c < g, d < h, e < h, g < k, h < i, h < j, h < k, j < l
fn select_2549([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < g {
        select_2548([a, b, c, h, f, i, g, j, k, l])
    } else {
        select_1504([f, a, b, d, e, g, h, i, k])
    }
}
/// n = 15, i = 7, a < k, a < m, b < j, c < i, c < m, d < f, d < g, e < h, e < l, f < l, g < i, h < j, i < l, j < k, j < m, l < n, m < o
fn select_2550([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < i {
        select_2549([c, a, d, b, h, f, g, j, k, m, i, o])
    } else {
        select_2539([a, b, f, c, g, e, h, i, l, n, j])
    }
}
/// n = 16, i = 7, a < k, b < i, b < n, c < j, c < n, d < f, d < g, e < h, e < m, f < l, f < m, g < j, h < i, h < k, i < l, j < m, k < n, m < o, n < p
fn select_2551([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < k {
        select_2543([a, b, c, d, e, f, g, h, j, i, k, l, m, o, n])
    } else {
        select_2550([b, a, c, d, e, f, g, h, j, k, i, m, n, o, p])
    }
}
/// n = 16, i = 7, a < k, b < f, b < n, c < j, c < n, d < i, d < m, e < g, e < h, f < l, g < l, g < m, h < j, i < k, j < m, k < n, m < o, n < p
fn select_2552([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if f < i {
        select_2507([a, c, d, b, e, g, h, f, i, j, l, k, m, o, n])
    } else {
        select_2551([a, b, c, e, d, g, h, i, f, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 7, a < l, b < f, b < n, c < k, c < n, d < i, d < m, e < g, e < h, g < k, h < j, h < m, i < l, k < m, l < n, m < o, n < p
fn select_2553([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < f {
        select_2535([b, a, c, d, e, g, i, h, j, k, l, f, m, n, o, p])
    } else {
        select_2552([a, b, c, d, e, f, h, g, i, k, l, j, m, n, o, p])
    }
}
/// n = 16, i = 7, b < n, c < l, d < k, d < n, e < i, e < m, f < g, f < h, g < k, h < j, h < m, i < l, k < m, l < n, m < o, n < p
fn select_2554([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_2523([c, a, d, e, f, b, i, g, h, j, l, k, m, o, n])
    } else {
        select_2553([c, b, d, e, f, a, g, h, i, j, k, l, m, n, o, p])
    }
}
/// n = 9, i = 4, a < f, b < f, c < g, c < i, d < e, e < g, f < i, g < h
fn select_2555([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_1572([a, b, c, e, g, h, f])
    } else {
        select_11([c, d, f, e, i])
    }
}
/// n = 8, i = 4, a < h, b < e, c < d, c < g, d < h, e < f, e < g
fn select_2556([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_321([a, c, b, d, e, f, h, g])
    } else {
        select_88([a, f, c, g, d])
    }
}
/// n = 10, i = 4, a < g, a < j, b < e, b < i, c < d, c < g, c < j, d < h, e < f, e < h, e < j, g < i
fn select_2557([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_659([f, a, c, d, h, g, j])
    } else {
        select_598([b, c, d, g, e, f, j, i])
    }
}
/// n = 10, i = 4, a < h, b < g, b < j, c < g, c < j, d < e, d < i, e < f, e < h, e < j, g < i
fn select_2558([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_2556([c, d, a, b, e, f, h, g])
    } else {
        select_2557([c, d, b, a, e, f, g, h, i, j])
    }
}
/// n = 10, i = 4, a < e, b < h, b < j, c < h, c < j, d < f, d < i, f < g, f < j, h < i
fn select_2559([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_2555([b, c, d, a, e, h, f, g, i])
    } else {
        select_2558([a, b, c, d, f, g, h, e, i, j])
    }
}
/// n = 9, i = 4, a < i, b < d, c < e, d < f, d < h, e < g, e < h, g < i
fn select_2560([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < d {
        select_11([a, b, g, d, i])
    } else {
        select_163([a, c, d, f, e, g, h])
    }
}
/// n = 11, i = 6, a < k, b < g, b < h, c < h, c < j, d < h, d < j, e < f, f < g, f < j, h < i, j < k
fn select_2561([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_2560([a, b, e, h, f, i, j, g, k])
    } else {
        select_2429([a, b, c, d, g, j, h, k])
    }
}
/// n = 8, i = 3, a < g, a < h, b < d, b < f, b < h, c < e, c < h, e < f, e < g
fn select_2562([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < e {
        select_11([a, d, c, e, h])
    } else {
        select_640([a, b, e, d, f, g, h])
    }
}
/// n = 10, i = 4, a < i, a < j, b < d, b < e, b < j, c < f, e < h, f < g, f < h, f < j, g < i
fn select_2563([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_2562([a, b, f, d, g, e, i, j])
    } else {
        select_164([d, a, e, c, f, g, h, j])
    }
}
/// n = 12, i = 6, a < i, b < k, b < l, c < g, c < j, d < g, d < j, e < f, f < i, f < j, f < l, g < h, g < l, j < k
fn select_2564([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < g {
        select_2561([b, a, c, d, e, f, i, g, h, j, k])
    } else {
        select_2563([b, g, e, h, a, f, j, i, k, l])
    }
}
/// n = 13, i = 6, a < i, b < j, b < m, c < g, c < l, d < g, d < l, e < f, f < i, f < j, f < l, g < h, j < k, l < m
fn select_2565([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < g {
        select_2559([a, c, d, b, i, j, k, l, m, g])
    } else {
        select_2564([a, b, c, d, e, f, g, h, i, l, m, j])
    }
}
/// n = 8, i = 4, b < f, b < h, c < f, c < g, c < h, d < e, e < g, e < h
fn select_2566([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_1463([b, c, e, a, g, f, h])
    } else {
        select_157([b, a, d, c, f, e, g])
    }
}
/// n = 8, i = 5, a < h, b < g, b < h, c < f, d < e, d < h, f < g
fn select_2567([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < h {
        select_637([a, b, f, d, e, g])
    } else {
        select_637([e, a, b, c, f, h])
    }
}
/// n = 11, i = 6, a < h, b < f, b < k, c < g, d < e, d < h, f < i, g < i, g < j, g < k, h < j, h < k
fn select_2568([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_2566([e, b, h, c, g, f, j, k])
    } else {
        select_2567([a, f, c, d, e, g, i, h])
    }
}
/// n = 9, i = 4, a < f, b < f, b < g, c < d, c < e, c < i, e < g, f < h, f < i
fn select_2569([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_758([a, c, e, f, h, g, d, i])
    } else {
        select_302([a, d, e, b, g, f])
    }
}
/// n = 9, i = 5, a < g, b < f, c < e, c < g, d < e, d < f, f < h, g < i
fn select_2570([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < f {
        select_159([b, d, g, i, e, f])
    } else {
        select_159([a, c, f, h, e, g])
    }
}
/// n = 10, i = 5, a < g, b < h, c < e, d < g, d < h, e < f, e < j, h < i, h < j
fn select_2571([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < a {
        select_2569([b, d, e, f, a, h, g, i, j])
    } else {
        select_2570([b, c, d, a, g, e, h, f, i])
    }
}
/// n = 13, i = 7, a < i, a < k, b < f, c < e, c < k, d < g, e < j, f < h, f < i, g < j, g < k, g < m, i < m, k < l
fn select_2572([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < k {
        select_2568([a, c, d, f, h, e, g, i, j, m, k])
    } else {
        select_2571([e, a, b, g, f, h, j, k, l, i])
    }
}
/// n = 13, i = 7, a < h, a < j, b < f, c < g, d < e, d < m, e < i, e < j, f < j, f < k, f < m, g < h, g < i, h < k, h < m, j < l
fn select_2573([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < h {
        select_2116([a, b, c, f, g, i, h, j, l])
    } else {
        select_474([b, d, e, f, k, i, m])
    }
}
/// n = 9, i = 5, a < g, b < g, b < i, c < f, d < e, e < h, e < i, f < h, f < i
fn select_2574([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_567([a, b, c, f, h, g, i])
    } else {
        select_580([b, c, d, g, e, f, h])
    }
}
/// n = 13, i = 7, a < e, a < m, b < i, b < j, c < f, d < g, e < j, f < j, f < k, f < m, g < h, g < i, i < k, i < m, j < l
fn select_2575([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < h {
        select_2573([b, c, d, a, e, f, g, i, h, j, k, l, m])
    } else {
        select_2574([h, a, b, c, f, i, e, k, m])
    }
}
/// n = 10, i = 5, a < h, b < h, b < j, c < f, d < e, d < h, e < j, f < g, f < j, h < i
fn select_2576([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < g {
        select_554([e, a, c, b, f, h, i, j])
    } else {
        select_373([a, b, d, f, g, e, j])
    }
}
/// n = 13, i = 7, a < j, a < k, b < f, c < e, c < i, d < g, e < k, f < h, f < j, g < i, g < k, g < l, j < l, k < m
fn select_2577([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if j < i {
        select_2575([c, a, d, b, e, g, f, h, j, k, l, m, i])
    } else {
        select_2576([e, a, b, g, i, f, h, k, m, j])
    }
}
/// n = 13, i = 7, a < i, b < k, c < j, c < k, d < f, e < g, f < h, f < j, g < i, g < k, g < m, j < m, k < l
fn select_2578([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < a {
        select_2572([c, d, b, e, a, f, g, h, j, i, k, l, m])
    } else {
        select_2577([c, d, a, e, b, f, g, h, i, j, k, m, l])
    }
}
/// n = 14, i = 7, a < j, b < k, b < n, c < f, c < m, d < g, d < k, e < h, f < i, g < m, h < j, h < k, h < m, k < l, m < n
fn select_2579([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < f {
        select_2565([a, b, c, g, e, h, f, i, j, k, l, m, n])
    } else {
        select_2578([a, b, d, c, e, f, h, i, j, g, k, l, m])
    }
}
/// n = 9, i = 4, a < f, a < i, b < f, b < g, b < i, c < d, d < e, d < g, d < i, f < h
fn select_2580([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_82([e, a, b, g, i])
    } else {
        select_474([c, a, f, d, g, h, i])
    }
}
/// n = 11, i = 6, a < h, b < e, b < k, c < h, c < i, d < f, e < g, f < i, f < j, f < k, h < j, h < k
fn select_2581([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < e {
        select_2580([b, h, d, f, i, e, j, g, k])
    } else {
        select_2567([a, c, d, e, g, f, i, h])
    }
}
/// n = 13, i = 7, a < g, a < k, b < h, c < g, c < l, d < h, d < i, e < f, f < i, f < k, f < l, g < j, h < k, h < l, l < m
fn select_2582([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < h {
        select_2581([b, c, d, e, g, f, j, h, i, k, l])
    } else {
        select_442([a, c, e, f, i, g, k, j, l, m])
    }
}
/// n = 10, i = 5, a < g, a < j, b < h, c < e, c < i, d < g, d < h, e < f, h < i, h < j
fn select_2583([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_1340([a, c, h, g, i, j, e, f])
    } else {
        select_1366([a, b, d, e, f, g, h, i])
    }
}
/// n = 14, i = 7, a < h, a < i, b < g, b < k, c < h, c < k, d < g, d < l, d < m, e < f, f < i, f < k, f < m, g < j, h < m, k < l, m < n
fn select_2584([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < k {
        select_2582([b, c, d, a, e, f, g, h, i, j, k, m, n])
    } else {
        select_2583([a, c, d, f, g, j, i, k, l, h])
    }
}
/// n = 8, i = 4, a < f, a < h, b < e, b < f, c < d, c < e, d < h, f < g
fn select_2585([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_474([b, c, d, f, g, h, e])
    } else {
        select_52([d, a, b, e, h, f])
    }
}
/// n = 7, i = 4, a < g, b < e, c < d, c < g, e < f
fn select_2586([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_843([a, c, e, f, d])
    } else {
        select_843([d, a, b, e, g])
    }
}
/// n = 10, i = 5, a < h, a < i, b < e, c < d, c < g, d < f, d < h, e < g, e < i, i < j
fn select_2587([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < f {
        select_2585([a, e, c, d, g, i, j, h])
    } else {
        select_2586([a, b, d, f, e, g, h])
    }
}
/// n = 9, i = 4, a < f, a < i, b < e, b < f, c < d, c < e, d < i, e < g, f < h
fn select_2588([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_474([b, c, d, f, h, i, e])
    } else {
        select_567([d, a, b, e, g, i, f])
    }
}
/// n = 11, i = 5, a < g, a < i, b < e, c < d, c < h, d < f, d < g, e < h, e < i, h < j, i < k
fn select_2589([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < j {
        select_2587([a, b, c, d, e, f, h, g, i, k])
    } else {
        select_2588([a, e, c, d, h, i, j, k, g])
    }
}
/// n = 11, i = 5, a < g, a < k, b < f, b < g, c < g, c < h, d < e, e < f, e < h, g < j, h < i, h < k
fn select_2590([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_3([j, d, e])
    } else {
        select_186([a, b, c, f, h, i, k, g])
    }
}
/// n = 8, i = 4, a < f, b < f, b < g, c < d, d < e, d < g, f < h
fn select_2591([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < g {
        select_449([a, b, c, d, e, f, h, g])
    } else {
        select_3([e, g, a])
    }
}
/// n = 12, i = 6, a < i, a < j, a < l, b < g, b < i, c < f, d < e, d < l, e < h, e < i, f < g, f < h, g < j, g < l, i < k
fn select_2592([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < g {
        select_2591([a, b, c, f, h, i, g, k])
    } else {
        select_42([a, d, j, e, h, l])
    }
}
/// n = 13, i = 6, a < g, a < j, b < i, b < j, b < l, c < f, d < e, d < i, e < h, e < j, f < g, f < h, f < i, g < l, i < k, j < m
fn select_2593([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < g {
        select_2590([a, e, b, c, f, h, j, i, k, m, g])
    } else {
        select_2592([b, a, c, d, e, f, g, h, j, l, m, i])
    }
}
/// n = 13, i = 6, a < g, a < i, b < e, b < k, c < g, c < k, c < m, d < f, e < j, f < i, f < j, f < k, g < h, i < m, k < l
fn select_2594([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < e {
        select_2589([b, d, a, g, f, h, e, i, k, m, l])
    } else {
        select_2593([a, c, d, b, e, f, i, j, k, g, l, m, h])
    }
}
/// n = 14, i = 7, a < j, b < h, b < k, c < h, c < l, c < n, d < f, d < k, e < g, f < l, g < j, g < k, g < l, h < i, k < n, l < m
fn select_2595([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < f {
        select_2584([a, b, d, c, e, g, h, f, j, i, k, n, l, m])
    } else {
        select_2594([b, f, c, e, a, g, h, i, k, j, l, m, n])
    }
}
/// n = 10, i = 4, a < d, b < f, b < g, c < e, c < h, e < g, e < j, f < h, f < j, h < i
fn select_2596([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_247([a, b, e, d, g, f, h, j])
    } else {
        select_1204([a, c, f, d, h, e, g, j, i])
    }
}
/// n = 9, i = 4, a < g, b < f, c < d, c < e, e < h, e < i, f < g, f < h, f < i
fn select_2597([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < d {
        select_842([a, b, c, e, f, d, g, h, i])
    } else {
        select_306([d, e, b, a, f, h, i])
    }
}
/// n = 12, i = 6, a < i, b < g, c < f, c < h, d < e, d < j, e < h, e < l, f < j, f < l, g < i, g < j, g < l, j < k
fn select_2598([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < e {
        select_2596([a, c, d, i, e, f, h, j, k, l])
    } else {
        select_2597([a, b, c, h, f, g, i, j, l])
    }
}
/// n = 9, i = 4, a < g, b < f, c < g, c < h, d < e, d < f, e < h, h < i
fn select_2599([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_109([b, c, e, f, a, h, i, g])
    } else {
        select_125([b, c, d, a, g, f, e, h])
    }
}
/// n = 12, i = 6, a < i, b < f, b < h, c < g, d < e, d < j, e < h, e < l, f < k, f < l, g < i, g < j, g < l, j < k
fn select_2600([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < j {
        select_2598([a, c, b, d, e, f, g, h, i, j, k, l])
    } else {
        select_2599([e, a, b, g, j, i, h, f, k])
    }
}
/// n = 8, i = 4, a < d, b < f, c < e, e < g, e < h, f < g, f < h
fn select_2601([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_255([a, e, b, f, d, g, h])
    } else {
        select_162([c, d, b, f, e])
    }
}
/// n = 9, i = 5, a < g, b < h, c < g, c < h, d < e, e < f, e < i, h < i
fn select_2602([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_2601([a, b, d, g, e, h, f, i])
    } else {
        select_158([b, a, c, e, f, g, h])
    }
}
/// n = 8, i = 5, a < h, b < g, c < g, c < h, d < e, e < f
fn select_2603([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if h < f {
        select_580([b, a, d, g, e, h, f])
    } else {
        select_158([a, b, c, e, f, g, h])
    }
}
/// n = 9, i = 5, a < g, b < h, c < g, c < h, d < e, d < f, f < i, h < i
fn select_2604([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < e {
        select_2602([a, b, c, d, f, e, g, h, i])
    } else {
        select_2603([a, b, c, d, e, f, h, g])
    }
}
/// n = 11, i = 6, a < i, b < h, b < j, c < g, d < e, d < f, e < k, f < h, g < i, g < j, j < k
fn select_2605([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < i {
        select_1719([a, e, f, b, c, g, h, j, k])
    } else {
        select_2604([a, b, g, d, f, e, i, j, k])
    }
}
/// n = 12, i = 6, a < i, b < f, b < j, c < e, c < g, d < h, f < l, g < k, g < l, h < i, h < j, h < l, j < k
fn select_2606([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < e {
        select_2600([a, c, d, b, f, g, h, e, i, j, k, l])
    } else {
        select_2605([a, b, d, c, g, e, h, f, i, j, k])
    }
}
/// n = 14, i = 7, a < j, b < l, b < n, c < f, c < k, d < g, d < k, e < h, f < i, g < l, h < j, h < k, h < l, k < n, l < m
fn select_2607([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < f {
        select_2595([a, c, b, d, e, g, h, f, i, j, k, l, m, n])
    } else {
        select_2606([a, d, f, e, i, g, b, h, j, k, n, l])
    }
}
/// n = 14, i = 7, a < j, b < l, c < k, d < l, d < n, e < g, e < k, f < h, g < i, h < j, h < k, h < l, k < n, l < m
fn select_2608([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if b < c {
        select_2579([a, d, e, b, f, g, c, h, i, j, l, m, k, n])
    } else {
        select_2607([a, d, e, c, f, g, b, h, i, j, k, l, m, n])
    }
}
/// n = 11, i = 5, a < f, b < g, c < e, c < g, d < e, d < f, d < j, f < i, g < h, g < j, j < k
fn select_2609([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_2020([b, c, d, f, i, e, g, h, j])
    } else {
        select_59([h, a, d, e, j, k, f])
    }
}
/// n = 13, i = 6, a < j, a < m, b < f, b < i, c < f, c < g, c < j, d < e, d < j, e < g, e < i, f < h, f < m, i < k, j < l
fn select_2610([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_2609([a, b, e, c, g, j, i, k, l, f, h])
    } else {
        select_1189([a, b, d, e, f, g, h, j, l, m, i, k])
    }
}
/// n = 10, i = 4, a < g, a < h, b < d, b < g, c < e, c < h, d < j, e < f, e < j, h < i
fn select_2611([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_151([e, b, a, f, h, i, g, d])
    } else {
        select_1154([a, b, c, d, e, f, g, j, h])
    }
}
/// n = 8, i = 4, a < g, b < f, b < g, c < d, c < e, e < h, f < h
fn select_2612([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_132([a, f, c, g, e, d, h])
    } else {
        select_277([e, a, d, b, f, g])
    }
}
/// n = 10, i = 4, a < g, a < h, b < e, b < g, c < d, c < f, e < j, f < h, f < j, h < i
fn select_2613([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_371([d, b, f, a, g, h, i])
    } else {
        select_2612([a, b, c, d, f, e, g, j])
    }
}
/// n = 10, i = 4, a < e, a < j, b < h, b < j, c < g, c < h, d < f, d < g, f < j, h < i
fn select_2614([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < a {
        select_2611([c, d, b, f, a, e, g, h, i, j])
    } else {
        select_2613([c, d, a, e, f, b, g, h, i, j])
    }
}
/// n = 10, i = 4, a < f, b < f, b < g, b < i, c < d, c < e, e < g, e < i, f < h, i < j
fn select_2615([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_4([c, f, h, d, e])
    } else {
        select_989([d, a, b, e, g, i, j, f])
    }
}
/// n = 10, i = 4, a < f, a < j, b < g, b < j, c < f, c < g, d < e, d < g, e < j, g < h, g < i
fn select_2616([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_158([b, c, a, d, e, f, j])
    } else {
        select_503([a, e, c, b, g, h, i, j])
    }
}
/// n = 11, i = 4, a < h, a < k, b < e, b < h, c < e, c < f, d < f, d < h, f < g, f < k, h < i, h < j
fn select_2617([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_122([g, a, b, e, k])
    } else {
        select_2073([a, c, d, e, f, h, g, i, j])
    }
}
/// n = 11, i = 4, a < f, a < g, b < f, b < i, c < e, c < i, d < g, d < i, g < h, i < j, i < k
fn select_2618([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < g {
        select_2616([a, d, b, c, e, f, i, j, k, g])
    } else {
        select_2617([c, b, a, d, f, g, h, i, j, k, e])
    }
}
/// n = 12, i = 5, a < h, a < i, b < e, b < g, c < g, c < i, d < f, d < i, d < j, f < h, f < l, g < j, g < l, i < k
fn select_2619([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_2615([a, d, b, e, g, i, j, k, f, l])
    } else {
        select_2618([a, f, b, c, e, h, i, k, g, j, l])
    }
}
/// n = 12, i = 5, a < i, a < j, b < e, b < h, c < h, c < j, d < f, d < j, d < k, f < g, f < i, h < k, j < l
fn select_2620([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_2614([b, c, a, f, e, g, i, j, l, h])
    } else {
        select_2619([a, b, c, d, e, f, h, i, j, k, l, g])
    }
}
/// n = 13, i = 6, a < k, a < l, b < f, b < k, c < e, c < h, d < h, d < j, d < k, e < g, f < j, h < i, h < l, k < m
fn select_2621([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < e {
        select_2610([a, c, d, b, f, h, j, i, e, k, g, m, l])
    } else {
        select_2620([a, e, b, d, g, h, i, f, l, k, j, m])
    }
}
/// n = 9, i = 3, a < e, a < i, b < c, b < d, b < h, d < f, d < i, e < g, e < h
fn select_2622([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_35([b, e, g, c, d, h])
    } else {
        select_409([c, a, d, f, e, i])
    }
}
/// n = 11, i = 4, a < f, a < g, b < e, b < f, b < k, c < d, c < e, c < g, e < i, f < j, g < h, g < k
fn select_2623([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_2622([b, c, d, g, f, h, j, e, k])
    } else {
        select_1544([d, a, b, e, g, i, h, k, f])
    }
}
/// n = 13, i = 6, a < i, a < j, a < k, b < g, b < k, c < f, c < g, c < j, d < e, d < f, e < j, e < k, g < h, g < i, j < l, k < m
fn select_2624([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < g {
        select_2623([b, a, c, f, j, k, g, h, l, m, i])
    } else {
        select_32([h, a, d, f, i, e, j, k])
    }
}
/// n = 8, i = 4, a < g, b < f, b < g, c < d, c < f, d < e, g < h
fn select_2625([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_159([a, b, d, e, f, g])
    } else {
        select_4([c, g, h, f, d])
    }
}
/// n = 9, i = 4, a < g, a < h, b < e, c < d, c < g, d < f, d < i, e < h, e < i
fn select_2626([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < a {
        select_97([c, e, a, d, h, f, i, g])
    } else {
        select_2625([b, a, c, d, f, g, e, h])
    }
}
/// n = 10, i = 4, a < g, a < h, b < d, b < h, c < e, c < g, c < j, d < i, e < f, e < i, h < j
fn select_2627([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_2020([b, a, c, e, f, g, h, d, j])
    } else {
        select_2626([a, b, c, e, d, f, g, h, i])
    }
}
/// n = 9, i = 3, a < e, a < g, b < d, b < f, c < f, c < g, f < h, g < i
fn select_2628([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_275([e, b, c, d, g, i, f])
    } else {
        select_275([d, a, c, e, f, h, g])
    }
}
/// n = 10, i = 4, a < e, a < g, b < d, b < h, c < f, c < g, e < j, f < h, f < j, h < i
fn select_2629([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_1963([b, e, c, g, d, f, j])
    } else {
        select_2628([a, b, f, d, g, h, e, i, j])
    }
}
/// n = 11, i = 5, a < h, a < j, b < e, b < i, c < d, c < h, c < k, d < g, d < i, e < f, e < j, j < k
fn select_2630([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_2627([a, e, c, f, d, i, h, j, g, k])
    } else {
        select_2629([d, a, b, h, g, e, i, j, k, f])
    }
}
/// n = 13, i = 6, a < e, a < f, b < g, b < k, b < l, c < g, c < h, d < f, d < h, d < l, e < l, g < j, h < i, h < k, l < m
fn select_2631([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < g {
        select_2624([b, c, d, a, e, f, h, i, k, l, g, m, j])
    } else {
        select_2630([a, b, d, h, g, j, i, f, k, e, l])
    }
}
/// n = 9, i = 3, a < c, a < h, b < d, b < h, c < g, d < e, d < f, h < i
fn select_2632([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_42([c, b, g, h, i, d])
    } else {
        select_210([a, d, e, f, h, i, c])
    }
}
/// n = 9, i = 4, a < e, a < i, b < f, b < h, c < d, c < f, e < h, f < g, f < i
fn select_2633([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < b {
        select_25([a, b, e, f, g])
    } else {
        select_98([a, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, a < d, a < h, b < e, b < i, c < f, c < g, e < g, f < h, f < i, i < j
fn select_2634([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_2632([b, f, e, a, d, h, g, i, j])
    } else {
        select_2633([b, c, a, d, e, f, h, g, i])
    }
}
/// n = 10, i = 4, a < d, a < i, b < g, b < h, c < e, c < f, e < j, f < h, g < i, g < j
fn select_2635([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < g {
        select_133([f, a, e, b, d, h, g, j])
    } else {
        select_2634([a, c, b, d, f, g, h, i, e, j])
    }
}
/// n = 9, i = 4, a < g, a < h, b < d, b < h, c < e, c < h, d < g, d < i, e < f, e < i
fn select_2636([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_150([a, b, e, f, h, d, i])
    } else {
        select_256([a, c, d, g, h, e, f, i])
    }
}
/// n = 9, i = 5, a < f, b < d, b < i, c < e, c < i, d < h, e < g, f < h, f < i
fn select_2637([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_452([d, a, c, e, f, i, h, g])
    } else {
        select_1509([b, a, e, g, f, d, i])
    }
}
/// n = 10, i = 5, a < d, a < i, b < f, c < e, c < i, e < g, e < j, f < h, f < i, h < j
fn select_2638([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_2636([a, f, c, h, e, g, d, i, j])
    } else {
        select_2637([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 9, i = 4, a < e, b < d, b < g, c < f, d < h, e < i, f < g, f < h, f < i
fn select_2639([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_26([b, c, e, d, f, i])
    } else {
        select_140([a, b, f, d, g, e, h, i])
    }
}
/// n = 10, i = 4, a < d, a < i, b < f, b < i, b < j, c < e, c < g, c < j, d < j, e < h, e < i, f < g, f < h
fn select_2640([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_470([d, b, c, e, f, i, j])
    } else {
        select_1888([a, f, c, e, g, d, h, i])
    }
}
/// n = 11, i = 5, a < d, a < j, b < f, c < e, c < j, c < k, d < k, e < g, e < i, f < h, f < i, f < k, h < j
fn select_2641([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_2639([a, e, b, g, d, f, i, h, k])
    } else {
        select_2640([a, c, f, d, h, e, i, g, j, k])
    }
}
/// n = 11, i = 5, a < e, a < j, b < g, b < j, b < k, c < d, c < f, d < j, e < k, f < i, f < k, g < h, g < i
fn select_2642([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < f {
        select_2638([a, c, b, e, g, d, h, f, j, i])
    } else {
        select_2641([a, c, b, e, g, f, h, d, i, j, k])
    }
}
/// n = 11, i = 5, a < e, a < i, b < d, b < g, c < f, c < i, c < k, e < k, f < h, f < j, g < j, g < k
fn select_2643([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < d {
        select_2635([f, b, a, h, e, i, g, d, j, k])
    } else {
        select_2642([a, c, b, d, e, g, f, h, j, i, k])
    }
}
/// n = 13, i = 6, a < k, a < l, b < f, b < h, c < e, c < i, d < h, d < i, d < l, e < g, f < l, i < j, i < k, l < m
fn select_2644([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < e {
        select_2631([b, a, c, d, f, h, e, i, j, g, k, l, m])
    } else {
        select_2643([b, e, d, g, f, i, a, j, h, k, l])
    }
}
/// n = 13, i = 6, a < h, b < k, c < k, c < l, d < f, d < i, e < h, e < i, e < k, f < g, i < j, i < l, k < m
fn select_2645([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < a {
        select_2621([c, b, d, e, f, a, g, i, j, h, k, l, m])
    } else {
        select_2644([c, a, d, e, f, b, g, h, i, j, l, k, m])
    }
}
/// n = 14, i = 7, b < j, c < l, d < l, d < m, e < g, e < k, f < h, g < i, h < j, h < k, h < l, k < m, l < n
fn select_2646([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < k {
        select_2608([b, c, a, d, e, f, g, h, i, j, k, l, n, m])
    } else {
        select_2645([b, c, d, e, h, g, i, j, k, a, l, m, n])
    }
}
/// n = 9, i = 5, a < f, b < h, b < i, c < e, c < i, d < g, g < h, g < i
fn select_2647([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < i {
        select_1387([b, a, c, d, e, f, g, h])
    } else {
        select_1206([a, b, g, c, e, h, i])
    }
}
/// n = 11, i = 6, a < k, b < f, c < i, c < j, d < h, e < g, e < i, g < j, h < i, h < j, i < k
fn select_2648([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_1770([b, c, a, e, d, f, h, g, i, k, j])
    } else {
        select_2647([b, c, g, d, a, f, h, i, j])
    }
}
/// n = 12, i = 6, a < g, b < k, b < l, c < j, d < i, e < i, e < j, f < h, h < i, h < j, i < k, j < l
fn select_2649([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < d {
        select_2648([b, a, e, f, c, g, d, h, j, i, l])
    } else {
        select_2648([b, a, e, f, d, g, c, h, i, j, k])
    }
}
/// n = 11, i = 6, a < i, b < f, c < h, c < i, c < j, d < g, e < g, f < h, f < j, g < i, g < j, i < k
fn select_2650([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_1899([a, b, c, f, i, h, j, k])
    } else {
        select_372([a, d, e, c, h, g, j])
    }
}
/// n = 10, i = 5, a < f, b < i, c < f, c < h, d < g, e < g, g < h, g < i, i < j
fn select_2651([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_165([b, c, d, e, f, g, h, i])
    } else {
        select_123([a, b, c, h, i, j, f])
    }
}
/// n = 12, i = 6, a < j, b < g, c < h, d < h, e < f, e < j, e < l, f < i, g < l, h < i, h < j, h < l, j < k
fn select_2652([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_2650([a, b, e, c, d, g, h, f, j, l, k])
    } else {
        select_2651([b, a, f, c, d, g, h, i, j, k])
    }
}
/// n = 10, i = 6, a < j, b < g, c < h, d < h, e < f, e < i, f < j, g < i, h < i, h < j
fn select_2653([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < f {
        select_524([a, c, d, e, g, f, h, i])
    } else {
        select_925([a, b, c, d, f, g, h, j])
    }
}
/// n = 10, i = 6, a < j, b < f, c < h, d < h, e < g, e < i, g < j, h < i, h < j
fn select_2654([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < i {
        select_2653([a, b, c, d, e, g, f, h, i, j])
    } else {
        select_1008([a, g, b, h, i, f, j])
    }
}
/// n = 12, i = 6, a < i, b < j, c < g, d < j, d < l, e < h, f < h, g < l, h < i, h < j, h < l, j < k
fn select_2655([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < a {
        select_2652([b, c, e, f, d, a, g, h, i, j, k, l])
    } else {
        select_2654([b, c, e, f, a, g, d, h, i, j])
    }
}
/// n = 8, i = 3, a < h, b < d, c < e, e < f, e < h, f < g
fn select_2656([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < d {
        select_58([b, a, e, f, g, h])
    } else {
        select_124([a, b, c, d, e, h])
    }
}
/// n = 9, i = 4, a < f, b < e, c < g, d < f, d < g, g < h, h < i
fn select_2657([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < e {
        select_2656([b, a, c, f, g, h, i, e])
    } else {
        select_158([c, a, d, b, e, f, g])
    }
}
/// n = 12, i = 6, a < i, b < k, b < l, c < j, d < g, e < h, f < h, g < l, h < i, h < j, h < l, j < k
fn select_2658([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < j {
        select_2655([a, c, d, b, e, f, g, h, i, j, k, l])
    } else {
        select_2657([a, d, c, h, g, i, j, b, k])
    }
}
/// n = 13, i = 6, a < j, a < l, b < g, c < k, c < m, d < i, e < h, f < i, f < k, g < m, h < k, i < j, i < m, k < l
fn select_2659([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_2649([b, a, c, d, f, e, g, h, i, k, j, l])
    } else {
        select_2658([a, c, e, b, d, f, g, i, j, h, k, m])
    }
}
/// n = 11, i = 6, a < f, b < g, c < h, d < h, e < g, e < k, f < k, g < i, g < j, h < i, h < j, h < k
fn select_2660([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_1328([a, c, d, g, f, h, i, j])
    } else {
        select_523([b, e, f, c, d, h, g, k])
    }
}
/// n = 11, i = 6, a < f, b < g, b < k, c < g, c < i, d < h, e < h, f < k, g < j, h < i, h < j, h < k
fn select_2661([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_2660([a, c, d, e, b, f, g, h, i, j, k])
    } else {
        select_488([b, c, a, h, f, i, g, k])
    }
}
/// n = 10, i = 4, a < g, b < e, b < f, c < f, c < g, d < e, d < g, d < h, f < h, g < i, g < j
fn select_2662([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_53([a, c, d, e, f, h])
    } else {
        select_514([b, a, c, e, g, i, j])
    }
}
/// n = 12, i = 6, a < g, a < i, b < f, b < j, c < g, c < j, d < h, e < h, g < l, h < i, h < j, h < l, j < k
fn select_2663([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < j {
        select_2661([b, c, a, d, e, f, g, h, i, l, j])
    } else {
        select_2662([b, a, c, h, i, g, j, l, f, k])
    }
}
/// n = 8, i = 5, a < f, b < e, c < g, d < g, g < h
fn select_2664([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < g {
        select_141([a, c, d, e, f, g, h])
    } else {
        select_16([b, a, g, h, f, e])
    }
}
/// n = 6, i = 3, b < e, b < f, c < d, c < f
fn select_2665([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < f {
        select_53([a, c, b, e, d, f])
    } else {
        select_88([a, e, c, f, d])
    }
}
/// n = 9, i = 5, a < f, b < e, b < i, c < g, d < g, g < h, g < i
fn select_2666([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < i {
        select_2664([a, b, c, d, e, f, g, h])
    } else {
        select_2665([a, b, g, h, e, i])
    }
}
/// n = 12, i = 6, a < k, b < g, b < i, c < f, c < k, d < h, e < h, g < j, h < i, h < j, h < k, k < l
fn select_2667([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < g {
        select_2663([b, c, a, d, e, f, g, h, i, k, l, j])
    } else {
        select_2666([c, g, d, e, a, f, h, i, j])
    }
}
/// n = 8, i = 3, a < e, b < f, b < h, c < d, c < f, e < h, f < g
fn select_2668([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < e {
        select_88([a, d, b, f, g])
    } else {
        select_124([b, c, a, d, e, h])
    }
}
/// n = 12, i = 6, a < h, a < i, b < h, b < j, b < l, c < f, c < j, d < g, e < g, f < h, g < i, g < j, i < l, j < k
fn select_2669([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_878([a, b, d, e, h, g, j, i])
    } else {
        select_2668([a, b, c, f, i, j, k, l])
    }
}
/// n = 10, i = 4, a < g, b < e, b < f, c < f, c < g, c < i, d < e, d < g, d < h, e < i, f < h, g < j
fn select_2670([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < f {
        select_1899([a, b, c, e, g, f, i, j])
    } else {
        select_1899([a, b, d, f, g, e, h, j])
    }
}
/// n = 13, i = 6, a < h, a < j, b < f, b < i, c < g, d < g, e < f, e < j, e < k, f < h, f < l, g < i, g < j, g < l, i < k, j < m
fn select_2671([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < h {
        select_2670([a, b, e, g, i, f, j, l, k, m])
    } else {
        select_7([c, d, h])
    }
}
/// n = 13, i = 6, a < f, a < j, b < g, b < i, c < g, c < j, c < k, d < h, e < h, g < l, h < i, h < j, h < l, i < k, j < m
fn select_2672([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < g {
        select_2669([b, c, a, d, e, f, h, g, i, j, m, k])
    } else {
        select_2671([a, b, d, e, c, g, h, f, i, j, k, l, m])
    }
}
/// n = 13, i = 6, a < k, a < l, b < f, b < k, c < g, c < i, d < h, e < h, g < j, h < i, h < j, h < k, i < l, k < m
fn select_2673([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < g {
        select_2672([b, c, a, d, e, f, g, h, i, k, l, j, m])
    } else {
        select_2666([b, g, d, e, a, f, h, i, j])
    }
}
/// n = 13, i = 6, a < i, b < j, c < k, c < m, d < g, d < k, e < h, f < h, h < i, h < j, h < k, j < m, k < l
fn select_2674([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_2667([c, a, d, e, f, g, b, h, i, j, k, l])
    } else {
        select_2673([c, d, b, e, f, g, a, h, j, i, k, m, l])
    }
}
/// n = 14, i = 6, a < j, a < n, b < h, c < g, c < l, d < k, d < l, e < i, f < i, f < k, h < k, i < j, i < l, k < n, l < m
fn select_2675([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_2649([c, a, d, e, f, b, g, h, i, k, j, n])
    } else {
        select_2674([a, b, d, c, e, f, g, i, j, h, l, m, k])
    }
}
/// n = 14, i = 6, b < l, c < j, c < n, d < h, e < k, e < l, f < i, g < i, g < k, h < k, i < j, i < l, k < n, l < m
fn select_2676([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_2659([c, a, e, f, d, g, b, h, i, j, k, n, l])
    } else {
        select_2675([c, d, b, e, f, g, a, h, i, j, k, l, m, n])
    }
}
/// n = 12, i = 6, a < f, b < h, c < h, d < g, d < i, e < g, e < i, g < k, g < l, h < i, h < k, h < l, i < j
fn select_2677([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < a {
        select_1204([a, d, e, f, i, g, k, l, j])
    } else {
        select_524([f, b, c, d, e, g, h, i])
    }
}
/// n = 12, i = 6, a < f, b < g, b < j, c < h, d < h, e < g, e < i, e < j, g < l, h < i, h < j, h < l, j < k
fn select_2678([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_2677([a, c, d, b, e, f, g, h, j, k, i, l])
    } else {
        select_1372([a, b, e, h, f, i, j, k, g])
    }
}
/// n = 12, i = 6, a < f, b < k, c < h, d < h, e < g, e < i, e < k, g < j, h < i, h < j, h < k, k < l
fn select_2679([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < g {
        select_2678([a, b, c, d, e, f, g, h, i, k, l, j])
    } else {
        select_145([a, g, c, d, f, b, h, i, j, k])
    }
}
/// n = 8, i = 5, b < f, c < g, d < g, e < f, e < h, g < h
fn select_2680([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < a {
        select_1652([a, b, e, d, g, f, h])
    } else {
        select_157([b, c, d, e, f, g, h])
    }
}
/// n = 11, i = 6, a < f, b < g, c < h, d < h, e < g, e < i, g < j, g < k, h < i, h < j, h < k
fn select_2681([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < a {
        select_609([a, b, e, i, f, g, j, k])
    } else {
        select_2680([f, b, c, d, e, g, h, i])
    }
}
/// n = 11, i = 6, a < k, b < f, c < h, d < h, e < g, e < i, g < j, g < k, h < i, h < j, h < k
fn select_2682([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < g {
        select_2681([b, a, c, d, e, f, g, h, i, j, k])
    } else {
        select_145([b, g, c, d, f, a, h, i, j, k])
    }
}
/// n = 12, i = 6, a < j, b < g, c < k, d < i, d < k, e < h, f < h, h < i, h < j, h < k, k < l
fn select_2683([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < a {
        select_2679([b, c, e, f, d, g, a, h, i, j, k, l])
    } else {
        select_2682([c, b, e, f, a, g, d, h, j, i, k])
    }
}
/// n = 9, i = 4, b < f, c < g, d < g, e < f, e < g, g < h, g < i
fn select_2684([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < b {
        select_1461([a, b, d, e, f, g, h, i])
    } else {
        select_2164([a, c, d, e, f, g])
    }
}
/// n = 10, i = 4, a < j, b < i, c < g, c < h, d < f, e < f, f < g, f < h, f < i, f < j
fn select_2685([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_562([b, c, a, d, e, f, j])
    } else {
        select_363([b, c, f, a, j, g, h, i])
    }
}
/// n = 10, i = 4, a < j, b < f, c < h, c < i, d < g, e < f, e < g, g < h, g < i, g < j
fn select_2686([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_526([a, c, d, b, e, f, g])
    } else {
        select_2685([a, b, c, d, e, g, h, i, f, j])
    }
}
/// n = 11, i = 4, a < f, b < j, b < k, c < h, d < g, d < h, e < f, e < g, e < h, g < j, g < k, h < i
fn select_2687([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_2684([b, a, c, d, e, f, h, g, i])
    } else {
        select_2686([c, a, b, d, e, f, g, j, k, h])
    }
}
/// n = 13, i = 6, a < i, b < k, c < g, c < l, d < j, d < k, e < h, f < h, h < i, h < j, h < k, j < l, k < m
fn select_2688([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < j {
        select_2655([a, b, c, d, e, f, g, h, i, k, m, j])
    } else {
        select_2687([a, c, b, d, h, i, j, k, m, g, l])
    }
}
/// n = 13, i = 6, b < m, c < i, d < k, e < j, e < k, f < h, g < h, h < i, h < j, h < k, j < m, k < l
fn select_2689([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < b {
        select_2683([c, a, d, e, f, g, b, h, j, i, k, l])
    } else {
        select_2688([c, d, b, e, f, g, a, h, i, j, k, m, l])
    }
}
/// n = 12, i = 6, c < g, c < j, d < h, e < h, f < g, f < i, f < j, g < l, h < i, h < j, h < l, j < k
fn select_2690([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_2678([a, c, d, e, f, b, g, h, i, j, k, l])
    } else {
        select_2678([b, c, d, e, f, a, g, h, i, j, k, l])
    }
}
/// n = 10, i = 5, a < e, b < g, c < g, d < f, d < h, f < i, f < j, g < h, g < i, g < j
fn select_2691([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_64([a, d, h, e, f, i, j])
    } else {
        select_372([e, b, c, d, f, g, h])
    }
}
/// n = 10, i = 5, c < g, d < g, e < f, e < h, f < i, f < j, g < h, g < i, g < j
fn select_2692([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_2691([a, c, d, e, b, f, g, h, i, j])
    } else {
        select_2691([b, c, d, e, a, f, g, h, i, j])
    }
}
/// n = 10, i = 5, c < f, c < i, d < g, e < g, f < j, g < h, g < i, g < j
fn select_2693([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < h {
        select_2692([a, b, d, e, c, f, g, i, h, j])
    } else {
        select_279([a, b, c, g, h, i, f])
    }
}
/// n = 12, i = 6, c < k, d < h, e < h, f < g, f < i, f < k, g < j, h < i, h < j, h < k, k < l
fn select_2694([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < g {
        select_2690([a, b, c, d, e, f, g, h, i, k, l, j])
    } else {
        select_2693([a, b, g, d, e, c, h, i, j, k])
    }
}
/// n = 8, i = 4, b < f, b < g, c < f, c < g, d < e, d < g, d < h, f < h
fn select_2695([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_273([a, b, c, d, e, g, f])
    } else {
        select_105([f, d, e, g, a, h])
    }
}
/// n = 9, i = 6, b < h, c < h, d < h, e < g, e < i, f < g, f < i, h < i
fn select_2696([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < g {
        select_1831([a, b, c, d, e, f, g, h, i])
    } else {
        select_141([g, b, c, d, a, h, i])
    }
}
/// n = 10, i = 6, c < h, d < h, e < g, e < i, f < g, f < i, g < j, h < i, h < j
fn select_2697([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_2695([b, e, f, h, a, g, i, j])
    } else {
        select_2696([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 11, i = 6, b < k, c < j, d < h, e < h, f < g, f < j, g < i, g < k, h < i, h < j, h < k
fn select_2698([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < c {
        select_1695([a, b, d, e, g, c, h, i, j, k])
    } else {
        select_2697([a, b, d, e, c, f, g, h, j, i])
    }
}
/// n = 10, i = 3, a < j, b < h, b < i, c < g, c < i, d < e, d < g, d < i, e < f, e < h, e < j
fn select_2699([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < c {
        select_2427([a, b, e, f, c, g, h, j])
    } else {
        select_989([a, b, c, d, g, e, f, i])
    }
}
/// n = 13, i = 6, b < m, c < k, c < l, d < h, e < h, f < g, f < i, g < j, g < l, h < i, h < j, h < l, i < k, i < m
fn select_2700([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if a < i {
        select_2698([b, c, a, d, e, f, g, h, j, i, l])
    } else {
        select_2699([b, c, g, h, i, a, j, k, l, m])
    }
}
/// n = 14, i = 6, b < n, c < k, c < m, d < j, e < i, e < k, f < h, g < h, h < i, h < j, h < k, j < m, j < n, k < l
fn select_2701([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if e < d {
        select_2694([a, b, c, f, g, e, d, h, i, j, k, l])
    } else {
        select_2700([a, b, c, f, g, d, e, h, j, i, m, k, n])
    }
}
/// n = 15, i = 6, b < m, c < k, c < l, d < j, d < l, e < i, f < h, f < m, g < h, g < i, h < j, h < l, i < k, i < m, l < n, m < o
fn select_2702([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_2689([a, d, c, b, f, e, g, i, k, h, m, o, j])
    } else {
        select_2701([a, b, c, e, d, f, g, h, j, i, l, n, k, m])
    }
}
/// n = 15, i = 6, b < m, c < k, c < l, d < h, d < l, e < j, f < i, f < m, g < i, g < j, i < l, j < k, j < m, l < n, m < o
fn select_2703([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_2676([a, b, c, d, f, e, g, h, j, k, i, m, o, l])
    } else {
        select_2702([a, b, c, d, e, f, g, i, j, h, k, l, m, n, o])
    }
}
/// n = 16, i = 7, b < m, c < k, c < o, d < i, e < l, e < m, f < g, f < h, g < l, h < j, h < o, i < k, i < m, l < o, m < n, o < p
fn select_2704([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if i < g {
        select_2646([a, c, b, e, f, d, h, i, j, k, g, m, l, n])
    } else {
        select_2703([a, b, c, h, d, e, g, j, l, i, k, o, m, p, n])
    }
}
/// n = 16, i = 7, b < m, c < h, d < g, d < o, e < l, e < m, f < i, f < j, h < m, i < l, j < k, j < o, l < o, m < n, o < p
fn select_2705([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if g < h {
        select_2554([a, b, c, e, d, f, i, j, g, k, l, h, o, m, p, n])
    } else {
        select_2704([a, b, d, c, e, f, i, j, h, k, g, l, m, n, o, p])
    }
}
/// n = 9, i = 5, c < i, d < g, e < g, f < g, g < h, g < i
fn select_2706([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < b {
        select_834([c, a, d, e, f, b, g, h, i])
    } else {
        select_834([c, b, d, e, f, a, g, h, i])
    }
}
/// n = 9, i = 5, c < g, d < g, e < f, e < h, f < i, g < h, g < i
fn select_2707([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_308([b, f, c, d, a, g, h, i])
    } else {
        select_2680([b, a, c, d, e, f, g, h])
    }
}
/// n = 10, i = 5, b < j, c < i, d < h, d < j, e < g, f < g, g < h, g < i, g < j
fn select_2708([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < c {
        select_1695([a, b, e, f, d, c, g, h, i, j])
    } else {
        select_2707([a, b, e, f, c, d, g, i, h])
    }
}
/// n = 11, i = 5, b < k, c < i, c < j, d < h, d < k, e < g, e < h, f < g, f < h, g < i, g < k, h < j
fn select_2709([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_2706([a, b, c, d, e, f, h, g, j])
    } else {
        select_2708([a, b, c, d, e, f, g, h, i, k])
    }
}
/// n = 11, i = 6, a < f, b < i, c < i, c < j, d < g, d < k, e < h, g < j, h < i, h < j, i < k
fn select_2710([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < g {
        select_1770([a, c, d, b, e, f, h, g, i, k, j])
    } else {
        select_1376([a, c, e, g, f, b, h, j, i])
    }
}
/// n = 11, i = 6, c < i, d < i, d < j, e < g, e < k, f < h, g < j, h < i, h < j, i < k
fn select_2711([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_2710([a, c, d, e, f, b, g, h, i, j, k])
    } else {
        select_2710([b, c, d, e, f, a, g, h, i, j, k])
    }
}
/// n = 12, i = 6, b < l, c < g, c < k, d < j, d < l, e < i, e < j, f < h, h < i, h < j, i < l, j < k
fn select_2712([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_2709([a, b, c, d, e, h, i, j, g, k, l])
    } else {
        select_2711([a, b, d, e, c, f, g, h, j, i, k])
    }
}
/// n = 11, i = 6, a < e, b < h, c < g, c < i, d < f, d < j, f < i, g < j, h < i, h < j, j < k
fn select_2713([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_718([a, d, g, b, e, h, f, j, k, i])
    } else {
        select_1018([a, c, f, b, e, h, g, i, j])
    }
}
/// n = 10, i = 4, a < e, b < h, b < i, c < g, c < i, d < f, d < i, f < g, f < h, i < j
fn select_2714([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < b {
        select_151([a, c, f, e, b, h, g, i])
    } else {
        select_2375([a, c, b, d, e, f, h, i, j, g])
    }
}
/// n = 12, i = 6, a < f, b < e, b < k, c < h, d < g, d < i, g < j, g < k, h < i, h < k, i < j, k < l
fn select_2715([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < i {
        select_2713([a, c, d, b, f, e, g, h, i, k, l])
    } else {
        select_2714([a, b, g, h, f, i, j, e, k, l])
    }
}
/// n = 10, i = 6, a < e, b < h, c < g, d < f, d < i, f < j, g < i, g < j, h < i, h < j
fn select_2716([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_1117([a, c, f, b, e, h, g, i, j])
    } else {
        select_1018([a, d, g, b, e, h, f, i, j])
    }
}
/// n = 8, i = 4, a < h, b < g, c < e, d < f, f < g, f < h
fn select_2717([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_292([c, d, a, e, b, f, h, g])
    } else {
        select_292([c, d, b, e, a, f, g, h])
    }
}
/// n = 11, i = 6, a < e, b < f, c < h, d < g, d < i, f < k, g < j, g < k, h < i, h < k, i < j
fn select_2718([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < i {
        select_2716([a, b, c, d, e, g, h, f, i, k])
    } else {
        select_2717([b, g, a, h, e, i, j, f])
    }
}
/// n = 12, i = 6, b < f, c < k, d < h, e < g, e < i, g < j, g < k, h < i, h < k, i < j, k < l
fn select_2719([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < a {
        select_2715([b, c, d, e, a, f, g, h, i, j, k, l])
    } else {
        select_2718([b, a, d, e, f, c, g, h, i, j, k])
    }
}
/// n = 10, i = 3, a < d, b < h, c < e, c < h, e < f, e < g, g < i, h < j
fn select_2720([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_1475([a, e, f, d, b, g, i, h])
    } else {
        select_1204([a, b, c, d, h, e, f, g, j])
    }
}
/// n = 12, i = 6, a < f, b < h, b < i, c < h, c < i, d < h, d < i, e < g, g < i, g < l, h < l, i < j, i < k
fn select_2721([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_2215([a, b, c, d, e, f, g, h, l])
    } else {
        select_48([a, h, g, i, j, k, l])
    }
}
/// n = 13, i = 6, a < f, b < j, c < h, c < j, d < h, d < j, e < g, g < i, g < j, h < i, i < k, j < l, j < m
fn select_2722([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < h {
        select_2721([a, b, c, d, e, f, g, h, j, l, m, i])
    } else {
        select_111([a, e, h, f, b, g, i, k, j])
    }
}
/// n = 13, i = 6, a < m, b < f, c < h, c < i, d < h, d < i, e < g, g < i, g < k, h < k, i < j, i < m, k < l
fn select_2723([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < a {
        select_2720([b, h, g, f, i, j, a, k, m, l])
    } else {
        select_2722([b, a, c, d, e, f, g, h, k, i, l, j, m])
    }
}
/// n = 8, i = 3, c < e, c < f, d < e, d < f, e < h, f < g
fn select_2724([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_172([b, f, e, a, h, g])
    } else {
        select_491([b, a, c, d, f, g, e])
    }
}
/// n = 9, i = 3, b < i, c < g, d < e, d < g, e < f, e < i, g < h
fn select_2725([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < c {
        select_59([a, b, e, f, c, g, i])
    } else {
        select_2724([a, b, c, d, e, g, h, f])
    }
}
/// n = 11, i = 5, b < e, c < g, d < f, d < h, f < j, g < h, g < j, h < i, j < k
fn select_2726([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_713([a, b, c, d, e, f, g, j, k, h])
    } else {
        select_2725([a, b, f, g, h, i, j, k, e])
    }
}
/// n = 13, i = 6, a < m, b < f, c < j, d < h, e < g, e < j, g < i, h < i, h < j, i < k, j < l, j < m
fn select_2727([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < g {
        select_2723([a, b, c, e, d, f, h, g, j, l, i, k, m])
    } else {
        select_2726([a, b, d, g, f, c, h, i, k, j, l])
    }
}
/// n = 13, i = 6, a < m, b < g, c < i, d < j, e < j, e < l, f < h, h < i, h < j, i < l, j < k, j < m
fn select_2728([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < e {
        select_2719([a, b, d, f, c, g, e, h, i, l, j, k])
    } else {
        select_2727([a, b, d, f, e, g, c, h, i, j, l, k, m])
    }
}
/// n = 14, i = 6, a < n, b < g, b < m, c < k, d < h, e < j, e < k, f < i, f < j, h < j, i < k, j < m, k < l, k < n
fn select_2729([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_2712([a, c, b, e, f, d, g, h, i, j, m, k])
    } else {
        select_2728([a, b, d, c, e, f, g, i, h, k, l, j, n])
    }
}
/// n = 8, i = 4, b < g, b < h, c < f, d < e, d < g, d < h, f < g, f < h
fn select_2730([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < f {
        select_740([a, b, c, d, e, f, g, h])
    } else {
        select_163([a, f, d, e, b, g, h])
    }
}
/// n = 8, i = 6, c < h, d < h, e < h, f < g
fn select_2731([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < a {
        select_388([a, b, d, e, f, g, h])
    } else {
        select_1097([b, c, d, e, f, g, h])
    }
}
/// n = 10, i = 6, c < i, c < j, d < h, e < h, f < g, g < i, g < j, h < i, h < j
fn select_2732([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_2730([b, c, f, h, a, g, i, j])
    } else {
        select_2731([b, c, a, d, e, f, g, h])
    }
}
/// n = 12, i = 6, b < l, c < h, d < i, d < k, e < h, e < i, f < g, g < i, g < k, h < k, i < j, i < l
fn select_2733([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_1646([a, b, c, d, e, g, i, j, h, l])
    } else {
        select_2732([a, b, d, c, e, f, g, h, i, k])
    }
}
/// n = 8, i = 4, b < g, b < h, c < e, c < h, d < f, f < g, f < h
fn select_2734([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < a {
        select_127([b, c, f, a, e, g, h])
    } else {
        select_12([b, c, a, d, e, f, g])
    }
}
/// n = 8, i = 5, b < h, c < g, d < g, d < h, e < f, f < h
fn select_2735([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < f {
        select_2680([a, c, b, e, d, g, f, h])
    } else {
        select_1652([a, c, d, f, b, g, h])
    }
}
/// n = 9, i = 5, c < h, c < i, d < g, e < f, e < h, f < i, g < h, g < i
fn select_2736([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_2734([b, c, f, d, a, g, h, i])
    } else {
        select_2735([b, c, a, e, d, g, f, h])
    }
}
/// n = 12, i = 6, b < l, c < g, d < i, e < i, e < k, f < h, g < k, h < i, h < k, i < j, i < l
fn select_2737([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < g {
        select_2733([a, b, c, e, d, f, h, g, i, j, k, l])
    } else {
        select_2736([a, b, e, f, g, d, h, k, i])
    }
}
/// n = 9, i = 3, a < i, b < g, c < g, c < h, d < e, e < f, e < h, e < i
fn select_2738([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < c {
        select_59([b, a, e, f, c, h, i])
    } else {
        select_1461([a, b, d, c, g, e, f, h])
    }
}
/// n = 9, i = 5, b < g, b < i, c < g, c < i, d < g, d < i, e < f, e < h, f < i, g < h
fn select_2739([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < a {
        select_1627([f, b, c, d, a, g, h, i])
    } else {
        select_782([a, b, c, d, e, f, g, h])
    }
}
/// n = 11, i = 5, b < k, c < g, c < k, d < g, d < k, e < f, e < h, f < i, f < j, f < k, g < h, g < i, g < j
fn select_2740([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < b {
        select_223([a, e, g, b, f, h, i, j])
    } else {
        select_2739([a, b, c, d, e, f, g, h, k])
    }
}
/// n = 13, i = 5, a < m, b < h, c < f, c < i, d < g, d < h, e < g, e < h, f < k, f < l, g < i, g < k, g < l, h < j, h < m
fn select_2741([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < f {
        select_2738([a, g, c, b, h, j, i, f, m])
    } else {
        select_2740([a, b, d, e, c, f, g, i, k, l, h])
    }
}
/// n = 13, i = 5, a < m, b < k, b < l, c < j, c < l, d < h, e < g, e < h, f < g, f < h, g < j, g < k, g < l, h < i, h < m
fn select_2742([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < c {
        select_2741([a, d, b, e, f, c, g, h, k, i, j, l, m])
    } else {
        select_2741([a, d, c, e, f, b, g, h, j, i, k, l, m])
    }
}
/// n = 14, i = 6, a < n, b < l, b < m, c < g, c < m, d < j, e < i, e < j, f < h, h < i, h < j, i < l, i < m, j < k, j < n
fn select_2743([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < i {
        select_2737([b, a, c, d, e, f, g, h, j, k, i, n])
    } else {
        select_2742([a, b, c, d, e, h, i, j, k, g, l, m, n])
    }
}
/// n = 9, i = 3, a < e, a < i, b < d, b < g, c < f, c < g, f < i, g < h
fn select_2744([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < f {
        select_109([a, c, b, e, d, g, h, f])
    } else {
        select_1231([a, b, c, e, f, g, h, d, i])
    }
}
/// n = 9, i = 4, a < g, b < f, b < i, c < e, c < h, d < f, d < g, d < i, e < i, g < h
fn select_2745([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < e {
        select_125([b, c, d, a, e, f, g, h])
    } else {
        select_470([b, d, e, a, f, g, i])
    }
}
/// n = 10, i = 4, b < g, b < j, c < e, c < h, d < f, d < g, d < j, e < j, f < h, h < i
fn select_2746([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_2744([b, f, c, a, g, e, h, i, j])
    } else {
        select_2745([a, b, c, d, e, g, f, h, j])
    }
}
/// n = 10, i = 4, b < e, b < h, b < j, c < f, c < h, d < f, d < h, f < g, f < j, h < i
fn select_2747([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_36([f, b, e, g, h, a, i, j])
    } else {
        select_2355([a, b, c, d, e, f, g, j, h])
    }
}
/// n = 10, i = 4, b < g, b < h, c < e, c < h, d < f, d < h, e < i, f < g, f < i, h < j
fn select_2748([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_2365([a, b, c, d, f, e, g, h, j])
    } else {
        select_1270([b, d, c, f, e, a, g, h, i, j])
    }
}
/// n = 11, i = 4, b < g, b < h, c < f, c < h, c < k, d < e, d < g, d < h, e < i, f < i, g < k, h < j
fn select_2749([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < e {
        select_2747([a, c, b, d, f, g, e, h, j, k])
    } else {
        select_2748([a, b, c, d, f, e, g, h, i, j])
    }
}
/// n = 12, i = 4, b < i, c < h, c < k, d < g, d < h, e < f, e < g, e < h, f < i, g < k, h < j, i < l
fn select_2750([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < c {
        select_2746([a, d, b, e, c, f, g, i, l, h])
    } else {
        select_2749([a, d, c, e, f, b, g, h, i, j, k])
    }
}
/// n = 10, i = 4, a < g, a < j, b < e, b < j, c < d, c < h, d < f, d < j, e < g, e < h, h < i
fn select_2751([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_58([f, a, b, e, h, j])
    } else {
        select_110([a, c, g, d, h, i, f])
    }
}
/// n = 10, i = 4, b < g, b < j, c < e, c < h, d < f, d < j, e < j, f < g, f < h, h < i
fn select_2752([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < e {
        select_727([b, a, c, d, f, e, g, h, i])
    } else {
        select_2751([b, d, c, e, f, a, g, h, i, j])
    }
}
/// n = 11, i = 4, b < i, c < h, d < g, d < i, e < f, e < i, f < g, f < h, h < j, i < k
fn select_2753([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < b {
        select_2752([a, d, c, e, b, f, g, h, j, i])
    } else {
        select_2748([a, d, b, e, c, f, g, i, h, k])
    }
}
/// n = 12, i = 6, a < f, b < j, b < l, c < h, d < h, e < g, e < j, g < i, g < l, h < i, h < j, h < l, j < k
fn select_2754([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < g {
        select_2677([a, c, d, b, e, f, g, h, j, k, i, l])
    } else {
        select_2691([a, c, d, g, f, b, h, i, j, l])
    }
}
/// n = 12, i = 6, c < j, c < l, d < h, e < h, f < g, f < j, g < i, g < l, h < i, h < j, h < l, j < k
fn select_2755([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_2754([a, c, d, e, f, b, g, h, i, j, k, l])
    } else {
        select_2754([b, c, d, e, f, a, g, h, i, j, k, l])
    }
}
/// n = 14, i = 6, b < k, c < j, c < m, d < g, d < j, e < h, f < h, g < m, h < i, h < j, h < m, i < k, j < l, k < n
fn select_2756([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if i < g {
        select_2753([a, c, b, d, h, i, g, k, j, n, l])
    } else {
        select_2755([a, b, c, e, f, d, g, h, i, j, l, m])
    }
}
/// n = 14, i = 6, b < l, c < j, c < k, d < g, d < k, e < h, f < h, h < i, h < j, h < k, i < l, k < m, l < n
fn select_2757([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < g {
        select_2750([a, b, d, c, h, i, j, k, l, m, g, n])
    } else {
        select_2756([a, b, c, d, e, f, g, h, i, k, l, m, j, n])
    }
}
/// n = 15, i = 6, a < o, b < k, c < j, c < m, d < g, d < m, e < i, e < k, f < h, f < i, h < k, i < j, i < m, k < l, k < o, m < n
fn select_2758([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < i {
        select_2743([a, c, d, b, e, f, g, h, i, k, l, j, m, o])
    } else {
        select_2757([a, b, c, d, e, f, g, i, h, j, m, k, n, l])
    }
}
/// n = 15, i = 6, a < o, b < k, c < h, c < m, d < g, d < m, e < j, e < k, f < i, f < j, i < k, j < m, k < l, k < o, m < n
fn select_2759([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < j {
        select_2729([a, d, b, c, e, f, g, h, i, j, k, l, m, o])
    } else {
        select_2758([a, b, c, d, e, f, g, i, j, h, k, l, m, n, o])
    }
}
/// n = 9, i = 3, a < e, a < i, b < c, b < d, d < f, d < i, e < g, e < h
fn select_2760([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_105([b, e, g, h, c, d])
    } else {
        select_409([c, a, d, f, e, i])
    }
}
/// n = 13, i = 7, a < f, a < j, b < g, b < m, c < i, d < i, e < i, f < l, f < m, g < h, g < k, i < j, i < k, i < l, i < m
fn select_2761([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < i {
        select_1138([h, a, c, d, e, f, i, k, j])
    } else {
        select_2760([b, a, j, f, g, l, h, k, m])
    }
}
/// n = 13, i = 7, a < f, a < m, b < g, b < k, c < i, d < i, e < i, f < h, g < l, g < m, i < j, i < k, i < l, i < m
fn select_2762([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < j {
        select_2761([b, a, c, d, e, g, f, h, i, k, j, l, m])
    } else {
        select_2060([j, a, b, k, g, l, f, m, h])
    }
}
/// n = 10, i = 4, a < g, a < j, b < e, b < h, c < d, c < g, c < h, e < f, h < i, h < j
fn select_2763([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_107([d, a, b, g, h, i, e, f, j])
    } else {
        select_959([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 14, i = 7, a < g, a < k, b < f, b < l, c < i, d < i, e < i, f < h, g < n, i < j, i < k, i < l, i < n, l < m
fn select_2764([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < l {
        select_2762([b, a, c, d, e, f, g, h, i, j, k, n, l])
    } else {
        select_2763([a, b, i, j, f, h, k, l, m, g])
    }
}
/// n = 14, i = 7, a < l, b < k, c < g, c < m, d < i, e < i, f < i, g < h, i < j, i < k, i < l, i < m, m < n
fn select_2765([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_2764([a, c, d, e, f, g, b, h, i, j, l, m, n, k])
    } else {
        select_2764([b, c, d, e, f, g, a, h, i, j, k, m, n, l])
    }
}
/// n = 10, i = 6, a < f, b < i, b < j, c < h, d < h, e < g, g < i, g < j, h < i, h < j
fn select_2766([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_844([a, b, h, e, g, f, i, j])
    } else {
        select_1097([b, c, d, f, e, g, h])
    }
}
/// n = 12, i = 6, a < f, b < h, b < k, c < i, c < l, d < h, d < i, e < g, g < i, g < l, h < l, i < j, i < k
fn select_2767([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_915([a, b, c, d, g, f, i, j, h, k])
    } else {
        select_2766([a, c, b, d, e, f, g, h, i, l])
    }
}
/// n = 12, i = 6, a < f, b < i, c < g, c < l, d < i, d < k, e < h, g < k, h < i, h < k, i < j, i < l
fn select_2768([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if b < g {
        select_2767([a, c, d, b, e, f, h, g, i, j, l, k])
    } else {
        select_1376([a, d, e, g, f, b, h, k, i])
    }
}
/// n = 9, i = 5, a < i, b < h, c < f, d < g, d < i, e < g, e < i, f < i, g < h
fn select_2769([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < f {
        select_2680([a, c, d, e, b, f, g, h])
    } else {
        select_856([a, d, e, f, b, g, i])
    }
}
/// n = 11, i = 5, a < f, b < j, b < k, c < h, d < g, d < h, e < g, e < h, g < j, h < i, h < k
fn select_2770([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_2738([a, g, b, c, h, i, j, k, f])
    } else {
        select_2769([c, b, a, d, e, f, g, j, h])
    }
}
/// n = 12, i = 6, a < g, b < f, b < l, c < j, d < i, d < j, e < h, h < i, h < j, j < k, j < l
fn select_2771([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < i {
        select_2768([a, c, b, d, e, g, f, h, j, k, i, l])
    } else {
        select_2770([a, b, c, d, h, g, i, j, k, f, l])
    }
}
/// n = 9, i = 4, b < i, c < g, c < h, c < i, d < f, d < i, e < f, e < i, f < g, f < h
fn select_2772([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_391([b, c, a, d, e, f, i])
    } else {
        select_37([b, c, f, a, g, h, i])
    }
}
/// n = 11, i = 5, a < h, b < e, b < f, c < g, c < h, d < g, d < h, f < j, f < k, g < j, g < k, h < i
fn select_2773([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_106([g, b, a, e, h, i, f])
    } else {
        select_2772([e, a, f, c, d, g, j, k, h])
    }
}
/// n = 8, i = 3, a < f, b < f, b < g, b < h, c < d, d < e, d < g, d < h
fn select_2774([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < a {
        select_28([e, b, g, h, a])
    } else {
        select_2437([a, b, c, d, f, g, h, e])
    }
}
/// n = 12, i = 5, a < h, b < e, b < i, b < l, c < g, c < h, d < g, d < h, e < f, e < k, g < i, g < k, h < j, h < l
fn select_2775([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_2774([g, b, a, h, j, i, e, l])
    } else {
        select_2355([a, e, c, d, f, g, i, k, h])
    }
}
/// n = 12, i = 5, a < f, a < l, b < j, b < k, b < l, c < h, d < g, d < h, e < g, e < h, g < j, g < l, h < i, h < k
fn select_2776([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_2773([c, a, d, e, f, b, g, h, i, j, l])
    } else {
        select_2775([c, b, d, e, a, f, g, h, j, i, l, k])
    }
}
/// n = 13, i = 6, a < g, a < m, b < j, c < f, c < l, c < m, d < i, d < j, e < h, h < i, h < j, i < m, j < k, j < l
fn select_2777([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if i < f {
        select_2776([a, c, b, d, h, g, i, j, k, f, l, m])
    } else {
        select_2768([a, b, c, d, e, g, f, h, j, k, i, l])
    }
}
/// n = 14, i = 6, a < n, b < l, b < m, c < g, c < m, d < j, e < i, e < j, f < h, h < i, h < j, i < m, j < k, j < l, j < n
fn select_2778([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < b {
        select_2771([c, a, d, e, f, b, g, h, i, j, k, n])
    } else {
        select_2777([c, d, b, e, f, a, g, h, i, j, k, l, m])
    }
}
/// n = 15, i = 7, a < n, b < m, b < o, c < j, d < g, d < i, e < j, e < l, f < i, f < j, g < h, g < o, i < l, j < k, j < m, j < n, l < o
fn select_2779([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if j < i {
        select_2765([a, b, d, c, e, f, g, h, j, k, m, n, i, l])
    } else {
        select_2778([a, b, g, c, e, f, h, i, l, j, k, m, o, n])
    }
}
/// n = 11, i = 5, a < f, a < k, b < h, b < j, c < g, c < h, d < g, d < h, e < g, e < h, g < j, g < k, h < i
fn select_2780([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < f {
        select_1275([a, g, b, h, i, j, f, k])
    } else {
        select_404([b, a, c, d, e, f, h, i, g])
    }
}
/// n = 10, i = 5, a < f, b < h, b < j, c < g, c < h, d < g, d < h, e < g, e < h, g < j, h < i
fn select_2781([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_404([b, a, c, d, e, f, h, i, g])
    } else {
        select_461([a, g, b, h, i, j, f])
    }
}
/// n = 11, i = 5, b < k, c < h, c < j, d < g, d < h, e < g, e < h, f < g, f < h, g < j, g < k, h < i
fn select_2782([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < a {
        select_2780([b, c, d, e, f, a, g, h, i, j, k])
    } else {
        select_2781([a, c, d, e, f, b, g, h, i, j])
    }
}
/// n = 13, i = 6, a < m, b < i, b < l, c < g, c < h, d < h, d < i, e < h, e < i, f < h, f < i, g < k, h < k, h < l, i < j, i < m
fn select_2783([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < i {
        select_2782([a, g, b, d, e, f, h, i, j, l, k])
    } else {
        select_172([j, a, c, g, h, m])
    }
}
/// n = 8, i = 4, b < h, c < f, d < g, e < f, e < g, g < h
fn select_2784([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_742([a, b, c, d, e, g, f, h])
    } else {
        select_526([a, b, d, c, e, f, g])
    }
}
/// n = 9, i = 4, b < g, c < h, d < f, d < i, e < f, e < g, e < h, h < i
fn select_2785([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_2001([a, b, d, c, e, f, h, i, g])
    } else {
        select_2784([a, c, d, b, e, f, g, h])
    }
}
/// n = 14, i = 7, a < n, b < k, b < l, c < h, d < g, d < i, e < i, e < l, f < i, f < l, g < j, h < l, i < j, i < k, l < m, l < n
fn select_2786([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < i {
        select_2783([a, b, d, e, f, h, g, i, l, m, j, k, n])
    } else {
        select_2785([a, g, c, b, i, k, j, h, l])
    }
}
/// n = 10, i = 6, a < j, b < g, c < h, d < h, e < h, f < h, h < i, h < j
fn select_2787([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_834([a, b, d, e, f, g, h, i, j])
    } else {
        select_834([a, b, c, e, f, g, h, i, j])
    }
}
/// n = 12, i = 6, a < l, b < i, c < g, d < i, d < k, e < i, e < k, f < h, f < i, g < h, h < k, i < j, i < l
fn select_2788([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < h {
        select_2787([a, c, b, d, e, f, g, i, j, l])
    } else {
        select_1493([a, b, d, e, f, g, h, k])
    }
}
/// n = 9, i = 5, b < g, c < h, d < g, d < h, e < f, e < h, f < i, h < i
fn select_2789([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < g {
        select_742([a, b, f, c, d, h, i, g])
    } else {
        select_926([a, c, b, d, e, f, g, h])
    }
}
/// n = 9, i = 5, b < i, c < g, d < h, e < h, f < g, f < h, h < i
fn select_2790([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < b {
        select_2789([a, c, e, f, d, b, g, h, i])
    } else {
        select_374([a, c, e, f, b, d, g, h, i])
    }
}
/// n = 14, i = 7, a < n, b < k, b < m, c < g, d < h, e < j, e < k, f < j, f < k, g < k, h < i, i < j, j < m, k < l, k < n
fn select_2791([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < i {
        select_2788([a, b, d, e, f, g, h, i, k, l, j, n])
    } else {
        select_2790([a, b, c, e, f, i, g, j, m])
    }
}
/// n = 14, i = 7, a < n, b < k, b < l, c < h, d < g, d < j, e < j, e < l, f < j, f < l, g < i, h < l, j < k, l < m, l < n
fn select_2792([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < i {
        select_2786([a, b, c, d, e, f, g, h, j, i, k, l, m, n])
    } else {
        select_2791([a, b, c, d, e, f, h, g, i, j, l, m, k, n])
    }
}
/// n = 9, i = 4, c < h, c < i, d < g, d < i, e < f, f < g, f < h, f < i
fn select_2793([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_296([a, b, e, c, d, f, h, g, i])
    } else {
        select_296([a, b, e, d, c, f, g, h, i])
    }
}
/// n = 11, i = 5, b < i, b < k, c < g, c < k, d < h, d < j, e < f, e < h, f < j, g < j, h < i, h < k
fn select_2794([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_2736([a, b, d, c, e, f, g, h, j])
    } else {
        select_2793([a, f, b, c, d, h, g, i, k])
    }
}
/// n = 10, i = 5, a < e, b < g, c < g, c < j, d < f, d < h, e < j, f < i, f < j, g < h, g < i
fn select_2795([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_291([a, g, d, f, h, e, i])
    } else {
        select_240([b, c, e, d, f, g, j])
    }
}
/// n = 8, i = 4, a < d, b < f, b < h, c < e, c < g, e < h, f < g
fn select_2796([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < f {
        select_135([a, b, e, d, h, f, g])
    } else {
        select_135([a, c, f, d, g, e, h])
    }
}
/// n = 10, i = 5, a < i, a < j, b < e, c < g, d < f, d < i, e < j, f < h, f < j, g < h, g < i
fn select_2797([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_2795([b, c, a, d, e, f, g, i, h, j])
    } else {
        select_2796([b, d, g, e, a, f, h, i])
    }
}
/// n = 9, i = 5, a < i, b < e, c < g, d < f, d < i, f < h, g < h, g < i
fn select_2798([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < f {
        select_1018([b, a, d, c, e, g, f, i, h])
    } else {
        select_292([b, c, f, e, a, g, h, i])
    }
}
/// n = 11, i = 6, a < j, a < k, b < e, c < f, d < g, e < k, f < h, f < j, g < i, g < j, i < k
fn select_2799([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < h {
        select_2797([a, b, c, g, e, i, f, h, j, k])
    } else {
        select_2798([a, b, d, f, e, h, g, i, j])
    }
}
/// n = 8, i = 4, a < f, b < f, c < f, c < g, d < e, e < g, g < h
fn select_2800([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < g {
        select_240([b, a, c, d, e, f, g])
    } else {
        select_19([b, g, a, h])
    }
}
/// n = 11, i = 6, a < g, b < g, c < h, d < h, d < k, e < f, f < i, f < k, g < k, h < i, i < j
fn select_2801([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < g {
        select_2800([a, b, h, e, f, g, i, j])
    } else {
        select_240([c, d, g, e, f, h, k])
    }
}
/// n = 9, i = 5, a < g, b < g, c < f, d < e, d < h, e < i, f < h, h < i
fn select_2802([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if a < h {
        select_1699([b, a, c, d, e, f, g, h])
    } else {
        select_82([b, e, h, a, i])
    }
}
/// n = 11, i = 6, a < j, a < k, b < h, c < h, d < f, e < g, f < i, g < i, g < k, h < k, i < j
fn select_2803([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < f {
        select_2801([b, c, d, a, e, g, h, f, i, j, k])
    } else {
        select_2802([b, c, e, f, a, g, h, i, j])
    }
}
/// n = 10, i = 5, a < i, a < j, b < h, c < g, d < g, e < f, e < j, f < h, g < j, h < i
fn select_2804([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_926([b, a, c, d, e, f, g, j])
    } else {
        select_526([c, d, a, b, f, h, i])
    }
}
/// n = 11, i = 6, a < j, a < k, b < i, c < i, d < f, e < g, f < j, g < h, g < k, h < j, i < k
fn select_2805([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_2803([a, b, c, d, e, f, g, i, h, j, k])
    } else {
        select_2804([a, d, b, c, g, h, i, f, j, k])
    }
}
/// n = 11, i = 6, b < j, b < k, c < f, d < g, e < h, f < k, g < j, h < i, h < j, i < k
fn select_2806([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_2799([b, c, d, e, f, g, h, a, i, j, k])
    } else {
        select_2805([b, a, d, c, e, f, h, i, g, k, j])
    }
}
/// n = 12, i = 6, b < f, b < l, c < j, c < k, d < g, d < l, e < h, g < k, h < i, h < j, i < k, j < l
fn select_2807([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < f {
        select_2794([a, b, d, c, h, i, g, j, f, k, l])
    } else {
        select_2806([a, c, d, b, e, g, f, h, i, j, k])
    }
}
/// n = 15, i = 7, a < o, b < l, c < k, c < l, d < h, d < n, e < g, e < j, f < j, f < l, g < i, g < n, h < l, j < k, k < n, l < m, l < o
fn select_2808([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if b < j {
        select_2792([a, c, d, e, b, f, g, h, i, j, k, l, m, o])
    } else {
        select_2807([a, g, c, d, f, i, h, j, b, k, l, n])
    }
}
/// n = 15, i = 7, a < n, b < g, b < o, c < k, d < k, d < m, e < h, e < j, f < j, f < k, h < i, h < o, j < m, k < l, k < n, m < o
fn select_2809([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if k < g {
        select_2779([a, b, c, e, d, f, h, i, j, k, l, m, g, n, o])
    } else {
        select_2808([a, c, d, b, e, f, h, g, i, j, m, k, l, o, n])
    }
}
/// n = 16, i = 7, a < n, b < l, c < l, d < g, d < o, e < k, e < l, f < h, f < i, h < k, i < j, i < o, k < o, l < m, l < n, o < p
fn select_2810([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < b {
        select_2759([a, c, d, i, e, h, j, g, b, k, l, m, o, p, n])
    } else {
        select_2809([a, d, c, e, f, b, g, i, j, h, l, m, k, n, o])
    }
}
/// n = 7, i = 4, a < g, b < g, c < g, d < f, e < f, e < g
fn select_2811([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_158([c, d, e, a, b, f, g])
    } else {
        select_158([c, d, e, b, a, f, g])
    }
}
/// n = 10, i = 4, b < f, c < g, d < g, e < f, e < g, g < h, g < i, g < j
fn select_2812([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_2811([a, c, d, b, e, f, g])
    } else {
        select_582([b, c, d, f, g, a, h, i, j])
    }
}
/// n = 10, i = 4, a < f, b < f, c < g, d < g, e < f, e < g, g < h, g < i, g < j
fn select_2813([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < a {
        select_576([b, g, a, h, i, j])
    } else {
        select_391([c, d, b, a, e, f, g])
    }
}
/// n = 11, i = 4, a < e, b < g, c < g, c < k, d < f, d < g, d < k, e < f, e < k, g < h, g < i, g < j
fn select_2814([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_33([a, h, i, j])
    } else {
        select_1071([b, c, e, d, f, k, g])
    }
}
/// n = 12, i = 5, a < g, b < h, c < h, c < l, d < g, d < h, e < f, e < h, e < l, g < l, h < i, h < j, h < k
fn select_2815([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < g {
        select_2813([a, f, b, c, d, g, h, i, j, k])
    } else {
        select_2814([a, b, c, e, g, f, h, i, j, k, l])
    }
}
/// n = 13, i = 5, a < g, b < i, c < h, c < i, c < m, d < g, d < i, e < f, e < i, e < m, f < h, g < m, i < j, i < k, i < l
fn select_2816([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_2812([a, f, b, d, c, h, i, j, k, l])
    } else {
        select_2815([a, b, c, d, e, f, g, i, j, k, l, m])
    }
}
/// n = 10, i = 5, a < g, b < g, b < i, c < e, c < h, d < f, e < i, f < h, f < i, f < j, g < j
fn select_2817([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_487([a, c, b, d, e, f, g, j, i])
    } else {
        select_2164([a, b, e, f, h, i])
    }
}
/// n = 10, i = 6, a < h, b < h, c < g, d < g, d < j, e < f, f < i, f < j, g < i, h < j
fn select_2818([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_1511([a, b, g, e, f, i, h])
    } else {
        select_240([c, d, h, e, f, g, j])
    }
}
/// n = 14, i = 6, a < h, b < h, b < j, c < g, c < j, d < g, d < j, d < n, e < f, f < i, f < j, f < n, g < i, h < n, j < k, j < l, j < m
fn select_2819([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if j < h {
        select_1130([a, g, b, f, i, j, k, l, m])
    } else {
        select_2818([a, b, c, d, e, f, g, h, i, n])
    }
}
/// n = 14, i = 6, a < h, b < j, c < h, c < j, d < g, e < f, e < j, e < n, f < i, g < i, g < j, g < n, h < n, j < k, j < l, j < m
fn select_2820([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < b {
        select_2817([a, c, f, d, b, g, h, i, j, n])
    } else {
        select_2819([a, c, b, e, d, g, f, h, i, j, k, l, m, n])
    }
}
/// n = 14, i = 6, a < i, b < j, c < i, c < j, d < f, d < j, d < n, e < g, g < h, g < j, g < n, i < n, j < k, j < l, j < m
fn select_2821([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < f {
        select_2816([a, b, d, c, g, h, i, f, j, k, l, m, n])
    } else {
        select_2820([a, b, c, e, d, f, g, i, h, j, k, l, m, n])
    }
}
/// n = 8, i = 4, a < g, b < f, c < d, c < e, e < h, f < g, f < h
fn select_2822([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < e {
        select_1926([a, b, c, d, f, e, g])
    } else {
        select_53([d, b, e, a, f, h])
    }
}
/// n = 9, i = 5, a < i, b < d, c < e, d < h, d < i, e < f, e < g
fn select_2823([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < f {
        select_846([a, c, d, h, e, g, i, f])
    } else {
        select_2822([a, b, e, g, f, d, i, h])
    }
}
/// n = 8, i = 4, a < g, a < h, b < e, c < d, c < g, c < h, e < f, e < h
fn select_2824([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_53([d, b, a, g, e, h])
    } else {
        select_159([a, c, e, f, d, h])
    }
}
/// n = 10, i = 4, a < g, a < h, b < e, b < h, c < d, c < g, c < h, e < f, h < i, h < j
fn select_2825([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < e {
        select_811([d, g, b, h, e, i, j])
    } else {
        select_2824([a, b, c, d, e, f, g, h])
    }
}
/// n = 12, i = 5, a < i, a < j, b < e, c < d, c < j, d < g, d < h, e < f, e < i, e < j, j < k, j < l
fn select_2826([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < j {
        select_2823([a, b, c, e, d, g, h, f, i])
    } else {
        select_2825([a, c, e, f, d, h, i, j, k, l])
    }
}
/// n = 9, i = 4, a < h, a < i, b < d, b < i, c < e, d < g, e < f, e < h, e < i
fn select_2827([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < d {
        select_22([f, b, a, h, i, d])
    } else {
        select_1062([a, c, b, d, e, g, h, f, i])
    }
}
/// n = 9, i = 4, a < c, b < d, c < g, c < i, d < e, d < f, d < i, g < h
fn select_2828([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_88([e, f, a, c, i])
    } else {
        select_474([b, c, g, d, f, h, i])
    }
}
/// n = 11, i = 4, a < d, b < c, b < i, c < g, d < e, d < f, d < i, g < h, i < j, i < k
fn select_2829([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if i < c {
        select_811([e, f, b, i, c, j, k])
    } else {
        select_2828([b, a, c, d, e, f, g, h, i])
    }
}
/// n = 11, i = 4, a < g, b < g, b < i, c < d, d < e, d < f, d < i, g < h, i < j, i < k
fn select_2830([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < b {
        select_2828([a, c, b, d, e, f, g, h, i])
    } else {
        select_2829([c, b, a, d, e, f, g, h, i, j, k])
    }
}
/// n = 12, i = 5, a < g, b < h, b < j, c < g, c < j, d < e, e < f, e < h, e < j, g < i, j < k, j < l
fn select_2831([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if a < b {
        select_2827([b, c, d, g, e, f, i, h, j])
    } else {
        select_2830([a, c, d, e, f, h, g, i, j, k, l])
    }
}
/// n = 12, i = 5, b < i, b < j, c < e, c < j, d < f, e < h, f < g, f < i, f < j, j < k, j < l
fn select_2832([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < a {
        select_2826([b, d, c, e, f, g, a, h, i, j, k, l])
    } else {
        select_2831([a, b, c, d, f, g, e, i, h, j, k, l])
    }
}
/// n = 15, i = 7, a < k, b < h, b < m, c < j, c < k, d < g, d < h, e < f, e < k, e < n, g < j, h < i, h < n, h < o, j < n, k < l, k < m, k < o
fn select_2833([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < k {
        select_2821([g, a, c, e, b, f, h, i, j, k, l, m, o, n])
    } else {
        select_2832([f, b, d, a, g, k, l, j, m, h, i, o])
    }
}
/// n = 11, i = 4, b < f, b < i, c < e, c < i, d < f, d < h, d < i, e < h, f < g, i < j, i < k
fn select_2834([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < e {
        select_766([a, f, c, g, i, j, k, e])
    } else {
        select_2036([a, e, b, d, h, i, j, k, f])
    }
}
/// n = 11, i = 4, b < h, b < i, c < e, c < i, d < f, d < i, e < h, f < g, i < j, i < k
fn select_2835([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_1038([a, c, f, g, e, b, i])
    } else {
        select_2834([a, d, c, b, e, f, g, h, i, j, k])
    }
}
/// n = 11, i = 4, b < f, b < i, c < e, c < i, d < g, d < i, g < h, i < j, i < k
fn select_2836([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_2835([a, b, c, d, e, g, h, f, i, j, k])
    } else {
        select_2835([a, c, b, d, f, g, h, e, i, j, k])
    }
}
/// n = 10, i = 3, a < d, b < g, b < h, b < j, c < e, c < g, c < h, e < f, e < j, h < i
fn select_2837([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < b {
        select_1816([c, a, d, e, b, f, g, h])
    } else {
        select_254([c, b, a, e, f, d, g, h, i, j])
    }
}
/// n = 10, i = 4, a < d, b < f, c < e, c < g, c < h, e < j, f < g, f < h, f < j, h < i
fn select_2838([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < d {
        select_59([a, e, f, g, h, i, j])
    } else {
        select_292([a, b, c, d, e, f, g, j])
    }
}
/// n = 11, i = 4, a < e, b < d, b < i, b < j, c < f, f < g, f < i, f < j, g < h, j < k
fn select_2839([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < d {
        select_2837([a, b, f, e, g, h, i, j, k, d])
    } else {
        select_2838([a, c, b, e, d, f, i, j, k, g])
    }
}
/// n = 12, i = 5, a < d, b < f, c < e, c < i, e < g, e < j, e < k, f < h, f < j, f < k, h < i, k < l
fn select_2840([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < h {
        select_2838([a, b, e, d, g, f, j, k, l, h])
    } else {
        select_258([a, c, f, d, h, e, j, i, k, l, g])
    }
}
/// n = 12, i = 5, a < d, b < f, c < e, c < i, e < h, e < j, e < k, f < g, f < i, f < j, f < k, k < l
fn select_2841([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if i < g {
        select_2839([a, e, b, h, d, f, i, g, j, k, l])
    } else {
        select_2840([a, b, c, d, e, f, h, g, i, j, k, l])
    }
}
/// n = 13, i = 6, a < i, a < k, b < d, b < i, c < e, d < g, e < f, e < h, e < k, e < m, h < j, i < l, i < m
fn select_2842([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < i {
        select_2836([f, a, h, b, j, k, d, g, i, l, m])
    } else {
        select_2841([d, c, a, g, i, e, f, l, k, m, h, j])
    }
}
/// n = 15, i = 7, a < f, a < o, b < i, c < h, c < m, d < i, d < l, e < g, e < h, g < l, h < j, h < n, h < o, i < k, i < m, i < n, l < o
fn select_2843([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if a < i {
        select_2833([b, c, d, e, a, f, g, h, j, l, i, k, m, o, n])
    } else {
        select_2842([c, e, b, g, i, k, l, a, h, f, m, j, n])
    }
}
/// n = 10, i = 5, a < f, b < e, b < h, c < d, c < g, c < j, d < i, e < g, e < j, f < h, f < i, f < j
fn select_2844([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < h {
        select_1062([d, a, b, e, f, g, i, j, h])
    } else {
        select_470([e, c, f, h, g, d, j])
    }
}
/// n = 8, i = 4, a < h, b < f, c < f, c < g, d < e, e < g, e < h
fn select_2845([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < a {
        select_848([c, d, b, a, e, f, g, h])
    } else {
        select_452([c, d, a, b, e, h, g, f])
    }
}
/// n = 10, i = 5, a < f, a < h, b < g, c < d, c < e, c < j, e < i, f < j, g < h, g < i, g < j
fn select_2846([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < d {
        select_2844([b, a, c, e, f, g, d, h, i, j])
    } else {
        select_2845([e, d, a, b, g, f, h, i])
    }
}
/// n = 8, i = 3, b < g, c < f, c < h, d < e, d < f, d < g, e < h
fn select_2847([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_724([a, b, d, e, c, g, f])
    } else {
        select_1639([a, c, d, e, b, f, g, h])
    }
}
/// n = 10, i = 5, a < e, a < i, b < g, c < d, c < f, c < h, f < j, g < h, g < i, g < j
fn select_2848([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < h {
        select_2846([a, b, c, d, f, e, g, i, j, h])
    } else {
        select_2847([d, f, a, g, h, i, j, e])
    }
}
/// n = 11, i = 5, a < e, b < g, c < d, c < f, c < i, f < j, f < k, g < h, g < i, g < j, h < k
fn select_2849([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if a < h {
        select_2848([a, b, c, d, e, f, g, i, h, j])
    } else {
        select_1453([h, c, d, i, a, f, j, e, k])
    }
}
/// n = 11, i = 5, a < d, b < f, c < e, e < g, e < i, e < j, f < h, f < i, f < j, j < k
fn select_2850([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < h {
        select_2838([a, b, e, d, g, f, i, j, k, h])
    } else {
        select_2838([a, c, f, d, h, e, i, j, k, g])
    }
}
/// n = 11, i = 4, a < f, b < g, b < h, c < g, c < h, d < e, d < g, d < h, g < k, h < i, h < j
fn select_2851([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < h {
        select_2165([a, b, c, d, f, e, g, k, h])
    } else {
        select_1628([a, b, c, d, f, g, h, e, i, j, k])
    }
}
/// n = 13, i = 6, a < e, b < h, b < l, c < h, c < i, c < l, d < f, f < g, f < i, f < k, g < l, h < j, h < k, l < m
fn select_2852([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if h < g {
        select_2850([a, b, d, e, f, h, i, j, k, g, l])
    } else {
        select_2851([a, b, g, c, i, e, l, h, j, k, m])
    }
}
/// n = 13, i = 6, a < e, b < j, b < l, c < f, c < l, d < g, f < h, f < k, g < i, g < j, g < k, i < l, l < m
fn select_2853([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < b {
        select_2849([a, d, f, h, e, b, g, i, k, j, l])
    } else {
        select_2852([a, c, b, d, e, g, i, f, j, h, k, l, m])
    }
}
/// n = 10, i = 4, a < d, b < e, b < h, c < f, e < j, f < g, f < h, f < j, h < i
fn select_2854([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < e {
        select_109([a, b, f, d, g, h, i, e])
    } else {
        select_1598([a, c, b, d, e, f, h, i, g, j])
    }
}
/// n = 10, i = 4, a < d, b < e, b < h, c < f, c < h, e < j, f < g, f < j, h < i
fn select_2855([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < f {
        select_571([e, a, c, d, h, i, f, g])
    } else {
        select_2854([a, b, c, d, e, f, g, h, i, j])
    }
}
/// n = 8, i = 4, a < g, b < e, b < h, c < d, c < g, e < f, g < h
fn select_2856([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if e < d {
        select_406([a, e, c, f, d, g, h])
    } else {
        select_642([d, a, b, g, e, f, h])
    }
}
/// n = 10, i = 4, a < d, b < f, c < e, c < h, e < g, e < j, f < h, f < j, h < i
fn select_2857([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < f {
        select_2856([b, c, a, d, e, g, f, h])
    } else {
        select_478([c, f, a, e, g, d, h, i, j])
    }
}
/// n = 10, i = 4, a < j, b < e, c < h, c < j, d < f, d < h, f < g, f < j, h < i
fn select_2858([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < a {
        select_2855([b, c, d, e, a, f, g, h, i, j])
    } else {
        select_2857([b, a, d, e, f, c, g, h, i, j])
    }
}
/// n = 8, i = 3, a < h, b < g, c < e, d < f, f < g, f < h
fn select_2859([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_2101([c, d, a, b, e, f, h])
    } else {
        select_2101([c, d, b, a, e, f, g])
    }
}
/// n = 10, i = 4, a < e, b < g, b < i, b < j, c < f, c < g, d < f, d < g, f < i, f < j, g < h
fn select_2860([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < e {
        select_641([a, f, b, g, h, i, j, e])
    } else {
        select_1949([b, a, c, d, e, g, h, f])
    }
}
/// n = 10, i = 4, a < e, b < g, b < h, c < f, c < h, c < j, d < f, d < g, d < h, g < j, h < i
fn select_2861([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < g {
        select_1372([a, b, c, d, e, f, h, i, g])
    } else {
        select_2860([a, c, b, d, e, g, h, i, f, j])
    }
}
/// n = 11, i = 4, a < e, b < i, c < g, c < h, c < i, d < f, d < g, d < i, f < h, h < j, i < k
fn select_2862([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < b {
        select_971([a, f, c, g, e, b, h, j, i])
    } else {
        select_2861([a, b, c, d, e, g, f, i, k, h])
    }
}
/// n = 11, i = 4, a < f, b < i, c < h, c < i, d < e, d < g, d < i, g < h, h < j, i < k
fn select_2863([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < c {
        select_2859([b, g, a, e, f, c, h, i])
    } else {
        select_2862([a, b, c, d, f, g, e, h, i, j, k])
    }
}
/// n = 11, i = 4, a < g, b < j, c < h, d < f, d < j, e < h, e < j, h < i, j < k
fn select_2864([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < d {
        select_2858([b, a, e, c, g, d, f, h, i, j])
    } else {
        select_2863([a, b, e, d, f, g, c, h, j, i, k])
    }
}
/// n = 13, i = 6, a < e, b < k, b < l, c < f, c < l, d < g, f < i, g < h, g < j, g < k, j < l, l < m
fn select_2865([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < h {
        select_2853([a, b, c, d, e, f, g, i, j, k, h, l, m])
    } else {
        select_2864([a, j, h, b, c, k, e, f, i, l, m])
    }
}
/// n = 15, i = 7, a < f, a < o, b < i, c < h, c < n, d < g, d < h, e < i, e < m, g < m, h < l, h < o, i < j, i < k, i < n, m < o
fn select_2866([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < j {
        select_2843([a, b, c, e, d, f, g, h, i, l, k, m, n, j, o])
    } else {
        select_2865([a, c, d, b, f, g, i, k, m, j, n, h, l])
    }
}
/// n = 11, i = 5, a < e, a < k, b < g, c < g, d < f, d < h, d < k, f < i, f < j, g < h, g < i, g < j, g < k
fn select_2867([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < a {
        select_36([a, d, h, e, f, i, j, k])
    } else {
        select_740([e, b, c, d, f, g, h, k])
    }
}
/// n = 8, i = 3, a < f, a < h, b < f, b < g, c < d, d < e, d < g, d < h
fn select_2868([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_474([c, a, b, d, e, f, h])
    } else {
        select_474([c, b, a, d, e, f, g])
    }
}
/// n = 9, i = 4, a < d, a < i, b < f, c < e, c < g, c < i, e < h, f < g, f < h, f < i
fn select_2869([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < d {
        select_2868([a, e, b, f, g, d, h, i])
    } else {
        select_476([c, b, a, d, f, g, e, i])
    }
}
/// n = 12, i = 6, a < e, a < l, b < h, c < h, d < f, f < g, f < j, f < l, g < k, h < i, h < j, h < k, h < l
fn select_2870([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < i {
        select_2867([a, b, c, f, e, g, h, j, i, k, l])
    } else {
        select_2869([a, d, h, e, i, f, j, g, l])
    }
}
/// n = 11, i = 6, a < e, b < h, c < h, d < f, f < g, f < j, g < k, h < i, h < j, h < k
fn select_2871([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_2691([a, b, c, f, e, g, h, j, i, k])
    } else {
        select_292([a, d, h, e, i, f, j, g])
    }
}
/// n = 12, i = 6, a < e, a < l, b < h, c < h, d < f, d < g, f < k, g < j, g < l, h < i, h < j, h < k, h < l
fn select_2872([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < f {
        select_2870([a, b, c, d, e, g, f, h, i, j, k, l])
    } else {
        select_2871([a, b, c, d, e, f, g, h, i, k, j])
    }
}
/// n = 9, i = 3, a < d, a < g, a < h, b < c, b < g, b < h, b < i, c < f, d < e, d < i
fn select_2873([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_105([c, a, g, h, f, d])
    } else {
        select_222([d, b, g, h, e, c, i])
    }
}
/// n = 10, i = 4, a < f, a < g, b < e, b < h, b < i, b < j, c < d, c < f, e < g, f < h, f < i, f < j
fn select_2874([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < f {
        select_1509([a, b, c, d, e, f, g])
    } else {
        select_1533([c, b, e, f, d, g, h, i, j])
    }
}
/// n = 10, i = 4, a < d, a < j, b < f, b < i, c < e, c < g, c < h, c < j, e < i, f < g, f < h, f < j
fn select_2875([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if f < a {
        select_2873([f, c, e, a, d, i, g, h, j])
    } else {
        select_2874([b, c, a, d, e, f, i, g, h, j])
    }
}
/// n = 11, i = 5, a < e, a < k, b < g, c < d, c < f, c < i, c < k, f < j, g < h, g < j, h < i, h < k
fn select_2876([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < d {
        select_2875([a, g, c, e, f, h, d, i, j, k])
    } else {
        select_823([f, a, d, b, e, g, j, h, i])
    }
}
/// n = 13, i = 7, a < e, a < m, b < i, c < i, d < f, f < g, f < h, h < l, h < m, i < j, i < k, i < l, i < m
fn select_2877([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if g < j {
        select_2872([a, b, c, f, e, g, h, i, k, l, j, m])
    } else {
        select_2876([a, d, i, k, e, j, f, h, l, g, m])
    }
}
/// n = 9, i = 4, b < f, c < f, c < g, d < e, e < g, g < h, g < i
fn select_2878([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < a {
        select_69([b, e, f, g, a, h, i])
    } else {
        select_158([a, b, c, d, e, f, g])
    }
}
/// n = 11, i = 5, a < g, b < f, b < i, c < d, c < e, c < i, f < h, g < h, g < i, i < j, i < k
fn select_2879([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < d {
        select_2039([f, a, c, e, g, d, h, i, j, k])
    } else {
        select_2878([e, d, b, a, g, f, i, j, k])
    }
}
/// n = 10, i = 4, a < g, a < h, b < f, b < h, c < d, c < e, c < h, f < g, h < i, h < j
fn select_2880([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_1607([a, b, c, f, d, e, g, h, i, j])
    } else {
        select_1607([a, b, c, f, e, d, g, h, i, j])
    }
}
/// n = 11, i = 5, a < f, a < i, b < g, c < d, c < e, c < i, g < h, g < i, i < j, i < k
fn select_2881([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < h {
        select_2879([b, a, c, d, e, f, g, h, i, j, k])
    } else {
        select_2880([a, g, c, d, e, h, f, i, j, k])
    }
}
/// n = 11, i = 5, a < f, a < i, b < g, b < i, c < d, c < e, c < i, g < h, g < k, i < j, i < k
fn select_2882([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < i {
        select_2881([a, b, c, d, e, f, g, h, i, j, k])
    } else {
        select_997([d, e, f, b, i, j, g, h, k])
    }
}
/// n = 14, i = 7, a < e, a < l, b < i, c < i, d < f, d < l, f < g, f < h, h < m, i < j, i < k, i < l, i < m, l < n
fn select_2883([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if h < l {
        select_2877([a, b, c, d, e, f, g, h, i, j, k, m, l])
    } else {
        select_2882([a, d, i, j, k, e, f, g, l, n, h])
    }
}
/// n = 8, i = 2, d < e, d < f, f < g, f < h
fn select_2884([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < d {
        select_47([b, c, a, d, e, f])
    } else {
        select_117([b, c, d, a, e, f, g, h])
    }
}
/// n = 9, i = 3, c < f, c < g, d < e, d < g, g < h, g < i
fn select_2885([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_1545([a, b, c, d, e, g, h, i, f])
    } else {
        select_1545([a, b, d, c, f, g, h, i, e])
    }
}
/// n = 9, i = 3, c < f, c < i, d < e, d < g, g < h, g < i
fn select_2886([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if g < c {
        select_2884([a, b, e, g, h, c, f, i])
    } else {
        select_2885([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 3, c < f, c < h, d < e, d < g, g < i, h < j
fn select_2887([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if g < h {
        select_2886([a, b, c, d, e, f, g, i, h])
    } else {
        select_2886([a, b, d, c, f, e, h, j, g])
    }
}
/// n = 14, i = 7, a < e, b < i, c < i, d < f, d < l, f < g, f < h, h < n, i < j, i < k, i < l, i < n, l < m
fn select_2888([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if a < l {
        select_2883([a, b, c, d, e, f, g, h, i, j, k, l, n, m])
    } else {
        select_2887([j, k, f, l, m, g, a, h, e, n])
    }
}
/// n = 8, i = 4, b < f, c < g, d < g, e < f, e < g, g < h
fn select_2889([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < b {
        select_371([a, b, d, e, f, g, h])
    } else {
        select_2164([a, c, d, e, f, g])
    }
}
/// n = 5, i = 3, d < e
fn select_2890([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_843([c, a, d, e, b])
    } else {
        select_843([c, b, d, e, a])
    }
}
/// n = 8, i = 4, a < f, b < g, c < g, d < e, d < f, d < h, g < h
fn select_2891([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < f {
        select_372([a, b, c, d, e, g, h])
    } else {
        select_2890([b, c, e, a, f])
    }
}
/// n = 9, i = 5, a < h, b < h, c < g, c < h, d < e, d < f, d < i, h < i
fn select_2892([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_2889([f, e, a, b, c, g, h, i])
    } else {
        select_2891([g, a, b, d, f, e, h, i])
    }
}
/// n = 10, i = 4, b < f, c < f, c < g, d < e, e < g, g < h, g < i, g < j
fn select_2893([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_158([a, b, c, d, e, f, g])
    } else {
        select_547([b, e, f, g, a, h, i, j])
    }
}
/// n = 13, i = 6, a < h, b < h, b < j, c < g, d < e, d < f, f < j, g < i, g < j, h < i, j < k, j < l, j < m
fn select_2894([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if f < a {
        select_2893([e, a, b, c, g, h, j, k, l, m])
    } else {
        select_849([c, b, d, e, f, h, g, i, j])
    }
}
/// n = 13, i = 6, a < h, b < i, b < j, c < i, c < j, d < h, d < j, e < f, e < g, g < j, h < i, j < k, j < l, j < m
fn select_2895([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < c {
        select_2894([a, d, b, e, f, g, c, h, i, j, k, l, m])
    } else {
        select_2894([a, d, c, e, f, g, b, h, i, j, k, l, m])
    }
}
/// n = 14, i = 6, a < g, b < j, c < i, c < j, d < i, d < j, e < f, e < h, e < n, g < i, h < j, i < n, j < k, j < l, j < m
fn select_2896([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < b {
        select_2892([c, d, g, e, f, h, b, i, n])
    } else {
        select_2895([a, c, d, b, e, f, h, g, i, j, k, l, m])
    }
}
/// n = 9, i = 4, a < h, a < i, b < e, b < f, b < i, c < f, c < g, c < i, d < e, d < g, d < i, g < h
fn select_2897([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < g {
        select_214([a, d, c, f, e, g, i])
    } else {
        select_2562([a, b, d, f, g, e, h, i])
    }
}
/// n = 9, i = 4, a < h, a < i, b < f, b < i, c < f, c < g, c < i, d < e, d < g, d < i, g < h
fn select_2898([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < b {
        select_1304([a, e, c, g, b, f, h])
    } else {
        select_2897([a, b, c, d, e, f, g, h, i])
    }
}
/// n = 10, i = 4, a < j, b < f, b < h, c < f, c < g, c < h, d < e, d < g, d < h, g < j, h < i
fn select_2899([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if h < a {
        select_65([e, g, b, f, h, a, i])
    } else {
        select_2898([a, b, c, d, e, f, g, j, h])
    }
}
/// n = 8, i = 5, a < g, b < g, b < h, c < g, c < h, d < g, d < h, e < f, f < h
fn select_2900([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < f {
        select_560([b, c, d, e, f, g])
    } else {
        select_9([a, b, c, d, h])
    }
}
/// n = 10, i = 5, b < g, b < j, c < g, c < j, d < g, d < j, e < f, e < h, e < i, f < j, g < h, g < i
fn select_2901([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if a < g {
        select_2900([a, b, c, d, e, f, g, j])
    } else {
        select_29([g, e, a, f, j, h, i])
    }
}
/// n = 8, i = 2, c < d, d < e, d < f, d < g, d < h
fn select_2902([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < b {
        select_547([a, c, b, d, e, f, g, h])
    } else {
        select_547([b, c, a, d, e, f, g, h])
    }
}
/// n = 8, i = 5, b < g, b < h, c < g, c < h, d < g, d < h, e < f, f < h
fn select_2903([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < g {
        select_2900([a, b, c, d, e, f, g, h])
    } else {
        select_25([g, e, a, f, h])
    }
}
/// n = 11, i = 5, b < g, b < h, c < g, c < h, d < g, d < h, e < f, e < h, h < i, h < j, h < k
fn select_2904([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if h < f {
        select_2902([a, g, e, h, f, i, j, k])
    } else {
        select_2903([a, b, c, d, e, f, g, h])
    }
}
/// n = 13, i = 5, b < l, b < m, c < h, d < g, d < h, e < g, e < h, f < g, f < h, g < l, g < m, h < i, h < j, h < k
fn select_2905([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if b < c {
        select_2901([a, d, e, f, b, c, g, l, m, h])
    } else {
        select_2904([a, d, e, f, c, b, g, h, i, j, k])
    }
}
/// n = 15, i = 6, a < i, a < j, b < k, c < h, c < k, d < h, d < k, e < f, e < g, e < j, g < k, h < i, h < j, j < o, k < l, k < m, k < n
fn select_2906([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < g {
        select_2899([b, a, h, e, f, i, g, j, o, k])
    } else {
        select_2905([f, a, b, c, d, g, h, k, l, m, n, i, j])
    }
}
/// n = 15, i = 6, a < g, a < j, b < k, c < i, c < k, d < i, d < k, e < f, e < h, e < j, h < k, i < j, j < o, k < l, k < m, k < n
fn select_2907([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if g < i {
        select_2896([a, b, c, d, e, f, g, h, i, k, l, m, n, j])
    } else {
        select_2906([a, b, c, d, e, f, h, i, g, j, k, l, m, n, o])
    }
}
/// n = 9, i = 2, b < e, b < g, c < d, c < f, f < h, g < i
fn select_2908([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_339([a, e, g, c, d, f, i])
    } else {
        select_339([a, d, f, b, e, g, h])
    }
}
/// n = 10, i = 3, b < d, b < h, c < e, c < h, e < f, e < g, g < i, h < j
fn select_2909([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_339([a, f, g, b, d, h, i])
    } else {
        select_681([a, d, c, h, e, f, g, j])
    }
}
/// n = 10, i = 3, b < d, c < e, c < f, e < h, f < g, f < i, i < j
fn select_2910([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < b {
        select_2908([a, e, f, g, h, i, b, j, d])
    } else {
        select_2909([a, b, c, d, f, g, i, e, j, h])
    }
}
/// n = 10, i = 3, b < f, c < d, c < e, c < g, g < h, g < i, i < j
fn select_2911([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_2910([a, b, c, f, d, g, h, e, i, j])
    } else {
        select_2910([a, b, c, f, e, g, h, d, i, j])
    }
}
/// n = 12, i = 5, b < f, c < g, d < g, e < g, g < h, g < i, g < j, g < k, k < l
fn select_2912([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if k < b {
        select_349([a, h, i, j, k, b, l])
    } else {
        select_352([a, b, c, d, e, f, g, h, i, j, k])
    }
}
/// n = 11, i = 3, a < d, a < e, b < f, c < e, c < f, e < k, f < g, f < h, f < i, f < j
fn select_2913([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < a {
        select_189([a, g, h, i, j])
    } else {
        select_58([d, b, c, e, k, f])
    }
}
/// n = 9, i = 4, a < i, b < f, b < g, c < f, c < g, c < i, d < e, d < g, e < i, g < h
fn select_2914([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < g {
        select_1868([a, b, c, d, e, f, g, i])
    } else {
        select_503([a, e, b, c, g, f, h, i])
    }
}
/// n = 12, i = 4, a < f, a < g, b < h, c < f, c < g, c < h, d < e, d < g, d < h, g < l, h < i, h < j, h < k
fn select_2915([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if h < e {
        select_2913([a, b, d, f, g, h, e, i, j, k, l])
    } else {
        select_2914([b, a, c, d, e, f, g, l, h])
    }
}
/// n = 14, i = 6, a < g, a < i, b < j, c < h, d < h, e < f, e < i, e < j, h < i, h < j, i < n, j < k, j < l, j < m
fn select_2916([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < h {
        select_1503([b, c, d, g, e, f, h, i])
    } else {
        select_2915([a, b, h, e, f, g, i, j, k, l, m, n])
    }
}
/// n = 15, i = 6, a < g, a < m, b < i, c < h, c < i, d < h, d < i, e < f, e < i, e < m, h < m, i < j, i < k, i < l, l < n, m < o
fn select_2917([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < h {
        select_2912([f, a, b, c, d, g, i, h, j, k, l, n])
    } else {
        select_2916([a, b, c, d, e, f, g, h, m, i, j, k, l, o])
    }
}
/// n = 15, i = 6, a < g, a < m, b < i, c < f, c < l, c < m, d < h, d < i, e < h, e < i, h < m, i < j, i < k, i < l, l < n, m < o
fn select_2918([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if i < c {
        select_2911([h, a, i, j, k, g, c, f, l, n])
    } else {
        select_2917([a, b, d, e, c, f, g, h, i, j, k, l, m, n, o])
    }
}
/// n = 15, i = 6, a < g, a < m, b < j, c < f, c < h, c < m, d < i, d < j, e < i, e < j, h < n, i < m, j < k, j < l, j < n, m < o
fn select_2919([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if h < j {
        select_2907([a, b, d, e, c, f, g, h, i, m, j, k, l, n, o])
    } else {
        select_2918([a, b, c, d, e, f, g, i, j, k, l, h, m, n, o])
    }
}
/// n = 16, i = 7, a < e, a < o, b < j, c < j, c < m, d < f, d < g, f < m, g < h, g < i, g < o, i < n, j < k, j < l, j < n, m < o, o < p
fn select_2920([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < f {
        select_2888([a, b, c, d, e, g, h, i, j, k, l, f, m, n])
    } else {
        select_2919([a, b, g, c, f, h, e, i, m, j, k, l, o, n, p])
    }
}
/// n = 16, i = 7, a < n, b < f, b < o, c < j, d < g, d < h, e < j, e < m, g < m, h < i, h < o, j < k, j < l, j < n, m < o, o < p
fn select_2921([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < h {
        select_2866([b, c, a, d, e, f, g, h, j, k, l, i, m, n, o])
    } else {
        select_2920([b, c, e, d, f, g, h, i, a, j, k, l, m, n, o, p])
    }
}
/// n = 16, i = 7, b < n, c < k, d < g, d < o, e < k, e < m, f < h, f < i, h < m, i < j, i < o, k < l, k < n, m < o, o < p
fn select_2922([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < k {
        select_2810([b, a, c, d, e, f, g, h, i, j, m, k, l, n, o, p])
    } else {
        select_2921([b, d, c, f, e, g, h, i, j, k, a, l, m, n, o, p])
    }
}
/// n = 16, i = 7, b < h, c < l, d < g, d < o, e < l, e < n, f < i, f < j, i < n, j < k, j < o, l < m, n < o, o < p
fn select_2923([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if h < l {
        select_2705([a, c, b, d, e, f, g, h, i, j, k, n, l, m, o, p])
    } else {
        select_2922([a, b, c, d, e, f, g, i, j, k, l, m, n, h, o, p])
    }
}
/// n = 16, i = 7, b < i, c < m, d < h, d < o, e < j, e < m, f < g, f < k, j < o, k < l, k < o, m < n, o < p
fn select_2924([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if j < g {
        select_2487([a, b, c, d, f, e, h, i, j, k, l, m, g, n, o, p])
    } else {
        select_2923([a, b, c, d, e, f, h, i, g, k, l, m, n, j, o, p])
    }
}
/// n = 16, i = 7, b < j, c < m, d < i, d < o, e < h, e < m, f < g, f < k, k < l, k < o, m < n, o < p
fn select_2925([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if o < h {
        select_1982([a, b, c, e, d, f, g, i, j, k, l, m, n, o, p, h])
    } else {
        select_2924([a, b, c, d, e, f, g, i, j, h, k, l, m, n, o, p])
    }
}
/// n = 16, i = 7, c < j, d < i, d < o, e < h, e < k, f < g, f < l, k < n, l < m, l < o, o < p
fn select_2926([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if k < a {
        select_955([b, c, d, e, f, g, h, i, j, k, l, m, a, n, o, p])
    } else {
        select_2925([b, c, a, d, e, f, g, h, i, j, l, m, k, n, o, p])
    }
}
/// n = 16, i = 7, c < l, d < k, e < j, f < i, f < o, g < h, g < m, m < n, m < o, o < p
fn select_2927([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if c < d {
        select_2926([a, b, e, f, c, g, h, l, i, j, d, m, n, k, o, p])
    } else {
        select_2926([a, b, e, f, d, g, h, k, i, j, c, m, n, l, o, p])
    }
}
/// n = 16, i = 7, c < l, d < k, e < j, f < i, f < n, g < h, g < m, m < o, n < p
fn select_2928([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if m < n {
        select_2927([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n, p])
    } else {
        select_2927([a, b, c, d, e, g, f, i, h, j, k, l, n, p, m, o])
    }
}
/// n = 16, i = 7, c < n, d < m, e < l, f < k, g < j, h < i, h < o, o < p
fn select_2929([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if e < g {
        select_2928([a, b, c, d, f, e, h, i, l, k, m, n, o, g, p, j])
    } else {
        select_2928([a, b, c, d, f, g, h, i, j, k, m, n, o, e, p, l])
    }
}
/// n = 16, i = 7, c < p, d < o, e < n, f < m, g < l, h < k, i < j
fn select_2930([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if c < f {
        select_2929([a, b, d, e, g, h, i, c, p, j, k, l, n, o, f, m])
    } else {
        select_2929([a, b, d, e, g, h, i, f, m, j, k, l, n, o, c, p])
    }
}
/// n = 16, i = 7, e < p, f < o, g < n, h < m, i < l, j < k
fn select_2931([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_2930([c, d, a, e, f, g, h, i, j, k, l, m, n, o, p, b])
    } else {
        select_2930([c, d, b, e, f, g, h, i, j, k, l, m, n, o, p, a])
    }
}
/// n = 16, i = 7, g < p, h < o, i < n, j < m, k < l
fn select_2932([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_2931([c, d, e, f, a, g, h, i, j, k, l, m, n, o, p, b])
    } else {
        select_2931([c, d, e, f, b, g, h, i, j, k, l, m, n, o, p, a])
    }
}
/// n = 16, i = 7, i < p, j < o, k < n, l < m
fn select_2933([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_2932([c, d, e, f, g, h, a, i, j, k, l, m, n, o, p, b])
    } else {
        select_2932([c, d, e, f, g, h, b, i, j, k, l, m, n, o, p, a])
    }
}
/// n = 16, i = 7, k < p, l < o, m < n
fn select_2934([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_2933([c, d, e, f, g, h, i, j, a, k, l, m, n, o, p, b])
    } else {
        select_2933([c, d, e, f, g, h, i, j, b, k, l, m, n, o, p, a])
    }
}
/// n = 16, i = 7, m < p, n < o
fn select_2935([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_2934([c, d, e, f, g, h, i, j, k, l, a, m, n, o, p, b])
    } else {
        select_2934([c, d, e, f, g, h, i, j, k, l, b, m, n, o, p, a])
    }
}
/// n = 16, i = 7, o < p
fn select_2936([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_2935([c, d, e, f, g, h, i, j, k, l, m, n, a, o, p, b])
    } else {
        select_2935([c, d, e, f, g, h, i, j, k, l, m, n, b, o, p, a])
    }
}
/// n = 16, i = 7
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    if a < b {
        select_2936([c, d, e, f, g, h, i, j, k, l, m, n, o, p, a, b])
    } else {
        select_2936([c, d, e, f, g, h, i, j, k, l, m, n, o, p, b, a])
    }
}
