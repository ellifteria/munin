set b00 0
stl v00 i00
set v01 0
cmp v01 v00
jon l
jmp 16
set v02 v00
isub v02 v01
isub v02 1
stnb v03 i00 v01
stnb v04 i00 v02
cmp v03 v04
jon e
end
iadd v01 1
jmp 3
set b00 1
end