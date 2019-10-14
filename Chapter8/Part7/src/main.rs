fn main() {

    let mut v = vec![10,20,30,40,50];

    for i in &mut v{
        
            *i += 50


   }

    println!("{:?}",v);


    let v = vec![10,20,30,40,50];

    for i in v{
        
        println!("{}",i );

   }

}
