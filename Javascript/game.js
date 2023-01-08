function getRandomInt(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min) + min); // The maximum is exclusive and the minimum is inclusive
  };
console.log(('Welcome to a Higher or Lower game! You will choose a number between 1 and 100, and the script will tell you if its higher, lower, or correct!'))
var tries = 0
var choice
var number = String(getRandomInt(1, 100))
console.log('The number has been chosen.')

async function game(){
    let first = new Promise((resolve, reject) => {
        console.log('What is your guess?')
        const prompt = require("prompt-sync")({ sigint: true });
        const answer = prompt("Number: ");
        choice = answer
        resolve()
    })
    let second = new Promise((resolve, reject) => {
        if (tries == '0') {
            tries++;
            resolve()
        }
    })
    let third = new Promise((resolve, reject) => {
        if (choice > number) {
            console.log('Sorry, but the number is lower than that!')
            tries++;
            game()
            resolve()
        }
    })
    let fourth = new Promise((resolve, reject) => {
        if (choice < number) {
            console.log('Sorry, but the number is higher than that!')
            tries++;
            game()
            resolve()
        }
    })
    let fifth = new Promise((resolve, reject) => {
        if (choice == number) {
            console.log('That is correct!')
            if (tries == 100) {
                console.log('It took you '+tries+" try!")
                resolve()
                process.exit(200); // Means it worked
            }
            else {
                console.log('It took you '+tries+" tries!")
                resolve()
                process.exit(200); // Means it worked
            }
        }
    })
    await Promise.all([first, second, third, fourth, fifth])
}
game()
