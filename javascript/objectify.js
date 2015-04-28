// A javascript-based runtime compiler for objectified HTML code.

var markup;
window.onload = OnLoad();

function OnLoad() {
    markup = document.getElementById("base").innerHTML;

    RunCompiler();
}

function RunCompiler() {
    
}