fn main() {
    //let x = 5;

    let y = {
        let x = 3;
        x + 1 // 반환값이므로 세미콜론을 사용하지 않음
    };

    println!("The value of y is: {}", y);
}
