use context::Context;
use futures::Future;
use futures::future;
use futures::task;
// use net2::TcpBuilder;
#[cfg(not(windows))]
// use net2::unix::UnixTcpBuilderExt;
// use num_cpus;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::thread;
use std::time::{Duration};
// use std::time::{Duration, Instant};
use tokio;
use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;
// use tokio::timer::Interval;
use tokio_codec::Framed;

use super::super::http::Http;
use super::super::request::Request;
// use super::super::codes;
use super::super::response::Response;
use super::batch::BatchedRequest;
use super::batch::BatchedRequestData;
// use super::batch::INCOMING_BATCH;


// pub struct RequestContext {
//     pub batch: Vec<Box<BatchedRequest<'static>>>,
// }

pub trait App<T: 'static + Context + Send> {
    fn preprocess (&self, request: Box<Request>) -> BatchedRequestData<'static>;
    fn process_batch (&self, batch_in_processing: &Box<Vec<Box<BatchedRequest>>>);
}

// pub fn new_vec<'a, 'b>() -> Vec<&'a BatchedRequest<'b>> {
//     Vec::with_capacity(1024)
// }



/**
 *
 *  Right now a single batch service is assumed to server only one type of request.
 *  This allows for the simplest possible solution, with one OS process per service.
 *  This also allows for very simple up-scaling.  Once a given request is known
 *  to be running too hot for too long, another service instance can simply be stood
 *  up (and added to the load balancer) for that type of request.
 *  This also makes upgrading a bit easier - if code is changed for one type of
 *  request then only that type of request is affected by the upgrade.  All the
 *  rest of the services can continue running without any interruption.
 *
 *  The downside is that this creates lots of processes to manage at the OS level.
 *  But, on the plus side, it is easy to see which request is eating up the CPU,
 *  using just the regular OS level tools like top.
 *
 *  For now, this is considered to be the preferred approach given that there
 *  aren't a lot of operations in VC:
 *
 *  Direction: Create + Read
 *  Dimension: Create + Read
 *  Poll:      Create + Read
 *  Vote:      Create
 *
 *
 *
 */
pub struct Server<T: 'static + Context + Send> {

    pub app: Box<App<T> + Send + Sync>,

    pub batch: Box<Vec<Box<BatchedRequest<'static>>>>,
}

impl<T: 'static + Context + Send> Server<T> {

    pub fn new(
        app: Box<App<T> + Send + Sync>
    ) -> Server<T> {
        let batch = Box::new(Vec::with_capacity(2048));
        Server {
            app,
            batch
        }
    }

    pub fn start_single_threaded(server: Server<T>, host: &str, port: u16) {
        
        let tcp_server = server.configure(host, port);

        thread::spawn(move || {
            server.start_batch();
        }); 

            tokio::run(tcp_server);

            // let copy_server = batch_arc_server.clone();
        // Setup the batching process

        // Start the batching process
        // tokio::run(task);
    //   });
    }

    fn configure(&self, host: &str, port: u16) {
        let addr = (host, port).to_socket_addrs().unwrap().next().unwrap();
    
        let listener = TcpListener::bind(&addr).unwrap();
        let arc_server = Arc::new(*self);
        let batch_arc_server = arc_server.clone();
        
        let tcp_server = listener.incoming()
            .map_err(|e| println!("error = {:?}", e))
            .for_each(move |socket| {
                let _ = socket.set_nodelay(true);
                process(arc_server.clone(), socket);
                Ok(())
        });   
        
        println!("Server running on {}", addr);    

       fn process<T: Context + Send>(server: Arc<Server<T>>, socket: TcpStream) {
           let framed = Framed::new(socket, Http);
           let (tx, rx) = framed.split();

           let task = tx.send_all(rx.and_then(move |request: Request| {
               server.resolve(request)
           })).then(|_| future::ok(()));

           // Spawn the task that handles the connection.
           tokio::spawn(task);
       }

       tcp_server
    }

    fn start_batch(mut self) {
        loop {
                let batch_in_processing = self.batch;

                self.batch = Box::new(Vec::with_capacity(2048));

                self.app.process_batch(&batch_in_processing);

                for batchRequest in (*batch_in_processing).iter() {
                    batchRequest.task.notify();
                }

                drop(batch_in_processing);

            thread::sleep(Duration::from_millis(1000));
        }
    }

    /// Resolves a request, returning a future that is processable into a Response
    fn resolve(
        &self, 
        mut request: Request,
        ) -> impl Future<Item=Response, Error=io::Error> + Send {
        let boxed_request = Box::new(request);

        let data = self.app.preprocess(boxed_request);

        if data.is_not_valid {
            return future::ok(*data.output);
        }

        let batched_request = Box::new(BatchedRequest {
            input: data.input,
            // Park the request until the batch job timer wakes up
            // Should work as park and suspend the task
            task: &task::current(),
            output: data.output,
        });

        self.batch.push(batched_request);

        // let request_reference: &'a mut BatchedRequest<'b, T> = &mut request;

        // let future = BatchedFuture {
        //     request: request_reference
        // };

        // let boxed_future: MiddlewareReturnValue<Response> = self.next(future);

        // future.and_then(|_| {
            future::ok(*data.output)
        // })

        // boxed_future.and_then(|response| {
        //     future::ok(response)
        // })
    }
    
}
