# Version Windows: Windows 10

# Determina la direccion de mysql en el dispositivo
$MYSQL_DIR = "C:\xampp\mysql"
# Direccion Completa de la carpeta output para 
$BACKUP_FOLDER = "back_up"

$BACKUPDATE = Get-Date -Format "dd-MM-yyyy"

$dbuser = 'data_base_user'
$dbpass

$DATA_BASES = "base_de_datos_1","base_de_datos_2","base_de_datos_3","base_de_datos_4","base_de_datos_5"

try {
    Set-Location "$MYSQL_DIR\bin"
    $DATA_BASES | ForEach-Object {
        .\mysqldump.exe -u $dbuser $_ > "$BACKUP_FOLDER\${_}($BACKUPDATE).sql"
    }
}
catch {
    Write-Host "Ups... valio verg..."
}
