use std::collections::HashMap;

pub fn run() {
    create_hash_map();
    access_hash_map();
    update_hash_map();
}

/**
 * rust의 해시맵은 SipHash 알고리즘을 사용하여 구현되어 있음
 */
fn create_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("create hash map => {:?}", scores);
}

fn access_hash_map() {
    let s1 = String::from("Blue");
    let mut scores = HashMap::new();
    scores.insert(s1, 10);
    scores.insert(String::from("Yellow"), 50);

    // get 메소드는 Option을 반환함, copied 함수를 통해 참조가 아닌 값을 반환함
    let score = scores.get("team_name").copied();
    match score {
        Some(score) => println!("access hash map by get function => {}", score),
        None => println!("access hash map by get function => None"),
    }

    // for문을 이용하여 모든 키와 값을 순회할 수 있음
    for (key, value) in &scores {
        println!("access hash map by for loop => {}: {}", key, value);
    }

    // s1은 소유권이 해시맵으로 이동하였기 때문에 사용할 수 없음
    // 이것을 방지하기 위해 해시맵에 대해 참조를 사용한다면, 해시맵이 존재하는 기간 동안은 s1이 존재한다는 라이프사이클을 명시해야함
    // println!("access hash map by get function => {}", s1);
}

fn update_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // insert 함수는 이미 존재하는 키에 대해 값을 업데이트함
    scores.insert(String::from("Blue"), 25);

    println!("update hash map => {:?}", scores);

    // 키에 대한 값이 없을 때만 값이 추가됨, entry 함수는 키값으로 값을 검사 후 Entry 열거형을 반환함
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    println!("update hash map by entry function => {:?}", scores);

    // 기존 값에 대해 연산을 수행할 수 있음
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert는 가변 참조자를 반환하기 때문에 역참조를 통해 기존의 값을 업데이트 할 수 있음
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("update hash map by entry function => {:?}", word_count)
}
