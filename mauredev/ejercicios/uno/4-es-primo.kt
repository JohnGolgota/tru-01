/*
 * Escribe un programa que se encargue de comprobar si un número es o no primo.
 * Hecho esto, imprime los números primos entre 1 y 100.
 */

fun isPrime(n: Int): Boolean {
    if (n <= 1) return false
    if (n == 2) return true
    if (n % 2 == 0) return false
    for (i in 3..n / 2) {
        if (n % i == 0) return false
    }
    return true
}

fun main() {
    for (i in 2..100) {
        if (isPrime(i)) println(i)
    }
}
