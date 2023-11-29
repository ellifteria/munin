stl v00 i00
stl v01 i01
cmp v01 v00
sni l
set v00 v01
set v02 0
jmp 32
set v06 0
cmp v06 v00
sni l
jmp 30
set v07 0
set v08 v06
set v09 0
jmp 47
set v13 v09
set v08 v00
sub v08 v06
sub v08 1
add v08 v02
set v07 1
jmp 47
set v14 v09
cmp v13 v14
sni ne
jmp 28
set b00 0
end
add v06 1
jmp 8
set b00 1
end
set v03 0
stnb v04 i00 v03
stnb v05 i01 v03
sni nc
addb v04 1
addb v04 v05
add v03 1
cmp v03 v00
sni ge
jmp 33
set v02 0
sni nc
set v02 1
clf
jmp 7
clf
set v10 0
stnb v11 i00 v10
stnb v12 i01 v10
sni nc
addb v11 1
addb v11 v12
add v10 1
cmp v10 v08
sni g
jmp 49
set v09 v11
cmp v07 0
sni ne
jmp 15
jmp 22