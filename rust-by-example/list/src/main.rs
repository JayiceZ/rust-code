use std::fmt::{Display, Formatter,Result};

struct List{
    val:i32,
    next:Option<Box<List>>,
}


impl List{
    pub fn add_to_head(self,num:i32)->List{
        let new_node=List{val:num,next:Some(Box::new(self))};
        new_node
    }
}

impl Display for List{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"[")?;
        let mut node=self;
        loop{
            write!(f,"{}",node.val)?;
            match node.next{
                None => {break;}
                Some(ref b) => {
                    node=&*b;
                }
            }
        }
        write!(f,"]")
    }
}



fn main(){
    let mut list=List { val:0,next:None };
    list=list.add_to_head(1);
    list=list.add_to_head(2);
    list=list.add_to_head(3);
    println!("{}",&list);
}