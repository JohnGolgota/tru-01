$Selected_Option = Read-Host -Prompt 'Elige opcion uno "1" o opcion dos "2" ->'

if ($Selected_Option -eq '1') {
    Write-Host "elegiste 1"
}
elseif ($Selected_Option -eq '2') {
    Write-Host "elegiste 2"
}
else {
    Write-Host "Graciosito eh?"
}
$Introducir_Nombres = Read-Host -Prompt 'escribir nombres? (S)'
if ($Introducir_Nombres -eq 's') {
    $Data_B = @()
    while ($Introducir_Nombres -eq 's') {

        $Data_B += Read-Host -Prompt 'Nombre de una base de datos'
        $Introducir_Nombres = Read-Host -Prompt 'escribir nombres? (S)'

    }
    $Data_B | ForEach-Object {
        Write-Host "$_"
    }
}

Write-Host "$Selected_Option"