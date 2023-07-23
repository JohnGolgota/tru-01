# Declaracion de una variable con valor
$Nombre = Read-Host -Prompt 'Nombre ->'
# Read-Host (fn) indica la espera de un valor introducido por el usuario
# -Prompt (param) agrega un mensage visible para expresar al usuario que introduzca un dato
# valor entre comillas... pos es un valor 'String'
$Otro_Nombre = Read-Host -Prompt 'Otro? ->'
# Get-Date (fn) retorna la fecha actual ma o menos
# -Format (param) indica el formato en el que debe retornar la fecha la fn Get-Date
$fecha = Get-Date -Format "dd-MM-yyyy"

# Write-Host retorna algun output
Write-Host "Nombre: $Nombre, Otro?: $Otro_Nombre`n$fecha"

# Verb's no se que son pero se que estan hechos en .NET o .NET Core
# Y se supone que existe alguna diferencia entre las Function, Alias y Cmdlet
# Get es una accion o algo no se lo que me importa ahora es que obtiene cosas
# En este caso retornaria una lista de Command's
# Los Command's que tengan algo que ver con los obtener esto incluye las Fuction, Cmdlet y sus Alias
Get-Command -Verb Get -Noun File*
# -Noun (param) no lo se no entendi
# pero el valor File* despues del param -Noun indica... a la funcion... que...
# RETORNE LA LISTA DE COMMAND'S RELACIONADOS A LOS ARCHIVOS
# En conclusion listaria los command's que obtengan cosas de archivos del sistema? Exacto.
# Bueno en realidad no se.