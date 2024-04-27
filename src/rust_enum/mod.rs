#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

pub fn run() {
    rust_enum();
}

fn rust_enum() {
    let v4 = get_ipAddr();
    println!("V4 enum :{:#?}", v4);
    let ip = get_ip();
    println!("ip: {:#?}", ip);
    get_green();
    option();
}

// enum은 기존에 언어처럼 사용하면
fn get_ipAddr() -> IpAddrKind {
    IpAddrKind::V4
}

// enum을 이용한 구조체 반환
fn get_ip() -> IpAddr {
    IpAddr {
        addr: String::from("127.0.0.1"),
        kind: IpAddrKind::V6,
    }
}

// 이런식으로도 구성가능함
enum RGB {
    White(i8, i8, i8),
    Black(i8, i8, i8),
    Green(String),
    Palate { color: String, opacity: i8 },
}

impl RGB {
    fn get_green(&self) {
        println!("get_green");
    }
}

fn get_green() {
    let black = RGB::Black(1, 1, 1);
    black.get_green()
}

fn option() {
    let str = String::from("bye");
    let some: Option<String> = Some(str);
    let none: Option<i32> = None;
    // none 또는 some 값으로 안전하게 구분 할 수 있다
    let value: Option<String> = get_option(some);
    // println!("some: {}", some.unwrap());
    println!("option: {}", value.unwrap());
}

fn get_option(num: Option<String>) -> Option<String> {
    match num {
        Some(mut v) => {
            v.push_str("~~~");
            Some(v)
        }
        None => None,
    }
}
