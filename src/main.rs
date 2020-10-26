fn main() {
  println!("Hello, world!");
  /*
  배열은 고정적인 데이터를 가질 때 사용함
  스택에 데이터가 쌓임
   */
  let a = [1,2,3,4];
  let one = a[0];
  let two = a[1];

  // 함수 호출은 표현식,
  let value = test(29);
  println!("return value is {}", value);
  condition();
  repeat();
  ownership();
}

/*
함수의 위치는 어디든 있어도 호출 가능
 */
fn test(x: i32) -> i32 {
  // 구문은 명령들의 나열로 어떠한 값을 반환하지 않는 것을 의미
  // 표현식은 결과값을 산출하여 반환함
  // 함수 자체는 하나의 구문, let을 이용하여 선언하는 것도 마찬가지
  println!("test fun: {}", x);

  10; // 하나의 값 자체가 하나의 표현식, 그렇기 때문에 10이란 표현식은 10을 반환한다
  10 + 1; // 더하기 연산도 하나의 표현식이기 때문에 결과를 반환한다
  // 블럭도 하나의 표현식
  let y = {
    let mut yy = 0;
    yy = yy + 1; // 세미콜론이 있다면 구문으로 처리, 아래와 같이 없다면 하나의 표현식으로 처리함
    yy  // 즉 y의 값은 yy로 처리됨
  };

  println!("test fun: y = {}", y);
  y + 1   // 표현식으로 사용되어 반환값으로 처리됨, 만약 세미콜론으로 구문으로 처리하면 컴파일 단계에서 에러가 나면서 세미콜론을 제거하라고 함....정말 어썸..
}

fn condition() {
  let number = 3;
  if number > 2 { // if문 안에서는 무조건 boolean 타입이여야 함, 자바스크립트 처럼 가짜 부울린 타입은 적용 안됨.. 강력한 타입 시스템
    println!("big");
  } else {
    println!("small");
  }

  // if는 표현식이기 때문에 값을 반환 한다
  let res = if number != 0 {
    "number"
  } else {
    "zero"  // 만약 넘버를 반환 한다면, 컴파일시 에러 발생함 -- 모든 리턴 타입이 동일해야 하는 강력한 타입 시스템
  };
  println!("if return value = {}", res);
}

fn repeat() {
  let mut number = 3;

  while number != 0 {
    println!("number repeat = {}", number);
    number -= 1;
  }

  let array = [1,2,3,4];
  let mut index = 0;
  // 일반적인 while문
  while index < array.len() {
    println!("array index = {}", array[index]);
    index += 1;
  }

  // 자바스크립트처럼 이터레이터를 이용해 안전하게 순회
  for number in array.iter() {
    println!("array iter = {}", number);
  }

  for number in (1..3).rev() {
    println!("range number = {}", number);
  }
}

// 소유권은 러스트의 가장 큰 특성으로 가비지 컬렉션 없이 메모리의 안정성을 보장해주는 방법이다
fn ownership() {
  // 소유권 원칙 - 1. 러스트는 각각의 값에 오너라고 불리는 변수를 가지고 있다., 2. 한번에 딱 하나의 오너만 존재함. , 3. 오너가 스코프 밖을 벗어나면 값을 버려진다.

  let s = "hello"; // 스트링 리터럴 (문자열은 고정된 크기 - 컴파일 타입에서 고정된 사이즈를 가지고 있기 때문에 스택에 저장 !, String 타입은 가변성을 지니므로 힙에 저장 - 컴파일 타입에 크기를 알 수 없음)
  let mut s2 = String::from("hello"); // String 타입,
  s2.push_str(", world");
  println!("String type = {}", s2);
  // 컴파일 타임에 알수 있는 고정적인 문자열은 스트링 리터럴로 만드는게 좋음, 가변적인 문자열은 String 타입으로 작성하는 것이 효율적..어차피 스코프 벗어나면 값이 버려지기 때문에 메모리 안정성 보장... 물론 참조자로 지속적으로 오너를 가지고 있는 코드라면 버려지지 않음
}