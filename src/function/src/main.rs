fn main() {
   // 함수의 선언
   // 1. keyword : fn
   // 2. case : snake case
   // 3. 외부에서 사용할 수 있도록 접근제한자 설정 : pub fn
   greeting();
   // parameter
   // (pub) fn {함수명} (...parameter_name: data_type) { ..some code }
   // return 값이 있을 경우 
   //   (pub) fn {함수명}(..name: type) -> {type} { ... return value }
   println!("{}", sum(1,2));
   println!("=====");
   let a = 1;
   let a = 2;
   {
        let a = a+1;
        println!("{}", a);
   }
   println!("{}", a);
}

fn greeting() {
    println!("Hello Function")
}

fn sum(a:i32, b:i32) -> i32 {
    return a + b;
}