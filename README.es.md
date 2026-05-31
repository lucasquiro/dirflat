# dirflat

Una aplicación rápida, confiable y de propósito único con interfaz de línea de comandos (CLI) y gráfica (GTK4), escrita en Rust y diseñada para aplanar árboles de directorios profundos y caóticos en grupos nítidamente categorizados.

[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/) [English](README.md) | [Español](README.es.md)
<div id="spanish">

## Descripción General

`dirflat` toma un directorio con un árbol de carpetas masivo y anidado, extrae todos los archivos y los reorganiza en categorías estructuradas y distintas en un destino especificado. Sigue la filosofía Unix de hacer una sola cosa y hacerla bien.

### Categorías Clave
- **Imágenes:** Fotos, gráficos y archivos vectoriales.
- **Videos:** Grabaciones de clips, películas y animaciones.
- **Documentos:** PDFs, archivos de texto, hojas de cálculo y presentaciones.
- **Audio/Música:** Pistas de sonido, notas de voz y archivos de audio.
- **Archivos Comprimidos:** Archivos empaquetados y comprimidos.
- **Otros:** Cualquier archivo que no encaje en las categorías principales.

---

## Hoja de Ruta Técnica y Plan de Ejecución

Este es el flujo planificado para el pipeline de ejecución de `dirflat`. Actualmente, el proyecto se encuentra en desarrollo activo.

### 1. Validación Previa a la Ejecución
- [x] **Verificación de Rutas:** Comprobar y validar que tanto la ruta de origen como la de destino existan.
- [x] **Verificación de la Misma Partición:** Asegurar que el origen y el destino residan en la misma unidad de disco físico para permitir operaciones de archivos instantáneas o eficientes.
- [ ] **Verificación de Capacidad de Almacenamiento:** Calcular el tamaño total del origen y verificar si la unidad de destino tiene suficientes bytes libres disponibles antes de iniciar los procesos de copia o movimiento.

### 2. Escaneo y Mapeo
- [ ] **Recorrido Recursivo del Árbol:** Enumerar de forma recursiva cada archivo dentro de las estructuras de directorios profundamente anidadas.
- [ ] **Clasificación de Contenido:** Ejecutar un algoritmo de clasificación interno para separar los archivos en mapas estructurados (Archivos Comprimidos, Imágenes, Videos, Documentos, Audio).
- [ ] **Generación de Estadísticas:** Construir un resumen de ejecución que muestre el recuento y los tamaños totales por categoría.

### 3. Resolución de Colisiones (Deduplicación)
- [ ] **Verificación de Colisión de Nombres:** Detectar nombres coincidentes en el mapa aplanado.
- [ ] **Verificación de Tamaño:** Si los nombres coinciden, comparar los tamaños de los archivos en bytes.
- [ ] **Hash Criptográfico:** Si los tamaños coinciden, generar hashes criptográficos (por ejemplo, SHA-256) de ambos archivos.
  - *Si los hashes coinciden:* Conservar solo 1 instancia y eliminar de forma segura el duplicado.
  - *Si los hashes difieren:* Retener ambos archivos, preservando la ruta original mediante un atributo de metadatos `rename` para asignar de forma segura un nombre único.

### 4. Fase de Reorganización
- [ ] **Creación de Directorios:** Generar las carpetas raíz de clasificación en el objetivo de destino.
- [ ] **Reubicación de Archivos:** Mover eficientemente los archivos a sus respectivas carpetas utilizando las rutas mapeadas calculadas previamente.
- [ ] **Barrido de Verificación:** Análisis posterior al movimiento para escanear nuevamente el árbol de origen e identificar cualquier archivo que haya quedado atrás o se haya omitido debido a bloqueos del sistema.
- [ ] **Plan de Respaldo General:** Mover los datos restantes no clasificados a la carpeta `others/`.

### 5. Limpieza y Preservación del Estado
- [ ] **Depuración del Árbol de Origen:** Verificar si el tamaño del árbol de directorios original es igual a 0 bytes; si está completamente vacío, purgar el esqueleto de la estructura de directorios.
- [ ] **Instantánea Idempotente y Restauración:** Guardar el historial del mapa interno del árbol en un archivo, permitiendo a los usuarios revertir completamente y restaurar la estructura original de directorios profundamente anidada si es necesario.

---

## Instalación y Compilación

### Prerrequisitos
Asegúrate de tener instalados en tu sistema el toolchain de Rust y las librerías de desarrollo de GTK4.

</div>
```bash
# Dependencias para Ubuntu / Debian / Mint
sudo apt install libgtk-4-dev build-essential pkg-config