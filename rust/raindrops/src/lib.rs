fn factors(n: u32) -> Vec<u32> {
    let mut n = n;
    let mut i = 2;
    let mut ret: Vec<u32> = Vec::new();
    while i <= n {
        if n % i == 0 {
            n /= i;
            ret.push(i);
        } else {
            i += 1;
        }
    }
    ret

}

pub fn raindrops(n: u32) -> String {
    let factors = factors(n);
    let mut ret = String::new();
    if factors.contains(&3) {
        ret.push_str("Pling");
    }
    if factors.contains(&5) {
        ret.push_str("Plang");
    }
    if factors.contains(&7) {
        ret.push_str("Plong");
    }
    if ret.is_empty() {
        ret.push_str(&n.to_string());
    }
    ret
}
