use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::create_empty_response;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};

use serenity::cache::CacheRwLock;
use serenity::client::Context;
use serenity::http::{CacheHttp, Http};
use serenity::model::id::GuildId;

use crate::futures::{future, Future, Stream};
use hyper::{Body, HeaderMap, Method, Response, StatusCode, Uri, Version};

use serde_json::Value;

use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

/// Request counting struct, used to track the number of requests made.
///
/// Due to being shared across many worker threads, the internal counter
/// is bound inside an `Arc` (to enable sharing) and a `Mutex` (to enable
/// modification from multiple threads safely).
///
/// This struct must implement `Clone` and `StateData` to be applicable
/// for use with the `StateMiddleware`, and be shared via `Middleware`.
#[derive(Clone, StateData)]
struct SerenityCache {
    context: Arc<Mutex<Context>>,
}

/// Counter implementation.
impl SerenityCache {
    /// Creates a new request counter, setting the base state to `0`.
    fn new(context: &Context) -> Self {
        Self {
            context: Arc::new(Mutex::new(context.clone())),
        }
    }
}

/// Extract the main elements of the request except for the `Body`
fn print_request_elements(state: &State) {
    let method = Method::borrow_from(state);
    let uri = Uri::borrow_from(state);
    let http_version = Version::borrow_from(state);
    let headers = HeaderMap::borrow_from(state);
    println!("Method: {:?}", method);
    println!("URI: {:?}", uri);
    println!("HTTP Version: {:?}", http_version);
    println!("Headers: {:?}", headers);
}

/// TODO: Validate travis api token
fn post_assign_role(mut state: State) -> Box<HandlerFuture> {
    print_request_elements(&state);
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content: Value =
                    serde_json::from_str(&String::from_utf8(valid_body.to_vec()).unwrap()).unwrap();
                println!("Body: {:?}", body_content);
                let context = SerenityCache::borrow_from(&state).context.lock().unwrap();

                if let Some(travis_env) = body_content.get("global_env").unwrap().as_object() {
                    let guild_id = travis_env
                        .get("guild_id")
                        .unwrap()
                        .as_u64()
                        .expect("guild_id");
                    let user_id = travis_env
                        .get("user_id")
                        .unwrap()
                        .as_u64()
                        .expect("user_id");
                    let role = travis_env.get("role").unwrap().as_str().expect("role");

                    let guild = context.cache.read().guild(guild_id).unwrap();
                    let guild_read = guild.read();
                    let role = guild_read.role_by_name(role).unwrap();
                    guild_read
                        .member(context.http.as_ref(), user_id)
                        .unwrap()
                        .add_role(&context.http, role);
                    drop(context);

                    let res = create_empty_response(&state, StatusCode::OK);
                    future::ok((state, res))
                } else {
                    drop(context);
                    future::err((state, std::io::Error::last_os_error().into_handler_error()))
                }
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

/// Create a `Router`
///
/// /products?name=...             --> GET
pub fn router(context: &Context) -> Router {
    // create the counter to share across handlers
    let _context = SerenityCache::new(context);

    // create our state middleware to share the counter
    let middleware = StateMiddleware::new(_context);

    // create a middleware pipeline from our middleware
    let pipeline = single_middleware(middleware);

    // construct a basic chain from our pipeline
    let (chain, pipelines) = single_pipeline(pipeline);
    build_router(chain, pipelines, |route| {
        route
            .post("/role")
            // This tells the Router that for requests which match this route that query string
            // extraction should be invoked storing the result in a `QueryStringExtractor` instance.
            .to(post_assign_role);
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use gotham::test::TestServer;
//     use hyper::StatusCode;

//     #[test]
//     fn product_name_is_extracted() {
//         let test_server = TestServer::new(router()).unwrap();
//         let response = test_server
//             .client()
//             .get("http://localhost/products?name=t-shirt")
//             .perform()
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);

//         let body = response.read_body().unwrap();
//         let expected_product = Product {
//             name: "t-shirt".to_string(),
//         };
//         let expected_body = serde_json::to_string(&expected_product).expect("serialized product");
//         assert_eq!(&body[..], expected_body.as_bytes());
//     }
// }
