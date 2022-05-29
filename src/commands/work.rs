use crate::context::Context;
use crate::errors::Error;
use crate::server;

pub fn exec(ctx: &mut Context) -> Result<(), Error> {
    // let server = Server::new(ctx, "hello world");
    // server.run();
    server::run();

    Ok(())
}

// struct Server {
//     pac: &'static str
// }

// impl Server {
//     fn new(ctx: &Context, pac: &'static str) -> Self {
//         Server {
//             pac: pac,
//         }
//     }

//     #[get("/pac/proxy.js")]
//     async fn pac_proxy_js(&self) -> &'static str {
//         self.pac
//     }

//     #[actix_web::main]
//     async fn run(&self) -> std::io::Result<()> {
//         HttpServer::new(|| {
//             App::new().service(self.pac_proxy_js)
//         })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
//     }
// }

// struct Worker {

// }

// impl Worker {
//     fn run() {

//     }
// }