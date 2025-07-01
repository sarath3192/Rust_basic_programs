#[derive(Debug)]
struct Node<'a>{
    number: i32,
    refernce: Option<&'a Box<Node>>,
}

fn main(){
    let a = Node{number: 3, refernce: Some(& Box::new(& Node{number: 2, refernce: Some(& Box::new(Node{number: 1, refernce: Some(& Box::new(Node{number: 0, refernce: None}))}))}))};
    let b = Node{number: 4, refernce: Some(& Box::new(a))};
    let c = Node{number: 5, refernce: Some(& Box::new(a))};
}