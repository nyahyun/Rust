fn main() {
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}


fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
//fn dangle() -> &String { // dangle은 String의 참조자를 반환합니다.
//    let s = String::from("hello"); //s는 새로운 String 입니다.
//
//   &s // 우리는 String s의 참조자를 반환합니다.
//
//} // 여기서 s는 스코프를 벗어나고 버려집니다. 이것의 메모리는 사라집니다. 위험합니다.       
//이 함수의 반환 타입은 빌린 값을 포함하고 있는데, 빌려온 실제 값은 없습니다.
//
//즉 s가 dangle 안에서 만들어 졌으니 dangle 함수가 끝이 나면 s는 할당해제가 되어야하는데 이것을
//참조자로 반환하려고 하여 컴파일 에러 >> 해법은 String을 직접 반환하는 것이다.

