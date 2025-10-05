// Обобщаем поведение с помощью шаблонов и статического полиморфизма.
// Пусть у нас 3 типа фигур: треугольник, прямоугольник и круг
// Создайте трейт Shape, в котором есть методы:
// get_area(&self) -> f64 // возвращает зачение площади фигуры
// get_perimeter(&self) -> f64 // возвращает значение периметра фигуры
// Реализуйте данный трейт для треугольника, прямоугольника и круга

// Напишите 1 функцию perimeter_by_area, которая может принимать любую фигуру
// и возвращает отнощение ее периметра к площади (P/A)

struct Triangle {
    sides_lens: [f64; 3],
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

trait Shape {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

impl Shape for Triangle {
    fn get_area(&self) -> f64 {
        let p = self.get_perimeter() / 2f64;

        (p * (p - self.sides_lens[0]) * (p - self.sides_lens[1]) * (p - self.sides_lens[2])).sqrt()
    }

    fn get_perimeter(&self) -> f64 {
        self.sides_lens.iter().sum()
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.height * self.width
    }

    fn get_perimeter(&self) -> f64 {
        (self.width + self.height) * 2f64
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2f64)
    }

    fn get_perimeter(&self) -> f64 {
        2f64 * std::f64::consts::PI * self.radius
    }
}

fn perimeter_by_area<T: Shape>(shape: T) -> f64 {
    shape.get_perimeter() / shape.get_area()
}

fn main() {
    let a = Triangle {
        sides_lens: [3.0, 4.0, 5.0],
    };
    let b = Rectangle {
        width: 10.0,
        height: 4.0,
    };
    let c = Circle { radius: 2.0 };

    println!("{}", perimeter_by_area(a));
    println!("{}", perimeter_by_area(b));
    println!("{}", perimeter_by_area(c));
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn test() {
        relative_eq!(
            perimeter_by_area(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            }),
            2.0
        );
        relative_eq!(perimeter_by_area(Circle { radius: 2.0 }), 1.0);
        relative_eq!(
            perimeter_by_area(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
            1.6666
        );
    }
}
