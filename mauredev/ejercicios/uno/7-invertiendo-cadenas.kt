/*
 * Crea un programa que invierta el orden de una cadena de texto
 * sin usar funciones propias del lenguaje que lo hagan de forma automática.
 * - Si le pasamos "Hola mundo" nos retornaría "odnum aloH"
 */

fun stringInvertir(cadena: String): String {
	val cadenaInvertida = StringBuilder()
	// val cadenaArray = cadena.split("")
	val cadenaArray = cadena.toCharArray()
	for (i in cadenaArray.size downTo 1) {
		cadenaInvertida.append(cadenaArray[i-1])
	}
	return cadenaInvertida.toString()
}

fun main() {
	println(stringInvertir("Hola mundo"))
}
