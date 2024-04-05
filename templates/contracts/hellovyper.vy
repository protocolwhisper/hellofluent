#pragma line
# @version ^0.2.0

#Contract Hello World

#Storage variables
greeting: String[79]
friend: String[20]
greeting_friend: public(String[100])

#Constructor
@external
def __init__(_greeting: String[79], _friend: String[20]):
    self.greeting = _greeting
    self.friend = _friend
    self.greeting_friend = concat(self.greeting, " ", self.friend)

@external
@payable
def set_greeting(_greeting: String[79]):
    assert msg.value > 1 * 10**18, "You have to pay at least 1 ether for this greeting change"
    self.greeting = _greeting
    self.greeting_friend = concat(self.greeting, " ", self.friend)

@external
def set_friend(_friend: String[20]):
    self.friend = _friend
    self.greeting_friend = concat(self.greeting, " ", self.friend)