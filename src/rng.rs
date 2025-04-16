use std::ops::{Bound, RangeBounds};

use rand_core::RngCore;

pub trait FloatRng: RngCore {
    fn next_percentage(&mut self) -> f32 {
        let random_number = self.next_u32() as f32;
        random_number / u32::MAX as f32
    }

    fn next_percentage_clamped<R: RangeBounds<f32>>(&mut self, range: R) -> f32 {
        let (min, max) = translate_range(range);
        self.next_percentage().clamp(min, max)
    }

    fn next_f32_range<R: RangeBounds<f32>>(&mut self, range: R) -> f32 {
        let p = self.next_percentage();
        let (min, max) = translate_range(range);
        min + p * (max - min)
    }
}

impl<T> FloatRng for T where T: RngCore {}

fn translate_range<R: RangeBounds<f32>>(range: R) -> (f32, f32) {
    let min = match range.start_bound() {
        Bound::Excluded(f) => *f - 0.000001,
        Bound::Included(f) => *f,
        Bound::Unbounded => f32::MIN,
    };
    let max = match range.end_bound() {
        Bound::Excluded(f) => *f - 0.000001,
        Bound::Included(f) => *f,
        Bound::Unbounded => f32::MAX,
    };

    (min, max)
}
