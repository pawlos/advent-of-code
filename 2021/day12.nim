import sequtils
import strutils

type
    CaveConnection = ref object of RootObj
        cave_from: string
        cave_to: string 

let inp = toSeq(lines "data/day12/input")

var connections: seq[CaveConnection]

proc toString(cc: CaveConnection): string =
    "from " & cc.cave_from & " to " & cc.cave_to

proc isSmallCave(l:string):bool =
    l.allIt(isLowerAscii(it))

proc isLargeCave(l:string):bool =
    not isSmallCave(l)

proc canVisitCave(c,l:string):bool =
    if c == "start":
        return false
    if c == "end" and l.contains("end"):
        return false
    if isLargeCave(c):
        return true
    if isSmallCave(c):
        let ss = c
        let parts = l.split("-")
        let cnt = parts.countIt(it == ss)
        if cnt > 2:
            return false
        else:
            for p in parts:
                let cc = parts.countIt(it == p)
                if cc >= 2 and isSmallCave(p) and cnt >= 1:
                    return false
            return true

proc findRoute(l:string):int =
    if l.endsWith("end"):
        return 1
    var parts = l.split("-")
    var last_segment = parts[len(parts)-1]
    var conns = connections.filterIt(it.cave_from == last_segment or it.cave_to == last_segment)
    var cnt = 0
    for c in conns:
        if c.cave_from == last_segment:
            if isLargeCave(c.cave_to) or not l.contains(c.cave_to&"-"): 
                cnt += findRoute(l&"-"&c.cave_to)
        elif c.cave_to == last_segment:
            if isLargeCave(c.cave_from) or not l.contains(c.cave_from&"-"):
                cnt += findRoute(l&"-"&c.cave_from)
    return cnt

proc findRoute2(l:string):int =
    if l.endsWith("end"):
        return 1
    var parts = l.split("-")
    var last_segment = parts[len(parts)-1]
    var conns = connections.filterIt(it.cave_from == last_segment or it.cave_to == last_segment)
    var cnt = 0
    for c in conns:
        if c.cave_from == last_segment:
            if canVisitCave(c.cave_to, l): 
                cnt += findRoute2(l&"-"&c.cave_to)
        elif c.cave_to == last_segment:
            if canVisitCave(c.cave_from, l):
                cnt += findRoute2(l&"-"&c.cave_from)
    return cnt

for l in inp:
    var parts = l.split("-")
    var cc = CaveConnection(cave_from: parts[0], cave_to: parts[1])
    connections.add(cc)

var starts = connections.filterIt(it.cave_from == "start" or it.cave_to == "start")
var count1 = 0
var count2 = 0
for s in starts:
    var r = "start-"
    if s.cave_from == "start":
        r &= s.cave_to
    elif s.cave_to == "start":
        r &= s.cave_from
    count1 += findRoute(r)
echo "Part1: ", count1

for s in starts:
    var r = "start-"
    if s.cave_from == "start":
        r &= s.cave_to
    elif s.cave_to == "start":
        r &= s.cave_from
    count2 += findRoute2(r)

echo "Part2: ", count2
#echo connections
 