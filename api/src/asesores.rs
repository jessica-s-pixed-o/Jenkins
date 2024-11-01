use actix_web::{web, HttpResponse, Responder}; //Framework
use anyhow::Context; //Manejo de errores
use mysql::prelude::*; //Conexión a la BD
use mysql::{params, Pool};
use serde::{Deserialize, Serialize}; //Convertir objetos a Json

use crate::conexion::get_conexion;

#[derive(Serialize, Deserialize)]
pub struct Asesor {
    id: i32,
    nombre: String,
    cubiculo: String,
}

pub async fn mostrar_todos(pool: web::Data<Pool>) -> impl Responder {
    let mut conexion = get_conexion(&pool);

    //select
    let asesores: Vec<(i32, String, String)> = conexion
        .query("SELECT id, nombre, cubiculo FROM asesores")
        .context("Error al consultar tabla asesores")
        .unwrap();

    // Mapea el resultado a la estructura Asesor en formato json
    let asesores_json: Vec<Asesor> = asesores
        .into_iter()
        .map(|(id, nombre, cubiculo)| Asesor {
            id,
            nombre,
            cubiculo,
        })
        .collect();

    HttpResponse::Ok().json(asesores_json)
}

pub async fn mostrar_uno(pool: web::Data<Pool>, path: web::Path<String>) -> impl Responder {
    let mut conexion = get_conexion(&pool);
    let id = path.into_inner();

    let asesores: Vec<(i32, String, String)> = conexion
        .exec(
            "SELECT id, nombre, cubiculo FROM asesores
                    WHERE id = :id",
            params! {
                "id" => id,
            },
        )
        .context("Error al consultar tabla alumnos")
        .unwrap();

    let asesores_json: Vec<Asesor> = asesores
        .into_iter()
        .map(|(id, nombre, cubiculo)| Asesor {
            id,
            nombre,
            cubiculo,
        })
        .collect();

    HttpResponse::Ok().json(asesores_json)
}

pub async fn insertar(pool: web::Data<Pool>, info: web::Json<Asesor>) -> impl Responder {
    let mut conexion = get_conexion(&pool);

    conexion
        .exec_drop(
            "INSERT INTO asesores (id, nombre, cubiculo) VALUES (:id, :nombre, :cubiculo)",
            params! {
                "id" => &info.id,
                "nombre" => &info.nombre,
                "cubiculo" => &info.cubiculo,
            },
        )
        .context("Error al insertar a asesores")
        .unwrap();

    HttpResponse::Ok().body("Operación exitosa")
}

pub async fn actualizar(pool: web::Data<Pool>, info: web::Json<Asesor>) -> impl Responder {
    let mut conexion = get_conexion(&pool);

    conexion
        .exec_drop(
            "UPDATE asesores SET nombre = :nombre, cubiculo = :cubiculo
                  WHERE id = :id",
            params! {
                "id" => &info.id,
                "nombre" => &info.nombre,
                "cubiculo" => &info.cubiculo,
            },
        )
        .context("Error al modificar de asesores")
        .unwrap();

    HttpResponse::Ok().body("Operación exitosa")
}

pub async fn eliminar(pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let mut conexion = get_conexion(&pool);
    let id = path.into_inner();

    conexion
        .exec_drop(
            "DELETE FROM asesores WHERE id = :id",
            params! {"id" => id},
        )
        .context("Error al eliminar asesor")
        .unwrap();

    HttpResponse::Ok().body("Operación exitosa")
}
