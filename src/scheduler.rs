// Copyright 2018
// Author(s): Andres Goens

// This file implements a simple scheduler for actors.
// Currently it will only execute the discrete event model of
// computation.

//mod discrete_events;
//use discrete_events::director;
//use std::collections::Vec; //instead of vec?
use ::discrete_events::actor::{Actor};
use ::discrete_events::channel::{Channel};
use ::discrete_events::director::{DirectorState,addToQueue,canExecute,
                                initDirector,nextStep,executeActor,
                                isExecuting};
pub struct SchedulerState{
    queue : Vec<Actor>,
}

pub fn initScheduler(dir : &DirectorState) -> SchedulerState{
    //TODO: This should just take the actors that can execute.
    // Currently it just takes them all.
    let queue = dir.actors.clone(); 
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
fn executeStep(dir : &DirectorState, state : &mut SchedulerState)
               -> Vec<Actor>{
    let mut executedActors : Vec<Actor> =
        Vec::new();
    while !state.queue.is_empty(){
        // Execute current actor.
        let mut nextActor = state.queue.pop().unwrap();
        // Next in queue cannot be executed? Return it to queue
        // and stop executing.
        if !canExecute(dir,&nextActor) {
            state.queue.push(nextActor);
            break;
        }
        else{
            executeActor(&mut nextActor);
            executedActors.push(nextActor);
        }
    }
    return executedActors;
}

pub fn execute(state : &mut SchedulerState, dir : &mut DirectorState){
    while isExecuting(dir){
        let executedActors = executeStep(dir,state);
        addToQueue(dir,&mut state.queue,executedActors);
        nextStep(dir);
    }
}

pub fn run(actors : Vec<Actor>, channels : Vec<Channel>,
           resolution : f64){
    //Initialize.
    let mut dir = initDirector(actors,channels, resolution);
    let mut state = initScheduler(&dir);

    execute(&mut state,&mut dir);
}
