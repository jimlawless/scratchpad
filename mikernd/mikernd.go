package main

import (
	"fmt"
)

type PRNG func() uint32
var i uint32

func starter() uint32 {
    return i
}

func (p PRNG) Roll(n, d int) (result int) {
    for ; n > 0; n-- {
        result += int(p() % uint32(d))
    }
    return result
}

func main() {
    for i=1;i<22;i++ {
        x:=PRNG(starter)
        fmt.Printf("%4v %4v\n",x.Roll(2,6),x.Roll(3,6))
    }
}
