#[allow(dead_code)]
pub fn read() {
    println!("A thread pool is a group of spawned threads that are waiting and ready to handle a task");

    //When a thread is done handling a task, it's returned to the list of available threads and can be used again
    //We should limit our threads to a certain number so we are protected from DoS attacks
        //if we don't limit our threads and made a new one for each request, 
        //someone could make millions of requests and use all of our server's resources

    //We'll call the list of waiting threads the 'pool'
        //When a request is received, it's sent to the pool and given to the next available thread
        //When the request is done being handled, it's returned to the pool
        //That means we can handle as many concurrent requests as we have threads
    
    println!("This isn't immune to problems though");
    //We've just made it so we can handle num_threads requests simultaneously, if someone made 10 requests that all took a minute we'd still stall
        //but it's a solution

    //Before we actually make the pool, let's think about how we should instantiate and use this pool
        //Writing the client interface first may help us design better - write code the way you want it to be called
}