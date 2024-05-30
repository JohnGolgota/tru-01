/**
 * https://kotlinlang.org/docs/home.html
 */
// Comentarios en Kotlin

// Variables
/**
 * Respecto a las convenciones de Kotlin, las variables usan camelCase
 */
// Tipos de variables
const val a = 1
fun cosasDeVariablesYTipos() {
	val b = 2
	var c = 3

	// tipos de datos
	// Numeros
	val numero = 1 // Int
	val threeBillion = 1_000_000_000 // Long
	val oneLong = 1L // Long
	val oneByte: Byte = 1 // Byte
	// Float
	val pi = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679
	val oneHalf = 0.5 // Float
	val eFloat = 1.0f // Float
	// val one: Double = 1 // Error: type mismatch
	// Unsigned Numbers
	val maxUnsigned = 0u // UInt no expected type provided
	val UbyteUnsigned: UByte = 0u // UByte
	val ushortUnsigned: UShort = 0u // UShort
	val ulongUnsigned: ULong = 0u // ULong
	// val ulongUnsigned2 = 0UL // ULong
	// Boolean
	val trueBoolean = true // Boolean
	val falseBoolean = false // Boolean
	val boolNull: Boolean? = null // Boolean?
	// char
	val aChar = 'a' // Char
	// String
	val aString = "aa" // String
	// templates
	// String templates
	var templateString = "$aString $aString"
	val templateStringMultiline = """
	$aString
	$aString
	"""
	// array templates
	var templateArray = arrayOf("aa", "bb")
	templateArray += "dd"

	val nullArray: Array<Int?> = arrayOfNulls(3)
	var otherArray = emptyArray<String>()
}

fun main() {
	println("Hello World")
}
