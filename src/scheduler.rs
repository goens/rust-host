// Copyright 2018
// Author(s): Andres Goens

// This file implements a simple scheduler for actors.
// Currently it will only execute the discrete event model of
// computation.

use discrete_events::director;
//use std::collections::Vec; //instead of vec?

pub mod scheduler {
    struct SchedulerState{
        queue : Vec<director::Actor>,
    }
    
    pub fn init(dir : director::DirectorState) -> SchedulerState{
        let queue = foo; //TODO: get actors that can execute
        let state = SchedulerState{
            queue : queue,
        };
        return state;
    }
    
    // This is a very simple scheduler for now. Will just execute things
    // in the order they are found in the list, and then increase the time
    // counter.
    //
    // The director is responsible for keeping the queue state in order.
    fn executeStep(state : &mut SchedulerState)
                   -> Vec<director::Actor>{
        let mut executedActors : Vec<director::Actor> =
            Vec::new();
        while !state.queue.is_empty(){
            // Execute current actor.
            let mut nextActor = state.queue.pop();
            // Next in queue cannot be executed? Return it to queue
            // and stop executing.
            if !director::canExecute(dir,nextActor) {
                state.queue.push(nextActor);
                break;
            }
            else{
                director::executeActor(nextActor);
                executedActors.push(nextActor);
            }
        }
        return executedActors;
    }
    
    pub fn execute(state : &mut SchedulerState, dir : &mut director::DirectorState){
        while director::isExecuting(dir){
            let executedActors = executeStep(state);
            director::addToQueue(dir,state.queue,executedActors);
            director::nextStep(dir);
        }
    }
    
    pub fn run(actors : Vec<director::Actor>, resolution : f64){
        //Initialize.
        let dir = director::init(actors,resolution);
        let state = init(dir);
    
        execute(state,dir);
    }

}
