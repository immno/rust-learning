use std::ops::Add;

use anyhow::Result;
use axum::{
    body::Body,
    http::Request,
    http::StatusCode,
    http::Uri,
    Router,
    routing::get,
};
use clap::Parser;
use colored::*;
use mime::Mime;

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);

    // build our application with a single route
    let app = Router::new()
        .route(&opts.context, get(get_handler).post(post_handler));

    // // 运行 web 服务器
    let addr = String::from("0.0.0.0:") + &opts.port.to_string();
    println!("Listening on {}", addr);

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 处理get请求
async fn get_handler(req: Request<Body>) -> Result<&'static str, StatusCode> {
    print_req(&req).await;
    Ok("ok")
}

// 处理POST请求
async fn post_handler(req: Request<Body>) -> Result<&'static str, StatusCode> {
    print_req(&req).await;
    Ok("ok")
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
    if context.eq(&root) {
        return Ok(root);
    }
    let mut path = String::from("");
    if !context.eq(&root) && !context.starts_with(&root) {
        path.push_str(&root);
        path.push_str(context);
    }
    if !context.eq(&root) && !context.ends_with(&root) {
        path.push_str(&root);
    }
    Ok(path)
}


// 打印整个请求
async fn print_req(req: &Request<Body>) -> Result<()> {
    let method = req.method().as_str();
    let uri = req.uri().to_string();
    println!("{} {} \n", method, uri);

    print_status(req);
    print_headers(req);
    // let mime = get_content_type(&req);
    let body = req.body();
    // print_body(mime, &body);
    Ok(())
}

// 打印服务器版本号 + 状态码
fn print_status(req: &Request<Body>) {
    let status = format!("{:?}", req.version()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的 HTTP header
fn print_headers(req: &Request<Body>) {
    for (name, value) in req.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}

// 打印服务器请求的 HTTP body
// fn print_body(m: Option<Mime>, body: &Body) {
//     body.data();
//     match m {
//         // 对于 "application/json" 我们 pretty print
//         Some(v) if v == mime::APPLICATION_JSON => {
//             println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
//         }
//         // 其它 mime type，我们就直接输出
//         _ => println!("{}", body),
//     }
// }


//
// 将服务器返回的 content-type 解析成 Mime 类型
// fn get_content_type(resp: &Request<Body>) -> Option<Mime> {
//     resp.headers()
//         .get(header::CONTENT_TYPE)
//         .map(|v| v.to_str().unwrap().parse().unwrap())
// }