// Copyright 2018
// Author(s): Andres Goens

// This file shall implement the channels (some day).
#[derive(Copy)]
pub struct Channel{
    // Dummy channel for now, just to test the scheduler.
    dummy : u32, 
}

impl Clone for Channel {
    fn clone(&self) -> Channel { *self }
}
