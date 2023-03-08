# Articulos

## Despliegue

Necesitaremos tener instalado ***docker*** y ***docker-compose*** ya que montamos una base de datos Microsoft SQL Server con ello.

### Opcional

Tambien usamos cargo-make para correr los scripts del *Makefile.toml*.

### Inicializacion

Solo tendriamos que correr el comando
```
makers db-reset
```
para inicializar nuestra base de datos con algunos ejemplos.

Las variables de conexion se cogen del .env, en donde tenemos una cadena de conexion muy sencilla para hacer pruebas en local.

## Modelo de dominio

Los camareros nos piden que añadamos, quitemos o modifiquemos distintos artículos, que están asociados
a unas familias que no van a cambiar (los artículos pueden dejar de asociarse con una familia para asociarse
con otra, pero las familias son inmutables) y también están asociados a un precio, que sí que nos pedirán modificar.
Además, debemos ser capaces de mostrar los artículos solo en algunos TPV.

Las limitaciones y los requisitos que nos piden son:
- Las familias ya están definidas y tienen un código de 4 números.
- Los artículos deben comenzar con el número de la familia (ej. 031300000001).
- Cuando se crea un artículo asociado a una familia, aparece en todos los TPV y si queremos limitarlo, tendremos que quitarlos de los TPV.
