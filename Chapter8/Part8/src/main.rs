fn main() {

let row = vec![SpreadSheetCell::Int(50), SpreadSheetCell::Float(10.5), SpreadSheetCell::Text(String::from("hello"))];

}

enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String)
}
