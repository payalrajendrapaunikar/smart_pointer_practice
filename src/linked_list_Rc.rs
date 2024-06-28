use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::{btree_map::Values, linked_list}, rc::Rc};



#[derive(Debug)]
pub struct Node{
    value : i32,
    next : Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
pub struct LinkedList{
    head : Option<Rc<RefCell<Node>>>
}

impl LinkedList {
    
    pub fn new()->Self{
        LinkedList{head : None}
    }

    pub fn insert(&mut self,value:i32){

        println!("the self node is : {:?}",self);

        let new_node = Rc::new(RefCell::new(
            Node{
                value,
                next : self.head.clone(),
            }
        ));

        println!("the new node created is : {:?}",new_node);

        self.head = Some(new_node);
    }

   
}


pub fn create_node(){

    let mut  create_new_linked_list = LinkedList::new();

    println!("the linked list is : {:?}",create_new_linked_list);

    create_new_linked_list.insert(2);

    println!("the linked list is : {:?}",create_new_linked_list);

    create_new_linked_list.insert(3);

    println!("the linked list is : {:?}",create_new_linked_list);

   

    //first borrow
    let first_borrow_node = create_new_linked_list.borrow();
    let head = &first_borrow_node.head;
   
   
    println!("the first boorow node is : {:?}",first_borrow_node);

     // Mutable borrow
     {
        let second_borrow_node = create_new_linked_list.borrow_mut();
        second_borrow_node.insert(5);
           println!("The second borrow node after mutation is: {:?}", second_borrow_node);
    } // Mutable borrow goes out of scope here

    let third_borrow_node = create_new_linked_list.borrow_mut();
    println!("the third borrow node is : {:?}",third_borrow_node);


    

}

