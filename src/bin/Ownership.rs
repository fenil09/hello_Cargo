fn main(){
  
  let s1 = String::from("hello");
  print!("accessing s1 before tranfering the ownership{}",s1);
  let s2=s1;
  // on printing the value of S1 we get a error and warning from compiler 

// The main question is why do we get that.
// The reason is after we transfer the ownership to s2 and try to access s1,
// s1 is actually dead it does not hold any value in case of rust the value is moved completely to 
// s2.
// Here the ownership rules get into action where a variable can have only one owner at a time.


// the actual thing to understand over here is that there would be condition of double free error.
// Lets say while trying to access the s1 variable we do not get any error, 
// now when both the variables reaches the scope like the complete programs is finished then rust would try to 
// free up the memory for s2 from the heap and the again it would do the same for
// s1 as well the exact same data. so here rust is clearing same data twice which would corrupt the programs memory.

// This is the main reason why we get an error when we try to access a variable using ownership is changed.

// now lets see how borrowing helps to fix this issue so we would be just sharing the data in the case of borrowing.

let s3 = String::from("hey my name is fenil");
print!("accessing s3 before borrowing{}",s3);
let s4=&s3;
print!("accessing s3 after allowing s4 to borrow it{}",s3);



}







