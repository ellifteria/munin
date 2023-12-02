; given max(size(x), size(y)) -> n
; computes nth bit of x + y
; takes 2 arguments: x, y
stl v00 i00
stl v01 i01
cmp v00 v01
jon g
set v00 v01
set v02 0
stnb v03 i00 v02
stnb v04 i01 v02
set v05 v03
jon nc
badd v05 1
badd v05 v04
iadd v02 1
cmp v02 v00
jon ge
jmp 6
jon nc
set v05 1
end