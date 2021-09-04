all: main

main: main.o sort.o vector.o
	gcc -o main main.o sort.o vector.o

main.o: main.c sort.h
	gcc -c main.c

sort.o: sort.c sort.h
	gcc -c sort.c

vector.o: vector.c vector.h
	gcc -c vector.c

clean:
	rm -f *.o main
