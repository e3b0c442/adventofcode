CC := gcc
CCFLAGS := -Wall -g -pedantic -std=c17
LDFLAGS := 

DAYNUMS := 1 2
DAYS := $(addprefix day, $(DAYNUMS))
DAY_BINS := $(addprefix bin/, $(DAYS))
DAY_OBJS := $(addprefix obj/, $(addsuffix .o, $(DAYS)))
DAY_MAINS := $(addprefix obj/main, $(addsuffix .o, $(DAYS)))
TARGETS := $(DAYS)
MAINS := $(addsuffix .o, $(TARGETS))

.PHONY: clean all advent2015 $(DAYS)

all: advent2015 $(DAYS)

advent2015: obj bin bin/advent2015

$(DAYS): % : obj bin bin/%

$(DAY_BINS): bin/% : obj/%.o obj/main%.o obj/lib.o
	$(CC) -o $@ $^ $(CCFLAGS) $(LDFLAGS)

$(DAY_MAINS): src/main.c
	$(CC) -c -o $@ $< -D DAYNUM=$(@:obj/main%.o=%) $(CCFLAGS)

bin/advent2015: obj/advent2015.o obj/lib.o $(DAY_OBJS)
	$(CC) -o $@ $^ $(CCFLAGS) $(LDFLAGS)

obj/advent2015.o: src/main.c
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
