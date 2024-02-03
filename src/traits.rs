trait Fruits {
    fn price(&self) -> u32;
}
struct Apple;

impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}
trait Summary {
    // 処理を書くとそれがデフォルトの処理になる
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    // 処理を書くと上書きをした状態になる
    // fn summarize(&self) -> String {
    //    format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}
impl Message for NewsArticle {}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

pub fn run() {
    // インスタンスを作る
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweer: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Penguins win the Stenley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("1 new tweer: {}", article.summarize());
    notify(&article);
    notify_another(&article);
    // the trait bound `Tweet: Message` is not satisfiedthe trait `Message` is implemented for `NewsArticle` 81:20:1 rustc E0277
    // required by a bound introduced by this call 81:5:1 rustc E0277
    // Symmary traitを持っていないのに引数に渡しているのでエラーになっている
    // notify_another(&tweet);
}

fn get_price<T: Fruits>(fruits: T) {
    // Fruitsというインスタンスの中にpriecというメソッドが必ず存在するので実行が可能
    println!("price is: {}", fruits.price());
}

// 引数はSummary traitを実装しているデータ型であればitemに渡すことができる
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message! {}", item.message());
}
