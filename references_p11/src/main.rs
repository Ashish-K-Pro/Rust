fn main() {

// references are to store the address in Rust

let x = 10;

let xr = &x;

println!("The valuse of x is: {}",xr);

// mutable reference

let mut y = 10;

let dom = &mut y;

*dom += 1;

println!("Dom here is : {}",dom);

}
