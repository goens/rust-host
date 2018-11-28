// Copyright 2018
// Author(s): Andres Goens

// This file shall implement the actors (some day)

pub struct Actor{
    fire : fn(),
    inputChannels : Vec<Channel>,
    outputChannels : Vec<Channel> ,
    nextFiringTime : u64,
}
