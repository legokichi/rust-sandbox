use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stars = vec![
        ("Polaris", ((02, 31, 49.09456), (1, 89, 15, 50.7923)), 2.02),
        ("Kochab", ((14, 50, 42.32580), (1, 74, 09, 19.8142)), 2.08),
        (
            "Dubhe",
            // "alf UMa",
            ((11, 03, 43.67152), (1, 61, 45, 03.7249)),
            1.79,
        ),
        (
            "Caph",
            // "bet Cas",
            ((00, 09, 10.68518), (1, 59, 08, 59.2120)),
            2.27,
        ),
        (
            "Merak",
            // "bet UMa",
            ((11, 01, 50.47654), (1, 56, 22, 56.7339)),
            2.37,
        ),
        (
            "Alioth",
            // "eps UMa",
            ((12, 54, 01.74959), (1, 55, 57, 35.3627)),
            1.77,
        ),
        (
            "Schedar",
            // "alf Cas",
            ((00, 40, 30.4410679), (1, 56, 32, 14.39217)),
            2.23,
        ),
        ("Mizar", ((13, 23, 55.54048), (1, 54, 55, 31.2671)), 2.27),
        (
            "Mirfak",
            // "alf Per",
            ((03, 24, 19.37009), (1, 49, 51, 40.2455)),
            1.79,
        ),
        (
            "Alkaid",
            // "Benetnasch",
            // "eta UMa",
            ((13, 47, 32.43776), (1, 49, 18, 47.7602)),
            1.86,
        ),
        ("Capella", ((05, 16, 41.35871), (1, 45, 59, 52.7693)), 0.08),
        ("Deneb", ((20, 41, 25.91514), (1, 45, 16, 49.2197)), 1.25),
        ("Vega", ((18, 36, 56.33635), (1, 38, 47, 01.2802)), 0.03),
        ("Castor", ((07, 34, 35.87319), (1, 31, 53, 17.8160)), 1.58),
        (
            "Alpheratz",
            // "alf And",
            ((00, 08, 23.25988), (1, 29, 05, 25.5520)),
            2.06,
        ),
        (
            "Pollux",
            // "bet Gem",
            ((07, 45, 18.94987), (1, 28, 01, 34.3160)),
            1.14,
        ),
        (
            "Alphecca",
            // "Gemma",
            ((15, 34, 41.26800), (1, 26, 42, 52.8940)),
            2.24,
        ),
        (
            "Arcturus",
            // "alf Boo",
            ((14, 15, 39.67207), (1, 19, 10, 56.6730)),
            -0.05,
        ),
        (
            "Aldebaran",
            // "alf Tau",
            ((04, 35, 55.23907), (1, 16, 30, 33.4885)),
            0.86,
        ),
        (
            // "Markab",
            "kap Vel",
            ((09, 22, 06.79945), (-1, 55, 00, 38.1847)),
            2.48,
        ),
        (
            "Denebola",
            // "bet Leo",
            ((11, 49, 03.57834), (1, 14, 34, 19.4090)),
            2.13,
        ),
        (
            "Rasalhague",
            // "alf Oph",
            ((17, 34, 56.06945), (1, 12, 33, 36.1346)),
            2.07,
        ),
        (
            "Regulus",
            // "alf Leo",
            ((10, 08, 22.31099), (1, 11, 58, 01.9516)),
            1.40,
        ),
        (
            "Altair",
            // "alf Aql",
            ((19, 50, 46.99855), (1, 08, 52, 05.9563)),
            0.76,
        ),
        (
            "Betelgeuse",
            ((05, 55, 10.30536), (1, 07, 24, 25.4304)),
            0.42,
        ),
        (
            "Bellatrix",
            ((05, 25, 07.86325), (1, 06, 20, 58.9318)),
            1.64,
        ),
        (
            "Procyon",
            // "alf CMi",
            ((07, 39, 18.11950), (1, 05, 13, 29.9552)),
            0.37,
        ),
        (
            "Rigel",
            // "bet Ori",
            ((05, 14, 32.27210), (-1, 08, 12, 05.8981)),
            0.13,
        ),
        (
            "Alphard",
            // "alf Hya",
            ((09, 27, 35.24270), (-1, 08, 39, 30.9583)),
            1.97,
        ),
        (
            "Spica",
            // "alf Vir",
            ((13, 25, 11.57937), (-1, 11, 09, 40.7501)),
            0.97,
        ),
        (
            "Sirius",
            // "alf CMa",
            ((06, 45, 08.91728), (-1, 16, 42, 58.0171)),
            -1.46,
        ),
        (
            "Diphda",
            // "bet Cet",
            ((00, 43, 35.37090), (-1, 17, 59, 11.7827)),
            2.01,
        ),
        (
            "Antares",
            // "alf Sco",
            ((16, 29, 24.45970), (-1, 26, 25, 55.2094)),
            0.91,
        ),
        (
            // "Nunki",
            "sig Sgr",
            ((18, 55, 15.92650), (-1, 26, 17, 48.2068)),
            2.067,
        ),
        (
            "Fomalhaut",
            ((22, 57, 39.04625), (-1, 29, 37, 20.0533)),
            1.16,
        ),
        (
            // "Shaula",
            "lam Sco",
            ((17, 33, 36.52012), (-1, 37, 06, 13.7648)),
            1.62,
        ),
        (
            "Canopus",
            // "alf Car",
            ((06, 23, 57.10988), (-1, 52, 41, 44.3810)),
            -0.74,
        ),
        (
            "Peacock",
            // "alf Pav",
            ((20, 25, 38.85705), (-1, 56, 44, 06.3230)),
            1.918,
        ),
        (
            // "alf Eri",
            "Achernar",
            ((01, 37, 42.84548), (-1, 57, 14, 12.3101)),
            0.46,
        ),
        (
            "Mimosa",
            // "bet Cru",
            ((12, 47, 43.26877), (-1, 59, 41, 19.5792)),
            1.25,
        ),
        (
            // "Hadar",
            // "Agena",
            "bet Cen",
            ((14, 03, 49.40535), (-1, 60, 22, 22.9266)),
            0.60,
        ),
        (
            // "Rigil Kentaurus",
            "alf Cen",
            ((14, 39, 36.204), (-1, 60, 50, 08.23)),
            0.01,
        ),
        (
            // "alf Cru",
            "Acrux",
            ((12, 26, 35.89522), (-1, 63, 05, 56.7343)),
            1.28,
        ),
        (
            "Atria",
            // "alf TrA",
            ((16, 48, 39.89508), (-1, 69, 01, 39.7626)),
            1.92,
        ),
        (
            "Miaplacidus",
            // "bet Car",
            ((09, 13, 11.97746), (-1, 69, 43, 01.9473)),
            1.69,
        ),
        (
            // "Adhara",
            "eps CMa",
            ((06, 58, 37.54876), (-1, 28, 58, 19.5102)),
            1.50,
        ),
        (
            // "Gacrux",
            "gam Cru",
            ((12, 31, 09.95961), (-1, 57, 06, 47.5684)),
            1.64,
        ),
        (
            "Elnath",
            // "bet Tau",
            ((05, 26, 17.51312), (1, 28, 36, 26.8262)),
            1.65,
        ),
        (
            //"Alnilam",
            "eps Ori",
            ((05, 36, 12.81335), (-1, 01, 12, 06.9089)),
            1.69,
        ),
        (
            "Alnair",
            // "alf Gru",
            ((22, 08, 13.98473), (-1, 46, 57, 39.5078)),
            1.71,
        ),
        (
            // "Al Suhail al-Muhlif",
            // "Regor",
            "gam Vel",
            ((08, 09, 31.95013), (-1, 47, 20, 11.7108)),
            1.83,
        ),
        (
            //"Alnitak",
            "zet Ori",
            ((05, 40, 45.52666), (-1, 01, 56, 33.2649)),
            1.79,
        ),
        (
            // "Wezen",
            "del CMa",
            ((07, 08, 23.4805188773), (-1, 26, 23, 35.460726261)),
            1.84,
        ),
        (
            // "Kaus Australis",
            "eps Sgr",
            ((18, 24, 10.31840), (-1, 34, 23, 04.6193)),
            1.85,
        ),
        (
            "tet Sco",
            // "Sargas",
            ((17, 37, 19.12985), (-1, 42, 59, 52.1808)),
            1.862,
        ),
        (
            "Menkalinan",
            // "bet Aur",
            ((05, 59, 31.72293), (1, 44, 56, 50.7573)),
            1.90,
        ),
        (
            // "gam Gem",
            "Alhena",
            ((06, 37, 42.71050), (1, 16, 23, 57.4095)),
            1.92,
        ),
        (
            // "Alsephina",
            "del Vel",
            ((08, 44, 42.22658), (-1, 54, 42, 31.7493)),
            1.95,
        ),
        (
            "eps Car",
            // "Avior",
            ((08, 22, 30.83526), (-1, 59, 30, 34.1431)),
            1.953,
        ),
        (
            "Mirzam",
            // "bet CMa",
            ((06, 22, 41.9813538058), (-1, -17, 57, 21.275324448)),
            1.97,
        ),
        (
            // "alf Ari",
            "Hamal",
            ((02, 07, 10.40570), (1, 23, 27, 44.7032)),
            2.01,
        ),
        (
            // "tet Cen",
            "Menkent",
            ((14, 06, 40.94752), (-1, 36, 22, 11.8371)),
            2.05,
        ),
        (
            "Mirach",
            // "bet And",
            ((01, 09, 43.92388), (1, 35, 37, 14.0075)),
            2.05,
        ),
        (
            // "Saiph",
            "kap Ori",
            ((05, 47, 45.3911118311), (-1, 09, 40, 10.580841979)),
            2.06,
        ),
        (
            // "gam And",
            "Almach",
            ((02, 03, 53.95229), (1, 42, 19, 47.0223)),
            2.10,
        ),
        (
            "bet Gru",
            // "Tiaki",
            ((22, 42, 40.05027), (-1, 46, 53, 04.4752)),
            2.11,
        ),
        (
            // "bet Per",
            "Algol",
            ((03, 08, 10.13245), (1, 40, 57, 20.3280)),
            2.12,
        ),
        (
            "gam Cen",
            // "Muhlifain",
            ((12, 4, 31.04008), (-1, 48, 57, 35.5375)),
            2.17,
        ),
        (
            "lam Vel",
            // "Suhail",
            ((09, 07, 59.75787), (-1, 43, 25, 57.3273)),
            2.21,
        ),
        (
            "Etamin",
            // "gam Dra",
            ((17, 56, 36.36988), (1, 51, 29, 20.0242)),
            2.230,
        ),
        (
            // "Sadr",
            "gam Cyg",
            ((20, 22, 13.70184), (1, 40, 15, 24.0450)),
            2.23,
        ),
        (
            // "Naos",
            "zet Pup",
            ((08, 03, 35.04754), (-1, 40, 00, 11.3321)),
            2.25,
        ),
        (
            " iot Car",
            // "Aspidiske",
            ((09, 17, 05.40686), (-1, 59, 16, 30.8353)),
            2.26,
        ),
        (
            "alf Lup",
            ((14, 41, 55.75579), (-1, 47, 23, 17.5155)),
            2.286,
        ),
        (
            // "Larawag",
            "eps Sco",
            ((16, 50, 09.82023), (-1, 34, 17, 35.6590)),
            2.29,
        ),
        (
            "eps Cen",
            // "Birdun",
            ((13, 39, 53.25774), (-1, 53, 27, 59.0081)),
            2.30,
        ),
        (
            "eta Cen",
            ((14, 35, 30.3744634832), (-1, 42, 09, 28.673614318)),
            2.31,
        ),
        (
            // "Dschubba",
            "del Sco",
            ((16, 00, 20.00528), (-1, 22, 37, 18.1431)),
            2.32,
        ),
        (
            // "gam Leo",
            "Algieba",
            ((10, 19, 58.35056), (1, 19, 50, 29.3468)),
            2.01,
        ),
        (
            "Ankaa",
            // "alf Phe",
            ((00, 26, 17.05140), (-1, 42, 18, 21.5539)),
            2.37,
        ),
        (
            "kap Sco",
            // "Girtab",
            ((17, 42, 29.27520), (-1, 39, 01, 47.9391)),
            2.386,
        ),
        (
            "eps Boo",
            // "Izar",
            ((14, 44, 59.21746), (1, 27, 04, 27.2099)),
            2.39,
        ),
        (
            // "eps Peg",
            "Enif",
            ((21, 44, 11.15614), (1, 09, 52, 30.0311)),
            2.39,
        ),
        (
            // "gam Cas",
            "Tsih",
            ((00, 56, 42.5317), (1, 60, 43, 00.265)),
            2.39,
        ),
        (
            "del Ori",
            // "Mintaka",
            ((05, 32, 00.40009), (-1, 00, 17, 56.7424)),
            2.23,
        ),
        (
            "Scheat",
            // "bet Peg",
            ((23, 03, 46.45746), (1, 28, 04, 58.0336)),
            2.42,
        ),
        (
            "eta Oph",
            // "Sabik",
            ((17, 10, 22.68689), (-1, 15, 43, 29.6639)),
            2.42,
        ),
        (
            // "gam UMa",
            "Phecda",
            ((11, 53, 49.84732), (1, 53, 41, 41.1350)),
            2.440,
        ),
        (
            // "del UMa",
            "Megrez",
            ((12, 15, 25.56063), (1, 57, 01, 57.4156)),
            3.320,
        ),
        (
            // "Aludra",
            "eta CMa",
            ((07, 24, 05.70228), (-1, 29, 18, 11.1798)),
            2.45,
        ),
        (
            "Alderamin",
            // "alf Cep",
            ((21, 18, 34.77233), (1, 62, 35, 08.0681)),
            2.460,
        ),
        (
            "Aljanah",
            // "eps Cyg",
            ((20, 46, 12.68236), (1, 33, 58, 12.9250)),
            2.480,
        ),
        (
            // "alf Peg",
            "Markab",
            ((23, 04, 45.65345), (1, 15, 12, 18.9617)),
            2.48,
        ),
        (
            "Albireo",
            // "bet Cyg A",
            ((19, 30, 43.28052), (1, 27, 57, 34.8483)),
            3.085,
        ),
        (
            // "gam Peg",
            "Algenib",
            ((00, 13, 14.15123), (1, 15, 11, 00.9368)),
            2.84,
        ),
        (
            "Ruchbah",
            // "del Cas",
            ((01, 25, 48.95147), (1, 60, 14, 07.0225)),
            2.680,
        ),
        (
            "Segin",
            // "eps Cas",
            ((01, 54, 23.7261769782), (1, 63, 40, 12.372216932)),
            3.37,
        ),
        (
            "Menkar",
            // "alf Cet",
            ((03, 02, 16.77307), (1, 04, 05, 23.0596)),
            2.53,
        ),
        (
            "alf02 CVn",
            // "Cor Caroli",
            ((12, 56, 01.66622), (1, 38, 19, 06.1541)),
            2.88,
        ),
        (
            "del Cru",
            // "Imai",
            ((12, 15, 08.7189075373), (-1, 58, 44, 56.150111429)),
            2.752,
        ),
        (
            "del Cap",
            // "Deneb Algedi",
            ((21, 47, 02.44424), (-1, 16, 07, 38.2335)),
            2.83,
        ),
        (
            "bet Cap",
            // "Dabih",
            ((20, 21, 00.67326), (-1, 14, 46, 52.9791)),
            3.08,
        ),
        (
            "gam Gru",
            // "Aldhanab",
            ((21, 53, 55.7161028973), (-1, 37, 21, 53.550288216)),
            3.01,
        ),
        ("bet TrA", ((15, 55, 08.56206), (-1, 63, 25, 50.6155)), 2.85),
        (
            "bet Aqr",
            // "Sadalsuud",
            ((21, 31, 33.53171), (-1, 05, 34, 16.2320)),
            2.89,
        ),
        (
            "alf Aqr",
            // "Sadalmelik",
            ((22, 05, 47.036), (-1, 00, 19, 11.46)),
            2.94,
        ),
        (
            "Skat",
            // "del Aqr",
            ((22, 54, 39.01351), (-1, 15, 49, 14.9782)),
            3.28,
        ),
        (
            "omi Cet",
            // "Mira",
            ((02, 19, 20.79210), (-1, 02, 58, 39.4956)),
            3.04,
        ),
        (
            "bet Sco",
            // "Acrab",
            ((16, 05, 26.2), (-1, 19, 48, 10.0)),
            2.50,
        ),
        (
            "del Leo",
            // "Zosma",
            ((11, 14, 06.50142), (1, 20, 31, 25.3853)),
            2.53,
        ),
        (
            "alf Lep",
            ((05, 32, 43.81612), (-1, 17, 49, 20.2414)),
            // "Arneb",
            2.57,
        ),
        (
            "gam Crv",
            // "Gienah",
            ((12, 15, 48.37081), (-1, 17, 32, 30.9496)),
            2.58,
        ),
        (
            // "Ascella",
            "zet Sgr",
            ((19, 02, 36.73024), (-1, 29, 52, 48.2279)),
            2.607,
        ),
        (
            // "Zubeneschamali",
            "bet Lib",
            ((15, 17, 00.41382), (-1, 09, 22, 58.4919)),
            2.62,
        ),
        (
            "tet Aur",
            // "Mahasim",
            ((05, 59, 43.27012), (1, 37, 12, 45.3047)),
            2.62,
        ),
        (
            // "Unukalhai",
            "alf Ser",
            ((15, 44, 16.07431), (1, 06, 25, 32.2633)),
            2.630,
        ),
        (
            // "Kraz",
            "bet Crv",
            ((12, 34, 23.22764), (-1, 23, 23, 48.4475)),
            2.64,
        ),
        (
            // "Phact",
            "alf Col",
            ((05, 39, 38.9410319), (-1, 34, 04, 26.794991)),
            2.65,
        ),
        (
            // "Sheratan",
            "bet Ari",
            ((01, 54, 38.41099), (1, 20, 48, 28.9133)),
            2.65,
        ),
        (
            // "Kaus Media",
            "del Sgr",
            ((18, 20, 59.64354), (-1, 29, 49, 41.1659)),
            2.668,
        ),
        (
            "alf Mus",
            ((12, 37, 11.01789), (-1, 69, 08, 08.0332)),
            2.677,
        ),
        (
            // "Muphrid",
            "eta Boo",
            ((13, 54, 41.07892), (1, 18, 23, 51.7946)),
            2.680,
        ),
        (
            "iot Aur",
            // "Hassaleh",
            ((04, 56, 59.62109), (1, 33, 09, 57.9585)),
            2.69,
        ),
        ("mu Vel", ((10, 46, 46.17877), (-1, 49, 25, 12.9244)), 2.69),
        ("pi Pup", ((07, 17, 08.55678), (-1, 37, 05, 50.8962)), 2.70),
        (
            // "Lesath",
            "ups Sco",
            ((17, 30, 45.83712), (-1, 37, 17, 44.9285)),
            2.70,
        ),
        (
            // "Tarazed",
            "gam Aql",
            ((19, 46, 15.58029), (1, 10, 36, 47.7408)),
            2.72,
        ),
        (
            "iot Cen",
            // "Alhakim",
            ((13, 20, 35.81737), (-1, 36, 42, 44.2447)),
            2.73,
        ),
        (
            "gam Vir",
            // "Porrima",
            ((12, 41, 39.64344), (-1, 01, 26, 57.7421)),
            2.74,
        ),
        (
            // "Athebyne",
            "eta Dra",
            ((16, 23, 59.48594), (1, 61, 30, 51.1699)),
            2.74,
        ),
        (
            // "Yed Prior",
            "del Oph",
            ((16, 14, 20.73853), (-1, 03, 41, 39.5612)),
            2.75,
        ),
        ("tet Car", ((10, 42, 57.40197), (-1, 64, 23, 40.0208)), 2.76),
        (
            "gam lup",
            ((15, 35, 08.44835), (-1, 41, 10, 00.3247)),
            2.765,
        ),
        ("bet Lup", ((14, 58, 31.92536), (-1, 43, 08, 02.2699)), 2.68),
        (
            // "Kornephoros",
            "bet Her",
            ((16, 30, 13.19955), (1, 21, 29, 22.6008)),
            2.77,
        ),
        (
            // "Hatysa",
            "iot Ori",
            ((05, 35, 25.9831233810), (-1, 05, 54, 35.620268233)),
            2.77,
        ),
        (
            // "Cebalrai",
            "bet Oph",
            ((17, 43, 28.35265), (1, 04, 34, 02.2955)),
            2.78,
        ),
        (
            // "Cursa",
            "bet Eri",
            ((05, 07, 50.98549), (-1, 05, 05, 11.2055)),
            2.79,
        ),
        (
            // "Vindemiatrix",
            "eps Vir",
            ((13, 02, 10.5932205584), (1, 10, 57, 32.904222160)),
            2.79,
        ),
        ("bet Hyi", ((00, 25, 45.07036), (-1, 77, 15, 15.2860)), 2.79),
        (
            "alf Lib",
            // "Zubenelgenubi",
            ((14, 50, 41.18097), (-1, 15, 59, 50.0482)),
            2.8,
        ),
        //////
        ("zet Oph", ((16, 37, 09.53905), (-1, 10, 34, 01.5295)), 2.56),
        ("zet Cen", ((13, 55, 32.38565), (-1, 47, 17, 18.1482)), 2.55),
        ("del Cen", ((12, 08, 21.49764), (-1, 50, 43, 20.7386)), 2.52),
        ("gam TrA", ((15, 18, 54.58198), (-1, 68, 40, 46.3654)), 2.89),
        ("Sgr A*", ((17, 45, 40.03599), (-1, 29, 0, 28.1699)), 3.0),
        ("0", ((0, 0, 0.0), (1, 0, 0, 0.0)), 3.0),
        ("3", ((3, 0, 0.0), (1, 0, 0, 0.0)), 3.0),
        // ("6", ((6, 0, 0.0), (1, 0, 0, 0.0)), 3.0),
        ("9", ((9, 0, 0.0), (1, 0, 0, 0.0)), 3.0),
        ("12", ((12, 0, 0.0), (1, 0, 0, 0.0)), 3.0),
        ("15", ((15, 0, 0.0), (1, 0, 0, 0.0)), 3.0),
        ("18", ((18, 0, 0.0), (1, 0, 0, 0.0)), 3.0),
        ("21", ((21, 0, 0.0), (1, 0, 0, 0.0)), 3.0),
        ("24", ((24, 0, 0.0), (1, 0, 0, 0.0)), 3.0),
        ("90", ((12, 0, 0.0), (1, 90, 0, 0.0)), 3.0),
        ("75", ((12, 0, 0.0), (1, 75, 0, 0.0)), 3.0),
        // ("60", ((12, 0, 0.0), (1, 60, 0, 0.0)), 3.0),
        ("45", ((12, 0, 0.0), (1, 45, 0, 0.0)), 3.0),
        ("30", ((12, 0, 0.0), (1, 30, 0, 0.0)), 3.0),
        // ("15", ((12, 0, 0.0), (1, 15, 0, 0.0)), 3.0),
        ("-15", ((12, 0, 0.0), (-1, 15, 0, 0.0)), 3.0),
        ("-30", ((12, 0, 0.0), (-1, 30, 0, 0.0)), 3.0),
        ("-45", ((12, 0, 0.0), (-1, 45, 0, 0.0)), 3.0),
        // ("-60", ((12, 0, 0.0), (-1, 60, 0, 0.0)), 3.0),
        ("-75", ((12, 0, 0.0), (-1, 75, 0, 0.0)), 3.0),
        ("-90", ((12, 0, 0.0), (-1, 90, 0, 0.0)), 3.0),
    ];
    let stars = stars.into_iter().map(Star::from).collect::<Vec<_>>();

    let root = BitMapBackend::new("0.png", (1000, 1000)).into_drawing_area();
    root.fill(&White)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Equatorial coordinate system", ("Arial", 40).into_font())
        .margin(50)
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_ranged(-24f64..0f64, -90f64..90f64)?;

    chart
        .configure_mesh()
        .x_labels(24)
        .y_labels(30)
        .x_label_formatter(&|x| format!("{}", x.abs()))
        .draw()?;

    for star in &stars {
        let x = star.coord.alpha.to_rad() / (std::f64::consts::PI * 2_f64) * -24_f64;
        let y = star.coord.delta.to_rad() / (std::f64::consts::PI / 2_f64) * 90_f64;
        let size = match star.mag.round() as i32 {
            -2_i32 => panic!("{}: {}", star.name, star.mag),
            -1_i32 => 5,
            0_i32 => 4,
            1_i32 => 3,
            2_i32 => 2,
            3_i32 => 1,
            _ => panic!("{}: {}", star.name, star.mag),
        };
        let elm = EmptyElement::at((x, y))
            + Circle::new((0, 0), size, ShapeStyle::from(&Black).filled())
            + Text::new(
                // format!("{}({:.3},{:.3})", star.name, x, y),
                format!("{}", star.name),
                (3, 0),
                ("Arial", 20)
                    .into_font()
                    .color(&Black.mix(size as f64 / 5_f64)),
            );
        chart.plotting_area().draw(&elm)?;
    }
    chart.draw_series(LineSeries::new(
        (-240_i64..=0).map(|x| {
            (
                (x as f64) / 10_f64,
                Delta::from((23, 26, 52, 21.406)).to_degree()
                    * (((x as f64 / 10_f64) / 24_f64) * std::f64::consts::PI * 2_f64
                        + std::f64::consts::PI)
                        .sin(),
            )
        }),
        &Red,
    ))?;
    chart.draw_series(LineSeries::new(
        (-240_i64..=0).map(|x| {
            let x = (x as f64) / 10_f64;

            (
                x,
                Delta::from((1, 62, 52, 0.0)).to_degree()
                    * ((x / 24_f64) * (std::f64::consts::PI * 2_f64)
                        - (std::f64::consts::PI / 2_f64)
                        + Alpha::from((12, 51, 26.282)).to_rad())
                    .sin(),
            )
        }),
        &Blue,
    ))?;
    Ok(())
}

struct Star {
    name: String,
    coord: Equatorial,
    mag: f64,
}
impl<S: Into<String>, E: Into<Equatorial>> From<(S, E, f64)> for Star {
    fn from(o: (S, E, f64)) -> Star {
        Star {
            name: o.0.into(),
            coord: o.1.into(),
            mag: o.2,
        }
    }
}
struct Equatorial {
    alpha: Alpha,
    delta: Delta,
}
impl<A: Into<Alpha>, D: Into<Delta>> From<(A, D)> for Equatorial {
    fn from(o: (A, D)) -> Equatorial {
        Equatorial {
            alpha: o.0.into(),
            delta: o.1.into(),
        }
    }
}
struct Alpha {
    h: i64,
    m: u64,
    s: f64,
}
impl Alpha {
    fn to_degree(&self) -> f64 {
        (360_f64 / 24_f64)
            * ((self.h as f64) + (self.m as f64 / 60_f64) + (self.s / 60_f64 / 60_f64))
    }
    fn to_rad(&self) -> f64 {
        (self.to_degree() / 180_f64) * std::f64::consts::PI
    }
}
impl From<(i64, u64, f64)> for Alpha {
    fn from(o: (i64, u64, f64)) -> Alpha {
        Alpha {
            h: o.0,
            m: o.1,
            s: o.2,
        }
    }
}
struct Delta {
    flag: i8,
    deg: i64,
    m: u64,
    s: f64,
}
impl Delta {
    fn to_degree(&self) -> f64 {
        let flag = self.flag.signum() as f64;
        flag * ((self.deg as f64) + (self.m as f64 / 60_f64) + (self.s / 60_f64 / 60_f64))
    }
    fn to_rad(&self) -> f64 {
        (self.to_degree() / 180_f64) * std::f64::consts::PI
    }
}
impl From<(i8, i64, u64, f64)> for Delta {
    fn from(o: (i8, i64, u64, f64)) -> Delta {
        Delta {
            flag: o.0,
            deg: o.1,
            m: o.2,
            s: o.3,
        }
    }
}
