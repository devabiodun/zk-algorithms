use mod_exp::mod_exp;
use rand::Rng;

fn main() {
    let n = 89;
    let (s, d) = find_s_and_d(n - 1);

    println!("s = {}, d = {}", s, d);

    let k = 5; // Number of iterations

    let mut prime = true;
    for _ in 0..k {
        let a = generate_random_number(&n);
        let mut x = mod_exp(a, d, n);
        if x != 1 && x != n - 1 {
            let mut i = 1;
            while i < s && x != n - 1 {
                x = (x * x) % n;
                if x == 1 {
                    prime = false;
                    break;
                }
                i += 1;
            }
            if x != n - 1 {
                prime = false;
            }
        }
    }

    if prime {
        println!("{} is probably prime", n);
    } else {
        println!("{} is composite.", n);
    }
}

pub fn generate_random_number(n: &u64) -> u64 {
    rand::thread_rng().gen_range(2..n - 2)
}

fn find_s_and_d(mut n: u64) -> (u64, u64) {
    let mut s = 0;
    while n % 2 == 0 {
        n /= 2;
        s += 1;
    }
    (s, n)
}
