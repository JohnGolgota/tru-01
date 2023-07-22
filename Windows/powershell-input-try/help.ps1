# Read-Host (function) indica la espera de un dato introducido por el usuario
$Nombre = Read-Host -Prompt 'Nombre'
# -Prompt (parametro) declara un mensage para remarcar la intencion de obtener un input
$Otro_Nombre = Read-Host -Prompt 'otro ->'
# Get-Date (fn) para obtener la fecha actual
# -Format (param) cambia la forma en la que retorna la fecha la fn Get-Date // ojala fuera así en JS
$fecha = Get-Date -Format "dd-MM-yyyy"

# Write-Host (fn) el console.log de PowerShell... resumiendo
Write-Host "Nombre: $Nombre, Otro Nombre: $Otro_Nombre`n$fecha"

# todos los Verbs de ps pueden ser listados con Get-Verb
# Estos Verbs han sido desarrollados en .NET o .NET core y pueden ser invocados desde PowerShell...
# Obiamente si los estoy usando aquí.

# Get command para listar los Verbs // no se que es eso //
# Verbs relacionados al Command // sea eso lo que sea
Get-Command -Verb Get -Noun File*
# Este es un ejemplo de comando que busca especificamente los archovis relacionados al obtener archivos
# Por los parametros -Verb y el valor Get esta porcion indica la obtencion de... no se no entendi
# Y el parametro -Noun y valor File* esta porcion indica la interaccion con archivos