use actix_web::{web, HttpResponse, Responder}; //Framework
use anyhow::Context; //Manejo de errores
use mysql::prelude::*; //Conexión a la BD
use mysql::{params, Pool};
use serde::{Deserialize, Serialize}; //Convertir objetos a Json

use crate::conexion::get_conexion;

#[derive(Serialize, Deserialize)]
pub struct Proyecto {
    id: Option<i32>,
    n_control: Option<String>,
    id_asesor: i32,
    nombre_proyecto: String,
    empresa: String,
    periodo: String,
}

//Toda la info. que muestra el select
#[derive(Serialize, Deserialize)]
pub struct ProyectoDetalles {
    id: Option<i32>,
    n_control: String,
    nombre_alumno: String,
    carrera: String,
    semestre: i8,
    id_asesor: i32,
    nombre_asesor: String,
    nombre_proyecto: String,
    empresa: String,
    periodo: String,
}

pub async fn mostrar_todos(pool: web::Data<Pool>) -> impl Responder {
    let mut conexion = get_conexion(&pool);

    //select
    let proyectos: Vec<(i32, String, String, String, i8, i32, String, String, String, String)> = conexion
        .query("SELECT Pr.id, Pr.n_control, Al.nombre 'nombre_alumno', Al.carrera, Al.semestre, Ase.id 'id_asesor', Ase.nombre 'asesor', Pr.nombre 'nombre_proyecto', Pr.empresa, Pr.periodo 
                FROM proyectos Pr
                JOIN alumnos Al ON Al.n_control = Pr.n_control
                JOIN asesores Ase ON Ase.id = Pr.id_asesor")
        .context("Error al consultar tabla proyectos")
        .unwrap();

    // Mapea el resultado a la estructura Proyecto en formato json
    let proyectos_json: Vec<ProyectoDetalles> = proyectos
        .into_iter()
        .map(
            |(
                id,
                n_control,
                nombre_alumno,
                carrera,
                semestre,
                id_asesor,
                nombre_asesor,
                nombre_proyecto,
                empresa,
                periodo,
            )| ProyectoDetalles {
                id: Some(id),
                n_control,
                nombre_alumno,
                carrera,
                semestre,
                id_asesor,
                nombre_asesor,
                nombre_proyecto,
                empresa,
                periodo,
            },
        )
        .collect();

    HttpResponse::Ok().json(proyectos_json)
}

pub async fn mostrar_uno(pool: web::Data<Pool>, path: web::Path<String>) -> impl Responder {
    let mut conexion = get_conexion(&pool);
    let n_control = path.into_inner();

    //select
    let proyectos: Vec<(i32, String, String, String, i8, i32, String, String, String, String)> = conexion
        .exec("SELECT Pr.id, Pr.n_control, Al.nombre 'nombre_alumno', Al.carrera, Al.semestre, Ase.id 'id_asesor', Ase.nombre 'asesor', Pr.nombre 'nombre_proyecto', Pr.empresa, Pr.periodo 
                FROM proyectos Pr 
                JOIN alumnos Al ON Al.n_control = Pr.n_control
                JOIN asesores Ase ON Ase.id = Pr.id_asesor
                WHERE Pr.n_control = :n_control",
                params! {
                    "n_control" => n_control,
                }
        ).context("Error al consultar tabla proyectos")
        .unwrap();

    // Mapea el resultado a la estructura Proyecto en formato json
    let proyectos_json: Vec<ProyectoDetalles> = proyectos
        .into_iter()
        .map(
            |(
                id,
                n_control,
                nombre_alumno,
                carrera,
                semestre,
                id_asesor,
                nombre_asesor,
                nombre_proyecto,
                empresa,
                periodo,
            )| ProyectoDetalles {
                id: Some(id),
                n_control,
                nombre_alumno,
                carrera,
                semestre,
                id_asesor,
                nombre_asesor,
                nombre_proyecto,
                empresa,
                periodo,
            },
        )
        .collect();

    HttpResponse::Ok().json(proyectos_json)
}

//web::Json<Proyecto> = la petición tiene cuerpo y corresponde a la estructura Proyecto
pub async fn insertar(pool: web::Data<Pool>, info: web::Json<Proyecto>) -> impl Responder {
    let mut conexion = get_conexion(&pool);

    //exec_drop permite parámetros
    //Insertar a proyectos
    conexion
        .exec_drop(
            "INSERT INTO proyectos (nombre, n_control, id_asesor, empresa, periodo) 
                    VALUES (:nombre, :n_control, :id_asesor, :empresa, :periodo)",
            params! {
                "n_control" => &info.n_control,
                "id_asesor" => &info.id_asesor,
                "nombre" => &info.nombre_proyecto,
                "empresa" => &info.empresa,
                "periodo" => &info.periodo,
            },
        )
        .context("Error al insertar a proyectos")
        .unwrap();

    HttpResponse::Ok().body("Operación exitosa")
}

pub async fn actualizar(pool: web::Data<Pool>, info: web::Json<Proyecto>) -> impl Responder {
    let mut conexion = get_conexion(&pool);

    //exec_drop permite parámetros
    conexion
        .exec_drop(
            "UPDATE proyectos 
                SET nombre = :nombre, empresa = :empresa, periodo = :periodo, id_asesor = :id_asesor
                WHERE id = :id",
            params! {
                "id" => &info.id,
                "nombre" => &info.nombre_proyecto,
                "empresa" => &info.empresa,
                "id_asesor" => &info.id_asesor,
                "periodo" => &info.periodo,
            },
        )
        .context("Error al modificar de proyectos")
        .unwrap();

    HttpResponse::Ok().body("Operación exitosa")
}

pub async fn eliminar(pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let mut conexion = get_conexion(&pool);
    let id = path.into_inner();

    conexion
        .exec_drop(
            "DELETE FROM proyectos WHERE id = :id",
            params! {"id" => id },
        )
        .context("Error al eliminar proyecto")
        .unwrap();

    HttpResponse::Ok().body("Operación exitosa")
}
