INSTRUCCIONES PARA EJECUTAR EL PROYECTO

(Comandos marcados con ‼️)
📥 Paso 1: Instalar Requisitos
Node.js (para Electron/frontend):

    Descargar instalador desde: https://nodejs.org/es/download/

    Hacer doble clic en el archivo descargado y seguir los pasos.

Rust (para el backend Actix):

    Descargar desde: https://www.rust-lang.org/es/tools/install (indicar npm en la pagina)

    Ejecutar en PowerShell o CMD:
    bash

    ‼️ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    (Si falla en Windows, usar el instalador gráfico)

    Presionar Enter para instalar con opciones predeterminadas.

🖥️ Paso 2: Ejecutar el Proyecto
🔵 Backend (Terminal 1):

    Abrir PowerShell o CMD como administrador.

    Navegar a la carpeta del backend:
    bash

‼️ cd ruta\al\proyecto\backend

Compilar e iniciar el servidor:
bash

    ‼️ cargo build --release --target x86_64-pc-windows-gnu
    ‼️ cargo run --release

    Esperar a que aparezca: Servidor iniciado en http://localhost:8080

🟢 Frontend (Terminal 2):

    Abrir otra ventana de PowerShell o CMD.

    Navegar a la carpeta del frontend:
    bash

‼️ cd ruta\al\proyecto\frontend

Instalar dependencias (solo primera vez):
bash

‼️ npm install

Iniciar la interfaz:
bash

    ‼️ npm start

    Se abrirá automáticamente la ventana de Electron con tu aplicación.

⚠️ Si hay errores:

    Puertos ocupados:

        Verifica que ningún programa use el puerto 8080 (backend) o 3000 (frontend).

        Para cambiar puertos:

            Backend: Modificar 127.0.0.1:8080 en backend/src/main.rs.

            Frontend: Ajustar la URL en frontend/main.js (línea win.loadFile).

    Errores de compilación:

        Asegúrate de tener el target de Windows instalado:
        bash

    ‼️ rustup target add x86_64-pc-windows-gnu

Electron no inicia:

    Reinstalar dependencias:
    bash

        ‼️ npm cache clean --force
        ‼️ rm -rf node_modules package-lock.json
        ‼️ npm install

📌 Notas clave:

    Electron se conecta al backend en http://localhost:8080 (definido en tu main.js).

    El frontend carga directamente index.html desde ../public/ (asegúrate de que la ruta sea correcta).

    Para distribuir:
    bash

‼️ cd frontend
‼️ npm run make

(Genera el instalador en frontend/out/make/)