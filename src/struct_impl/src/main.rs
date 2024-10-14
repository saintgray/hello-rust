// impl 구현 block
// impl keyword 는 구현 블럭(=implementation block) 을 정의할 때 사용한다.
// impl block 내에는 method, 혹은 연관함수(=associated function) 이 주로 들어가며
// impl 안에 정의된 함수는 그 타입과 연관된 함수라는 의미에서 associated function 이라 불리운다.

// method 는 첫번째 파라미터로 항상 self 를 갖지만, 연관함수는 self 를 갖지 않는다.
// 다시 말하면 method 는 항상 type instance 를 갖지만, 연관함수는 그렇지 않다.
// 호출 시 method 는 접속 연산자 (.) 를 사용하며 연관함수는 namespace (::) 로 호출한다
struct V {
    no: u16,
    name: String
}

impl V {
    // associated function
    fn new(no: u16, name: String) -> V {
        V { no, name }
    }
    // method
    fn introduce(&self) {
        println!("도시의 이름 : {}", self.name);
    }
    
    fn change_name_to(&mut self, _name: String) {
        self.name = _name;
    }
}

fn main() {
    // call associated function
    let v = V::new(1, "Yeosu".to_owned());
    // call method
    v.introduce();

    // init mutable struct
    let mut v2 = V::new(2, "Iksan".to_owned());
    v2.introduce();
    v2.change_name_to("Seoul".to_owned());
    v2.introduce();
}
