fn main() {
    
    let textodenotas = value_as_notes(8877);
    println!("{}",textodenotas);
}
/*

Leia um valor inteiro. A seguir, calcule o menor número de notas 
possíveis (cédulas) no qual o valor pode ser decomposto. As notas 
consideradas são de 100, 50, 20, 10, 5, 2 e 1. A seguir mostre o 
valor lido e a relação de notas necessárias.

Entrada
O arquivo de entrada contém um valor inteiro N (0 < N < 1000000).

Saída
Imprima o valor lido e, em seguida, a quantidade mínima de notas 
de cada tipo necessárias, conforme o exemplo fornecido. Não esqueça 
de imprimir o fim de linha após cada linha, caso contrário seu 
programa apresentará a mensagem: “Presentation Error”.

Exemplo de Entrada
576

Exemplo de Saída
576
5 nota(s) de R$ 100,00
1 nota(s) de R$ 50,00
1 nota(s) de R$ 20,00
0 nota(s) de R$ 10,00
1 nota(s) de R$ 5,00
0 nota(s) de R$ 2,00
1 nota(s) de R$ 1,00

*/

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn value_as_notes(v: u32) -> String {
    let mut value = v;
    let resto_10 = value%10;
    let mut count_50 = 0;
    let mut count_20 = 0;
    let mut count_10 = 0;
    let mut count_5 = 0;
    let mut count_2 = 0;
    let mut count_1 = 0;
   
    match resto_10 {
        // Entrada 1 real
        1 => count_1=1,
        // Entrada 2 reais
        2 => count_2=1,
        3 => {
            count_1=1;
            count_2=1;
        },
        4 => count_2=2,
        5 => count_5=1,
        6 => {
            count_5=1;
            count_1=1;
        },
        7 => {
            count_5=1;
            count_2=1;
        },
        8 => {
            count_5=1;
            count_2=1;
            count_1=1;
        },
        9 => {
            count_5=1;
            count_2=2;
        },
        _ => {
            count_1=0;
            count_2=0;
            count_5=0;
        },
    }

    value = (value - resto_10)/10;
    let resto_100 = value%10;

    match resto_100 {
        // Entrada 10 reais
        1 => count_10=1,
        // Entrada 20 reais
        2 => count_20=1,
        3 => {
            count_10=1;
            count_20=1;
        },
        4 => count_20=2,
        5 => count_50=1,
        6 => {
            count_50=1;
            count_10=1;
        },
        7 => {
            count_50=1;
            count_20=1;
        },
        8 => {
            count_50=1;
            count_20=1;
            count_10=1;
        },
        9 => {
            count_50=1;
            count_20=2;
        },
        _ => {
            count_10=0;
            count_20=0;
            count_50=0;
        },
    }

    value = (value - resto_100)/10;
    let count_100 = value;
    let string_saida=(format!("{} nota(s) de R$ 100,00\n{} nota(s) de R$ 50,00\n{} nota(s) de R$ 20,00\n{} nota(s) de R$ 10,00\n{} nota(s) de R$ 5,00\n{} nota(s) de R$ 2,00\n{} nota(s) de R$ 1,00\n",count_100, count_50, count_20, count_10, count_5, count_2, count_1)).to_string();
    string_saida
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn testa_500() {
        let result = value_as_notes(500);
        assert_eq!(result, "5 nota(s) de R$ 100,00\n0 nota(s) de R$ 50,00\n0 nota(s) de R$ 20,00\n0 nota(s) de R$ 10,00\n0 nota(s) de R$ 5,00\n0 nota(s) de R$ 2,00\n0 nota(s) de R$ 1,00\n");
   }
   #[test]
   fn testa_501() {
       let result = value_as_notes(501);
       assert_eq!(result, "5 nota(s) de R$ 100,00\n0 nota(s) de R$ 50,00\n0 nota(s) de R$ 20,00\n0 nota(s) de R$ 10,00\n0 nota(s) de R$ 5,00\n0 nota(s) de R$ 2,00\n1 nota(s) de R$ 1,00\n");
  }
  
}