CC=gcc
CFLAGS=-Wall -pedantic -g3 -O3
LIBS=-lavformat -lavutil -lavcodec
OBJS=test.o

all: test

test:${OBJS}
	${CC} ${CFLAGS} -o $@ ${OBJS} ${LIBS}

.PHONY: clean
clean:
	@rm -r *.o core test test.ts

.c.o:
	${CC} ${CFLAGS} -c $<

