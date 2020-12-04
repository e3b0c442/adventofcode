CC := gcc
CCFLAGS := -Wall -g -pedantic -std=c17
LDFLAGS := -lpcre

DAYNUMS := 1 2 3 4
DAYS := $(addprefix day, $(DAYNUMS))
DAY_BINS := $(addprefix bin/, $(DAYS))
DAY_OBJS := $(addprefix obj/, $(addsuffix .o, $(DAYS)))
DAY_MAINS := $(addprefix obj/main, $(addsuffix .o, $(DAYS)))
TARGETS := $(DAYS)
MAINS := $(addsuffix .o, $(TARGETS))

.PHONY: clean all advent2020 $(DAYS) 

all: advent2020 $(DAYS)

advent2020: obj bin bin/advent2020

$(DAYS): % : obj bin bin/%

$(DAY_BINS): bin/% : obj/%.o obj/main%.o obj/lib.o
	$(CC) -o $@ $^ $(CCFLAGS) $(LDFLAGS)

$(DAY_MAINS): src/main.c
	$(CC) -c -o $@ $< -D DAYNUM=$(@:obj/main%.o=%) $(CCFLAGS)

bin/advent2020: obj/advent2020.o obj/lib.o $(DAY_OBJS)
	$(CC) -o $@ $^ $(CCFLAGS) $(LDFLAGS)

obj/advent2020.o: src/main.c
	$(CC) -c -o $@ $< $(CCFLAGS)

$(DAY_OBJS): obj/%.o : src/%.c 
	$(CC) -c -o $@ $< $(CCFLAGS)

obj/lib.o: src/lib.c
	$(CC) -c -o $@ $< $(CCFLAGS)

obj:
	mkdir -p obj

bin:
	mkdir -p bin

clean:
	rm -rf obj
	rm -rf bin%                                                             