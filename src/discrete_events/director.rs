// Copyright 2018
// Author(s): Andres Goens

// This file implements the discrete events director.
// For now, channels don't do anything

//use discrete_events::channel{Channel};
//use discrete_events::actor{Actor};

use ::discrete_events::actor::{Actor};
use ::discrete_events::channel::{Channel};

use std::cmp::min;
const MAX_STEPS : u64 = 100;

pub struct DirectorState{
    curTime : u64,
    timeResolution : f64,
    pub actors : Vec<Actor>, //fixme: Can I get rid of this pub?
    channels :  Vec<Channel>,
}

// This should get a topology somehow at some point.
pub fn initDirector(actors : Vec<Actor>, channels : Vec<Channel>, resolution :f64)
              -> DirectorState{
    let dir = DirectorState{
        actors : actors,
        timeResolution : resolution,
        curTime : 0,
        channels : channels,
    };
    dir

}

pub fn minimalTime(dir : &DirectorState, actor : &Actor) -> u64{
    if actor.nextFiringTime > dir.curTime{
        return actor.nextFiringTime - dir.curTime;
    }
    else{
        return 0;
    }
}

pub fn canExecute(dir : &DirectorState, actor : &Actor) -> bool{
    if minimalTime(dir,actor) > 0 {
        return false;
    }
    else {
        return true;
    }
}

pub fn executeActor(actor : &mut Actor){
    (actor.fire)();
}

// This function goes to the next time step where something will
// happen. Currently it will just advance one, at some point it
// should look at the times and skip several steps if possible.
pub fn nextStep(dir : &mut DirectorState){
    let mut step : u64 = 1; //At least should move forward.
    dir.curTime += step;
    println!("Current time {}",dir.curTime);
}

// This function should put the new actors in the right place.
// The implementation for now will just naively append them.
// This will *break* as soon as we start getting anywhere interesting
// with the time semantics. Here we'll need topological sort and
// all that good jazz.
pub fn addToQueue(dir : &DirectorState, queue : &mut Vec<Actor>,
                  toAdd: Vec<Actor>){
    queue.extend(toAdd);
}

// This function should understand if a model is done executing
// according to its semantics. Currently it is hardcoded to execute
// 100 time steps.
pub fn isExecuting(dir : &DirectorState) -> bool{
    return dir.curTime < MAX_STEPS;
}


