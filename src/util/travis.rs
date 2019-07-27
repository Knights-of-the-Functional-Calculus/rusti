use std::env;

pub fn post_travis_repo(
    resource: &str,
    owner: &str,
    repo: &str,
    action: &str,
    body: Option<&serde_json::Value>,
) -> serde_json::Value {
    let req = reqwest::Client::new()
        .post(&format!(
            "https://api.travis-ci.com/{}/{}%2F{}/{}",
            resource, owner, repo, action
        ))
        .header("Travis-API-Version", "3")
        .header(
            "Authorization",
            format!("token {}", env::var("TRAVIS_TOKEN").unwrap()).as_str(),
        );
    let mut resp = if let Some(b) = body {
        req.json(b).send().unwrap()
    } else {
        req.send().unwrap()
    };

    println!("{:?}", resp);

    if let Ok(val) = resp.json() {
        println!("{:?}", val);
        return val;
    }
    serde_json::Value::Null
}
