use actix_web::{middleware, web, App, HttpServer};
use actix_files::Files;

use std::env::set_var;

mod config;
mod security;
mod template;
mod setssl;
mod serve;
mod redirect;

use config::{setup_config, get_config};
use security::check_config_security;
use template::render_templates;
use setssl::getssl_config;
use serve::{serve_upload, serve_routed};
use redirect::RedirectHTTPS;

fn server_init() {
    // setup server configuration
    setup_config();

    // validate routes for security vulnerabilities
    check_config_security();

    // render templates
    render_templates();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialize server config & files
    server_init();

    let config = get_config();

    // enable/disable logging
    if config["enable_logging"].as_bool().unwrap() {
        set_var("RUST_LOG", "actix_web=info");
        env_logger::init();
    }
    
    // ASCII art banner always looks cool
    println!(r"
         _____       _____              _____                     
        |   __| _ _ |  _  | ___  ___   |   __| ___  ___  _ _  ___ 
        |__   || | ||   __|| -_||  _|  |__   || -_||  _|| | || -_|
        |_____||___||__|   |___||_|    |_____||___||_|   \_/ |___|  v0.1.0
    ");

    let host = config["server"]["host"].to_string().replace("\"", "");
    let httpport = config["server"]["httpport"].as_u64().unwrap();
    let httpsport = match config["server"]["httpsport"].as_u64() {
        Some(u) => u,
        None => 0
    };

    let server = HttpServer::new(move || {
        App::new()
        // Enable the logger middleware.
        .wrap(middleware::Logger::default())
        // Compress using `brotli` alogorithm.
        .wrap(middleware::Compress::default())
        // Redirect all http requests to https.
        .wrap(RedirectHTTPS::with_replacements(httpport, httpsport))
        // // if the below `.service()` is uncommented, server will
        // // serve files from `static/assets/{filename}` as `static/{filename}`.
        // // Comes useful when you want to serve js/css files instead using cdn files.
        // .service(
        //     // `.show_files_listing()` mode is set when `directory_listing` is true in config
        //     match config["directory_listing"].as_bool().unwrap() {
        //         true => Files::new("/static", "static/assets/")
        //             .show_files_listing()
        //             .prefer_utf8(true)
        //             .use_last_modified(true),
        //         false => Files::new("/static", "static/assets/")
        //             .prefer_utf8(true)
        //             .use_last_modified(true)
        //     }
        // )
        // serve HTTP POST requests for the upload_api in config.
        .route(
            config["template_variables"]["upload_api_address"].as_str().unwrap(),
            web::post().to(serve_upload)
        )
        // serve uploaded image files from `uploaded_files` as `directed_to`
        // for example, `./tmp/*.png` will be accessed from `https://web.site/img/*.png`
        .service(
            Files::new(
                &format!(
                    "/{}",
                    config["uploaded_files"]["directed_to"].as_str().unwrap()
                ),
                &format!(
                    "{}/",
                    config["uploaded_files"]["raw_directory"].as_str().unwrap()
                )
            )
            .use_last_modified(true)
        )
        // serve HTTP GET requests with routing rules from `routes`
        .route("/{route:.*}", web::get().to(serve_routed))
    });

    if httpsport != 0 {
        println!(
            "\nYour server is up and running at https://{}:{}/",
            host, httpsport
        );
        println!(
            "All incoming requests at http://{}:{}/ will be redirected to https.\n",
            host, httpport
        );

        server
        .bind(format!("{}:{}", host, httpport))?
        .bind_rustls(format!("{}:{}", host, httpsport), getssl_config())?
        .run().await
    }
    else {
        println!(
            "\nYour server is up and running at http://{}:{}/",
            host, httpport
        );
        println!(
            "HTTPS is disabled; change `server->httpsport` in config file to enable\n",
        );

        server
        .bind(format!("{}:{}", host, httpport))?
        .run().await
    }
}
