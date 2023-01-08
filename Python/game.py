import random

print('Welcome to a Higher or Lower game! You will choose a number between 1 and 100, and the script will tell you if its higher, lower, or correct!')
tries = '0'
number = str(random.randint(1,100))
print('The number has been chosen.')

def makechoice():
    print('What is your guess?')
    global tries
    i = input("Enter number: ")
    if tries == '0':
        tries = int(1)
    if i > number:
        print('Sorry, but the number is lower than that!')
        temp = int(tries) + int(1)
        tries = temp
        makechoice()
    if i < number:
        print('Sorry, but the number is higher than that!')
        temp = int(tries) + int(1)
        tries = temp
        makechoice()
    if i == number:
        print('That is correct!')
        if tries == int(1):
            printtries ="It took you "+str(tries)+" try!"
            print(printtries)
        else:
            printtries ="It took you "+str(tries)+" tries!"
            print(printtries)

makechoice()