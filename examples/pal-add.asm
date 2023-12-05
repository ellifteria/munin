set-to-length-of v00 i00
set-to-length-of v01 i01
compare v01 v00
jump-over-next-if less
set v00 v01
set v02 0
jump-to 32
set v06 0
compare v06 v00
jump-over-next-if less
jump-to 30
set v07 0
set v08 v06
set v09 0
jump-to 47
set v13 v09
set v08 v00
int-subtract v08 v06
int-subtract v08 1
int-add v08 v02
set v07 1
jump-to 47
set v14 v09
compare v13 v14
jump-over-next-if not-equal
jump-to 28
set b00 0
end
int-add v06 1
jump-to 8
set b00 1
end
set v03 0
set-to-nth-bit v04 i00 v03
set-to-nth-bit v05 i01 v03
jump-over-next-if no-carry
bit-add v04 1
bit-add v04 v05
int-add v03 1
compare v03 v00
jump-over-next-if greater-or-equal
jump-to 33
set v02 0
jump-over-next-if no-carry
set v02 1
clear-flags
jump-to 7
clear-flags
set v10 0
set-to-nth-bit v11 i00 v10
set-to-nth-bit v12 i01 v10
jump-over-next-if no-carry
bit-add v11 1
bit-add v11 v12
int-add v10 1
compare v10 v08
jump-over-next-if greater
jump-to 49
set v09 v11
compare v07 0
jump-over-next-if not-equal
jump-to 15
jump-to 22