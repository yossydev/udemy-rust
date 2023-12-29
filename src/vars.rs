use std::usize;

pub mod sub_a;
mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("here is vars modules");
    // sub_a::func_a();
    // sub_b::func_a();
    let mut x = 5;
    println!("Value: {}", x);
    x = 6;
    println!("Value: {}", x);
    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("memory address of const is: {:p}", &MAX_POINTS);

    // stackでどこの番地に保存されているか確認する
    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("stack address of const is: {:p}", &i2);
    println!("stack address of const is: {:p}", &i3);

    // シャドーイング 後からletで定義すると、新しいstackの領域に保存されるようになる
    let y = 5; // 後からlet定義されたyが優先されるので、この値は隠れてしまうらしい
    println!("stack address of const is: {:p}", &y);
    let y = y + 1;
    println!("stack address of const is: {:p}", &y);
    let y = y * 2;
    println!("stack address of const is: {:p}", &y);
    println!("last value: {}", &y);
    {
        // このスコープを抜けると12に戻る
        let y = 0;
        println!("the value: {}", &y);
    }
    println!("last value: {}", &y);

    // タプル型 かっこで、複数の値を定義できる。この時にデータ型でもいい
    // 固定の文字列を入れた場合は、文字列スライス型になる(&str)
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("タプル型へのアクセス方法: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    // 0, 1を上書きしたい時には「参照剥がし」を使う
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2); // 複雑なデータ型は:?が必要

    // 配列
    // Rustでは配列の値は決まっていないといけない。そのため全てStackに積まれていく
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]); // [1, 2, 3, 4, 5] [0, 0, 0, 0, 0, 0, 0, 0, 0, 0] 3 4
}
