//Property of God'sgift David Ifeanyi 

use std::io::Write;
use std::fs::File;
use std::io;
use colored::Colorize;
use ansi_term::Color;
use ansi_term::Style;

struct Staff {
    name: String,
    department: String,
    services: Vec<String>,

}

fn code_7(staff: &Staff){

    let mut file = File::create(format!("{}.txt",staff.name)).unwrap();

    let services_string = staff.services.join(",");
    let file_contents = format!("Department: {}\nServices:{}\n",staff.department, services_string);

    file.write_all(file_contents.as_bytes()).unwrap();         
}

fn code_8(staff: &Staff){
    let mut file = File::create(format!("{}.txt",staff.name)).unwrap();

    let services_string = staff.services.join(",");
    let file_contents = format!("Department: {}\nServices:{}\n",staff.department, services_string);

    file.write_all(file_contents.as_bytes()).unwrap();  
}

fn code_9(staff: &Staff){
    let mut file = File::create(format!("{}.txt",staff.name)).unwrap();

    let services_string = staff.services.join(",");
    let file_contents = format!("Department: {}\nServices:{}\n",staff.department, services_string);

    file.write_all(file_contents.as_bytes()).unwrap();  
}

fn codes7(){

    let staff = vec![

        Staff {
        name:"Juliet Aigbona".to_string(),
        department: "Consulting".to_string(),
        services: vec![
            "Analytics consulting services".to_string(),
            "\nCostumer experience".to_string(),
            "\nCybersevurity".to_string(),
            "\nRisk".to_string(),
            "\nCompliance and Resilience".to_string(),
            "\nDigital transformation".to_string(),
            "\nRisk consulting services".to_string(),
            "\nSupply chain and Operations".to_string(),
            "\nTechnology transformation".to_string(),
            ],
    },
        Staff {
        name:"Ilika Akpevwe".to_string(),
        department: "Assurance".to_string(),
        services: vec![
            "Audit services".to_string(),
            "\nClimate change and sustainabilty services".to_string(),
            "\nFinancial accounting advisory services".to_string(),
            "\nForensic and integrity services".to_string(),
            "\nPrivate client audit experience".to_string(),
            "\nAccounting Link".to_string(),
            "\nAssurance".to_string(),
            ],
    },


        ];

    code_7(&staff[0]);
    code_7(&staff[1]);
}

fn codes8(){

    let staff = vec![

        Staff {
        name:"Sagamu Adamu".to_string(),
        department: "Tax".to_string(),
        services: vec![
            "Tax planning".to_string(),
            "\nTax funcyion operations".to_string(),
            "\nTax policy and Controversy".to_string(),
            "\nGlobal trade".to_string(),
            "\nTax accounting".to_string(),
            "\nTax compliance".to_string(),
            "\nTransaction tax".to_string(),
            ],
    },
        Staff {
        name:"Daniels Gbenga".to_string(),
        department: "People and workforce".to_string(),
        services: vec![
            "Change management and experience".to_string(),
            "\nHR Transformation".to_string(),
            "\nIntegrated workforce mobility".to_string(),
            "\nLearning and development consulting".to_string(),
            "\nRecognition and reward advisory".to_string(),
            "\nWorkforce analytics".to_string(),
            "\nPeople and workforce".to_string(),
            ],
    },


        ];

    code_8(&staff[0]);
    code_8(&staff[1]);
}

fn codes9(){

    let staff = vec![

        Staff {
        name:"Ero Ehis".to_string(),
        department: "Strategy".to_string(),
        services: vec![
            "Strategy consulting".to_string(),
            "\nCoperate and growth strategy".to_string(),
            "\nTransaction strategy and execution".to_string(),
            "\nRestructuring and turnaround strategy".to_string(),
            "\nIndustry strategy".to_string(),
            "\nDigital business building".to_string(),
            "\nCommercial strategy".to_string(),
            ],
    },
        Staff {
        name:"Akinsola Maria".to_string(),
        department: "Transactions and Coperate finance".to_string(),
        services: vec![
            "Corporate finance".to_string(),
            "\nDivestments and carve-outs".to_string(),
            "\nSustaianability and ESG Services".to_string(),
            "\nM&A advisory".to_string(),
            "\nM&A integration".to_string(),
            "\nM&A technology and tools".to_string(),
            "\nM&A advanced analytics".to_string(),
            ],
    },


        ];

    code_9(&staff[0]);
    code_9(&staff[1]);
}

fn main() {

    println!("would you like to restruct the staff data\nYes or No");
            let mut code = String::new();
            io::stdin().read_line(&mut code).expect("Failed to read input");
            let choice:String = code.trim().parse().expect("Invalid input");

            let yes = "yes";
            let no = "no";

    if choice == yes {
        codes7();
        codes8();
        codes9();
        println!("{}",format!("Data files have been created").bold().green().underline());
    }
    else if choice == no{
        println!("Okay Then");
    }
    else {
        println!("{}",format!("Error").bold().red().underline());
    }
}
