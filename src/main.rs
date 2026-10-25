use std::io;
use std::fs;

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
            
            let i = (l as usize) - ('A' as usize); //giving the char a number like array startign from 0  
            
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
   fn search(&self, word: &str) -> bool{ 
        
        let mut current = &self.root;

        for l in word.chars() {
            
            let i = (l as usize) - ('A' as usize); 

            if  current.children_node[i].is_none(){  
                return false;
            }  
            
            current = current.children_node[i].as_ref().unwrap(); //making it immutable
        } 
       
         current.is_end_of_word 

   } 


   fn delete(&mut self, word: &str) -> bool{
    
    let mut current = &mut self.root; 

    for l in word.chars() {
            
            let i = (l as usize) - ('A' as usize); 

            if  current.children_node[i].is_none(){  
                return false;
            }  
            
            current = current.children_node[i].as_mut().unwrap(); //making it mutable
        }  
    
    //after reaching the last node, check if that node is marked as an end 
    if current.is_end_of_word {  // if the word was there, change it false(remove the word)
        current.is_end_of_word = false;
        return true;
    } 
    else {
        return false;
    }

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






fn validity_check(word: &str) -> Result<String, &'static str> {
    
    if word.is_empty() {
        return Err("Please enter a word!");
    } 

     if !word.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("Please enter letters A-Z only!");
    } 

    let word = word.to_ascii_uppercase(); 

    Ok(word) //sends the uppercase of the word 


}
 


// reading the file 
fn text_reader(trie: &mut trie_root, filename: &str) {

    //reading the whole file as string 
    let contents = fs::read_to_string(filename).expect("Could not read dictionary file");

    for line in contents.lines() { //reading line by line 
        let word = line.trim();

        if !word.is_empty() {
            match validity_check(word) {
                Ok(valid_word) => {
                    trie.insert(&valid_word); // this one is writing all the words in the memory one by one 
                }
                Err(message) => {
                    println!("Skipping '{}': {}", word, message);
                }
            }
        }
    }
    
    
    println!("Dictionary loaded successfully!");


}





fn main() {
    
    let mut trie_dictionary = trie_root::new();

    // Optional starting words
    //trie_dictionary.insert("CAR");
    //trie_dictionary.insert("CART");
    //trie_dictionary.insert("CARE"); 

    //Paasing the existing trie to the rader function 
    text_reader(&mut trie_dictionary, r"C:\Users\yidid\rust_A2\trie\data\dictionary.txt");

    println!("Welcome to the Trie dictionary!");

loop{  
    println!("Enter a word (or '0' to exit)");

    let mut input_word = String::new();  

    //accepting the user input 
    io::stdin().read_line(&mut input_word).unwrap();

    // removing the before and after whitespace from the input 
    let word = input_word.trim(); 

    //checking if the user is trying to quit
    if word == "0" {
         println!("Goodbye!");
         break;
    }


    // Validating the word
    match validity_check(word) {
       
        Ok(valid_word) => {
           
            println!("Choose a task: Insert, Search, Delete");

            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();

            match task.trim().to_ascii_lowercase().as_str() {

                "insert" => {
                                if trie_dictionary.search(&valid_word) {
                                    println!("Word already exists!");
                                } 
                                else {trie_dictionary.insert(&valid_word);
                                        println!("Word added!");
                                }
                            }

                "search" => {
                                if trie_dictionary.search(&valid_word) {
                                    println!("Word found!");
                                } else {
                                         println!("Word not found!"); 
                                         println!("Adding it to the Trie now!"); 

                                        trie_dictionary.insert(&valid_word); 

                                        println!("Word '{}' has been added!", valid_word);

                                }
                            }

                "delete" => { 
                                if trie_dictionary.delete(&valid_word) {
                                    println!("The Word is deleted!");
                                } 

                                else{ 
                                    println!("The Word is not in the dictionary! It is safe!");
                                } 
                            }

                _ => {
                        println!("Invalid task! Please make sure to choose from insert/ search/ delete ");
                     }

            }
        }
        
        Err(message) => {
                println!("{}", message);
                continue; 
        }
    }


}
 }



  

