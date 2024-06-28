

#[derive(Debug)]
pub struct Node{
    value : i32,
    next : Option<Box<Node>>,
}


#[derive(Debug)]
//created the header for linked list
pub struct  LinkedList{
    head : Option<Box<Node>>,
}


// created the function for header
impl LinkedList {
    pub fn new()->Self{
        LinkedList{ head : None}
    }

   

    pub fn insert(&mut self,value : i32){
 
      println!("the self node is : {:?}",self);

        let new_node = Box::new(
            Node{
                value,
                next : self.head.take(),
            }
        );
    
         println!("the new node is : {:?}",new_node);
         
        self.head = Some(new_node);
    }

    pub fn print(&self){

        let mut current = &self.head;

       // let mut  i = 1;
        
        while let Some(node) = current {
            print!("{} -> ", node.value);
           // println!("the current is : {:?}",current);
            current = &node.next;
            // i += 1;
            // println!("the value of i is : {}",i);

        }
        println!("None");

        
    }

   

      pub fn search_element(&self, value : i32) -> i32 {

      // println!("the self is : {:?}",self);

      let mut current_node = &self.head;

       while let Some(node) = current_node {
          if node.value != value{
            current_node = &node.next;
           // return  value;
          }  
           else {
               return value;
           }
           
       }

        
         return 0;
      

      }
}

pub fn add_value(){
    // let mut linked_list = LinkedList::new();

    // println!("the linked list when calling new is : {:?}",linked_list);

    // linked_list.insert(2);   
    // println!("the linked list when calling new is : {:?}",linked_list);

    // linked_list.insert(6);   
    // println!("the linked list when calling new is : {:?}",linked_list);

    let mut linked_list = LinkedList::new();

    // let address : *const LinkedList  = &linked_list;

    // println!("the address of linked list is : {:?}",address);

     linked_list.insert(2);
     linked_list.insert(3);
     linked_list.insert(4);

      linked_list.print();
     let get_serch_value = linked_list.search_element(3);

     
    // let first_borrow = &linked_list;
    // println!("the first borrow is : {:?}",first_borrow);

    // let second_borrow = &mut linked_list;
    // println!("the second borrow is : {:?}",second_borrow);

    // let third_borrow = &linked_list;
    // println!("the third borrow is : {:?}",third_borrow);

     if get_serch_value != 0 {
         println!("the value is found");
     }
     else{
        println!("value not found");
     }

 
      
    
}