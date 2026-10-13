#[allow(unused)]
pub const N: usize = 16;
#[allow(unused)]
pub const I: usize = 7;

#[allow(unused)]
pub fn run(input: [usize; 16]) -> usize {
  select_0(input)
}


/// n = 1
fn select_29([a]: [usize; 1]) -> usize {
    a
}
/// n = 2, i = 0
fn select_28([a, b]: [usize; 2]) -> usize {
    let a = if a < b {
        a
    } else {
        b
    };
    select_29([a])
}
/// n = 3, i = 0
fn select_27([a, b, c]: [usize; 3]) -> usize {
    let a = if a < b {
        a
    } else {
        b
    };
    select_28([a, c])
}
/// n = 4, i = 0
fn select_26([a, b, c, d]: [usize; 4]) -> usize {
    let a = if a < b {
        a
    } else {
        b
    };
    select_27([a, c, d])
}
/// n = 8, i = 1, a < d, a < e, a < f, b < c, b < g, b < h
fn select_25([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    let (b, d, e, f) = if a < b {
        (b, d, e, f)
    } else {
        (a, c, g, h)
    };
    select_26([b, d, e, f])
}
/// n = 7, i = 1, a < c, a < d, a < e, b < f, b < g
fn select_30([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if a < b {
        select_26([b, c, d, e])
    } else {
        select_27([a, f, g])
    }
}
/// n = 9, i = 1, a < d, a < e, a < f, b < g, b < h, c < i
fn select_24([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_25([a, b, c, d, e, f, g, h])
    } else {
        select_30([a, c, d, e, f, b, i])
    }
}
/// n = 9, i = 1, a < c, a < d, a < e, b < f, b < g
fn select_23([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    let (h, i) = if h < i {
        (h, i)
    } else {
        (i, h)
    };
    select_24([a, b, h, c, d, e, f, g, i])
}
/// n = 4, i = 1, a < b, a < c
fn select_34([a, b, c, d]: [usize; 4]) -> usize {
    if a < d {
        select_27([b, c, d])
    } else {
        select_29([a])
    }
}
/// n = 8, i = 2, a < b, a < g, b < d, b < e, b < f, c < g, c < h
fn select_33([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_26([c, d, e, f])
    } else {
        select_34([a, b, g, h])
    }
}
/// n = 9, i = 2, a < b, a < h, a < i, b < e, b < f, b < g, c < h, d < i
fn select_32([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    let (c, h, d) = if c < d {
        (c, h, d)
    } else {
        (d, i, c)
    };
    select_33([a, b, c, e, f, g, h, d])
}
/// n = 10, i = 3, a < c, a < h, a < i, b < c, b < h, b < j, c < e, c < f, c < g, d < h
fn select_31([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    let (b, i, j) = if i < j {
        (b, i, j)
    } else {
        (a, j, i)
    };
    select_32([b, c, d, i, e, f, g, h, j])
}
/// n = 11, i = 3, a < c, a < d, a < i, b < c, b < d, b < j, c < e, c < f, c < g, d < h
fn select_22([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < k {
        select_23([c, d, e, f, g, h, k, i, j])
    } else {
        select_31([a, b, c, k, e, f, g, d, i, j])
    }
}
/// n = 8, i = 1, a < c, a < d, a < e, b < f, b < g, b < h
fn select_38([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    let (b, c, d, e) = if a < b {
        (b, c, d, e)
    } else {
        (a, f, g, h)
    };
    select_26([b, c, d, e])
}
/// n = 6, i = 1, a < c, a < d, a < e, b < f
fn select_39([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_26([b, c, d, e])
    } else {
        select_28([a, f])
    }
}
/// n = 8, i = 1, a < c, a < d, a < e, b < f, b < g
fn select_37([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < h {
        select_38([a, b, c, d, e, f, g, h])
    } else {
        select_39([a, h, c, d, e, b])
    }
}
/// n = 7, i = 1, a < c, a < d, a < e, b < f
fn select_40([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < g {
        select_30([a, b, c, d, e, f, g])
    } else {
        select_39([a, g, c, d, e, b])
    }
}
/// n = 8, i = 1, a < c, a < d, a < e, b < f
fn select_36([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < g {
        select_37([a, b, c, d, e, f, g, h])
    } else {
        select_40([a, g, c, d, e, b, h])
    }
}
/// n = 8, i = 1, a < d, a < e, a < f, b < g, c < h
fn select_42([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    let (b, c, g) = if b < c {
        (b, c, g)
    } else {
        (c, b, h)
    };
    select_30([a, b, d, e, f, c, g])
}
/// n = 9, i = 2, a < b, a < g, a < h, b < d, b < e, b < f, c < g
fn select_41([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if h < i {
        select_42([b, c, h, d, e, f, g, i])
    } else {
        select_32([a, b, c, i, d, e, f, g, h])
    }
}
/// n = 9, i = 2, a < b, a < f, a < g, b < c, b < d, b < e
fn select_35([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if f < h {
        select_36([b, f, c, d, e, h, g, i])
    } else {
        select_41([a, b, h, c, d, e, f, g, i])
    }
}
/// n = 11, i = 3, a < c, a < d, a < i, b < d, b < e, b < j, c < e, c < g, c < h, d < f
fn select_21([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if b < c {
        select_22([a, b, c, d, e, g, h, f, i, j, k])
    } else {
        select_35([c, b, d, e, j, g, h, i, k])
    }
}
/// n = 6, i = 1, a < d, a < e, b < c, b < f
fn select_47([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, d, e) = if a < b {
        (b, d, e)
    } else {
        (a, c, f)
    };
    select_27([b, d, e])
}
/// n = 3, i = 1, a < b
fn select_49([a, b, c]: [usize; 3]) -> usize {
    if a < c {
        select_28([b, c])
    } else {
        select_29([a])
    }
}
/// n = 6, i = 2, a < d, a < e, a < f, b < d, b < e, c < f
fn select_48([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < c {
        select_34([a, d, e, c])
    } else {
        select_49([a, f, b])
    }
}
/// n = 7, i = 2, a < c, a < e, a < f, b < e, b < f, c < d
fn select_46([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_47([b, c, d, e, f, g])
    } else {
        select_48([a, b, g, e, f, c])
    }
}
/// n = 9, i = 3, a < c, a < e, a < g, b < c, b < d, b < h, c < f, d < g, e < h
fn select_45([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    let (a, d, e, g) = if d < e {
        (a, d, e, g)
    } else {
        (b, e, d, h)
    };
    select_46([a, d, c, f, e, g, i])
}
/// n = 5, i = 1, a < c, a < d, b < e
fn select_52([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < b {
        select_27([b, c, d])
    } else {
        select_28([a, e])
    }
}
/// n = 5, i = 2, a < d, a < e, b < e, c < d
fn select_53([a, b, c, d, e]: [usize; 5]) -> usize {
    let (e, c) = if b < c {
        (e, c)
    } else {
        (d, b)
    };
    select_49([a, e, c])
}
/// n = 6, i = 2, a < b, a < e, b < d, c < e
fn select_51([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < f {
        select_52([b, c, d, f, e])
    } else {
        select_53([a, c, f, b, e])
    }
}
/// n = 8, i = 3, a < c, a < d, b < c, b < f, b < g, c < e, d < f
fn select_50([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < g {
        select_46([b, d, c, e, f, g, h])
    } else {
        select_51([a, c, g, e, d, h])
    }
}
/// n = 10, i = 3, a < d, a < e, a < f, b < d, b < h, c < e, d < g, e < i, f < h
fn select_44([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < c {
        select_45([a, b, d, c, f, g, e, h, j])
    } else {
        select_50([c, a, e, b, i, d, f, j])
    }
}
/// n = 6, i = 2, a < b, b < d, b < e, c < f
fn select_58([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < c {
        select_27([c, d, e])
    } else {
        select_49([a, b, f])
    }
}
/// n = 2, i = 1
fn select_61([a, b]: [usize; 2]) -> usize {
    let b = if a < b {
        b
    } else {
        a
    };
    select_29([b])
}
/// n = 4, i = 2, a < c, b < d
fn select_60([a, b, c, d]: [usize; 4]) -> usize {
    let (b, c) = if c < d {
        (b, c)
    } else {
        (a, d)
    };
    select_61([b, c])
}
/// n = 6, i = 3, a < c, b < e, b < f, c < e, d < f
fn select_59([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < d {
        select_49([b, e, d])
    } else {
        select_60([a, b, c, f])
    }
}
/// n = 7, i = 3, a < c, a < f, b < c, b < g, c < e, d < f
fn select_57([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_58([b, c, d, e, g, f])
    } else {
        select_59([b, a, g, d, c, f])
    }
}
/// n = 9, i = 3, a < d, a < e, a < i, b < d, b < h, c < e, c < h, d < f, e < g
fn select_56([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    let (b, d, c, f) = if d < e {
        (b, d, c, f)
    } else {
        (c, e, b, g)
    };
    select_57([b, a, d, c, f, h, i])
}
/// n = 5, i = 2, a < d, a < e, b < d, c < e
fn select_63([a, b, c, d, e]: [usize; 5]) -> usize {
    let (d, c) = if b < c {
        (d, c)
    } else {
        (e, b)
    };
    select_49([a, d, c])
}
/// n = 3, i = 2
fn select_65([a, b, c]: [usize; 3]) -> usize {
    let b = if a < b {
        b
    } else {
        a
    };
    select_61([b, c])
}
/// n = 4, i = 3
fn select_64([a, b, c, d]: [usize; 4]) -> usize {
    let b = if a < b {
        b
    } else {
        a
    };
    select_65([b, c, d])
}
/// n = 9, i = 5, a < d, b < d, c < d, d < g, d < h, e < g, e < i, f < h, f < i
fn select_62([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < i {
        select_63([d, e, f, g, h])
    } else {
        select_64([a, b, c, i])
    }
}
/// n = 11, i = 5, a < c, b < c, c < f, c < g, d < f, d < j, e < g, e < j, f < h, g < i
fn select_55([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < k {
        select_56([c, d, e, f, g, h, i, j, k])
    } else {
        select_62([a, b, k, c, d, e, f, g, j])
    }
}
/// n = 5, i = 3, a < d, b < d, c < d
fn select_67([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_29([d])
    } else {
        select_64([a, b, c, e])
    }
}
/// n = 9, i = 4, a < c, b < c, c < e, c < g, d < g, d < h, e < f
fn select_66([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < h {
        select_46([c, d, e, f, g, h, i])
    } else {
        select_67([a, b, h, c, i])
    }
}
/// n = 12, i = 5, a < c, b < c, c < f, c < g, c < k, d < f, d < h, e < g, f < i, g < j, h < k
fn select_54([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < h {
        select_55([a, b, c, d, e, f, g, i, j, h, l])
    } else {
        select_66([a, b, c, h, f, i, k, e, l])
    }
}
/// n = 12, i = 5, a < c, b < c, c < f, c < g, c < j, d < f, d < k, e < g, f < h, g < i
fn select_43([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if j < k {
        select_44([c, d, e, f, g, j, h, k, i, l])
    } else {
        select_54([a, b, c, d, e, f, g, k, h, i, j, l])
    }
}
/// n = 13, i = 5, a < c, b < c, c < e, c < f, c < j, d < f, d < h, d < k, e < h, e < i, f < g
fn select_20([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < l {
        select_21([c, d, e, f, h, g, i, l, j, k, m])
    } else {
        select_43([a, b, c, d, l, f, e, g, i, j, k, m])
    }
}
/// n = 8, i = 2, a < d, a < e, a < f, b < c, b < e, b < f, c < h, d < g
fn select_73([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    let (a, c, h) = if c < d {
        (a, c, h)
    } else {
        (b, d, g)
    };
    select_52([a, c, e, f, h])
}
/// n = 4, i = 1, a < d, b < c
fn select_76([a, b, c, d]: [usize; 4]) -> usize {
    let (b, d) = if a < b {
        (b, d)
    } else {
        (a, c)
    };
    select_28([b, d])
}
/// n = 6, i = 2, a < c, a < e, b < e, b < f, c < d
fn select_75([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < f {
        select_76([b, c, d, e])
    } else {
        select_49([a, e, f])
    }
}
/// n = 4, i = 1, a < c, b < d
fn select_78([a, b, c, d]: [usize; 4]) -> usize {
    let (b, c) = if a < b {
        (b, c)
    } else {
        (a, d)
    };
    select_28([b, c])
}
/// n = 4, i = 1, a < b
fn select_77([a, b, c, d]: [usize; 4]) -> usize {
    let (c, d) = if c < d {
        (c, d)
    } else {
        (d, c)
    };
    select_78([a, c, b, d])
}
/// n = 7, i = 2, a < c, a < d, b < d, b < f, b < g, c < e
fn select_74([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < f {
        select_75([a, b, c, e, d, g])
    } else {
        select_77([a, d, f, g])
    }
}
/// n = 9, i = 2, a < d, a < f, a < g, a < h, b < f, b < g, c < h, c < i, d < e
fn select_72([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_73([a, b, c, d, f, g, e, i])
    } else {
        select_74([a, c, d, h, e, b, i])
    }
}
/// n = 7, i = 2, a < c, a < d, a < e, b < d, b < e, b < g, c < f
fn select_80([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_52([b, c, d, e, f])
    } else {
        select_34([a, d, e, g])
    }
}
/// n = 10, i = 3, a < c, a < e, a < g, b < d, b < e, b < h, c < h, c < i, d < g, d < j, e < f
fn select_79([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    let (b, c, d, h, i) = if c < d {
        (b, c, d, h, i)
    } else {
        (a, d, c, g, j)
    };
    select_80([b, c, e, d, h, f, i])
}
/// n = 11, i = 3, a < c, a < d, a < e, b < d, b < f, b < h, b < i, c < h, c < j, d < g, e < i, f < k
fn select_71([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_72([b, e, c, d, g, f, i, h, j])
    } else {
        select_79([a, b, c, f, d, g, e, h, j, k])
    }
}
/// n = 6, i = 1, a < e, a < f, b < c, b < d
fn select_84([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, e, f) = if a < b {
        (b, e, f)
    } else {
        (a, c, d)
    };
    select_27([b, e, f])
}
/// n = 8, i = 2, a < c, a < f, a < g, b < f, b < g, b < h, c < d, c < e
fn select_83([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < h {
        select_84([b, c, d, e, f, g])
    } else {
        select_34([a, f, g, h])
    }
}
/// n = 9, i = 2, a < c, a < d, a < g, a < h, b < g, b < h, b < i, c < e, d < f
fn select_82([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    let (c, d, e) = if c < d {
        (c, d, e)
    } else {
        (d, c, f)
    };
    select_83([a, b, c, d, e, g, h, i])
}
/// n = 7, i = 2, a < d, a < e, b < c, b < e, c < g, d < f
fn select_86([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    let (a, c, g) = if c < d {
        (a, c, g)
    } else {
        (b, d, f)
    };
    select_78([a, c, e, g])
}
/// n = 8, i = 2, a < c, a < f, a < g, b < f, b < h, c < e, d < g
fn select_85([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < d {
        select_74([a, b, c, f, e, d, h])
    } else {
        select_86([a, d, b, c, g, e, h])
    }
}
/// n = 11, i = 3, a < d, a < e, a < f, a < h, b < c, b < d, b < i, c < h, c < k, d < g, e < i, f < j
fn select_81([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < f {
        select_82([a, c, d, e, g, i, f, h, k])
    } else {
        select_85([b, f, d, e, g, c, i, j])
    }
}
/// n = 11, i = 3, a < c, a < d, a < j, b < d, b < e, b < g, b < k, c < g, c < h, d < f, e < i
fn select_70([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if j < k {
        select_71([a, b, c, d, j, e, f, g, k, h, i])
    } else {
        select_81([b, a, c, d, k, e, f, g, j, i, h])
    }
}
/// n = 7, i = 2, a < c, a < d, a < e, a < f, b < c, b < d, b < g
fn select_89([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    let (e, f) = if e < f {
        (e, f)
    } else {
        (f, e)
    };
    select_80([a, b, e, c, d, f, g])
}
/// n = 7, i = 4, a < d, b < d, c < d, d < f, e < g
fn select_90([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < g {
        select_49([d, f, e])
    } else {
        select_64([a, b, c, g])
    }
}
/// n = 9, i = 4, a < c, b < c, c < e, c < f, c < g, d < e, d < f, d < h
fn select_88([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < i {
        select_89([c, d, e, f, g, i, h])
    } else {
        select_90([a, b, i, c, d, g, h])
    }
}
/// n = 6, i = 2, a < d, a < e, b < d, b < f, c < e, c < f
fn select_94([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, e) = if a < b {
        (b, e)
    } else {
        (a, f)
    };
    select_78([b, c, d, e])
}
/// n = 6, i = 2, a < d, a < e, b < d, b < f, c < e
fn select_93([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < f {
        select_94([a, b, c, d, e, f])
    } else {
        select_76([a, f, c, d])
    }
}
/// n = 5, i = 2, a < c, a < d, b < c, b < e
fn select_96([a, b, c, d, e]: [usize; 5]) -> usize {
    let (b, d) = if d < e {
        (b, d)
    } else {
        (a, e)
    };
    select_49([b, c, d])
}
/// n = 7, i = 3, a < d, a < f, b < e, b < f, b < g, c < f, c < g, d < e
fn select_95([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_96([a, b, f, d, g])
    } else {
        select_49([b, e, c])
    }
}
/// n = 7, i = 3, a < d, a < e, a < f, b < d, b < e, c < d, c < g
fn select_92([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if f < g {
        select_93([c, b, f, d, g, e])
    } else {
        select_95([c, a, b, g, f, d, e])
    }
}
/// n = 7, i = 4, a < c, b < c, c < f, d < f, e < g
fn select_97([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_63([c, d, e, f, g])
    } else {
        select_67([a, b, g, c, d])
    }
}
/// n = 8, i = 4, a < b, b < e, b < f, c < e, c < f, d < e, d < g
fn select_91([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < h {
        select_92([b, c, d, e, f, h, g])
    } else {
        select_97([a, h, b, c, d, f, g])
    }
}
/// n = 11, i = 5, a < e, a < f, b < e, b < i, c < e, c < j, d < f, d < g, d < k, e < g, e < h, f < i
fn select_87([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if e < f {
        select_88([a, b, e, d, f, g, h, k, j])
    } else {
        select_91([a, f, b, c, e, i, j, k])
    }
}
/// n = 13, i = 5, a < d, a < e, b < d, b < j, c < e, c < f, c < l, d < f, d < i, d < k, e < i, e < j, f < h, g < m
fn select_69([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if d < g {
        select_70([c, d, e, f, g, h, i, j, m, l, k])
    } else {
        select_87([a, b, g, c, d, e, f, k, j, m, l])
    }
}
/// n = 13, i = 5, a < d, a < e, b < d, b < i, c < e, c < f, c < k, d < f, d < h, d < j, e < h, e < i, f < g
fn select_68([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    let (l, m) = if l < m {
        (l, m)
    } else {
        (m, l)
    };
    select_69([a, b, c, d, e, f, l, g, h, i, j, k, m])
}
/// n = 14, i = 5, a < c, a < g, b < c, b < j, c < e, c < f, c < k, d < e, d < g, d < l, e < h, f < i, g < j
fn select_19([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < g {
        select_20([a, b, c, d, f, e, h, g, i, k, l, m, n])
    } else {
        select_68([a, b, d, c, g, e, h, f, j, k, l, m, n])
    }
}
/// n = 5, i = 0
fn select_104([a, b, c, d, e]: [usize; 5]) -> usize {
    let a = if a < b {
        a
    } else {
        b
    };
    select_26([a, c, d, e])
}
/// n = 9, i = 2, a < b, a < h, b < d, b < e, b < f, b < g, c < h, c < i
fn select_103([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_104([c, d, e, f, g])
    } else {
        select_34([a, b, h, i])
    }
}
/// n = 8, i = 2, a < b, b < d, b < e, b < f, b < g, c < h
fn select_105([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_104([c, d, e, f, g])
    } else {
        select_49([a, b, h])
    }
}
/// n = 9, i = 2, a < b, a < h, b < d, b < e, b < f, b < g, c < h
fn select_102([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < i {
        select_103([a, b, c, d, e, f, g, h, i])
    } else {
        select_105([a, b, i, d, e, f, g, c])
    }
}
/// n = 6, i = 3, a < e, a < f, b < e, c < e, d < f
fn select_107([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, c) = if b < c {
        (b, c)
    } else {
        (c, b)
    };
    select_59([b, a, c, d, e, f])
}
/// n = 7, i = 3, a < c, a < f, b < e, b < f, c < e, d < f
fn select_106([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_93([b, c, d, e, f, g])
    } else {
        select_107([a, b, d, g, f, c])
    }
}
/// n = 10, i = 3, a < c, a < d, b < c, b < h, c < e, c < f, c < g, d < h
fn select_101([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < i {
        select_102([b, c, d, e, f, g, i, h, j])
    } else {
        select_106([a, b, d, i, h, c, j])
    }
}
/// n = 10, i = 3, a < c, a < h, b < c, b < g, c < d, c < e, c < f
fn select_100([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    let (b, a, g, h) = if g < h {
        (b, a, g, h)
    } else {
        (a, b, h, g)
    };
    select_101([b, a, c, g, d, e, f, h, i, j])
}
/// n = 4, i = 2, a < c, b < c
fn select_112([a, b, c, d]: [usize; 4]) -> usize {
    if c < d {
        select_29([c])
    } else {
        select_65([a, b, d])
    }
}
/// n = 9, i = 3, a < c, b < c, c < e, c < f, c < g, c < h, d < i
fn select_111([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_104([d, e, f, g, h])
    } else {
        select_112([a, b, c, i])
    }
}
/// n = 9, i = 3, a < c, b < c, c < d, c < e, c < f, c < g
fn select_110([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    let (h, i) = if h < i {
        (h, i)
    } else {
        (i, h)
    };
    select_111([a, b, c, h, d, e, f, g, i])
}
/// n = 11, i = 4, a < d, a < e, b < d, b < i, c < d, c < j, d < f, d < g, d < h, e < i
fn select_109([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_110([a, b, d, e, f, g, h, j, k])
    } else {
        select_91([a, e, b, c, d, i, j, k])
    }
}
/// n = 11, i = 4, a < d, a < h, b < d, b < j, c < d, c < i, d < e, d < f, d < g
fn select_108([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    let (a, c, h, i) = if h < i {
        (a, c, h, i)
    } else {
        (c, a, i, h)
    };
    select_109([a, c, b, d, h, e, f, g, i, j, k])
}
/// n = 12, i = 5, a < d, a < i, b < d, b < k, c < e, c < i, d < f, d < g, d < h, e < f, e < g, e < j
fn select_99([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_100([c, d, e, f, g, j, h, i, k, l])
    } else {
        select_108([a, b, e, d, f, g, h, i, j, k, l])
    }
}
/// n = 7, i = 1, a < b, a < c, a < d
fn select_116([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    let (e, f) = if e < f {
        (e, f)
    } else {
        (f, e)
    };
    select_40([a, e, b, c, d, f, g])
}
/// n = 6, i = 2, a < c, a < d, a < e, b < c, b < f
fn select_118([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (d, e) = if d < e {
        (d, e)
    } else {
        (e, d)
    };
    select_75([a, b, d, e, c, f])
}
/// n = 5, i = 2, a < c, a < d, b < e
fn select_119([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < c {
        select_96([a, b, c, d, e])
    } else {
        select_49([c, b, d])
    }
}
/// n = 6, i = 2, a < c, a < d, a < e, b < c
fn select_117([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < f {
        select_118([a, b, c, d, e, f])
    } else {
        select_119([a, f, d, e, b])
    }
}
/// n = 8, i = 2, a < b, a < e, a < f, b < c, b < d
fn select_115([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < g {
        select_116([b, c, d, g, e, f, h])
    } else {
        select_117([a, g, b, e, f, h])
    }
}
/// n = 8, i = 2, a < b, b < c, b < d, b < e, b < f
fn select_121([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    let (g, h) = if g < h {
        (g, h)
    } else {
        (h, g)
    };
    select_105([a, b, g, c, d, e, f, h])
}
/// n = 7, i = 4, a < d, b < e, b < f, c < e, c < g, d < e
fn select_122([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    let (c, f, g) = if f < g {
        (c, f, g)
    } else {
        (b, g, f)
    };
    select_59([a, c, d, f, e, g])
}
/// n = 10, i = 4, a < d, a < i, b < d, b < h, c < d, c < j, d < e, d < f, d < g
fn select_120([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < h {
        select_121([b, d, e, f, g, h, i, j])
    } else {
        select_122([b, a, c, h, d, i, j])
    }
}
/// n = 10, i = 4, a < c, a < i, b < c, b < h, c < e, c < f, c < g, d < e, d < j
fn select_114([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_115([c, d, e, j, f, g, h, i])
    } else {
        select_120([a, b, d, c, e, f, g, h, i, j])
    }
}
/// n = 6, i = 2, a < d, a < e, b < e, b < f, c < d, c < f
fn select_127([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < b {
        select_76([b, c, d, e])
    } else {
        select_78([a, c, e, f])
    }
}
/// n = 7, i = 2, a < c, a < f, b < f, b < g, c < e, d < g
fn select_126([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_52([c, b, d, e, f])
    } else {
        select_127([a, b, d, c, f, g])
    }
}
/// n = 6, i = 3, a < c, b < c, c < e, d < f
fn select_129([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < f {
        select_49([c, e, d])
    } else {
        select_65([a, b, f])
    }
}
/// n = 8, i = 4, a < e, a < h, b < e, b < h, c < e, d < f, e < g, f < h
fn select_128([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < f {
        select_129([a, b, e, d, g, f])
    } else {
        select_112([a, b, h, c])
    }
}
/// n = 9, i = 4, a < d, a < e, b < d, b < e, c < d, c < h, d < g, e < h, f < i
fn select_125([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < i {
        select_126([c, e, d, f, g, h, i])
    } else {
        select_128([a, b, c, f, d, i, g, e])
    }
}
/// n = 6, i = 3, a < d, a < e, b < d, b < f, c < e, c < f
fn select_132([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (c, d) = if d < e {
        (c, d)
    } else {
        (b, e)
    };
    select_60([a, c, d, f])
}
/// n = 4, i = 2, a < d, b < c
fn select_133([a, b, c, d]: [usize; 4]) -> usize {
    let (a, c) = if c < d {
        (a, c)
    } else {
        (b, d)
    };
    select_61([a, c])
}
/// n = 6, i = 3, a < d, a < e, b < d, b < f, c < e
fn select_131([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < f {
        select_132([a, b, c, d, e, f])
    } else {
        select_133([a, f, c, d])
    }
}
/// n = 7, i = 3, a < c, a < f, b < e, b < f, c < e, d < g
fn select_130([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_75([c, b, d, g, e, f])
    } else {
        select_131([a, d, b, c, f, g])
    }
}
/// n = 9, i = 4, a < d, a < e, b < d, b < h, c < d, d < g, e < h, f < i
fn select_124([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < e {
        select_125([a, c, b, d, e, f, g, h, i])
    } else {
        select_130([e, b, c, f, d, h, i])
    }
}
/// n = 9, i = 4, a < d, a < h, b < d, b < g, c < d, d < f, e < i
fn select_123([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    let (b, a, g, h) = if g < h {
        (b, a, g, h)
    } else {
        (a, b, h, g)
    };
    select_124([b, a, c, d, g, e, f, h, i])
}
/// n = 10, i = 4, a < c, a < h, b < c, b < g, c < e, c < f, d < e, d < i
fn select_113([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < j {
        select_114([a, b, c, d, e, f, j, g, h, i])
    } else {
        select_123([a, b, j, c, d, f, g, h, i])
    }
}
/// n = 13, i = 5, a < c, a < k, b < e, b < f, b < l, c < f, c < g, c < j, d < e, d < k, e < g, e < i, f < h
fn select_98([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_99([d, b, a, e, c, f, g, i, k, j, l, m])
    } else {
        select_113([b, c, f, d, e, h, j, l, k, m])
    }
}
/// n = 15, i = 6, a < d, a < h, b < d, b < l, c < e, c < g, c < n, d < f, d < g, d < k, e < f, e < m, f < i, g < j, h < l
fn select_18([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if e < h {
        select_19([a, b, d, e, f, g, h, i, j, l, k, m, n, o])
    } else {
        select_98([b, c, d, h, e, g, f, j, m, k, l, n, o])
    }
}
/// n = 15, i = 6, a < d, a < l, b < d, b < m, c < e, c < g, c < n, d < f, d < g, d < j, e < f, e < k, f < h, g < i
fn select_17([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    let (a, b, l, m) = if l < m {
        (a, b, l, m)
    } else {
        (b, a, m, l)
    };
    select_18([a, b, c, d, e, f, g, l, h, i, j, m, k, n, o])
}
/// n = 15, i = 6, a < c, a < k, b < c, b < l, c < f, c < g, c < j, d < f, d < m, e < g, e < n, f < h, g < i
fn select_16([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    let (d, e, g, f, i, h, n, m) = if d < e {
        (d, e, g, f, i, h, n, m)
    } else {
        (e, d, f, g, h, i, m, n)
    };
    select_17([a, b, d, c, e, g, f, i, h, j, n, k, l, m, o])
}
/// n = 7, i = 2, a < b, b < d, b < e, b < f, c < g
fn select_142([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_26([c, d, e, f])
    } else {
        select_49([a, b, g])
    }
}
/// n = 8, i = 2, a < b, a < g, b < d, b < e, b < f, c < g
fn select_141([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < h {
        select_33([a, b, c, d, e, f, g, h])
    } else {
        select_142([a, b, h, d, e, f, c])
    }
}
/// n = 8, i = 2, a < b, a < f, b < c, b < d, b < e
fn select_140([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_40([b, f, c, d, e, g, h])
    } else {
        select_141([a, b, g, c, d, e, f, h])
    }
}
/// n = 9, i = 3, a < c, a < h, b < c, b < i, c < e, c < f, c < g, d < h, d < i
fn select_145([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_26([d, e, f, g])
    } else {
        select_96([a, b, c, h, i])
    }
}
/// n = 9, i = 3, a < c, a < h, b < c, b < i, c < e, c < f, c < g, d < h
fn select_144([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < i {
        select_145([a, b, c, d, e, f, g, h, i])
    } else {
        select_142([a, c, i, e, f, g, d])
    }
}
/// n = 10, i = 4, a < d, a < h, b < d, b < i, c < d, c < j, d < e, d < f, d < g
fn select_143([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    let (b, h, i) = if h < i {
        (b, h, i)
    } else {
        (a, i, h)
    };
    select_144([b, c, d, h, e, f, g, i, j])
}
/// n = 10, i = 4, a < c, a < h, b < c, b < i, c < e, c < f, c < g, d < e, d < f, d < j
fn select_139([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_140([c, d, e, f, j, g, h, i])
    } else {
        select_143([a, b, d, c, e, f, g, h, i, j])
    }
}
/// n = 10, i = 4, a < c, a < h, b < c, b < i, c < e, c < f, c < g, d < e, d < j
fn select_146([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_115([c, d, e, j, f, g, h, i])
    } else {
        select_143([a, b, d, c, e, f, g, h, i, j])
    }
}
/// n = 12, i = 4, a < d, a < e, a < j, b < e, b < f, b < k, c < d, c < l, d < f, d < i, e < g, e < h
fn select_138([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if d < e {
        select_139([a, c, d, b, e, f, i, j, l, k])
    } else {
        select_146([a, b, e, c, d, g, h, j, k, l])
    }
}
/// n = 9, i = 3, a < c, a < g, b < c, b < h, c < d, c < e, c < f
fn select_149([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    let (b, g, h) = if g < h {
        (b, g, h)
    } else {
        (a, h, g)
    };
    select_141([b, c, g, d, e, f, h, i])
}
/// n = 9, i = 3, a < b, a < g, b < d, b < e, b < f, c < d, c < h
fn select_148([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if b < c {
        select_115([b, c, d, h, e, f, g, i])
    } else {
        select_149([a, c, b, d, e, f, g, h, i])
    }
}
/// n = 6, i = 2, a < b, b < c, b < d
fn select_152([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (e, f) = if e < f {
        (e, f)
    } else {
        (f, e)
    };
    select_58([a, b, e, c, d, f])
}
/// n = 6, i = 3, a < d, a < f, b < d, b < e, c < d
fn select_154([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (a, e, f) = if e < f {
        (a, e, f)
    } else {
        (b, f, e)
    };
    select_63([a, c, e, d, f])
}
/// n = 5, i = 3, a < d, b < d, c < e
fn select_156([a, b, c, d, e]: [usize; 5]) -> usize {
    if d < e {
        select_61([c, d])
    } else {
        select_65([a, b, e])
    }
}
/// n = 6, i = 3, a < c, b < d, b < e, c < d
fn select_155([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if c < f {
        select_96([b, c, d, e, f])
    } else {
        select_156([a, f, b, c, e])
    }
}
/// n = 8, i = 4, a < c, b < e, b < g, b < h, c < e, c < f, d < e, d < g
fn select_153([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if f < g {
        select_154([b, c, d, e, f, h])
    } else {
        select_155([a, d, c, e, g, h])
    }
}
/// n = 9, i = 4, a < d, a < e, b < d, b < g, b < i, c < d, c < g, d < f, e < h
fn select_151([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < e {
        select_152([a, d, e, f, g, i])
    } else {
        select_153([a, b, e, c, d, h, g, i])
    }
}
/// n = 6, i = 2, a < c, a < d, a < e, b < c, b < d, b < f
fn select_160([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, e) = if e < f {
        (b, e)
    } else {
        (a, f)
    };
    select_34([b, c, d, e])
}
/// n = 6, i = 2, a < c, a < d, a < e, b < c, b < d
fn select_159([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < f {
        select_160([a, b, c, d, e, f])
    } else {
        select_133([a, f, b, e])
    }
}
/// n = 7, i = 3, a < d, a < e, a < f, b < d, b < e, c < d, c < f
fn select_158([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    let (c, f, e) = if b < c {
        (c, f, e)
    } else {
        (b, e, f)
    };
    select_159([a, c, d, f, e, g])
}
/// n = 5, i = 2, a < d, b < d, c < e
fn select_162([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < e {
        select_63([a, b, c, d, e])
    } else {
        select_49([e, a, b])
    }
}
/// n = 6, i = 3, a < d, a < e, b < d, c < f
fn select_161([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_162([c, e, b, f, d])
    } else {
        select_59([c, a, f, b, e, d])
    }
}
/// n = 8, i = 4, a < d, a < g, b < e, b < f, b < h, c < e, c < f, d < e
fn select_157([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if d < h {
        select_158([b, c, d, e, f, h, g])
    } else {
        select_161([a, h, c, d, g, f])
    }
}
/// n = 9, i = 4, a < d, a < g, a < h, b < d, b < g, c < d, d < f, e < i
fn select_150([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < e {
        select_151([c, a, b, d, e, f, g, i, h])
    } else {
        select_157([e, a, b, c, d, g, i, h])
    }
}
/// n = 10, i = 4, a < c, a < g, a < h, b < c, b < g, c < e, c < f, d < e, d < i
fn select_147([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < j {
        select_148([b, c, d, e, f, j, g, i, h])
    } else {
        select_150([a, b, j, c, d, f, g, h, i])
    }
}
/// n = 13, i = 5, a < c, a < i, b < f, b < g, b < l, c < e, c < j, d < e, d < m, e < g, e < h, f < i, f < k
fn select_137([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if c < f {
        select_138([c, b, d, e, f, g, i, k, h, j, l, m])
    } else {
        select_147([f, a, c, d, e, j, i, k, m, l])
    }
}
/// n = 5, i = 1, a < b, a < c
fn select_168([a, b, c, d, e]: [usize; 5]) -> usize {
    let (d, e) = if d < e {
        (d, e)
    } else {
        (e, d)
    };
    select_52([a, d, b, c, e])
}
/// n = 8, i = 2, a < c, a < d, a < e, b < d, b < e, b < g, b < h, c < f
fn select_167([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < g {
        select_80([a, b, c, d, e, f, h])
    } else {
        select_168([a, d, e, g, h])
    }
}
/// n = 6, i = 2, a < d, a < e, b < c, b < f
fn select_169([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, c, f, d, e) = if a < b {
        (b, c, f, d, e)
    } else {
        (a, d, e, c, f)
    };
    select_168([b, c, f, d, e])
}
/// n = 8, i = 2, a < d, a < e, a < f, a < g, b < d, b < e, c < h
fn select_166([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_167([b, a, c, d, e, h, f, g])
    } else {
        select_169([a, c, b, f, g, h])
    }
}
/// n = 8, i = 2, a < c, a < d, a < e, a < f, b < c, b < d
fn select_165([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    let (g, h) = if g < h {
        (g, h)
    } else {
        (h, g)
    };
    select_166([a, b, g, c, d, e, f, h])
}
/// n = 7, i = 2, a < b, b < e, b < f, c < d, d < g
fn select_173([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_27([c, e, f])
    } else {
        select_78([a, d, b, g])
    }
}
/// n = 6, i = 3, a < c, b < c, c < d
fn select_174([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (e, f) = if e < f {
        (e, f)
    } else {
        (f, e)
    };
    select_129([a, b, c, e, d, f])
}
/// n = 10, i = 4, a < e, a < f, b < e, b < f, c < e, d < f, d < j, e < g, e < h, f < i
fn select_172([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < j {
        select_173([c, e, d, f, g, h, i])
    } else {
        select_174([a, b, f, i, c, j])
    }
}
/// n = 7, i = 3, a < c, a < f, b < c, b < f, c < e, d < g
fn select_175([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_77([c, e, d, f])
    } else {
        select_156([a, b, d, f, g])
    }
}
/// n = 11, i = 5, a < c, a < j, b < f, b < g, c < g, c < i, d < f, e < g, e < k, f < i, f < j, g < h
fn select_171([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < f {
        select_172([b, c, d, e, f, g, i, j, h, k])
    } else {
        select_175([a, f, c, e, i, j, k])
    }
}
/// n = 8, i = 3, a < d, a < g, b < d, b < h, c < g, c < h, d < f, e < g
fn select_177([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if a < c {
        select_126([b, c, d, e, f, h, g])
    } else {
        select_93([a, b, e, d, g, h])
    }
}
/// n = 7, i = 3, a < c, a < f, b < c, c < e, d < f
fn select_178([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_58([b, c, d, e, g, f])
    } else {
        select_107([a, b, g, d, c, f])
    }
}
/// n = 9, i = 4, a < d, a < e, b < d, b < h, c < d, c < i, d < g, e < h, f < h
fn select_176([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < i {
        select_177([b, c, e, d, f, g, h, i])
    } else {
        select_178([a, b, d, i, g, e, f])
    }
}
/// n = 11, i = 5, a < c, a < i, b < f, b < g, c < f, c < j, d < f, d < k, e < g, f < h, g < i
fn select_170([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < j {
        select_171([a, b, c, e, d, g, f, h, j, i, k])
    } else {
        select_176([c, b, d, f, j, e, h, g, k])
    }
}
/// n = 13, i = 5, a < b, a < k, b < e, b < l, c < e, c < m, d < g, d < h, d < i, e < h, e < i, e < j, f < g, g < k
fn select_164([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13]) -> usize {
    if e < f {
        select_165([e, d, h, i, f, j, l, m])
    } else {
        select_170([a, f, b, c, d, e, g, j, k, l, m])
    }
}
/// n = 5, i = 1, a < b, a < c, a < d
fn select_183([a, b, c, d, e]: [usize; 5]) -> usize {
    if a < e {
        select_26([b, c, d, e])
    } else {
        select_29([a])
    }
}
/// n = 7, i = 2, a < b, a < f, b < d, b < e, c < g
fn select_182([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_183([b, c, d, e, f])
    } else {
        select_96([a, c, b, f, g])
    }
}
/// n = 7, i = 3, a < e, a < f, a < g, b < e, b < f, c < e, d < g
fn select_184([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_95([c, a, b, d, g, e, f])
    } else {
        select_131([a, d, b, g, f, c])
    }
}
/// n = 8, i = 3, a < c, a < f, a < g, b < c, b < f, c < e, d < g
fn select_181([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < h {
        select_182([b, c, d, e, h, f, g])
    } else {
        select_184([a, b, h, d, c, f, g])
    }
}
/// n = 5, i = 2, a < c, a < e, b < c, b < d
fn select_188([a, b, c, d, e]: [usize; 5]) -> usize {
    let (a, d) = if d < e {
        (a, d)
    } else {
        (b, e)
    };
    select_49([a, c, d])
}
/// n = 6, i = 2, a < d, a < e, a < f, b < d, c < e
fn select_187([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, d, c) = if b < c {
        (b, d, c)
    } else {
        (c, e, b)
    };
    select_188([a, b, d, c, f])
}
/// n = 8, i = 3, a < d, a < g, a < h, b < d, c < g, c < h, d < f, e < g
fn select_186([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < c {
        select_51([a, d, c, f, h, e])
    } else {
        select_187([a, b, e, d, g, h])
    }
}
/// n = 9, i = 4, a < d, a < e, b < d, b < h, b < i, c < d, d < g, e < h, f < h
fn select_185([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < i {
        select_186([b, c, e, d, f, g, h, i])
    } else {
        select_178([a, c, d, i, g, e, f])
    }
}
/// n = 11, i = 5, a < d, a < g, b < e, b < i, c < d, d < i, d < j, e < h, e < k, f < h, f < j, g < h
fn select_180([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_181([d, b, e, f, k, i, j, g])
    } else {
        select_185([a, e, c, d, g, f, i, h, k])
    }
}
/// n = 6, i = 3, a < e, a < f, b < e, b < f, c < e, d < f
fn select_192([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (e, d) = if c < d {
        (e, d)
    } else {
        (f, c)
    };
    select_112([a, b, e, d])
}
/// n = 6, i = 3, a < d, a < e, b < d, b < e, c < d, c < e
fn select_193([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if d < f {
        select_28([d, e])
    } else {
        select_67([a, b, c, e, f])
    }
}
/// n = 9, i = 5, a < e, a < h, b < e, b < h, c < f, c < i, d < f, e < i, f < h, g < i
fn select_191([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if e < f {
        select_192([c, e, d, g, f, i])
    } else {
        select_193([a, b, f, e, h, g])
    }
}
/// n = 6, i = 3, a < d, a < e, b < d, b < f, c < d
fn select_195([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, e, f) = if e < f {
        (b, e, f)
    } else {
        (a, f, e)
    };
    select_63([b, c, e, d, f])
}
/// n = 6, i = 4, a < e, b < e, c < f, d < f
fn select_197([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (c, d, e) = if e < f {
        (c, d, e)
    } else {
        (a, b, f)
    };
    select_65([c, d, e])
}
/// n = 6, i = 4, a < d, b < d, c < e
fn select_196([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if e < f {
        select_156([a, b, e, d, f])
    } else {
        select_197([a, b, c, f, d, e])
    }
}
/// n = 7, i = 4, a < c, b < e, b < f, c < e, d < e
fn select_194([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_195([b, c, d, e, f, g])
    } else {
        select_196([a, g, b, c, f, d])
    }
}
/// n = 11, i = 6, a < c, b < f, b < j, c < g, c < i, d < g, d < i, e < f, f < i, f < k, g < j, h < j
fn select_190([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < k {
        select_191([c, d, b, e, g, f, h, i, j])
    } else {
        select_194([a, f, c, d, i, k, h])
    }
}
/// n = 7, i = 4, a < f, a < g, b < f, b < g, c < d, d < f, e < g
fn select_200([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if d < e {
        select_112([a, b, f, e])
    } else {
        select_156([a, b, c, g, d])
    }
}
/// n = 8, i = 5, a < f, a < g, b < f, b < g, c < f, c < h, d < f, e < h
fn select_199([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if g < h {
        select_107([c, e, g, d, h, f])
    } else {
        select_200([a, b, e, h, d, g, f])
    }
}
/// n = 4, i = 2, a < b
fn select_203([a, b, c, d]: [usize; 4]) -> usize {
    let (c, d) = if c < d {
        (c, d)
    } else {
        (d, c)
    };
    select_60([a, c, b, d])
}
/// n = 7, i = 4, a < f, a < g, b < c, c < f, d < g, e < g
fn select_202([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < d {
        select_203([a, f, d, e])
    } else {
        select_59([b, a, c, e, f, g])
    }
}
/// n = 5, i = 3, a < c, b < c
fn select_204([a, b, c, d, e]: [usize; 5]) -> usize {
    let (d, e) = if d < e {
        (d, e)
    } else {
        (e, d)
    };
    select_156([a, b, d, c, e])
}
/// n = 8, i = 5, a < c, b < d, c < g, d < h, e < g, f < g
fn select_201([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < h {
        select_202([c, b, d, e, f, h, g])
    } else {
        select_204([a, h, c, e, f])
    }
}
/// n = 10, i = 6, a < c, b < f, b < h, c < h, c < i, d < h, d < i, e < f, f < j, g < h
fn select_198([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if i < j {
        select_199([c, d, b, g, e, h, i, f])
    } else {
        select_201([a, e, c, f, d, g, h, j])
    }
}
/// n = 11, i = 6, a < e, a < j, b < e, c < f, c < j, d < g, d < i, e < i, e < k, f < i, g < j, h < j
fn select_189([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_190([c, a, f, d, b, e, g, h, i, j, k])
    } else {
        select_198([d, a, g, c, b, e, h, j, f, k])
    }
}
/// n = 12, i = 6, a < d, a < g, b < d, b < j, c < g, c < h, d < i, d < k, e < g, f < i, f < l, g < j, h < i
fn select_179([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if g < l {
        select_180([c, b, e, g, d, f, h, i, j, l, k])
    } else {
        select_189([a, b, c, f, d, h, l, e, i, g, k])
    }
}
/// n = 14, i = 6, a < c, a < h, b < c, b < k, c < f, c < l, d < f, d < m, e < h, e < i, e < n, f < i, f < j, g < h, h < k
fn select_163([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < n {
        select_164([b, c, d, e, f, g, h, i, n, j, k, l, m])
    } else {
        select_179([a, b, e, c, g, d, h, n, f, k, l, m])
    }
}
/// n = 14, i = 6, a < c, a < g, b < c, b < j, c < f, c < k, d < f, d < l, e < g, e < h, e < m, f < h, f < i, g < j
fn select_136([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if g < n {
        select_137([b, e, c, d, f, g, h, i, j, k, n, m, l])
    } else {
        select_163([a, b, c, d, e, f, n, g, h, i, j, k, l, m])
    }
}
/// n = 6, i = 1, a < b, a < c, a < d
fn select_210([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (e, f) = if e < f {
        (e, f)
    } else {
        (f, e)
    };
    select_39([a, e, b, c, d, f])
}
/// n = 5, i = 2, a < c, a < d, b < c
fn select_211([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < e {
        select_96([a, b, c, d, e])
    } else {
        select_133([a, e, b, d])
    }
}
/// n = 7, i = 2, a < b, a < e, b < c, b < d
fn select_209([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < f {
        select_210([b, c, d, f, e, g])
    } else {
        select_211([a, f, b, e, g])
    }
}
/// n = 7, i = 2, a < b, b < c, b < d, b < e
fn select_213([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    let (f, g) = if f < g {
        (f, g)
    } else {
        (g, f)
    };
    select_142([a, b, f, c, d, e, g])
}
/// n = 9, i = 4, a < d, a < g, b < d, b < h, c < d, c < i, d < e, d < f
fn select_212([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < g {
        select_213([a, d, e, f, g, h, i])
    } else {
        select_122([a, b, c, g, d, h, i])
    }
}
/// n = 9, i = 4, a < c, a < g, b < c, b < h, c < e, c < f, d < e, d < i
fn select_208([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_209([c, d, e, i, f, g, h])
    } else {
        select_212([a, b, d, c, e, f, g, h, i])
    }
}
/// n = 11, i = 4, a < d, a < e, a < i, b < d, b < j, c < e, c < k, d < f, d < g, e < h
fn select_207([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if d < e {
        select_146([a, b, d, c, e, f, g, i, j, k])
    } else {
        select_208([a, c, e, b, d, h, i, k, j])
    }
}
/// n = 6, i = 2, a < c, a < d, b < c, b < e
fn select_217([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, e, d) = if a < b {
        (b, e, d)
    } else {
        (a, d, e)
    };
    select_168([b, c, e, d, f])
}
/// n = 5, i = 2, a < d, b < c
fn select_218([a, b, c, d, e]: [usize; 5]) -> usize {
    let (b, c, d) = if a < b {
        (b, c, d)
    } else {
        (a, d, c)
    };
    select_77([b, c, d, e])
}
/// n = 6, i = 2, a < c, a < d, b < c
fn select_216([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < e {
        select_217([a, b, c, d, e, f])
    } else {
        select_218([a, e, b, d, f])
    }
}
/// n = 8, i = 2, a < b, a < e, b < c, b < d
fn select_215([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if b < f {
        select_140([a, b, c, d, f, e, g, h])
    } else {
        select_216([a, f, b, e, g, h])
    }
}
/// n = 7, i = 3, a < d, a < e, a < f, b < d, b < e, b < g, c < d, c < f, c < g
fn select_223([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    let (c, e) = if e < f {
        (c, e)
    } else {
        (b, f)
    };
    select_96([a, c, d, e, g])
}
/// n = 7, i = 3, a < d, a < e, a < f, b < d, b < e, b < g, c < d, c < f
fn select_222([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_223([a, b, c, d, e, f, g])
    } else {
        select_133([a, g, c, e])
    }
}
/// n = 9, i = 3, a < c, a < g, a < i, b < c, b < h, c < e, c < f, d < g, d < h
fn select_221([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_183([c, d, e, f, i])
    } else {
        select_222([d, a, b, c, g, h, i])
    }
}
/// n = 8, i = 3, a < c, a < g, b < c, c < e, c < f, d < g
fn select_224([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < h {
        select_142([b, c, d, e, f, h, g])
    } else {
        select_107([a, b, h, d, c, g])
    }
}
/// n = 10, i = 4, a < d, a < e, b < d, b < i, b < j, c < d, c < h, d < f, d < g, e < h
fn select_220([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if e < i {
        select_221([b, c, d, e, f, g, i, h, j])
    } else {
        select_224([a, c, d, i, f, g, e, j])
    }
}
/// n = 10, i = 4, a < d, a < g, a < h, b < d, b < i, c < d, c < j, d < e, d < f
fn select_219([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    let (b, c, i, j) = if i < j {
        (b, c, i, j)
    } else {
        (c, b, j, i)
    };
    select_220([b, a, c, d, i, e, f, j, g, h])
}
/// n = 10, i = 4, a < c, a < g, a < h, b < c, b < i, c < e, c < f, d < e, d < j
fn select_214([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_215([c, d, e, j, f, g, h, i])
    } else {
        select_219([a, b, d, c, e, f, g, h, i, j])
    }
}
/// n = 12, i = 5, a < c, a < i, b < e, b < i, c < f, c < h, d < f, d < l, e < j, e < k, f < g
fn select_206([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if c < e {
        select_207([c, b, d, e, f, j, k, g, h, i, l])
    } else {
        select_214([e, a, c, d, f, h, j, k, i, l])
    }
}
/// n = 7, i = 2, a < c, a < d, a < e, b < c, b < f, b < g
fn select_229([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    let (a, b, d, e, g) = if d < f {
        (a, b, d, e, g)
    } else {
        (b, a, f, g, e)
    };
    select_118([a, b, c, d, e, g])
}
/// n = 6, i = 2, a < c, b < e, b < f, c < d
fn select_230([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if a < e {
        select_75([a, b, c, d, e, f])
    } else {
        select_49([e, a, f])
    }
}
/// n = 7, i = 2, a < c, a < d, a < e, b < c, b < f
fn select_228([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < g {
        select_229([a, b, c, d, e, f, g])
    } else {
        select_230([g, a, b, f, d, e])
    }
}
/// n = 6, i = 2, a < c, a < d, b < e
fn select_231([a, b, c, d, e, f]: [usize; 6]) -> usize {
    if b < c {
        select_217([a, b, c, d, e, f])
    } else {
        select_77([c, b, d, f])
    }
}
/// n = 7, i = 2, a < c, a < d, a < e, b < c
fn select_227([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < f {
        select_228([a, b, c, d, e, f, g])
    } else {
        select_231([a, f, d, e, b, g])
    }
}
/// n = 5, i = 2, a < b, b < c
fn select_235([a, b, c, d, e]: [usize; 5]) -> usize {
    if b < d {
        select_34([b, c, d, e])
    } else {
        select_112([a, d, b, e])
    }
}
/// n = 5, i = 3, a < c, b < d
fn select_236([a, b, c, d, e]: [usize; 5]) -> usize {
    let (b, c, d) = if c < d {
        (b, c, d)
    } else {
        (a, d, c)
    };
    select_112([b, c, d, e])
}
/// n = 7, i = 3, a < c, a < f, b < c, c < e, d < g
fn select_234([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_235([b, c, e, d, f])
    } else {
        select_236([a, d, f, g, b])
    }
}
/// n = 11, i = 5, a < f, a < g, b < c, c < g, c < i, d < f, e < g, e < k, f < i, f < j, g < h
fn select_233([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if c < f {
        select_172([a, c, d, e, f, g, i, j, h, k])
    } else {
        select_234([f, b, c, e, i, j, k])
    }
}
/// n = 11, i = 5, a < c, b < f, b < g, c < f, c < j, d < f, d < k, e < g, f < h, g < i
fn select_232([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if g < j {
        select_233([b, a, c, e, d, g, f, h, j, i, k])
    } else {
        select_176([c, b, d, f, j, e, h, g, k])
    }
}
/// n = 12, i = 5, a < b, b < e, b < k, c < e, c < l, d < g, d < h, e < h, e < i, f < g, g < j
fn select_226([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if e < f {
        select_227([e, d, h, f, i, k, l])
    } else {
        select_232([a, f, b, c, d, e, g, i, j, k, l])
    }
}
/// n = 7, i = 2, a < b, a < c, b < e, c < f, d < g
fn select_241([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < d {
        select_52([b, c, d, e, f])
    } else {
        select_75([a, d, c, f, b, g])
    }
}
/// n = 9, i = 4, a < c, b < d, b < e, c < d, c < h, d < g, e < h, f < i
fn select_240([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < f {
        select_241([b, d, e, f, g, h, i])
    } else {
        select_155([a, b, c, d, e, i])
    }
}
/// n = 7, i = 2, a < c, a < e, a < f, b < e, b < g, c < d
fn select_243([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < c {
        select_160([a, b, c, e, f, g])
    } else {
        select_34([c, b, d, f])
    }
}
/// n = 7, i = 3, a < c, b < d, b < e, b < f, c < d, c < e
fn select_244([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < g {
        select_160([b, c, d, e, f, g])
    } else {
        select_156([a, g, b, c, f])
    }
}
/// n = 9, i = 4, a < c, b < e, b < g, b < h, c < f, c < g, c < h, d < g, d < i, e < f
fn select_242([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if c < d {
        select_243([b, d, e, f, g, h, i])
    } else {
        select_244([a, b, c, g, h, e, i])
    }
}
/// n = 10, i = 4, a < b, b < f, b < h, b < i, c < f, d < h, d < i, e < h, e < j, f < g
fn select_239([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if c < d {
        select_240([a, c, b, f, d, e, g, i, j])
    } else {
        select_242([a, d, b, e, c, f, h, i, j])
    }
}
/// n = 6, i = 3, a < d, a < e, b < d, c < e
fn select_246([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, d, c) = if d < e {
        (b, d, c)
    } else {
        (c, e, b)
    };
    select_204([a, b, d, c, f])
}
/// n = 7, i = 4, a < b, b < e, c < e, d < f
fn select_245([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if b < f {
        select_246([b, c, d, e, f, g])
    } else {
        select_204([a, f, b, c, g])
    }
}
/// n = 10, i = 4, a < b, b < f, b < h, c < f, d < h, d < i, e < h, e < j, f < g
fn select_238([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if b < i {
        select_239([a, b, c, d, e, f, g, h, i, j])
    } else {
        select_245([d, i, a, e, b, j, c])
    }
}
/// n = 8, i = 3, a < c, a < f, a < g, b < c, b < f, c < e, d < h
fn select_248([a, b, c, d, e, f, g, h]: [usize; 8]) -> usize {
    if c < d {
        select_168([c, d, e, f, g])
    } else {
        select_92([a, b, d, c, f, g, h])
    }
}
/// n = 7, i = 3, a < c, b < c, c < d, c < e
fn select_250([a, b, c, d, e, f, g]: [usize; 7]) -> usize {
    if c < f {
        select_183([c, d, e, f, g])
    } else {
        select_67([a, b, f, c, g])
    }
}
/// n = 6, i = 4, a < c, b < d
fn select_251([a, b, c, d, e, f]: [usize; 6]) -> usize {
    let (b, c, d) = if c < d {
        (b, c, d)
    } else {
        (a, d, c)
    };
    select_204([b, c, d, e, f])
}
/// n = 9, i = 4, a < d, a < h, b < d, c < d, d < f, d < g, e < i
fn select_249([a, b, c, d, e, f, g, h, i]: [usize; 9]) -> usize {
    if d < i {
        select_250([b, c, d, f, g, e, h])
    } else {
        select_251([a, e, h, i, b, c])
    }
}
/// n = 10, i = 5, a < d, b < d, c < e, c < g, d < g, d < h, e < i, f < j
fn select_247([a, b, c, d, e, f, g, h, i, j]: [usize; 10]) -> usize {
    if d < e {
        select_248([d, c, e, f, i, g, h, j])
    } else {
        select_249([e, a, b, d, f, g, h, i, j])
    }
}
/// n = 11, i = 5, a < f, a < g, b < c, b < f, c < h, c < j, d < g, e < h, e < k, f < h, g < i
fn select_237([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11]) -> usize {
    if f < g {
        select_238([a, f, d, c, e, g, i, h, j, k])
    } else {
        select_247([a, d, b, g, c, e, f, i, j, k])
    }
}
/// n = 12, i = 5, a < b, a < k, b < f, b < j, c < f, c < l, d < g, d < k, e < g, f < h, g < i
fn select_225([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < k {
        select_226([a, b, c, d, f, e, g, k, h, i, j, l])
    } else {
        select_237([d, a, b, e, c, k, g, f, i, j, l])
    }
}
/// n = 12, i = 5, a < b, a < i, b < e, b < h, c < e, c < k, d < f, d < i, e < g, f < j
fn select_205([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12]) -> usize {
    if f < l {
        select_206([a, d, b, c, f, e, g, h, i, j, l, k])
    } else {
        select_225([a, b, c, d, l, e, f, g, j, h, i, k])
    }
}
/// n = 14, i = 6, a < c, a < g, b < c, b < k, c < e, c < j, d < e, d < l, e < h, e < i, f < h, f < m, g < k
fn select_135([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    if f < g {
        select_136([a, b, c, d, f, e, g, h, i, k, j, l, m, n])
    } else {
        select_205([b, c, d, g, e, f, i, j, k, m, l, n])
    }
}
/// n = 14, i = 6, a < c, a < j, b < c, b < k, c < e, c < i, d < e, d < l, e < g, e < h, f < g, f < m
fn select_134([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14]) -> usize {
    let (a, b, j, k) = if j < k {
        (a, b, j, k)
    } else {
        (b, a, k, j)
    };
    select_135([a, b, c, d, e, f, j, g, h, i, k, l, m, n])
}
/// n = 15, i = 6, a < c, a < g, a < k, b < c, b < l, c < f, c < i, d < f, d < m, e < g, e < n, f < h, g < j
fn select_15([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15]) -> usize {
    if c < g {
        select_16([a, b, c, d, e, f, g, h, j, i, k, l, m, n, o])
    } else {
        select_134([a, e, g, b, c, d, f, i, j, k, n, l, m, o])
    }
}
/// n = 16, i = 7, a < c, a < m, b < d, b < n, c < g, c < k, d < h, e < g, e < o, f < h, f < p, g < i, h < j
fn select_14([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (c, b, d, f, e, h, g, j, l, i, k, n, p, o, m) = if c < d {
        (c, b, d, f, e, h, g, j, l, i, k, n, p, o, m)
    } else {
        (d, a, c, e, f, g, h, i, k, j, l, m, o, p, n)
    };
    select_15([c, b, d, f, e, h, g, j, l, i, k, n, p, o, m])
}
/// n = 16, i = 7, a < b, b < f, b < j, c < f, c < n, d < g, d < o, e < h, e < p, f < i, g < k, h < l
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (d, g, e, h, l, k, o, p) = if g < h {
        (d, g, e, h, l, k, o, p)
    } else {
        (e, h, d, g, k, l, p, o)
    };
    select_14([a, d, b, g, c, e, f, h, i, l, j, k, m, o, n, p])
}
/// n = 16, i = 7, a < b, a < l, b < e, c < e, c < m, d < f, d < n, e < i, f < k, g < o, h < p
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (g, h, p, o) = if g < h {
        (g, h, p, o)
    } else {
        (h, g, o, p)
    };
    select_13([a, b, c, d, g, e, f, h, i, j, k, p, l, m, n, o])
}
/// n = 16, i = 7, a < b, a < k, b < d, c < d, c < l, d < i, e < m, f < n, g < o, h < p
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (e, f, n, m) = if e < f {
        (e, f, n, m)
    } else {
        (f, e, m, n)
    };
    select_12([a, b, c, e, d, f, g, h, i, j, n, k, l, m, o, p])
}
/// n = 16, i = 7, a < c, a < k, b < d, b < l, c < i, e < m, f < n, g < o, h < p
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (a, c, b, d, j, i, k, l) = if c < d {
        (a, c, b, d, j, i, k, l)
    } else {
        (b, d, a, c, i, j, l, k)
    };
    select_11([a, c, b, d, e, f, g, h, j, i, k, l, m, n, o, p])
}
/// n = 16, i = 7, a < b, b < i, c < k, d < l, e < m, f < n, g < o, h < p
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (c, d, l, k) = if c < d {
        (c, d, l, k)
    } else {
        (d, c, k, l)
    };
    select_10([a, c, b, d, e, f, g, h, i, l, j, k, m, n, o, p])
}
/// n = 16, i = 7, a < i, c < k, d < l, e < m, f < n, g < o, h < p
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (a, b, j, i) = if a < b {
        (a, b, j, i)
    } else {
        (b, a, i, j)
    };
    select_9([a, b, c, d, e, f, g, h, j, i, k, l, m, n, o, p])
}
/// n = 16, i = 7, a < h, b < i, c < j, d < k, e < l, f < m, g < n
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (o, p) = if o < p {
        (o, p)
    } else {
        (p, o)
    };
    select_8([a, b, c, d, e, f, g, o, h, i, j, k, l, m, n, p])
}
/// n = 16, i = 7, a < g, b < h, c < i, d < j, e < k, f < l
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (m, n) = if m < n {
        (m, n)
    } else {
        (n, m)
    };
    select_7([a, b, c, d, e, f, m, g, h, i, j, k, l, n, o, p])
}
/// n = 16, i = 7, a < f, b < g, c < h, d < i, e < j
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (k, l) = if k < l {
        (k, l)
    } else {
        (l, k)
    };
    select_6([a, b, c, d, e, k, f, g, h, i, j, l, m, n, o, p])
}
/// n = 16, i = 7, a < e, b < f, c < g, d < h
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (i, j) = if i < j {
        (i, j)
    } else {
        (j, i)
    };
    select_5([a, b, c, d, i, e, f, g, h, j, k, l, m, n, o, p])
}
/// n = 16, i = 7, a < d, b < e, c < f
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (g, h) = if g < h {
        (g, h)
    } else {
        (h, g)
    };
    select_4([a, b, c, g, d, e, f, h, i, j, k, l, m, n, o, p])
}
/// n = 16, i = 7, a < c, b < d
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (e, f) = if e < f {
        (e, f)
    } else {
        (f, e)
    };
    select_3([a, b, e, c, d, f, g, h, i, j, k, l, m, n, o, p])
}
/// n = 16, i = 7, a < b
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (c, d) = if c < d {
        (c, d)
    } else {
        (d, c)
    };
    select_2([a, c, b, d, e, f, g, h, i, j, k, l, m, n, o, p])
}
/// n = 16, i = 7
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16]) -> usize {
    let (a, b) = if a < b {
        (a, b)
    } else {
        (b, a)
    };
    select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
}
