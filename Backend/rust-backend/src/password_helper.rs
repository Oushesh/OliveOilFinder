use sha2::{Sha256,Digest}; //Import Digest to get .finalize() and .update() methods
use sha2::digest::Update;

pub fn hash_password(password:String) -> String {

    // 1. Initialize the hasher
    let mut hascher = Sha256::new();

    // 2. Feed the data(as bytes) into the hasher
    hasher.update(password.as_bytes());

    // 3. Consume the hasher to produce the final hash result
    let result = hasher.finalize();

    // 4. Convert the binary hash into a readable hex string
    hex::encode(result) //in Rust the last expression in a function
    //is its return value if you omit the semicolon

    //same as above but less idiomatic.
    //return hex::encode(result);
}


//Analysis of the above function
//Samll Analysis of the function above: 

//Function hash_password -> Takes Argument password of type String
//returns type string.

//Lesson 2: What let vs let mu means
// In most programming languages, variables can be changed by default.
// In Rust the default is opposite: everything is immutable (read-only)

//In python, variables are essentially names pointing to objects.
//By default, you can point a name to a new object at any time.

//In python: you just write x=5 and then x=6. Its opt-out for protection.
//in python tuple or frozenset
//IN Rust you must write let x=5. if you must: "opt-in" let mut x=5

//Python Speed of writing, Rust focuses on predictability and safety.
//multithreading: This is the big one. If a variable is immutable, Rust knows its 
//100% safe. Rust knows its 100% safe for 10 different parts of your program to read it
// at the same time. if it were mutable by default, Rust would have to worry 
//about one part of the program changing the date while another is reading it.

//3. Readability: When you see let mut in a Rust function, it acts like 
// a warning light. You immediately know, "Okay, this specific variable is going 
//to be moving and changing"

///// Note on a better algorithm:
// Argon 2 > sha-256
//Advnatages: SHA-256 is extremely fast but can be brute forced or "Rainbow Tables to guess millions of passwords per second"

//Real Login System: Password Hashing Algorithm like 
//Argon2, bcrypt or scrypt.


//Salt: A random string added to each password
//to prevent identical passwords from having the same hash.

//Cost Factor: They are designed to be intentionally slow 
//to stop brute-force attacks.

