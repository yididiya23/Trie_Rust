use std::io::SeekFrom::Current;


struct trie_root{  //defining a root that represents the whole trie  
    root: trieNode,  // the root node to have  an array with size 26 positons children initatied to be None
} 


impl trie_root{ //to make the following func work on the struct defined above
    
    //the function to create trieNode
    fn new() -> trie_root{   
       
       trie_root{root:trieNode::new()},   // calling the struct and making it to be formed in the trieNode format and make that as root 
       
    } 


}





struct trieNode{  // struct definition for each individual node
    children_node: [Option<Box<trieNode>>; 26],  // node defifion as option type and of size 26 "arr[option<t>; 26]""
                                                //option - to allow child nodes exist or not (can use some(node) or None later)
                                                // Box - to store the child on heap and save the ptr on stack 
                                               // the box: to make a pointer be stored here and the value on heap
    is_end_of_word: bool,
}  
 

//impl : to define a function assoicated with the struct defined 
impl trieNode{
    
    fn new() -> trieNode{  //function called "new" returns trieNode
        trieNode 
            { 
            children_node: std::array::from_fn(|_| None), // |_| means "receive an argument, but ignore it." and None is the value returned for each position.
            is_end_of_word: false, 
            }
    }
 } 





fn main() {
    let root = trie_root::new();
    
}
  


