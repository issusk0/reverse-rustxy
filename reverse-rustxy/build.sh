#!/bin/bash

# 1. Cargar el entorno de Rust (esto soluciona el error 'cargo: orden no encontrada')
source "$HOME/.cargo/env"

PROJECT_NAME="rustxy"

echo "--- Iniciando compilación de $PROJECT_NAME ---"

# 2. Compilar para Linux
echo "Compilando para Linux..."
cargo build --release
if [ $? -eq 0 ]; then
    echo "Linux: OK (./target/release/$PROJECT_NAME)"
else
    echo "Error compilando para Linux"
    exit 1
fi

# 3. Preparar y Compilar para Windows
echo "Configurando target de Windows..."
rustup target add x86_64-pc-windows-gnu

echo "Compilando para Windows..."
cargo build --release --target x86_64-pc-windows-gnu

if [ $? -eq 0 ]; then
    echo "Windows: OK (./target/x86_64-pc-windows-gnu/release/$PROJECT_NAME.exe)"
else
    echo "Error: No se pudo compilar para Windows."
    echo "Asegúrate de tener instalado mingw-w64 (sudo apt install mingw-w64)"
fi

echo "--- Proceso terminado ---"