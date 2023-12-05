set-to-length-of v00 i02
set-to-length-of v01 i00
compare v01 v00
jump-over-next-if greater
jump-to 7
set b00 0
end
set-to-length-of v01 i01
compare v01 v00
jump-over-next-if less-or-equal
jump-to 5
set v01 0
set-to-nth-bit v02 i00 v01
set-to-nth-bit v03 i01 v01
set-to-nth-bit v04 i02 v01
bit-add-with-carry v02 v03
compare v04 v02
jump-over-next-if not-equal
jump-to 21
set b00 0
end
int-add v01 1
compare v01 v00
jump-over-next-if greater-or-equal
jump-to 12
set b00 1
end