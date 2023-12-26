#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    name: String,
    role: String,
    created_at: u64,
    updated_at: Option<u64>,
}

impl User {
    fn new(payload: UserPayload, user_id: u64) -> User {        
        User {
            id: user_id,
            name: payload.name,
            role: payload.role,
            created_at: time(),
            updated_at: None,
        }
    }
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Article {
    id: u64,
    writer_id: u64,
    reviewer_id: Option<u64>,
    title: String,
    state: String,
    content: String,
    created_at: u64,
    updated_at: Option<u64>,
}

impl Article {
    fn new(article: ArticlePayload, article_id: u64) -> Article {
        Article {
            id: article_id,
            writer_id: article.writer_id,
            reviewer_id: None,
            state: String::from("draft"),
            title: article.title,
            content: article.content,
            created_at: time(),
            updated_at: None,
        }
    }
}

pub trait Summarizable {
    fn summary(&self) -> String;
}

impl Summarizable for Article {
    fn summary(&self) -> String {
        match _get_user(&self.writer_id) {
            Some(user) => {
                format!("{} by {}", self.title, user.name)
            },
            None => "Couldn't verify article's author!".to_string()
        }
    }
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for Article {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for Article {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static USER_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ARTICLE_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static USER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(USER_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static ARTICLE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(ARTICLE_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a counter")
    );

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            USER_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static ARTICLE_STORAGE: RefCell<StableBTreeMap<u64, Article, Memory>> =
        RefCell::new(StableBTreeMap::init(
            ARTICLE_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UserPayload {
    name: String,
    role: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct ArticlePayload {
    writer_id: u64,
    title: String,
    content: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UserUpdatePayload {
    name: String,
    role: String,
}

#[ic_cdk::query]
fn get_user(id: u64) -> Result<User, Error> {
    match _get_user(&id) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("a user with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn add_user(user: UserPayload) -> Result<User, Error> {
    let id = USER_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let user_role = &user.role;
    if !(user_role == "user" || user_role == "writer" || user_role == "reviewer") {
        return Err(Error::ServerError {
            msg: format!("Incorrect role input. Role should be either 'user', 'writer' or 'reviewer'"),
        });
    }

    let user = User::new(user, id);
    do_insert_user(&user);
    Ok(user)
}

#[ic_cdk::update]
fn update_user(id: u64, payload: UserUpdatePayload) -> Result<User, Error> {
    match USER_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut user) => {
            if payload.name.len() > 1 {
                user.name = payload.name;
            }
            if payload.role.len() > 1 {
                user.role = payload.role;
            }
            user.updated_at = Some(time());

            do_insert_user(&user);
            Ok(user)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a user with id={}. user not found",
                id
            ),
        }),
    }
}

// helper method to perform insert.
fn do_insert_user(user: &User) {
    USER_STORAGE.with(|service| service.borrow_mut().insert(user.id, user.clone()));
}

#[ic_cdk::update]
fn delete_user(id: u64) -> Result<User, Error> {
    match USER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a user with id={}. user not found.",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn add_article(payload: ArticlePayload) -> Result<Article, Error> {
    let id = ARTICLE_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment article id counter");
        
        if !verify_writer(payload.writer_id) {
            return Err(Error::ServerError {
                msg: format!(
                    "Role of the submitted user with id {} is not writer. Please update role or create a new user with 'writer' role.",
                    payload.writer_id
                ),
            });
        }
        let article = Article::new(payload, id);
        do_insert_article(&article);
        Ok(article)
}

#[ic_cdk::update]
fn request_review(writer_id: u64, article_id: u64) -> Result<Article, Error> {
    if !verify_writer(writer_id) {
        return Err(Error::ServerError {
            msg: format!(
                "Couldn't verify writer with user id {}.", writer_id
            ),
        });
    }

    match _get_article(&article_id) {
        Some(mut article) => {
            if article.writer_id != writer_id {
                return Err(Error::ServerError {
                    msg: format!(
                        "Writer with id {} is not the author of this article.",
                        article_id
                    ),
                })
            }

            if article.state == "pending_review" {
                return Err(Error::ServerError {
                    msg: format!(
                        "Article with id {} is already in pending review state.",
                        article_id
                    ),
                })
            }

            article.state = "pending_review".to_string();
            do_insert_article(&article);
            Ok(article)
        }
        None => Err(Error::NotFound {
                    msg: format!(
                        "couldn't find the article with id {}.",
                        article_id
                    ),
                }),
    } 
}

// Helper method to verify writer
fn verify_writer(writer_id: u64) -> bool {
    match _get_user(&writer_id) {
        Some(user) => {
            if user.role == "writer" {
                return true;
            }
            false
        }
        None => false
    }
}

#[ic_cdk::update]
fn approve(reviewer_id: u64, article_id: u64) -> Result<Article, Error> {
    if !verify_reviewer(reviewer_id) {
        return Err(Error::ServerError {
            msg: format!(
                "Couldn't verify reviewer with user id {}.", reviewer_id
            ),
        });
    }

    match _get_article(&article_id) {
        Some(mut article) => {
            if article.state == "published" {
                return Err(Error::ServerError {
                    msg: format!(
                        "Article with id {} is already pubished!",
                        article_id
                    ),
                })
            }

            article.state = "published".to_string();
            article.reviewer_id = Some(reviewer_id);
            do_insert_article(&article);
            Ok(article)
        }
        None => Err(Error::NotFound {
                    msg: format!(
                        "couldn't find the article with id {}.",
                        article_id
                    ),
                }),
    } 
}

// Helper method to verify reviewer
fn verify_reviewer(reviewer_id: u64) -> bool {
    match _get_user(&reviewer_id) {
        Some(user) => {
            if user.role == "reviewer" {
                return true;
            }
            false
        }
        None => false
    }
}

#[ic_cdk::query]
fn get_article(id: u64) -> Result<Article, Error> {
    match _get_article(&id) {
        Some(article) => {
            if article.state != "published" {
                return Err(Error::ServerError {
                    msg: format!("Article with id={} is yet to be published!", id),
                })
            }
            Ok(article)
            
        },
        None => Err(Error::NotFound {
            msg: format!("Article with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_article_summary(id: u64) -> Result<String, Error> {
    match _get_article(&id) {
        Some(article) => {
            Ok(article.summary())
        },
        None => Err(Error::NotFound {
            msg: format!("Article with id {} not found", id),
        }),
    }
}

// helper method to perform article insert.
fn do_insert_article(article: &Article) {
    ARTICLE_STORAGE.with(|service| service.borrow_mut().insert(article.id, article.clone()));
}

// a helper method to get witer's article
fn _get_article(article_id: &u64) -> Option<Article> {
    ARTICLE_STORAGE.with(|service| service.borrow().get(article_id))
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    Exists { msg: String },
    ServerError { msg: String }
}

// a helper method to get a user by id. used in get_user/update_user
fn _get_user(id: &u64) -> Option<User> {
    USER_STORAGE.with(|service| service.borrow().get(id))
}

// need this to generate candid
ic_cdk::export_candid!();