// Iterators

fn main() {
    let vec = vec![1, 2, 3, 4];

    for num in vec {
        println!("{}", num);
    }

    let vec = vec![1, 2, 3, 4];

    // What actually happens
    for num in vec.into_iter() {
        println!("{}", num);
    }

    let vec = vec![1, 2, 3, 4];

    // For each is an ITERATOR CONSUMER
    vec.into_iter().for_each(|x: i32| println!("{}", x));

    let vec = vec![1, 2, 3, 4];

    let even_total = vec // normally should type the var; used turbofish on sum instead
        .into_iter()
        .map(|x: i32| x * 3) //  FnMut(Self::Item) -> B
        .filter(|x: &i32| *x % 2 == 0) // FnMut(&Self::Item) -> bool
        .sum::<i32>(); // Iterator consumer --- Sum<Self::Item>

    println!("The total is {}", even_total);

    // println!("The vector is NOT usable: {:?}", vec); // Vec is consumed with into_iter

    let vec = [1, 2, 3, 4];

    let _ = vec
        .iter()
        .map(|x| x * 3)
        .filter(|x| *x % 2 == 0)
        .sum::<i32>();

    // Vec is still usable because we used iter() instead of into_iter()
    println!("The vector is still usable: {:?}", vec);

    let mut vec = vec![1, 2, 3, 4];

    println!("Before mutation: {:?}", vec); // Before mutation: [1, 2, 3, 4]

    // iter_mut allows us to mutate the items in the vector
    vec.iter_mut().for_each(|x| *x *= 3);

    println!("After mutation: {:?}", vec); // After mutation: [3, 6, 9, 12]

    // vec.into_iter() == for _ in vec
    // vec.iter() == for _ in &vec
    // vec.iter_mut() == for _ in &mut vec

    let mut vec = vec![1, 2, 3, 4, 5];

    println!("Before draining: {:?}", vec); // Before draining: [1, 2, 3, 4, 5]

    let drained = vec.drain(1..4).collect::<Vec<_>>();

    println!("Drained: {:?}", drained); // Drained: [2, 3, 4]

    println!("Remaining: {:?}", vec); // Remaining: [1, 5]
}
