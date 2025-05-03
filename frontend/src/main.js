const { app, BrowserWindow } = require('electron');
const path = require('path');
const { exec } = require('child_process'); // Importar child_process para ejecutar el backend

// Ejecutar el backend (Actix Web)
function startBackend() {
  const backendExecutable = path.join(__dirname, '../backend/target/x86_64-pc-windows-gnu/release/backend.exe'); // Ruta al ejecutable del backend

  exec(backendExecutable, (error, stdout, stderr) => {
    if (error) {
      console.error(`Error al ejecutar el backend: ${error.message}`);
      return;
    }
    if (stderr) {
      console.error(`stderr: ${stderr}`);
      return;
    }
    console.log(`stdout: ${stdout}`);
  });
}

function createWindow() {
  const win = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false,
    },
  });

  // Cargar la interfaz frontend
  win.loadFile(path.join(__dirname, '../public/index.html')); // Correct path resolution
}

app.whenReady().then(() => {
  startBackend(); // Iniciar el backend
  createWindow(); // Crear la ventana del frontend
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});
