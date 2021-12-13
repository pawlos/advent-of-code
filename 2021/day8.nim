import sequtils
import std/[tables,strutils]

let inp = toSeq(lines "data/day8/input")

var 
    cnt:int
    sum:int

proc containsAll(s:string;l:string):bool=
    for c in s:
        if not l.contains(c):
            return false

    return true

proc findIf[T](s: seq[T], el: T, pred: proc(x,y: T): bool): int =
  result = -1  # return -1 if no items satisfy the predicate
  for i, x in s:
    if pred(el, x): return i

proc eq(el, s:string):bool =
    return containsAll(el, s) and containsAll(s, el)

proc determineOutput(left,right:string):int =
    let wiring = left.split(" ")
    
    #1
    let one = wiring.filterIt(len(it) == 2)[0]
    #4
    let four = wiring.filterIt(len(it) == 4)[0]
    #7
    let seven = wiring.filterIt(len(it) == 3)[0]
    #8
    let eight = wiring.filterIt(len(it) == 7)[0]
    # 3 - 5 segments
    let three = wiring.filterIt(len(it) == 5 and containsAll(one, it))[0]
    # 9 - 6 segments
    let nine = wiring.filterIt(len(it) == 6 and containsAll(three, it))[0]
    # 0 - 6 segments
    let zero = wiring.filterIt(len(it) == 6 and containsAll(it, eight) and it != nine and containsAll(one, it))[0]
    # 6 - 6 segments
    let six = wiring.filterIt(len(it) == 6 and containsAll(it, eight) and it != nine and it != zero)[0]
    # 5 - 5 segments
    let five = wiring.filterIt(len(it) == 5  and it != three and containsAll(it,six))[0]
    # 2 - 5 segments
    let two = wiring.filterIt(len(it) == 5 and it != three and it != five)[0]
    #echo "0: ",zero,", 1: ",one,", 2:", two, ", 3: ",three,", 4: ",four
    #echo "5: ", five,", 6: ",six,", 7: ",seven,", 8: ",eight," 9: ",nine
    #echo "---"
    let dict = @[zero,one,two,three,four,five,six,seven,eight,nine]    
    
    let digits = right.split(" ")
    for d in digits:
        result += findIf(dict, d, eq)
        result *= 10

    result = (int)result/10

cnt = 0
for l in inp:
    echo l
    var content = l.split(" | ")
    var digits = content[1].split(" ")
    for d in digits:
        if len(d) == 2 or len(d) == 3 or len(d) == 4 or len(d) == 7:
            cnt += 1
    let v = determineOutput(content[0], content[1])
    echo content[1]," - ",v
    sum += v


#echo wires
echo "Part1: ",cnt
echo "Part2: ", sum