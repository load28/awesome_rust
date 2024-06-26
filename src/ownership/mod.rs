pub fn run() {
    ownership();
}

// 소유권은 러스트의 가장 큰 특성으로 가비지 컬렉션 없이 메모리의 안정성을 보장해주는 방법이다
pub fn ownership() {
    // 소유권 원칙 - 1. 러스트는 각각의 값에 오너라고 불리는 변수를 가지고 있다., 2. 한번에 딱 하나의 오너만 존재함. , 3. 오너가 스코프 밖을 벗어나면 값을 버려진다.
    let s = "hello"; // 스트링 리터럴 (문자열은 고정된 크기 - 컴파일 타입에서 고정된 사이즈를 가지고 있기 때문에 스택에 저장 !, String 타입은 가변성을 지니므로 힙에 저장 - 컴파일 타입에 크기를 알 수 없음)
    let mut s2 = String::from("hello"); // String 타입,
    s2.push_str(", world");
    println!("String type = {}", s2);
    // 컴파일 타임에 알수 있는 고정적인 문자열은 스트링 리터럴로 만드는게 좋음, 가변적인 문자열은 String 타입으로 작성하는 것이 효율적..어차피 스코프 벗어나면 값이 버려지기 때문에 메모리 안정성 보장... 물론 참조자로 지속적으로 오너를 가지고 있는 코드라면 버려지지 않음

    let s3 = String::from("s3");
    // s3의 소유권은 s4로 이동하게 되면 s3는 사용하지 못하게 된다. 내부적으로 카피 트레잇을 구현하면 사용 가능하다. String 타입은 카피 트레잇이 존재하지 않는다.
    // let s4 = s3;
    let s4 = s3.clone(); // 클론을 통해서 깊은 복사를 수행 할 수 있음
    println!("{}", s3);

    let x = 10;
    let y = x;
    println!("{}", x); // 스택에 쌓이는 데이터는 소유권을 가질 필요가 없기 때문에 x는 여전히 사용 가능함 (정수형, 부울린, 부동소수점, Copy 트레잇이 구성된 튜플들)

    let moved = String::from("take");
    // 해당 함수를 실행하면 소유권이 함수로 이전됨, 그래서 moved 변수는 버려지므로 컴파일 에러가 발생
    // take_ownership(moved);
    println!("moved: {}", moved);

    let borrowing = String::from("borrowing");
    borrowing_fun(&borrowing); // 불변 참조연산자를 통해서 소유권을 이동시키지 않음
    println!("borrowing 그대로 사용 할수 있음: {}", borrowing);

    let mut mut_borrowing = String::from("mut");
    mut_fun(&mut mut_borrowing); // 함수 내부에서 mut_borrowing을 수정한다

    // 불변 참조자가 있는 상태에서 가변 참조자를 만들 수 없다
    // 하나의 가변 참조자 또는 임의의 불변 참조자들로만 구성 할 수 있다 - 이것을 통해서 데이터의 동기화 문제를 해결 할 수 있다
    // 또한 가변 참조자를 하나만 만드는 것과 스코프를 벗어날때에 버려지는 것은 해제된 메모리를 참조하는것을 방지한다
    let mut_r2 = &mut_borrowing;
    // let mut_r1 = &mut mut_borrowing;

    println!("mut_borrowing: {}, {}", mut_borrowing, mut_r2);
}

// some이라는 값을 다시 호출하는 쪽에서 사용하려면 some의 소유권을 반환하는 코드를 작성해야한다. 이런 코드는 상당히 보일랫한 코드를 만들게 된다
//
fn take_ownership(some: String) -> (String, i32) {
    println!("take - {}", some);
    let x = 10;
    // 소유권을 다시 돌려주기 위해서 작성함
    // some
    // 기존 소유권을 돌려주면서 새로운 값을 튜플형태로 반환하여 사용 할 수도 있지만, 이런 코드도 좋지 못하다
    (some, x)
}

// &연산자를 통해서 소유권을 이동시키지 않고 빌림으로서 사용 할 수 있다
// 주로 값을 읽어와서 다른 값을 만들어 내는 곳에 사용된다
fn borrowing_fun(some: &String) -> usize {
    println!("borrowing: {}", some);
    some.len()
}

// 가변 참조자를 통해서 소유권을 가지지 않고 값을 수정 할 수 있다
fn mut_fun(some: &mut String) {
    some.push_str("plus");
}

// 스트링 슬라이스는 String 타입의 참조 연산자를 통해서 데이터를 가져온다 (가변 참조 연산자...이다)

fn slice() {
    let s = String::from("slice");
    let iter = "iter";
    let x = &s[0..2];
    slice_fn(iter); // 가능
    slice_fn(&s[0..]); // 슬라이스를 통해서 가능
    slice_fn(x);

    let a = [1, 2, 3, 4]; // 메모리상 하나만 존재하며 그 값을 고정된 크기만큼 불변적으로 참조하는 연산자를 만들 수 있는 것이 슬라이스
    let b = &a[1..]; // 스트링 리터럴이 불변인 이유는 불변 참조자를 통해서 슬라이스가 만들어지기 때문이다
}

// 즉 &str 타입의 파라미터에 String 타입과 String 리터럴을 모두 받을 수 있게 된다
fn slice_fn(str: &str) {
    println!("str: {}", str);
}

fn error_handler() {
    let num = default_str("222232323");

    println!("{}", num);
}

fn default_str(num_str: &str) -> i32 {
    // 예외 발생시 특정 값으로 맵핑 할 수 있음
    String::from(num_str).parse().unwrap_or(0)
    // 대부분의 함수는 Result 인터페이스를 따름, 그리고 매치 표현식으로 처리해서 성공, 실패에 대해여 처리,
    // 실패 또는 성공에 대해서 로직이 늘어나면 매치로 처리하는게 좋아보임, 간단한 기본값을 가지는 경우 unwrap_or으로 처리하는게 좋음
    // match String::from(num_str).parse() {
    //   Ok(num) => {
    //     num
    //   },
    //   Err(e) => {
    //     0
    //   }
    // }
}
