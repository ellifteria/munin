; computes nth bit of x + y
; takes 3 arguments: x, y, n
set v00 i02
set v01 0
set-to-nth-bit v02 i00 v01
set-to-nth-bit v03 i01 v01
jump-over-next-if no-carry
bit-add v02 1
bit-add v02 v03
int-add v01 1
compare v01 v00
jump-over-next-if greater
jump-to 2
set v04 v02
end