fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// 가변 참조자는 특정 스코프 내에 특정 데이터 조각에 대한 가변 참조자를 딱 하나만 만들수 있음
// 가변 참조자와 불변 참조자를 혼용할 경우 컴파일 오류 발생
