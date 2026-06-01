
fn main(){
    let  x = 5;
    print!("The value of x is : {}",x);

// by Default the variable in Rust are immutable like after creating we 
// cannot reassign the value for the variable and to do so we need to use mut keyword before the variable
  let mut y =50;
  print!("Value of y before reassiging is {}",y);
  y=85;
  print!("The value of y is:{}" ,y);

  const counter:u32 = 5000;
  // The concept of shadowing which basically means that we can create a new variable
  // using an existing name.

  // One intresting data type which we can see in Rust is Tuples.

   let tup = ("Fenil",5411);
   let (channelname, count) = tup;
   let count = tup.1;
   // we can create a tuple and then decouple it by allocating the values to separate variables.
   print!("The count is {}",count);

   // lets set arrays in Rust

   let error_codes = [10,44,55];

        customadd(88, 10); 
        controlflow();


}


fn customadd(x:i32,y:i32){
   let sum = x+y;
   print!("The addition Result is {}",sum);
}


fn controlflow(){

  // the Best way to setup loop in Rust is to use the loop keyword

  loop {
      print!("You have reached the looping statement");
  }

}