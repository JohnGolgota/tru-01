# Config

## steps

1. [winget](#winget)
2. [Git](#git)
3. [GitHub Desktop](#github-desktop)
4. [vscode](#visual-studio-code)
5. [wsl2](#windows-subsystem-for-linux)

## [Winget](https://learn.microsoft.com/es-es/windows/package-manager/winget/)

### instalacion

From [github](https://github.com/microsoft/winget-cli/releases) with

```bash
Install-Module -Name Microsoft.WinGet.Client
```

o [microsoft store](https://apps.microsoft.com/detail/9NBLGGH4NNS1?hl=es-co&gl=CO)

## [Git](https://git-scm.com/download/win)

<!-- Puta Session -->
```bash
winget install --id Git.Git -e --source winget

# Configs
git config --global user.name "JohnGolgota"
git config --global user.email "js684new@gmail.com"

# Clone primary
git clone https://github.com/JohnGolgota/tru-01.git $HOME/tru-01
git clone https://github.com/JohnGolgota/JS.git $env:JS

winget import $HOME/tru-01/.configuration/winget.json --ignore-unavailable --ignore-version --no-upgrade

```

## [GitHub Desktop](https://desktop.github.com/)
<!-- Session -->

## [Scoop](https://github.com/ScoopInstaller/Install#installation)

```bash
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

```bash
irm get.scoop.sh | iex
# You can use proxies if you have network trouble in accessing GitHub, e.g.
irm get.scoop.sh -Proxy 'http://<ip:port>' | iex
```

## [Visual Studio Code](https://code.visualstudio.com/)
<!-- Session -->
```bash
code $HOME/tru-01/.configuration/config.code-workspace
code-insiders $HOME/tru-01/.configuration/config.code-workspace
```

## [Windows Subsystem for Linux]()
