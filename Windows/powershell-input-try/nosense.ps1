try {
    nonsense
}
catch {
    Write-Host "error"
    Write-Host $_
    Write-Error "ups"
}