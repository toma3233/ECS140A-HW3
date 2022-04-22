class SimpleParser:
    def __init__(self, s):
        self.input = s
        self.char_pos = 0

    def fun_s(self):
        try:
            if len(self.input) == 0:
                raise Exception("Syntax error at character position " + str(self.char_pos)) 
            letter = self.input[self.char_pos]
            if letter == 'a':
                while letter == 'a' and self.char_pos < len(self.input):
                    letter = self.input[self.char_pos]
                    self.char_pos += 1
                self.char_pos -= 1
                # input string consists entirely of 'a's
                if letter == 'a':
                    raise Exception("Syntax error at character position " + str(self.char_pos)) 
                else:
                    self.fun_x()
            elif letter == 'b':
                # only 1 b
                if self.char_pos == len(self.input) - 1:
                    raise Exception("Syntax error at character position " + str(self.char_pos)) 
                else:
                    self.char_pos += 1
                    self.fun_x()
            else:
                self.fun_x()
        except Exception as error:
            print(repr(error))

    def fun_x(self):
        if (self.input[self.char_pos] == 'c' or self.input[self.char_pos] == 'd'):
            if self.char_pos == len(self.input) - 1:
                print("Input is valid")
            else:
                # acd
                self.char_pos += 1
                raise Exception("Syntax error at character position " + str(self.char_pos)) 
        else:
            raise Exception("Syntax error at character position " + str(self.char_pos)) 

test = {
    "bc": "Input is valid", 
    "acd": "Syntax error at character position 2", 
    "aaad": "Input is valid",
    "c": "Input is valid",
    "3yz": "Syntax error at character position 0",
    "": "Syntax error at character postion 0"
}

for input in test.keys():
    sp = SimpleParser(input)
    print("Should be: " +  test[input])
    sp.fun_s()
    print()
    print()