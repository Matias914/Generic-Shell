# 🦀 Rust Shell (msh)

![Rust](https://img.shields.io/badge/Made_with-Rust-orange?style=for-the-badge&logo=rust)
![Status](https://img.shields.io/badge/Status-Educational-blue?style=for-the-badge)

Una implementación ligera y funcional de una shell estilo Unix escrita completamente en **Rust**. Este proyecto explora los fundamentos de los sistemas operativos, el manejo de procesos y la manipulación de descriptores de archivos a bajo nivel.

> **Nota:** Este es un proyecto educativo personal e independiente.

## ✨ Funcionalidades

Actualmente, la shell soporta las siguientes características:

- **Ejecución de Comandos:** Ejecuta programas binarios usando la variable de entorno `PATH` para su localización.
- **Pipes y Redirecciones:**
  - `|`: Tuberías para encadenar la salida de un proceso con la entrada de otro.
  - `>` : Redirección de salida estándar (sobrescribir).
  - `>>`: Redirección de salida estándar (adjuntar/append).
  - `2>`: Redirección de errores (stderr).
- **Entrada en Raw Mode:**
  - La shell opera en modo *Raw* para un control preciso de la entrada del teclado, evitando el buffer de línea predeterminado del terminal.
- **Gestión de Historial (`history`):**
  - Almacenamiento persistente configurable mediante la variable de entorno `HISTFILE`.
  - Soporte para flags como `-r` (leer) y `-a` (append).
- **Built-in Commands:**
  - `cd`: Cambiar de directorio.
  - `pwd`: Mostrar directorio actual.
  - `echo`: Imprimir texto.
  - `type`: Identificar tipo de comando.
  - `exit`: Salida ordenada.

## 🚀 Cómo ejecutarla

### Prerrequisitos
Necesitas tener **Rust** y **Cargo** instalados.

### Pasos

1. **Clonar el repositorio:**
```bash
git clone https://github.com/Matias914/Generic-Shell.git
cd Generic-Shell

```


2. **Compilar y Ejecutar:**
   Modo desarrollo (rápido):
```bash
cargo run

```


Modo optimizado (release):
```bash
cargo build --release
./target/release/msh

```



## 💻 Ejemplos de Uso

Una vez dentro de la shell, puedes probar combinaciones avanzadas como:

```bash
# Navegación y variables
$ pwd
/home/usuario

# Uso de Pipes (|) y Redirecciones (>)
$ ls -la | grep "rs" > archivos_rust.txt$ cat archivos_rust.txt

# Manejo del historial
$ history
1 ls -la | grep "rs" > archivos_rust.txt
2 cat archivos_rust.txt
$ history -w

```