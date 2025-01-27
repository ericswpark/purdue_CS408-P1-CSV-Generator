const VALUE_RANGE: [i32; 7] = [0, 1, 2, 100, 199, 200, 201];

fn main() {
    println!("a,b,c,expected");

    for a in VALUE_RANGE {
        for b in VALUE_RANGE {
            for c in VALUE_RANGE {
                // Skip if two or more variables have a non-nominal extreme value
                let mut extremes = 0;
                for v in [a, b, c] {
                    if v != VALUE_RANGE[VALUE_RANGE.len() / 2] {
                        extremes += 1;
                    }
                }
                if extremes >= 2 {
                    continue;
                }

                let result = triangle_type(a as usize, b as usize, c as usize);
                println!("{},{},{},{}", a, b, c, result as i8);
            }
        }
    }
}

enum TriangleType {
    OutOfRange = -2,
    Invalid = -1,
    Scalene = 0,
    Isosceles = 1,
    Equilateral = 2,
}

fn triangle_type(a: usize, b: usize, c: usize) -> TriangleType {
    if in_range(a) && in_range(b) && in_range(c) {
        if a == b && b == c {
            TriangleType::Equilateral
        } else if a + b > c && a + c > b && b + c > a {
            if a == b || b == c || a == c {
                TriangleType::Isosceles
            } else if a != b && b != c && a != c {
                return TriangleType::Scalene;
            } else {
                panic!("Shouldn't happen");
            }
        } else {
            TriangleType::Invalid
        }
    } else {
        TriangleType::OutOfRange
    }
}

fn in_range(n: usize) -> bool {
    (1..=200).contains(&n)
}
