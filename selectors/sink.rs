/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Small helpers to abstract over different containers.
#![deny(missing_docs)]

use smallvec::SmallVec;

/// A trait to abstract over a `push` method that may be implemented for
/// different kind of types.
///
/// Used to abstract over `Array`, `SmallVec` and `Vec`, and also to implement a
/// type which `push` method does only tweak a byte when we only need to check
/// for the presence of something.
pub trait Push<T> {
    /// Push a value into self.
    fn push(&mut self, value: T);
}

impl<T> Push<T> for Vec<T> {
    fn push(&mut self, value: T) {
        Vec::push(self, value);
    }
}

impl<T, const N: usize> Push<T> for SmallVec<T, N> {
    fn push(&mut self, value: T) {
        SmallVec::push(self, value);
    }
}
