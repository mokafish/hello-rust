pub fn main() {
    println!("--- s07_structs ---");
    let mut _user1 = User {
        _active: true,
        _username: String::from("someusername123"),
        _email: String::from("someone@example.com"),
        _sign_in_count: 1,
    };

    _user1._email = String::from("anotheremail@example.com");

    let _user2 = User {
        _email: String::from("another@example.com"),
        .._user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(5, 0, 5);
    let _subject = AlwaysEqual;
    println!("{} {} {}", _black.0, _black.1, _black.2);
    println!("{} {} {}", _origin.0, _origin.1, _origin.2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 为：{:?}", rect1);
    println!("rect1 为：{:#?}", rect1);

    let scale = 2;

    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    let rect3 = Rectangle::square(20);
    dbg!(&rect3);
    
    println!("该矩形的面积为 {} 平方像素。", rect1.area());
    println!("rect1 可以容纳 rect2 吗？{}", rect1.can_hold(&rect2));
    println!("rect1 可以容纳 rect3 吗？{}", rect1.can_hold(&rect3));
}

struct User {
    _active: bool,
    _username: String,
    _email: String,
    _sign_in_count: u64,
}

// 没有命名字段
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 不带任何字段的类单元值结构体
struct AlwaysEqual;

// 添加属性以派生 Debug 特质
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.width > other.height && self.height > other.width)
    }

    // 关联函数（构造函数）
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
