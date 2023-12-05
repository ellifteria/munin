set-to-length-of v00 i00
set-to-length-of v01 i01
compare v00 v01
jump-over-next-if greater
set v00 v01
set v02 0
set-to-nth-bit v03 i00 v02
set-to-nth-bit v04 i01 v02
set v05 v03
jump-over-next-if no-carry
bit-add v05 1
bit-add v05 v04
int-add v02 1
compare v02 v00
jump-over-next-if greater-or-equal
jump-to 6
end