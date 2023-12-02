; computes nth bit of x + y
; takes 3 arguments: x, y, n
set v00 i02
set v01 0
stnb v02 i00 v01
stnb v03 i01 v01
jon nc
badd v02 1
badd v02 v03
iadd v01 1
cmp v01 v00
jon g
jmp 2
set v04 v02
end