use actix_web::{web, HttpResponse, Responder}; //Framework
use anyhow::Context; //Manejo de errores
use mysql::prelude::*; //Conexión a la BD
use mysql::{params, Pool};
use serde::{Deserialize, Serialize}; //Convertir objetos a Json

use crate::conexion::get_conexion;

#[derive(Serialize, Deserialize)]
pub struct Alumno {
    n_control: String,
    nombre: String,
    carrera: String,
    semestre: i8,
}

pub async fn mostrar_todos(pool: web::Data<Pool>) -> impl Responder {
    let mut conexion = get_conexion(&pool);

    //select
    let alumnos: Vec<(String, String, String, i8)> = conexion
        .query("SELECT n_control, nombre, carrera, semestre FROM alumnos")
        .context("Error al consultar tabla alumnos")
        .unwrap();

    // Mapea el resultado a la estructura Alumno en formato json
    let alumnos_json: Vec<Alumno> = alumnos
        .into_iter()
        .map(|(n_control, nombre, carrera, semestre)| Alumno {
            n_control,
            nombre,
            carrera,
            semestre,
        })
        .collect();

    HttpResponse::Ok().json(alumnos_json)
}

pub async fn mostrar_uno(pool: web::Data<Pool>, path: web::Path<String>) -> impl Responder {
    let mut conexion = get_conexion(&pool);
    let n_control = path.into_inner();

    let alumnos: Vec<(String, String, String, i8)> = conexion
        .exec(
            "SELECT n_control, nombre, carrera, semestre FROM alumnos
                    WHERE n_control = :n_control",
            params! {
                "n_control" => n_control,
            },
        )
        .context("Error al consultar tabla alumnos")
        .unwrap();

    let alumnos_json: Vec<Alumno> = alumnos
        .into_iter()
        .map(|(n_control, nombre, carrera, semestre)| Alumno {
            n_control,
            nombre,
            carrera,
            semestre,
        })
        .collect();

    HttpResponse::Ok().json(alumnos_json)
}

pub async fn insertar(pool: web::Data<Pool>, info: web::Json<Alumno>) -> impl Responder {
    let mut conexion = get_conexion(&pool);

    //Insertar a alumnos
    conexion
        .exec_drop(
            "INSERT INTO alumnos (n_control, nombre, carrera, semestre) 
                   VALUES (:n_control, :nombre, :carrera, :semestre)",
            params! {
                "n_control" => &info.n_control,
                "nombre" => &info.nombre,
                "carrera" => &info.carrera,
                "semestre" => &info.semestre,
            },
        )
        .context("Error al insertar a alumnos")
        .unwrap();

    HttpResponse::Ok().body("Operación exitosa")
}

pub async fn actualizar(pool: web::Data<Pool>, info: web::Json<Alumno>) -> impl Responder {
    let mut conexion = get_conexion(&pool);

    conexion
        .exec_drop(
            "UPDATE alumnos SET nombre = :nombre, carrera = :carrera, semestre = :semestre
                    WHERE n_control = :n_control",
            params! {
                "n_control" => &info.n_control,
                "nombre" => &info.nombre,
                "carrera" => &info.carrera,
                "semestre" => &info.semestre,
            },
        )
        .context("Error al modificar de alumnos")
        .unwrap();

    HttpResponse::Ok().body("Operación exitosa")
}

pub async fn eliminar(pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let mut conexion = get_conexion(&pool);
    let n_control = path.into_inner();

    conexion
        .exec_drop(
            "DELETE FROM alumnos WHERE n_control = :n_control",
            params! {"n_control" => n_control},
        )
        .context("Error al eliminar alumno")
        .unwrap();

    HttpResponse::Ok().body("Operación exitosa")
}
