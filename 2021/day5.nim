import sequtils
import std/strutils
import lists

type
    Line = ref object of RootObj
        x1: int
        y1: int
        x2: int
        y2: int

    Matrix[W, H: static[int]] =
        array[0..W, array[0..H, int]]

    Board = Matrix[1000,1000]

proc printLine(l:Line)=
    echo "X1: ",l.x1,", Y1: ",l.y1,", X2: ",l.x2,", Y2: ",l.y2

proc isHorizontal(l:Line):bool=
    return l.y1 == l.y2

proc isVertical(l:Line):bool=
    return l.x1 == l.x2

proc isHorizontalOrVertical(l:Line):bool =
    return isHorizontal(l) or isVertical(l)

proc toLine(l:string): Line =
    let parts = l.split(" -> ")
    let fr = parts[0].split(",")
    let to = parts[1].split(",")
    let x1 = parseInt(fr[0])
    let y1 = parseInt(fr[1])
    let x2 = parseInt(to[0])
    let y2 = parseInt(to[1])
    result = Line(x1: x1, y1: y1, x2: x2, y2: y2)

proc countWithValAbove1(b:Board):int=
    for x in 0..high(b):
        for y in 0..high(b[0]):
            if b[x][y] >= 2:
                result += 1

proc markLine(b:Board, l:Line, incDiagonal:bool=false):Board=
    result = b
    if isHorizontal(l):
        let fromX = min(l.x1,l.x2)
        let toX = max(l.x1,l.x2)
        for i in countup(fromX,toX):            
            result[i][l.y1] = b[i][l.y1] + 1
    elif isVertical(l):
        let fromY = min(l.y1,l.y2)
        let toY = max(l.y1,l.y2)
        for i in countup(fromY, toY):
            result[l.x1][i] = b[l.x1][i] + 1 
    else: #diagonal
        if not incDiagonal:
            return
        #echo "Doing diagonal"
        #printLine(l)
        var xslope = l.x2 - l.x1
        if xslope < 0:
            xslope = -1
        else:
            xslope = 1

        var yslope = l.y2 - l.y1
        if yslope < 0:
            yslope = -1
        else:
            yslope = 1

        let ll = abs(l.x1 - l.x2)
        var x1 = l.x1
        var y1 = l.y1
        for i in 0..ll:
            result[x1][y1] = b[x1][y1] + 1
            x1 += xslope
            y1 += yslope

let inp = toSeq(lines "data/day5/input")

var 
    lines = initSinglyLinkedList[Line]()
    maxX: int = 0
    maxY: int = 0

for l in inp:
    let line = toLine(l)
    if line.x1 > maxX:
        maxX = line.x1
    if line.x2 > maxX:
        maxX = line.x2
    if line.y1 > maxY:
        maxY = line.y1
    if line.y2 > maxY:
        maxY = line.y2
    lines.add(line)

var board1, board2: Board

for line in lines:
    if isHorizontalOrVertical(line):        
        board1 = markLine(board1, line)

for line in lines:
    board2 = markLine(board2, line, true)

echo countWithValAbove1(board1)
echo countWithValAbove1(board2)