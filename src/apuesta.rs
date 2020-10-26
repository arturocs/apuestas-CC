use std::{error::Error, fmt, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct Resultado {
    pub(crate) goles_club1: u8,
    pub(crate) goles_club2: u8,
}
#[derive(Debug, PartialEq)]
pub struct Apuesta {
    pub(crate) club1: String,
    pub(crate) club2: String,
    pub(crate) resultado: Resultado,
}

impl fmt::Display for Apuesta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} - {}-{}",
            self.club1, self.club2, self.resultado.goles_club1, self.resultado.goles_club2
        )
    }
}

impl FromStr for Resultado {
    type Err = Box<dyn Error>;

    fn from_str(resultado: &str) -> Result<Self, Self::Err> {
        let goles = resultado
            .split("-")
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<_>, _>>()?;
        if goles.len() == 2 {
            Ok(Resultado {
                goles_club1: goles[0],
                goles_club2: goles[1],
            })
        } else {
            Err("Formato del resultado incorrecto".into())
        }
    }
}

impl Apuesta {
    pub fn new(club1: &str, club2: &str, resultado: &str) -> Result<Apuesta, Box<dyn Error>> {
        Ok(Apuesta {
            club1: club1.to_string(),
            club2: club2.to_string(),
            resultado: resultado.parse()?,
        })
    }
    pub fn cambiar_resultado(&mut self, resultado: &str) -> Result<(), Box<dyn Error>> {
        self.resultado = resultado.parse()?;
        Ok(())
    }

    pub fn ganador(&self) -> &str {
        if self.resultado.goles_club1 > self.resultado.goles_club2 {
            &self.club1
        } else if self.resultado.goles_club1 < self.resultado.goles_club2 {
            &self.club2
        } else {
            "Empate"
        }
    }
}
