# WSL2 https://docs.microsoft.com/en-us/windows/wsl/install-win10
wsl --install -d Debian
wsl --set-default-version 2
wsl {
	# Dependecies wsl
	sudo apt update
	sudo apt upgrade -y
	# Dependecies avaliable with apt
	sudo apt install -y curl wget git zsh zip unzip gzip tar
	# tools
	sudo apt install -y tmux
	# Dependecies neovim
	git config --global user.name "JohnGolgota"
	git config --global user.email "js684new@gmail.com"
	nvm & pnpm { # Dependecies node https://github.com/nvm-sh/nvm#installing-and-updating
		curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
		nvm install --lts
		# https://pnpm.io/es/installation
		curl -fsSL https://get.pnpm.io/install.sh | sh -
	}

	neovim { # https://github.com/neovim/neovim/wiki/Installing-Neovim#linux
		curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage
		chmod u+x nvim.appimage
		./nvim.appimage
		# If the ./nvim.appimage command fails, try:
		err {
			./nvim.appimage --appimage-extract
			./squashfs-root/AppRun --version

			# Optional: exposing nvim globally.
			sudo mv squashfs-root /
			sudo ln -s /squashfs-root/AppRun /usr/bin/nvim
			nvim
		}
		# nvim Copilot https://github.com/github/copilot.vim
		copilot {
			git clone https://github.com/github/copilot.vim.git \
			~/.config/nvim/pack/github/start/copilot.vim
			{
				:Copilot setup
			}
		}
	}
	profilings {
		nvim ~/.bash_aliases | nvim ~/.zshrc | nvim ~/.bashrc
	}
	# Configs vscode https://code.visualstudio.com/docs/remote/wsl
}



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