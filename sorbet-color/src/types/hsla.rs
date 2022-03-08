/*
    Copyright 2022 Jacob Birkett

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

use std::hash;

use crate::types::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsla {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub alpha: f64,
}

impl Eq for Hsla {}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for Hsla {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.h.to_bits().hash(state);
        self.s.to_bits().hash(state);
        self.l.to_bits().hash(state);
        self.alpha.to_bits().hash(state);
    }
}

//
// Implement to/from primitives
//

impl From<[f64; 4]> for Hsla {
    fn from(array: [f64; 4]) -> Self {
        Self {
            h: array[0],
            l: array[1],
            s: array[2],
            alpha: array[3],
        }
    }
}

impl From<Hsla> for [f64; 4] {
    fn from(color: Hsla) -> Self {
        [color.h, color.s, color.l, color.alpha]
    }
}

//
// Implement From for all other Color types
//

impl From<Rgb> for Hsla {
    fn from(other: Rgb) -> Self {
        Self::from(Rgba::from(other))
    }
}

impl From<Hsv> for Hsla {
    fn from(other: Hsv) -> Self {
        Self::from(Hsva::from(other))
    }
}

impl From<Hsl> for Hsla {
    fn from(other: Hsl) -> Self {
        Self {
            h: other.h,
            s: other.s,
            l: other.l,
            alpha: 1.0,
        }
    }
}

impl From<Rgba> for Hsla {
    fn from(other: Rgba) -> Self {
        let Hsl { h, s, l } = Hsl::from(other);

        Self {
            h,
            s,
            l,
            alpha: other.alpha,
        }
    }
}

impl From<Hsva> for Hsla {
    fn from(other: Hsva) -> Self {
        let Hsl { h, s, l } = Hsl::from(other);

        Self {
            h,
            s,
            l,
            alpha: other.alpha,
        }
    }
}