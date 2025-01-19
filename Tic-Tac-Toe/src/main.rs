use rand::Rng;


fn main() {
    let mut board: Vec<char> = vec!['-', '-', '-', // доска, которая потом меняет свои внутренности
                                    '-', '-', '-', 
                                    '-', '-', '-'];

    let moves = ["A1", "B1", "C1",  // все возможные ходы, используются для последующего индексирования
                            "A2", "B2", "C2", 
                            "A3", "B3", "C3"];

    let win_combinations = [ // все возможные комбинации побед, используются для проверки на победу
        [0, 1, 2],  
        [3, 4, 5],  
        [6, 7, 8],  
        [0, 3, 6],  
        [1, 4, 7],  
        [2, 5, 8],  
        [0, 4, 8],  
        [2, 4, 6],  
    ];
    
    
    let mut game_mode_input = String::new();
    println!("Enter the desired gamemode: 1-on one keyboard; 2-second player is random;");
    std::io::stdin().read_line(&mut game_mode_input).unwrap();

    match game_mode_input.trim().parse() {
        Ok(game_mode) if game_mode == 1 || game_mode == 2 => {
            match game_mode {
                1 => {
                    play_on_one_keyboard(&mut board, moves, &win_combinations);
                    println!("You decided: play on one keyboard!");
                }

                2 => {
                    play_with_random(&mut board, moves, &win_combinations);
                    println!("You decided: play with random!");
                }
                    
                _ => unreachable!(), // Это условие никогда не должно выполниться благодаря проверке выше
            }
        },
        _ => {
            println!("Invalid input! Please enter 1 or 2."); // Сообщение об ошибке
        }
    }
}   


fn cord_to_index(cord: &str, moves: &[&str]) -> Option<usize> {
    moves.iter().position(|&shag| shag.to_lowercase() == cord) // извлечение индекса из коордиант
}

fn display_board(board: &Vec<char>) {
    println!("   A   B   C");
    println!("");
    for i in 0..9 {
        if (i==0) || (i==3) || (i==6) { // вывод нумерации строк
            print!("{}", i/3 + 1);
        }

        print!("  {} ", board[i]); // вывод крестиков-ноликов

        if (i==2) || (i==5) || (i==8) { // разделитель ВЕРТИКАЛЬНЫЙ
            print!("\n");
            println!("  --------------");
        }
        
        if (i!=2) && (i!=5) && (i!=8) { // разделитель горизонтальный
            print!("|");
        }
    }
}

fn play_on_one_keyboard(board: &mut Vec<char>, moves: [&str; 9], win_combinations: &[[usize; 3]; 8]) {
    let mut move_counter: u8 = 0;
    let mut first_display = true;  // просто on/off для первого показа доски, чтобы часто не мигала
    loop {
        if first_display { // этот блок if как раз работает для первого показа доски
            display_board(&board);
            first_display = false;
        }

        let mut cord = String::new();
        
        println!("Enter your move: ");
        std::io::stdin().read_line(&mut cord).expect("msg1");

        if moves.iter().any(|&shag| shag.to_lowercase() == cord.trim().to_lowercase()) { // извлечение хода введенным пользователем

            let index = cord_to_index(&cord.trim().to_lowercase(), &moves);

            if let Some(index) = index { // если такой индекс существует, то ->
                if board[index] == 'X' || board[index] == 'O' {
                    println!("Coordinate is already taken!");
                } else {
                    if move_counter % 2 == 0 { // -> решается ставиться ли X ->
                        board[index] = 'X';
                        println!("Now it's O turn!");
                    } else {                   // -> или О
                        board[index] = 'O';
                        println!("Now it's X turn!");
                    }
                    move_counter += 1;
                }
                display_board(&board);
            }
            for combo in win_combinations { // цикл проверяет на победу
                let [a, b, c] = combo;
                if board[*a] == board[*b] && board[*b] == board[*c] && board[*a] != '-' {
                    println!("Player {} WON!", board[*a]);
                    return;
                } else if !board.contains(&'-') {
                    println!("DRAW!");
                    return;
                } 
            }
        } else {
            println!("seems there's no move like this. Maybe try to change your keyboard to ENG");
        }
    }
}

fn play_with_random(board: &mut Vec<char>, moves: [&str; 9], win_combinations: &[[usize; 3]; 8]) {
    let mut move_counter: u8 = 0;
    let mut first_display = true;  // просто on/off для первого показа доски, чтобы часто не мигала
    let mut rng = rand::thread_rng();
    loop {

        if first_display { // этот блок if как раз работает для первого показа доски
            display_board(&board);
            first_display = false;
        }

        if move_counter % 2 == 0 {
            let mut cord = String::new();
            println!("Enter your move: ");
            std::io::stdin().read_line(&mut cord).expect("msg1");

            if moves.iter().any(|&shag| shag.to_lowercase() == cord.trim().to_lowercase()) {
                let index = cord_to_index(&cord.trim().to_lowercase(), &moves);
                if let Some(index) = index { 
                    if board[index] == 'X' || board[index] == 'O' {
                        println!("Coordinate is already taken!");
                    } else {
                        board[index] = 'X';
                        move_counter += 1;
                    }
                }
            } else {
                println!("seems there's no coordinate like this");
            }
        } else {
            loop {
                let index = rng.gen_range(0..9);
                if board[index] == '-' {
                    board[index] = 'O';
                    move_counter += 1;
                    println!("Now it's X turn!");
                    break;
                }
            }
        }
        display_board(&board);

        // Проверка на победу или ничью
        for combo in win_combinations {
            let [a, b, c] = combo;
            if board[*a] == board[*b] && board[*b] == board[*c] && board[*a] != '-' {
                println!("Player {} WON!", board[*a]);
                return;
            } else if !board.contains(&'-') {
                println!("DRAW!");
                return;
            }
        }
    } 
}