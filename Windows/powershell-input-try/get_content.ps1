# Operaciones con archivos de SO
$item = Read-Host -Prompt "Directory"
$item_content = Get-Content $item

Write-Host "this ->`n$item"
Write-Host "those ->`n$item_content"

$item_content | ForEach-Object {
    Write-Host $_
}

function Make_BackupFromTextFile {
    param (
        $DataBasesFile
    )
    $item = Read-Host -Prompt "Directory"
    $item_content = Get-Content $item
}