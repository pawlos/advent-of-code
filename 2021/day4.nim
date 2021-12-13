import std/strutils
import sequtils
import lists
import macros

macro debug*(n: varargs[typed]): untyped =
  result = newNimNode(nnkStmtList, n)
  for i in 0..n.len-1:
    if n[i].kind == nnkStrLit:
      # pure string literals are written directly
      result.add(newCall("write", newIdentNode("stdout"), n[i]))
    else:
      # other expressions are written in <expression>: <value> syntax
      result.add(newCall("write", newIdentNode("stdout"), toStrLit(n[i])))
      result.add(newCall("write", newIdentNode("stdout"), newStrLitNode(": ")))
      result.add(newCall("write", newIdentNode("stdout"), n[i]))
    if i != n.len-1:
      # separate by ", "
      result.add(newCall("write", newIdentNode("stdout"), newStrLitNode(", ")))
    else:
      # add newline
      result.add(newCall("writeLine", newIdentNode("stdout"), newStrLitNode("")))

type
    #Num* = ref object of RootObj
    #    val*: int
    #    checked*: bool
    
    #Row* = openArray[int] 

    Board* = ref object of RootObj
        r1*: ref array[0..4, int]
        r2*: ref array[0..4, int]
        r3*: ref array[0..4, int]
        r4*: ref array[0..4, int]
        r5*: ref array[0..4, int]
        rest: int
        won: bool

proc convertToRow(l: string): ref array[5, int] =
    #echo "Parsing: ", l
    result = new array[0..4, int]
    var v = l.split(' ')
    var i = 0
    for vv in v:
        if vv == "":
            continue
        #echo vv,", "        
        result[i] = parseInt(vv)
        i += 1

proc checkRow(r: ref array[0..4, int], n: int): bool =
    if r[0] == n:
        r[0] = -1
        return true
    if r[1] == n:
        r[1] = -1
        return true
    if r[2] == n:
        r[2] = -1
        return true
    if r[3] == n:
        r[3] = -1
        return true
    if r[4] == n:
        r[4] = -1
        return true
    return false

proc markNumberIfOnboard(b: Board, n:int) =
    if checkRow(b.r1, n):
        b.rest -= n
        return
    if checkRow(b.r2, n):
        b.rest -= n
        return
    if checkRow(b.r3, n):
        b.rest -= n
        return
    if checkRow(b.r4, n):
        b.rest -= n
        return
    if checkRow(b.r5, n):
        b.rest -= n
        return

proc checkWinningRow(r: ref array[0..4, int]): bool =
    if r[0] == -1 and r[1] == -1 and r[2] == -1 and r[3] == -1 and r[4] == -1:
        return true
    else:
        return false

proc checkWinningCol(b: Board, col: int): bool =
    if b.r1[col] == -1 and b.r2[col] == -1 and b.r3[col] == -1 and b.r4[col] == -1 and b.r5[col] == -1:
        return true
    else:
        return false

proc sum(a: ref array[0..4, int]): int =
    return a[0] + a[1] + a[2] + a[3] + a[4] 

proc checkIfWinning(b: Board): bool =
    if checkWinningRow(b.r1):
        return true
    if checkWinningRow(b.r2):
        return true
    if checkWinningRow(b.r3):
        return true
    if checkWinningRow(b.r4):
        return true
    if checkWinningRow(b.r5):
        return true
    if checkWinningCol(b, 0):
        return true
    if checkWinningCol(b, 1):
        return true
    if checkWinningCol(b, 2):
        return true
    if checkWinningCol(b, 3):
        return true
    if checkWinningCol(b, 4):
        return true
    result = false

proc sumOfUnmarked(b: Board): int =
    return b.rest

proc createBoard(l1, l2, l3, l4, l5: string): Board =
    result = new(Board)
    result.r1 = convertToRow(l1)
    result.r2 = convertToRow(l2)
    result.r3 = convertToRow(l3)
    result.r4 = convertToRow(l4)
    result.r5 = convertToRow(l5)
    result.rest = sum(result.r1) + sum(result.r2) + sum(result.r3) + sum(result.r4) + sum(result.r5)

proc printBoard(b: Board) =
    echo b.r1[0]," ",b.r1[1]," ",b.r1[2]," ",b.r1[3]," ",b.r1[4]
    echo b.r2[0]," ",b.r2[1]," ",b.r2[2]," ",b.r2[3]," ",b.r2[4]
    echo b.r3[0]," ",b.r3[1]," ",b.r3[2]," ",b.r3[3]," ",b.r3[4]
    echo b.r4[0]," ",b.r4[1]," ",b.r4[2]," ",b.r4[3]," ",b.r4[4]
    echo b.r5[0]," ",b.r5[1]," ",b.r5[2]," ",b.r5[3]," ",b.r5[4]
    echo "Rest: ",b.rest
    echo "----"

let inp = toSeq(lines "data/day4/input")

let vals = inp[0].split(',')
var numbers = initSinglyLinkedList[int]()

var boards = initSinglyLinkedList[Board]()

for i in vals:
    numbers.add(parseInt(i))

var
    line_no: int = 2

while line_no < inp.len:
    let l1 = inp[line_no]
    let l2 = inp[line_no+1]
    let l3 = inp[line_no+2]
    let l4 = inp[line_no+3]
    let l5 = inp[line_no+4]
    boards.add(createBoard(l1,l2,l3,l4,l5))
    line_no += 6

echo numbers

var won: bool = false
var pos: int = 1
for n in numbers:
    if won:
        break
    for b in boards:
        markNumberIfOnboard(b,n)        
        if checkIfWinning(b) and not b.won:
            echo "Board won as with value ", sumOfUnmarked(b) * n, " when called with num ",n
            #printBoard(b)
            b.won = true
            pos += 1            
