import * as wasm from "igo-wasm-demo";

function showResults(morphemes, comment) {
    let tbody = document.getElementById("results");
    while (tbody.firstChild) {
        tbody.removeChild(tbody.firstChild);
    }

    morphemes.forEach(function (m) {
        let tr = document.createElement("tr");

        let td = document.createElement("td");
        td.innerText = m.surface;
        tr.appendChild(td);

        td = document.createElement("td");
        td.innerText = m.feature;
        tr.appendChild(td);

        tbody.appendChild(tr);
    });

    let p = document.getElementById("comment");
    p.innerText = comment;
}

let tagger = null;
const textInput = document.getElementById("textInput");

function analyze() {
    if (tagger) {
        let text = textInput.value;
        let startTime = new Date();
        let results = wasm.parse(tagger, text);
        let elapsed = (new Date() - startTime) / 1000;
        showResults(results, `elapsed: ${elapsed} sec.`);
    }
}

textInput.addEventListener("input", function () {
    analyze();
}, false);

window.requestAnimationFrame(function () {
    let startTime = new Date();
    tagger = wasm.init_tagger();
    let elapsed = (new Date() - startTime) / 1000;
    textInput.value = "すもももももももものうち";
    analyze();
    document.getElementById("comment").innerText = `elapsed: ${elapsed} sec.`;
});
