# Assignment 10 - Rust

This program takes a text file containing four fields delimited by commas to be inserted into HashMap, with the first field will consist of the key and the three remaining fields (with the first field) will contain the value for the key-value pair, for each line of the text file. Following is a continuous loop for the user to insert a key to be searched from the HashMap created from the text file; if key is found, will print out the value for the key, if not then will print a statement saying otherwise. Once the user is finished with the program, they are to insert Cntl-D as an argument to end the program. 

## Setup

The Rust program will be run using 'cargo'

## Running

From the command line:
```
cargo run main.rs file_name.txt
```

From the command line (if error):
```
cargo build
target/debug/hw10 file_name.txt
```

## Notes
Replace 'file_name.txt' with desired file to operate on.

File must be put into program's directory to be accessed for this program.

Program compiles and runs with no errors.
