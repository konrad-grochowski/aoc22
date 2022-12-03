package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	raw_contents, err := os.ReadFile("./input")
	check(err)
	contents := string(raw_contents)
	scanner := bufio.NewScanner(strings.NewReader(contents))
	var all_elves_calories []int
	current := 0
	for scanner.Scan() {

		if line := scanner.Text(); line == "" {
			all_elves_calories = append(all_elves_calories, current)
			current = 0
		} else {
			single_item_calories, err := strconv.Atoi(line)
			check(err)
			current += single_item_calories
		}
	}
	first_task(all_elves_calories)
	second_task(all_elves_calories)
}

func check(e error) {
	if e != nil {
		panic(e)
	}

}

func first_task(all_elves_calories []int) {
	max := all_elves_calories[0]
	for _, element :=  range all_elves_calories {
		if element > max  {
			max = element
		}
	}
	fmt.Printf("%v\n", max)
}

func second_task(all_elves_calories []int) {


	sort.Ints(all_elves_calories)
	three_most_caloric_elves := all_elves_calories[len(all_elves_calories)-3:]
	sum :=0
	for _,x := range three_most_caloric_elves {
		sum+=x
	}
	fmt.Printf("%v\n", sum)
}