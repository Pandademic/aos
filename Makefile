build:
	@rustup run nightly xargo bootimage

run: build
	@qemu-system-x86_64 target/x86_64-aos/debug/bootimage-aos.bin 

prebuilt:
	@qemu-system-x86_64 target/x86_64-aos/debug/bootimage-aos.bin

clean: 
	@rustup run nightly cargo clean
