package main

import (
	"fmt"
	"slices"

	"github.com/izoslav/aoc2024/utils"
)

type Sector struct {
	id  int
	len int
	idx int
}

func calculateLengths(encodedDisk []int) (int, int) {
	totalLen := 0
	packedLen := 0
	for i, length := range encodedDisk {
		totalLen += length

		if i%2 == 0 {
			packedLen += length
		}
	}

	return totalLen, packedLen
}

func decodeDisk(encodedDisk []int) []int {
	totalLen, _ := calculateLengths(encodedDisk)

	id := 0
	idx := 0
	decodedDisk := make([]int, totalLen)
	for i, length := range encodedDisk {
		for j := 0; j < length; j++ {
			if i%2 == 0 {
				decodedDisk[idx] = id
			} else {
				decodedDisk[idx] = -1
			}
			idx++
		}

		if i%2 == 0 {
			id++
		}
	}

	return decodedDisk
}

func packBlocks(decodedDisk []int, packedLength int) []int {
	packedDisk := []int{}
	packedDisk = append(packedDisk, decodedDisk[:packedLength]...)

	backIdx := len(decodedDisk) - 1
	for i := range packedDisk {
		if packedDisk[i] != -1 {
			continue
		}
		for decodedDisk[backIdx] == -1 {
			backIdx--
		}
		packedDisk[i] = decodedDisk[backIdx]
		backIdx--
	}

	return packedDisk
}

func calculateBlanks(files []Sector) []Sector {
	blanks := []Sector{}
	for i := 0; i < len(files)-1; i++ {
		f1 := &files[i]
		f2 := &files[i+1]

		freeSpace := f2.idx - f1.idx - f1.len

		if freeSpace > 0 {
			blanks = append(blanks, Sector{id: -1, len: freeSpace, idx: f1.idx + f1.len})
		}
	}

	return blanks
}

func findBlank(files []Sector, minLength int, maxIdx int) *int {
	for i := 0; i < len(files)-1; i++ {
		f1 := &files[i]
		f2 := &files[i+1]

		diff := f2.idx - f1.idx - f1.len

		if diff >= minLength {
			idx := f1.idx + f1.len

			if idx < maxIdx {
				return &idx
			} else {
				return nil
			}
		}
	}

	return nil
}

func findFileId(files []Sector, id int) int {
	for i := range files {
		if files[i].id == id {
			return i
		}
	}
	return 0
}

func packFiles(encodedDisk []int) []int {
	sectorIdx := 0
	files := make([]Sector, len(encodedDisk)/2+1)
	blanks := make([]Sector, len(encodedDisk)/2)

	for i, len := range encodedDisk {
		if i%2 == 0 {
			files[i/2] = Sector{id: i / 2, len: len, idx: sectorIdx}
		} else {
			blanks[i/2] = Sector{id: -1, len: len, idx: sectorIdx}
		}

		sectorIdx += encodedDisk[i]
	}

	for i := len(files) - 1; i > 0; i-- {
		slices.SortFunc(files, func(a, b Sector) int { return a.idx - b.idx })
		fi := findFileId(files, i)
		file := &files[fi]

		if blankIdx := findBlank(files, file.len, file.idx); blankIdx != nil {
			file.idx = *blankIdx
		}
	}

	slices.SortFunc(files, func(a, b Sector) int { return a.idx - b.idx })

	last := &files[len(files)-1]
	packed := make([]int, last.idx+last.len+1)
	for _, sector := range files {
		for i := sector.idx; i < sector.idx+sector.len; i++ {
			packed[i] = sector.id
		}
	}

	return packed
}

func calculateChecksum(disk []int) int {
	checksum := 0
	for i, v := range disk {
		if v == -1 {
			continue
		}
		checksum += i * v
	}
	return checksum
}

func main() {
	// input := utils.ReadFile("day08/test.txt")
	input := utils.ReadFile("day08/input.txt")

	encodedDisk := make([]int, len(input))
	for i, c := range input {
		encodedDisk[i] = utils.Atoi(string(c))
	}

	_, packedLen := calculateLengths(encodedDisk)
	decodedDisk := decodeDisk(encodedDisk)
	blockPackedDisk := packBlocks(decodedDisk, packedLen)
	filePackedDisk := packFiles(encodedDisk)

	fmt.Println("\n=== day 08 ===")
	fmt.Println("part 1:", calculateChecksum(blockPackedDisk))
	fmt.Println("part 2:", calculateChecksum(filePackedDisk))
}
