package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)



func main() {
	raw_contents, err := os.ReadFile("./input")
	check(err)
	contents := string(raw_contents)
	first_task(contents)
	second_task(contents)
		
	
}
func first_task(contents string) {
	scanner := bufio.NewScanner(strings.NewReader(contents))
	fully_overlapping_ranges := 0
	for scanner.Scan() {
		one_pair_ranges := scanner.Text() 
		raw_left_range, raw_right_range, _:= strings.Cut(one_pair_ranges, ",")
		left_low, left_high := create_range_tuple(raw_left_range)
		right_low, right_high := create_range_tuple(raw_right_range)
		if left_low <= right_low && left_high >= right_high {
			fully_overlapping_ranges +=1
		} else if right_low <= left_low && right_high >= left_high {
			fully_overlapping_ranges +=1
		}
	}
	fmt.Printf("Overlapping ranges number = %v\n", fully_overlapping_ranges)
}
func second_task(contents string) {
	scanner := bufio.NewScanner(strings.NewReader(contents))
	slightly_overlapping_ranges := 0
	for scanner.Scan() {
		one_pair_ranges := scanner.Text() 
		raw_left_range, raw_right_range, _:= strings.Cut(one_pair_ranges, ",")
		left_low, left_high := create_range_tuple(raw_left_range)
		right_low, right_high := create_range_tuple(raw_right_range)
		if !(left_low > right_high || right_low > left_high) {
			slightly_overlapping_ranges +=1
		}
	}
	fmt.Printf("Overlapping ranges number = %v\n", slightly_overlapping_ranges)
}
func create_range_tuple(raw_range string) (int, int) {
	raw_low, raw_high, _:= strings.Cut(raw_range, "-")
	low, _ := strconv.Atoi(raw_low)
	high, _ := strconv.Atoi(raw_high)
	return low,high
}

func check(e error) {
	if e != nil {
		panic(e)
	}

}

