CC=gcc
CFLAGS=-I. -g
DEPS = encoding.h
OBJ = encoding.o test.o 

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

test: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)

.PHONY: clean
clean: 
	rm -rf *.o
