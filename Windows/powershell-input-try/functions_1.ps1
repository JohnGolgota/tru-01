function Saludos {
    Write-Host "hola"
}

Saludos

function Sal-Uno {
    param (
        $paramname
    )
    Write-Host $paramname
}

Sal-Uno -paramname "esto"

# Muy complicado queda pendiente