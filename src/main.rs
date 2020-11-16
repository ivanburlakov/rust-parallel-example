// імпортуємо потрібні утиліти та бібліотеку для математичних обчислень
use std::io;
use std::io::prelude::*;
use std::thread;
use ndarray::arr2;

// задаємо функцію для паузи терміналу
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    
    write!(stdout, "\nExit the script with ENTER").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

// задаємо головну функцію
fn main() {
    // запускаємо треди, яки роблять обчислення, складаємо треди в змінні
    let f1 = thread::spawn(|| {
        let mo = arr2(&[
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1]
        ]);

        let me = arr2(&[
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1]
        ]);

        let a = arr2(&[
            [1, 1],
            [1, 1],
            [1, 1],
            [1, 1]
        ]);

        let b = arr2(&[
            [1, 1],
            [1, 1],
            [1, 1],
            [1, 1]
        ]);

        let first_move = mo.dot(&me);
        let second_move = first_move.dot(&b);
        let third_move = &a + &second_move;

        return third_move
    });

    let f2 = thread::spawn(|| {
        let mk = arr2(&[
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1]
        ]);

        let ml = arr2(&[
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1]
        ]);

        let mg = arr2(&[
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1]
        ]);

        let first_move = mk.dot(&ml);
        let second_move = first_move.dot(&mg);
        let third_move = &second_move - &mk;

        return third_move
    });

    let f3 = thread::spawn(|| {
        let mp = arr2(&[
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1]
        ]);

        let mr = arr2(&[
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1]
        ]);

        let t = arr2(&[
            [1, 1],
            [1, 1],
            [1, 1],
            [1, 1]
        ]);

        let first_move = mp.dot(&mr);
        let second_move = first_move.t();
        let third_move = second_move.dot(&t);

        return third_move
    });

    // робимо join для кожного треда та складаємо результати в змінні
    let result_f1 = f1.join().unwrap();
    let result_f2 = f2.join().unwrap();
    let result_f3 = f3.join().unwrap();

    // прінтуємо результати у термінал
    println!("F1 (C):");
    println!("{}", result_f1);

    println!("F2 (MF):");
    println!("{}", result_f2);

    println!("F3 (O):");
    println!("{}", result_f3);

    // ставимо термінал на паузу
    pause();
}
