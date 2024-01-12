use std::sync::{RwLockWriteGuard,RwLock};
use actix_web::{web::{self,Path, Json}, HttpResponse, Responder, HttpServer, App};
use serde::{Deserialize, Serialize};
use uuid::Uuid;





   #[derive(Deserialize)]
   pub struct EntityUuid{
       id:Uuid
   }
   
   #[derive(Deserialize, Serialize, Debug, Clone)]
   pub struct CombinedIds{
       id:Uuid,
       id_two:Uuid
   }
   
   #[derive(Deserialize, Serialize, Debug, Clone)]
   pub struct PostId{
       id: Uuid
   }
   #[derive(Deserialize, Serialize, Debug, Clone)]
   pub struct FinalPost{
       id: Uuid,
       body:String,
       comments:Vec<Comment>,
       author: User
   }
   
   #[derive(Deserialize, Serialize, Debug, Clone)]
   pub struct NewPost{
       body:String,
       author:User
   }
   #[derive(Deserialize, Serialize, Debug, Clone)]
   pub struct Comment{
       id:Uuid,
       post_id:Uuid,
       body:String,
       user_id:Uuid
   
   }
   
   #[derive(Deserialize, Serialize, Debug, Clone)]
   pub struct NewComment{
       post_id:Uuid,
       user_id:Uuid,
       body:String
   }
   #[derive(Deserialize, Serialize, Debug, Clone)]
   pub struct User{
       id: Uuid,
       first_name:String,
       last_name:String,
       email:String,
       user_name:String
   }
   #[derive(Deserialize, Serialize)]
   pub struct RegisterUser{
       first_name:String,
       last_name:String,
       email:String,
       user_name:String
   }
   
   #[derive(Deserialize, Serialize)]
   pub struct GenericReponse{
       message:String
   }
   
   
   
   struct AppDataState {
    users_v2: RwLock<Vec<User>>,
    posts: RwLock<Vec<FinalPost>>   
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let app_data: web::Data<AppDataState> = web::Data::new(AppDataState {
        users_v2: RwLock::new(vec![]),
        posts: RwLock::new(vec![]),
    });

    HttpServer::new(move ||{
        App::new()
            .app_data(app_data.clone())
            .service(
                web::scope("/account")
                .service(
                    web::resource("/user")
                         .route(web::post().to(insert_user))
                )
                .service(
                    web::resource("/user/{id}")
                        .route(web::get().to(get_user))
            )
        )
        .service(
            web::scope("/posts")
            .service(
                web::resource("/new")
                .route(web::post().to(create_post))
            )

            .service(
                web::resource("/all")
                .route(web::get().to(get_posts))
            )

            .service(
                web::resource("/{id}")
                .route(web::get().to(get_post))
                .route(web::delete().to(delete_post))
            )
            .service(
                web::scope("/{id}")
                .service(
                    web::resource("/comment")
                    .route(web::post().to(add_comment))
                    
                )

                .service(
                    web::resource("/all_comments")
                    .route(web::get().to(get_post_comments))
                )

                .service(
                    web::resource("/{id_two}")
                    .route(web::get().to(get_post_comment))
                    .route(web::delete().to(delete_comment))
            )
        )
    )           
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}

pub async fn insert_user(app_data: web::Data<AppDataState>, new_user:Json<RegisterUser>) -> impl Responder{
    println!("Posts create route");
    let mut users = app_data.users_v2.write().unwrap();
    users.push(User{
        id: Uuid::new_v4(),
        first_name:new_user.first_name.clone(), 
        last_name:new_user.last_name.clone(),
        email:new_user.email.clone(),
        user_name:new_user.user_name.clone()});

        // Use a reference to the last user in the response
    let response: Option<&User> = users.last().map(|u: &User| u as &User);

    HttpResponse::Created().json(response)
}

pub async fn get_user(app_data: web::Data<AppDataState>, params: web::Path<EntityUuid>) -> HttpResponse {
    println!("{:?}", "Get user route");

    let users: std::sync::RwLockReadGuard<'_, Vec<User>> = app_data.users_v2.read().unwrap();
  
    let response: HttpResponse = users
    .iter()
    .filter(|user| user.id == params.id)
    .map(User::clone)
    .next()
    .and_then(|user: User| Some(HttpResponse::Ok().json(user)))
    .map_or(HttpResponse::NotFound().finish(), |response: HttpResponse| response);

  return response

}


pub async fn create_post(app_data: web::Data<AppDataState>) -> HttpResponse {
    println!("{:?}", "Create post route");
    let mut posts: RwLockWriteGuard<'_, Vec<FinalPost>> = app_data.posts.write().unwrap();
    let new_post:FinalPost = FinalPost{ id: Uuid::new_v4(),
        body: "test".to_string(),
        comments:vec![],
        author: User { id: Uuid::new_v4(), first_name:"test".to_string(), last_name:"test".to_string(), email: "testmail".to_string(), user_name:"test".to_string()}
    };  
     posts
    .push(new_post);

    let response: Option<&FinalPost> = posts.last().map(|p: &FinalPost| p as &FinalPost);

    HttpResponse::Created().json(response)

}

pub async fn get_posts(app_data: web::Data<AppDataState>) -> HttpResponse {
    println!("{:?}", "Get all posts route");

    let posts: std::sync::RwLockReadGuard<'_, Vec<FinalPost>> = app_data.posts.read().unwrap();

    if posts.len() == 0{
        let not_found_message:GenericReponse = GenericReponse{message:"No Posts Found".to_string()};
        return HttpResponse::NoContent().json(not_found_message);
    } else {
    let final_posts: Vec<FinalPost> = posts.clone();
    return HttpResponse::Found().json(final_posts);
    }

}

pub async fn get_post(app_data: web::Data<AppDataState>, params:web::Path<PostId>) -> HttpResponse {
    println!("{:?}", "Get post route");
    let posts: std::sync::RwLockReadGuard<'_, Vec<FinalPost>> = app_data.posts.read().unwrap();
    let not_found_message:GenericReponse = GenericReponse{message:"Post Not Found".to_string()};
    let response: HttpResponse = posts
    .iter()
    .filter(|post: &&FinalPost| post.id == params.id)
    .map(FinalPost::clone)
    .next()
    .and_then(|post| Some(HttpResponse::Ok().json(post)))
    .map_or(HttpResponse::NotFound().json(not_found_message), |response| response);
  return response
}

pub async fn add_comment(app_data: web::Data<AppDataState>, params: web::Path<PostId>) -> HttpResponse {
    println!("{:?}", "Post comment route");

    let mut posts: std::sync::RwLockWriteGuard<'_, Vec<FinalPost>> = app_data.posts.write().unwrap();

    let error_message: GenericReponse = GenericReponse {
        message: "Requires more than 0 characters".to_string(),
    };

    if let Some(post) = posts.iter_mut().find(|p: &&mut FinalPost| p.id == params.id) {
        println!("{:?}", "Post found");

        let test_comment: Comment = Comment {
            id: Uuid::new_v4(),
            post_id: post.id,
            body: "Test".to_string(),
            user_id: Uuid::new_v4(),
        };

        if test_comment.body.len() > 0 {
            post.comments.push(test_comment.clone());
            println!("{:?}", "Comment added");
            HttpResponse::Created().json(test_comment)
        } else {
            println!("{:?}", "Empty comment body");
            HttpResponse::BadRequest().json(error_message)
        }
    } else {
        println!("{:?}", "Post not found");
        HttpResponse::NotFound().finish()
    }
}



pub async fn get_post_comments(app_data: web::Data<AppDataState>, params:web::Path<PostId>) -> HttpResponse{
    let posts: std::sync::RwLockReadGuard<'_, Vec<FinalPost>> = app_data.posts.read().unwrap();
    if let Some(post) = posts.iter().find(|p: &&FinalPost| p.id == params.id){
       let comments: Vec<Comment> = post.comments.clone();
      return  HttpResponse::Ok().json(comments);
    } else {
      return  HttpResponse::NotFound().finish();
    }
}

pub async fn get_post_comment(app_data: web::Data<AppDataState>, params: web::Path<CombinedIds>) -> HttpResponse {
    let posts: std::sync::RwLockReadGuard<'_, Vec<FinalPost>> = app_data.posts.read().unwrap();

    let error_message: GenericReponse = GenericReponse {
        message: "Could not find the comment".to_string()
    };
    
    if let Some(post) = posts.iter().find(|p: &&FinalPost| p.id == params.id){
        let comments: Vec<Comment> = post.comments.clone();

        println!("Post found: {:?}", post);  // Debug print

        if let Some(comment) = comments.iter().find(|c: &&Comment| c.id == params.id_two){
            println!("Comment found: {:?}", comment);  // Debug print
            return HttpResponse::Ok().json(comment);
        } else {
            println!("Comment not found");  // Debug print
            return HttpResponse::NotFound().json(error_message);
        }
    }

    println!("Post not found");  // Debug print
    return HttpResponse::NotFound().finish();
}


pub async fn delete_post(app_data: web::Data<AppDataState>, params: web::Path<EntityUuid>) -> HttpResponse{
    println!("{:?}", "Post delete route");
    let mut posts : std::sync::RwLockWriteGuard<'_, Vec<FinalPost>> = app_data.posts.write().unwrap();


    if let Some(index) = posts.iter().position(|p| p.id == params.id){
        println!("{:?}", "Found post to delete");
        posts.remove(index);
        HttpResponse::Ok().json("Delted post")
    } else {
        println!("{:?}", "Post to delete not found in vec in app state
        ");
       HttpResponse::NotFound().finish()
    }
}

pub async fn delete_comment(app_data: web::Data<AppDataState>, params: web::Path<CombinedIds>) -> HttpResponse {
    println!("{:?}", "Delete comment route");

    let mut posts: std::sync::RwLockWriteGuard<'_, Vec<FinalPost>> = app_data.posts.write().unwrap();

    if let Some(post) = posts.iter_mut().find(|p| p.id == params.id) {
        if let Some(index) = post.comments.iter().position(|c| c.id == params.id_two) {
            post.comments.remove(index);
            return HttpResponse::Ok().json("Deleted Comment")
        } else {
            return HttpResponse::NotFound().json("Could not find comment to delete")
        }
    }

    HttpResponse::NotFound().json("Could not find post")
}

