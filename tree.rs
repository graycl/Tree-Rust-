type Link<T> = Option<Box<Node<T>>>;
use std::cmp::Ordering;
#[derive(Debug)]
struct Node<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

fn find1<T:Ord>( node : &Link<T>,key: &T) -> bool {
    //unimplemented!()

    match node {
	 	    &Some(ref deNode) => {
   	 	    	match key.cmp(&deNode.key){
		        	Ordering::Less=>find1(&deNode.left,key),
		        	Ordering::Greater=>find1(&deNode.right,key),
		        	Ordering::Equal=>true
    			}
	 	    	},
	 	    
	 	    &None => false,
	 	}

}
fn insert1<T:Ord>( node:Link<T>,key: T) -> Link<T>{
        //unimplemented!()
        let mut node = node;	
        match node {
   	 	    Some(ref mut deNode) => {
       	 	    	match key.cmp(&deNode.key){
			        	Ordering::Less=>deNode.left = insert1(deNode.left.take(),key),
			        	Ordering::Greater=> deNode.right = insert1(deNode.right.take(),key),
			        	Ordering::Equal=>{}
        			}
   	 	   	},
   	 	    
   	 	    None => {
				node=Node::new_link(key);
   	 	    },
   	 	}
    	node
    }
///////////////////////////////////////////////////////////////
// fn in1<'b,M>(iterNode:&'b Node<M>,mut vector: Vec<&'b Link<M>>) {
// 		//用于初始化节点
// 		vector.push(&iterNode.left);
// 	    match iterNode.left {
// 	        Some(ref deIterNode) => {
// 	        	in1(deIterNode,vector);
// 	        },
// 	        None => {},
// 	    };
// 	}
fn in1<'a,T>(iterNode:&'a Node<T>,vector: &mut Vec<&'a Link<T>>) {
    match iterNode.left {
        Some(ref deIterNode) => {
        	in1(deIterNode,vector);
        },
        None => {},
    };
	vector.push(&iterNode.left);
}  
fn post1<'a,T>(iterNode:&'a Node<T>,vector: &mut Vec<(&'a Link<T>,bool)>) {
    match iterNode.left {
        Some(ref deIterNode) => {
        	post1(deIterNode,vector);
        },
        None => {},
    };
	vector.push((&iterNode.left,false));
} 
///////////////////////////////////////////////////////////////
impl<T> Node<T> {
    fn new_link(key: T) -> Link<T> {
        //unimplemented!()        
        Some(Box::new(Node::create(key)))
    }
    fn create(key: T) -> Node<T> {
        Node{key: key,left: None,right: None}
	}
}
#[derive(Debug)]
pub struct Tree<T> {
    root: Link<T>,
}

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        //unimplemented!()
        let mut tree=Tree{root:None};
        tree
    }	

    pub fn insert(&mut 	self, key: T) -> bool {
        //unimplemented!()
        
        if self.find(&key){
        	return false;
        }
      	self.root = insert1(self.root.take(),key);
      	true
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.

    pub fn find(&self, key: &T) -> bool {
        //unimplemented!()
        find1(&self.root,key)
      
    }
    pub fn preorder(&self) -> IterPreorder<T> {
        // unimplemented!()
        let mut container=IterPreorder::<T>::create();
        container.vecStack.push(&self.root);
        container
    }
    
    pub fn inorder(&self) -> IterInorder<T> {
        // unimplemented!();
        let mut container=IterInorder::<T>::create();
        let mut vector=vec![];
        vector.push(&self.root);
        match self.root {
            Some(ref iter) =>{
            	in1(iter, &mut vector) 
            },
            None => {},
        };
        
        // for v in &vector {
        // 	println!("{:?}", v);
        // }
        vector.push(&self.root);
        vector.reverse();
        vector.pop();
        vector.pop();
        // println!("{:?}", &vector);
        container.vecStack=vector;
        container
    }

    pub fn postorder(&self) -> IterPostorder<T> {
        // unimplemented!();

        let mut container=IterPostorder::<T>::create();
        let mut vector=vec![];
        vector.push((&self.root,false));
        match self.root {
            Some(ref iter) =>{
            	post1(iter, &mut vector) 
            },
            None => {},
        };
        
        vector.push((&self.root,false));
        vector.reverse();
        vector.pop();
        vector.pop();
        // for v in &vector {
        // 	println!("{:?}", v);
        // }
        container.vecStack=vector;
        container
    }
   
}
///////////////////////////////////////////////////////////////

pub struct IterPreorder<'a, T: 'a> {
    // unimplemented!();
    vecStack :Vec<&'a Link<T>>,
}

impl<'a, T> IterPreorder<'a, T> {
    // unimplemented!();
    fn create() -> IterPreorder<'a,T> {
        IterPreorder{vecStack:Vec::new()}
	}
	
}

impl<'a, T> Iterator for IterPreorder<'a, T> {
    // unimplemented!();
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
		if let Some(&Some(ref node)) =self.vecStack.pop() {
		    if let Some(ref node1) = node.right{
		    	self.vecStack.push(&node.right);
			} 
			if let Some(ref node2) = node.left{
		    	self.vecStack.push(&node.left);
			}
			return Some(&node.key);
		}
		return None;
		}
	}

///////////////////////////////////////////////////////////////

pub struct IterInorder<'a, T: 'a> {
    // unimplemented!();
    vecStack : Vec<&'a Link<T>>,
}

impl<'a, T> IterInorder<'a, T> {
    // unimplemented!();
    fn create() -> IterInorder<'a,T> {
        // let mut container=
        IterInorder{vecStack: Vec::<&'a Link<T>>::new()}
        
	}
	
}

impl<'a, T> Iterator for IterInorder<'a, T> {
    // unimplemented!();
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
    	
		if let Some(&Some(ref node)) =self.vecStack.pop() {
		    if let Some(_) = node.right{
		    	self.vecStack.push(&node.right);
			} ;
			let mut vector=vec![];
	        match node.right {
	            Some(ref iter) =>{
	            	in1(iter, &mut vector) 
	            },
	            None => {},
	        };
	        vector.reverse();
	        vector.pop();
	        // println!("{:?}",vector );
	        self.vecStack.append(&mut vector);
	        // for v in &vector {
	        // 	println!("{:?}",v	 );
	        // }
			return Some(&node.key)
		}
			return None;

	}
}
///////////////////////////////////////////////////////////////

pub struct IterPostorder<'a, T: 'a> {
    // unimplemented!();
    vecStack : Vec<(&'a Link<T>,bool)>,
}

impl<'a, T> IterPostorder<'a, T> {
    // unimplemented!();
    fn create() -> IterPostorder<'a,T> {
        // let mut container=
        IterPostorder{vecStack: Vec::<(&'a Link<T>,bool)>::new()}
        
	}
	
}

impl<'a, T> Iterator for IterPostorder<'a, T> {
    // unimplemented!();
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
    	
    	if let Some(ref theTuple) = self.vecStack.pop() {
    	    let thisNode=theTuple;
    	
    	// let thisNode=self.vecStack.pop();
			if let &Some(ref node) =thisNode.0 {
				if thisNode.1==false{
					self.vecStack.push((thisNode.0,true));
				    if let Some(_) = node.right{
				    	self.vecStack.push((&node.right,false));
					} ;
					let mut vector=vec![];
			        match node.right {
			            Some(ref iter) =>{
			            	post1(iter, &mut vector) 
			            },
			            None => {},
			        };
			        vector.reverse();
			        vector.pop();
			        // println!("left children  {:?}",vector );
			        self.vecStack.append(&mut vector);
			        // for v in &vector {
			        // 	println!("after appending  {:?}",v);
			        // }
					// return Some(&node.key)
					return self.next();

				}
				else {
					// println!("11111");
					// println!("{:?}", &node.key);
					return Some(&node.key);
				}
			} 
			else {
				// println!("22222");
			    return None;
			}
			// return None
		}
		else {
		    
		}
		// println!("33333");
		return None;
	}
}



