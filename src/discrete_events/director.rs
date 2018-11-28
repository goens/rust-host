// Copyright 2018
// Author(s): Andres Goens

// This file implements the discrete events director.
// For now, channels don't do anything

use discrete_events::channel{Channel};
use discrete_events::actor{Actor};

pub mod director{
    
    use std::cmp::min;
    const MAX_STEPS : u64 = 100;
    
    struct DirectorState{
        curTime : u64,
        timeResolution : f64,
        actors : Vec<Actor>,
        channels :  Vec<Channel>,
    }
    
    // This should get a topology somehow at some point.
    pub init(actors: Vec<Actor>, resolution :f64)
                  -> DirectorState{
    
    
    }
    
    pub fn minimalTime(DirectorState dir, Actor actor) -> u64{
        return actor.nextFiringTime - dir.curTime;
    }
    
    pub fn canExecute(DirectorState dir, Actor actor) -> bool{
        if minimalTime(dir,actor) > 0 {
            return false;
        }
        else {
            return true;
        }
    }
    
    pub fn executeActor(mut& Actor actor) -> Actor{
        actor.fire();
    }
    
    // This function goes to the next time step where something will
    // happen. Currently it will just advance one, at some point it
    // should look at the times and skip several steps if possible.
    pub fn nextStep(mut& DirectorState dir){
        let mut nextStep : u64 = 1; //At least should move forward.
    
    }
    
    // This function should put the new actors in the right place.
    // The implementation for now will just naively append them.
    // This will *break* as soon as we start getting anywhere interesting
    // with the time semantics. Here we'll need topological sort and
    // all that good jazz.
    pub fn addToQueue(DirectorState dir, mut& Vec<Actor> queue,
                      Vec<Actor> toAdd){
        queue.extend(toAdd[..]);
    }
    
    // This function should understand if a model is done executing
    // according to its semantics. Currently it is hardcoded to execute
    // 100 time steps.
    pub fn isExecuting(DirectorState dir) -> Bool{
        return (dir.curTime < MAX_STEPS);
    }
    

}
