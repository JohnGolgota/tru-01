New-Item $PROFILE -Type File -Force
Add-Content -Path $PROFILE -Value ". Custom_Funciones.ps1"
Add-Content -Path $PROFILE -Value "Set-CustomMain"