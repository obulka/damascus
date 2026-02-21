// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FrameCounter {
    pub first_frame: u32,
    pub frame: u32,
    pub previous_frame_time: SystemTime,
    pub fps: f32,
    pub frames_to_update_fps: u32,
    pub paused: bool,
}

impl Default for FrameCounter {
    fn default() -> Self {
        Self {
            first_frame: 0,
            frame: 0,
            previous_frame_time: SystemTime::now(),
            fps: 0.,
            frames_to_update_fps: 10,
            paused: false,
        }
    }
}

impl FrameCounter {
    pub fn first_frame(mut self, first_frame: u32) -> Self {
        self.first_frame = first_frame;
        self
    }

    pub fn paused(mut self, paused: bool) -> Self {
        self.paused = paused;
        self
    }

    pub fn tick(&mut self) {
        if self.paused {
            self.update_frame_time();
            if self.frame == 0 {
                self.frame = 1;
            }
            return;
        }

        if self.frame != self.first_frame
            && self.frames_since_first() % self.frames_to_update_fps == 0
        {
            match SystemTime::now().duration_since(self.previous_frame_time) {
                Ok(frame_time) => {
                    self.fps = self.frames_to_update_fps as f32 / frame_time.as_secs_f32();
                }
                Err(_) => self.fps = 0.,
            }

            self.update_frame_time();
        }

        self.frame += 1;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn play(&mut self) {
        self.paused = false;
    }

    pub fn frames_since_first(&self) -> u32 {
        self.frame - self.first_frame
    }

    pub fn update_frame_time(&mut self) {
        self.previous_frame_time = SystemTime::now();
    }

    pub fn reset(&mut self) {
        self.update_frame_time();
        self.frame = self.first_frame;
    }
}
