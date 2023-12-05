set v0 i0
set v1 i1
set v2 i2
int-add v0 v1
set b0 0
compare v0 v2
jump-over-next-if not-equal
set b0 1
end