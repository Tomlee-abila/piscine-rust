pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    (player == table[0][0] && player == table[1][1] && player == table[2][2]) || 
    (player == table[2][0] && player == table[1][1] && player == table[0][2])
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    (player == table[0][0] && player == table[0][1] && player == table[0][2]) ||
    (player == table[1][0] && player == table[1][1] && player == table[1][2]) ||
    (player == table[2][0] && player == table[2][1] && player == table[2][2])
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (player == table[0][0] && player == table[1][0] && player == table[2][0]) ||
    (player == table[0][1] && player == table[1][1] && player == table[2][1]) ||
    (player == table[0][2] && player == table[1][2] && player == table[2][2])
}

pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return "player X won".to_string();
    } else if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return "player O won".to_string();
    } else {
        return "tie".to_string();
    }
}
    


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];
        let horz = [['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']];
        let vert = [['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']];
        assert_eq!(diagonals('X', diag), true);
        assert_eq!(horizontal('X', horz), false);
        assert_eq!(vertical('O', vert), true);
        assert_eq!(tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']]), "tie".to_string());
        assert_eq!(tic_tac_toe(vert), "player O won".to_string());
        assert_eq!(tic_tac_toe(diag), "player X won".to_string());

    }
}
