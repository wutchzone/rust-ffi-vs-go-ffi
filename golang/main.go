package main

import (
	"os"
	"unsafe"
)

/*
#cgo LDFLAGS: -lavformat -lavutil -lavcodec
#include "vertigo.h"
#include <stdlib.h>
#include <string.h>
*/
import "C"

//export go_read
func go_read(opaque unsafe.Pointer, buf unsafe.Pointer, p1 C.int) C.int {
	foo := (*Test)(opaque)
	return (C.int)(foo.read(buf, int(p1)))
}

//export go_write
func go_write(opaque unsafe.Pointer, buf unsafe.Pointer, p1 C.int) C.int {
	foo := (*Test)(opaque)
	return (C.int)(foo.write(buf, int(p1)))
}

var (
	file       *os.File
	outputFile *os.File
)

type Test struct {
}

func (s *Test) read(buf unsafe.Pointer, size int) int {
	tmp := make([]byte, size)
	n, _ := file.Read(tmp)
	C.memcpy(buf, (unsafe.Pointer)(&tmp[0]), (C.ulong)(n))
	return n
}

func (s *Test) write(buf unsafe.Pointer, size int) int {
	outputFile.Write(C.GoBytes(buf, (C.int)(size)))
	return size
}

func main() {
	data := &Test{}
	for i := 0; i < 400; i++ {
		if f, err := os.Open(os.Args[1]); err != nil {
			panic(err)
		} else {
			file = f
		}

		outputFile, _ = os.Create("trololo.ts")
		C.go_transmuxer(unsafe.Pointer(&data))
        outputFile.Close()
        file.Close()
	}
}
