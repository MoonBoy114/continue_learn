




fn main() {
//     let x = 4;
//     let y = 5;
//     let sum = x + y;
//     let multi = x * y;
//     let divide = x / y;
//     let diff = x - y;
//     let ostatok = x % y;
//     println!("{}\n{}\n{}\n{}\n{}", sum, diff, multi, divide, ostatok);
//     let mass: [i32; 5] = [1,2,3,4,5];
//     println!("{:?}", mass);
//     let index = 5-1;
//     let elem = mass[index];
//     println!("{}", elem);
//     let g = sumi(4, 5);
//     println!("{}", g);
//     let _x = true;
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     let cond = if _x {
//         String::from("Hello")
//     } else {
//         String::from("Орел")
//     };
//     println!("{}", cond);
//     let mut i = 0;
//     let counter = loop {
//         i += 1;
//         if i == 10 {
//             break i * 2;
//         }
//     };
//     let mut x = 10;
//     println!("{}", counter);
//     let mut  i= 0;
//     while i < 10 {
//         i += 1;
//         x *= 4;
//         println!("{}", x);    
//     }
//     let mass = [11,22,33,44,55];
//     for elem in mass.iter() {
//         println!("{}", elem);
//     }
// }
// fn sumi(a: i32, b: i32) -> i32{
//     return a + b;
// }

// let tempc = 89.0;
// let result = temp_c(tempc);
// let result1 = temp_f(tempc);
// println!("{} градусов Фаренгейта", result.floor());
// println!("{} градусов Цельсия", result1.floor());
// }
// fn temp_c (a: f64) -> f64 {
//     (a * 1.8) + 32.0 
// }
// fn temp_f (b:f64) -> f64 {
//     (b - 32.0) / 1.8
// }


//  let chislo = 21;
//  let mut f1 = 1;
//  let mut f2 = 1;
//  let mut next  = f1 + f2;
//  while next <= chislo {
//     f1 =f2;
//     f2 = next;
//     next = f1 + f2;
//  }
//  println!("Число фибоначчи после {} равно {}", chislo, next);

let mut i = 0;
while i <= 3 {
    if i == 1 {
        println!("В {} день Рождества
Моя любимая отправила мне:
Куропатку на грушевом дереве", i);
    }
    else if i == 2 {
        println!("Во {} день Рождества
Моя любимая отправила мне:
Двух голубей
И куропатку на грушевом дереве", i);
    }
    else if i == 3 {
        println!("В {} Рождества
Моя любимая отправила мне:
Три французские курицы
Двух голубей
И куропатку на грушевом дереве
", i );
    }
    i += 1;
}

}
