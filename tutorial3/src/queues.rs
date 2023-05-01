// QUEUE =================
#[macro_use] extern crate queues;
use queues::*;

fn main() {
    println!("Demo pragrma to show queue in rust !!");
    let mut demoqueue: Queue<isize> = queue![];
    demoqueue.add(200);
    demoqueue.add(300);
    demoqueue.add(400);
    demoqueue.add(500);
    demoqueue.add(600);
    println!(" value inside the queue is {}", demoqueue );
}

// PRIORITY QUEUE =================
use priority_queue::PriorityQueue;

let mut pq = PriorityQueue::new();

assert!(pq.is_empty());
pq.push("Apples", 5);
pq.push("Bananas", 8);
pq.push("Strawberries", 23);

assert_eq!(pq.peek(), Some((&"Strawberries", &23)));

pq.change_priority("Bananas", 25);
assert_eq!(pq.peek(), Some((&"Bananas", &25)));

for (item, _) in pq.into_sorted_iter() {
println!("{}", item);
}