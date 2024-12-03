SESSION ?=

ifndef SESSION
$(error "SESSION must be set")
endif

INPUT_FILES := $(patsubst src/bin/day%.rs, src/bin/%.txt, $(wildcard src/bin/*.rs))

input: $(INPUT_FILES)

%.txt:
	curl https://adventofcode.com/2024/day/$(notdir $*)/input --cookie "session=$(SESSION)" > $@
