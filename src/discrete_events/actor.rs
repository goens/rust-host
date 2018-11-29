// Copyright 2018
// Author(s): Andres Goens

// This file shall implement the actors (some day)
use ::discrete_events::channel::{Channel};

#[derive(Clone)]
pub struct Actor{
    pub fire : fn(),
    pub inputChannels : Vec<Channel>,
    pub outputChannels : Vec<Channel> ,
    pub nextFiringTime : u64,
}

// Should fix the visibility of the struct to remove the pub from
// nextFiringTime (make everything be part of the module
// discrete_events).
