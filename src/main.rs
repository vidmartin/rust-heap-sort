use std::{cmp::max_by_key, ops::Deref};
use rand::{self, Rng};

// dostane prvek v haldě na správné místo
fn heap_fix_node(arr: &mut [i32], index: usize) -> bool {
    if index >= arr.len() {
        return false;
    } else  if arr.len() == 0 {
        return true;
    }

    let mut curr = index;
    while curr * 2 + 1 < arr.len() {
        // zjistit většího potomka
        let (bigger_child_value, bigger_child_index) = 
            if curr * 2 + 1 == arr.len() - 1 {
                (arr[curr * 2 + 1], curr * 2 + 1) // pouze jeden potomek
            } else {
                max_by_key( // vícero potomků - vybrat většího
                (arr[curr * 2 + 1], curr * 2 + 1),
                (arr[curr * 2 + 2], curr * 2 + 2),
                 | tuple| tuple.0)
            };
        
        // pokud je potomek větší než aktuální prvek
        if bigger_child_value > arr[curr] {
            // prohodit většího potomka a aktuální prvek
            arr[bigger_child_index] = arr[curr];
            arr[curr] = bigger_child_value;

            // přesunout se na potomka, kterého jsme prohodili
            curr = bigger_child_index;
        } else {
            // pakliže jsou oba potomci menší, jsme spoko
            return true;
        }
    }

    return true;
}

// vyrobí haldu z pole
fn make_heap(arr: &mut [i32]) -> () {
    if arr.len() == 0 { return; }

    let last_index_with_children = (arr.len() - 1) / 2; // zaokrouhlí se dolu - to je správně
    for i in (0..last_index_with_children).rev() { // projít odzadu
        heap_fix_node(arr, i);
    }
}

// vrátí nejvyšší prvek v haldě a odstraní ho z haldy, sníživši velikost haldy o 1
fn heap_pop(arr: &mut [i32]) -> Option<i32> {
    if arr.len() == 0 {
        return None;
    }

    let out = arr[0];
    arr[0] = arr[arr.len() - 1];
    heap_fix_node(arr, 0);

    return Some(out);
}

// seřadí haldu. nejprve je třeba zavolat make_heap
fn heap_sort(arr: &mut [i32]) {
    if arr.len() == 0 {
        return;
    }

    for i in (0..arr.len()).rev() { // projít odzadu
        arr[i] = heap_pop(&mut arr[0..(i + 1)]).unwrap();
        heap_fix_node(&mut arr[0..i], 0);
    }
}

// spojí věci do řetězce
fn str_chain<T: Iterator>(join: &str, mut iter: T) -> String where T::Item: ToString {

    let mut s = match iter.next() {
        Some(item) => item.to_string(),
        None => return String::from("")
    };

    for i in iter {
        s.push_str(join);
        s.push_str(i.to_string().deref());
    }

    return s;
}

// zkontroluje, zdali je pole srovnané
fn check_sorted(arr: &[i32]) -> bool {
    let mut iter = arr.iter();
    let last = match iter.next() {
        Some(val) => val,
        None => return true
    };
    
    return loop {
        let curr = match iter.next() {
            Some(val) => val,
            None => break true
        };

        if curr < last {
            return false;
        }
    }
}

fn main() {
    let mut arr = [0; 20];
    for i in 0..arr.len() {
        arr[i] = rand::thread_rng().gen_range(0..50);
    }

    println!("before: {}", str_chain(", ", arr.iter()));
    println!("the array {} sorted", if check_sorted(&arr) { "is" } else { "isn't" });
    make_heap(&mut arr);
    println!("after make_heap: {}", str_chain(", ", arr.iter()));
    heap_sort(&mut arr);
    println!("after heap_sort: {}", str_chain(", ", arr.iter()));
    println!("the array {} sorted", if check_sorted(&arr) { "is" } else { "isn't" });
}
