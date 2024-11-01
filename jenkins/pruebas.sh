sleep 120 #Esperar a que la api esté totalmente activa

FALLAS=0

#Alumnos
  #Insertar alumnos
  ALUMNO='{
      "n_control": "20240250",
      "nombre": "Cristian",
      "carrera": "ISC",
      "semestre": 9
  }'
  
  if curl -X POST -H "Content-Type: application/json" -d "$ALUMNO" http://api:8000/alumnos/insertar \
  | grep "Operación exitosa"; then
      echo "Inserción de alumnos EXITOSA." 
  else
      echo "Inserción de alumnos FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi
  
  #Mostrar un alumno
  if curl http://api:8000/alumnos/mostrar/20240250 | grep "$ALUMNO"; then
      echo "Muestra de un alumno EXITOSA."  
  else
      echo "Muestra de un alumno FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi
  
  #Modificar alumno
  ALUMNO='{
    "n_control": "20240250",
    "nombre": "Cristian",
    "carrera": "Mecatrónica",
    "semestre": 10
	}'
    
  if curl -X PATCH -H "Content-Type: application/json" -d "$ALUMNO" http://api:8000/alumnos/actualizar \
  | grep "Operación exitosa"; then
      echo "Actualización de un alumno EXITOSA."  
  else
      echo "Actualización de un alumno FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi

  
  #Eliminar un alumno
    if curl -X DELETE http://api:8000/alumnos/eliminar/20240250 \
  | grep "Operación exitosa"; then
      echo "Eliminación de alumnos EXITOSA." 
  else
      echo "Eliminación de alumnos FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi
  
#Asesores
  #Insertar asesores
  ASESOR='{
    "id": 1,
    "nombre": "Levy",
    "cubiculo": "D10"
}'
  
  if curl -X POST -H "Content-Type: application/json" -d "$ASESOR" http://api:8000/asesores/insertar \
  | grep "Operación exitosa"; then
      echo "Inserción de asesores EXITOSA." 
  else
      echo "Inserción de asesores FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi
  
  #Mostrar un asesor
  if curl http://api:8000/asesores/mostrar/1 | grep "$ASESOR"; then
      echo "Muestra de un asesor EXITOSA."  
  else
      echo "Muestra de un asesor FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi
  
  #Modificar asesor
  ASESOR='{
    "id": 1,
    "nombre": "Levy Rojas",
    "cubiculo": "D20"
}'
    
  if curl -X PATCH -H "Content-Type: application/json" -d "$ASESOR" http://api:8000/asesores/actualizar \
  | grep "Operación exitosa"; then
      echo "Actualización de un asesor EXITOSA."  
  else
      echo "Actualización de un asesor FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi

  
  #Eliminar un asesor
    if curl -X DELETE http://api:8000/asesores/eliminar/1 \
  | grep "Operación exitosa"; then
      echo "Eliminación de asesores EXITOSA." 
  else
      echo "Eliminación de asesores FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi
  
#Proyectos
  #Insertar proyectos
  PROYECTO='{
    "n_control": "20240250",
    "id_asesor": 1,
    "nombre_proyecto": "ERP",
    "empresa": "Castores",
    "periodo": "AGO-DIC 2024"
}'
  curl -s -X POST -H "Content-Type: application/json" -d "$ALUMNO" http://api:8000/alumnos/insertar
  curl -s -X POST -H "Content-Type: application/json" -d "$ASESOR" http://api:8000/asesores/insertar
  if curl -X POST -H "Content-Type: application/json" -d "$PROYECTO" http://api:8000/proyectos/insertar \
  | grep "Operación exitosa"; then
      echo "Inserción de proyecto EXITOSA." 
  else
      echo "Inserción de proyecto FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi
  
  #Mostrar un proyecto
  PROYECTO='{
        "id": 1,
        "n_control": "20240250",
        "nombre_alumno": "Cristian",
        "carrera": "Mecatrónica",
        "semestre": 10,
        "id_asesor": 1,
        "nombre_asesor": "Levy Rojas",
        "nombre_proyecto": "ERP",
        "empresa": "Castores",
        "periodo": "AGO-DIC 2024"
}'
  
  if curl http://api:8000/proyectos/mostrar/20240250 | grep "$PROYECTO"; then
      echo "Muestra de un proyecto EXITOSA."  
  else
      echo "Muestra de un proyecto FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi
  
  #Modificar proyecto
  PROYECTO='{
    "id": 1,
    "id_asesor": 1,
    "nombre_proyecto": "App",
    "empresa": "Serviacero",
    "periodo": "ENE-JUN 2025"
}'

  if curl -s -X PATCH -H "Content-Type: application/json" -d "$PROYECTO" http://api:8000/proyectos/actualizar \
  | grep "Operación exitosa"; then
      echo "Actualización de un proyecto EXITOSA."  
  else
      echo "Actualización de un proyecto FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi

  
  #Eliminar un proyecto
    if curl -s -X DELETE http://api:8000/proyectos/eliminar/1 \
  | grep "Operación exitosa"; then
      echo "Eliminación de proyecto EXITOSA." 
  else
      echo "Eliminación de proyecto FALLIDA." 
      FALLAS=$((FALLAS + 1))
  fi

if [ "$FALLAS" -eq 0 ]; then
    echo "Todas las pruebas fueron exitosas"
    exit 0
else
	echo "Pruebas fallidas: $FALLAS"
    exit 1
fi
