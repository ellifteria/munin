; checks x + y = z
; takes 3 arguments: x, y, z
stl v00 i02
set v01 0
stnb v02 i00 v01
stnb v03 i01 v01
stnb v04 i02 v01
sni nc
addb v02 1
addb v02 v03
cmp v04 v02
sni ne
jmp 13
set v05 0
end
add v01 1
cmp v01 v00
sni ge
jmp 2
set v05 1
end