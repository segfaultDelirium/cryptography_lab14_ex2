use math::round;

fn shanks(n: u128, alfa: u128, beta: u128) -> u128 {
    let m: u128 = round::ceil(f64::sqrt(n as f64), 0) as u128;
    let mut l1: Vec<(u128, u128)> = (0..m)
        .into_iter()
        .map(|j| (j, potegowanie(alfa, m * j, n)))
        .collect();
    // l1.sort_by(|x, y| x.1 < y.1);
    // println!("l1 = {:?}", l1);
    l1.sort_by(|x, y| (x.1).partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal));
    // println!("l1 sorted = {:?}", l1);

    let mut l2: Vec<(u128, u128)> = (0..m)
        .into_iter()
        .map(|i| {
            // let odwrotnosc = odwrotnosc_multiplikatywna(alfa as i128, n as i128) as u128;
            let value = modulo_euclid(
                (beta
                    * odwrotnosc_multiplikatywna(potegowanie(alfa, i, n) as i128, n as i128)
                        as u128) as i128,
                n as i128,
            ) as u128;
            // println!("odwrotnosc = {}", odwrotnosc);
            (i, value)
        })
        .collect();

    l2.sort_by(|x, y| (x.1).partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal));

    let found_pairs: Vec<((u128, u128), (u128, u128))> = l1
        .into_iter()
        .map(|(j, y_a)| {
            let found_match = l2.clone().into_iter().find(|(i, y_b)| y_a == *y_b);
            if let Some((i, y_b)) = found_match {
                return Some(((j, y_a), (i, y_b)));
            }
            None
        })
        .flatten()
        .collect();

    // let zipped = l1.into_iter().zip(l2.into_iter());
    // zipped.clone().take(10).for_each(|(pair)| {
    //     println!("pair = {:?}", pair);
    // });
    // // println!("l2 sorted = {:?}", l2);
    // let found_pairs = zipped.clone().find(|(pair_a, pair_b)| pair_a.1 == pair_b.1);

    println!("found pairs = {:?}", found_pairs);
    if let Some(((j, y_a), (i, y_b))) = found_pairs.get(0) {
        let res = modulo_euclid((m * j + i) as i128, n as i128);
        return res as u128;
    }

    // if let Some(pairs) = found_pairs {
    //     let ((i, y_a), (j, y_b)) = pairs;
    //     // i ma wyjsc 141, j ma wyjsc 114
    //     // y ma wyjsc 19999
    //     let res = modulo_euclid((m * j + i) as i128, n as i128);
    //     return res as u128;
    // }
    123
}

fn ex2_a() {
    println!("a)");
    let alfa = 106;
    let beta = 12375;
    let n = 24691;
    // y ma wyjsc 19999

    let shanks_res = shanks(n, alfa, beta);
    println!("shanks res = {:?}", shanks_res);
}

fn ex2_b() {
    println!("b)");
    let alfa = 6;
    let beta = 248388;
    let n = 458009;

    let shanks_res = shanks(n, alfa, beta);
    println!("shanks res = {:?}", shanks_res);
}

fn main() {
    // let n = 113;
    // let sqrt_res = f64::sqrt(n as f64);
    // println!("sqrt_res = {sqrt_res}");
    // let m = round::ceil(f64::sqrt(n as f64), 0);

    // let alfa = 106;
    // let beta = 12375;
    // let n = 24691;

    // let shanks_res = shanks(n, alfa, beta);
    // println!("shanks res = {:?}", shanks_res);

    ex2_a();
    ex2_b();
    // println!("m = {m}");
    println!("Hello, world!");
}

// fn potegowanie(a: u128, e: u128, n: u128) -> u128 {
//     let e_binary = reverse(create_binary(e));
//     let mut d = 1;
//     let mut i = e_binary.len() as i32 - 1;
//     while (i >= 0) {
//         d = modulo_euclid(d * d, n as i128);
//         if e_binary[i as usize] == 1 {
//             d = modulo_euclid(d * a as i128, n as i128)
//         }

//         i -= 1;
//     }
//     return d as u128;
// }

fn potegowanie(a: u128, e: u128, n: u128) -> u128 {
    let e_binary = reverse(create_binary(e));
    let mut d = 1;
    let mut i = e_binary.len() as i32 - 1;
    while (i >= 0) {
        d = modulo_euclid(d * d, n as i128);
        if e_binary[i as usize] == 1 {
            d = modulo_euclid(d * a as i128, n as i128)
        }

        i -= 1;
    }
    return d as u128;
}

fn create_binary(value: u128) -> Vec<u128> {
    let binary_string = format!("{:b}", value);
    let res = binary_string
        .chars()
        .into_iter()
        .map(|c| if c == '0' { 0 } else { 1 })
        .collect();
    return res;
}

fn reverse(vec: Vec<u128>) -> Vec<u128> {
    let mut vec_clone = vec.clone();
    vec_clone.reverse();
    // vec_clone.sort_by_key(|&num| (false , Reverse(num)));
    return vec_clone;
}

fn modulo_euclid(j: i128, k: i128) -> i128 {
    let res = j % k;
    if res < 0 {
        return res + k;
    } else {
        return res;
    }
}

fn rozNWD(j: i128, k: i128) -> (i128, i128, i128) {
    if j == 0 {
        return (k, 0, 1);
    }
    // let r = k % j;
    let r = modulo_euclid(k, j);
    // let r = k % j;
    let (d, x_prim, y_prim) = rozNWD(r, j);
    let x = y_prim - (k / j) * x_prim;
    let y = x_prim;
    return (d, x, y);

    // (k, 1, 0)
}

fn odwrotnosc_multiplikatywna(j: i128, k: i128) -> i128 {
    // a to jest 17 czyli j
    // n = 101 czyli k
    let (_d, x, _y) = rozNWD(j, k);
    // println!("d = {d}, x = {x}, y = {y}");
    return modulo_euclid(x, k);
}
