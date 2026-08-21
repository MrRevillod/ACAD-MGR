#set document(title: "Plataforma de Visualización y Gestión de Investigación")
#set page(paper: "a4", margin: (x: 2.5cm, y: 2.2cm), numbering: "1")
#set text(font: "P052", size: 10.5pt, lang: "es")
#set par(justify: true, leading: 0.72em)
#set heading(numbering: "1.1.")

#show heading.where(level: 1): set text(size: 13pt, weight: "bold")
#show heading.where(level: 1): set block(above: 2em, below: 1em)

#show heading.where(level: 2): set text(size: 11.5pt, weight: "bold")
#show heading.where(level: 2): set block(above: 2em, below: 1em)

#show heading.where(level: 3): set text(size: 11.5pt, weight: "bold")
#show heading.where(level: 3): set block(above: 2em, below: 1em)

#let report-head(
  title,
  subtitle,
  name: "Luciano Ignacio Revillod Jeréz",
  email: "lrevillod2022@alu.uct.cl",
  show-line: true,
) = {
  align(center)[
    #text(font: "P052", size: 18pt, weight: "bold")[#title]
    #v(-0.1cm)
    #text(font: "P052", size: 11pt, style: "italic", fill: luma(100))[#subtitle]
    #v(8pt)
    #name
    #linebreak()
    #email
    #linebreak()
    Estudiante de Ingeniería Civil en Informática, Universidad Católica de Temuco
    #linebreak()
    #if show-line {
      v(6pt)
      line(length: 100%)
    }
  ]
}

#report-head(
  "Plataforma de Visualización y Gestión de Investigación",
  "Reporte del Proyecto",
  show-line: true,
)

#v(6pt)

#heading(numbering: none)[Resumen]

Este documento presenta la plataforma de gestión de la producción científica de la Facultad de Ingeniería de la Universidad Católica de Temuco. Describe las capacidades implementadas, la proyección de su evolución futura y los aspectos técnicos de su implementación.

#heading(numbering: none)[Introducción]

Al comenzar el proyecto y el análisis de la situación, se identificó que la Facultad de Ingeniería carecía de un *sistema centralizado* para gestionar y analizar la producción científica de sus académicos. El requerimiento general consistió en construir una *plataforma web* que permitiera importar, clasificar y analizar las publicaciones académicas de los investigadores de la facultad, con el objetivo de reemplazar la gestión manual basada en planillas Excel.

El resultado es un sistema funcional que cubre el *ciclo completo* de la gestión de la producción científica: desde la importación de los datos, pasando por su clasificación y análisis, hasta su difusión en una vista pública. La plataforma está operativa y accesible en línea, y distingue claramente una *vista pública* de divulgación y una *vista administrativa* de gestión interna.

= El sistema actual

== Contexto institucional

El sistema está pensado para una sola unidad académica: la *Facultad de Ingeniería* de la UCT. La estructura de la facultad es la siguiente:

- *4 departamentos:* Ciencias Matemáticas y Físicas; Obras Civiles y Geología; Procesos Industriales; Ingeniería Informática.
- *10 carreras* distribuidas entre esos departamentos

El diseño del modelo de datos refleja esta jerarquía institucional de forma explícita. La organización se representa con una cadena *facultad → departamento → carrera*, a la que se suman dos dimensiones propias del ámbito académico: el *cargo laboral* del académico y su *categorización académica* (planta permanente o adjunta, con una opción de docencia o investigación y horas asociadas).

Este dato es relevante para el alcance: la plataforma *no está diseñada (por ahora) para múltiples facultades o para toda la universidad*, aunque su modelo de datos lo permitiría bajo ciertos cambios.

== Capacidades del sistema

La plataforma ofrece un conjunto de soluciones de alto nivel que reemplazan la gestión manual. En su gran mayoría, estas capacidades tienen que ver con la representación y visualización de la producción académica de la facultad. Cada una de ellas se describe a continuación con el detalle de su funcionamiento.

=== Gestión de académicos

La plataforma mantiene un *registro centralizado* de cada académico con su información personal, institucional y académica. Los datos cubren:

- *Identificación:* RUT, nombres y apellidos, correo institucional y *ORCID*.
- *Datos personales:* sexo, fecha de nacimiento, nacionalidad y ciudad.
- *Datos laborales:* cargo, departamento, carrera y *JCE* (Jornada Completa Equivalente, expresada en horas).
- *Categorización académica:* planta (permanente/adjunta), categoría (p. ej. Profesor Titular, Asociado, Asistente), opción (docencia/investigación), horas de la categoría y horas de descuento anual.
- *Grados académicos:* hasta tres niveles (profesional, magíster y doctor), cada uno con nombre, universidad, país y fecha de obtención.

La creación y edición de registros incorpora *validaciones de negocio* que evitan datos inconsistentes: el formato del RUT y del ORCID se comprueban, la carrera debe pertenecer al departamento indicado, y la JCE no puede superar el máximo configurado por la institución. De igual forma se previene el alta duplicada por RUT o por ORCID.

=== Importación de datos académicos desde CSV

Para la carga inicial masiva, la plataforma permite *importar académicos desde un archivo CSV* con un formato definido, evitando el registro uno a uno. El proceso es transaccional y controlado:

- Cada fila del archivo se *valida de forma independiente*: primero el formato de los campos y luego las reglas de negocio (existencia de referencias, consistencia entre categoría, planta, opción y horas).
- Las referencias (departamento, carrera, cargo, categoría) se *resuelven por nombre* contra los catálogos existentes, y se comprueba la coherencia entre la categoría elegida, su planta y las horas declaradas.
- Las filas que superan la validación se *persisten en una transacción*; si alguna falla, se registra el error sin afectar a las filas correctas.
- Al finalizar, se entrega un *reporte de resultados* con el número de académicos importados y el detalle de errores por fila, lo que permite corregir el archivo de forma dirigida.

La importación está pensada tanto para la *carga inicial* como para la *actualización* de datos. Si el archivo contiene un académico que ya existe (mismo RUT o mismo correo), esa fila *se actualiza*: el sistema lo reconoce por RUT o por correo, reemplaza sus datos con los del archivo y refresca sus grados académicos. El resumen de resultados distingue los académicos *creados* de los *actualizados*. Si alguna fila presenta un error (por ejemplo, referencias inexistentes o datos inconsistentes), se omite y el motivo se reporta en la fila correspondiente con un mensaje claro.

=== Sincronización de publicaciones

La plataforma importa automáticamente las publicaciones de cada académico desde fuentes públicas, evitando el registro manual. El flujo combina dos fuentes:

- *ORCID* se consulta para obtener el identificador de cada publicación asociada al perfil del académico.
- *OpenAlex* se consulta para completar la información con metadatos completos y estandarizados (taxonomía, tópicos, palabras clave, ISSN de las revistas, autores y afiliaciones, entre otros).

De cada publicación se captura: título, DOI, fecha y año de publicación, idioma, estado (aceptado/publicado), revista o fuente, autores y afiliaciones. El proceso *distingue autores internos* (académicos de la facultad) de *autores externos* a partir de su ORCID, lo que es la base para los análisis por unidad y para la red de colaboración.

La sincronización puede ejecutarse *por académico* (desde su perfil) o *de forma masiva* para todos los académicos, y al finalizar se entrega un *resumen de resultados* (publicaciones creadas, autores enlazados, tópicos y palabras clave asociados, obras sin DOI o no encontradas, y errores puntuales). Es un proceso idempotente: al repetirlo, actualiza los registros existentes en lugar de duplicarlos.

Conviene precisar algunos casos particulares de la sincronización:

- *Solo se importan publicaciones visibles en ORCID*: la fuente de partida es el perfil ORCID del académico, por lo que una obra que aún no aparece allí (o que no está visible públicamente) no se importa hasta que esté disponible en la fuente. Puede ser necesario esperar a que la fuente externa refleje la publicación.
- *Se importan únicamente obras con DOI y presentes en OpenAlex*: las obras sin DOI se omiten, y las que no se encuentran en OpenAlex tampoco se incorporan; ambos casos se cuentan en el resumen de resultados.
- *Solo se incorporan artículos*: otros tipos de obra (libros, capítulos, actas, etc.) quedan fuera de la importación.
- *La sincronización puede tardar*: el proceso consulta OpenAlex con un intervalo entre peticiones, y la sincronización masiva recorre a todos los académicos, por lo que puede tomar varios minutos. Al repetir una sincronización, las obras ya registradas se actualizan sin duplicarse y las correcciones manuales se conservan.
- *Persistencia de las ediciones*: cuando se corrige manualmente una publicación, los cambios se guardan de forma separada de los datos originales. En una re-sincronización, los metadatos base se actualizan desde la fuente pero *las correcciones manuales se conservan*.
- *Desvinculación*: si una obra deja de aparecer en ORCID, se desvincula del académico (se registra en el resumen como autorías desvinculadas).

=== Clasificación institucional propia (líneas de investigación)

Dentro de los requerimientos institucionales se presentó la necesidad de clasificar cada publicación según una serie de *líneas de investigación*. Esto corresponde a una clasificación propia de la institución, que se nutre y complementa con la taxonomía estándar de OpenAlex. La plataforma permite:

- Mantener sus propias *líneas de investigación* (Materiales Avanzados y Bioproductos; Ciencias de la Tierra; Sostenibilidad; IA, Sistemas Complejos y Modelamiento Matemático; Educación en Ingeniería).
- Clasificar cada publicación a través de la *taxonomía estándar de OpenAlex*: dominio → campo → subcampo → tópico → palabra clave.
- *Asignar subcampos a líneas de investigación* de forma visual (mediante arrastrar y soltar) desde la administración, de modo que las publicaciones queden agrupadas según el interés institucional.

La asignación de una publicación a su línea es *automática y trazable*: se usa la línea indicada manualmente cuando existe, o se infiere a partir del tópico de mayor relevancia en OpenAlex; si no hay coincidencia, la obra queda en la categoría "Sin asignar". Esta regla se aplica de forma consistente en las estadísticas.

=== Indexación en WoS y Scopus

La plataforma determina si cada publicación proviene de una revista *indexada en Web of Science (WoS)* o en *Scopus*, mediante tablas de ISSN. Esto permite:

- Distinguir entre publicaciones *indexadas* y *no indexadas*.
- Desglosar todas las estadísticas y rankings por tipo de indexación, que es un criterio habitual de evaluación de la producción científica.

Este dato se integra en el resto de las vistas: cada publicación muestra su indexación, y los dashboards permiten filtrar y comparar la producción WoS frente a la de Scopus.

=== Estadísticas y dashboards

La plataforma entrega un conjunto de vistas analíticas con gráficos y KPIs que permiten comprender la producción de la facultad a distintos niveles de detalle:

- *A nivel de facultad:* total de publicaciones, conteo WoS/Scopus, tendencia por año, distribución por departamento y por línea de investigación, y ranking de publicadores.
- *A nivel de departamento:* resumen, top de publicadores y tendencia por tipo de indexación.
- *A nivel de línea de investigación:* resumen, distribución por departamento y top de publicadores.
- *A nivel de académico:* distribución por línea, línea dominante, tendencia anual y *contribución relativa*.

La *contribución relativa* es una de las vistas de mayor valor: muestra cuánto aporta un académico a su facultad, a su departamento y a su línea de investigación, mediante indicadores comparativos. Esto permite valorar el desempeño individual en el contexto de su unidad. Todas las vistas admiten filtros de rango de años, departamento y tipo de indexación.

=== Red de colaboraciones y recomendaciones

La plataforma visualiza la *colaboración científica* entre académicos como un *grafo*: los nodos representan académicos y las aristas las coautorías, con un peso según el número de publicaciones compartidas.

Sobre esta base, el sistema *recomienda potenciales colaboradores* que no pertenecen a la red directa del académico consultado. La recomendación se fundamenta en la *afinidad temática*: coincidencia en tópicos, palabras clave y líneas de investigación, con un umbral de coincidencia configurable. De esta forma, la herramienta no solo describe la colaboración existente, sino que apoya la búsqueda de nuevos vínculos científicos.

=== Vistas públicas y administrativas

- *Vista pública:* directorio de académicos (con filtros de búsqueda, departamento y carrera) y perfil público de cada uno, organizado en pestañas de publicaciones, estadísticas y red de colaboración. No requiere iniciar sesión, por lo que sirve como carta de presentación de la producción científica de la facultad.
- *Vista administrativa:* gestión completa de académicos, categorías, opciones, cargos, líneas de investigación, publicaciones y usuarios.

=== Autenticación y gestión de usuarios

El acceso administrativo está protegido con *inicio de sesión por correo y contraseña*, basado en tokens (JWT) gestionados mediante cookies y *sesiones revocables*. La gestión de usuarios permite crear, editar y eliminar cuentas de administración.

=== Autoservicio de perfil por parte del académico

La plataforma permite que el propio académico mantenga actualizada su información, con controles de seguridad:

- El administrador puede *enviar códigos de edición de perfil de un solo uso* por correo (individual o masivo). Los códigos son de 8 caracteres y la plataforma mantiene un número de códigos vigentes por académico.
- El académico usa el código para *solicitar un enlace de edición* que llega a su correo. El enlace es *temporal* (con vencimiento) y queda vinculado al estado del registro, de modo que solo permite editar la versión vigente de sus datos.
- A través de ese enlace, el académico puede actualizar sus propios datos (nombres, ORCID, sexo, fecha de nacimiento, nacionalidad y ciudad).
- Además, el académico puede *corregir las publicaciones* donde es autor o coautor cuando la información proveniente de la fuente externa (ORCID/OpenAlex) presenta errores: puede editar metadatos (título, resumen, DOI, año, estado), indicar el *autor correspondiente*, ajustar las *afiliaciones* de cada autor y reasignar la *línea de investigación*.

De este modo, la plataforma combina la automatización de la importación con la capacidad de ajuste fino por parte de quienes conocen mejor su propia producción, sin comprometer la seguridad ni la integridad de los datos.

= Proyección a futuro

Este apartado presenta las líneas de evolución previstas para la plataforma.

= Apartado técnico e implementación

Este apartado describe el stack, la arquitectura, la estructura del código y la base de datos del sistema.
