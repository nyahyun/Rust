#![allow(unused)]
//stack에 대한 데이터는 clone 없이도 복사 가능, String 데이터는 clone 필요
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}


