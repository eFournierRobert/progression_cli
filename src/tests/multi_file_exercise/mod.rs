use crate::{question, structs::question::Question};

mod deserialize;

pub fn complete_json_multiple_files_single_language_exercise() -> &'static str {
    r#"
        {
            "data": {
                "id": "1",
                "attributes": {
                    "auteur": "Albert Einstein",
                    "description": "Ceci est une question simple avec un langage",
                    "niveau": "débutant",
                    "titre": "Appeler une fonction paramétrée",
                    "énoncé": "La fonction `salutations` affiche une salution autant de fois que la valeur reçue en paramètre. Utilisez-la pour faire afficher «Bonjour le monde!» autant de fois que le nombre reçu en entrée.",
                    "licence": "poétique"
                }
            },
            "included": [
                {
                    "type": "prog",
                    "attributes": {
                        "langage": "python",
                        "fichiers": {
                            "main.py": "code",
                            "salut.py": "code"
                        }
                    }
                }
            ]
        }"#
}

pub fn complete_json_multiple_files_multiple_languages_exercise() -> &'static str {
    r#"
        {
            "data": {
                "id": "1",
                "attributes": {
                    "auteur": "Albert Einstein",
                    "description": "Ceci est une question simple avec un langage",
                    "niveau": "débutant",
                    "titre": "Appeler une fonction paramétrée",
                    "énoncé": "La fonction `salutations` affiche une salution autant de fois que la valeur reçue en paramètre. Utilisez-la pour faire afficher «Bonjour le monde!» autant de fois que le nombre reçu en entrée.",
                    "licence": "poétique"
                }
            },
            "included": [
                {
                    "type": "prog",
                    "attributes": {
                        "langage": "python",
                        "fichiers": {
                            "main.py": "code",
                            "salut.py": "code"
                        }
                    }
                },
                {
                    "type": "prog",
                    "attributes": {
                        "langage": "python",
                        "fichiers": {
                            "main.py": "code",
                            "salut.py": "code"
                        }
                    }          
                } 
            ]
        }"#
}

pub fn minimalist_json_multiple_files_single_language_exercise() -> &'static str {
    r#"
        {
            "data": {
                "id": "1",
                "attributes": {
                    "auteur": null,
                    "description": null,
                    "niveau": null,
                    "titre": null,
                    "énoncé": null,
                    "licence": null
                }
            },
            "included": [
                {
                    "type": "prog",
                    "attributes": {
                        "langage": "code",
                        "fichiers": {
                            "fichier1": "code",
                            "fichier2": "code"
                        }
                    }
                }
            ]
        }"#
}
