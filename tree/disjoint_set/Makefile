all: main

main: main.o disjoint_set.o
	gcc -o main main.o disjoint_set.o

%.o: %.c disjoint_set.h
	gcc -c $<

clean:
	rm -f main *.o
