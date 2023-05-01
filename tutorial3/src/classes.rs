trait Quack {
    fn quack(&self);
    // You can also implement functions

    fn quack_with_generics<Q> (q: &Q)
        where Q: Quack {
        q.quack();
    }
}

struct Duck ();

impl Quack for Duck {
    fn quack(&self) {
        println!("quack!");
    }
}

struct RandomBird {
    is_a_parrot: bool
}

impl Quack for RandomBird {
    fn quack(&self) {
        if ! self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squawk!");
        }
    }
}

// and why the hell not!
impl Quack for i32 {
    fn quack(&self) {
        for i in 0..*self {
            print!("quack {} ",i);
        }
        println!("");
    }
}

// Iterator with generics!
fn quack_everyone <I> (iter: I)
    where I: Iterator<Item=Box<Quack>> {
    for d in iter {
        d.quack();
    }
}

fn animals() {
    let duck1 = Duck();
    let duck2 = RandomBird{is_a_parrot: false};
    let parrot = RandomBird{is_a_parrot: true};

    let ducks: Vec<&Quack> = vec![&duck1,&duck2,&parrot];

    for d in &ducks {
        d.quack();
    }


    let ducks: Vec<Box<Quack>> = vec![Box::new(duck1),Box::new(duck2),Box::new(parrot),Box::new(int)];

    quack_everyone(ducks.into_iter());
}

// Trait inheritance
trait Show {
    fn show(&self) -> String;
}

trait Location {
    fn location(&self) -> String;
}

trait ShowTell: Show + Location {}