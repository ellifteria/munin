stl v00 i02
stl v01 i00
cmp v01 v00
jon g
jmp 7
set b00 0
end
stl v01 i01
cmp v01 v00
jon le
jmp 5
set v01 0
stnb v02 i00 v01
stnb v03 i01 v01
stnb v04 i02 v01
badc v02 v03
cmp v04 v02
jon ne
jmp 21
set b00 0
end
iadd v01 1
cmp v01 v00
jon ge
jmp 12
set b00 1
end