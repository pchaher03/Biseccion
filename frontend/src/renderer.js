document.getElementById('calculateButton').addEventListener('click', function() {
    const polynomial = document.getElementById('polynomial').value.trim();
    const intervalA = document.getElementById('intervalA').value.trim();
    const intervalB = document.getElementById('intervalB').value.trim();

    if (polynomial === "") {
        alert("Por favor, ingrese un polinomio.");
        return;
    }

    if (intervalA === "" || intervalB === "") {
        alert("Por favor, ingrese ambos límites del intervalo.");
        return;
    }

    const a = parseFloat(intervalA);
    const b = parseFloat(intervalB);

    if (isNaN(a) || isNaN(b)) {
        alert("Los límites del intervalo deben ser números.");
        return;
    }

    if (a >= b) {
        alert("El límite inferior debe ser menor que el límite superior.");
        return;
    }

    // Mostrar los datos ingresados en la interfaz
    document.getElementById('result').innerText = `Polinomio: ${polynomial}\nIntervalo: [${a}, ${b}]`;

    const dataToSend = JSON.stringify({ polynomial, a, b });
    console.log("Sending JSON:", dataToSend);

    // Enviar los datos al backend
    fetch('http://localhost:8080/solve', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ polynomial, a, b })
    })
    .then(response => {
        console.log("Raw response:", response);
        return response.json();
    })
    .then(data => {
        console.log("Parsed JSON:", data); // <-- Esto nos dirá qué está devolviendo el backend
        document.getElementById('result').innerText = `Raíz encontrada: ${data.root}\nError: ${data.error}\nIteraciones: ${data.iterations}`;
    })

    .catch(error => {
        console.error('Error:', error);
        alert("Hubo un error al calcular la raíz. Verifique la consola para más detalles.");
    });
});
