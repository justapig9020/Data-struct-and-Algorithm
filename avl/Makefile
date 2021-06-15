all: main

main: main.o avl.o
	gcc -o main main.o avl.o

%.o: %.c avl.h
	gcc -c $< -o $@

clean:
	rm -f *.o main
