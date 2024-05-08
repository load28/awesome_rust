pub fn run() {
    create_string();
    update_string();
    string_index();
}

fn create_string() {
    println!("create string by String::new => {}", String::new());
}

fn update_string() {
    let mut s = String::from("foo");
    let str = "bar";
    s.push_str(str);
    // 하나의 문자열을 추가함
    s.push('l');

    // push_str 함수가 str의 소유권을 가져가지 않기 때문에 여전히 str을 사용할 수 있음
    println!("update string => {}", str);
    println!("update string => {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // + 연산자는 add 메서드를 호출하고, add 메서드는 self와 다른 문자열을 가져와서 새로운 문자열을 반환함
    // add 함수는 string slice를 인자로 갖지만 String 타입을 인자로 받을 수 있음, 왜냐하면 String 참조가 string slice 참조로 자동 변환되기 때문임
    // 이것은 역참조 강제 변환(deref coercion)이라고 불리는 러스트의 특성임 참조로 변경되면서 소유권이 이동되지 않음
    let s3 = s1 + &s2;
    // s1은 더 이상 사용할 수 없음
    // println!("update string => {}", s1);
    println!("update string => {}", s3);

    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    // format! 매크로는 string을 string slice로 내부적으로 변환하여 새로운 문자열을 반환하기 때문에 소유권이 이동되지 않음
    let format_string = format!("{}-{}", s3, s4);
    println!("update string => {}", format_string);
}

/**
 * 러스트에서 문자열 인덱싱 접근방식은 지원하지 않기 때문에 에러를 발생시킴
 * 이유는 러스트의 문자열은 vec<u8>로 구현되어 있기 때문에 인덱싱을 사용하면 바이트 값을 가져옴
 * 유니코드에서는 문자마다 차지하는 바이트 수가 다르기 때문에 인덱싱을 사용하면 문자가 깨질 수 있음
 *
 * 러스트에서는 바이트 -> 유니코드 스칼라 -> 문자소 클러스터 형태로 구성됨
 * 바이트는 유니코드의 바이트를 나타내며 바이트들이 모여 하나의 유니코드 스칼라를 이루고, 유니코드 스칼라들이 모여 문자소 클러스터를 이룸
 * (유니코드 스칼라에는 발음기호 등이 있어 온전한 문자가 아님)
 */
fn string_index() {
    let s = String::from("hello");
    // println!("string index => {}", &s[0]);
    // 범위를 지정하면 인덱스로 접근이 가능하나 유니코드는 언어에 따라 차지하는 바이트 수가 다르므로 깨질 수 있음, 권장하지 않음
    println!("string index => {}", &s[0..1]);

    let hello = "Здравствуйте";
    // bytes 함수는 바이트 값을 반환함
    for b in hello.bytes() {
        println!("string index => {}", b);
    }

    // chars 함수는 유니코드 스칼라 값을 반환함
    for c in hello.chars() {
        println!("string index => {}", c);
    }
}
