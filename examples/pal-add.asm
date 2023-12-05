stl v00 i00
stl v01 i01
cmp v01 v00
jon l
set v00 v01
set v02 0
jmp 32
set v06 0
cmp v06 v00
jon l
jmp 30
set v07 0
set v08 v06
set v09 0
jmp 45
set v13 v09
set v08 v00
isub v08 v06
isub v08 1
iadd v08 v02
set v07 1
jmp 45
set v14 v09
cmp v13 v14
jon ne
jmp 28
set b00 0
end
iadd v06 1
jmp 8
set b00 1
end
set v03 0
stnb v04 i00 v03
stnb v05 i01 v03
badc v04 v05
iadd v03 1
cmp v03 v00
jon ge
jmp 33
set v02 0
jon nc
set v02 1
clf
jmp 7
clf
set v10 0
stnb v11 i00 v10
stnb v12 i01 v10
badc v11 v12
iadd v10 1
cmp v10 v08
jon g
jmp 47
set v09 v11
cmp v07 0
jon ne
jmp 15
jmp 22