use std::rc::Rc;

#[derive(Debug)]
struct Node{
    number: i32,
    refernce: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node1{
    num: i32,
    re: Option<Rc<Node1>>,
}

fn main(){
    // let a = Node{number: 3, refernce: Some(& Box::new(& Node{number: 2, refernce: Some(& Box::new(Node{number: 1, refernce: Some(& Box::new(Node{number: 0, refernce: None}))}))}))};
    // let b = Node{number: 4, refernce: Some(& Box::new(a))};
    // let c = Node{number: 5, refernce: Some(& Box::new(a))};
    let a = Rc::new(Node1{num: 1, re: Some(Rc::new((Node1{num: 0, re: None})))});
    let b = Node1{num: 2, re: Some(a.clone())};
    let c = Node1{num: 2, re: Some(a.clone())};
    println!("{:?}\n{:?}", b, c);
}
