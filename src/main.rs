fn main() {
    let a = double_int32(10);
    assert_eq!(a, 20);

    let b = double_int64(100);
    assert_eq!(b, 200);

    let c = double_float32(3.5);
    assert_eq!(c, 7f32);

    let d = double_float64(4.5);
    assert_eq!(d, 9f64);

    let e = int_plus_float_to_float(10, 6.25);
    assert_eq!(e, 16.25);

    let e = int_plus_float_to_int(10, 6.25);
    assert_eq!(e, 16);

    let f = tuple_sum((2, 3));
    assert_eq!(f, 5);

    let g = array_sum([4, 6]);
    assert_eq!(g, 10);
}

fn double_int32(a: i32) -> i32 {
    a * 2
}

fn double_int64(a: i32) -> i64 {
    a as i64 * 2
}

fn double_float32(a: f32) -> f32 {
    a * 2f32
}

fn double_float64(a: f32) -> f64 {
    a as f64 * 2f64
}

fn int_plus_float_to_float(a: i32, b: f32) -> f64 {
    a as f64 + b as f64
}

fn int_plus_float_to_int(a: i32, b: f32) -> i64 {
    a as i64 + b as i64
}

fn tuple_sum(items: (i32, i32)) -> i32 {
    items.0 + items.1
}

fn array_sum(items: [i32; 2]) -> i32 {
    items[0] + items[1]
}
