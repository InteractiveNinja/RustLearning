use std::env;

/*
Warum muss es eigentlich eine Referenz sein, bzw. warum muss ich ein neues String objekt erstellen,
liegt es dran weil ich dann den Scope verliere und darum da keine Referenz mehr haben werden, muss ich darum
ein neues Objekt erstellen damit es funktioniert, ich verstehe ich nicht genau, ebenfalls warum kann ich nur
die Referenzen benutzen von dem Objekt shrug

 */
pub fn get_path() -> String {
    let path: Vec<String> = env::args().collect();
    format_path(path[0].split("/").collect())
}

fn format_path(path: Vec<&str>) -> String {
    let arr_size = path.len();
    let sector = &path[0..=arr_size - 2];
    let mut full_path = String::from("");
    for i in 0..sector.len() {
        if i == 0 {
            full_path.push_str(sector[i]);
            continue;
        }
        full_path.push_str(&format!("/{}",sector[i]));
    }

    full_path
}
