// TODO:

// Задача:
//
// n: 1 <= n <= 10^18
// m: 2 <= m <= 10^5
//
// Найти остаток от деления n-го числа Фибоначчи на m


// Шаги алгоритма:
//
// 1) создать вектор для чисел Фибоначчи
// 2) рассчитывать в цикле: текущее число фибоначчи % m
// 3) сопоставить значение с началом вектора:
// 3.1) если совпадает значение, но не индекс элемента - кинуть в temp-массив для поиска цикла Пизано
// 3.2) если не совпадает значение - очищаем temp-массив
// 4) после нахождения цикла Пизано - находим остаток от деления n на длину temp-массива
// 5) остаток от деления - индекс целевого числа в temp-массиве, достаем соответсвующее значение по индексу

// 0, 1, 2, 3, 4, 5, 6,  7,  8,  9, 10, 11,  12

// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144

// 0, 1, 1, 2, 0, 2, 2,  1,  0,  1,  1,  2,   0

fn find_remain(n: u64, m: u64) -> u64 {
    let mut remains: Vec<u64> = vec![0, 1];

    let start_pizano_idx = 0;
    let mut end_pizano_idx = 1;

    let mut cycle_value = 1;
    let mut cycle_counter = 0;

    let mut done = false;

    while !done {
        let previous_value = remains[remains.len() - 2];
        let last_value = remains[remains.len() - 1];

        let remain = (previous_value + last_value) % m;

        remains.push(remain);

        if n == remains.len() as u64 {
            done = true;
        }
    }

    *remains.last().unwrap()
}


fn main() {

    let a = find_remain(10, 5);

    println!("{}", a);
}
