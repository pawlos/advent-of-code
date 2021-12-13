import sequtils
import std/strutils
import lists

let inp = toSeq(lines "data/day9/input")

type
  Matrix[W, H: static[int]] =
    array[0..W-1, array[0..H-1, int]]

proc notAllBasins[W, H](a: Matrix[W, H]):bool =
    for i in 1..high(a):
        for j in 1..high(a[0]):
            if a[i][j] < 9:
                return true

    return false

var
    inp2: Matrix[102,102]
    low_points_sum:int
    i:int
    j:int

for i in countup(1, len(inp)-1):
    for j in countup(1, len(inp[0])-1):
        let v = inp[i][j]
        if inp[i][j-1] > v and inp[i][j+1] > v and inp[i-1][j] > v and inp[i+1][j] > v:
            let vv =  (ord(v) - 0x30) + 1
            low_points_sum += vv

echo "Low point sum: ", low_points_sum
#part 2
for i in countup(0, len(inp)-1):
    for j in countup(0, len(inp[0])-1):
        let v = inp[i][j]
        if v == '~':
            inp2[i][j] = 999999999
        else:
            inp2[i][j] = ord(v)-0x30   

var
    basin = 10
for i in countup(1, len(inp)-2):
    for j in countup(1, len(inp[0])-2):
        let v = inp[i][j]
        if inp[i][j-1] > v and inp[i][j+1] > v and inp[i-1][j] > v and inp[i+1][j] > v:
            inp2[i][j] = basin
            if inp[i][j-1] < '9' and inp[i][j-1] != '~':
                inp2[i][j-1] = basin
            if inp[i][j+1] < '9' and inp[i][j+1] != '~':
                inp2[i][j+1] = basin
            if inp[i-1][j] < '9' and inp[i-1][j] != '~':
                inp2[i-1][j] = basin
            if inp[i+1][j] < '9' and inp[i+1][j] != '~':
                inp2[i+1][j] = basin

            basin = basin+1
            #echo inp2

while true:
    let c = notAllBasins(inp2)
    if not c:
        echo "breaking"
        break
    for i in countup(1, len(inp)-1):
        for j in countup(1, len(inp[0])-1):
            let v = inp2[i][j]
            if v < 9:
                # searching for basin
                if inp2[i][j-1] > 9 and inp2[i][j-1] != 999999999: #got a basin
                    inp2[i][j] = inp2[i][j-1]
                elif inp2[i][j+1] > 9 and inp2[i][j+1] != 999999999:
                    inp2[i][j] = inp2[i][j+1]
                elif inp2[i+1][j] > 9 and inp2[i+1][j] != 999999999:
                    inp2[i][j] = inp2[i+1][j]
                elif inp2[i-1][j] > 9 and inp2[i-1][j] != 999999999:
                    inp2[i][j] = inp2[i-1][j]


var 
    a,b,c:int
    count:int

for basin_c in countup(10, basin):
    count = 0
    for i in countup(1, len(inp)-1):
        for j in countup(1, len(inp[0])-1):
            let v = inp2[i][j]
            if v == basin_c:
                count += 1

    if count > a:
        c = b
        b = a
        a = count 
    elif count > b:
        c = b
        b = count
    elif count > c:
        c = count

echo "A: ",a,", B: ",b,", C: ",c
echo a*b*c
