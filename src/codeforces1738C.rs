// 2022-10-10
// https://codeforces.com/problemset/problem/1738/C

fn read_line_as_str() -> String {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).expect("failed to read line");
    inp
}

fn str_to_int(inp: String) -> i32 {
    inp.trim().parse::<i32>().unwrap()
}

fn solve() {
    let inp = read_line_as_str();
    let mut n_even = 0;
    let mut n_odd = 0;

    for num in inp.split_whitespace() {
        if str_to_int(num.to_string()) % 2 == 0 {
            n_even += 1;
        } else {
            n_odd += 1;
        }
    }

    n_even = n_even % 2;
    n_odd = n_odd % 4;

    let res = match n_even + n_odd {
        4 => n_even > 2 || n_odd > 2,
        3 => n_even == 0 || n_odd == 0,
        1 | 2 => n_even != 0,
        _ => true,
    };

    println!("{}", if res { "Alice" } else { "Bob" });
}

fn main() {
    let t = str_to_int(read_line_as_str());
    for _i in 0..t {
        read_line_as_str(); // no need to remember number of elements in test
        solve();
    }
}
