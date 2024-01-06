pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    // コピーされる場合はエラーにならない
    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);

    // 文字列リテラルをチェックする
    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);

    // deep copy
    let s3 = String::from("hello");
    let s4 = s3.clone();

    let s5 = String::from("hello");
    println!("stack address of v1 is: {:p}", &s5);
    println!("stack address of v1 is: {:p}", s5.as_ptr());
    println!("stack address of v1 is: {}", s5.len());
    println!("stack address of v1 is: {}", s5.capacity());
    take_ownership(s5);
    // println!("{}", s5) s5の所有権はs5に移ったので、この時点ではもうs5にはアクセスできなくなっている

    let s6 = String::from("hello");
    println!("stack address of v1 is: {:p}", &s6);
    println!("stack address of v1 is: {:p}", s6.as_ptr());
    println!("stack address of v1 is: {}", s6.len());
    let s7 = take_giveback_ownership(s6); // s6の所有権がs7にmoveする
    println!("stack address of v1 is: {:p}", &s7);
    println!("stack address of v1 is: {:p}", s7.as_ptr()); // 実データはs6と全く同じになる
    println!("stack address of v1 is: {}", s7.len());

    let s8 = String::from("hello"); // イミュータブルなリファレンス
    let len = calclate_length(&s8); // 参照している
    println!("The length of '{}' is {}.", s8, len);

    let mut s9 = String::from("hello"); // ミュータブルなリファレンス
    change(&mut s9);
    println!("{}", s9); // 参照して、値を変更する

    // リファレンス(参照)には、いくつかのルールがある
    // 1. イミュータブルなリファンスは、何個でも作ることができる
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // 2. イミュータブルなリファレンスと、ミュータブルなリファレンスは、共存することができない
    // let mut s10 = String::from("hello");
    // len r1 = &s10;
    // len r2 = &mut s10;
    // println!("{} {}",  r1, r2);

    // 3. イミュータブルなリファレンスの領域で、親の値を使うことができない
    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    // println!("{}", s11); // r1の領域なので、ここで使うとエラーになる
    println!("{}", r1);
    println!("{}", s11); // 一つ前でr1を使用して参照し終わっているので、ここでs11が使える
    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} and {}", r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("hello_updated");
    println!("{}", s12);
}

fn take_ownership(s: String) {
    println!("stack address of v1 is: {:p}", &s);
    println!("stack address of v1 is: {:p}", s.as_ptr()); // 値が丸ごとs5から引き継がれる
    println!("stack address of v1 is: {}", s.len());
    println!("stack address of v1 is: {}", s.capacity());
    println!("{}", s); // このsが抜けるタイミングで、s5の実データもヒープから解放される
}

// Rustでは、returnを明示的に書かず、セミコロンがないものがreturn値として判定される
fn take_giveback_ownership(s: String) -> String {
    s
}

fn calclate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}
