packages-installed.txt: parser.py
	#pip install --user ply

test: packages-installed.txt
	python3 -m unittest parser.py

.PHONY: build

build: packages-installed.txt

.PHONY: run

run: packages-installed.txt
	python3 parser.py $F

.PHONY: clean

clean:
