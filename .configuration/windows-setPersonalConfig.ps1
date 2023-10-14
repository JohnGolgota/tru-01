# WSL2 https://docs.microsoft.com/en-us/windows/wsl/install-win10
wsl --install -d Debian
wsl --set-default-version 2
	# Dependecies wsl
	sudo apt update
	sudo apt upgrade -y
	# Dependecies zip
	sudo apt install -y curl wget git zsh
	git config --global user.name "JohnGolgota"
	git config --global user.email ""
	sudo apt install -y zip unzip gzip tar
	# Dependecies node
	curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
		nvm install --lts
		curl -fsSL https://get.pnpm.io/install.sh | sh -
	# tools
	sudo apt install -y tmux nvim # nvim Copilot https://github.com/github/copilot.vim
	# Configs vscode https://code.visualstudio.com/docs/remote/wsl
	


# Microsoft.VisualStudio.Community

# nvm https://github.com/coreybutler/nvm-windows
nvm install --lts

# Notion

# Android Studio

# dbeaver

# Docker

# Git https://git-scm.com/download/win
git config --global user.name "JohnGolgota"
git config --global user.email ""

# Github Desktop

# Pcmanager

# Edge

# DevHome

# Windows Terminal

# Python https://www.python.org/downloads/

# Node https://nodejs.org/en/download/

# VS Code