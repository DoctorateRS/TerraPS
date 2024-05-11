use rand::{rngs::ThreadRng, thread_rng, Rng};
pub struct TRng(pub ThreadRng);
impl TRng {
    pub fn new() -> Self {
        Self(thread_rng())
    }
}

unsafe impl Send for TRng {}

pub fn sample<T: PartialEq + Clone>(vec: Vec<T>, n: usize) -> Vec<T> {
    let mut rng = TRng::new();
    let mut res = Vec::with_capacity(n);
    let len = vec.len();
    if n >= len {
        vec
    } else {
        loop {
            let index = rng.0.gen_range(0..len);
            let x = vec.get(index).unwrap();
            if res.contains(x) {
                continue;
            } else {
                res.push(x.clone());
            }
            if res.len() == n {
                break;
            }
        }
        res
    }
}
