use clap::Parser;
use salvo::__private::tracing;
use salvo::compression::Compression;
use salvo::logging::Logger;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;

/// Static server - 基于 Rust 的静态资源服务器
#[derive(Parser, Debug)]
#[command(name = "r-static")]
#[command(author = "Rust Static Server Team")]
#[command(version = "0.2.0")]
#[command(about = "Rust 静态资源服务器，支持配置证书自动化等", long_about = None)]
struct Args {
    /// 静态服务监听端口
    #[arg(long, default_value = "0.0.0.0:8080")]
    http_addr: String,

    /// 静态服务监听端口
    #[arg(long, default_value = "0.0.0.0:8443")]
    https_addr: String,

    /// http3 开关
    #[arg(long)]
    http3_enable: bool,

    /// HTTPS 开关
    #[arg(long)]
    https_enable: bool,

    /// 强制 HTTPS (HTTP 请求重定向到 HTTPS)
    #[arg(long)]
    force_https: bool,

    /// 响应压缩 (默认启用)
    #[arg(long)]
    compression: bool,

    /// 自动签发证书
    #[arg(long)]
    acme_domain: Option<String>,

    /// 静态资源文件夹
    #[arg(long, default_values = &["static"])]
    static_dir: Vec<String>,

    /// 默认文件名 (当请求目录时)
    #[arg(long, default_value = "index.html")]
    try_file: String,

    /// 默认显示页面 (当找不到文件时)
    #[arg(long, default_value = "index.html")]
    default_page: String,

    /// 日志级别 (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,
}

fn initialize_logging(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => {
            eprintln!("无效的日志级别 '{}', 使用默认值 'info'", log_level);
            tracing::Level::INFO
        }
    };

    tracing_subscriber::fmt().with_max_level(level).init();
}

#[tokio::main]
async fn main() {
    // 解析命令行参数
    let args = Args::parse();
    initialize_logging(&args.log_level);

    let static_router = Router::with_path("{*path}").get(
        StaticDir::new(args.static_dir)
            .include_dot_files(false)
            .defaults(args.try_file) // 使用 try_file 作为默认文件
            .fallback(args.default_page)
            .auto_list(true),
    );

    let mut router = Router::new().hoop(Logger::new());

    if args.compression {
        // 启用压缩的路由
        router = router.push(Router::with_hoop(Compression::new()).push(static_router));
    } else {
        // 不压缩的路由
        router = router.push(static_router);
    }

    if args.https_enable {
        if let Some(ref acme_domain) = args.acme_domain {
            let https_listener = TcpListener::new(args.https_addr.clone())
                .acme()
                .cache_path("temp/letsencrypt")
                .add_domain(acme_domain.clone())
                .http01_challenge(&mut router);

            // 无论是否强制 HTTPS，都需要同时监听 HTTP 端口（用于跳转或直接访问）
            if args.http3_enable {
                let acceptor = https_listener
                    .quinn(args.https_addr.clone())
                    .join(TcpListener::new(args.http_addr.clone()))
                    .bind()
                    .await;
                Server::new(acceptor).serve(router).await;
            } else {
                let acceptor = https_listener
                    .join(TcpListener::new(args.http_addr.clone()))
                    .bind()
                    .await;
                Server::new(acceptor).serve(router).await;
            }
        } else {
            eprintln!("错误：启用 HTTPS 时必须提供 --acme-domain 参数");
            std::process::exit(1);
        }
    } else {
        // 仅 HTTP 模式
        let acceptor = TcpListener::new(args.http_addr.clone()).bind().await;
        Server::new(acceptor).serve(router).await;
    }
}
