use std::collections::HashSet;

fn main() {
    // hola mundo
    println!("Hello, world!");

    // variable
    let mut my_string: &str="cadena de texto";
    println!("{my_string}");

    my_string="Aqui cambia el valor";
    println!(" Variable: {my_string}");

    //my_string=6; Error

    let my_string2: String= String::from("esto es otro sting mas optimo");
    println!("{my_string2}");

    let mut my_int=7;
    my_int=my_int + 4;
    println!("{my_int}");
    println!("{}",my_int-1);
    println!("{my_int}");

    println!("Este es el valor de la variable my_int: {}",my_int);

    let my_int64: i64=7;
    println!("{my_int64}");

    let my_float=6.5;
    println!("{my_float}");

    //my_float=my_float + my_int; // Error

    let my_float2: f32=6.5;
    println!("{my_float2}");

    let mut my_bool: bool= false;
    my_bool=true;
    println!("{my_bool}");

    //constantes 
    const MY_CONST: &str="mi propia constante";
    println!("{MY_CONST}");

    //CONTROL DE FLUJO
    if my_int==10{
        println!("el valor es 10")
    } else if  my_int==11{
        println!("El valor es 11")
    }else{
        println!("el valor es 10 o 11")
    }

    // lista
    let mut my_list=vec!["nombre","apellido","edad"];
    my_list.push("arge");
    println!("{:?}",my_list);
    println!("{}",my_list[0]);

    // Sets
    let my_set: HashSet<&str> =vec!["nombre","apellido","@edad"].into_iter().collect();
    println!("{:?}",my_set);

}
