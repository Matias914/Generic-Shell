# 🦀 Rust Shell (msh)

![Rust](https://img.shields.io/badge/Made_with-Rust-orange?style=for-the-badge&logo=rust)
![Status](https://img.shields.io/badge/Status-Work_in_Progress-yellow?style=for-the-badge)

Una implementación ligera y funcional de una shell estilo Unix escrita completamente en **Rust**. Este proyecto explora los fundamentos de los sistemas operativos, el manejo de procesos y la manipulación de descriptores de archivos.

> **Nota:** Este proyecto es educativo y está en desarrollo activo.

## ✨ Funcionalidades

Actualmente, la shell soporta las siguientes características:

- **Ejecución de Comandos:** Ejecuta cualquier programa binario presente en el `PATH` (ej: `ls`, `grep`, `vim`).
- **Built-in Commands:** Comandos internos implementados directamente en la shell:
  - `cd`: Cambiar de directorio (soporta rutas absolutas y relativas).
  - `pwd`: Mostrar directorio actual.
  - `echo`: Imprimir texto en pantalla.
  - `type`: Identificar si un comando es un binario, un built-in o un alias.
  - `exit`: Cerrar la shell ordenadamente.
- **Gestión de Historial (`history`):**
  - Almacenamiento persistente del historial.
  - Soporte para flags como `-r` (leer archivo) y `-a` (append/añadir).
- **Redirecciones I/O:**
  - `>` : Redirección de salida estándar (sobrescribir).
  - `>>`: Redirección de salida estándar (adjuntar/append).
  - `2>`: Redirección de errores (stderr).
- **Manejo de Errores:** Mensajes claros cuando un comando no existe o falla.

## 🚀 Cómo ejecutarla

### Prerrequisitos
Necesitas tener **Rust** y **Cargo** instalados en tu sistema.

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh

```

### Pasos

1. **Clonar el repositorio:**
```bash
git clone [https://github.com/TU_USUARIO/tu-repo-shell.git](https://github.com/TU_USUARIO/tu-repo-shell.git)
cd tu-repo-shell

```


2. **Compilar y Ejecutar:**
   Para una ejecución rápida en modo desarrollo:
```bash
cargo run

```


Para compilar una versión optimizada (release):
```bash
cargo build --release
./target/release/tu_shell_name

```



## 💻 Ejemplos de Uso

Una vez dentro de la shell, puedes probar comandos como:

```bash
# Navegación básica
$ pwd
/home/usuario
$ cd /tmp

# Ejecución de programas y redirección
$ls -la > lista_archivos.txt$ cat lista_archivos.txt

# Manejo del historial
$ history
1 ls -la > lista_archivos.txt
2 cat lista_archivos.txt
$ history -w  # Guarda la sesión actual al disco

```

## 🛠️ Estructura del Proyecto

* `src/main.rs`: Punto de entrada y bucle principal (REPL).
* `src/commands.rs`: Implementación de los comandos built-in.
* `src/history.rs`: Lógica de persistencia y manejo del archivo de historial.
* `src/parser.rs`: Tokenización y parsing de los inputs del usuario.

## 🔮 Próximos Pasos (Roadmap)

* [ ] Implementar Pipes (`|`) para encadenar comandos.
* [ ] Soporte para `Raw Mode` (autocompletado con tabulador y flechas de dirección).
* [ ] Manejo de señales (Ctrl+C, Ctrl+Z).
* [ ] Variables de entorno (`export`).

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Si tienes una idea para mejorar el manejo de memoria o añadir una nueva feature, siéntete libre de abrir un Pull Request.

## 📄 Licencia

Este proyecto está bajo la Licencia MIT - mira el archivo [LICENSE](https://www.google.com/search?q=LICENSE) para más detalles.

```

---

### Consejos para que quede perfecto:

1.  **Reemplaza `TU_USUARIO` y `tu-repo-shell`** con los datos reales de tu GitHub.
2.  **Añade una captura de pantalla (Opcional pero recomendado):**
    * Saca una captura de tu terminal ejecutando `ls`, un `echo` y luego un `history`.
    * Guárdala en tu repo (ej: carpeta `screenshots/demo.png`).
    * Agrégala al README debajo del título así: `![Demo de la Shell](screenshots/demo.png)`.
3.  **Licencia:** Si no tienes un archivo `LICENSE`, crea uno (MIT es el estándar open source más común) para que la gente sepa que puede usar tu código para aprender.

```