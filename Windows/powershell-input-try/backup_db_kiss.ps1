# Version Windows: Windows 10

# Determina la direccion de MySQL en el dispositivo
$MYSQL_DIR = "C:\xampp\mysql"
# Direccion Completa de la carpeta output para el resultado del backup
$BACKUP_FOLDER = "back_up_folder"

# Determinamos la fecha con el formato mas comodo para manipular los archivos.
# Cual? cualquier formato de fecha que no sea estado unidense
$BACKUPDATE = Get-Date -Format "dd-MM-yyyy"

# Credenciales de la base de datos MySQL
$dbuser = 'data_base_user'
$dbpass = 'data_base_password'

$Selected_Option = Read-Host -Prompt "Elige: Bases de datos escritas; '1' o Todas; '2'"
# Lista de bases de datos a importar desde MySQL (opcion 1)
$DATA_BASES = "base_de_datos_1", "base_de_datos_2", "base_de_datos_3", "base_de_datos_4", "base_de_datos_5"
# (Opcion 1)
if ($Selected_Option -eq '1') {
    try {
        # Nos Posisionamos el la direccion del dispositivo en la que este mysqldump.exe
        # con base a la direccion general de MySQL
        Set-Location "$MYSQL_DIR\bin"
        # Recorremos el array con las bases de datos
        $DATA_BASES | ForEach-Object {
            # Por esto necesitamos las direcciones completas del .exe de mysql y la carpeta output
            .\mysqldump.exe -u $dbuser -p $dbpass $_ > "$BACKUP_FOLDER\${_}($BACKUPDATE).sql"
        }
    }
    catch {
        # Queria usar un try catch no te voy a mentir
        Write-Error "Ups... valio verg..."
        Write-Host $_
    }
}
# (opcion 2)
elseif ($Selected_Option -eq '2') {
    try {
        # Parecido al anterior pero...
        Set-Location "$MYSQL_DIR\bin"
        # El array que recorremos en este caso es una consulta directa a la base de datos.
        # Traera un error porque intenta inportar el titulo de la lista que retorna MySQL.
        # Y tambiem inportara todas las otras bases de datos propias de MySQL
        .\mysql.exe -u $dbuser -e 'show databases' | ForEach-Object {
            # El resto funciona igual
            # Por esto necesitamos las direcciones completas del .exe de mysql y la carpeta output
            .\mysqldump.exe -u $dbuser -p $dbpass $_ > "$BACKUP_FOLDER\${_}($BACKUPDATE).sql"
        }
    }
    catch {
        Write-Host "error men"
        Write-Error $_
    }

} else {
    Write-Host "Che comediante encontre tu ciudad natal:`nhttps://www.google.com/maps/place/Alta+Gracia,+C%C3%B3rdoba,+Argentina/@-31.6555372,-64.4412516,13z/data=!3m1!4b1!4m6!3m5!1s0x942d574ade89939b:0x5290b7919d5d43fd!8m2!3d-31.6584428!4d-64.4273429!16zL20vMDQ3cWNi?entry=ttu"
}
