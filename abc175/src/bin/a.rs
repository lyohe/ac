// https://atcoder.jp/contests/abc175/tasks/abc175_a
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut max = 0;
    let mut cnt = 0;

    for e in buf.chars() {
        if e == 'R' {
            cnt += 1;
            max = cnt;
        } else {
            cnt = 0;
        }
    }

    println!("{}", max);
}
