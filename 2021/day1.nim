from std/strutils import parseInt
import sequtils

let l = toSeq(lines "data/day1/input").map(parseInt)
let z = zip(l, l[1 .. len(l)-1])
let c = len(filter(z, proc (a:(int,int)) : bool = a[1] > a[0]))

echo c


let zz = zip(l, zip(l[1 .. len(l)-1], l[2 .. len(l)-1]))

let w = map(zz, proc (a:(int,(int,int))) : int = a[0] + a[1][0] + a[1][1])
let ww = zip(w, w[1 .. len(w) - 1])
let cc = len(filter(ww, proc(ww:(int,int)) : bool = ww[1] > ww[0]))

echo cc