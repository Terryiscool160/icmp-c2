pub fn xor(mut a: u8, mut b: u8) -> u8 {
    let mut p = 1;
    let mut c = 0;

    while a > 0 && b > 0 {
        let ra = a % 2;
        let rb = b % 2;

        match ra != rb {
            true => c += p,
            false => (),
        }

        a = (a - ra) / 2;
        b = (b - rb) / 2;

        p = p * 2;
    }

    match a < b {
        true => a = b,
        false => (),
    }

    while a > 0 {
        let ra = a % 2;

        match ra > 0 {
            true => c += p,
            false => (),
        }

        a = (a - ra) / 2;
        p = p * 2;
    }

    c
}
