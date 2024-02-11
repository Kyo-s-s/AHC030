submit:
	python bandle.py
	cat bandle/src/main.rs | xclip -selection clipboard

run:
	cd main && cargo build --release
	python run.py 10

all:
	cd main && cargo build --release
	python run.py 50

test:
	python bandle.py
	cd bandle && cargo build --release
	./tools/target/release/tester ./bandle/target/release/bandle < ./tools/in/0000.txt > ./tools/out/0000.txt
