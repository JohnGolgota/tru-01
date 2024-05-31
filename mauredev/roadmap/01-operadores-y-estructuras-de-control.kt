/*
 * EJERCICIO:
 * - Crea ejemplos utilizando todos los tipos de operadores de tu lenguaje:
 *   Aritméticos, lógicos, de comparación, asignación, identidad, pertenencia, bits...
 *   (Ten en cuenta que cada lenguaje puede poseer unos diferentes)
 * - Utilizando las operaciones con operadores que tú quieras, crea ejemplos
 *   que representen todos los tipos de estructuras de control que existan
 *   en tu lenguaje:
 *   Condicionales, iterativas, excepciones...
 * - Debes hacer print por consola del resultado de todos los ejemplos.
 *
 * DIFICULTAD EXTRA (opcional):
 * Crea un programa que imprima por consola todos los números comprendidos
 * entre 10 y 55 (incluidos), pares, y que no son ni el 16 ni múltiplos de 3.
 *
 * Seguro que al revisar detenidamente las posibilidades has descubierto algo nuevo.
 */

// El = es el operador de asignación
// Tambien es usado para asignar valores por defecto a parámetros de funciones
// Operadores aritméticos
fun main() {
	var a = 10
	var bb = 5

	// Mathematical operators
	print(a + bb)
	print(a - bb)
	print(a * bb)
	print(a / bb)
	print(a % bb)
	// "*" is also used to pass an array to a verang parameter.
	fun <T> asList(vararg ts: T): List<T> {
		val result = ArrayList<T>()
		for (t in ts) // ts is an Array
		result.add(t)
		return result
	}
	val arrForList = arrayOf(1, 2, 3)
	// val list = asList(-1, 0, *a, 4) // Argument type mismatch: actual type is 'kotlin.Int', but 'kotlin.Array<out T>' was expected.
	val list = asList(-1, 0, 4)
	// augmented assignment
	a += 1
	a -= 1
	a *= 2
	a /= 2
	a %= 2
	// increment and decrement
	a++
	a--
	// Operadores lógicos
	var cc = true
	var d = false

	print(cc && d)
	print(cc || d)
	print(!cc)

	// Operadores de comparación
	var e = 10
	var f = 5

	print(e == f)
	print(e != f)
	print(e > f)
	print(e < f)
	print(e >= f)
	print(e <= f)

	// Operadores de asignación
	var g = 10
	g = 20
	print(g)

	// Operadores de identidad
	print(g === g)
	print(g !== g)

	// Operadores de pertenencia
	var h = arrayOf(1, 2, 3)
	print(h.size)

	// indexed access operator
	print(h[0])

	// !! assert that an expression is not null
	var b = "Cosas"
	val l = b!!.length

	// performs a safe call
	val toSafeCall: String? = null
	val safeCall = toSafeCall?.length

	// elvis operator
	val noElvis: Int = if (b != null) b.length else -1
	val elvis = b?.length ?: -1

	// :: creates a member reference or a class reference
	fun isOdd(x: Int) = x % 2 != 0
	val number_list = listOf(1, 2, 3, 4, 5)
	println(number_list.filter( ::isOdd )) // [1, 3, 5]

	// .. ..< create ranges
	println(4 in 1..4) // true
	println(4 in 1..< 4) // false

	// : separate a name from a type in a declaration
	val aWhitType: Int = 10

	// ? makes a type as nullable
	val bWhitTypeAndNullable: Int? = 10

	// -> separates the parameters and the body of a lambda expression.
	// -> separates the parameters and return type declaration in a function type.
	val sum: (Int, Int) -> Int = { a: Int, b: Int -> a + b }

	// -> separates the condition and the body of a when expression branch.
	val argToWhen = 1
	when (argToWhen) {
		1 -> println("one")
		2 -> println("two")
		else -> {
			println("other")
		}
	}

	// @ introduces an annotation
	/* No se esto lo saque de la documentacion... pero lo leo despues
	@Fancy class Foo {
		@Fancy fun baz(@Fancy foo: Int): Int {
			return (@Fancy 1)
		}
	}
	*/

// @ introduces or references a loop label
loop@ for (i in 1..10) {
	for (j in 1..10) {
		if (true) {
			break@loop
		}
	}
}

// @ introduces or references a lambda label
fun foo() {
	listOf(1, 2, 3, 4, 5).forEach lit@{
		if (it == 3) {
			return@lit
		}
		println(it)
	}
	println("done")
}

// @ references a "this" expression from an outer scope
class A { // implicit label @A
inner class B { // implicit label @B
fun Int.foo() { // implicit label @foo
val a = this@A // A's this
val b = this@B // B's this

val c = this // foo()'s receiver, an Int
val c1 = this@foo // foo()'s receiver, an Int

val funLit = lambda@ fun String.() {
	val d = this // funLit's receiver, a String
}

val funLit2 = { s: String ->
	// foo()'s receiver, since enclosing lambda expression
	// doesn't have any receiver
	val d1 = this
}
		}
	}
}

// references an outer superclass
open class Rectangle{
	open fun draw() { println("Drawing a rectangle") }
	val borderColor: String get() = "black"
}
class FilledRectangle: Rectangle() {
	override fun draw() {
		super.draw()
		println("Drawing a filled rectangle")
	}
	val fillColor: String get() = super.borderColor

	inner class Filler {
		fun fill() { println("Filling the rectangle") }
		fun drawAndFill() {
			super@FilledRectangle.draw()
			fill()
			println("Drawing the filler")
		}
	}
}
// ; separetes multiple statements on the same line
val aInSameLineStatemebts = 1; println(aInSameLineStatemebts)

// $ references a variable or expression in a string template
val name = "John"
println("Hello, $name!")

// _ substitutes an unused parameter in a lambda expression
val map = mapOf("key" to 1, "key2" to 2)
map.forEach { (_, value) -> println(value) }

// _ substitutes an unused parameter in a destructuring declaration
val (_, b2, c) = arrayOf(1, 2, 3)

// Condicionales
if (true) {
	print("Si")
}

val maxLimit = 1
val maxOrLimit = if(maxLimit > 10) 10 else if (maxLimit > 5) 5 else 1


// -> separates the condition and the body of a when expression branch.
// when structure
val argToWhen2 = 1
when (argToWhen2) {
	1 -> println("one")
	2 -> println("two")
	else -> {
		println("other")
	}
}

// for structure
for (i in 1..10) println(i)

// while structure
while (false) {
	println("while")
}

// do-while structure
do {
	println("do")
} while (false)

// try-catch-finally
try {
	throw Exception("Hello Exception")
} catch (e: Exception) {
	println("Caught exception: $e")
} finally {
	println("Finally")
}
}
