$Selected_Option = Read-Host -Prompt "Elige un numero 1 0 2"

switch ($Selected_Option) {
    '1' { Write-Host "Correcto" }
    '2' { Saludos }
    Default { Write-Error "Don comedia el circo esta pa' alla 👉" }
}