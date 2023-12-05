set b00 0
set-to-length-of v00 i00
set v01 0
compare v01 v00
jump-over-next-if less
jump-to 16
set v02 v00
int-subtract v02 v01
int-subtract v02 1
set-to-nth-bit v03 i00 v01
set-to-nth-bit v04 i00 v02
compare v03 v04
jump-over-next-if equal
end
int-add v01 1
jump-to 3
set b00 1
end