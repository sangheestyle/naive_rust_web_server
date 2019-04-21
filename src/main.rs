#![feature(async_await, futures_api)]

use tide::error::ResultExt;

async fn hello(cx: tide::Context<()>) -> tide::EndpointResult<String> {
    let user: String = cx.param("user").client_err()?;
    Ok(format!("Hello, {}!", user))
}

async fn goodbye(cx: tide::Context<()>) -> tide::EndpointResult<String> {
    let user: String = cx.param("user").client_err()?;
    Ok(format!("Goodbye, {}.", user))
}

fn main() {
    let mut app = tide::App::new(());

    app.at("/hello/:user").get(hello);
    app.at("/goodbye/:user").get(goodbye);
    app.at("/").get(async move |_| {
        "Use /hello/{your name} or /goodbye/{your name}"
    });

    app.serve("127.0.0.1:8000").unwrap();
}
