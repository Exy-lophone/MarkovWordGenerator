package main

import (
	"fmt"
	"math/rand/v2"
	"os"
	"slices"
	"strings"
	"unicode"
)

const PATH_WORDLIST string = "./data/wordlist-fr-formated.txt"

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	// fmt.Printf("Random number: %d", rand.IntN(100))

	words, err := GetWordList(PATH_WORDLIST, "\n")
	check(err)
	chain, _ := MakeThirdOrderMarkovChain(words)
	newWords := GenerateWords(chain, 10)
	for _, v := range newWords {
		fmt.Println("Generated word: " + v)
	}
	// fmt.Println("[i,o,n]")
	// ConvertTokenCountToRange(chain[[3]rune{[]rune("i")[0], []rune("o")[0], []rune("n")[0]}])
	// fmt.Printf("Number of tokens: %d\n", len(tokens))
	// ConvertChainToPercent(chain)
	// PrintMarkovChain(chain, tokens)
}

func GetWordList(path string, spliter string) ([]string, error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return []string{}, err
	}
	words := strings.Split(string(bytes), spliter)
	return words, nil
}

func MakeThirdOrderMarkovChain(words []string) (map[[3]rune]map[rune]int, []rune) {
	const order int = 3
	count := 0
	chain := make(map[[3]rune]map[rune]int)
	tokens := make(map[rune]struct{})
	for i := range len(words) {
		runes := []rune(words[i])
		if len(runes) < order {
			fmt.Printf("Word too shord ! (%s)\n", words[i])
			continue
		}
		for j := range len(runes) - order {
			s := runes[j : j+order+1]
			stateA := [3]rune{unicode.ToLower(s[0]), unicode.ToLower(s[1]), unicode.ToLower(s[2])}
			stateB := unicode.ToLower(s[3])
			freq, ok := chain[stateA]
			if !ok {
				m := make(map[rune]int)
				m[stateB] = 1
				chain[stateA] = m
				continue
			}
			freq[stateB]++
			_, exist := tokens[stateB]
			if !exist {
				count++
				tokens[stateB] = struct{}{}
			}
		}
	}
	s := make([]rune, 0, count)
	for k, _ := range tokens {
		s = append(s, k)
	}
	slices.Sort(s)
	return chain, s
}

func PrintMarkovChain(chain map[[3]rune]map[rune]int, tokens []rune) {
	builder := strings.Builder{}
	builder.WriteString("Matrix ")
	for i := range len(tokens) {
		fmt.Fprintf(&builder, "|  %c  ", tokens[i])
	}
	for k, v := range chain {
		fmt.Fprintf(&builder, "\n[%c,%c,%c]", k[0], k[1], k[2])
		for i := range len(tokens) {
			fmt.Fprintf(&builder, "|%4d ", v[tokens[i]])
		}
	}
	fmt.Println(builder.String())
}

func ConvertTokenCountToRange(t map[rune]int) (map[rune][2]int, int) {
	m := make(map[rune][2]int)
	c := -1
	s := 0
	for k, v := range t {
		m[k] = [2]int{c + 1, c + v}
		c = m[k][1]
		s += v
	}
	return m, s
}

func GenerateWords(chain map[[3]rune]map[rune]int, amount int) []string {
	words := make([]string, 0, amount)
	for i := range amount {
		words = append(words, "###")
		for {
			runes := []rune(words[i])
			lastRunes := [3]rune(runes[len(runes)-3:])
			ranges, sum := ConvertTokenCountToRange(chain[lastRunes])
			rnd := rand.IntN(sum)
			for k, v := range ranges {
				if rnd >= v[0] && rnd <= v[1] {
					words[i] += fmt.Sprintf("%c", k)
					break
				}
			}
			if strings.Contains(words[i], "$") {
				break
			}
		}
	}
	return words
}
