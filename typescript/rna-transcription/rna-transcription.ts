class Transcriptor {
    toRna(dna: string): string {
        let rna: string = ""
        for (const nucleotide of dna) {
            if ("GCTA".indexOf(nucleotide) === -1) {
                throw new Error("Invalid input DNA.")
            }
            rna += (nucleotide === "G" ? "C" :
                    nucleotide === "C" ? "G" :
                    nucleotide === "T" ? "A" :
                                         "U")
        }
        return rna
    }
}

export default Transcriptor
