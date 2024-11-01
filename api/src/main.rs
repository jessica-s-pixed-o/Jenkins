use actix_web::{web, App, HttpServer, Responder}; //Framework
use anyhow::{Context, Result}; //Manejo de errores

mod conexion;
mod proyectos;
mod alumnos;
mod asesores;

async fn test() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    let pool = conexion::conectar_bd();

    conexion::crear_estructura(&pool)?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) //Compartir pool de la BD con cada ruta
            .service(
                //Rutas de proyecto
                web::scope("/proyectos")
                    .route("/", web::get().to(test))
                    .route("/mostrar", web::get().to(proyectos::mostrar_todos))
                    .route("/mostrar/{n_control}", web::get().to(proyectos::mostrar_uno))
                    .route("/insertar", web::post().to(proyectos::insertar))
                    .route("/actualizar", web::patch().to(proyectos::actualizar))
                    .route("/eliminar/{id}", web::delete().to(proyectos::eliminar)),
            )
            .service(
                //Rutas de alumnos
                web::scope("/alumnos")
                    .route("/", web::get().to(test))
                    .route("/mostrar", web::get().to(alumnos::mostrar_todos))
                    .route("/mostrar/{n_control}", web::get().to(alumnos::mostrar_uno))
                    .route("/insertar", web::post().to(alumnos::insertar))
                    .route("/actualizar", web::patch().to(alumnos::actualizar))
                    .route("/eliminar/{id}", web::delete().to(alumnos::eliminar)),
            )
            .service(
                //Rutas de asesores
                web::scope("/asesores")
                    .route("/", web::get().to(test))
                    .route("/mostrar", web::get().to(asesores::mostrar_todos))
                    .route("/mostrar/{id}", web::get().to(asesores::mostrar_uno))
                    .route("/insertar", web::post().to(asesores::insertar))
                    .route("/actualizar", web::patch().to(asesores::actualizar))
                    .route("/eliminar/{id}", web::delete().to(asesores::eliminar)),
            )
    })
    .bind("0.0.0.0:8000")
    .context("Fallo con la dirección del servidor")?
    .run()
    .await
    .context("Fallo al ejecutar el servidor")?;

    Ok(())
}
