stl v00 i00
stl v01 i01
cmp v00 v01
sni g
set v00 v01
set v02 0
stnb v03 i00 v02
stnb v04 i01 v02
set v05 v03
sni nc
addb v05 1
addb v05 v04
add v02 1
cmp v02 v00
sni ge
jmp 6
end