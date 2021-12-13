import std/strutils
import lists
import sequtils
import bitops
let lines = toSeq(lines "data/day3/input")

var
    num: int
    cnt: int

var nums = initSinglyLinkedList[int]()
var size = len(lines[0])
for l in lines:
    num = frombin[int](l)
    nums.add(num)
    cnt += 1

var 
    gamma: int
    epsilon: int
    o2: int
    co2: int

var 
    o2list  = toSeq(nums.copy())
    co2list = toSeq(nums.copy())

for i in 0..size-1:
    let j = size - i - 1
    let c1 = countIt(nums, testBit(it, j))
    let c0 = countIt(nums, not testBit(it, j))

    if c1 >= c0:
        setBit(gamma, j)
    else:
        setBit(epsilon, j)


for i in 0..size-1:
    let j = size - i - 1
    let c1 = countIt(o2list, testBit(it, j))
    let c0 = countIt(o2list, not testBit(it, j))
    if c1 >= c0:       
        if len(o2list) > 1:
            o2list = filterIt(o2list, testBit(it,j))
    else:
        if len(o2list) > 1:
            o2list = filterIt(o2list, not testBit(it,j))
    #echo o2list

for i in 0..size-1:
    let j = size - i - 1
    let c1 = countIt(co2list, testBit(it, j))
    let c0 = countIt(co2list, not testBit(it, j))
    #echo "c1: ",c1, ", c0: ",c0
    if c1 >= c0:
        if len(co2list) > 1:
            co2list = filterIt(co2list, not testBit(it, j))
    else:
        if len(co2list) > 1:
            co2list = filterIt(co2list, testBit(it, j))
    #echo co2list

echo "Gamma: ",gamma
echo "Eps: ", epsilon
echo gamma * epsilon
#echo o2list
#echo co2list
echo o2list[0]*co2list[0]