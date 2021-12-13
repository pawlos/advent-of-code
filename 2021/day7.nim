import std/strutils
import sequtils
import lists
import math

proc findMax(a:SinglyLinkedList):int =
    for v in a:
        if v > result:
            result = v

proc countFuel(n:int):int=
    result = (int)(n*(n+1)/2)
    #echo "Fuel: ",n," -> ", result


let inp = toSeq(lines "data/day7/input")[0]

var
    whales = initSinglyLinkedList[int]()

for l in inp.split(","):
    whales.add(parseInt(l))

let max = findMax(whales)
let min = 0

var
    min_fuel: int
    sum: int
    min_fuel2: int
    sum2: int

min_fuel = high(int)
min_fuel2 = high(int)

for pos in countup(0,max):
    #echo "----- ",pos," ------"
    sum = 0
    sum2 = 0
    for v in whales:
        sum += abs(v-pos)
        sum2 += countFuel(abs(v-pos))

    if sum < min_fuel:
        min_fuel = sum

    if sum2 < min_fuel2:
        min_fuel2 = sum2

echo min,", ",max
echo min_fuel,", ",min_fuel2