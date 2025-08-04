use std::fmt::Pointer;

pub fn main() {
    println!("--- s08_enums ---");
    println!("Todo ...");
    // 创建不同的交通信号实例
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green(30);
    let broken = TrafficLight::Broken;
    
    for x in [&red, &yellow, &green, &broken].iter() {
        match x {
            TrafficLight::Green(n) => println!("绿灯时间: {n}秒"),
            oth => println!("不是绿灯 {}", oth.noop()),
        }
    }

    // 示例1: 使用 match 处理所有枚举变体
    println!("::: 使用 match 处理所有情况 :::");
    check_light(&red);
    check_light(&yellow);
    check_light(&green);
    check_light(&broken);

    // 示例2: 使用 if let 处理特定变体
    println!("\n::: 使用 if let 提取数据 :::");
    extract_timer(&green);
    extract_timer(&red); // 不会触发处理

    // 示例3: 使用 matches! 宏检查变体
    println!("\n::: 使用 matches! 宏检查状态 :::");
    println!("绿灯是否工作? {}", green.is_working());
    println!("故障灯是否工作? {}", broken.is_working());
}

// 定义一个表示交通信号的枚举
enum TrafficLight {
    Red,       // 无附加数据
    Yellow,    // 无附加数据
    Green(u8), // 附加倒计时秒数
    Broken,    // 无附加数据
}

// 为枚举实现方法
impl TrafficLight {
    // 检查信号是否正常工作
    fn is_working(&self) -> bool {
        !matches!(self, TrafficLight::Broken)
    }
    
    fn noop(&self) -> &str {""}
}

// 使用 match 表达式处理枚举
fn check_light(light: &TrafficLight) {
    match light {
        TrafficLight::Red => println!("红灯: 停止!"),
        TrafficLight::Yellow => println!("黄灯: 准备!"),
        // 解构带数据的变体
        TrafficLight::Green(seconds) => println!("绿灯: 可通过! 剩余时间: {}秒", seconds),
        // _ 通配符处理剩余情况
        _ => println!("信号灯故障, 请谨慎通行!"),
    }
}

// 使用 if let 处理特定枚举变体
fn extract_timer(light: &TrafficLight) {
    // 只处理 Green 变体并提取数据
    println!("extract_timer");
    if let TrafficLight::Green(time) = light {
        println!("检测到绿灯倒计时: {}秒", time);
    }
    // 其他变体自动忽略
}
