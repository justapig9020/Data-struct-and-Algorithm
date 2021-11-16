FLAG = CORRECT

ifeq ($(FLAG), WRONG)
	DEF := -DWRONG
endif

all: main

main: main.o thread.o
	gcc -o main main.o thread.o

wrong: main.o

%.o: %.c thread.h
	gcc -c $< -o $@ $(DEF)

clean:
	rm -f *.o main
