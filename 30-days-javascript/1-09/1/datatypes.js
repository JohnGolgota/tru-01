import { cadena, bool, indefinido, nulo } from "./variables.js";

console.log(typeof cadena);
console.log(typeof bool);
console.log(typeof indefinido);
console.log(typeof nulo);

console.log(typeof undefined);
console.log(typeof null);

console.log(typeof NaN);
console.log(typeof Infinity);
console.log(typeof -Infinity);
console.log(typeof 0);
console.log(typeof 1);
console.log(typeof "hola");
console.log(typeof new Date());

console.log(typeof []);
console.log(typeof [1, 2, 3]);
console.log(typeof ["hola", "mundo"]);
console.log(typeof new Array(3));
console.log(typeof new Array(3).fill(0));

console.log(typeof {});
console.log(typeof { hola: "mundo" });
console.log(typeof { hola: "mundo", adios: "adios" });
console.log(typeof new Object());

console.log(typeof function() {});
console.log(typeof (function() {}));
console.log(typeof (function() { return "hola"; }));
console.log(typeof (function() { return { hola: "mundo" }; }));

console.log(typeof Promise);
console.log(typeof Promise.resolve());
console.log(typeof Promise.reject());

console.log(typeof async function() {});
