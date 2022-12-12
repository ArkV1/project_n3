// ACTIX
use actix_web::{get, App, HttpRequest, HttpServer, Responder};
// HTTPS
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
// TEMPLATING
use tera::Tera;
// LOCALS
use unic_langid::{LanguageIdentifier, langid};
use fluent_templates::{FluentLoader, static_loader};

const US_ENGLISH: LanguageIdentifier = langid!("en-US");

static_loader! {
    // Declare our `StaticLoader` named `LOCALES`.
    static LOCALES = {
        // The directory of localisations and fluent resources.
        locales: "./locales",
        // The language to falback on if something is not present.
        fallback_language: "en-US",
        // Optional: A fluent resource that is shared with every locale.
        core_locales: "./locales/core.ftl",
    };
}
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

    let mut tera = tera::Tera::default();
    let ctx = tera::Context::default();
    tera.register_function("fluent", FluentLoader::new(&*LOCALES));
    
    HttpServer::new(|| {
        App::new()
            // HTTPS ONLY
            .wrap(
                RedirectSchemeBuilder::new()
                    .temporary() // 307 Temporary Redirect
                    .replacements(&[(":8080", ":8443")]) // !!!TESTING PORTS!!!
                    .build(),
            )
            .service(index)
    })
    .bind("127.0.0.1:8080")? // Testing port :8080 | Production port :80 - UNSECURED
    .bind_openssl("127.0.0.1:8443", builder)? // Testing port :8443 | Production port :443 - SECURED
    .run()
    .await
}
