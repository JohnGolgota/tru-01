$Ser = Read-Host -Prompt 'Nombre?'
$User = Read-Host -Prompt 'otro ->'
$fecha = Get-Date -Format "dd-MM-yyyy"

Write-Host "your -> $Ser, other -> $User `n $fecha"