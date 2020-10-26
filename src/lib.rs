pub mod apuesta;
#[cfg(test)]
mod tests {
    use crate::apuesta::{Apuesta, Resultado};
    #[test]
    fn creacion_de_apuesta() {
        assert_eq!(
            Apuesta::new("Polopos", "Alhama", "2-3").unwrap(),
            Apuesta {
                club1: "Polopos".to_string(),
                club2: "Alhama".to_string(),
                resultado: Resultado {
                    goles_club1: 2,
                    goles_club2: 3,
                },
            }
        );
    }

    #[test]
    fn apuesta_to_string() {
        let apuesta = Apuesta::new("Polopos", "Alhama", "2-3").unwrap();
        assert_eq!(apuesta.to_string(), "Polopos: Alhama - 2-3");
    }

    #[test]
    fn parsear_resultado() {
        assert_eq!(
            " 3 - 1".parse::<Resultado>().unwrap(),
            Resultado {
                goles_club1: 3,
                goles_club2: 1
            }
        );
    }

    #[test]
    fn cambiar_apuesta() {
        let mut apuesta = Apuesta::new("Polopos", "Alhama", "2-3").unwrap();
        apuesta.cambiar_resultado("1-1").unwrap();
        assert_eq!(apuesta, Apuesta::new("Polopos", "Alhama", "1-1").unwrap());
    }

    #[test]
    fn comprobar_ganador() {
        let apuesta = Apuesta::new("Polopos", "Alhama", "2-3").unwrap();
        assert_eq!(apuesta.ganador(), "Alhama");
    }

    #[test]
    fn comprobar_empate() {
        let apuesta = Apuesta::new("Polopos", "Alhama", "1-1").unwrap();
        assert_eq!(apuesta.ganador(), "Empate");
    }
}
