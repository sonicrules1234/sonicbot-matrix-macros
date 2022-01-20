use proc_macro::TokenStream;

#[proc_macro]
pub fn load_modules(item: TokenStream) -> TokenStream {
    let essential: String;
    let event: String;
    let mut item_iter = item.into_iter();
    if let proc_macro::TokenTree::Ident(y) = item_iter.next().unwrap() {
        essential = y.to_string();
    } else {
        panic!("essential is not an ident");
    }
    item_iter.next().unwrap();
    if let proc_macro::TokenTree::Ident(y) = item_iter.next().unwrap() {
        event = y.to_string();
    } else {
        panic!("event is not an ident");
    }
    let mut module_list: Vec<String> = Vec::new();
    let plugin_or_essential: String;
    let event_or_message: (String, String);
    if essential.as_str() == "true" {
        plugin_or_essential = String::from("essentials");
    } else {
        plugin_or_essential = String::from("plugins");
    }
    if event.as_str() == "true" {
        event_or_message = (String::from("on_event"), String::from("SonicBotEventModule"));
    } else {
        event_or_message = (String::from("on_message"), String::from("SonicBotMessageModule"));
    }
    let potentialfilesresults = glob::glob(format!("src/{}/{}/*.rs", plugin_or_essential.clone(), event_or_message.0.clone()).as_str()).unwrap();//.collect::<glob::GlobResult>().unwrap();
    let mut code = String::new();
    if event.as_str() == "true" {
        code.push_str("use crate::SonicBotEventModule;\n");
    } else {
        code.push_str("use crate::SonicBotMessageModule;\n");
    }
    for pfr in potentialfilesresults {
        let file_name: String = pfr.unwrap().as_path().file_name().unwrap().to_str().unwrap().to_string();
        let module_name = file_name.split(".").collect::<Vec<&str>>()[0];
        if module_name != "mod" {
            module_list.push(module_name.to_string());
        }
    }
    module_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    for module_name in module_list.clone() {
        code.push_str(format!("pub mod {};\n", module_name).as_str());
    }
    code.push_str(format!("pub fn get_{}_{}<'a>() -> Vec<{}<'a>> ", plugin_or_essential.clone(), event_or_message.0.clone(), event_or_message.1.clone()).as_str());
    code.push('{');
    code.push_str(format!("\n    let mut module_vec: Vec<crate::{}> = Vec::new();", event_or_message.1.clone()).as_str());
    for module_name in module_list.clone() {
        code.push_str(format!("\n    module_vec.push(crate::{} ", event_or_message.1.clone()).as_str());
        code.push_str("{\n");
        code.push_str(r#"        name: String::from(""#);
        code.push_str(module_name.as_str());
        code.push('"');
        code.push_str(format!("),\n        essential: {},\n        main: Box::new({}::main),\n        help: {}::help()\n    ", essential, module_name, module_name).as_str());
        code.push_str("});");
    }
    code.push_str("\n    module_vec\n}");
    //println!("{}, {}, \n{}", plugin_or_essential, event_or_message.0, code.clone());
    code.as_str().parse().unwrap()
}
