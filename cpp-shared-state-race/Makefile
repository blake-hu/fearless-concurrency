# Compiler
CXX = g++

# Compiler flags
CXXFLAGS = -std=c++23 -Wall

# Source file
SRC = main.cpp

# Executable name
TARGET = main.out

# Build rule
$(TARGET): $(SRC)
	$(CXX) $(CXXFLAGS) $(SRC) -o $(TARGET)

# Clean rule
clean:
	rm -f $(TARGET)

# Test rule
test: $(TARGET)
	./$(TARGET)