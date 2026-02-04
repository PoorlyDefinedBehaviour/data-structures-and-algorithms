package main

import (
	"fmt"
	"math"
	"math/rand"
)

type Count struct {
	counters []uint32
	a        []uint32
	b        []uint32
	w        uint32
	d        uint32
	p        uint32
}

func NewCount(w, d, p uint32) *Count {
	counters := make([]uint32, d*w)
	a := make([]uint32, d)
	b := make([]uint32, d)

	for j := range d {
		a[j] = 1 + (uint32(rand.Int())%p - 1)
		b[j] = 1 + (uint32(rand.Int())%p - 1)
	}

	return &Count{w: w, d: d, p: p, counters: counters, a: a, b: b}
}

func (count *Count) Update(i, c uint32) {
	for j := range count.d {
		hash := ((count.a[j]*i + count.b[j]) % count.p) % count.w
		index := j*count.w + hash
		count.counters[index] += c
	}
}

func (count *Count) Estimate(i uint32) uint32 {
	e := uint32(math.MaxUint32)
	for j := range count.d {
		hash := ((count.a[j]*i + count.b[j]) % count.p) % count.w
		index := j*count.w + hash
		e = min(e, count.counters[index])
	}
	return e
}

func main() {
	count := NewCount(100_000, 10, uint32(2147483647))
	model := make(map[uint32]uint32)

	for n := range 1_000_000 {
		i := uint32(rand.Intn(100_000))
		c := uint32(rand.Intn(10))
		model[i] += c
		count.Update(i, c)
		if n%10_000 == 0 {
			fmt.Printf("aaaaaaa model[i] %+v estimate %+v\n", model[i], count.Estimate(i))
		}
	}
}
