function sjekk_koden() {
    resultatseksjon = document.getElementById("resultatseksjon");
    resultatteskst = document.getElementById("resultattekst");

    if (kode.value == "b9p4kk_Qa?erty"){
        resultatseksjon.style.backgroundColor = "green";
        resultattekst.innerHTML = "Koden er gyldig! Venlegst ta kontakt med din næraste Kammatufsus AS ansatt for å avtale timen.";
    } else {
        resultatseksjon.style.backgroundColor = "red";
        resultattekst.innerHTML = "Koden er <b>UGYLDIG!</b> Prøver du å svindla Kammatufsus AS?!";
    }

    resultatseksjon.style.visibility = "visible";
}