use std::collections::HashMap;

fn main() {

let mut marks = HashMap::new();

marks.insert("Rust Programming",96);
marks.insert("Mobile Development",94);
marks.insert("Web Development",90);
marks.insert("Backend Development",74);


// finding length

println!("How many subjects you studied: {}",marks.len());



// Get a single value
match marks.get("Web Development"){
Some(Mark) => println!("You got {} for web Development",Mark),
None => println!("You didn't study web development.")
}

// Remove a value

marks.remove("Mobile Development");


// Loop through hash HashMap

for (subject, mark) in &marks {

println!("{} , {}", subject,mark);
}

// contains key method  

}
