#[derive(Clone)]
struct Values {
    Int: i32,
    Float: f32,
    Text: String,
}
fn VectorOperations() {
    let mut vector: Vec<i32> = [].to_vec();
    vector = [1, 2, 3, 4, 5].to_vec();
    let third: Option<&i32> = vector.get(2);

    match third {
        Some(third) => println!("Vector element found {third}"),
        None => println!("Vector element not found"),
    }

    for value in &mut vector {
        *value += 50;
        println!("Values in vector {value}")
    }

    let Values1: Values = Values {
        Int: 31,
        Float: 1.0,
        Text: String::from("Text"),
    };
    let mut ValuesVector: Vec<Values> = vec![Values1.clone(), Values1.clone()];

    for i in &ValuesVector {
        println!("Values {0}, {1}, {2}", i.Int, i.Float, i.Text)
    }
}

fn StoringUTFEncodedTextWithStrings()
{
    
}
fn main() {
    VectorOperations();
}
