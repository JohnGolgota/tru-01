winget {
	installation {
		Install-Module -Name Microsoft.WinGet.Client
	}
}
scoop {
	Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
	Invoke-RestMethod get.scoop.sh | Invoke-Expression
	# You can use proxies if you have network trouble in accessing GitHub, e.g.
	Invoke-RestMethod get.scoop.sh -Proxy 'http://<ip:port>' | Invoke-Expression
}
git {
	$Session = $true
	installation {
		winget install --id Git.Git -e --source winget
	}
	config {
		git config --global user.name "JohnGolgota"
		git config --global user.email "js684new@gmail.com"
	}
	primary repostory {
		git clone https://github.com/JohnGolgota/tru-01.git $HOME/tru-01
		git clone https://github.com/JohnGolgota/JS.git $env:JS
	}
	winget {
		import {
			winget import $HOME/tru-01/.configuration/winget.json --ignore-unavailable --ignore-version --no-upgrade
		}
	}
	if ($Session) {
		return $true
	}
}
github {
	primary repostory {
		github $HOME/tru-01
		github $env:JS
	}
	return $true
}
code & code-insiders {
	initial open {
		code $HOME/tru-01/.configuration/config.code-workspace
		code-insiders $HOME/tru-01/.configuration/config.code-workspace
	}
	return $true
}
pwsh {
	installation {
		winget install --id Microsoft.PowerShell -e --source winget
	}
	config {
		pwsh $HOME/tru-01/.configuration/config.pwsh.ps1
	}
	profile {
		code-insiders.cmd $PROFILE
		{
			. Custom_Funciones.ps1
			Set-CustomMain
		}
	}
	return $true
}
wt {
	installation {
		winget install --id Microsoft.WindowsTerminal -e --source winget
	}
	config {
		wt -p "Windows PowerShell" -d $HOME/tru-01/.configuration/config.wt.json
	}
	return $true
}
wsl {
	# WSL2 https://docs.microsoft.com/en-us/windows/wsl/install-win10
	wsl --install -d Debian
	wsl --set-default-version 2
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
	node js {
		nvm & pnpm {
			# Dependecies node https://github.com/nvm-sh/nvm#installing-and-updating
			curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
			nvm install --lts
		}
		pnpm {
			# https://pnpm.io/es/installation
			curl -fsSL https://get.pnpm.io/install.sh | sh -
		}
	}
	neovim {
		# https://github.com/neovim/neovim/wiki/Installing-Neovim#linux
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
		copilot {
			# nvim Copilot https://github.com/github/copilot.vim
			git clone https://github.com/github/copilot.vim.git \
			~/.config/nvim/pack/github/start/copilot.vim
			{
				:Copilot setup
			}
		}
	}
	code & code-insiders {
		# Configs vscode https://code.visualstudio.com/docs/remote/wsl
		code-insiders.cmd
		code.cmd
	}
	profilings {
		nvim ~/.bash_aliases | nvim ~/.zshrc | nvim ~/.bashrc
		{
			# alias x = nvim
			# alias c = code-insiders
		}
	}
}
# Microsoft.VisualStudio.Community
node {

	nvm {
		# nvm https://github.com/coreybutler/nvm-windows
		nvm install --lts
	}
	pnpm {
		# pnpm https://pnpm.io/es/installation
		Invoke-WebRequest https://get.pnpm.io/install.ps1 -useb | Invoke-Expression
	}
}

# Notion

# Android Studio

# dbeaver

# Docker

# Pcmanager

# Edge

# DevHome

# Windows Terminal

# Python https://www.python.org/downloads/
