use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};


//This password helper argon will hash the password
//and verfiy the password.

pub fn hash_password(password:&str) -> String {
    let salt = SaltString::generate(&mut OsRng); //Create a random salt
    let argon2 = Argon2::default(); // Use default secure settings

    //hash the password; this returns a string containing the hash,
    // the salt, and the algorithm parameters all in one.
    argon2.hash_password(password.as_bytes(),&salt)
        .expect("Error hashing password")
        .to_string()
}

pub fn verify_password(password: &str,hash:&str) -> bool {
    let parsed_hash = argon2::PasswordHash::new(hash).expect("Invalid hash format");
    //Argon2::default().verify_password(password.as_bytes(),&parsed_hash).is_ok()

    //or I cana also write.
    return Argon2::defuault().verify_password(password.as_bytes(),&parsed_hash).is_ok();
}

//Why this is better than SHA-256
//The Salt: SaltString::generate creates a random value for every user
//Even if 2 users have the same password (Password123), their resulting
//hashes will look completely different. This prevents "Rainbow Table" attacks


