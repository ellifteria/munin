set-to-length-of v0 i0
set v1 0
set v2 1
do-nothing
bit-add v2 1
int-add v1 1
compare v1 v0
jump-over-next-if greater-or-equal
jump-to 3
set b0 v2
end
