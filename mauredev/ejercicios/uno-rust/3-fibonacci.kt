/*
 * Escribe un programa que imprima los 50 primeros números de la sucesión
 * de Fibonacci empezando en 0.
 * - La serie Fibonacci se compone por una sucesión de números en
 *   la que el siguiente siempre es la suma de los dos anteriores.
 *   0, 1, 1, 2, 3, 5, 8, 13...
 */

fun fibonacci(n: Int): Int {
    return when (n) {
        0 -> 0
        1 -> 1
        else -> fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fun fibonacci2(limit: Int): Array<Long> {
	var n = emptyArray<Long>()
	for (i in 0..<limit) {
		if (n.size < 2 ) {
			n += i.toLong()
			continue
		}
		n += n[i - 1] + n[i - 2]
	}
	return n
}

fun main() {
	var n = fibonacci2(50)
	println(n.joinToString())

 //    var i = 0
	// while (n.size < 50) {
	// 	n += fibonacci(i)
 //        i++
	// }
	// println(n.joinToString())

}
