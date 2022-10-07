// 2022-10-06
// https://codeforces.com/problemset/problem/1738/B?locale=ru

fn read_line_as_str() -> String {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).expect("failed to read line");
    inp
}

fn str_to_int(inp: String) -> i32 {
    inp.trim().parse::<i32>().unwrap()//.expect("not int")
}

fn str_to_vec(inp: String) -> Vec<i32> {
    let mut res = Vec::new();

    for num in inp.split_whitespace() {
        res.push(str_to_int(num.to_string()));
    }

    res
}

fn main() -> std::io::Result<()> {
    // let mut i= "".to_string();
    // if std::io::stdin().read_line(&mut i).is_ok(){
    //     print!("{}", i);
    // }

    // let read_line_as_str = || std::io::stdin().read_line(&mut inp).expect("Failed to read line");
    // let str_int = |inp: String| inp.trim().parse().expect("not integer");

    // TODO:
    //  + (1) read number of elems
    //  + (2) read n & k
    //  + (3) read k numbers

    // number of datasets
    let t: i32 = str_to_int(read_line_as_str());

    // println!("{}", t)

    let mut answ = String::new();

    for _dataset in 0..t {
        let mut inp_vec = str_to_vec(read_line_as_str());
        let n = inp_vec[0];
        let k = inp_vec[1];

        inp_vec = str_to_vec(read_line_as_str());

        let delta: &mut i32 = &mut 0;
        let mut res = true;

        // todo: + (1) iterate over vector starting with 2nd element
        // (2) compare cur delta and prev delta

        for i in 1..k {
            let cur_summ: &i32 = &inp_vec[i as usize];
            if (&inp_vec[i as usize] - &inp_vec[i as usize - 1] < *delta) && (i != 1) { res = false };
            *delta = &inp_vec[i as usize] - &inp_vec[i as usize - 1]; // cur elem of the summ
            let expect: f64 = *cur_summ as f64 / (n - k + i + 1) as f64;
            if expect > *delta as f64 { res = false; }
            if !res { break; }
        }

        answ.push_str(if res { "yes\n" } else { "no\n" });
    }

    println!("{}", format!("{}", answ.trim()));
    Ok(())
}
