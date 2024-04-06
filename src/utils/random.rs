use rand::Rng;

pub fn sample<T: PartialEq + Clone>(vec: Vec<T>, n: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let mut res = Vec::with_capacity(n);
    loop {
        let index = rng.gen_range(0..vec.len());
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
