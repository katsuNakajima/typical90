#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let n = parse_line!(usize);
    let mut p1 = vec![0];
    let mut p2 = vec![0];
    let mut p1_sum = 0;
    let mut p2_sum = 0;
    for _ in 0..n {
        let (c_num, p_num) = parse_line!(i32, i32);
        if c_num == 1 {
            p1_sum += p_num;
        } else {
            p2_sum += p_num;
        }
        p1.push(p1_sum);
        p2.push(p2_sum);
    }
    let q = parse_line!(usize);
    let mut l = Vec::new();
    let mut r = Vec::new();
    for _ in 0..q {
        let (l_num, r_num) = parse_line!(usize, usize);
        l.push(l_num);
        r.push(r_num);
    }
    for i in 0..q {
        let left = l[i] - 1;
        let right = r[i];
        let ans1 = p1[right] - p1[left];
        let ans2 = p2[right] - p2[left];
        println!("{} {}", ans1, ans2);
    }
}
