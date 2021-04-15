# SuperServe

Minimal config Actix-based image hosting service. Backend code mainly based on [binserve](https://github.com/mufeedvh/binserve) with minor modification, and frontend driven by [Bootstrap-Fileinput](https://github.com/kartik-v/bootstrap-fileinput).

All credits to the open source community.

## Features

For backend:

- Speed. [Actix](https://actix.rs) has remained competent in [certain benchmarks](https://www.techempower.com/benchmarks/) compared to other backend frameworks.
- Portability. After getting acquainted with [Rust](https://www.rust-lang.org/learn/get-started), simply edit the configuration file, and run this code with `cargo run` will serve the job.
- Minimal configuration needed, and other features inherited from [binserve](https://github.com/mufeedvh/binserve), including:
    + Configuration in a single file;
    + Easiest Routing: change `routes` entry in json file for a new route entry;
    + [Handlebars](https://github.com/sunng87/handlebars-rust) template engine: renders every static file with `Handlebars` on the first run improving performance;
    + Secure by design. Runs security validation checks (featuring symlink checking) on the first run and will only run the server once configuration is confirmed secure. Check other features about security from [binserve/security](https://github.com/mufeedvh/binserve#security);
    + Custom Error Page Support: designing fancy error pages comes easy;
    + All features above should be credited to [mufeedvh](https://github.com/mufeedvh) and the other [contributors](https://github.com/mufeedvh/binserve/graphs/contributors) for [binserve](https://github.com/mufeedvh/binserve).
- HTTPS automatic redirection from [this Actix middleware](https://github.com/petertrotman/actix-web-middleware-redirect-https)
- SSL keys location configurable. Check out `serve_opt.json` for a glimpse.
- Introducing [Rustls](https://github.com/ctz/rustls) for importing ssl keys, and given valid ssl keys, will switch to HTTP/2 automatically. Thanks to support from [Actix HTTP/2 module](https://actix.rs/docs/http2/).
- Rename filenames of images automatically using UUIDs (Universally Unique Identifiers) so that filename crash will never happen.

For frontend:
- Designed based on [Bootstrap 4.5](https://getbootstrap.com/docs/4.5/) and [JQuery](https://jquery.com).
- File uploading from [Bootstrap-Fileinput](https://github.com/kartik-v/bootstrap-fileinput).
- Seeking better alternatives if given elegant and working code.

## Essential Explanations

The configuration file may seem complicate at first glance. Below will give an essential explanation about the file and possible settings.

- "follow_symlinks": if symlink files are included as files to be served, and this option is `false`, then warnings will be given to change symlinks into actual files.
- "enable_logging": Enable [Actix logging middleware](https://actix.rs/actix-web/actix_web/middleware/struct.Logger.html) to record history of the server.
- "routes": Basic routing for certain uri. Check [Actix Routing Rules](https://actix.rs/docs/url-dispatch/) for more options.
- "static_directory": Directory containing static files, including html files, icons, css/js files.
- "directory_listing": If css/js files are included indeed, you may want to serve them to the users instead of using cdn. Given the circumstance, it is reasonable to allow users to list all these files (which are contained in seperated folders). **Serving js/css files hasn't been made available as an option yet, if any configuration or modification should be made, please edit `src/main.rs` directly.**
- "server": Configuration for the server.
    + "host": set `0.0.0.0` for accessing the files from another device by the server's ip address; set `127.0.0.1` or `localhost` for self-testing. If you use [nginx](https://nginx.com/) or other reserse proxy services, the address for self-testing should be your choice.
    + "httpport": port for accessing by http. Note that if https are enabled, then http requests will be automatically redirected to https.
    + "httpsport": enable https by setting value to a valid port; disable by setting value to -1.
- "ssl": Generating SSL keys will be discussed later thoroughly.
    + "cert": Location to cert file, or named public key.
    + "key": Location to key file, or named private key.
- "uploaded_files": Configuration for the url dispatching of the uploaded image files.
    + "raw_directory": Where the uploaded files are saved, according to the upload api.
    + "directed_to": The preceding path of the URI; the image files are served as `domain_name/{directed_to}/filename`.
- "template_directory": Output directory for [Handlebars rendering engine](https://github.com/sunng87/handlebars-rust); also where the served static files, including html, css/js, icon files, are really stored. The `static_directory` contains the raw files, while files in this directory are rendered and served for public access.
- "template_variables": variables for [Handlebars engine](https://github.com/sunng87/handlebars-rust). Compare files in `static_directory` and `template_directory` for a glance of its effect.

## General Setup

Install Git following [Guide from Git-Scm](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) or [Guide from Github](https://github.com/git-guides/install-git), then install Rust compiler and cargo by installing [rustup](https://rustup.rs/).

Check the configuration in `./serve_opt.json`, then type the following commands to run the server.

    $ git clone https://github.com/normanade/superserve.git
    $ cd superserve/
    $ cargo run

Note that when playing with the config file, you should know the meaning behind the setting; read the doc above or save a backup file is the suggestion.

## SSL Keys for HTTPS and HTTP/2

HTTPS is a must for migrating to HTTP/2, while SSL keys, or certificates, are essential for HTTPS.

### Generating Self-Signed Certificates

Self-signed certs are handy when you just want to test locally and check whether functions are running or not. Here comes several alternatives (never a exhaustive list) for generating self-signed certs.

#### OpenSSL

SSL keys can be simply generated by [OpenSSL req Command](https://www.openssl.org/docs/manmaster/man1/req.html). A common usage is
    
    openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365

OpenSSL seems bundled in certain operating systems, but installing it should be easy if not in your system. After installation, Check the command usage for more information.

#### mkcert

If you want a modern UI and nice usage guide, check [mkcert](https://github.com/FiloSottile/mkcert), which is written in Golang.

#### rcgen

If you prefer a pure Rust approach, [rcgen](https://github.com/est31/rcgen) would suit your requirements.

### certbot from Let's Encrypt

Self-signed certificates mostly won't introduce a lock sign in your browser, since browsers won't trust these certificates. That will be quite a pain for perfectionists.

Personally I chose [certbot from ETF](https://github.com/certbot/certbot), not endorsing or advertisement, but just a choice. In certain way, Let's Encrypt cooperated with browser brands and made the lock sign available for site visitors. Apart from certbot, maybe there are also some service providers who does the similar things as certbot but I failed to find out.

For certbot users, after installing the new certs and setting the configuaration, you may find the file unreadable by the HTTP server process. Solutions are: run the process in root mode, or change the permissions of the certs so that they are readable.

## Contribution

If you feel satisfied or helped with the project, contribute by leaving a star and sharing it on social media; if you think further improvements can be made, pull requests or suggestions are always welcome; if you by any chance feel doubted or dissatisfied, raise an issue with detailed infomation.
