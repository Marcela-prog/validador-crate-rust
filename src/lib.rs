pub mod validadores {

pub fn cpf_cpf(cpf: &str) -> bool {
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

/////

    pub fn validar_cnpj(cnpj: &str) -> bool {
        // 1. Remove qualquer caractere que não seja dígito
        let digitos: Vec<u32> = cnpj
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
    
        // 2. O CNPJ precisa ter exatamente 14 dígitos
        if digitos.len() != 14 {
            return false;
        }
    
        // 3. Rejeita CNPJs com todos os dígitos iguais (ex: 11.111.111/1111-11)
        if digitos.iter().all(|&x| x == digitos[0]) {
            return false;
        }
    
        // 4. Cálculo do Primeiro Dígito Verificador
        // Os pesos para o primeiro dígito são: 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2
        let pesos_primeiro = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let mut soma = 0;
        for i in 0..12 {
            soma += digitos[i] * pesos_primeiro[i];
        }
        
        let resto = soma % 11;
        let primeiro_digito_calculado = if resto < 2 { 0 } else { 11 - resto };
    
        if digitos[12] != primeiro_digito_calculado {
            return false;
        }
    
        // 5. Cálculo do Segundo Dígito Verificador
        // Os pesos para o segundo dígito são: 6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2
        let pesos_segundo = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        soma = 0;
        for i in 0..13 {
            soma += digitos[i] * pesos_segundo[i];
        }
    
        let resto = soma % 11;
        let segundo_digito_calculado = if resto < 2 { 0 } else { 11 - resto };
    
        digitos[13] == segundo_digito_calculado
    }
}




