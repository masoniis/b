# Run the C code
run:
	gcc main.c -o main -L cbridge -lbcraft -Wl,-rpath,cbridge
	./main

# Clean up build artifacts
clean:
	rm -f main
