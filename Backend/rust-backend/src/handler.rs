use actix_web::web;
use crate::db_service::DbService;

pub async fn register(db:web::Data<DbService>,customer: web::Json<NewCustomer>)
-> HttpResponse
{
    //let mut data = 

}

#[post("/login")]
//Code for Login 
pub async fn login_user(db:web::Data<DbService>,login:web::Json<Login>)
-> HttpResponse
{
    let mut data:Login = login.into_inner();
    let hash: String = hash_password(data.hash_password);

    data.password = hash; //This is the hash needed 

    match db.login(data){
        Some(detail:Customer) =>
            let cookie = Cookie::build("token", generate_jwt
        (
            detail.member_id,"MY_SECRET_DEV"
        ).unwrap())
            .path("/")
            .max_age(Duration::seconds(3600))
            .same_site(SameSite::None)
            .http_only(true)
            .finish();

        HttpResponse::Ok()
            .cookie(cookie)
            .finish()
    },
    None => HttpResponse::InternalServerError().body("Try Again...!")
}

//Check the code above.
//Post request to profile

#[post("/profile")]
async fn profile(db: web::Data<DbService>, req: HttpRequest) -> HttpResponse
{
    if let Some(cookie) = req.cookie("token") {
        match decode_token(cookie.value(),"MY_SECRET_DEV")
        {
            Ok(token_data) => 
            {
                if isvalid(&token_data.claims)
                {
                   match db.get_by_member_id(&token_data.claims.sub) 
                   {
                    None => {
                        HttpResponse::NotFound().body("not found")
                    }, 
                    Some(Customer) => {
                        HttpResponse::Ok().json(customer.to_response())
                    }


                   }
                }

            }
        }

        
    }

}

//Lets finish this one here.

//handler.rs