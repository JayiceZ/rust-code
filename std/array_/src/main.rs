//unstable
use std::array::from_mut;

use std::array::from_ref;

fn main(){
    let mut num:i32=6;
    //return a mut i32 slice,len=1; without copying
    //let v=from_mut(&mut num);
    //v[0]=110;


    //return a i32 slice,len=1; without copying
    //let v=from_ref(&mut num);
    //v[0]=110;  //not ok

}