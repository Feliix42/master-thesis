# not working with lualatex ._.
# $aux_dir = ".textmp";   # write temporary files to separate folder
$pdflatex = "lualatex -shell-escape %O %S"; # set default latex engine to lualatex, allow shell escape for minted
$pdf_mode = 1;          # tex -> pdf
@cus_dep_list = (@cus_dep_list, "code/*.rs");