/*
 * Crea una única función (importante que sólo sea una) que sea capaz
 * de calcular y retornar el área de un polígono.
 * - La función recibirá por parámetro sólo UN polígono a la vez.
 * - Los polígonos soportados serán Triángulo, Cuadrado y Rectángulo.
 * - Imprime el cálculo del área de un polígono de cada tipo.
 */

// Fórmula para un Triángulo: Area = base x altura / 2
// Fórmula para un Cuadrado: Area = base x altura
// Fórmula para un Rectángulo: Area = base x altura

interface PolygonInterface {
	fun area(): Double
	fun polyName(): String
}

class Triangle(val base: Double, val altitude: Double) : PolygonInterface{
	override fun area(): Double {
		return base * altitude / 2
	}
	override fun polyName(): String {
		return "Triangle"
	}
}

class Square(val side: Double) : PolygonInterface {
	override fun area(): Double {
		return side * side
	}
	override fun polyName(): String {
		return "Square"
	}
}

class Rectangle(val base: Double, val altitude: Double) : PolygonInterface {
	override fun area(): Double {
		return base * altitude
	}
	override fun polyName(): String {
		return "Rectangle"
	}
}

fun printPolygonArea(polygon: PolygonInterface) {
	println("El área del ${polygon.polyName()} es: ${polygon.area()}")
}

fun main() {
	printPolygonArea(Triangle(3.0, 4.0))
	printPolygonArea(Square(5.0))
	printPolygonArea(Rectangle(3.0, 4.0))
}

