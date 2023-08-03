@ECHO OFF
@REM setlocal no se que hace
setlocal
@REM code-insiders porque uso vscode insiders... :)
@REM Si usara vscode seria: code ...
@REM %~dpo parece ser el current path del comando. no se cosas
code-insiders %~dpo..\.vscode\repo.code-workspace
@REM uso mucho los workspace. pero funcionaria con una carpeta y asi... pues es la misma monda de que: code . en la command line
@REM endlocal no que hace
endlocal
