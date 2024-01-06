pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    let st3 = String::from("x");
    // let res2; // 初期値も与えていないし方も書いていないのに補完が効くのはコンパイル時に逆算されているため
    {
        // st3に比べるとライフタイムが短い
        let st4 = String::from("y");
        let res2 = get_longest(&st3, &st4);
        // res2 = get_longest(&st3, &st4); // res2の値はライフタイムが短いst4の値になるが、res2が使用されているのがライフタイムの外側なのでダングリングポインタが発生してしまっている
        println!("{}", res2);
    }
}

// st1とst2の参照した値を引数で受け取れるようにしている
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > x.len() {
        x
    } else {
        y
    }
}

// fn dummy1<'a>() -> &'a str {
//     let s = String::from("demo");
//     &s // sがドロップされてしまうので、リファレンスが一人歩きしてしまうとダングリングポイントが発生してしまうのでエラーになる
// }

// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     &x
// }

fn dummy3<'a>() -> String {
    let s = String::from("demo");
    s
}
