fn main() {

// tuples are used to store different data types all together.

let tup1 = (20, "Rust",3.14, true);
println!("Second value in tuple is : {}",tup1.1);


// assign one tuple to another

let (x,y,z,boolean_Value) = tup1;
println!("X is : {}, y is : {}, and z is : {}",x,y,z);

}
