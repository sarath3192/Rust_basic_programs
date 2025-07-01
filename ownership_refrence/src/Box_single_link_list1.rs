#[derive(Debug)]
struct Node{
    Name: String,
    next_node: Option<Box<Node>>,
}

fn main(){
    let node2 = Node{
                    Name: String::from("Sarath"), 
                    next_node: None
                };
    let node1 = Node{
                    Name: String::from("Prasanth"), 
                    next_node: Some(Box::new(node2))
                };
    let node0 = Node{
                    Name: String::from("Nagraj"), 
                    next_node: Some(Box::new(node1))
                };
    let mut current = Some(Box::new(node0));
    match current{
        Some(k) => { println!("{:?}",k.Name);
                current = k.next_node;
                },
        None    => println!("No more node"), 
    }
}