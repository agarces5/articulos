# Articulos

## Requisitos

Necesitaremos tener instalado ***docker*** y ***docker-compose*** ya que montamos una base de datos Microsoft SQL Server con ello.

### Opcional

Tambien usamos cargo-make para correr los scripts del *Makefile.toml*.

## Inicializacion

Solo tendriamos que correr el comando
```
makers db-reset
```
para inicializar nuestra base de datos con algunos ejemplos.

Las variables de conexion se cogen del .env, en donde tenemos una cadena de conexion muy sencilla para hacer pruebas en local.

