trait Calculator{
    fn sum(&self, num1: i32, num2: i32)->i32;
    fn mul(&self, num1: i32, num2: i32)->i32;
}

struct Random{
}

impl Calculator for Random{
    fn sum(&self, num1: i32, num2: i32) -> i32{
        return num1+num2
    }
    fn mul(&self, num1: i32, num2: i32) -> i32{
        return num1*num2
    }
}

fn main(){
    let a = Random{};
    // println!("{} {}", Random::sum(10,20), Random::mul(2,3));
    println!("{} {}", a.sum(10, 20), a.mul(2,3));
}