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