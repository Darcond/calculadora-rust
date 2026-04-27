# Calculadora Web con Rust y WebAssembly

Una calculadora web interactiva que demuestra el poder de Rust compilado a WebAssembly (WASM) para realizar operaciones matemáticas de alto rendimiento en el navegador.

## 🚀 Características

- **Operaciones matemáticas completas**: Suma, resta, multiplicación, división, potencia y módulo.
- **Interfaz web intuitiva**: Diseño simple y responsivo con HTML, CSS y JavaScript.
- **Rendimiento optimizado**: Las operaciones se ejecutan en código Rust compilado a WASM para máxima eficiencia.
- **Manejo de errores robusto**: Gestión adecuada de divisiones por cero, valores infinitos y NaN.
- **Formato de resultados**: Visualización inteligente de números grandes/pequeños con notación científica o localización.

## 🏗️ Arquitectura

Este proyecto sigue una arquitectura híbrida que combina lo mejor de Rust y JavaScript:

- **Backend (Rust)**: Lógica de cálculo implementada en Rust, compilada a WebAssembly para ejecución en el navegador.
- **Interoperabilidad**: Uso de `wasm-bindgen` para exponer funciones Rust a JavaScript.
- **Frontend (JavaScript)**: Interfaz de usuario y manejo de eventos en JavaScript vanilla.
- **Estilos (CSS)**: Diseño responsivo y moderno.

### Diagrama de Arquitectura

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   HTML/CSS/JS   │────│   JavaScript     │────│   WebAssembly   │
│   (Interfaz)    │    │   (Interfaz)     │    │   (Rust Core)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## 📋 Requisitos

- **Rust**: Versión 1.70 o superior
- **wasm-pack**: Para compilar el proyecto a WebAssembly
- **Navegador web moderno**: Con soporte para WebAssembly (Chrome, Firefox, Safari, Edge)

## 🛠️ Instalación y Ejecución

### 1. Clonar el repositorio

```bash
git clone <url-del-repositorio>
cd mi-proyecto-wasm
```

### 2. Instalar dependencias

Asegúrate de tener `wasm-pack` instalado:

```bash
cargo install wasm-pack
```

### 3. Compilar el proyecto

```bash
wasm-pack build --target web
```

Esto generará los archivos WASM en el directorio `pkg/`.

### 4. Ejecutar la aplicación

Para servir los archivos estáticos, puedes usar cualquier servidor HTTP. Por ejemplo, con Python:

```bash
python -m http.server 8000
```

O con Node.js (si tienes `http-server` instalado):

```bash
npx http-server
```

Abre tu navegador y ve a `http://localhost:8000` (o el puerto correspondiente).

## 📖 Uso

1. Ingresa dos números en los campos de entrada.
2. Selecciona la operación deseada haciendo clic en el botón correspondiente.
3. El resultado se mostrará en la pantalla inferior.

### Operaciones disponibles

- **Sumar (+)**: `a + b`
- **Restar (-)**: `a - b`
- **Multiplicar (×)**: `a * b`
- **Dividir (÷)**: `a / b` (maneja división por cero)
- **Potencia (^)**: `a^b`
- **Módulo (%)**: `a % b`

## 🔧 Desarrollo

Para modificar el código Rust:

1. Edita `src/lib.rs`
2. Recompila con `wasm-pack build --target web`
3. Recarga la página en el navegador

Para modificar la interfaz:

1. Edita `index.html`, `index.css` o `index.js`
2. Recarga la página (no requiere recompilación)

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Ver el archivo `LICENSE` para más detalles.

## 🙏 Agradecimientos

- [Rust](https://www.rust-lang.org/) por el lenguaje de programación
- [WebAssembly](https://webassembly.org/) por la tecnología de compilación
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) por la interoperabilidad
