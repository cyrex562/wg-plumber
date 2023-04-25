use actix_web::{HttpServer, HttpResponse, App, web};
use anyhow::bail;
use clap::Args;

use crate::Config;

#[derive(Args, Clone)]
pub struct ServerArgs {}

pub async fn server(config: &Config, args: &ServerArgs) -> anyhow::Result<()> {
    let result = HttpServer::new(||
        App::new().service(
            web::resource("/").to(|| HttpResponse::Ok())))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    if result.is_err(){
        bail!("server start/run failed: {:?}", result.err());
    }
    Ok(())
}
