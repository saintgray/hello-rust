fn main() {
    // if 조건식의 예시
    let score = 'C';
    if score <= 'B' {
        println!("Excellent");
    } else if score <= 'C' {
        println!("Pass");
    } else {
        println!("Fail");
    }
    // rust 의 if - else block 은 expression(=식) 으로 간주되므로
    // 아래와 같이 기술 할 수 있다.
    // 단 분기별로 return type 이 모두 동일해야 하며
    // 이에 위배될 시 Compile Error 가 발생한다.
    let score = 'B';
    let passed = if score <= 'C' { true } else { false };
    // let passed = if score <= 'C' { true } else { "false" };   // << Compile Error
    println!("{}", passed);
}
