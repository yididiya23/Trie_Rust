


// option: represent a value that might either be present or absent. 
// It doesnt return the number just confirms its presence or absence

struct Node{
    value: char, 
    left: Option<Box<Node>>, // option <> : to make having child optional
    right: Option<Box<Node>>,  // Box<Node>: stores the Node on heap and gives you a ptr to it 

}  


fn tree_printer (node:&Node){
    println!("{}", node.value); 

    if let Some(left) = &node.left{
        tree_printer(left); 
    } 

    if let Some(right) = &node.right{
        tree_printer(right);
    }
}


fn main(){
    let root = Node {
        value: 'a', 
        left: None, 
        right: Some(Box::new(Node{  value: 'K', 
                                    left: None, 
                                    right: None,}))
    };  


    println!("Root: {}", root.value);  
    tree_printer(&root); 

} 

