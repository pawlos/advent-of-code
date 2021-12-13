import sequtils

type
  Matrix[W, H: static[int]] =
    array[0..W-1, array[0..H-1, int]]

  bMatrix[W, H: static[int]] =
    array[0..W-1, array[0..H-1, bool]]

let inp = toSeq(lines "data/day11/input")


var
    m: Matrix[10,10]
    col: int
    row: int
    flashes: int
    step_flashes: int

proc echoM(m:Matrix[10,10])=
    for i in countup(0,9):
        echo m[i]
    echo "---"

proc reset(m: Matrix[10,10]):Matrix[10,10] =
    for i in countup(0,9):
        for j in countup(0,9):
            result[i][j] = m[i][j]
            if result[i][j] < 0:
                result[i][j] = 0

proc anyStillCanFlash(m: Matrix[10,10]):bool =
    for i in countup(0,9):
        for j in countup(0,9):
            if m[i][j] > 9:
                return true

    return false

proc allZeros(m: Matrix[10,10]):bool=
    for i in countup(0,9):
        for j in countup(0,9):
            if m[i][j] != 0:
                return false

    return true

proc step(m: Matrix[10,10]):(Matrix[10,10],int) =
    var flashed = 0
    var next:Matrix[10,10]
    let off = [-1,0,1]
    for i in countup(0,9):
        for j in countup(0,9):
            next[i][j] = m[i][j]+1

    while anyStillCanFlash(next):
        for i in countup(0,9):
            for j in countup(0,9):
                if next[i][j] > 9:
                    next[i][j] = -10000000
                    flashed += 1
                    for k in off:
                        for l in off:
                            let i1 = i + k
                            let j1 = j + l
                            if i1 >= 0 and j1 >= 0 and i1 < 10 and j1 < 10:                                
                                next[i1][j1] += 1

    result = (next, flashed)

col = 0
row = 0
for l in inp:
    col = 0
    for c in l:
        var d = (ord(c) - 0x30)
        m[row][col] = d
        col += 1
    row += 1
#echoM(m)
for i in countup(1,1000):
    (m,step_flashes) = step(m)
    m = reset(m)
    flashes += step_flashes

    if i == 100:
        echo "Step: ",i,", flashed: ",flashes
    if allZeros(m):
        echo "Step in which all flash is: ", i
        break