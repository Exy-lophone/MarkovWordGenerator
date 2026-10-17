package main

import (
	"fmt"
	"math/rand/v2"
	"os"
	"slices"
	"strings"
	"unicode"
)

const PATH_WORDLIST string = "./dataset/fr/words.txt"

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	words := GetWordList(PATH_WORDLIST, string([]byte{13, 10}))
	chain, tokens := MakeMarkovChain(words, 1)
	PrintMarkovChain(chain, tokens)
	newWords := GenerateWords(chain, 10)
	for _, v := range newWords {
		fmt.Println("Generated word: " + v)
	}
	// fmt.Println("[i,o,n]")
	// ConvertTokenCountToRange(chain[[3]rune{[]rune("i")[0], []rune("o")[0], []rune("n")[0]}])
	// fmt.Printf("Number of tokens: %d\n", len(tokens))
	// ConvertChainToPercent(chain)
}

func GetWordList(path string, spliter string) []string {
	bytes, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	words := strings.Split(string(bytes), spliter)
	return words
}

func MakeMarkovChain(words []string, order int) (map[string]map[rune]int, []rune) {
	count := 0
	chain := make(map[string]map[rune]int)
	tokens := make(map[rune]struct{})

	for i := range words {
		word := strings.Repeat("#", order) + words[i] + "$"
		runes := []rune(word)
		for j := range len(runes) - order {
			s := runes[j : j+order+1]
			a := FormatKeySrting(s[0:order])
			b := unicode.ToLower(s[order])
			freq, ok := chain[a]
			if !ok {
				m := make(map[rune]int)
				m[b] = 1
				chain[a] = m
				continue
			}
			freq[b]++
			_, ok = tokens[b]
			if !ok {
				count++
				tokens[b] = struct{}{}
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

func FormatKeySrting(rs []rune) string {
	b := strings.Builder{}
	for i, r := range rs {
		if i == len(rs)-1 {
			fmt.Fprintf(&b, "%c", unicode.ToLower(r))
			continue
		}
		fmt.Fprintf(&b, "%c:", r)
	}
	return b.String()
}

func StringKeyToRunes(str string) []rune {
	substr := strings.Split(str, ":")
	rs := make([]rune, 0, len(substr))
	for _, str := range substr {
		rs = append(rs, []rune(str)[0])
	}
	return rs
}

func PrintMarkovChain(chain map[string]map[rune]int, tokens []rune) {
	builder := strings.Builder{}
	title := "Chain"
	builder.WriteString(title)
	titlelen := StringLenInRune(title)
	_, keylen := GetChainOrder(chain)
	keylen += 2

	if keylen > titlelen {
		builder.WriteString(strings.Repeat(" ", keylen-titlelen))
	}
	for i := range len(tokens) {
		fmt.Fprintf(&builder, "|  %c  ", tokens[i])
	}
	for k, v := range chain {
		fmt.Fprintf(&builder, "\n[%s]", k)
		if titlelen > keylen {
			builder.WriteString(strings.Repeat(" ", titlelen-keylen))
		}
		for i := range len(tokens) {
			fmt.Fprintf(&builder, "|%4d ", v[tokens[i]])
		}
	}
	fmt.Println(builder.String())
}

func StringLenInRune(str string) int {
	return len([]rune(str))
}

func GetChainOrder(chain map[string]map[rune]int) (int, int) {
	var order int = 0
	var keyLength int = 0
	for k := range chain {
		keyLength = len([]rune(k))
		order = len(StringKeyToRunes(k))
		break
	}
	return order, keyLength
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

func GenerateWords(chain map[string]map[rune]int, amount int) []string {
	words := make([]string, 0, amount)
	order, _ := GetChainOrder(chain)
	for i := range amount {
		words = append(words, strings.Repeat("#", order))
		for {
			runes := []rune(words[i])
			s := []rune(runes[len(runes)-order:])
			ranges, sum := ConvertTokenCountToRange(chain[FormatKeySrting(s)])
			if sum == 0 {
				words[i] += "$"
				break
			}
			rnd := rand.IntN(sum)
			for k, v := range ranges {
				if rnd >= v[0] && rnd <= v[1] {
					words[i] += fmt.Sprintf("%c", k)
					break
				}
			}
			if strings.Contains(words[i], "$") {
				words[i] = UnencapsulateWord(words[i])
				break
			}
		}
	}
	return words
}

func UnencapsulateWord(str string) string {
	str = strings.Trim(str, "#")
	str = strings.Trim(str, "$")
	return str
}
