build:
	cd lib && gfortran -shared -fPIC -o libmath.dylib math.f90
	cargo build

run:
	cd lib && source set_env.sh && cd .. && cargo run
