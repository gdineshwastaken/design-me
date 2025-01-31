struct SortStrategy {
    // Empty Struct
}

enum Algorithm {
    QuickSort,
    InsertionSort,
    BubbleSort,
}

impl SortStrategy {
    fn sort(alg: Algorithm, cont: &mut [i32]) {
        match alg {
            Algorithm::QuickSort => Self::quick_sort(cont),
            Algorithm::InsertionSort => Self::insertion_sort(cont),
            Algorithm::BubbleSort => Self::bubble_sort(cont),
        }
    }

    fn insertion_sort(cont: &mut [i32]) {
        let n = cont.len();
        for i in 1..n {
            let key = cont[i];
            let mut j = i;
            while j > 0 && cont[j - 1] > key {
                cont[j] = cont[j - 1];
                j -= 1;
            }
            cont[j] = key;
        }
    }

    fn bubble_sort(cont: &mut [i32]) {
        let len = cont.len();
        for i in 0..len - 1 {
            for j in 0..len - i - 1 {
                if cont[j] > cont[j + 1] {
                    let temp = cont[j];
                    cont[j] = cont[j + 1];
                    cont[j + 1] = temp;
                }
            }
        }
    }

    fn quick_sort(cont: &mut [i32]) {
        if cont.len() <= 1 {
            return;
        }
        let pivot = Self::partition(cont);
        Self::quick_sort(&mut cont[0..pivot]);
        Self::quick_sort(&mut cont[pivot + 1 ..]);
    }

    fn partition(cont: &mut [i32]) -> usize {
        let len = cont.len();
        let pivot = cont[len - 1];
        let mut i = 0;
        for j in 0..len-1 {
            if cont[j] <= pivot {
                let temp = cont[i];
                cont[i] = cont[j];
                cont[j] = temp;
                i += 1;
            }
        }
        let temp = cont[len - 1];
        cont[len - 1] = cont[i];
        cont[i] = temp;
        i
    }
}

fn main() {
    let mut data = vec![5, 4, 3, 2, 1];
    SortStrategy::sort(Algorithm::QuickSort, &mut data);
    println!("{:?}", data);
}
