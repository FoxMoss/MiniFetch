EXE = minifetch
FILES = $(wildcard ./*.rs)
OBJ_DIR = ./obj
OBJECTS = $(patsubst ./%.rs,$(OBJ_DIR)/%.rlib,$(FILES))
LIBS = $(patsubst $(OBJ_DIR)/%.rlib,-l%,$(OBJECTS))
FLAGS = -Anon-snake-case -g

all: $(EXE)

$(EXE): $(FILES)
	rustc main.rs -o $(EXE) $(FLAGS)

$(OBJ_DIR)/%.rlib: ./%.rs
	rustc -o $@ $< --crate-type rlib $(FLAGS)

