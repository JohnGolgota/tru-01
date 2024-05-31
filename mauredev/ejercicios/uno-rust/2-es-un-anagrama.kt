/*
 * Escribe una función que reciba dos palabras (String) y retorne
 * verdadero o falso (Bool) según sean o no anagramas.
 * - Un Anagrama consiste en formar una palabra reordenando TODAS
 *   las letras de otra palabra inicial.
 * - NO hace falta comprobar que ambas palabras existan.
 * - Dos palabras exactamente iguales no son anagrama.
 */

fun esUnAnagrama(palabra1: String, palabra2: String): Boolean {
    val palabra1List = palabra1.map { it.toLowerCaseChar() }
    val palabra2List = palabra2.map { it.toLowerCaseChar() }

    if (palabra1List.size != palabra2List.size) {
        return false
    }

    for (i in palabra1List.indices) {
        if (palabra1List[i] != palabra2List[palabra2List.size - i - 1]) {
            return false
        }
    }
    return true
}

fun main() {
	println(esUnAnagrama("alba", "alba"))
	println(esUnAnagrama("alba", "alb"))
	println(esUnAnagrama("alba", "abla"))
	println(esUnAnagrama("alba", "abal"))
	println(esUnAnagrama("alba", "bala"))
	println(esUnAnagrama("alba", "laba"))
	println(esUnAnagrama("alba", "baal"))
	println(esUnAnagrama("alba", "baal"))
}
