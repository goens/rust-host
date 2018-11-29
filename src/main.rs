mod scheduler;
mod discrete_events;
use discrete_events::channel::{Channel};
use discrete_events::actor::{Actor};
//mod discrete_events{
//    pub mod director;
//    pub mod channel;
//    pub mod actor;
//}

fn a1fire(){
 println!("I'm actor 1");
}

fn a2fire(){
 println!("I'm actor 2");
}
fn main() {
    let emptyChannels : Vec<Channel> = Vec::new();
    let a1 = Actor{
        fire : a1fire,
        inputChannels : emptyChannels.clone(),
        outputChannels : emptyChannels.clone(),
        nextFiringTime : 0,
    };
    let a2 = Actor{
        fire : a2fire,
        inputChannels : emptyChannels.clone(),
        outputChannels : emptyChannels.clone(),
        nextFiringTime : 0,
    };

    let actors = vec![a1,a2];
    scheduler::run(actors,emptyChannels,1.0);
}
