#!/bin/zsh

set -xe

xelatex praca_dyplomowa.tex
bibtex praca_dyplomowa
xelatex praca_dyplomowa.tex
xelatex praca_dyplomowa.tex
