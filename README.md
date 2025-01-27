# A collection of little projects created to deepen my understanding of Rust's semantics and rules
I made several projects in Rust, to understand it's rules. These projects do not represent any common usage, but mine. 

### Projects: 

# Tic-Tac-Toe
This program allows you to play Tic-Tac-Toe game in the console. 
You can play in two modes:
- two players on one keyboard.
- one player versus random moves.

Things to improve:
By using "Tokio" library I could add online support. 

# TemperatureConverter
Converts from Celcius to Fahrenheit and vice-versa.
- By working on it, I understood how numeric data types work.

# ATM-simulator: 
I wanted to recreate the usage of ATM machines. This project is able to:
- Add users to database
- Transfer money from one card to another.
  
By creating the project, I ran into the couple of problems:
1. Borrow checker. Often, the compilation would give errors. The closer I was to completing my project, the easier was it for  to foresee the following errors. The way of solving them also became much clearer.
2. Libraries. Firstly, I didn't grasp what library should I use for my purposes and used wrong ones, such as serde_json. Later I figured out what I needed - rusqlite. By working with this library, not only I made my project work in a desired way, but also improved my understanding of SQL.
