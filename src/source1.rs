const N: usize = 100;

pub fn example_bench1(a_: u64) -> u64 {
    let mut a = [0_u64; N];
    let mut b = [0_u64; N];
    let mut c = [1_u64; N];
    let mut d = [1_u64; N];

    for i in 0..N {
        a[i] = a_ * (i as u64) * 2 - 2;
        b[i] = (i as u64) + 3 - a[i];
    }

    for _ in 0..N {
        for j in 0..N {
            for i in 0..N {
                c[i] += a[i] + b[i] * d[j];
            }

            for i in 0..N {
                d[i] -= c[i] + b[j] * a[i];
            }

            a[j] += c[j] * d[j];
            b[j] -= d[j] * c[j];
        }

        for i in 0..N {
            d[i] -= a[i] * b[i] + c[i];
        }
    }

    let mut sum: u64 = 0;
    for i in 0..N {
        sum += d[i] - c[i];
    }
    sum
}
