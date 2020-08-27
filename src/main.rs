use std::collections::BTreeMap;
use std::collections::BinaryHeap;
//use std::thread::yield_now;

struct OrderDetails {
    id: u64,
    amount: i64,
    swap_type: bool,   //1 for swap-ins and 0 for swap-outs

}

struct Queues {
    swap_in: BTreeMap<u64,i64>,
    swap_out: BTreeMap<u64,i64>,
    priority_queue: BinaryHeap<i64>,
}

trait QueueOperations {
    fn swap_in_queue(&mut self,x:&u64,y:&i64) ;
    fn swap_out_queue(&mut self,x:&u64,y:&i64) ;
    fn priority_queue(&mut self,x:&i64);
    fn match_queue(&self);
}

impl QueueOperations for Queues {

    fn swap_in_queue(&mut self,x:&u64,y:&i64) {

        self.swap_in.insert(*x, *y);

    }

    fn swap_out_queue(&mut self,x:&u64,y:&i64) {

        self.swap_out.insert(*x, *y);

    }

    fn priority_queue(&mut self,x:&i64){

    self.priority_queue.push(*x);

    }

    fn match_queue(&self) {
        let mut m:bool = false;
        let mut x_sum:i64 = 0;
        if let Some(y) = self.swap_out.get(&1){
            for i in 1..5 {
                if let Some(x) = self.swap_in.get(&i)  {

                    x_sum += x;
                    if x_sum >= *y {
                        println!("Found match at order {} with sum:{} for {}", i,x_sum,y);
                        m = true;
                        break;
                    }
                }
                else {
                    println!("swap-in queue is empty, opening bot-wallet");
                    break;
                }

            }
            if m != true && self.swap_in.len() >= 5 {
                if let Some(l) = self.priority_queue.peek(){
                    if x_sum+l >= *y {
                        println!("Match found from priority queue:{}+{}={} for total trade value {}",
                                 l,x_sum,x_sum+l,y);
                        m = true;
                    }
                    else {
                        println!("Value in queue insufficient, opening bot-wallet, total needed:{},\
                        have only {}",y,x_sum+l);
                    }
                }
                else {
                    print!("Swap-in queue is empty! Opening bot wallet");
                }


            }

        }
        else { println!("Swap-out queue is empty"); }


    }

}

fn main() {
    let a: BTreeMap<u64,i64> = BTreeMap::new();
    let b: BTreeMap<u64,i64> = BTreeMap::new();
    let p:BinaryHeap<i64> = BinaryHeap::new();

    let mut c = Queues { swap_in: a, swap_out: b, priority_queue: p };
    let mut st:bool = true;
    let mut am =0;
    for i in 1..10 {
        am += 3;
        //let order = OrderDetails {id:i,amount:am,swap_type:st};
            QueueOperations::swap_in_queue(&mut c,&i,&am);
            QueueOperations::priority_queue(&mut c,&am);
        }
    st = false; am=0;

    for i in 1..10 {
        am += 31;
        let order = OrderDetails {id:i,amount:am,swap_type:st};
        QueueOperations::swap_out_queue(&mut c,&i,&am);
    }

    QueueOperations::match_queue(&c);
    println!("The number of items in the swap-in queue is:");
    for (i, j) in c.swap_in {
        println!("{}: \"{}\"", i, j);
    }
    println!("The number of items in the swap-out queue is:");
    for (i, j) in c.swap_out {
        println!("{}: \"{}\"", i, j);
    }
    println!("The priority queue from the swap-in order is:");
    for x in &c.priority_queue {
        println!("{}", x);
    }
    let length = c.priority_queue.len();
    let largest = c.priority_queue.peek();

}