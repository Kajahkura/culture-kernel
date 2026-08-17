use blinkhost_sdk::{respond, Response};

fn main() {
    if respond(Response::json(
        200,
        r#"{"language":"rust","canary":true}"#,
    ))
    .is_err()
    {
        std::process::abort();
    }
}
