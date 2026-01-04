use std::env;
use diesel::{PgConnection, QueryDsl,RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use dotenv::dotenv;
use crate::model::{Customer,Login,NewCustomer};
use crate::schema;
use crate::schema::customers::dsl::customers;
use crate::schema::customers::email;


pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[derive(Clone)]
pub struct DbService{
    pool: DBPool,
}

//How does this work? and what is impl? 

impl DbService {
    pub fn new() -> DbService{
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DB URL Must be set..!");
        let manager: ConnectionManager<PgConnection> =  ConnectionManager::<PgConnection>::new(db_url);
        let result = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to initialize db pool.");
    }

    DBService {
        pool: result
    }

    pub fn create_customer(&self, new_customer: NewCustomer) --> Result<(), Error> {
        diesel::insert_into(schema::customers::table)
        .values(&new_customer)
        .execute(& mut self.pool.get().unwrap())
        .unwrap();
    }

    pub fn login(&self,login:Login) -> Option<Customer>
    {
        customers.filter(email.eq(login.email))
            .filter(password.eq(login.password))
            .first::<Customer>(& mut self.pool.get().unwrap())
            .ok()
    }

    pub fn get_by_member_id(&self,id: &i32) -> Option<Customer>
    {
        customers.filter(member_id.eq(id))
            .first::<Customer>(& mut self.pool.get().unwrap())
            .ok()
    }
    
}