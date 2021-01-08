use std::thread;
use std::time::Duration;
fn main() {
   
    //single threaded code
    for i in 1..10{
    println!("excecuting from I loop {}",i);
    };
    for j in 1..10{
        println!("excecuting from J loop {}",j);
    };

    //to make two loops work/print simultaneously(multi-threading), we use threads.
    //we import a thread library for this.

    //multi-threaded code

    let handle = thread::spawn(||{for x in 0..10{

        println!("This is from New Thread X loop {}",x);
        thread::sleep(Duration::from_millis(1));
    };
}
); //if we use handle.join().unwrap() here. It will run the custom thread first then the main thread
    //Not simultaneously

    for y in 0..5{
        println!("This is from Main Thread Y loop {}",y);
        thread::sleep(Duration::from_millis(1));
    };
    
    //using handles we can excecute the threads in our program.
    handle.join().unwrap();
    //if we want to run the threads simultaneously. 
    //main kai function kai baad 'handle' use hoga.
    
}
