import std/strutils
import sequtils
# demo: 3,4,3,1,2
# after 18: 26
# after 80: 5934


let inp = toSeq(lines "data/day6/input")[0]

var lanternfish_array = @[0,0,0,0,0,0,0,0,0]

for l in inp.split(","):
    let idx = parseInt(l)
    lanternfish_array[idx] += 1

let days = 256

echo lanternfish_array
for day in countup(1,days):
    let from_0th_slot = lanternfish_array[0]

    lanternfish_array[0] = lanternfish_array[1]
    lanternfish_array[1] = lanternfish_array[2]
    lanternfish_array[2] = lanternfish_array[3]
    lanternfish_array[3] = lanternfish_array[4]
    lanternfish_array[4] = lanternfish_array[5]
    lanternfish_array[5] = lanternfish_array[6]
    lanternfish_array[6] = lanternfish_array[7]
    lanternfish_array[6] += from_0th_slot
    lanternfish_array[7] = lanternfish_array[8]
    lanternfish_array[8] = from_0th_slot
    #echo lanternfish_array

var
    sum: int =0

for i in countup(0, 8):
    sum += lanternfish_array[i]

echo sum