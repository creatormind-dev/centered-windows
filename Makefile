# Compiler and flags
CC = gcc
RC = windres
CFLAGS = -Wall -Wextra -DUNICODE -D_UNICODE -D_DEBUG

# Linker flags
LDFLAGS = -I$(INC_DIR) -L$(LIB_DIR)

# Directories
SRC_DIR = src
INC_DIR = include
BIN_DIR = bin
LIB_DIR = lib

# Source files and object files
SOURCES = $(wildcard $(SRC_DIR)/*.c)
HEADERS = $(wildcard $(INC_DIR)/*.h)
LIBS = $(wildcard $(LIB_DIR)/*.lib)
OBJECTS = $(patsubst $(SRC_DIR)/%.c, $(BIN_DIR)/%.o, $(SOURCES))
RESOURCE = resources.rc
RESOURCE_OBJ = $(BIN_DIR)/resources.o

# Executable
EXECUTABLE := Centered Windows

# Build rule
$(EXECUTABLE): $(OBJECTS) $(RESOURCE_OBJ)
	$(CC) $(CFLAGS) $(LDFLAGS) -o "$@" $^ $(LIBS)

# Object files rule
$(BIN_DIR)/%.o: $(SRC_DIR)/%.c $(HEADERS)
	$(CC) $(CFLAGS) $(LDFLAGS) -c -o $@ $<

# Resource file rule
$(RESOURCE_OBJ): $(RESOURCE)
	$(RC) -i $< -o $@


.PHONY: clean

# Clean rule
clean:
	rm -f $(BIN_DIR)/*.o "$(EXECUTABLE)" $(RESOURCE_OBJ)
