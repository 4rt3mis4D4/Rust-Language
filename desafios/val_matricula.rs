// 8. Validador de Matrículas (Slicing e Tipos Escalares)
// Dada uma lista de matrículas em um array de strings (ex: "2023-INF-01"), crie uma lógica que separe o ano do curso.

// Desafio: Use Slices de string (&str[0..4]) para extrair o ano e 
// converta-o para um inteiro. Verifique se o ano é anterior a 2020 usando if/else e 
// imprima um aviso de "Matrícula Antiga".

fn main(){
    // Array das matriculas
    let matriculas = ["2018-MAT-01", "2026-POR-02", "2016-FIS-03", "2023-INF-01", "2026-HIS-04"];
    
    let mut contador = 0;

    println!("=== Validador de Matrículas ===");
    for matricula in matriculas {
        contador += 1;

        let ano_str = &matricula[0..4];
        let ano_curso : u64 = ano_str.parse().expect("Falha ao converter ano para inteiro...");
        
        if ano_curso < 2020{
            println!("{}. Matricula: {} Antiga!", contador, matricula);
        } 
    }

    println!("\nFim do programa...");
}