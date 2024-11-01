use actix_web::web; //Framework
use anyhow::{Context, Result}; //Manejo de errores
use mysql::prelude::*; //Conexión a la BD
use mysql::{Pool, PooledConn};

pub fn conectar_bd() -> Pool {
    // mysql://username:password@localhost:3306/db
    // bd es el nombre del servicio de mysql que se define en el docker compose 
    let url = "mysql://root@bd:3306/residencias";

    //Crea pool de conexiones a la bd.
    //context para especificar mensaje en caso de error
    return Pool::new(url)
        .context("Fallo al conectar a la base de datos")
        .unwrap();
}

pub fn get_conexion(pool: &web::Data<Pool>) -> PooledConn {
    //mut indica que la conexión puede cambiar de estado al ejecutar la consulta (a abierta, cerrada, etc)
    //trae una conexión del pool
    return pool
        .get_conn()
        .context("Fallo al traer conexión del pool")
        .unwrap();
}

//Result<()> indica que la función necesita regresar un error o un Ok
pub fn crear_estructura(pool: &Pool) -> Result<()> {
    let mut conexion = get_conexion(&web::Data::new(pool.clone()));

    //query_drop ejecuta la consulta sin devolver un resultado
    //usar para create, drop, insert, update
    //? maneja si hay error y lo propaga
    conexion
        .query_drop(
            "CREATE TABLE IF NOT EXISTS alumnos (
                n_control VARCHAR(10) PRIMARY KEY,
                nombre VARCHAR(100) NOT NULL,
                carrera VARCHAR(100) NOT NULL,
                semestre SMALLINT NOT NULL)",
        )
        .context("Error al crear tabla alumnos")?;

    conexion
        .query_drop(
            "CREATE TABLE IF NOT EXISTS asesores (
                id VARCHAR(10) PRIMARY KEY, 
                nombre VARCHAR(100) NOT NULL,
                cubiculo VARCHAR(5) NOT NULL)",
        )
        .context("Error al crear tabla asesores")?;

    conexion
        .query_drop(
            "CREATE TABLE IF NOT EXISTS proyectos (
                id INT AUTO_INCREMENT PRIMARY KEY, 
                nombre VARCHAR(255),
                n_control VARCHAR(10) NOT NULL REFERENCES alumnos(n_control),
                id_asesor VARCHAR(10) REFERENCES asesores(id),
                empresa VARCHAR(255) NOT NULL,
                periodo VARCHAR(12) NOT NULL)",
        )
        .context("Error al crear tabla proyectos")?;

    Ok(())
}
