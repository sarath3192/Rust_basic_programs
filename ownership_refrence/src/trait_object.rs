trait Birds{
    fn fly(&self);
    fn eat(&self);
}
struct Parrot{
    fly_height: i32,
    kcal: u32,
    food: String, 
}
struct Eagle{
    fly_height: i32,
    kcal: u32,
    food: String,
}

impl Birds for Parrot{
    fn fly(&self){
        println!("I am Parrot:");
        println!("I can fly to a height of: {}meter", self.fly_height);
    }
    fn eat(&self){
        println!("I eat: {}", self.food);
        println!("For {}meter height I take {} kcals energy", self.fly_height, self.kcal);
    }
}
impl Birds for Eagle{
    fn fly(&self){
        println!("\nI am Eagle: ");
        println!("I can fly to a height of: {}meter", self.fly_height);
    }
    fn eat(&self){
        println!("I am Eagle: ");
        println!("I eat: {}", self.food);
        println!("For {}meter height I take {} kcals energy", self.fly_height, self.kcal);
    }
}

fn dynamic_dispatch(bird: &dyn Birds){
    bird.fly();
    bird.eat();
}

fn main(){
    let a = Parrot{fly_height: 34, kcal: 300, food: String::from("Greens")};
    let b = Eagle{fly_height: 100, kcal: 500, food: String::from("Rats")};

    dynamic_dispatch(&a);
    dynamic_dispatch(&b);
}