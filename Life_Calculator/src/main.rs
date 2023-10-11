use std::io;

fn main() {

    let mut age  = String::new();
    let mut name  = String::new();
    let mut allowance=String::new();
    let mut bolsa_dinero: i32=0;
    
    
    println!("Ingresa tu nombre: ");
    io::stdin().read_line(&mut name).expect("Error al leer el nombre");

    println!("¿A qué edad futura deseas conocer tu dinero?");
    io::stdin().read_line(&mut age).expect("Error al leer la edad");

    let convertion: Result<i32, _> = age.trim().parse();

    match convertion {
        Ok(age) => {
            if age>18{
                
                println!("Ingresa tu mesada anual: ");
                io::stdin().read_line(&mut allowance).expect("Error al leer la linea");
                let allowance_int: Result<i32, _> = allowance.trim().parse();

                match allowance_int {
                    Ok(valor)=>{
                        bolsa_dinero=bolsa_dinero+(valor*15);
                    }
                    Err(_) => {
                        println!("No ingresaste un número válido en mesada anual");
                    }   

                }

                let mut inheritance=String::new();

                println!("Ingresa tu herencia, si no tuviste ingresa 0 : ");
                io::stdin().read_line(&mut inheritance).expect("Error al leer la linea");
                let inheritance_int: Result<i32, _> = inheritance.trim().parse();

                match inheritance_int {
                        Ok(valor)=>{
                            bolsa_dinero=bolsa_dinero+valor
                        }
                        Err(_) => {
                            println!("No ingresaste un número válido en herencia");
                        }   
                }
                
                let mut deudas=String::new();

                println!("Ingresa el valor de deuda que heredaste si no tuviste ingresa 0: ");
                io::stdin().read_line(&mut deudas).expect("Error al leer la linea");
                let deudas_int: Result<i32, _> = deudas.trim().parse();

                match deudas_int {
                    Ok(valor)=>{
                        bolsa_dinero=bolsa_dinero-valor
                    }
                    Err(_) => {
                        println!("No ingresaste un número válido en deuda");
                    }   
                }

                let mut salary=String::new();

                println!("Ingresa tu salario aproximado anual ");
                io::stdin().read_line(&mut salary).expect("Error al leer la linea");
                let salary_int: Result<i32, _> = salary.trim().parse();

                match salary_int {
                    Ok(valor)=>{
                        bolsa_dinero=bolsa_dinero+valor*(age-18)
                    }
                    Err(_) => {
                        println!("No ingresaste un número válido en salario");
                    }   
                }

                let mut emprendimiento=String::new();

                println!("Tendrás emprendimiento, 1 para  SI, 0 para NO ");
                io::stdin().read_line(&mut emprendimiento ).expect("Error al leer la linea");
                let emprendimiento_int: Result<i32, _> = emprendimiento.trim().parse();

                match emprendimiento_int {
                    Ok(valor)=>{
                        if valor==1{

                            let mut ganancia=String::new();

                            println!("Ingresa cuanto ganas por tu empredimiento anual");
                            io::stdin().read_line(&mut ganancia ).expect("Error al leer la linea");
                            let ganancia_int: Result<i32, _> = ganancia.trim().parse();

                            match ganancia_int {
                                Ok(ganancia)=>{
                                    bolsa_dinero=bolsa_dinero+(ganancia*(age-18))
                                }
                                Err(_) => {
                                    println!("No ingresaste un número válido en emprendimiento");
                                }   
                            }
                        }
                    }
                    Err(_) => {
                        println!("No ingresaste un número válido en emprendimiento");
                    }   
                }

                let mut tenerpareja=String::new();

                println!("¿Tendrás pareja? 1 para  SI, 0 para NO");
                io::stdin().read_line(&mut tenerpareja ).expect("Error al leer la linea");
                let tenerpareja_int: Result<i32, _> = tenerpareja.trim().parse();

                match tenerpareja_int {
                    Ok(valor)=>{
                        if valor==1{

                            let mut pareja=String::new();

                            println!("Tu pareja representa un gasto o un ingreso, ingresa 0 para gasto ó  1 para ingreso");
                            io::stdin().read_line(&mut pareja ).expect("Error al leer la linea");
                            let pareja_int: Result<i32, _> = pareja.trim().parse();

                            match pareja_int {
                                Ok(valor)=>{

                                        let mut gastopareja=String::new();

                                        println!("Ingresa dicho monto");
                                        io::stdin().read_line(&mut gastopareja ).expect("Error al leer la linea");
                                        let gastopareja_int: Result<i32, _> = gastopareja.trim().parse();

                                    if valor==1{
                                        match gastopareja_int {
                                            Ok(gastopareja_int)=>{
                                                bolsa_dinero=bolsa_dinero+gastopareja_int
                                            }
                                            Err(_) => {
                                                println!("No ingresaste un número válido en monto");
                                            }   
                                        }
                                    }else if valor==0 {
                                        match gastopareja_int {
                                            Ok(gastopareja_int)=>{
                                                bolsa_dinero=bolsa_dinero-gastopareja_int
                                            }
                                            Err(_) => {
                                                println!("No ingresaste un número válido en monto");
                                            }   
                                        }
                                    }
                                }
                                Err(_) => {
                                    println!("No ingresaste un número válido en pareja");
                                }   
                            }
                        }
                    }
                    Err(_) => {
                        println!("No ingresaste un número válido en tenerpareja");
                    }   
                }

                let mut tenerhijos=String::new();

                println!("¿Tendrás hijos? 1 para  SI, 0 para NO? ");
                io::stdin().read_line(&mut tenerhijos ).expect("Error al leer la linea");
                let tenerhijos_int: Result<i32, _> = tenerhijos.trim().parse();

                match tenerhijos_int {
                    Ok(valor)=>{
                        if valor==1{

                            let mut hijo=String::new();

                            println!("Ingrese el dinero invertido a sus hijos por año");
                            io::stdin().read_line(&mut hijo ).expect("Error al leer la linea");
                            let hijo_int: Result<i32, _> = hijo.trim().parse();

                            match hijo_int {
                                Ok(valor)=>{
                                    bolsa_dinero=bolsa_dinero-(valor*18)
                                }
                                Err(_) => {
                                    println!("No ingresaste un número válido en hijo");
                                }   
                            }
                        }
                    }
                    Err(_) => {
                        println!("No ingresaste un número válido en tenerhijos");
                    }   
                }

                let mut vivienda=String::new();

                println!("Ingresa los gastos por vivienda anuales");
                io::stdin().read_line(&mut vivienda).expect("Error al leer la linea");
                let vivienda_int: Result<i32, _> = vivienda.trim().parse();

                match vivienda_int {
                    Ok(valor)=>{
                        bolsa_dinero=bolsa_dinero-valor*(age-18)
                    }
                    Err(_) => {
                        println!("No ingresaste un número válido en vienda");
                    }   
                }

                let mut alimentacion=String::new();

                println!("Ingresa los gastos de alimentación anuales");
                io::stdin().read_line(&mut alimentacion).expect("Error al leer la linea");
                let alimentacion_int: Result<i32, _> = alimentacion.trim().parse();

                match alimentacion_int {
                    Ok(valor)=>{
                        bolsa_dinero=bolsa_dinero-(valor*(age-18))
                    }
                    Err(_) => {
                        println!("No ingresaste un número válido en alimentacion");
                    }   
                }

                if age>60{

                    let mut pension=String::new();

                    println!("Ingresa el valor de tu pensión");
                    io::stdin().read_line(&mut pension).expect("Error al leer la linea");
                    let pension_int: Result<i32, _> = pension.trim().parse();

                    match pension_int {
                        Ok(valor)=>{
                            bolsa_dinero=bolsa_dinero+valor*(age-60)
                        }
                        Err(_) => {
                            println!("No ingresaste un número válido en pension");
                        }   
                    }

                }
                
            }else if age>4 && age<=18{
                    println!("Ingresa tu mesada anual: ");
                    io::stdin().read_line(&mut allowance).expect("Error al leer la linea");
                    let allowance_int: Result<i32, _> = allowance.trim().parse();
    
                    match allowance_int {
                         Ok(valor)=> {
                            bolsa_dinero=bolsa_dinero+valor*(age-3);
                        }
                        Err(_) => {
                            println!("No ingresaste un número válido en mesada anual");
                        }   
                    }
                    
                }

                println!(" {} la cantidad de dinero que tendras a la edad de {} años Sera de {} pesos",name,age,bolsa_dinero);

            }   Err(_) => {
                println!("No ingresaste un número válido en edad.");
            }
                
            }
        
}