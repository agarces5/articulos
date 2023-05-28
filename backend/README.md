### TODO
- Terminar DTOs para casos de uso.
- Terminar casos de uso.
- Revisar test para casos de uso.
- Refactoring casos de uso.

### ESQUEMA CONCEPTUAL
- Cargo.lock y Cargo.toml: archivos de configuración de Cargo, el administrador de paquetes y compilador de Rust.
- README.md: archivo de documentación del proyecto.
- src: directorio que contiene el código fuente del proyecto.
    - adapters: directorio que contiene los adaptadores de la aplicación para interactuar con el mundo exterior.
        - db.rs: adaptador de base de datos que implementa el repositorio de artículos.
        - http.rs: adaptador HTTP que implementa el puerto de entrada y salida de la aplicación.
        - redis.rs: adaptador de Redis que implementa el repositorio de sesiones.
    - application: directorio que contiene la capa de aplicación, donde se definen los casos de uso y se implementan los servicios de aplicación.
        - ports: directorio que contiene los puertos de entrada y salida de la aplicación.
            - inbound.rs: puerto de entrada que expone los endpoints HTTP de la aplicación.
            - outbound.rs: puerto de salida que define la interfaz del repositorio de artículos.
        - services.rs: servicios de aplicación que implementan la lógica de negocio de la aplicación.
        - usecases.rs: casos de uso que definen las acciones que pueden realizar los usuarios de la aplicación.
    - domain: directorio que contiene la capa de dominio, donde se definen las entidades y objetos de valor de la aplicación.
        - entities.rs: entidades de la aplicación, como la entidad Article.
        - errors.rs: errores que pueden ocurrir en la aplicación.
        - repositories.rs: interfaz del repositorio de artículos.
        - value_objects.rs: objetos de valor de la aplicación, como el objeto de valor ArticleTitle.
    - main.rs: archivo de entrada de la aplicación que inicializa los adaptadores y puertos de la aplicación.
- tests: directorio que contiene los archivos de prueba del proyecto, separados por capa.
