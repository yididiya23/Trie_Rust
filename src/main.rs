struct trie_root{  //defining a root that represents the whole trie  
    root: trieNode,  // the root node to have  an array with size 26 positons children initatied to be None
} 


//set of functions for the trie_root
impl trie_root{ //to make the following func work on the struct defined above
    
    //if variable is trie_root struct and new() function is called, do this 
    fn new() -> trie_root{   
       trie_root{root:trieNode::new()}  // call the trie_node struct and make it to be form a trie_node (because the root node is nothing different only the start)
    } 


    //or for a trie_root struct, if the insert() is called, do the following 
    fn insert(&mut self, word: &str){  
        
        //self:  to indicate the task of modifying that node itself 
        // word is the given string to be checked 
       
        let mut current = &mut self.root;  // borrow the address of the node's root and call it current 


        for l in word.chars(){  // breaking down the entry string to char
            
            let i = (l as usize) - ('a' as usize); //giving the char a number like array startign from 0  
            
            //creating a child node of that char if the position is still None 
            if current.children_node[i].is_none() { 
                current.children_node[i] = Some(Box::new(trieNode::new()));
            } 
             //move to that child, give it mutable access, and extract the node from option and update the current node
             current = current.children_node[i].as_mut().unwrap();


        } 

    
        current.is_end_of_word = true;  // checks if the last letter that is given by the user is the last one 
   }  



   // similar way of checking as the insert function 
   fn search(& mut self, word: &str) -> bool{ 
        
        let mut current = &self.root;

        for l in word.chars() {
            
            let i = (l as usize) - ('a' as usize); 

            if  current.children_node[i].is_none(){  
                return false;
            }  
            
            current = current.children_node[i].as_ref().unwrap();
        } 
       
         current.is_end_of_word 

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
            is_end_of_word: false,    //creates Array of size 26 with value None and end marker as false 
            }
    }
 } 






fn main() {
    let mut trie_dictionary = trie_root::new();

    trie_dictionary.insert("car");
    trie_dictionary.insert("cart");
    trie_dictionary.insert("care");

    let word = "cat";  

    if trie_dictionary.search(word) { //continues with the if segment if "true" is the output from the search 
       
        println!("Word '{}' already exists!", word);
    } 
   
    else {
        println!("Word '{}' was not found.", word);
        println!("Adding it to the Trie now!");

        trie_dictionary.insert(word);

        println!("The Word '{}' has been added!", word);
    } 
    
    
    println!("Does cat exist now? {}", trie_dictionary.search("cat"));
}



  

