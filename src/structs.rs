#[derive(Debug)] // logで確認するためには必要
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // logで確認するためには必要
struct Rectangle {
    width: u32,
    height: u32,
}
// 構造体にメソッドを追加する
impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
    // &selfではなくselfにすると、所有権が移動してしまうので、インスタンスが使えなくなってしまう。
    // fn area(&self){
    //  println!("{}", self.width * self.height);
    // }
}

pub fn run() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someysername123"),
        active: true,
        sign_in_count: 1,
    };
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someysername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{:#?}", user1);
    let user2 = build_user(String::from("user2@xxx.com"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
