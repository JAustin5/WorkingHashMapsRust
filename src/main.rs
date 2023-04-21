use core::fmt;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead, self};

// struct for movie file information
struct MovieInfo {
    release_year: String,
    movie_title: String,
    actor_name: String,
    main_char: String
}

// print format for struct
impl fmt::Display for MovieInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}, {}", self.release_year, self.movie_title, self.actor_name, self.main_char)
    }
}

// getting information out from text file and inserting into HashMap
fn main(){
    let args: Vec<String> = std::env::args().collect();
    let file_n = File::open(&args[2])
        .expect("No such file found");
    let reader = BufReader::new(file_n);

    let mut map_info : HashMap<String, MovieInfo> = HashMap::new();

    for line in reader.lines() {
        let line_in = line
            .expect("Error occurred");
        let vals : Vec<&str> = line_in.split(",").collect();
        
        let line_info = MovieInfo {
            release_year: vals[0].to_string(),
            movie_title: vals[1].to_string(),
            actor_name: vals[2].to_string(),
            main_char: vals[3].to_string(),
        };
        
        map_info.insert(line_info.release_year.clone(), MovieInfo { release_year: line_info.release_year.clone(), movie_title: line_info.movie_title.clone(), actor_name: line_info.actor_name.clone(), main_char: line_info.main_char.clone()});
    }

    // looping to check each user input to see if value is in the HashMap or if exiting program with Cntl-D entered
    println!("Insert a year to go through file to see if it is accessible.\nEnter Cntl-D to exit program. \n");
    loop {
        let mut entered_val = String::new();
        if io::stdin().read_line(&mut entered_val).expect("Error") == 0 {
            println!("Program exit");
            break;
        }
        
        let entered_val: String = match entered_val.trim().parse() {
                Ok(year) => year,
                Err(_) => {
                    println!("Error occurred");
                    continue;
            }
        };
         
        match map_info.get(&entered_val) {
            Some(result) => println!("Movie details:\n{result}\n"),
            None => println!("No such year could be found. Try again.\n")
        }
    }
}
