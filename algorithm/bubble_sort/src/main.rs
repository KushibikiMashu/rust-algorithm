// バブルソートを実装

fn main() {
    let mut slice: [u32; 10] = [5, 6, 3, 2, 1, 3, 7, 9, 2, 1];
    bubble_sort(&mut slice);
    println!("{:?}", slice);
}

fn bubble_sort(v: &mut [u32]) {
    let len = v.len();
    for _ in 0..len - 1 {
        for j in 0..len - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
}

mod tests {
    use super::bubble_sort;

    #[test]
    fn sort() {
        let mut v: [u32; 8] = [8, 4, 6, 5, 7, 1, 3, 2];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
