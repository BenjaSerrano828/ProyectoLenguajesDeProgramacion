use std::{fs, io};
use std::io::{BufRead, Write};
use crate::Tipo::Audifonos;

#[derive(Debug)]

struct Producto{
    nombre: String,
    precio: f64,
    stock: i32,
    tipo: Tipo,
    marca: Marca,
    descripcion: String,
}

#[derive(Debug)]
enum Tipo{
    Audifonos,
    Mouse,
    Teclado,
    Monitor,
    Periferico, // tipo default
}

#[derive(Debug)]
enum Marca{
    HyperX,
    Logitech,
    Samsung,
    Redragon,
    Razer,
    SinEspecificar, // marca default
}

impl Producto {
    fn new(nombre:String, precio:f64, stock:i32, tipo:Tipo, marca: Marca, descripcion: String) -> Producto{
        Self{
            nombre: nombre,
            precio: precio,
            stock: stock,
            tipo: tipo,
            marca: marca,
            descripcion: descripcion,
        }
    }
    fn mostrar_producto(&self){
        println!("Nombre: {}",self.nombre);
        println!("Precio: {}",self.precio);
        println!("Stock: {}",self.stock);
        println!("Tipo: {:?}",self.tipo);
        println!("Marca: {:?}",self.marca);
        println!("Descipción: {}",self.descripcion);
    }

}

fn cargar_producto(mut productos:&mut Vec<Producto>) -> Result<&Vec<Producto>, io::Error>{
    let contenido = fs::read_to_string("src/productos.txt".to_string());

    for linea in contenido?.lines (){
        let campos: Vec<&str> = linea.split((',')).collect();
        if campos.len() == 6 {
            let nombre = campos[0].to_string();
            let precio = campos[1].parse::<f64>().unwrap();
            let stock = campos[2].parse::<i32>().unwrap();
            let tipo = match campos[3] {
                "Audifonos" => Tipo::Audifonos,
                "Mouse" => Tipo::Mouse,
                "Teclado" => Tipo::Teclado,
                "Monitor" => Tipo::Monitor,
                _ => Tipo::Periferico,
            };
            let marca = match campos[4] {
                "HyperX" => Marca::HyperX,
                "Logitech" => Marca::Logitech,
                "Razer" => Marca::Razer,
                "Redragon" => Marca::Redragon,
                "Samsung" => Marca::Samsung,
                _ => Marca::SinEspecificar,
            };
            let descripcion = campos[5].to_string();
            let producto = Producto::new(nombre, precio, stock, tipo, marca, descripcion);
            productos.push(producto);
        }
    }
    Ok(productos)
}

fn registrar_producto(productos: &mut Vec<Producto>) -> Result<(), std::io::Error> {
    // Solicitar al usuario los datos del producto
    print!("Tipo (Audifonos/Mouse/Teclado/Monitor): ");
    io::stdout().flush()?;
    let mut tipo = String::new();
    io::stdin().read_line(&mut tipo)?;
    let tipo = match tipo.trim() {
        "Audifonos" => Tipo::Audifonos,
        "Mouse" => Tipo::Mouse,
        "Teclado" => Tipo::Teclado,
        "Monitor" => Tipo::Monitor,
        _ => Tipo::Audifonos, // Valor por defecto si el tipo no coincide
    };

    println!("Ingrese los datos del nuevo producto:");

    print!("Nombre: ");
    io::stdout().flush()?;
    let mut nombre = String::new();
    io::stdin().read_line(&mut nombre)?;

    print!("Marca (HyperX/Logitech/Samsung/Redragon): ");
    io::stdout().flush()?;
    let mut marca = String::new();
    io::stdin().read_line(&mut marca)?;
    let marca = match marca.trim() {
        "HyperX" => Marca::HyperX,
        "Logitech" => Marca::Logitech,
        "Samsung" => Marca::Samsung,
        "Redragon" => Marca::Redragon,
        _ => Marca::HyperX, // Valor por defecto si la marca no coincide
    };

    let precio:f64;
    loop{
        print!("Precio: ");
        io::stdout().flush()?;
        let mut precio_input = String::new();
        io::stdin().read_line(&mut precio_input)?;
        match precio_input.trim().parse::<f64>(){
            Ok(value) =>{
                precio = value;
                break;
            }
            Err(_) =>{
                println!("Error ingrese un valor númerico para el precio");
            }
        }
    }

    let stock:i32;
    loop{
        print!("Stock: ");
        io::stdout().flush()?;
        let mut stock_input = String::new();
        io::stdin().read_line(&mut stock_input)?;
        match stock_input.trim().parse::<i32>(){
            Ok(value) =>{
                stock = value;
                break;
            }
            Err(_)=>{
                println!("Error ingrese un valor númerico para el stock");
            }
        }
    }

    print!("Descripción: ");
    io::stdout().flush()?;
    let mut descripcion = String::new();
    io::stdin().read_line(&mut descripcion)?;

    // Crear una instancia de Producto con los datos ingresados
    let producto = Producto::new(nombre.trim().to_string(), precio, stock, tipo, marca, descripcion.trim().to_string());

    // Agregar el producto a la lista
    productos.push(producto);

    // Actualizar el archivo de texto
    let mut contenido = String::new();
    for producto in productos {
        contenido.push_str(&format!("{},{},{},{:?},{:?},{}\n", producto.nombre, producto.precio, producto.stock, producto.tipo, producto.marca, producto.descripcion));
    }
    fs::write("src/productos.txt", contenido)?;

    Ok(())
}

/*fn registrar_producto1(productos: &mut Vec<Producto>){
    loop{
        print!("Ingrese el tipo del producto: ");
        print!("1 : Audifonos");
        print!("2 : Mouse");
        print!("3 : Teclado");
        print!("4 : Monitor");
        let mut opcion_tipo = String::new();

        io::stdin()
            .read_line(&mut opcion_tipo)
            .expect("Error al leer la entrada");
        let opcion_tipo: u32 = match opcion_tipo.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        /*let tipo = match tipo.trim(){
            "Audifonos" => Tipo::Audifonos,
        }*/
        let tipo = match opcion_tipo {
            1 => Tipo::Audifonos,
            2 => Tipo::Mouse,
            3 => Tipo::Teclado,
            4 => Tipo::Monitor,
            _ => Tipo::Periferico,
        };

        print!("Ingrese el nombre del producto: ");
        let mut nombre = String::new();

        io::stdin()
            .read_line(&mut nombre)
            .expect("Error al leer la entrada");

        print!("Ingrese la marca del producto: ");
        print!("1 : Samsung");
        print!("2 : HyperX");
        print!("3 : Razer");
        print!("4 : Logitech");
        print!("5 : Redragon");
        let mut opcion_marca = String::new();

        io::stdin()
            .read_line(&mut opcion_marca)
            .expect("Error al leer la entrada");
        let opcion_marca: u32 = match opcion_marca.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let marca = match opcion_marca {
            1 => Marca::Samsung,
            2 => Marca::HyperX,
            3 => Marca::Razer,
            4 => Marca::Logitech,
            5 => Marca::Redragon,
            _ => Marca::SinEspecificar,
        };

        print!("Ingrese el precio del producto: ");
        let mut precio = String::new();
        io::stdin()
            .read_line(&mut precio)
            .expect("Error al leer la entrada");

        print!("Ingrese el stock: ");
        let mut stock = String::new();
        io::stdin()
            .read_line(&mut stock)
            .expect("Error al leer la entrada");

        print!("Ingrese una descripcion: ");
        let mut descripcion = String::new();
        io::stdin()
            .read_line(&mut descripcion)
            .expect("Error al leer la entrada");

        let producto = Producto::new(nombre, precio.parse().unwrap(), stock.parse().unwrap(), tipo, marca, descripcion);
        productos.push(producto);
    }

}*/

fn main(){
    let contenido = fs::read_to_string("src/productos.txt");
    let mut productos: Vec<Producto> = Vec::new();
    match contenido {

        Ok(contenido) => {

            cargar_producto(&mut productos);
            println!("Archivo leido exitosamente");
        }
        Err(e) =>{
            println!("Error al leer archivo: {}", e);
        }
    }
    loop {
        println!("Bienvenido al menú:");
        println!("(1) -> Registrar Producto");
        println!("(2) -> Eliminar Producto");
        println!("(3) -> Realizar Venta");
        println!("(4) -> Mostrar Productos");
        println!("(5) -> Salir");

        let mut opcion = String::new();

        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer la entrada");

        let opcion: u32 = match opcion.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match opcion {
            1 => {
                registrar_producto(&mut productos);
            }
            2 => {
                println!("seleccione el producto que desea eliminar");
            }
            3 => {
                println!("la venta se ha realizado con exito");
            }
            4 => {
                println!("Estos son los productos registrados");
                for producto in &productos {
                    println!("{:?}", producto.mostrar_producto());
                }
            }
            5 => {
                println!("Gracias por usar el programa");
                break;
            }
            _ => {
                println!("Opción no válida");
                continue;
            }

        }
    }

}