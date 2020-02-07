# Set up pdflatex
$pdflatex = 'pdflatex %O -shell-escape -interaction=nonstopmode %S';
$pdf_mode = 1;          # tex -> pdf

@default_files = ('thesis.tex');

# use bibtex
$bibtex_use = 1;
