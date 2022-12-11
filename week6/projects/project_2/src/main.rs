use std::io;
use colored::Colorize;

fn main(){

    let mut name:Vec<String> = Vec::new();

    println!("How many siblings do you have");

    let mut no_sib = String::new();
    io::stdin().read_line(&mut no_sib).expect("Failed to read input");
    let sib_no:i32 = no_sib.trim().parse().expect("Invalid input");

    if sib_no > 0{

        for project in 0..sib_no {
            
            let mut name = String::new();
            println!("What is Siblng {}'s name",project+1);
            io::stdin().read_line(&mut name).expect("Failed to read input");
            let sib_nm:String = name.trim().parse().expect("Invalid input");

            println!("what is their age");
            let mut age = String::new();
            io::stdin().read_line(&mut age).expect("Failed to read input");
            let sib_age:i32 = age.trim().parse().expect("Invalid input");

            if sib_age > 18{
                println!("is Siblng married type yes or no");
                let mut yes = "yes";
                let mut no = "no";
                let mut mard = String::new();
                io::stdin().read_line(&mut mard).expect("Failed to read input");
                let mard2:String = mard.trim().parse().expect("Invalid input");

                if mard2 == yes {
                    println!("Do you have any kids");
                    let mut yes = "yes";
                    let mut no = "no";
                    let mut kids = String::new();
                    io::stdin().read_line(&mut kids).expect("Failed to read input");
                    let kids2:String = kids.trim().parse().expect("Invalid input");

                    if kids2 == yes {
                        println!("where do they live");
                        let mut places = String::new();
                        io::stdin().read_line(&mut places).expect("Failed to read input");
                        let places2:String = places.trim().parse().expect("Invalid input");
                        println!("{} {}",format!("Details of sibling").green().underline(),project+1);
                        println!("Siblng name is: {}\nSibling age is: {}\nIs sibling married: {}\nDoes sibling have kids: {}\nSibling lives in: {}\n",sib_nm,sib_age,mard2,kids2,places2);
                    }
                    else if kids2 == no {
                        println!("{} {}",format!("Details of sibling").green().underline(),project+1);
                        println!("Siblng name is: {}\nSibling age is: {}\nIs sibling married: {}\nDoes sibling have kids: {}\n",sib_nm,sib_age,mard2,kids2);
                    }
                }
                else if mard2 == no {
                    println!("are you a student or worker");
                    let mut stdnt = "student";
                    let mut wrkr = "worker";
                    let mut ocup = String::new();
                    io::stdin().read_line(&mut ocup).expect("Failed to read input");
                    let ocup2:String = ocup.trim().parse().expect("Invalid input");

                    if ocup2 == stdnt{
                        println!("What uni does he attend");
                        let mut uni = "Pau";
                        let mut univ = String::new();
                        io::stdin().read_line(&mut univ).expect("Failed to read input");
                        let univ2:String = univ.trim().parse().expect("Invalid input");
                        
                        println!("What course are they studying");
                        let mut course = "computer sciences";
                        let mut crs = String::new();
                        io::stdin().read_line(&mut crs).expect("Failed to read input");
                        let crs2:String = crs.trim().parse().expect("Invalid input");
                        println!("{} {}",format!("Details of sibling").green().underline(),project+1);
                        println!("Siblng name is: {}\nSibling age is: {}\nIs sibling married: {}\nSibling is a {}\nSibling goes to {}\nSibling studies {}\n",sib_nm,sib_age,mard2,ocup2,univ2,crs2);
                    }
                    else if ocup2 == wrkr{
                        println!("{} {}",format!("Details of sibling").green().underline(),project+1);
                        println!("Siblng name is: {}\nSibling age is: {}\nIs sibling married: {}\nSibling is a {}\n",sib_nm,sib_age,mard2,ocup2);
                    }
                }
            }
            else if sib_age < 18{
                println!("Are they doing waec yes or no");
                let mut yes = "yes";
                let mut no = "no";
                let mut waec = String::new();
                io::stdin().read_line(&mut waec).expect("Failed to read input");
                let waec2:String = waec.trim().parse().expect("Invalid input");
                
                if waec2 == yes{
                    println!("what secondary school are they in");
                    let mut school = String::new();
                    io::stdin().read_line(&mut school).expect("Failed to read input");
                    let school2:String = school.trim().parse().expect("Invalid input");
                    println!("{} {}",format!("Details of sibling").green().underline(),project+1);
                    println!("Siblng name is: {}\nSibling age is: {}\nIs sibling doing waec: {}\nSibling is in {}\n",sib_nm,sib_age,waec2,school2);
                }
                else if waec2 == no{
                    println!("What is their current class level");
                    let mut grade = String::new();
                    io::stdin().read_line(&mut grade).expect("Failed to read input");
                    let grade2:String = grade.trim().parse().expect("Invalid input");
                    println!("{} {}",format!("Details of sibling").green().underline(),project+1);
                    println!("Siblng name is: {}\nSibling age is: {}\nIs sibling doing waec: {}\nSibling is in {}\n",sib_nm,sib_age,waec2,grade2);
                }
            }
        }
    }
}
