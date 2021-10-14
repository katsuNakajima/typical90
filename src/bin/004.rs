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
    let (h, w) = parse_line!(usize, usize);
    let mut mat = Vec::new();
    let mut col_sum = Vec::new();
    let mut low_sum = vec![0; w];
    for _ in 0..h {
        let col_num = parse_vec!(i32);
        mat.push(col_num.clone());
        let sum_c = col_num.iter().fold(0, |sum, i| sum + i);
        col_sum.push(sum_c);
        for j in 0..w {
            low_sum[j] += col_num[j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{} ", col_sum[i] + low_sum[j] - mat[i][j]);
        }
        println!();
    }
}
