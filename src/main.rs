use postgres::{Client, Error, NoTls};

struct TipoTramite {    
    description: String,    
    status: String,
    code: i32,
    book_code: i16,
    weight: i16,
    category: i16,
    code_web: i16,
}

fn main() -> Result<(), Error> {

    let mut client = Client::connect("postgresql://<user>:<password>@<host>:<port>/<database>", NoTls)?;    

    for row in client.query("SELECT ttr_des, ttr_est, ttr_cod, ttr_lib, ttr_pes, cat_id, ttr_codweb 
                            FROM registral.tipotramites  
                            WHERE ttr_codweb > 0  
                            ORDER BY ttr_des",&[],)? {
        
        let (description, status, code, book_code, weight, category, code_web): 
        (Option<String>, Option<String>, Option<i32>, Option<i16>, Option<i16>, Option<i16>, Option<i16>) = 
        (row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5), row.get(6));

        if description.is_some() && status.is_some()
        && code.is_some() && book_code.is_some()
        && weight.is_some() && category.is_some()
        && code.is_some() {
            let tipo_tramite = TipoTramite {

                description: description.unwrap(),    
                status: status.unwrap(),
                code: code.unwrap(),
                book_code: book_code.unwrap(),
                weight: weight.unwrap(),
                category: category.unwrap(),
                code_web: code_web.unwrap(),
            };
            println!("description: {}, status: {},  code: {}, book_code: {}, weight: {}, category: {}, code_web: {}", tipo_tramite.description, 
                    tipo_tramite.status, tipo_tramite.code,
                    tipo_tramite.book_code, tipo_tramite.weight, tipo_tramite.category, tipo_tramite.code_web);
        }
    }

    Ok(())
}
