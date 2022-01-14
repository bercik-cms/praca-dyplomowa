#!/bin/zsh

set -xe

lualatex praca_dyplomowa.tex
bibtex praca_dyplomowa
lualatex praca_dyplomowa.tex
lualatex praca_dyplomowa.tex
