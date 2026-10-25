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


   fn word_checker_from_file(&self, filename: &str) -> Result<(Vec<String>, Vec<String>), std::io::Error>{
             
        let contents = fs::read_to_string(filename)?;  

        let mut found_words: Vec<String> = Vec::new();
        let mut not_found_words: Vec<String> = Vec::new(); 


       // Check each line in the file (handles: comma, new lines, or just space separated list)
    for line in contents.split(|c: char| c == ',' || c.is_whitespace())  {
        let word = line.trim();

        // Skip empty lines
        if word.is_empty() {
            continue;
        } 

           match validity_check(word) { 

            Ok(valid_word) => { 
                if self.search(&valid_word) {
                    found_words.push(valid_word); 

                } else {
                    not_found_words.push(valid_word);
                }
            }

            Err(message) => {
                println!("Skipping '{}': {}", word, message);
            }
        }
    }

    // Return both lists
    Ok((found_words, not_found_words))
}

    

fn delete_list_from_file(&mut self, filename: &str) -> Result<(), std::io::Error> {

    // Read the file containing words to delete
    let contents = fs::read_to_string(filename)?;

    // Separate words at commas, spaces, or new lines
    for word in contents.split(|c: char| c == ',' || c.is_whitespace()) {

        let word = word.trim();

        // Skip empty spaces
        if word.is_empty() {
            continue;
        }

        // Check the word and convert it to uppercase
        match validity_check(word) {
            Ok(valid_word) => {

                // Delete the word if it exists
                if self.delete(&valid_word) {
                    println!("Deleted: {}", valid_word);
                } else {
                    println!("Word not found: {}", valid_word); // buts adds the word 
                }
            }

            Err(message) => {
                println!("Skipping '{}': {}", word, message);
            }
        }
    }

    Ok(())
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

    for line in contents.split(|c: char| c == ',' || c.is_whitespace()) { //fro loop to add new words from the text 
        let word = line.trim();

        if !word.is_empty() {
            match validity_check(word) {
                Ok(valid_word) => {
                    if !trie.search(&valid_word) { // this one is writing all the words that are not present yet
                                trie.insert(&valid_word);}
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

    // Load the dictionary into the Trie
    text_reader(&mut trie_dictionary,r"C:\Users\yidid\rust_A2\trie\data\dictionary.txt");

    println!("Welcome to the Trie dictionary!");

    loop {
        println!("\nChoose a task:");
        println!("1. Insert");
        println!("2. Search");
        println!("3. Delete");
        println!("4. GetFileChecked");
        println!("5. DeleteListFromFile");
        println!("0. Exit");

        let mut task = String::new();
        //reading the line entered
        io::stdin().read_line(&mut task).unwrap();

        let task = task.trim().to_ascii_lowercase();

        // for the Exit program, 
        if task == "0" || task == "exit" {
            println!("Bye Bye!");
            break;
        }

        // Check words from a file
        if task == "4" || task == "getfilechecked" {
            match trie_dictionary.word_checker_from_file("C:/Users/yidid/rust_A2/trie/data/words_to_check.txt") {
                Ok((found, not_found)) => {
                    println!( "Here are the words already in the trie_dictionary: {:?}",found);
                    println!("And these words were not found: {:?}", not_found);
                }
                Err(error) => {
                    println!("Could not read file: {}", error);
                }
            }

            continue;
        }

        // Delete words from a file
        if task == "5" || task == "deletelistfromfile" {
            match trie_dictionary.delete_list_from_file(
                "C:/Users/yidid/rust_A2/trie/data/words_to_check.txt"
            ) {
                Ok(()) => {
                    println!("The words are deleted from the dictionary as suggested in the text file !");
                }
                Err(error) => {
                    println!("Could not read file: {}", error);
                }
            }

            continue;
        }

        // Check that the task requires a single word
        if task != "1" && task != "insert"
            && task != "2" && task != "search"
            && task != "3" && task != "delete"
        {
            println!("Invalid task! Please choose a task from 0 to 5.");
            continue;
        }

        // Ask for a word only for Insert, Search, and Delete
        println!("Enter a word (or '0' to return to the menu):");

        let mut input_word = String::new();
        io::stdin().read_line(&mut input_word).unwrap();

        let word = input_word.trim();

        if word == "0" {continue;}

        // Validate the word
        match validity_check(word) {
            Ok(valid_word) => {
                match task.as_str() {

                    "1" | "insert" => {
                        if trie_dictionary.search(&valid_word) {
                            println!("Word already exists!");
                        } else {
                            trie_dictionary.insert(&valid_word);
                            println!("Word added!");
                        }
                    }

                    "2" | "search" => {
                        if trie_dictionary.search(&valid_word) {
                            println!("Word found!");
                        } else {
                            println!("Word not found!");
                        }
                    }

                    "3" | "delete" => {
                        if trie_dictionary.delete(&valid_word) {
                            println!("The word is deleted!");
                        } else {
                            println!("The word is not in the dictionary!");
                        }
                    }

                    _ => {}
                } 
                
            }

            Err(message) => {
                println!("{}", message);
            }
        }
    } 


} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_trie() {
        let trie = trie_root::new();
        assert!(!trie.search("APPLE"));
    }

    #[test]
    fn test_insert_and_search() {
        let mut trie = trie_root::new();

        trie.insert("APPLE");

        assert!(trie.search("APPLE"));
        assert!(!trie.search("BANANA"));
    }

    #[test]
    fn test_duplicate_insert() {
        let mut trie = trie_root::new();

        trie.insert("APPLE");
        trie.insert("APPLE");

        assert!(trie.search("APPLE"));
    }

    #[test]
    fn test_delete() {
        let mut trie = trie_root::new();

        trie.insert("APPLE");

        assert!(trie.delete("APPLE"));
        assert!(!trie.search("APPLE"));
    }

    #[test]
    fn test_delete_missing_word() {
        let mut trie = trie_root::new();

        assert!(!trie.delete("BANANA"));
    }

    #[test]
    fn test_delete_prefix_preserves_longer_word() {
        let mut trie = trie_root::new();

        trie.insert("CAR");
        trie.insert("CART");

        assert!(trie.delete("CAR"));
        assert!(!trie.search("CAR"));
        assert!(trie.search("CART"));
    }
}