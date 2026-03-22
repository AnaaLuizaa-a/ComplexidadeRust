pub fn potencias_de_dois(n: u64){
    let mut i: u64 = 1;
    while i< n{
        print!("{} ", i);
        i *= 2;
    }
}

fn main(){
    let _lista = vec![1000, 10000, 1000000];

    potencias_de_dois(10000);
}