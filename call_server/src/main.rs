use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    body::{Body, Bytes},
    error_handling::HandleErrorLayer,
    http::{header, Request, StatusCode},
    Router,
    routing::{get, post},
};
use clap::Parser;
use colored::*;
use mime::Mime;
use tower::{BoxError, filter::AsyncFilterLayer, ServiceBuilder};

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);

    // build our application with a single route
    let app = Router::new()
        .route(&opts.context, get(|| async move { "Ok" }))
        .route(&opts.context, post(|| async move { "Ok" }))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                }))
                // .layer(AndThenLayer::new(map_response))
                .layer(AsyncFilterLayer::new(map_request)),
        );

    // // 运行 web 服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], opts.port));
    println!("Listening on {:?}{}\n", addr, opts.context);

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn map_request(req: Request<Body>) -> Result<Request<Body>, BoxError> {
    print_req(&req);
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print(body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    Ok(req)
}

async fn buffer_and_print<B>(body: B) -> Result<Bytes, BoxError>
    where
        B: axum::body::HttpBody<Data=Bytes>,
        B::Error: Into<BoxError>,
{
    let bytes = hyper::body::to_bytes(body).await.map_err(Into::into)?;
    if let Ok(body) = std::str::from_utf8(&bytes) {
        println!("{}\n", jsonxf::pretty_print(body).unwrap().cyan())
        // tracing::debug!("{:?}\n",  body);
    }
    Ok(bytes)
}


/// 回调服务器；用来测试非结构化产品的数据订阅功能。
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "陶术江")]
struct Opts {
    /// 服务器端口: localhost:{port}
    #[clap(short, long, default_value = "8888")]
    port: u16,
    /// 服务器的context：localhost:{prost}/{context}/；接收时只会处理/{context}这个路径，比如/{context}/或者/{context}/zz都不会处理
    #[clap(short, long, default_value = "/")]
    #[clap(parse(try_from_str = parse_url))]
    context: String,
}

fn parse_url(context: &str) -> Result<String> {
    let root = String::from("/");
    let mut path = String::from("");
    if context.starts_with(&root) {
        path.push_str(context);
    } else {
        path.push_str(&root);
        path.push_str(context);
    }
    if !context.ends_with(&root) {
        path.push_str(&root);
    }
    Ok(path)
}

// 打印整个请求
fn print_req(req: &Request<Body>) {
    print_status(req);
    print_uri(req);
    // print_headers(req);
}

// 打印服务器版本号 + 状态码
fn print_status(req: &Request<Body>) {
    let mime = get_content_type(req);
    let status = format!("{:?} {}", req.version(), mime.as_ref().map_or("", |m| m.essence_str())).blue();
    println!("{}", status);
}

// 打印服务器版本号 + 状态码
fn print_uri(req: &Request<Body>) {
    let method = req.method().as_str();
    let uri = req.uri().to_string();
    println!("{} {}", method.green(), uri.green());
}

// 打印服务器返回的 HTTP header
#[allow(dead_code)]
fn print_headers(req: &Request<Body>) {
    for (name, value) in req.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}

// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Request<Body>) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}