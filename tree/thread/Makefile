all: main

main: main.o thread.o
	gcc -o main main.o thread.o

wrong: main.o

%.o: %.c thread.h
	gcc -c $< -o $@


clean:
	rm -f *.o main
