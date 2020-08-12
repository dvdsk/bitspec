CC=gcc
CFLAGS=-I.
DEPS = encoding.h
OBJ = encoding.o test.o 

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

test: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)
