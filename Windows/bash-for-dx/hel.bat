@ECHO OFF
@REM ECHO OFF
@REM INSPIRADO EN cmd /min /C "set __COMPAT_LAYER=RUNASINVOKER && start "" %1"
@REM en la linea anterior se ve que el archivo que arrastre y suelte sobre este .bat sera tratado como: %1
echo %1

@REM https://ss64.com/nt/for.html
FOR /F "tokens=1-7" %%a IN (%1) DO (
    ECHO %%a %%b %%c %%d %%e %%f %%g >> result.csv
    ECHO %%a %%b %%c %%d -%%e %%f %%g >> result.csv
)
@REM PAUSE fuerza al programa a permanecer a la espera de una accion para cerrarlo
PAUSE