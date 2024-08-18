mod binary_search;
use binary_search::binary_search;

fn main() {
    let nums = vec![1,2,3,4,5,6,7,8,9, 10];
    let target = 7;
    let result = binary_search(&nums, target);
    if result == -1{
        println!("O numero {} não foi encontrado no vetor.", target);
    }else{
        println!("O numero {} foi encontrado no indice {} do vetor.", target, result);
    }
}
