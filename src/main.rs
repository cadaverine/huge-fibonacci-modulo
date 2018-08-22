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


fn main() {
    println!("Hello, world!");
}
