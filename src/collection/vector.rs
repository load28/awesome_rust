pub fn run() {
    create_vector();
    update_vector();
    read_vector();
    iterate_vector();
    enum_vector();
}

fn create_vector() {
    // 생성 시점에 데이터를 지정
    let v: Vec<i32> = Vec::new();
    println!("create vector by Vec::new => {:?}", v);

    // 생성시에 데이터를 지정하여 제네릭 필요하지 않음
    let v_macro = vec![1, 2, 3];
    println!("create vector by vec! => {:?}", v_macro);
}

fn update_vector() {
    // vector를 변경하기 위해서는 mutable로 선언해야 함
    let mut v = Vec::new();
    // vector에 데이터를 업데이트 하기 때문에 컴파일러는 데이터를 통해 타입을 추론할 수 있음
    v.push(1);
    v.push(2);
    println!("update vector => {:?}", v);
}

fn read_vector() {
    let v = vec![1, 2, 3, 4, 5];

    // vector의 데이터를 읽기 위해서는 인덱스를 통해서 접근
    // 인덱스를 통해서 접근하는 것은 프로그램이 종료될 수 있음
    let third: &i32 = &v[2];
    println!("read_vector by index => {}", third);

    // 안전하게 데이터를 읽기 위해서는 get 메소드를 사용
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("read_vector by get function => {}", third),
        None => println!("read_vector by get function => None"),
    }

    let mut v2 = vec![1, 2, 3, 4, 5];
    let read_value = &v2[0];
    // v2.push(6); // -> 불변참조자가 println!에서 사용되기 때문에 컴파일러는 오류를 발생시킴
    // 만약 println!에서 사용되지 않는다면 컴파일러는 오류를 발생시키지 않음
    // 왜냐하면 불변 참조자가 사용되는 시점에 존재하는 것으로 컴파일러는 판단하기 때문
    println!("read_vector by immtauble value: {}", read_value);
}

fn iterate_vector() {
    let mut v = vec![1, 2, 3, 4, 5];

    // vector를 순회하기 위해서는 for문을 사용
    for i in &mut v {
        println!("iterate_vector by for loop => {}", i);
        // 역참조 연산자를 이용하여 가변 백터의 값을 변경 할 수 있음
        *i += 1;
        // v.push(6); -> 반복자 안에서 벡터를 변경하면 컴파일러는 오류를 발생시킴
    }
}

fn enum_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // vector는 생성되는 시점에 데이터의 크기를 알아야 하기 때문에 타입이 지정되어야 한다.
    // 이때 enum을 사용하여 다양한 타입을 지정할 수 있다.
    // 이것이 가능한 이유는 rust에서는 match를 통해서 모든 타입에 대한 처리를 보장하기 때문이다.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    match row.get(0) {
        Some(SpreadsheetCell::Int(value)) => println!("enum_vector => Int: {}", value),
        Some(SpreadsheetCell::Float(value)) => println!("enum_vector => Float: {}", value),
        Some(SpreadsheetCell::Text(value)) => println!("enum_vector => Text: {}", value),
        None => println!("enum_vector => None"),
    }
}
