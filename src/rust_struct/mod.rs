#[derive(Debug)]
struct User {
    name: String,
    age: i16,
    addr: String,
}

pub fn run() {
    let user = get_user(String::from("hi"), 20);
    let user2 = get_user(String::from("hi2"), 30);
    println!("struct: {:#?}", user);

    // 소유권 이전됨
    let user3 = sum_object(&user, &user2);
    println!("struct-3 : {:#?}", user3);

    // 실행시 에러 발생, 소유권이 user3으로 이동되어서 실행 안됨. 그래서 불변 참조자로 참조만 하게함
    println!("struct: {:#?}", user);
    println!("struct: {:#?}", user2);

    let user2_name = user2.get_new_name();
    println!("get user name: {:#?}", user2_name);

    let sum_age = user2.get_age();
    println!("sum user age: {}", sum_age);
    println!("change ?: {:#?}", user2);
}

// 변수명과 프로퍼티명이 같을 때에는 축약 표현 가
fn get_user(name: String, age: i16) -> User {
    User {
        name,
        age,
        addr: String::from("seoul"),
    }
}

// TODO 오브젝트 클론은 어떻게 하는거지..
fn sum_object(a: &User, b: &User) -> User {
    User {
        name: a.name.clone(),
        addr: b.addr.clone(),
        age: b.age,
    }
}

// 구조체 메소드 정의 가능
// 레퍼런스 타입은 참조 타입으로 넘거야지만 기존 데이터를 변경되지 않게 할 수 있음
// 나머지 타입들은 값이 다른 스코프로 이동해도 어차피 스택에서 복사될 것이기 때문에 기존 값에 영향을 미치지 않음
impl User {
    fn sum_age(&self, age: i16) -> i16 {
        age * 2
    }

    fn get_age(&self) -> i16 {
        self.sum_age(self.age)
    }

    // 만약 리턴시 format으로 새로운 String을 생성하고 참조자만 리턴한다면 라이프 사이클을 지정해줘야 한다. 왜냐면 소유권을 넘기지 않고 함수를 종료하게 되면 참조자도 값을 참조하지 못하기 떄문이다
    fn get_new_name(&self) -> String {
        format!("hi, {}", &self.name)
    }
}
