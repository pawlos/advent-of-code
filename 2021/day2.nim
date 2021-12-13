import sequtils
import std/options
from nre import findIter, re, captures, `[]`, RegexMatch, match
from std/strutils import parseInt

let l = toSeq(lines "data/day2/input")


var 
    depth : int = 0
    horizontal : int = 0
    m : Option[RegexMatch]

for line in l:
    m = nre.find(line, re"^forward (\d+)")
    if m.isSome:
        horizontal += parseInt(m.get().captures[0])
        continue
    m = nre.find(line, re"^down (\d+)")
    if m.isSome:
        depth += parseInt(m.get().captures[0])
    m = nre.find(line, re"^up (\d+)")
    if m.isSome:
        depth -= parseInt(m.get().captures[0])
echo "Part1: ",depth*horizontal

var
    aim : int = 0

horizontal = 0
depth = 0
for line in l:
    m = nre.find(line, re"^forward (\d+)")
    if m.isSome:
        let v = parseInt(m.get().captures[0])
        horizontal += v
        if aim != 0:
            depth += aim * v        
    m = nre.find(line, re"^down (\d+)")
    if m.isSome:
        aim += parseInt(m.get().captures[0])        
    m = nre.find(line, re"^up (\d+)")
    if m.isSome:
        aim -= parseInt(m.get().captures[0])        
    
echo "Part2: ",depth*horizontal