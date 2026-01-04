use serde::Serialize;

#[derive(Queryable, Serialize, Debug)]
pub struct Customer {
    pub member_id : i32,
    pub name: String,
    pub password: String,
    pub email: String
}


impl Customer {
    pub fn to_response(&self) -> ProfileDetails{
        ProfileDetails{
            name: self.name.clone(),
            email: self.email.clone()
        }

    }
}


#[derive(Insertable,Desrialize)]
#[diesel(table_name=customers)]
pub struct NewCustomer {
    pub name: String,
    pub password: String,
    pub email: String
}

#[derive(Desrialize)]
pub struct Login {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Debug)]
pub struct ProfileDetails {
    pub name: String, 
    pub email: String
}




