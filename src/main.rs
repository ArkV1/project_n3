use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
//use actix_web_middleware_redirect_https::RedirectHTTPS;


#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("nopass.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            // HTTPS ONLY
            // .wrap(RedirectHTTPS::with_replacements(&[(
            //     ":8080".to_owned(),
            //     ":8443".to_owned(),
            // )]))
            .service(index)
    })
    .bind("127.0.0.1:8080")? // Testing port
    //.bind("127.0.0.1:80")? // Production
    .bind_openssl("127.0.0.1:8443", builder)? // Testing port - SECURED
    //.bind_openssl("127.0.0.1:443", builder)? // Production port - SECURED
    .run()
    .await
}
