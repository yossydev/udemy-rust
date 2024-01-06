// 構造体に対するジェネリクス
struct Point<T> {
    x: T,
    y: T,
}
struct PointAnother<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointAnother<T, U> {
    // selfについて
    // 一つの構造体に対して、インスタンスは複数生成できる
    // mixupというメソッドが実行されるときにどのインスタンスから実行されるのかということを識別するためにselfのインスタンスの情報が必要になるjj
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];

    // 最大の処理を検索する処理
    // let mut largest = number_list[0];
    // for number in number_list {
    //    if number > largest {
    //        largest = number;
    //    }
    // }
    // println!("The largest is {}", larget_i32(number_list)) // 100

    // char型のリストの最大値を求める
    let char_list = vec!["a", "b", "c", "d"];
    println!("{}", largest(char_list)); // 100
    println!("{}", largest(number_list)); // 100

    // 構造体に対するインスタンスを作っている
    let p1 = Point { x: 1, y: 2 };
    // let p2 = Point { x: 1.0, y: 2 }; // xとyは同じ型を持つ必要があるのでエラーになる
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = PointAnother { x: 5, y: 10.4 };
    let p4 = PointAnother { x: "Rust", y: "a" };
    let p5 = p3.mixup(p4);
    println!("{} {}", p5.x, p5.y);
}

fn larget_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        // if item > largest { // Tだけだと、Tの範囲が広すぎるのでエラーになる
        if item > largest {
            largest = item;
        }
    }
    largest
}
