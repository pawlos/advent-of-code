import sequtils
import std/options
import std/deques
import std/algorithm

let inp = toSeq(lines "data/day10/input")

proc isMatchingClose(opening_c:char,closing_c:char):bool =
    if opening_c == '(' and closing_c == ')':
        return true
    if opening_c == '[' and closing_c == ']':
        return true
    if opening_c == '<' and closing_c == '>':
        return true
    if opening_c == '{' and closing_c == '}':
        return true

proc findCharVal(ch:char):int =
    if ch == '(':
        return 1
    elif ch == '[':
        return 2
    elif ch == '{':
        return 3
    elif ch == '<':
        return 4

    return -1

proc findInvalidChar(l: string):char =
    var starts = initDeque[char]()
    for c in l:
        if c in {'(','[','<','{'}:
            starts.addLast(c)            
        elif c in {')',']','>','}'}:
            var op = starts.popLast
            if not isMatchingClose(op,c):
                return c
    return '\x00'

proc findMissingCharsValue(l: string):int =
    #echo l
    var starts = initDeque[char]()
    for c in l:
        if c in {'(','[','<','{'}:
            starts.addLast(c)            
        elif c in {')',']','>','}'}:
            var op = starts.popLast
            if not isMatchingClose(op,c):
                return -1
                
    #incomplete?
    #echo starts
    while len(starts) > 0:
        result *= 5
        result += findCharVal(starts.popLast)
    #echo "Result: ", result
    return


var 
    sum1: int
    sum2: int
    vals: seq[int]

sum1 = 0
sum2 = 0

for l in inp:
    var wrong = findInvalidChar(l)
    if wrong == '}':
        sum1 += 1197
    elif wrong == '>':
        sum1 += 25137
    elif wrong == ']':
        sum1 += 57
    elif wrong == ')':
        sum1 += 3
    
    if wrong == '\x00':
        var v = findMissingCharsValue(l)
        vals.add(v)

sort(vals, system.cmp)
echo "Part1: ",sum1
echo "Part2: ",vals[(int)(len(vals)/2)]
    #echo "---"