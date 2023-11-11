
const TEST_01_IN: &str  = "1..|.6.|...|.7.|...|9..|4..|...|.7.|.58|64.|...|.21|3.8|..4|3.4|.72|...|.4.|537|...|..3|.26|.4.|...|9..|8.2"; 
const TEST_01_OUT: &str = "139|765|428|276|483|951|485|291|376|758|649|213|621|358|794|394|172|685|842|537|169|913|826|547|567|914|832";


fn main() { 
    solve_sudoku(TEST_01_IN); 
    println! ("Done.");
}

fn solve_sudoku (clues: &str) -> &str {
    let elements = resolve_elements(clues).unwrap();
    
    #[cfg(debug_assertions)]
    for(i, element) in elements.iter().enumerate() { 
        println!("Index: {}, Value: {}, Row: {}, Column: {}, Sector: {}, Clue: {}", 
        i, element.value, element.row, element.column, element.sector, element.clue); 
    }

    iterate_elements(&elements);

    // return last expression
    TEST_01_OUT
}

fn iterate_elements(elements: &[Element; 81]) {
    let mut count: u32 = 0;
    for(_, _element) in elements.iter().enumerate() {
        count += 1;
    }
    println!("{count}");
}

fn resolve_elements(clues: &str) -> Option<[Element; 81]> {

    let mut elements: [Element; 81] = [Element {value: 0, row: 0, column: 0, sector: 0, clue: false}; 81];

    let mut index: usize = 0;
    
    for(_, mut val) in clues.chars().enumerate() {
        if index > 80 {
             println! ("Incorrect number of elements! ({} > 81)", index + 1); 
             return None;
        }
        
        if val == '.' {
            val = '0';
        }
        
        if !val.is_numeric() { 
            continue; 
        }
        
        let dig = val.to_digit(10).unwrap();
        
        let layout = get_layout(index as u32);
        
        elements[index].value = dig; 
        elements[index].row = layout.0;
        elements[index].column = layout.1;
        elements[index].sector = layout.2;
        elements[index].clue = dig> 0;
        
        index += 1;
    }
    
    Some(elements)
}

fn get_layout(index: u32) -> (u32, u32, u32) {
    let row = index / 9;
    let column = index % 9; 
    let sector;
    
    match (row, column) {
        (0..=2, 0..=2) => sector = 0, 
        (0..=2, 3..=5) => sector = 1, 
        (0..=2, 6..=8) => sector = 2, 
        (3..=5, 0..=2) => sector = 3, 
        (3..=5, 3..=5) => sector = 4, 
        (3..=5, 6..=8) => sector = 5, 
        (6..=8, 0..=2) => sector = 6, 
        (6..=8, 3..=5) => sector = 7, 
        (6..=8, 6..=8) => sector = 8, 
        _ => sector = 999
    }
    
    (row, column, sector)
}

#[test]
fn test_sudoku_01() {
    assert_eq! (solve_sudoku(TEST_01_IN), TEST_01_OUT);
}

#[derive(Copy)]
#[derive (Clone)]
struct Element {
    value: u32,
    row: u32,
    column: u32,
    sector: u32,
    clue: bool
}
