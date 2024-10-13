fn main() {
    // overall();
    shadowing();
    // string_doc();
    // type_scarlar();
    // type_compound();
}

/**
 * overall()
 * Rust 변수의 개요
 */
fn overall() {
    // 1. 변수의 선언
    // let {변수명}
    let a = 1;
    let b = 3.14;
    // 2. 변수의 불변셩
    // Rust 의 모든 변수는 Default 로 불변성의 성질을 갖는다.
    // 즉 값이 한번 지정된 이후에는 그 값을 변경할 수 없다.
    // 값의 재할당은 값을 변경하는 것이 아니므로 가능
    // a = a +1; <-- Compile Error (cannot assign twice to immutable variable `a`)

    // 3. 가변적 변수 : mut
    // 변수의 값이 변경되어야 하는 경우 선언 시 mut keyword 로 선언한다
    let mut c = 3;
    c += c; // c = 6
    c = c + 10; // c = 16
    println!("{}", c); // expected 16

    // 4. 상수 : constant {변수명}
    // 상수는 선언시 불변성을 가지며 "항상" 데이터 타입을 지정해야 한다.
    const PI: f64 = 3.14159;
    println!("{}", PI);
    // PI = PI + 0.000002;    // <-- Compile Error
}

fn shadowing() {
    // Rust 기능 중 하나로 한 code scope 내에서 동일한 변수명을 let 으로
    // 여러 번 정의할 수 있다.
    let a: i32 = 1;  // a 는 singed 32 비트 정수 1
    println!("{}", a);
    let a = "hello"; // a 는 문자열 "hello"
    println!("{}", a);

    // code scope 별 Shadowing 의 예제 1
    let b = 1;  // b is 1
    let b = 2;  // b is 2
    {
        let b = b + 5; // b is 7
        println!("{}", b);  // expected 7
    }
    println!("{}", b);  // b is 2, expcted 2

    // code scope 별 Shadowing 의 예제 2 (mutable)
    let mut c = 1;   // c is 1
    c = 2;           // c is 2
    {
        c = c + 1; // c is 3
        println!("{}", c);  // expected 3
    }
    println!("{}", c);  // expected 3
}

/**
 * string_doc()
 * 문자열 관련 type 개요 및 주요 function
 */
fn string_doc() {
    // 1] &str 타입
    // 문자열을 표현하는 Primitive 문자열 타입
    // 1) 문자열 literal 을 변수에 할당 : &'static str type
    //    str type 은 항상 reference 이기 때문에 실제 코드에서는 &str 와 같이  사용된다
    //    runtime 동안 상시로 존재하므로 내부적으로 'static keyword 가 추가된다
    let my_str : &'static str = "hello";
    println!("myStr1 : {}" , my_str);
    // 아래 코드는 위와 동일하다
    let my_str : &str = "hello";
    println!("myStr2 : {}" , my_str);
    // 다음과 같이 축약 형태로 사용할 수 있다
    let my_str = "hello";
    println!("myStr3 : {}" , my_str);


    println!("========== striing_api [slice] ==========");
    // slice : 문자열 일부를 잘라 반환한다. {ref}[from..to]
    let full_string = "abcdefg";
    let sub_string: &str = &full_string[0..4];  // index 0 ~ 3 까지의 문자열을 반환
    println!("sub_string : {} , {}", full_string, sub_string);
    // exercise : trim(), to_lowercase(), to_uppercase(), to_string(), to_owned() 함수 연습
    // to_owned() : &str type 을 String type 으로 변환
    // to_string() : 임의 type 을 String type 으로 변환


    // 2] String 타입
    // lib : std::string::String
    // Heap memroy 상에 문자열을 저장하며, 고정된 길이 아닌 길이에 대한 가변성을 갖는다
    
    let my_string = String::from("hello");  // 변수의 선언
    println!("{}", my_string);
    // 2-1) push_str({string})
    // 문자열을 추가한다.
    // 변수에 할당된 값을 변경하므로 mutable 로 선언해야 함
    let mut my_string_mut = String::from("Hello");
    my_string_mut.push_str(" World!");
    println!("{}", my_string_mut);
}


/**
 * type_scarlar()
 * Rust Data type 에 대한 개요
 */
fn type_scarlar() {
    // Rust 는 Statically Type Language 로서
    // 프로그램 상의 모든 Data Type 을 Compile 시점에 결정하며
    // 각 타입에 위배되는 코드들이 없는 지 미리 체크하여 불필요한 에러를 사전에 방지한다.
    // Data Type 이 특정되지 않은 경우는 Compiler 가 추론하여 결정하며, 명시된 경우 명시된 Type 을 사용한다.

    /* Data Type 의 종류 */

    // 1. scalar type : 정수형, 부동 소수점형, 부울린형, 문자형
    // 1-1) 정수형
    // i8, i32, u8, u32 등과 같이 정수를 포함하는 type
    // i: singed(부호 O), u: unsigned (부호 X)

    // 크기	                Signed	Unsigned
    // 8비트	            i8	    u8
    // 16비트	            i16	    u16
    // 32비트	            i32	    u32
    // 64비트	            i64	    u64
    // 128비트	            i128	u128
    // 아키텍처에 따라 변동	 isize	 usize
    let a: i32 = 1_000_000; // 10진수, 백만 (가독성을 위해 언더바를 지원함)
    println!("a : {}", a);
    let a = 0xff;    // 16진수
    println!("a : {}", a);
    let a = 0o15;    // 8진수
    println!("a : {}", a);
    let a =  0b1111_1111; // 2진수
    println!("a : {}", a);
    let a: u8 = b'A';   // 바이트 (앞에 b를 붙임)
    println!("a : {}", a);  // 문자형 'A' 에 대한 10진수
    let a = 32; // 축약
    println!("a : {}", a);

    // 1-2) 부동 소수점형
    // f32 : 32비트 부동소수점 Data
    // f64 : 64비트 부동소수점 Data
    // 명시하지 않을 시 Default f64 로 지정
    let a: f32  = 3.14;   // f32
    let b = 3.14;         // f64

    // 1-3) 부울린형
    let visited: bool = true;
    let visited = true;

    // 1-4) 문자형
    let my_char: char = 'A';
    println!("my_char : {}", my_char);
}

fn type_compound() {
    // compound type : 복합 타입
    // 여러 개의 값들로 구성된 Type : Array, Tuple
    // 1) Array
    // 선언: let arr: [{type}; {size}] = [values]

    let arr: [i32; 3] = [1,2,3];
    // let arr: [i32; 3] = [1,2,3,4];  // expected compile error
    // 배열의 출력
    println!("arr : {:?}", arr);
    // 연속된 Data 배열
    let arr = [100; 50]; // 값 100 의 크기가 50개인 배열 초기화
    println!("arr : {:?}", arr);

    // 2) Tuple
    // 여러 데이터 타입의 값들의 집합
    // 괄호 () 를 사용하며, 각 데이터를 콤마(,) 로 구분
    // 한번 정의된 Typle은 새로운 요로를 추가하거나 삭제 불가

    // Declare Tuple
    let tuple_example: (i32, char, bool) = (1, 'B', false);
    let usr = ("JongHyean", 32, 172);
    // Read Element
    println!("{}, {}, {}", tuple_example.0, tuple_example.1, tuple_example.2);
    // Destructing from Tuple
    let (a,b,c) = usr;
    println!("{}, {}, {}", a, b, c);

}



