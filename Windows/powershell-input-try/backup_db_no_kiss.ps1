$MYSQL_DIR = "C:\xampp\mysql"
$BACKUP_FOLDER = "back_up_folder"
$BACKUPDATE = Get-Date -Format "dd-MM-yyyy"
$DB_USER = 'data_base_user'
$DB_PASS = 'data_base_password'
$DATA_BASES = "db_1", "db_2", "db_3", "db_4", "db_5"
$selected_option = Read-Host -Prompt "Choose: pre-database; '1' for all databases; '2'"
switch ($selected_option) {
    '1' { Make_BackupFromArray -DataBasesArrayParam $DATA_BASES }
    '2' { Make_AllDatabasesBackup }
    '3' { Make_BackupFromTextFile }
    Default {
        $do_in_names = Read-Host -Prompt "`nwrite database name? (Y)"
        if ($do_in_names -eq 'y') {
            $db_user_chose = @()
            while ($do_in_names -eq 'y') {
                $db_user_chose += Read-Host -Prompt 'database name'
                $do_in_names = Read-Host -Prompt "`nwrite database name? (Y)"
            }
            Make_BackupFromArray -DataBasesArrayParam $db_user_chose
        }
        else {
            Write-Host "Ok. Bye"
        }
    }
}
function Make_BackupFromArray {
    param (
        $DataBasesArrayParam
    )
    try {
        Set-Location "$MYSQL_DIR\bin"
        $DataBasesArrayParam | ForEach-Object {
            .\mysqldump.exe -u $DB_USER -p $DB_PASS $_ > "$BACKUP_FOLDER\${_}($BACKUPDATE).sql"
            Write-Host "export db $_"
        }
    }
    catch {
        Write-Error "ups..."
        Write-Host $_
    }
}
function Make_AllDatabasesBackup {
    try {
        Set-Location "$MYSQL_DIR\bin"
        .\mysql.exe -u $DB_USER -e 'show databases' | ForEach-Object {
            .\mysqldump.exe -u $DB_USER -p $DB_PASS $_ > "$BACKUP_FOLDER\${_}($BACKUPDATE).sql"
            Write-Host "export db $_"
        }
    }
    catch {
        Write-Host "error"
        Write-Error $_
    }    
}
function Make_BackupFromTextFile {
    $item = Read-Host -Prompt "Drag and drop file here"
    $item_content = Get-Content $item
    if ($item_content -is [array]) {
        Make_BackupFromArray -DataBasesArrayParam $item_content
    }
    else {
        Write-Host "can't find database list"
    }
}