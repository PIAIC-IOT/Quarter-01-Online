fn main() {
    
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    route(four); 
    route(six);
}

enum IPAddrKind{
    V4,
    V6
}

fn route(x: IPAddrKind){

}
