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
    let (n, m) = parse_line!(usize, usize);
    let mut vt = Vec::new();
    let mut ans = 0;
    for _i in 0..n {
        vt.push(Vec::new());
    }
    for _i in 0..m {
        let (a, b) = parse_line!(i32, i32);
        vt[(a - 1) as usize].push(b - 1);
        vt[(b - 1) as usize].push(a - 1);
    }
    for i in 0..n {
        let mut cnt = 0;
        for j in &vt[i] {
            if j < &(i as i32) {
                cnt += 1;
            }
        }
        if cnt == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
