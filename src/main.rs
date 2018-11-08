extern crate reqwest;



fn main() {
    match reqwest::get("http://www.google.com") {
      Ok(mut response) => {
    // Check if 200 OK
        if response.status() == reqwest::StatusCode::Ok {
            match response.text(){
              Ok(text) => println!("Response Text: {}",text),
              Err(_) => println!("Could not read response text!")
                               }
                               }
                                 else{
                                   println!("Response was not 200 Ok.");
                                      }
                                    }
          Err(_) => println!("Could not make the request!")


                              }

                           }
