pub fn validar_cpf(cpf: &str) -> bool {
    let cpf: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

    if cpf.len() != 11 {
        return false;
    }

    let numeros: Vec<u32> = cpf
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // Verifica se todos os dígitos são iguais (ex.: 11111111111)
    if numeros.iter().all(|&n| n == numeros[0]) {
        return false;
    }

    // Primeiro dígito verificador
    let mut soma = 0;
    for i in 0..9 {
        soma += numeros[i] * (10 - i as u32);
    }

    let resto = soma % 11;
    let digito1 = if resto < 2 { 0 } else { 11 - resto };

    if numeros[9] != digito1 {
        return false;
    }

    // Segundo dígito verificador
    soma = 0;
    for i in 0..10 {
        soma += numeros[i] * (11 - i as u32);
    }

    let resto = soma % 11;
    let digito2 = if resto < 2 { 0 } else { 11 - resto };

    numeros[10] == digito2
}

