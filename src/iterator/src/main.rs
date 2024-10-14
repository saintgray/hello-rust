fn main() {
    // Rust 의 반복문 keyword
    // loop, while, for
    
    // 1. loop 의 예시
    // loop 는 코드 블럭 내의 구문을 무한으로 실행하며
    // 적절한 조건에 의한 탈출 조건을 필요로 한다
    let mut i = 1;
    loop {
        if i > 10 { break; }
        println!("{}", i);
        i += 1;
    }

    // 2. while
    // loop 와 동일함

    // 3. for 
    // 배열, 벡터와 같은 컬렉션(collection) 으로부터
    // 각 요소를 하나씩 가져와 loop 를 돌며 처리하며
    // 가장 흔히 사용되는 반복 구분이다.
    let arr = [1,2,3,4,5];
    for el in arr {
        println!("{}", el);
    }

    // continue, break
    // 자세한 설명은 생략한다

    // expression 으로 취급될 수 있으며 예를 들면 다음과 같다
    // 1 ~ 100 까지 수 중 홀수인 수를 모두 더한 값을 반으로 나눈 값 구하기
    let mut i = 0;
    let mut sum = 0;
    println!("result is : {}", 
        loop {
            i += 1;
            if i % 2 == 0 { continue; }
            if i > 100 {
                // break 뒤에 expression 을 넣어 값을 return 할 수 있다.
                // 이 코드는 return 한 값을 출력하는 코드이다.
                break sum / 2;
            }
            sum += i;
        }
    )
    
}
