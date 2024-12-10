use actix_web::{get, web::{self, ServiceConfig}, HttpResponse};
use rand::Rng;
use serde_json::json;
use actix_web::ResponseError;
use serde::{Serialize, Deserialize};
use shuttle_actix_web::ShuttleActixWeb;
use thiserror::Error as ThisError;
use actix_files::NamedFile;

#[derive(Serialize)]
struct RandomNumber {
    number: u8,
}

#[derive(Deserialize)]
struct RandomNumberParams {
    min: Option<u8>,
    max: Option<u8>,
}

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
    age: i32,
    bio: Option<String>,
}

#[derive(Deserialize)]
struct UserFilter {
    id: Option<i32>,
    name: Option<String>,
    age: Option<i32>,
}

#[derive(Serialize)]
struct Post {
    id: i32,
    title: String,
    content: String,
    author_id: i32,
}

#[derive(Deserialize)]
struct PostFilter {
    id: Option<i32>,
    title: Option<String>,
    content: Option<String>,
    author_id: Option<i32>,
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Invalid range: {0}")]
    InvalidRange(String),
    #[error("User with id {0} not found")]
    UserNotFound(i32),
    #[error("Post with id {0} not found")]
    PostNotFound(i32),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InvalidRange(details) => HttpResponse::BadRequest().json(json!({
                "error": "Invalid range",
                "details": details
            })),
            Error::UserNotFound(id) => HttpResponse::NotFound().json(json!({
                "error": "User not found",
                "details": format!("User with id {} not found", id)
            })),
            Error::PostNotFound(id) => HttpResponse::NotFound().json(json!({
                "error": "Post not found",
                "details": format!("Post with id {} not found", id)
            })),
        }
    }
}

#[get("/random-number")]
async fn random_number(params: web::Query<RandomNumberParams>) -> Result<HttpResponse, Error> {
    let min = params.min.unwrap_or(0);
    let max = params.max.unwrap_or(255);

    if min > max {
        return Err(Error::InvalidRange(
            "Minimum value cannot be greater than maximum value.".to_string(),
        ));
    }

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(min..=max);

    Ok(HttpResponse::Ok().json(RandomNumber { number: random_number }))
}

#[get("/users")]
async fn get_users(filter: web::Query<UserFilter>) -> Result<HttpResponse, Error> {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            age: 25,
            bio: Some("I am a software developer".to_string()),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            age: 30,
            bio: None,
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            age: 35,
            bio: Some("I am a data scientist".to_string()),
        },
        User {
            id: 4,
            name: "David".to_string(),
            age: 40,
            bio: None,
        },
        User {
            id: 5,
            name: "Eve".to_string(),
            age: 45,
            bio: Some("I am a security analyst".to_string()),
        },
        User {
            id: 6,
            name: "Another Alice".to_string(),
            age: 25,
            bio: Some("I am also a software developer".to_string()),
        },
    ];

    let filtered_users = users.into_iter().filter(|user| {
        let id_match = filter.id.map_or(true, |id| user.id == id);
        let name_match = filter.name.as_ref().map_or(true, |name| user.name == *name);
        let age_match = filter.age.map_or(true, |age| user.age == age);

        id_match && name_match && age_match
    });

    Ok(HttpResponse::Ok().json(filtered_users.collect::<Vec<User>>()))
}

#[get("/posts")]
async fn get_posts(filter: web::Query<PostFilter>) -> Result<HttpResponse, Error> {
    let posts = vec![
        Post {
            id: 1,
            title: "Post 1".to_string(),
            content: "Content of post 1".to_string(),
            author_id: 1,
        },
        Post {
            id: 2,
            title: "Post 2".to_string(),
            content: "Content of post 2".to_string(),
            author_id: 2,
        },
        Post {
            id: 3,
            title: "Post 3".to_string(),
            content: "Content of post 3".to_string(),
            author_id: 3,
        },
        Post {
            id: 4,
            title: "Post 4".to_string(),
            content: "Content of post 4".to_string(),
            author_id: 4,
        },
        Post {
            id: 5,
            title: "Post 5".to_string(),
            content: "Content of post 5".to_string(),
            author_id: 5,
        },
        Post {
            id: 6,
            title: "Post 6".to_string(),
            content: "Content of post 6".to_string(),
            author_id: 1,
        },
        Post {
            id: 7,
            title: "Post 7".to_string(),
            content: "Content of post 7".to_string(),
            author_id: 2,
        },
        Post {
            id: 8,
            title: "Post 8".to_string(),
            content: "Content of post 8".to_string(),
            author_id: 3,
        },
        Post {
            id: 9,
            title: "Post 9".to_string(),
            content: "Content of post 9".to_string(),
            author_id: 4,
        },
    ];

    let filtered_posts = posts.into_iter().filter(|post| {
        let id_match = filter.id.map_or(true, |id| post.id == id);
        let title_match = filter.title.as_ref().map_or(true, |title| post.title == *title);
        let content_match = filter.content.as_ref().map_or(true, |content| post.content == *content);
        let author_id_match = filter.author_id.map_or(true, |author_id| post.author_id == author_id);

        id_match && title_match && content_match && author_id_match
    });

    Ok(HttpResponse::Ok().json(filtered_posts.collect::<Vec<Post>>()))
}

#[get("/users/{id}")]
async fn get_user_by_id(path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            age: 25,
            bio: Some("I am a software developer".to_string()),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            age: 30,
            bio: None,
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            age: 35,
            bio: Some("I am a data scientist".to_string()),
        },
        User {
            id: 4,
            name: "David".to_string(),
            age: 40,
            bio: None,
        },
        User {
            id: 5,
            name: "Eve".to_string(),
            age: 45,
            bio: Some("I am a security analyst".to_string()),
        },
    ];

    let id = *path;

    let user = users.into_iter().find(|user| user.id == id).ok_or(Error::UserNotFound(id))?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/posts/{id}")]
async fn get_post_by_id(path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let posts = vec![
        Post {
            id: 1,
            title: "Post 1".to_string(),
            content: "Content of post 1".to_string(),
            author_id: 1,
        },
        Post {
            id: 2,
            title: "Post 2".to_string(),
            content: "Content of post 2".to_string(),
            author_id: 2,
        },
        Post {
            id: 3,
            title: "Post 3".to_string(),
            content: "Content of post 3".to_string(),
            author_id: 3,
        },
        Post {
            id: 4,
            title: "Post 4".to_string(),
            content: "Content of post 4".to_string(),
            author_id: 4,
        },
        Post {
            id: 5,
            title: "Post 5".to_string(),
            content: "Content of post 5".to_string(),
            author_id: 5,
        },
        Post {
            id: 6,
            title: "Post 6".to_string(),
            content: "Content of post 6".to_string(),
            author_id: 1,
        },
        Post {
            id: 7,
            title: "Post 7".to_string(),
            content: "Content of post 7".to_string(),
            author_id: 2,
        },
        Post {
            id: 8,
            title: "Post 8".to_string(),
            content: "Content of post 8".to_string(),
            author_id: 3,
        },
        Post {
            id: 9,
            title: "Post 9".to_string(),
            content: "Content of post 9".to_string(),
            author_id: 4,
        },
    ];

    let id = *path;

    let post = posts.into_iter().find(|post| post.id == id).ok_or(Error::PostNotFound(id))?;
    Ok(HttpResponse::Ok().json(post))
}

#[get("/api-docs")]
async fn api_docs() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("static/api-docs.html").map_err(|_| Error::InvalidRange("File not found".to_string()))?)
}
#[get("/api-tutorial")]
async fn api_tutorial() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("static/api-tutorial.html").map_err(|_| Error::InvalidRange("File not found".to_string()))?)
}

#[get("/")]
async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("static/index.html").map_err(|_| Error::InvalidRange("File not found".to_string()))?)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(random_number);
        cfg.service(get_users);
        cfg.service(get_posts);
        cfg.service(get_user_by_id);
        cfg.service(get_post_by_id);
        cfg.service(api_docs);
        cfg.service(api_tutorial);
        cfg.service(index);
    };

    Ok(config.into())
}
