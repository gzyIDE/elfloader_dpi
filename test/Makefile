all: libelfloader_dpi.so main sim

libelfloader_dpi.so:
	cd ..
	cargo build
	cd -
	cp ../target/debug/libelfloader_dpi.so .


main: main.c
	riscv32-unknown-elf-gcc -static -o main main.c

sim: 
	verilator \
		ram_model_dpi.sv \
		-cc -exe ram_model_test.cpp \
		--exe ${CURDIR}/libelfloader_dpi.so
	make -C ./obj_dir -f Vram_model_dpi.mk
	./obj_dir/Vram_model_dpi

clean:
	rm -f main
	rm -rf obj_dir
	rm libelfloader_dpi.so
