# Compiler and flags
CC = gcc
RC = windres
CFLAGS = -Wall -Iinclude

# Directories
SRC_DIR = src
INC_DIR = include
BIN_DIR = bin

# Source files and object files
SOURCES = $(wildcard $(SRC_DIR)/*.c)
HEADERS = $(wildcard $(INC_DIR)/*.h)
OBJECTS = $(patsubst $(SRC_DIR)/%.c, $(BIN_DIR)/%.o, $(SOURCES))
RESOURCE = resources.rc
RESOURCE_OBJ = $(BIN_DIR)/resources.o

# Executable
EXECUTABLE := Centered Windows

# Build rule
$(BIN_DIR)/$(EXECUTABLE): $(OBJECTS) $(RESOURCE_OBJ)
	$(CC) $(CFLAGS) -o "$(BIN_DIR)/$(EXECUTABLE)" $^

# Object files rule
$(BIN_DIR)/%.o: $(SRC_DIR)/%.c $(HEADERS)
	$(CC) $(CFLAGS) -c -o $@ $<

# Resource file rule
$(RESOURCE_OBJ): $(RESOURCE)
	$(RC) -i $< -o $@

# Clean rule
clean:
	rm -f $(BIN_DIR)/*.o "$(BIN_DIR)/$(EXECUTABLE)" $(RESOURCE_OBJ)
