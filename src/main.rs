mod server;
mod client;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "awsr")]
enum Opt {
    /// Runs the server
    Server {
        /// The port to listen on
        #[structopt(long)]
        port: u16,     
    },
    /// Runs the client
    Client {
        /// The port to which to send
        #[structopt(long)]
        port: u16,     
    },
}

#[derive(Debug)]
enum Error {
    Server(tide::Error),
    Client(Box<dyn std::error::Error>)
}

fn main() -> Result<(), Error> {

    match Opt::from_args() {
        Opt::Server { port } => {
            server::run_blocking(port)?;
        }
        
        Opt::Client { port } =>  {
            client::run_blocking(port)?;        
        }
    }   

    Ok(())
}
