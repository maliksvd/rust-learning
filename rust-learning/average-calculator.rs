use std::io;

fn main() {
    // Demander à l'utilisateur d'entrer une liste de nombres séparés par des espaces
    println!("Entrez une liste de nombres séparés par des espaces :");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Echec de la lecture de l'entrée");

    // Fractionner la chaîne d'entrée en une liste de nombres
    let numbers: Vec<f64> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    // Vérifier si la liste est vide
    if numbers.is_empty() {
        println!("La liste est vide !");
        return;
    }

    // Calculer la somme des nombres dans la liste
    let sum: f64 = numbers.iter().sum();

    // Calculer la moyenne des nombres
    let average = sum / numbers.len() as f64;

    // Afficher la moyenne
    println!("La moyenne des nombres est : {}", average);
}
