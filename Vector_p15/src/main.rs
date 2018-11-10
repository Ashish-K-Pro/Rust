fn main() {

let mut myVector = vec![10,20,30,40,50];

myVector.push(10);
myVector.remove(1); // removes 20

for number in myVector.iter(){
println!("{}", number);
}

}
