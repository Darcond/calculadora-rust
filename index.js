import init, { calcular } from './pkg/mi_proyecto_wasm.js';

await init();

globalThis.ejecutar = (operacion) => {
    const a = Number(document.getElementById('n1').value);
    const b = Number(document.getElementById('n2').value);

    const resultado = calcular(operacion, a, b);
    const display = document.getElementById('display');

    if (Number.isNaN(resultado)) {
        display.innerText = "Error";
    } else if (!Number.isFinite(resultado)) {
        display.innerText = "∞";
    } else if (Math.abs(resultado) > 1e12 || (Math.abs(resultado) < 1e-4 && resultado !== 0)) {
        display.innerText = resultado.toExponential(4);
    } else {
        display.innerText = resultado.toLocaleString('es-ES', {
            maximumFractionDigits: 4
        });
    }
};

console.log("Calculadora Rust cargada con Top-level await");