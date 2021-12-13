import sequtils
import strutils
import system/io

type
    Point = ref object of RootObj
        x: int
        y: int


let inp = toSeq(lines "data/day13/input") 
var points: seq[Point]

proc toString(pts:seq[Point])=
    for p in pts:
        echo "{X=",p.x,", Y=",p.y,"}"

proc exists(pts:seq[Point],p:Point):bool=
    return pts.countIt(it.x == p.x and it.y == p.y) > 0

proc fold_horiz(pts:seq[Point],n:int):seq[Point]=
    for p in pts:
        if p.x > n:
            let pn = Point(x:2*n-p.x,y:p.y)
            if not exists(result, pn):
                result.add(pn)
        else:
            if not exists(result, p):
                result.add(p)

proc fold_vert(pts:seq[Point],n:int):seq[Point]=
    for p in pts:
        if p.y > n:
            let pn = Point(x:p.x,y:2*n - p.y)
            #echo p.x,",",p.y," -> ",pn.x,",",pn.y
            if not exists(result, pn):
                result.add(pn)
        else:
            if not exists(result, p):
                result.add(p)

proc fold(p:seq[Point],l:string):seq[Point] =
    if "fold along x=" in l:
        let n = parseInt(l.replace("fold along x=",""))
        result = fold_horiz(p,n)
    elif "fold along y=" in l:
        let n = parseInt(l.replace("fold along y=",""))
        result = fold_vert(p,n)

for l in inp:
    if "" == l:
        continue
    if "fold" in l:
        points = fold(points, l)
        echo len(points)
        continue

    var parts = l.split(",")
    let x = parseInt(parts[0])
    let y = parseInt(parts[1])
    points.add(Point(x:x,y:y))

let f = open("part2.txt",fmWrite)

for i in countup(0,10):
    var s = newString(40)
    let pts = points.filterIt(it.y == i)
    for i in countup(0,39):
        s[i] = ' '
    for p in pts:
        s[p.x] = '#'

    f.writeLine(s)