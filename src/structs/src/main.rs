// Rust 는 OOP 언어가 아니므로 일련된 데이터 그룹을 만들기 위해
// 구조체 (struct) 를 사용하며, 이 구조체에 연관되어 있는 함수들은
// "구조체 밖" 에서 구현한다.


// 1. struct 선언
struct Student {
    name: String,    // 이름
    age: u16,        // 나이
    present: bool,   // 출석여부
    hobby: String    // 취미
}

// 2. tuple struct
// tuple 처럼 필드명을 갖지 않고 필드들을 tuple 로 묶어 정의하는 구조체
// 예제 RGB Code
struct Color(u8, u8, u8);


fn main() {
    
    // struct 예시
    let me = Student {
        name: String::from("오종현"),
        age: 33,
        present: false,
        hobby: "tenis".to_owned()
    };
    println!("{}, {}, {}, {}", me.name, me.age, me.present, me.hobby);

    // mutable struct 예시
    let mut you = Student {
        name: String::from("오가현"),
        age: 34,
        present: true,
        hobby: "piano".to_owned()
    };
    you.age += 1;
    println!("{}, {}, {}, {}", you.name, you.age, you.present, you.hobby);
    
    // tuple struct 예시
    let red = Color(255, 0, 0);
    println!("R={}, G={}, B={}", red.0, red.1, red.2);

    // unit-like struct
    // unit 타입은 아무런 데이터를 갖지 않는 구조체를 의미
    struct Dummy;
    let _dummy = Dummy;

    // struct field 단축 초기화
    // 함수에 전달되는 파라미터명과 struct 필드명이 동일하다명
    // 파라미터명을 생략할 수 있다.
    let tom = new_member("Tom".to_owned(), 14, true, "None".to_owned());
    println!("{} {} {}", tom.name, tom.age, tom.present);
    
    // struct update
    // 만약 구조체 전체를 복제한 후 그 중 일부만을 변경하려 한다면
    // 펼침연산자를 사용하여 기술할 수 있다.
    // 이 때 펼침 연산자는 가장 마지막에 기술해야 한다.
    // let tommy = new_member("Tommy".to_owned(), 20, true);
    let tommy = Student {
        name: "Tommy".to_owned(),
        age: 24,
        present: false,
        hobby: "swimming".to_owned()
    };
    let jane = Student {
        // 이름은 Jane, 나이와 출석 여부는 Tommy 와 같다
        name: "Jane".to_owned(),
        ..tommy
    };
    println!("{} {} {}", jane.name, jane.age, jane.present);
    // 주의할 점은 이렇게 다른 구조체의 값을 가져올 경우, 소유권을 가진 데이타 타입에
    // 대해서는 소유권 이전 (move) 이 일어날 수 있다는 점이다.
    // 즉 위처럼 tommy 와 jane 을 선언할 경우
    // tommy 의 나이와 출석 여부는 jane 으로 소유권이 이전되어 사용이 불가능하다
    // tommy.present = false;

    // primitive type 은 소유권 개념이 없는 것으로 보인다
    println!("{} {}", tommy.age, tommy.present);    // 정상
    // println!("{}", tommy.hobby); // << Compile Error (value borrowed here after move)
}

fn new_member(name: String, age: u16, present: bool, hobby: String) -> Student {
    Student { name, age, present, hobby }
}
