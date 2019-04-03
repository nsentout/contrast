# Installation : sudo apt install texlive-full xindy python-pygments

default:
	pdflatex --shell-escape memoire
	makeglossaries memoire
	biber memoire
	pdflatex --shell-escape memoire
