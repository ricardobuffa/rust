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
    let mut count_100 = 0;
    let mut count_50 = 0;
    let mut count_20 = 0;
    let mut count_10 = 0;
    let mut count_5 = 0;
    let mut count_2 = 0;
    let mut count_1 = 0;

    if value >= 100 {
        count_100 = value / 100;
        value = value % 100;
    }

    if value >= 50 {
        count_100 = value / 50;
        value = value % 50;
    }

    if value >= 20 {
        count_100 = value / 20;
        value = value % 20;
    }

    if value >= 10 {
        count_100 = value / 10;
        value = value % 10;
    }

    if value >= 5 {
        count_100 = value / 5;
        value = value % 5;
    }

    if value >= 2 {
        count_100 = value / 2;
        value = value % 2;
    }

    if value >= 1 {
        count_1 = value
    }

    format!("{count_100} nota(s) de R$ 100,00\n{count_1} nota(s) de R$ 1,00")
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
        assert_eq!(result, "5 nota(s) de R$ 100,00\n0 nota(s) de R$ 1,00");
   }
   #[test]
   fn testa_501() {
       let result = value_as_notes(501);
       assert_eq!(result, "5 nota(s) de R$ 100,00\n1 nota(s) de R$ 1,00");
  }
  #[test]
   fn testa_501() {
       let result = value_as_notes(501);
       assert_eq!(result, "5 nota(s) de R$ 100,00\n1 nota(s) de R$ 1,00");
  }
}