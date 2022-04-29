use std::env;

/*
Warum muss es eigentlich eine Referenz sein, bzw. warum muss ich ein neues String objekt erstellen,
liegt es dran weil ich dann den Scope verliere und darum da keine Referenz mehr haben werden, muss ich darum
ein neues Objekt erstellen damit es funktioniert, ich verstehe ich nicht genau, ebenfalls warum kann ich nur
die Referenzen benutzen von dem Objekt shrug

 */
pub fn get_path() -> String {
    let path: Vec<String> = env::args().collect();
    path[0].clone()
}