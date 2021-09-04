all: main

main: main.o graph.o
	g++ -o main main.o graph.o

%.o: %.cpp graph.h
	g++ -c $<

clean:
	rm -f main *.o
